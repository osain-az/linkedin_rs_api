mod linkedin_api;

//use facebook_api_rs::prelude::{Accounts, Client, Config, Data, RedirectURL, Token};
//use facebook_api_rs::prelude::errors::ClientErr;
//e facebook_api_rs::prelude::video::{ContentCategory, FinalResponeResumableUpload ,VideoParams};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::io::{Cursor, Read};
//buffer.to_vec()
use actix_multipart_rfc7578::client::multipart;
use actix_rt::System;
use awc::error::SendRequestError;
//use awc::{Client as Awc_Web, ClientResponse};
use std::time::Duration;
use linkedin_api_rs::http_clinent::errors::ClientErr;
use linkedin_api_rs::prelude::{Client, Config, RedirectURL, Token};


use serde::{
    Serialize,
    de::{self, DeserializeOwned},
    Deserialize, Deserializer,
};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct Uploading {

    #[serde(rename(deserialize = "com.linkedin.digitalmedia.uploading.MediaUploadHttpRequest", serialize = "uploadUrl"))]
    uploadUrl:String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageInitResponse {
    pub value: Values,
}
#[derive( Deserialize,Serialize, Debug)]
pub struct Values {
    pub mediaArtifact: String,
    pub asset: String,
    pub uploadMechanism: Uploading,
}



#[tokio::main]
 async fn main() {

   //let result =  linkedin_api::linkedin_post_by_text().await;
  //let result =  linkedin_api::linkedin_post_by_text().await;
   let result =  linkedin_api::linkedin_video_status().await;
    if result.is_ok(){
        result.unwrap();
    }else {
        let etst =  result.unwrap_err();
        println!("testing{:?}",etst)
    }}