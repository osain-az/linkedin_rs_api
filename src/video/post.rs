use crate::http_clinent::client::HttpConnection;
use crate::http_clinent::errors::ClientErr;
use crate::share::file_utils::FileChunking;
use crate::video::utils::{InitVideoResponse, UploadInstructions, VideoUploadStatus};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
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

        let url = self.base_url.to_owned() + "action=initializeUpload";
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
        let base_url = self.base_url.replace("?", "/");
        let url = base_url + &video_id;
        let token = self.access_token.clone();
        let resp = HttpConnection::get::<VideoUploadStatus>(url, "".to_string(), token).await?;
        Ok(resp)
    }

    pub async fn video_upload_handeler(
        &self,
        video: File,
        video_caption: Option<File>,
        video_thumbnail: Option<File>,
        purpose: String,
    ) -> Result<VideoUploadStatus, ClientErr> {
        let token = self.clone().access_token.clone();
        let url = self.clone().base_url.clone();

        let mut etag_list: Vec<String> = Vec::new();
        let single_upload_size = 4194303; // 4 biytes

        let video_file = video.try_clone().unwrap_or(video);
        let file_size = video_file.metadata().unwrap().len();

        let video_thumb_nail = if video_thumbnail.is_some() {
            true
        } else {
            false
        };

        let _video_caption = if video_caption.is_some() { true } else { false };

        let video_init_params = InitVideoParams {
            initializeUploadRequest: InitializeUploadRequest {
                owner: self.person_id.clone(),
                purpose,
                fileSizeBytes: file_size,
                uploadCaptions: _video_caption,
                uploadThumbnail: video_thumb_nail,
            },
        };

        //it seems like linked allows max of 4mb of upload so it is greater than that then chunk it

        //Initialze request
        let init_resp = self.clone().init_video_upload(video_init_params).await?; // send init request

        let uploading_list = init_resp.clone().value.uploadInstructions.clone();
        let caption_url = init_resp.clone().value.captionsUploadUrl;
        let thumbs_url = init_resp.clone().value.thumbnailUploadUrl;
        let upload_token = init_resp.clone().value.uploadToken.clone();
        let upload_video_id = init_resp.clone().value.video.clone();

        if file_size < single_upload_size {
            //  if init_resp.is_ok() {
            let buffer_file = FileChunking::new(video_file).extract_to_end();
            //this is for single upload  so the upload uploading should be array of length 1
            let upload_url = uploading_list[0].uploadUrl.clone();
            //Upload video
            let video_resp = self
                .clone()
                .upload_media(upload_url.to_string(), buffer_file)
                .await?;

            //get etag in the reponse header
            etag_list.push(video_resp);
        } else {
            let video_init_resp = init_resp;

            let file_chunk = FileChunking::new(video_file.try_clone().unwrap());

            for upload_data in uploading_list.iter() {
                let upload_url = upload_data.uploadUrl.clone();
                let upload_size = upload_data.lastByte.clone();
                let previous_position = upload_data.firstByte.clone();

                let chunk_data = FileChunking::new(video_file.try_clone().unwrap())
                    .extract_by_size_and_offset(upload_size, previous_position);

                let etag_resp = self.clone().upload_media(upload_url, chunk_data).await?;
                etag_list.push(etag_resp);
            }

            let completed_etags = self
                .clone()
                .upload_by_chunk(video_init_resp.clone(), video_file.try_clone().unwrap())
                .await?;
            etag_list = completed_etags;
        }

        if video_thumbnail.is_some() {
            let buffer_file = FileChunking::new(video_thumbnail.unwrap()).extract_to_end();
            let thumb_resp = self.clone().upload_media(thumbs_url, buffer_file).await?;
            etag_list.push(thumb_resp)
        }

        if video_caption.is_some() {
            let buffer_file = FileChunking::new(video_caption.unwrap()).extract_to_end();
            let capt_resp = self.clone().upload_media(caption_url, buffer_file).await?;
            etag_list.push(capt_resp)
        }

        //confirm and finalize upload
        let confrim_data =
            UploadConfrimationRequest::new(upload_video_id.clone(), upload_token, etag_list);

        let data = FinalUploadData::new(confrim_data);
        let final_resp = self.clone().confirm_final_upload(data).await?;

        let status_resp = self.clone().upload_status(upload_video_id).await?;
        println!("status_resp {:?}", status_resp);

        Ok(status_resp)
    }

    async fn upload_by_chunk(
        &self,
        uploading_data: InitVideoResponse,
        video: File,
    ) -> Result<Vec<String>, ClientErr> {
        let mut etag_list: Vec<String> = Vec::new();
        let uploading_data_list = uploading_data.value.uploadInstructions;
        let mut count = 0;

        for upload_data in uploading_data_list.iter() {
            let upload_url = upload_data.uploadUrl.clone();
            let upload_size = upload_data.lastByte.clone();
            let previous_position = upload_data.firstByte.clone();

            println!("size {}  previous: {:?} ", upload_size, previous_position);
            let chunk_data = FileChunking::new(video.try_clone().unwrap())
                .extract_by_size_and_offset(upload_size, previous_position);
            let file_name = format!("/home/azibodusio/Downloads/Deer_{}.mp4", count);
            let mut f = File::create(file_name).unwrap();
            f.write_all(&chunk_data).unwrap();
            let etag_resp = self.clone().upload_media(upload_url, chunk_data).await?;
            etag_list.push(etag_resp);
            count += 1;
            // println!("count list {} etag: {:?} ", count, etag_list)
        }
        Ok(etag_list)
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

#[derive(Serialize)]
pub struct InitializeUploadRequest {
    owner: String,
    purpose: String,
    fileSizeBytes: u64,
    uploadCaptions: bool,
    uploadThumbnail: bool,
}

#[derive(Serialize)]
pub struct InitVideoParams {
    initializeUploadRequest: InitializeUploadRequest,
}
