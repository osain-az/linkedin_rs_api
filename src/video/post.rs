use std::fs::File;
use seed::video;
use serde::Serialize;
use crate::http_clinent::client::HttpConnection;
use crate::http_clinent::errors::ClientErr;
use crate::share::file_utils::FileChunking;
use crate::video::utils::{InitVideoParams, InitVideoResponse, UploadingVideos};

pub  struct VideoApi{
    base_url: String,
    person_id: String,
    access_token :String
}


impl VideoApi {
    pub fn new(base_url: String, person_id: String, access_token:String) -> SharePost {
        SharePost {
            base_url,
            person_id,
            access_token
        }
    }

    pub async fn init_video(&self, init_params: InitVideoParams) -> Result<InitVideoResponse, ClientErr> {

        let person_id = "urn:li:person:".to_owned() + &self.person_id.clone();

  //videos?action=initializeUpload
        let url = &self.base_url.clone();
        let resp = HttpConnection::video_post::<InitVideoResponse>(url.to_string(), init_params.serialize(Serialize), self.access_token.clone()).await?
        Ok(resp)
    }
    async fn upload_media(self, upload_url :String, buffer_file:Vec<u8>) ->Result<String,ClientErr> {
        let token = self.access_token.clone();
        let resp = HttpConnection::file_upload_post::<String>(upload_url,buffer_file,token,"VIDEO".to_string()).await?;
        Ok(resp)
    }

  async fn confirm_final_upload(self, uploading_data:FinalUploadData) ->Result< String,ClientErr> {
        let url = self.base_url+ "action=finalizeUpload";
        let token = self.access_token.clone();
        let resp = HttpConnection::video_post::<String>(url,uploading_data,token).await?;
        Ok(resp)
    }

  async fn upload_status(self, video_id:String) ->Result< String,ClientErr> {
        let url = self.base_url+ "action=finalizeUpload";
        let token = self.access_token.clone();
        let resp = HttpConnection::get::<VideoUploadStatus>(url,video!()).await?;
        Ok(resp)
    }

    pub async fn video_upload_handeler(&self, file_list:UploadingVideos, init_params: InitVideoParams) -> Result<String, ClientErr> {

        let token = self.access_token.clone();
        let url = self.base_url.clone();
        let mut etag_list : Vec<String> = Vec::new();

         let single_upload_size = 4194303; // 4 biytes
         if file_list.main_video.is_some() {
                    Err()
         }
          let video_file = file_list.main_video.unwrap().try_clone();

         let file_size = video_file.metadata().unwrap().len();

          if file_size < single_upload_size {
              let video_params = init_params.clone();
            let resp =    self.clone().init_video(init_params).await;// send init request

              if resp.is_ok() {
                   let resp_data = resp.unwrap().clone();
                  let buffer_file = FileChunking::new(video_file).extract_to_end();
                   let video_resp =  self.upload_media(resp_data.value.video.clone(), buffer_file).await;

                   if video_resp.is_ok() {
                       etag_list.push(video_resp.unwrap());
                       // upload vido caption
                       if !resp_data.value.captionsUploadUrl.is_empty(){
                           let caption_file =  file_list.video_caption.unwrap();
                           let buffer_file = FileChunking::new(caption_file).extract_to_end();
                           let capt_resp =  self.upload_media(resp_data.value.video.clone(), buffer_file).await?;
                           etag_list.push(capt_resp)
                       };

                       if !resp_data.value.thumbnailUploadUrl.is_empty(){
                           let thum_file =  file_list.video_thumbnail.unwrap();
                           let buffer_file = FileChunking::new(thum_file).extract_to_end();
                           let thumb_resp =  self.upload_media(resp_data.value.video.clone(), buffer_file).await?;
                           etag_list.push(thumb_resp_resp)

                       };

                       //confirm and finalize upload

                         let final_resp = self.confirm_final_upload().await?;


                   }else {
                       Err()
                   }


              }else {

              }


        }else {

            let file_chunk = FileChunking::new(file);
            let mut chunking = Some(true) ;
            while let  Some (is_chunking)  = chunking {

                if !file_chunk.is_completed() {
                    let chunked_data =  file_chunk.chunk_by_5mb();
                    let res = self.upload_media().await;
                }else {

                }

            }
        }


        if resp.is_ok() {

            if file_analyze.upload_method() == "normal_upload"  {
                let data = resp.unwrap();
                let media_aset = data.value.asset.clone();
                let res =  self.clone().upload_media(data.value.uploadMechanism.media_upload_http_request.uploadUrl, buffer_file).await;
            }else {

            }
        }else {

        }



    }
    }



}




#[derive(Deserialize, Serialize)]
struct  FinalUploadData{
    pub finalizeUploadRequest:FinalizeUploadRequest
}


#[derive(Deserialize, Serialize)]
struct FinalizeUploadRequest{
    video:String,
    uploadToken:String,
    uploadedPartIds: Vec<String>
}

impl FinalizeUploadRequest {
    pub fn new(video: String, uploadToken: String, uploadedPartIds: Vec<String>) -> Self {
        FinalizeUploadRequest { video, uploadToken, uploadedPartIds }
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


#[derive(Deserialize, Serialize)]
struct VideoUploadStatus{
    owner:String,
    id:String,
    status:String
}