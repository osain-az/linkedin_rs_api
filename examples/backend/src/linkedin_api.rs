use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::io::{Cursor, Read};

use linkedin_api_rs::http_clinent::errors::ClientErr;
use linkedin_api_rs::prelude::{Client, Config, RedirectURL, Token};
use linkedin_api_rs::video::utils::{InitVideoParams, UploadingVideos};


//check the frontend example on login and requesting of code
async fn exchange_code_to_token() -> Result<Token,ClientErr>{
     // The code has to request at the frontend

    let code = "xxxxxxxxxxxxx" ;
    let config =       Config::new(
        "https://www.linkedin.com/oauth/v2/accessToken".to_string(),
        "xxxxx".to_string(),
        "xxxxx".to_string(),
        "http://localhost:8001/".to_string());
    let built_url =  RedirectURL::new(config).token_exchange_url(code.to_string() ).full_url;

    let resp =   Client::accss_token(built_url.to_string().clone())
     .get()
      .await?;
     Ok(resp)
}//

async fn linkedin_token_authenticate() -> Result<String,ClientErr>{
    let token = "xxxxxxx";
    let resp =  Client::default().token(token.to_string()).authenticate().await?;
    Ok(resp)
}

// this let you share article
async fn post_by_share_article()-> Result<String, ClientErr>{
    let person_id = "xxxx"; //user id
    let token = "xxxxxxxxx";
    let post_message = "What we love about nature is that all of its  act are naturally and amazing "; //  Our message
    // link to article you want to share
    let media_url = "https://jackashepherd.medium.com/27-photos-of-natures-most-sublimely-impressive-goofs-fba0f19eb61d".to_string(); // some aticle
    let resp  = Client::default().share(person_id.to_string(),token.to_string()).post_article(post_message.to_string(),media_url)
        .await?;
        Ok(resp)
}

// this let you upload and post an image
pub async fn link_upload_and_post_image()  -> Result<String, ClientErr>{
    let person_id = "xxxx"; //user id
    let token = "xxxxxxxxx";
    let post_message = "The web that connect us all. The spider web  ";

    let mut file = File::open("/home/azibodusio/Downloads/spider-g5072ae4ca_1920.jpg").unwrap();

    let image_description  = "The spider web that govern all ". to_string();
    let image_title = "Spider web ".to_string();

    let resp  = Client::default().share(person_id.to_string(), token.to_string())
        .post_with_image_upload(
            file,
            post_message.to_string(),
            image_title,
            image_description
        )
        .await?;
    Ok(resp)
}

pub async fn linkedin_post_by_text() -> Result<String, ClientErr>{

    let person_id = "xxxx"; //user id
    let token = "xxxxxxxxx";
    let post_message = "THe road we drive, drives  everything  ";

    let resp  = Client::default().share(person_id.to_string(), token.to_string())
        .post_text(post_message.to_string())
        .await?;
    Ok(resp)
}

pub async fn linkedin_post_by_tennnxt() -> Result<String, ClientErr>{
     let person_id = "xxxx"; //user id
    let token = "xxxxxxxxx";
    let post_message = "THe road we drive, drives  everything  ";
     let video_file =File::open("/home/azibodusio/Downloads/Pexels Videos 2887.mp4").unwrap();

    let video_param = InitVideoParams::new("FEED_VIDEO".to_string(), video_file.metadata().unwrap().len(), false, false, person_id.to_string());
    let uploading_file = UploadingVideos::new(Some(video_file), None,None);

    let resp  = Client::default().publish_video(person_id.to_string(), token.to_string())
        .publish_video_upload(post_message.to_string(),"Choose the road".to_string(),video_param,uploading_file)
        .await?;
    Ok(resp)
}

// THis let you publish or share a video you have already upload on linkedin
pub async fn linkedin_video_publish_by_id() -> Result<String, ClientErr>{
    let person_id = "xxxx"; //user id
    let token = "xxxxxxxxx";

    let post_message = "THe road we drive, drives  everything  ";

     let media = "urn:li:video:C4D05AQF1ugKDaDvAIQ".to_string(); // the video id from linkedin

    let resp  = Client::default().publish_video(person_id.to_string(), token.to_string())
        .publish_media_by_id(post_message.to_string(),media,"the space of time ".to_string())
        .await?;
    Ok(resp)
}


//check video status
pub async fn linkedin_video_status() -> Result<String, ClientErr>{
    let person_id = "xxxx"; //user id
    let token = "xxxxxxxxx";
     let media = "urn:li:video:C4D05AQGpCgOmQIMfbw".to_string();
    let resp  = Client::default().video_upload(person_id.to_string(), token.to_string())
        .upload_status(media)
        .await?;
    Ok("".to_string())
}

// This will upload your video and attempt to share it.
//This uses new linkedin api
pub async fn linkedin_upload_and_share_video() -> Result<String, ClientErr>{
    let person_id = "xxxx"; //user id
    let token = "xxxxxxxxx";

    let post_message = "THe road we drive, drives  everything  ";
     let video_file =File::open("/home/azibodusio/Downloads/Satellite - 37713.mp4").unwrap();
    let video_param = InitVideoParams::new("FEED_VIDEO".to_string(), video_file.metadata().unwrap().len(), false, false, person_id.to_string());

    let uploading_file = UploadingVideos::new(Some(video_file), None,None);
     let media = "urn:li:video:C4D05AQGpCgOmQIMfbw".to_string();

    let resp  = Client::default().video_upload(person_id.to_string(), token.to_string())
        .upload_status(media)
        .await?;

    Ok("".to_string())
}
