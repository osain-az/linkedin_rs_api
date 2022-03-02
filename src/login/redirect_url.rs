#![allow(dead_code)]

use crate::login::config::Config;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use urlencoding::encode;

/// Contains the Config struct and is used for building the login flow
#[derive(Deserialize, Debug, Clone,Default, Serialize)]
pub struct RedirectURL {
    /// The Facebook url preamble for the oath dialog.
   pub  linkedin_oath_url: String,

    /// The ID of your app, found in your app's dashboard.
    client_id: String,
    client_secret: String,
    /// The URL that you want to redirect the person logging in back to.
    redirect_uri: String,

    /// A string value created by your app to maintain state between the request
    /// and callback.
    state: String,

    /// Determines whether the response data included when the redirect back to
    /// the app occurs is in URL parameters or fragments.
    response_type: String,
    grant_type: String,

    /// A comma or space separated list of Permissions to request from the
    /// person.
    scope: String,

    // response code
    /// The full url of the login flow.
   pub  full_url: String,
}

impl RedirectURL {
    /// Constructor of the RedirectURL
    /// facebook_oath_url, client_id, and redirect_uri are retrieved from the
    /// config.json file. which the user has to configure.
    /// A random state is provided or the user may chose to create their own
    /// state. response_type has to be configured depending on the use case
    /// of the application, or else the response will default to code upon
    /// the login flow redirect. scope is optional, but inclusion must
    /// fulfill a valid scope.
    pub fn new(config: Config) -> RedirectURL {
        RedirectURL::default()
            .add_linkedin_oath_url(&config.linkedin_oath_url())
            .add_client_id(&config.client_id())
            .add_redirect_uri(&config.redirect_uri())
            .add_random_state()
            .add_response_type("")
            //MUST ADD A VALID SCOPE!
            .add_scope(&[])
            .add_full_url()
            //This is used when exchanging code for token
            .add_client_secret(config.client_secret())
    }

    pub fn add_client_id(mut self, client_id: &str) -> Self {
        self.client_id = client_id.to_string();
        self
    }
    pub fn add_client_secret(mut self, client_secret: &str) -> Self {
        self.client_secret = client_secret.to_string();
        self
    }

    pub fn add_linkedin_oath_url(mut self, url: &str) -> Self {
        self.linkedin_oath_url = url.to_string();
        self
    }

    pub fn add_redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.redirect_uri = redirect_uri.to_string();
        self
    }

    pub fn add_state(mut self, state: &str) -> Self {
        self.state = state.to_string();
        self
    }

    pub fn add_response_type(mut self, response_type: &str) -> Self {
        self.response_type = response_type.to_string();
        self
    }

    pub fn add_scope(mut self, scope: &[String]) -> Self {
        self.scope = handle_scope(Vec::from(scope));
        self
    }

    pub fn add_random_state(mut self) -> Self {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        self.state = rand_string;
        self
    }

    /// Builds the redirect url for the login flow as a string so it may be
    /// passed through a GET request
    pub fn build_login_url(&mut self) -> String {
        let full_url = "".to_string()
            + &self.linkedin_oath_url
            + "?response_type="
            + &self.response_type
            + "&client_id="
            + &self.client_id
            + "&redirect_uri="
            + &self.redirect_uri
            + "&state="
            + &*self.state
            + "&scope="
            + &self.scope;
        full_url
    }
     fn build_token_exchange_url(&mut self, code:String) -> String {
        let full_url = "".to_string()
            + &self.linkedin_oath_url.replace("authorization","accessToken")
            + "?grant_type=authorization_code"
            +"&code="
            +&code
            + "&redirect_uri="
            + &self.redirect_uri
            + "&client_id="
            + &self.client_id
            +"&client_secret="
            + &self.client_secret;
        full_url
    }
    pub fn add_full_url(mut self) -> Self {
        self.full_url = self.build_login_url();
        self
    }
    pub fn token_exchange_url(mut self, code:String) -> Self {
        self.full_url = self.build_token_exchange_url(code);
        self
    }

    pub fn linkedin_oath_url(&self) -> &String {
        &self.linkedin_oath_url
    }

    pub fn client_id(&self) -> &String {
        &self.client_id
    }

    pub fn redirect_uri(&self) -> &String {
        &self.redirect_uri
    }

    pub fn state(&self) -> &String {
        &self.state
    }

    pub fn response_type(&self) -> &String {
        &self.response_type
    }

    pub fn scope(self) -> String {
        self.scope
    }

    pub fn get_full_url(&self) -> &String {
        &self.full_url
    }

    fn set_linkedin_oath_url(&mut self, linkedin_oath_url: String) {
        self.linkedin_oath_url = linkedin_oath_url ;
    }

    fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    fn set_redirect_uri(&mut self, redirect_uri: String) {
        self.redirect_uri = redirect_uri;
    }

    fn set_state(&mut self, state: String) {
        self.state = state;
    }

    fn set_response_type(&mut self, response_type: String) {
        self.response_type = response_type;
    }

    fn set_scope(&mut self, scope: Vec<String>) {

        self.scope = handle_scope(scope);
    }
}


fn  handle_scope(scope:Vec<String>) -> String{
    let scope_count = scope.len();
    let mut scopes_string = "".to_string();
    for (count, scope_type) in scope.into_iter().enumerate() {
        if count < scope_count - 1 {
            scopes_string = scopes_string.to_owned() + &scope_type + " ";
        } else {
            scopes_string = scopes_string.to_owned() + &scope_type; // remove the comma in the last filed
        }
    }
    let scopes = encode(&scopes_string);
    scopes.to_string()

}