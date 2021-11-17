//! An access token is an opaque string that identifies a user, app, or Page
//! and can be used by the app to make graph API calls.
//!
//! When someone connects with an app using Facebook Login and approves the
//! request for permissions, the app obtains an access token that provides
//! temporary, secure access to Facebook APIs. Access tokens are obtained via a
//! number of methods.
//! form more information check <https://developers.facebook.com/docs/facebook-login/access-tokens/?translation>

use chrono::prelude::*;
use seed::fetch::fetch;
use seed::prelude::IndexMap;
use seed::prelude::{Method, Request};
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

/// The following struct is used to describe a token which may be retrieved from
/// the login flow of Facebook.
#[derive(Deserialize, Default, Clone, Debug, Serialize)]
pub struct LoginResponse {
    /// access_token is used for API calls and it contains response data such as
    /// scopes
    pub code: String,
    /// A string value created by your app to maintain state between the request
    /// and callback.
    pub state: String,
}

impl LoginResponse {
    pub fn access_code(self) -> Self {
        self
    }
    pub fn new(state:String, access_code:String) -> LoginResponse {
        LoginResponse{
                 code: access_code,
                 state
        }
    }

    pub fn extract_user_tokens(hash: String) -> LoginResponse {

        let query = extract_query_fragments(hash);
        let iterations = query.iter();

        let mut response = LoginResponse::default();

        for e in iterations {
            match e.0.as_str() {
                "code" => {
                    response.code = e.1.to_string();
                }
                "state" => {
                    response.state = e.1.to_string();
                }
                _ => panic!("unknown field: {}", e.0.as_str()),
            }
        }
        response
    }
}
/// Note: when you try to debug a long live token, the expires_at value will
/// be  "expires_at: 0" which means it never exoires for information
/// check facbook deocumentation


/// Extract data from  from the url fragment and return an `IndexMap`
/// for the Enum Variant.
/// # Panics
/// The function will panic a key that has no value.
/// # Warns
/// with no query. Theses choices are opinionated for now.
pub fn extract_query_fragments(hash: String) -> IndexMap<String, String> {
    let mut query: IndexMap<String, String> = IndexMap::new();
    let mut  key_value: Vec<&str> = hash.split('&').collect();
    log!(key_value);

    for pair in key_value {
        let mut sub = pair.split('=');
        let key = sub.next().unwrap_or_else(|| {
            panic!(
                "we should have a key for the parameter key but got {}",
                hash
            )
        });
        let value = sub
            .next()
            .unwrap_or_else(|| panic!("we should have a value for the key but got {}", hash));
        query.insert(key.to_string(), value.to_string());
    }
    query
}

