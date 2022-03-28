use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize, Serialize,
};
use std::borrow::BorrowMut;
use std::convert::TryInto;
use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crate::http_clinent::errors::ClientErr;
//use crate::universal::reqwest::ReqwestClient;
#[cfg(any(feature = "reqwest_async"))]
use crate::http_clinent::reqwest::ReqwestClient;
use crate::http_clinent::response::{deserialize_response, ClientResult};

use crate::http_clinent::HttpClient;
use async_trait::async_trait;

#[cfg(any(feature = "reqwest_async"))]
use reqwest::multipart::Form;
//use reqwest::multipart::Form;
//#[async_trait(?Send)]

use serde_json::Value;
use std::fs::File;
use url::Url;

#[cfg(any(feature = "reqwest_async"))]
pub type HttpConnection = GenericClientConnection<ReqwestClient>;

#[derive(Debug, Clone)]
pub struct GenericClientConnection<HttpC: HttpClient> {
    http_client: Arc<HttpC>,
    url: Url,
}
impl<HttpC: HttpClient> GenericClientConnection<HttpC> {
    pub async fn get<T>(build_url: String, body: String, token:String) -> Result<T, ClientErr>
    where
        Self: Sized,
        T: DeserializeOwned, // response Type
    {
        let client = HttpC::new(None)?;
        let resp = client.get(build_url.parse().unwrap(), body, token).await?;

        let result = deserialize_response::<T>(resp.body())?;
        Ok(result)
    }
    pub async fn auth<T>(build_url: String, token: String) -> Result<T, ClientErr>
    where
        Self: Sized,
        T: DeserializeOwned, // response Type
    {
        let client = HttpC::new(None)?;
        let resp = client
            .authentication(build_url.parse().unwrap(), token)
            .await?;
        let result = deserialize_response::<T>(resp.body())?;
        Ok(result)
    }

    pub async fn post<R, T>(build_url: String, body: T, token:String) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
        T: Into<String> + Send,
    {
        let client = HttpC::new(None)?;
        let resp = client.post(build_url.parse().unwrap(), body, token).await?;
        let result = deserialize_response::<R>(resp.body())?;
        Ok(result)
    }
    pub async fn delete<R>(build_url: String, body: String, token : String) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
    {
        let client = HttpC::new(None)?;
        let resp = client.delete(build_url.parse().unwrap(), body, token).await?;
        let result = deserialize_response::<R>(resp.body())?;
        Ok(result)
    }

    #[cfg(any(feature = "reqwest_async"))]
    pub async fn video_post<R>(
        build_url: String,
        body: Value,
        access_token: String,
    ) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
                             //T: Send + DeserializeOwned,
    {
        let client = HttpC::new(None)?;

        let resp = client
            .video_post(build_url.parse().unwrap(), body, access_token)
            .await?;
            let result = deserialize_response::<R>(resp.body())?;
            Ok(result)
    }
    pub async fn request_empty_body<R>(
        build_url: String,
        body: Value,
        access_token: String,
    ) -> Result<String, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
                             //T: Send + DeserializeOwned,
    {
        let client = HttpC::new(None)?;

        let resp = client
            .video_post(build_url.parse().unwrap(), body, access_token)
            .await?;
        if resp.body().is_empty() {
            let custom_resp = "status : ".to_owned() + &resp.status().to_string();
            Ok(custom_resp)
        }else {
            // we are only expecting to use this for empty body request
            Err(ClientErr::LinkedinError(format!(
                "Unexpected request.  expecting an empty request body but found : {:?}",
                resp.body()
            )))
        }

    }

    #[cfg(any(feature = "reqwest_async"))]
    pub async fn file_upload_post<R>(
        build_url: String,
        body: Vec<u8>,
        token: String,
        upload_type: String,
    ) -> Result<String, ClientErr>
    where
        Self: Sized,
        // R: DeserializeOwned, // response Type
        R: Into<String> + Send,
    {
        let client = HttpC::new(None)?;
        let resp = client
            .file_upload(build_url.parse().unwrap(), body, token, upload_type)
            .await;
        if resp.is_ok() {
            let result = resp.unwrap().headers().get("etag").unwrap().to_str().unwrap().to_string();
            Ok(result)
        } else {
            Err(ClientErr::LinkedinError(format!(
                "Something went wrong.  Err message: {:?}",
                resp.err()
            )))
        }

        /*   let result = deserialize_response::<String>(resp.body())?;
        Ok(result)*/
    }
}
