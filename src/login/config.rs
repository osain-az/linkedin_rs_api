use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    /// The Linkedin url preamble for the oath dialog.
    pub linkedin_oath_url: String,

    /// The ID of your app, found in your app's dashboard.
    pub client_id: String,
    pub client_secret: String,
    /// The URL that you want to redirect the person logging in back to.
    pub redirect_uri: String,
}

impl Config {
    pub fn new(linkedin_oath_url: String, client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Config { linkedin_oath_url, client_id, client_secret, redirect_uri }
    }
}

impl Config {
    pub fn linkedin_oath_url(&self) -> &str {
        &self.linkedin_oath_url
    }
    pub fn client_id(&self) -> &str {
        &self.client_id
    }
    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }
    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }
}
