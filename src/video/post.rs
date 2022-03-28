use crate::http_clinent::client::HttpConnection;
use crate::http_clinent::errors::ClientErr;
use crate::share::file_utils::FileChunking;
use crate::video::utils::{InitVideoParams, InitVideoResponse, UploadingVideos, VideoUploadStatus};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Deserialize, Serialize, Clone)]
pub struct VideoApi {
    base_url: String,
    person_id: String,
    access_token: String,
}

impl VideoApi {
    pub fn new(base_url: String, person_id: String, access_token: String) -> VideoApi {
        VideoApi {
            base_url,
            person_id,
            access_token,
        }
    }

    async fn init_video_upload(
        &self,
        init_params: InitVideoParams,
    ) -> Result<InitVideoResponse, ClientErr> {
        let person_id = "urn:li:person:".to_owned() + &self.person_id.clone();

         let url = self.base_url.to_owned()+"action=initializeUpload";
        let resp = HttpConnection::video_post::<InitVideoResponse>(
            url.to_string(),
            serde_json::to_value(&init_params).unwrap(),
            self.access_token.clone(),
        )
        .await?;
        Ok(resp)
    }
    async fn upload_media(
        self,
        upload_url: String,
        buffer_file: Vec<u8>,
    ) -> Result<String, ClientErr> {
        let token = self.access_token.clone();
        let resp = HttpConnection::file_upload_post::<String>(
            upload_url,
            buffer_file,
            token,
            "PARTS".to_string(),
        )
        .await?;
        Ok(resp)
    }

    async fn confirm_final_upload(
        self,
        uploading_data: FinalUploadData,
    ) -> Result<String, ClientErr> {
        let url = self.base_url + "action=finalizeUpload";
        let token = self.access_token.clone();
        let resp = HttpConnection::request_empty_body::<String>(
            url,
            serde_json::to_value(&uploading_data).unwrap(),
            token,
        )
        .await?;
        Ok(resp)
    }

    pub async fn upload_status(self, video_id: String) -> Result<VideoUploadStatus, ClientErr> {
        let base_url   = self.base_url.replace("?","/");
        let url =  base_url+&video_id;
        let token = self.access_token.clone();
        let resp = HttpConnection::get::<VideoUploadStatus>(url, "".to_string(), token).await?;
        Ok(resp)
    }

