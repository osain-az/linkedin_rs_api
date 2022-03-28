use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Deserializer, Serialize,
};
use std::fs::File;

#[derive(Default)]
pub struct UploadingVideos {
    pub main_video: Option<File>,
    pub video_caption: Option<File>,
    pub video_thumbnail: Option<File>,
}

impl UploadingVideos {
    pub fn new(
        main_video: Option<File>,
        video_caption: Option<File>,
        video_thumbnail: Option<File>,
    ) -> Self {
        UploadingVideos {
            main_video,
            video_caption,
            video_thumbnail,
        }
    }
    pub fn set_main_video(&mut self, main_video: Option<File>) {
        self.main_video = main_video;
    }
    pub fn set_video_caption(&mut self, video_caption: Option<File>) {
        self.video_caption = video_caption;
    }
    pub fn set_video_thumbnail(&mut self, video_thumbnail: Option<File>) {
        self.video_thumbnail = video_thumbnail;
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct  InitializeUploadRequest{
    pub owner: String,
    pub purpose: String,
    pub fileSizeBytes: u64,
    pub uploadCaptions: bool,
    pub uploadThumbnail: bool,
}

impl InitializeUploadRequest {
    pub fn new(owner: String, purpose: String, fileSizeBytes: u64, uploadCaptions: bool, uploadThumbnail: bool) -> Self {
        InitializeUploadRequest { owner, purpose, fileSizeBytes, uploadCaptions, uploadThumbnail }
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct InitVideoParams {
    initializeUploadRequest : InitializeUploadRequest
}


impl InitVideoParams {
    pub fn new(
        purpose: String,
        fileSizeBytes: u64,
        uploadCaptions: bool,
        uploadThumbnail: bool,
        owner: String,
    ) -> Self {

        InitVideoParams {
          initializeUploadRequest: InitializeUploadRequest::new(
              owner,
              purpose,
              fileSizeBytes,
              uploadCaptions,
              uploadThumbnail,
          )
        }
    }

    pub fn set_owner(&mut  self, onwer: String) {
        self.initializeUploadRequest.owner = onwer;
    }
    pub fn set_purpose(&mut self, purpose: String) {
        self.initializeUploadRequest.purpose = purpose;

    }
    pub fn set_file_size(&mut self, file_size: u64){
        self.initializeUploadRequest.fileSizeBytes = file_size;
    }
    pub fn set_is_upload_caption(&mut self, is_upload_caption: bool){
        self.initializeUploadRequest.uploadCaptions =is_upload_caption;
    }
    pub fn set_is_upload_thumbnails(&mut self, is_upload_thumbnails: bool) {
        self.initializeUploadRequest.uploadThumbnail = is_upload_thumbnails;
    }

}

#[derive(Deserialize, Clone, Debug)]
pub struct InitVideoResponse {
    pub value: Values,
}

#[derive(Deserialize, Clone,Debug)]
pub struct Values {
    pub uploadUrlsExpireAt: u64,
    pub video: String,
    pub uploadInstructions: Vec<UploadInstructions>,
    pub uploadToken: String,
    #[serde(default = "default_field_val")]
    pub captionsUploadUrl: String,
    #[serde(default = "default_field_val")]
    pub thumbnailUploadUrl: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UploadInstructions {
    pub lastByte: u64,
    pub firstByte: u64,
    pub uploadUrl: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VideoUploadStatus {
    owner: String,
    id: String,
    status: String,
}

impl VideoUploadStatus {
    pub fn owner(&self) -> &str {
        &self.owner
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn status(&self) -> &str {
        &self.status
    }
}

fn default_field_val() -> String{
    "".to_string()
}