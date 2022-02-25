//! Reqwest HTTP client
use std::convert::TryInto;
use std::io::Read;

use crate::http_clinent::errors::ClientErr;
//use crate::universal::utils::generic_req;
use crate::http_clinent::HttpClient;
use ::reqwest::Body;
#[cfg(feature = "reqwest_async")]
use ::reqwest::Client;
use async_trait::async_trait;
use http::header::HeaderMap;
use http::Method;

//use reqwest::multipart;
use reqwest::multipart::Form;
use reqwest::{Request, RequestBuilder};

use serde_json::Value;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    pub client: Client,
    pub headers: HeaderMap,
}

#[async_trait(?Send)]
impl HttpClient for ReqwestClient {
    fn new<U: Into<Option<HeaderMap>>>(headers: U) -> Result<Self, ClientErr> {
        let client = Client::builder();
        let headers = match headers.into() {
            Some(h) => h,
            None => HeaderMap::new(),
        };

        client
            .default_headers(headers.clone())
            .build()
            .map(|c| ReqwestClient { client: c, headers })
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))
    }

    async fn request(
        &self,
        request: http::Request<String>,
    ) -> Result<http::Response<String>, ClientErr> {
        // No version on the response when using from client but works when using from server (backend)
        let version = request.version().clone();
        let req = request.try_into().unwrap();

        let resp = self
            .client
            .execute(req)
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;

        let status_code = resp.status();
        let headers = resp.headers().clone();
        // No version on the response when using from client but works when using from server (backend)
        // let version = resp.version();
        let content = resp
            .text()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
        let mut build = http::Response::builder();

        for header in headers.iter() {
            build = build.header(header.0, header.1);
        }

        build
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))
    }

    async fn video_request(
        &self,
        request: http::Request<Value>,
    ) -> Result<http::Response<String>, ClientErr> {
        //et req: Request = request.try_into().unwrap();
        // let req: RequestBuilder = generic_req::<FormData>(request).unwrap();
        let url = request.uri().to_string();
        let method = request.method().clone();
        let body = request.body().to_owned();

        let req = match method {
            Method::GET => Client::new().get(url),
            Method::POST => Client::new().post(url),
            Method::PUT => Client::new().put(url),
            Method::DELETE => Client::new().delete(url),

            m @ _ => return Err(ClientErr::HttpClient(format!("invalid method {}", m))),
        };
        let resp = req
            .header("X-Restli-Protocol-Version", "2.0.0")
            .json(&body)
            .send()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
        // No version on the response when using from client but works when using from server (backend)
        let version = request.version();
        let status_code = resp.status();
        let headers = resp.headers().clone();
        //let version = resp.version();

        let content = resp
            .text()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;

        let mut build = http::Response::builder();

        for header in headers.iter() {
            build = build.header(header.0, header.1);
        }

        build
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))
    }

    async fn file_upload_request(
        &self,
        request: http::Request<File>,
    ) -> Result<http::Response<String>, ClientErr> {
        //et req: Request = request.try_into().unwrap();
        // let req: RequestBuilder = generic_req::<FormData>(request).unwrap();
        let url = request.uri().to_string();
        let method = request.method().clone();
        let body = request.body().to_owned();

        let req = match method {
            Method::GET => Client::new().get(url),
            Method::POST => Client::new().post(url),
            Method::PUT => Client::new().put(url),
            m @ _ => return Err(ClientErr::HttpClient(format!("invalid method {}", m))),
        };
        let resp = req
            .header("X-Restli-Protocol-Version", "2.0.0")
            .body(Body::from(body))
            .send()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;
        // No version on the response when using from client but works when using from server (backend)
        let version = request.version();
        let status_code = resp.status();
        let headers = resp.headers().clone();
        //let version = resp.version();

        let content = resp
            .text()
            .await
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?;

        let mut build = http::Response::builder();

        for header in headers.iter() {
            build = build.header(header.0, header.1);
        }

        build
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))
    }
}
