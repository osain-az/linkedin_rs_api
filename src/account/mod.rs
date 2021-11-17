use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::*;
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
use crate::login::prelude::Token;
use seed::fetch::Method::Head;

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

    pub async fn token(&self) -> seed::fetch::Result<Token> {
        log!(self.base_url);

        let header =  Header::content_type("application/x-www-form-urlencoded") ;

        let request = Request::new(&self.base_url).method(Method::Post).header(header);
        fetch(request).await?.json::<Token>().await

    }

}

