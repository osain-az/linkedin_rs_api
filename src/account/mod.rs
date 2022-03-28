use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::*;
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
use crate::login::prelude::Token;
use seed::fetch::Method::Head;
use crate::http_clinent::client::HttpConnection;
use crate::http_clinent::errors::ClientErr;

pub struct AccountApi{
    base_url:String,
    access_code:String
}

impl AccountApi {

    pub fn new(base_url:String,access_code :String) -> AccountApi{
            AccountApi{
                base_url,
                access_code
            }
    }

    pub async fn get(&self) -> Result<Token, ClientErr> {
    let resp = HttpConnection::post::<Token,String>(self.base_url.to_string(),"".to_string(),"".to_string()).await?;
        Ok(resp)
    }

    pub async fn authenticate(&self) -> Result<String, ClientErr> {

        let resp = HttpConnection::auth::<String>(self.base_url.to_string(),self.access_code.to_string()).await?;
        Ok(resp)
    }

}

