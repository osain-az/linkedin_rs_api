use crate::http_clinent::client::HttpConnection;
use crate::http_clinent::errors::ClientErr;

use crate::share::srtuct_helper::ImageInitResponse;
use crate::share::utils::{create_data, PostParams};
use serde_json::json;
use std::fs::File;

pub struct SharePost {
    base_url: String,
    access_code: String,
    person_id: String,
}

impl SharePost {
    pub fn new(base_url: String, access_code: String, person_id: String) -> SharePost {
        SharePost {
            base_url,
            access_code,
            person_id,
        }
    }

    pub async fn post_text(&self, text_desciption: String) -> Result<String, ClientErr> {
        let person_id = "urn:li:person:".to_owned() + &self.person_id.clone();
        let url = &self.base_url;
        // The type of `john` is `serde_json::Value`
        let data = create_data("text", text_desciption, person_id, None);

        let resp = HttpConnection::video_post::<String>(url.to_string(), data).await?;
        Ok(resp)
    }

    pub async fn post_article(&self, post_params: PostParams) -> Result<String, ClientErr> {
        let person_id = "urn:li:person:".to_owned() + &self.person_id.clone();
        let data = create_data("article", "".to_string(), person_id, Some(post_params));
        let url = &self.base_url;
        // The type of `john` is `serde_json::Value`

        let resp = HttpConnection::video_post::<String>(url.to_string(), data).await?;
        Ok(resp)
    }

    pub async fn init_image_post(self) -> Result<ImageInitResponse, ClientErr> {
        let person_id = "urn:li:person:".to_owned() + &self.person_id.clone();
        let url = &self.base_url;

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

        let testing = json!({
            "value": {
                    "uploadMechanism": {
                    "com.linkedin.digitalmedia.uploading.MediaUploadHttpRequest": {
                    "uploadUrl":"String"
                    }
                    },
                    "mediaArtifact": "String",
                    "asset": "String"
                }
        });
        let result = serde_json::from_value(testing).unwrap();

        let resp = HttpConnection::video_post::<ImageInitResponse>(url.to_string(), data).await?;
        Ok(resp)
    }

    /*    pub async fn upload_image(self, post_params: PostParams, file: File)-> {

        let resp = self.init_image_post().await?; // send init request
    }

    pub async fn post_with_image(self, post_params: PostParams) {
        let resp = self.init_image_post().await?; // send init request
    }*/
}

/*struct ImageInitRespone{
    value: Value,

}
struct Value{
    mediaArtifact:String,
    uploadMechanism :UploadMech,
    asset:String,
}
struct UploadMech{
    "com.linkedin.digitalmedia.uploading.MediaUploadHttpRequest" : String
}*/
