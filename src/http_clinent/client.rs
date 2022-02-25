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
use url::Url;
use std::fs::File;

#[cfg(any(feature = "reqwest_async"))]
pub type HttpConnection = GenericClientConnection<ReqwestClient>;

#[derive(Debug, Clone)]
pub struct GenericClientConnection<HttpC: HttpClient> {
    http_client: Arc<HttpC>,
    url: Url,
}
impl<HttpC: HttpClient> GenericClientConnection<HttpC> {
    pub async fn get<T>(build_url: String, body: String) -> Result<T, ClientErr>
    where
        Self: Sized,
        T: DeserializeOwned, // response Type
    {
        let client = HttpC::new(None)?;
        let resp = client.get(build_url.parse().unwrap(), body).await?;
        let result = deserialize_response::<T>(resp.body())?;
        Ok(result)
    }

    pub async fn post<R, T>(build_url: String, body: T) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
        T: Into<String> + Send,
    {
        let client = HttpC::new(None)?;
        let resp = client.post(build_url.parse().unwrap(), body).await?;
        let result = deserialize_response::<R>(resp.body())?;
        Ok(result)
    }
    pub async fn delete<R>(build_url: String, body: String) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
    {
        let client = HttpC::new(None)?;
        let resp = client.delete(build_url.parse().unwrap(), body).await?;
        let result = deserialize_response::<R>(resp.body())?;
        Ok(result)
    }

    #[cfg(any(feature = "reqwest_async"))]
    pub async fn video_post<R>(build_url: String, body: Value) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
                             //T: Send + DeserializeOwned,
    {
        let client = HttpC::new(None)?;
        let resp = client.video_post(build_url.parse().unwrap(), body).await?;
        let result = deserialize_response::<R>(resp.body())?;
        Ok(result)
    }

    #[cfg(any(feature = "reqwest_async"))]
    pub async fn file_upload_post<R>(build_url: String, body: File) -> Result<R, ClientErr>
    where
        Self: Sized,
        R: DeserializeOwned, // response Type
                             //T: Send + DeserializeOwned,
    {
        let client = HttpC::new(None)?;
        let resp = client.file_upload()_post(build_url.parse().unwrap(), body).await?;
        let result = deserialize_response::<R>(resp.body())?;
        Ok(result)
    }
}
