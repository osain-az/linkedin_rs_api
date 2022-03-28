use crate::http_clinent::errors::ClientErr;

use async_trait::async_trait;
use http::{HeaderMap, Request, Response};
use serde_json::Value;
use std::fs::File;
use url::Url;

/*#[cfg(all(feature = "reqwest_async"))]
compile_error!(
    r#"feature "reqwest_async" and "surf_async" cannot be set at the same time.
If what you want is "seed_async", please turn off default features by adding "default-features=false" in your Cargo.toml"#
);*/
/*
#[cfg(all(feature = "reqwest_async"))]
compile_error!(r#"only one of features "reqwest_async", "seed_async" and "..." can be"#);*/

pub mod client;
pub mod errors;

#[cfg(any(feature = "reqwest_async"))]
pub mod reqwest;
pub mod response;

//#[async_trait::async_trait]
#[async_trait(?Send)]
pub trait HttpClient: Sync + Clone {
    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientErr>
    where
        Self: Sized;

    #[inline]
    async fn get<T>(&self, url: Url, request_body: T, token:String) -> Result<Response<String>, ClientErr>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(
            Request::get(url.to_string())
                .body(request_body.into())
                .unwrap(),
            token
        )
        .await
    }
    #[inline]
    async fn post<T>(&self, url: Url, request_body: T, token:String) -> Result<Response<String>, ClientErr>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(
            Request::post(url.to_string())
                .body(request_body.into())
                .unwrap(),
            token
        )
        .await
    }

    #[cfg(any(feature = "reqwest_async"))]
    #[inline]
    async fn video_post(
        &self,
        url: Url,
        request_body: Value,
        token:String
    ) -> Result<Response<String>, ClientErr> {
        self.video_request(Request::post(url.to_string()).body(request_body).unwrap(),token)
            .await
    }

    #[inline]
    async fn authentication(
        &self,
        url: Url,
        token: String,
    ) -> Result<Response<String>, ClientErr> {
        self.auth_request(Request::get(url.to_string()).body(token).unwrap())
            .await
    }

    #[cfg(any(feature = "reqwest_async"))]
    #[inline]
    async fn file_upload(
        &self,
        url: Url,
        request_body: Vec<u8>,
        token:String,
        upload_type:String
    ) -> Result<Response<String>, ClientErr> {
        if upload_type == "PARTS".to_string() {
            self.file_upload_request(Request::put(url.to_string()).body(request_body).unwrap(),token)
                .await
        }else {
            self.file_upload_request(Request::post(url.to_string()).body(request_body).unwrap(),token)
                .await
        }

    }

    #[inline]
    async fn put<T>(&self, url: Url, request_body: T, token:String) -> Result<Response<String>, ClientErr>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(
            Request::put(url.to_string())
                .body(request_body.into())
                .unwrap(),
            token
        )
        .await
    }
    #[inline]
    async fn delete<T>(&self, url: Url, request_body: T, token:String) -> Result<Response<String>, ClientErr>
    where
        Self: Sized,
        T: Into<String> + Send,
    {
        self.request(
            Request::delete(url.to_string())
                .body(request_body.into())
                .unwrap(),
            token

        )
        .await
    }


    async fn request(&self, request: Request<String>, token: String) -> Result<Response<String>, ClientErr>
    where
        Self: Sized;

    #[cfg(any(feature = "reqwest_async"))]
    async fn video_request(&self, request: Request<Value>, token:String) -> Result<Response<String>, ClientErr>
    where
        Self: Sized;

    #[cfg(any(feature = "reqwest_async"))]
    async fn file_upload_request(
        &self,
        request: Request<Vec<u8>>,
        token:String
    ) -> Result<Response<String>, ClientErr>
    where
        Self: Sized;

    #[cfg(any(feature = "reqwest_async"))]
    async fn auth_request(
        &self,
        request: Request<String>,
    ) -> Result<Response<String>, ClientErr>
        where
            Self: Sized;
}

