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
pub struct Token {
    /// access_token is used for API calls and it contains response data such as
    /// scopes
    pub acces_token: String,
    /// A string value created by your app to maintain state between the request
    /// and callback.
    pub expires_at: String,
}

impl Token {
    pub fn access_tokens(self) -> Self {
        self
    }
    pub fn expires_at(self) -> Self{
               self
    }
}



