use crate::account::AccountApi;

#[derive(Debug)]
pub struct Client {
    graph: String,
    /// The access token token type is used to indicate which type of token is
    /// currently passed to the method. It is required to provide either
    /// page_access_token  or user_access_token. corresponding to the token
    /// passed it
    access_token: String,
}


impl Default for Client {
    fn default() -> Self {
        let graph = "https://api.linkedin.com/v2/NODE/EDGE".to_string();

        Self {
            graph,
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

}