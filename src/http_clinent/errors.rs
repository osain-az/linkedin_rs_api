use std::fmt;

use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientErr {
    #[error("facebook error:  {0}")]
    LinkedinError(String),
    #[error("Error from server: {0}")]
    Linkedin(#[from] LinkedinAPiError),
    #[error("Error from serde: {0}")]
    Serde(#[from] serde_json::error::Error),
    #[error("HTTP client error: {0}")]
    HttpClient(String),
}

#[derive(Deserialize, Debug, Error)]
pub struct LinkedinAPiError {
    pub(crate) code: u16,
    #[serde(rename = "errorNum")]
    pub(crate) error_num: u16,
    #[serde(rename = "errorMessage")]
    pub(crate) message: String,
}

impl fmt::Display for LinkedinAPiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.message, self.error_num)
    }
}

impl LinkedinAPiError {
    /// Get the HTTP status code of an error response.

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn error_num(&self) -> u16 {
        self.error_num
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
