use crate::account::AccountApi;
use crate::prelude::{VideoApi, SharePost, PostAPI};

#[derive(Debug)]
pub struct Client {
    base_url: String,
    /// The access token token type is used to indicate which type of token is
    /// currently passed to the method. It is required to provide either
    /// page_access_token  or user_access_token. corresponding to the token
    /// passed it
    access_token: String,
}


impl Default for Client {
    fn default() -> Self {
        let base_url = "https://api.linkedin.com/v2/NODE/".to_string();

        Self {
            base_url,
            access_token: "".to_string(),
        }
    }
}

impl Client {


 /*   pub fn new(access_token: String) -> Client {
        Client::default().add_access_token(access_token)
    }
*/
    /// This method add access token to the client when the user has
    /// authenticate from the frontend
    pub fn accss_token(
        access_token_url: String,
    ) -> AccountApi {
        Client::default();
        AccountApi::new(access_token_url, "".to_string())
    }

    pub fn token(
        &self,
        access_token: String,
    ) -> AccountApi {
       let base_url =  Client::default().base_url.replace("NODE/", "me");

        AccountApi::new(base_url, access_token)
    }
    pub fn share(
        &self,
        person_id:String,
        access_token : String
    ) -> SharePost {
       let base_url =  Client::default().base_url.replace("NODE/", "ugcPosts");
        SharePost::new(base_url,person_id, access_token)
    }
    pub fn video_upload(
        &self,
        person_id:String,
        access_token : String
    ) -> VideoApi {
       let base_url =  Client::default().base_url.replace("NODE/", "videos?");
        VideoApi::new(base_url, person_id, access_token)
    }
    pub fn publish_video(
        &self,
        person_id:String,
        access_token : String
    ) -> PostAPI {
       let base_url =  Client::default().base_url.replace("NODE/", "posts?");
        PostAPI::new(base_url, person_id, access_token)
    }
}