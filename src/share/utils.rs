use serde_json::json;
use serde_json::Value;

pub fn create_data(
    post_type: &str,
    text_description: String,
    person_id: String,
    media: Option<PostParams>,
) -> Value {
    let form = if post_type == "text" {
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
                            "text":  &media.as_ref().unwrap(). shared_description
                        },
                        "shareMediaCategory":  "ARTICLE",
                        "media ":[
                         {
                            "status": "READY",
                            "description": {
                                "text": &media.as_ref().unwrap().media_description
                            },
                            "originalUrl": &media.as_ref().unwrap().media_url,
                            "title": {
                                "text":  &media.as_ref().unwrap().media_title
                            }
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

pub struct PostParams {
    shared_description: String,
    media_description: String,
    media_title: String,
    media_url: String,
}

impl PostParams {
    pub fn new(
        shared_description: String,
        media_description: String,
        media_title: String,
        media_url: String,
    ) -> Self {
        PostParams {
            shared_description,
            media_description,
            media_title,
            media_url,
        }
    }
    pub fn set_shared_description(&mut self, shared_description: String) {
        self.shared_description = shared_description;
    }
    pub fn set_media_description(&mut self, media_description: String) {
        self.media_description = media_description;
    }
    pub fn set_media_title(&mut self, media_title: String) {
        self.media_title = media_title;
    }
}