    pub async fn video_upload_handeler(
        &self,
        file_list: UploadingVideos,
        init_params: InitVideoParams,
    ) -> Result<VideoUploadStatus, ClientErr> {
        let token = self.clone().access_token.clone();
        let url = self.clone().base_url.clone();
        let mut etag_list: Vec<String> = Vec::new();
        let single_upload_size = 4194303; // 4 biytes

        if file_list.main_video.is_none() {
            //dont run the rest of the code
            Err(ClientErr::LinkedinError(format!(
                "Error in the passed the params. The video file is required, Try uploading again "
            )))
        } else {
            let video_file = file_list.main_video.unwrap().try_clone().unwrap();
            let file_size = video_file.metadata().unwrap().len();

            //it seems like linked allows max of 4mb of upload so it is greater than that then chunk it
            let video_params = init_params.clone();

            if file_size < single_upload_size {
                let resp = self.clone().init_video_upload(video_params.clone()).await; // send init request

                if resp.is_ok() {

                    let video_init_resp = resp.as_ref().unwrap().clone();
                    let buffer_file = FileChunking::new(video_file).extract_to_end();
                    //this is for single upload  so the upload uploading should be array of length 1
                    let upload_url = &video_init_resp.value.uploadInstructions[0]
                        .uploadUrl
                        .clone();
                    let video_resp = self
                        .clone()
                        .upload_media(upload_url.to_string(), buffer_file)
                        .await;

                    if video_resp.is_ok() {
                        //get etag in the reponse header
                        etag_list.push(video_resp.unwrap());
                        // upload vido caption
                        if !video_init_resp.value.captionsUploadUrl.is_empty()
                            && file_list.video_caption.is_some()
                        {
                            let caption_file = file_list.video_caption.unwrap();
                            let buffer_file = FileChunking::new(caption_file).extract_to_end();
                            let capt_resp = self
                                .clone()
                                .upload_media(video_init_resp.value.captionsUploadUrl, buffer_file)
                                .await?;
                            etag_list.push(capt_resp)
                        };

                        if !video_init_resp.value.thumbnailUploadUrl.is_empty()
                            && file_list.video_thumbnail.is_some()
                        {
                            let thum_file = file_list.video_thumbnail.unwrap();
                            let buffer_file = FileChunking::new(thum_file).extract_to_end();
                            let thumb_resp = self
                                .clone()
                                .upload_media(video_init_resp.value.thumbnailUploadUrl, buffer_file)
                                .await?;
                            etag_list.push(thumb_resp)
                        };

                        //confirm and finalize upload
                        let confrim_data = UploadConfrimationRequest::new(
                            video_init_resp.value.video.clone(),
                            video_init_resp.value.uploadToken.clone(),
                            etag_list,
                        );

                        let data = FinalUploadData::new(confrim_data);
                        let final_resp = self.clone().confirm_final_upload(data).await?;
                        let status_resp  = self
                            .clone()
                            .upload_status(video_init_resp.value.video)
                            .await?;
                        println!("status_resp {:?}" ,status_resp);

                        Ok(status_resp)
                    } else {
                        Err(ClientErr::LinkedinError(format!(
                            "Error in initalizing  vidoe upload, try again.  Err message: {:?}",
                            resp.err()
                        )))
                    }
                } else {
                    Err(ClientErr::HttpClient(format!(
                        "Error in initalizing  vidoe upload, try again.  Err message: {:?}",
                        resp.err()
                    )))
                }
            } else {
                let video_init_resp = self.clone().init_video_upload(init_params).await?; // send init request

                let file_chunk = FileChunking::new(video_file.try_clone().unwrap());

                let uploading_list = video_init_resp.clone().value.uploadInstructions.clone();
                let caption_url = video_init_resp.clone().value.captionsUploadUrl;
                let thumbs_url = video_init_resp.clone().value.thumbnailUploadUrl;
                let upload_token = video_init_resp.clone().value.uploadToken.clone();
                let upload_video_id = video_init_resp.clone().value.video.clone();

                for upload_data in uploading_list.iter() {
                    let upload_url = upload_data.uploadUrl.clone();
                    let upload_size = upload_data.lastByte.clone();
                    let previous_position = upload_data.firstByte.clone();

                    let chunk_data = FileChunking::new(video_file.try_clone().unwrap())
                        .extract_by_size_and_offset(upload_size , previous_position);

                    let etag_resp = self.clone().upload_media(upload_url, chunk_data).await?;
                    etag_list.push(etag_resp);
                }

                if !caption_url.is_empty() && file_list.video_caption.is_some() {
                    let caption_file = file_list.video_caption.unwrap();
                    let buffer_file = FileChunking::new(caption_file).extract_to_end();
                    let capt_resp = self.clone().upload_media(caption_url, buffer_file).await?;
                    etag_list.push(capt_resp)
                }

                if !thumbs_url.is_empty() && file_list.video_thumbnail.is_some() {
                    let thum_file = file_list.video_thumbnail.unwrap();
                    let buffer_file = FileChunking::new(thum_file).extract_to_end();
                    let thumb_resp = self.clone().upload_media(thumbs_url, buffer_file).await?;
                    etag_list.push(thumb_resp);
                }
                //confirm and finalize upload
                let confrim_data = UploadConfrimationRequest::new(
                    upload_video_id.clone(),
                    upload_token,
                    etag_list.clone(),
                );
                println!("etag lsis {:?}",etag_list.len() );

                let data = FinalUploadData::new(confrim_data);
                let final_resp = self.clone().confirm_final_upload(data).await?;
                  println!("confirmation {}",final_resp );
                let status_resp = self.clone().upload_status(upload_video_id.clone()).await?;
                println!("staus{:?}",status_resp );
                Ok(status_resp)
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct FinalUploadData {
    pub finalizeUploadRequest: UploadConfrimationRequest,
}

impl FinalUploadData {
    pub fn new(finalizeUploadRequest: UploadConfrimationRequest) -> Self {
        FinalUploadData {
            finalizeUploadRequest,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct UploadConfrimationRequest {
    video: String,
    uploadToken: String,
    uploadedPartIds: Vec<String>,
}

impl UploadConfrimationRequest {
    pub fn new(video: String, uploadToken: String, uploadedPartIds: Vec<String>) -> Self {
        UploadConfrimationRequest {
            video,
            uploadToken,
            uploadedPartIds,
        }
    }
    pub fn set_video(&mut self, video: String) {
        self.video = video;
    }
    pub fn set_uploadToken(&mut self, uploadToken: String) {
        self.uploadToken = uploadToken;
    }
    pub fn set_uploadedPartIds(&mut self, uploadedPartIds: Vec<String>) {
        self.uploadedPartIds = uploadedPartIds;
    }
}
