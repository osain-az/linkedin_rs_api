use crate::http_clinent::client::HttpConnection;
use crate::http_clinent::errors::ClientErr;

use crate::share::srtuct_helper::{ImageInitResponse, UploadingUrl};
use crate::share::utils::{create_data, create_file_upload_data};
use serde_json::json;
use std::fs::File;

#[derive(Clone)]
pub struct SharePost {
    base_url: String,
    person_id: String,
    access_token :String
}

impl SharePost {
    pub fn new(base_url: String, person_id: String, access_token:String) -> SharePost {
        SharePost {
            base_url,
            person_id,
            access_token
        }
    }

    pub async fn post_text(&self, text_desciption: String) -> Result<String, ClientErr> {
        let person_id = "urn:li:person:".to_owned() + &self.person_id.clone();
        let url = &self.base_url.clone();
        let data = create_data("TEXT", text_desciption, person_id, "".to_string());
        let resp = HttpConnection::video_post::<String>(url.to_string(), data, self.access_token.clone()).await?;
        Ok(resp)
    }

    pub async fn post_article(&self, description:String, source_url:String) -> Result<String, ClientErr> {
        let person_id = "urn:li:person:".to_owned() + &self.person_id.clone();
        let data = create_data("ARTICLE", description, person_id, source_url);
        let url = &self.base_url;
       println!("contnet{:?} ",data);
        let resp = HttpConnection::video_post::<String>(url.to_string(), data, self.access_token.clone()).await?;
        Ok(resp)
    }

     async fn init_image_post(self) -> Result<ImageInitResponse, ClientErr> {
        let person_id = "urn:li:person:".to_owned() + &self.person_id.clone();
        let url = &self.base_url.replace("ugcPosts","assets?action=registerUpload");

        let data = json!({

                "registerUploadRequest": {
                    "recipes": [
                        "urn:li:digitalmediaRecipe:feedshare-image"
                    ],
                    "owner":  person_id,
                    "serviceRelationships": [
                        {
                            "relationshipType": "OWNER",
                            "identifier": "urn:li:userGeneratedContent"
                        }
                    ]
                }
        });

        let resp = HttpConnection::video_post::<ImageInitResponse>(url.to_string(), data, self.access_token).await?;
        Ok(resp)
    }

       async fn upload_image(self, upload_url :String, buffer_file:Vec<u8>)->Result<String,ClientErr> {
          let token = self.access_token.clone();
           let person_id = "urn:li:person:".to_owned() + &self.person_id.clone();

          let resp = HttpConnection::file_upload_post::<String>(upload_url,buffer_file,token).await?;
          Ok(resp)
    }

    pub async fn post_with_image(self, buffer_file:  Vec<u8>, post_description:String, image_title:String, image_description:String) -> Result<String,ClientErr>{
         let token = self.access_token.clone();
        let url = self.base_url.clone();
        let person_id = "urn:li:person:".to_owned() + &self.person_id.clone();
        let resp = self.clone().init_image_post().await; // send init request

        if resp.is_ok() {
             let data = resp.unwrap();
             let media_aset = data.value.asset.clone();
            let res =  self.clone().upload_image(data.value.uploadMechanism.media_upload_http_request.uploadUrl,buffer_file).await;

            if res.is_ok() {
                let data = create_file_upload_data(
                    "IMAGE",
                    person_id.to_string(),
                    post_description.to_string(),
                    image_title,
                    image_description,
                    media_aset

                 ); // send init request
                let resp = HttpConnection::video_post::<String>(url.to_string(),data,token.to_string()).await?;
                Ok(resp)

            }else {
                Err(ClientErr::LinkedinError(format!("Error in initiazing  images post, try again.  Err message: {:?}",res.err())))

            }

        } else {
          Err(ClientErr::LinkedinError(format!("Error in initiazing  images post, try again.  Err message: {:?}",resp.err())))
        }
    }
}
