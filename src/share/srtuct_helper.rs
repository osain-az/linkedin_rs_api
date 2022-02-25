// https://github.com/serde-rs/serde/issues/868
// https://users.rust-lang.org/t/need-help-with-serde-deserialize-with/18374

use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Deserializer,
};
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Uploading {
    #[serde(
        rename(deserialize = "com"),
        deserialize_with = "deserialize_something"
    )]
    pub uploadUrl: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageInitResponse {
    pub value: Values,
}
#[derive(Serialize, Deserialize)]
pub struct Values {
    pub mediaArtifact: String,
    pub asset: String,
    pub uploadMechanism: Uploading,
}

fn main() {
    let json_string = r#"
        {
            "com": {
                "linkedin": {
                    "digitalmedia" : {
                        "uploading": {
                          "MediaUploadHttpRequest":{
                            "uploadUrl": 1234
                            }
                        }
                    }
                }
            }
        }
    "#;
    let sws = serde_json::from_str::<Uploading>(json_string).unwrap();
    println!("{sws:#?}");
}

fn deserialize_something<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let mut json: Value = de::Deserialize::deserialize(deserializer)?;
    let smth =
        json["linkedin"]["digitalmedia"]["uploading"]["MediaUploadHttpRequest"]["uploadUrl"].take();
    serde_json::from_value(smth).map_err(de::Error::custom)
}
