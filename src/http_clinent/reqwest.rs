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
use http::{Method, Response};

//use reqwest::multipart;
use reqwest::multipart::Form;
use reqwest::{Request, RequestBuilder};

use serde_json::Value;
use std::fs::File;
use seed::header;
use url::Url;

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
        token:String
    ) -> Result<http::Response<String>, ClientErr> {
        // No version on the response when using from client but works when using from server (backend)
        let version = request.version().clone();
        let body = request.body().clone();
        let method = request.method().clone();
        let url = request.uri().to_string();

        let req = match method {
            Method::GET => Client::new().get(url),
            Method::POST => Client::new().post(url),
            Method::PUT => Client::new().put(url),
            Method::DELETE => Client::new().delete(url),
            m @ _ => return Err(ClientErr::HttpClient(format!("invalid method {}", m))),
        };
          let bear_token = "Bearer ".to_owned() +&token;
        let resp = req
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", bear_token)
            .header("Content-Length", "0")
            .send()
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
        println!("{}", content);

        for header in headers.iter() {
            build = build.header(header.0, header.1);
        }

        build
            .status(status_code)
            .version(version)
            .body(content)
            .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))
    }

    async fn auth_request(
        &self,
        request: http::Request<String>,
    ) -> Result<http::Response<String>, ClientErr> {
        // No version on the response when using from client but works when using from server (backend)
        let version = request.version().clone();
        let body = request.body().clone();
        let method = request.method().clone();
        let url = request.uri().to_string();
        let token = "Bearer ".to_owned() + &body;
        let req = match method {
            Method::GET => Client::new().get(url),
            Method::POST => Client::new().post(url),
            m @ _ => return Err(ClientErr::HttpClient(format!("invalid method {}", m))),
        };

        let resp = req
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", token)
            .send()
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
        println!("{}", content);

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
        token: String,
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
        let resp = if token.is_empty() {
            req.header("X-Restli-Protocol-Version", "2.0.0")
                .json(&body)
                .send()
                .await
                .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?
        } else {
            let bear_token = "Bearer ".to_owned() + &token;
            req.header("X-Restli-Protocol-Version", "2.0.0")
                .header("Content-Type", "application/json")
                .header("Authorization", bear_token)
                .json(&body)
                .send()
                .await
                .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?
        };

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
        request: http::Request<Vec<u8>>,
        token: String,
    ) -> Result<http::Response<String>, ClientErr> {
        //et req: Request = request.try_into().unwrap();
        // let req: RequestBuilder = generic_req::<FormData>(request).unwrap();
        let url = request.uri().to_string();
        let method = request.method().clone();
        let body = request.body();
        let bear_token = "Bearer ".to_owned() + &token;

        let req = match method {
            Method::GET => Client::new().get(url),
            Method::POST => Client::new().post(url),
            Method::PUT => Client::new().put(url),
            m @ _ => return Err(ClientErr::HttpClient(format!("invalid method {}", m))),
        };
        // let file_size = body.metadata().unwrap().len().clone();
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::BufReader;
        let resp = if method == Method::PUT {
            println!("printing here ");
            req.header("Content-Type","application/octet-stream" )
                .header("Authorization", bear_token)
                .body(Body::from(body.to_vec()))
                .send()
                .await
                .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?
        } else {
            req.header("Authorization", bear_token)
                .body(Body::from(body.to_vec()))
                .send()
                .await
                .map_err(|e| ClientErr::HttpClient(format!("{:?}", e)))?
        };

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
