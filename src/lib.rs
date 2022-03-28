pub mod account;
pub mod client;
pub mod http_clinent;
mod login;
pub mod share;
pub mod video;
pub mod  post;

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::login::prelude::*;
    pub use crate::share::post::*;
    pub use crate::video::post::*;
    pub use crate::post::publish::*;
}
