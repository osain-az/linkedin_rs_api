use crate::http_clinent::client::HttpConnection;
use crate::http_clinent::errors::ClientErr;
use crate::post::utils::create_data;

use std::{
    thread,
    time::{Duration, Instant},
};

/*use crate::share::srtuct_helper::{
    MediaUploadInitResponse, UploadingUrl, VideoPartUploadInitResponse,
};
use crate::share::utils::{
    create_data, create_file_upload_data, init_media_upload_data, MediaAnalyze,
};*/

use crate::share::file_utils::FileChunking;
use serde_json::json;
use std::fs::File;
use crate::prelude::VideoApi;
use crate::video::utils::{InitVideoParams, UploadingVideos, VideoUploadStatus};

#[derive(Clone)]
pub struct PostAPI {
    base_url: String,
    person_id: String,
    access_token: String,
}

impl PostAPI {
    pub fn new(base_url: String, person_id: String, access_token: String) -> PostAPI {
        PostAPI {
            base_url,
            person_id,
            access_token,
        }
    }

    pub async fn publish_media_by_id(&self, post_desciption: String, media_id:String, media_title:String) -> Result<String, ClientErr> {

        let person_id = "urn:li:person:".to_owned() + &self.person_id.clone();
        let url = &self.base_url.clone();
        let data = create_data(post_desciption, self.person_id.clone(), media_title, media_id);
        println!("data : {}", data);
        let resp =
            HttpConnection::video_post::<String>(url.to_string(), data, self.access_token.clone())
                .await?;
        Ok(resp)
    }
    pub async fn publish_video_upload(&self, post_desciption: String, media_title:String, video_params:InitVideoParams, file_list:UploadingVideos) -> Result<String, ClientErr> {
         let person_id = self.person_id.clone();
         let base_url = self.base_url.clone();
         let video_base_url = base_url.replace("posts","videos");
        let video_api = VideoApi::new(video_base_url, person_id, self.access_token.clone());

        let upload_resp = video_api.video_upload_handeler(file_list,video_params).await?;

         let video_id = upload_resp.id().to_string();
        let person_id =  self.person_id.clone();
        let url = &self.base_url.clone();
        let data = create_data(post_desciption, person_id, media_title,video_id);

       /* let resp =
            HttpConnection::video_post::<String>(url.to_string(), data, self.access_token.clone())
                .await?;*/
        Ok("".to_string())
    }
}