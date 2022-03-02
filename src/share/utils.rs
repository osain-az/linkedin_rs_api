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
    }
    else {
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
 pub fn create_file_upload_data(post_type:&str, person_id:String,  post_description:String,media_title:String, media_description:String, media_aset:String) -> Value{
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
