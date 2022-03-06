use serde_json::json;
use serde_json::Value;

pub fn create_data(
    post_type: &str,
    text_description: String,
    person_id: String,
    source_url: String,
) -> Value {
    let form = if post_type == "TEXT" {
        let form = json!({
              "author": person_id,
              "lifecycleState": "PUBLISHED",
                "specificContent": {
                    "com.linkedin.ugc.ShareContent": {
                        "shareCommentary": {
                            "text": text_description
                        },
                        "shareMediaCategory": "NONE",
                    }
                        },
             "visibility": {
                "com.linkedin.ugc.MemberNetworkVisibility": "PUBLIC"
            }
        });
        form
    } else {
        let mut form = json!({
              "author": person_id,
              "lifecycleState": "PUBLISHED",
                "specificContent": {
                    "com.linkedin.ugc.ShareContent": {
                        "shareCommentary": {
                            "text":  &text_description
                        },
                        "shareMediaCategory":  post_type,
                        "media":[
                         {
                            "status": "READY",

                            "originalUrl": source_url,
                       }
                        ]
                    }
                        },
             "visibility": {
                "com.linkedin.ugc.MemberNetworkVisibility": "PUBLIC"
            }
        });
        form
    };
    form
}
pub fn create_file_upload_data(
    post_type: &str,
    person_id: String,
    post_description: String,
    media_title: String,
    media_description: String,
    media_aset: String,
) -> Value {
    let mut form = json!({
          "author": person_id,
          "lifecycleState": "PUBLISHED",
            "specificContent": {
                "com.linkedin.ugc.ShareContent": {
                    "shareCommentary": {
                        "text":  &post_description
                    },
                    "shareMediaCategory": post_type,
                    "media":[
                     {
                        "status": "READY",
                         "media": media_aset,
                    "title": {
                       "text": media_title
                      },
                      "description": {
                    "text": media_description
                     },

                   }
                    ]
                }
                    },
         "visibility": {
            "com.linkedin.ugc.MemberNetworkVisibility": "PUBLIC"
        }
    });
    form
}

use std::fs::File;

#[derive(Default, Debug)]
pub struct MediaAnalyze {
    upload_method: String,
    file_size_bytes: u64,
    file_size_mb: u64,
}

impl MediaAnalyze {
    pub fn upload_method(&self) -> &str {
        &self.upload_method
    }
    pub fn file_size_bytes(&self) -> u64 {
        self.file_size_bytes
    }
    pub fn file_size_mb(&self) -> u64 {
        self.file_size_mb
    }
}

impl MediaAnalyze {
    pub fn file_analyze(mut self, file: &File) -> MediaAnalyze {
        let bytes = 1048576;
        let max_siz = 209715200; // 200 mb if greater than this then upload file in chunk
        let file_size = file.metadata().unwrap().len();
        self.file_size_bytes = file_size.clone();
        self.file_size_mb = file_size * bytes;

        if file_size >= max_siz {
            self.upload_method = "chunking_upload".to_string()
        } else {
            self.upload_method = "normal_upload".to_string()
        }
        self
    }
}

pub fn init_media_upload_data(media_type: &str, upload_type: &str, person_id: String) -> Value {
    let mut upload_mechan = "";
    let media_identity = if media_type == "IMAGE" {
        upload_mechan = "SYNCHRONOUS_UPLOAD";
        "urn:li:digitalmediaRecipe:feedshare-image"
    } else {
        if upload_type == "MULTIPART_UPLOAD" {
            upload_mechan = "MULTIPART_UPLOAD";
        } else {
            upload_mechan = ""
        };
        "urn:li:digitalmediaRecipe:feedshare-video"
    };

    let data = json!({

            "registerUploadRequest": {
                "recipes": [
                     media_identity,
                ],
                "owner":  person_id,
                "serviceRelationships": [
                    {
                        "relationshipType": "OWNER",
                        "identifier": "urn:li:userGeneratedContent"
                    }
                ],
            "supportedUploadMechanism":[
                   upload_mechan
                   ],
            }
    });
    data
}
