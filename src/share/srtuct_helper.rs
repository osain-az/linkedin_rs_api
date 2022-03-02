
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
pub struct ImageInitResponse {
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
