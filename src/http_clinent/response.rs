use std::ops::Deref;

use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};
use serde_json::value::Value;

use crate::http_clinent::errors::{ClientErr, LinkedinAPiError};

///
pub(crate) fn deserialize_response<T>(text: &str) -> Result<T, ClientErr>
where
    T: DeserializeOwned,
{
    let response: Response<T> = serde_json::from_str(text)?;
    Ok(Into::<Result<T, LinkedinAPiError>>::into(response)?)
}

#[derive(Debug)]
pub(crate) enum Response<T> {
    Ok(T),
    Err(LinkedinAPiError),
}

impl<T> Into<Result<T, LinkedinAPiError>> for Response<T> {
    fn into(self) -> Result<T, LinkedinAPiError> {
        match self {
            Response::Ok(success) => Ok(success),
            Response::Err(err) => Err(err),
        }
    }
}

impl<'de, T> Deserialize<'de> for Response<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = serde_json::Map::deserialize(deserializer)?;
        let error = map
            .get("error")
            .map_or_else(|| Ok(false), Deserialize::deserialize)
            .map_err(de::Error::custom)?;
        let rest = Value::Object(map);

        if error {
            LinkedinAPiError::deserialize(rest)
                .map(Response::Err)
                .map_err(de::Error::custom)
        } else {
            T::deserialize(rest)
                .map(Response::Ok)
                .map_err(de::Error::custom)
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct ClientResult<T> {
    #[serde(rename = "result")]
    result: T,
}

impl<T> ClientResult<T> {
    pub fn unwrap(self) -> T {
        self.result
    }
}

impl<T> Deref for ClientResult<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.result
    }
}
