
use serde::{
    Serialize,
    de::{self, DeserializeOwned},
    Deserialize, Deserializer,
};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct UploadingUrl {
    pub uploadUrl: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaUploadInitResponse {
    pub value: Values,
}
#[derive( Deserialize,Serialize, Debug)]
pub struct Values {
    pub mediaArtifact: String,
    pub asset: String,
    pub uploadMechanism:UploadMechanism
}

#[derive(Deserialize, Serialize, Debug)]
 pub struct UploadMechanism {
    #[serde(rename = "com.linkedin.digitalmedia.uploading.MediaUploadHttpRequest")]
   pub  media_upload_http_request: UploadingUrl
}




#[derive(Serialize, Deserialize, Debug)]
pub struct VideoPartUploadInitResponse {
    pub value: PartValues,
}

#[derive( Deserialize,Serialize, Debug)]
pub struct PartValues {
    pub mediaArtifact: String,
    pub asset: String,
    pub uploadMechanism:PartUploadMechanism
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct PartUploading {
    pub partUploadRequests: Vec<UploadingValues>,
    metadata:String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PartUploadMechanism {
    #[serde(rename = "com.linkedin.digitalmedia.uploading.MultipartUpload")]
    pub part_upload: PartUploading,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UploadingValues{
    url: String,
    urlExpiresAt:String,
    BytesRange:BytesRangeValues
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BytesRangeValues{
    lastByte:i16,
    firstByte: i16
}

