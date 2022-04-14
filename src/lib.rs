pub mod account;
pub mod client;
pub mod http_clinent;
mod login;
pub mod post;
pub mod share;
pub mod video;

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::login::prelude::*;
    pub use crate::post::publish::*;
    pub use crate::share::post::*;
    //  pub use crate::video::post::*;
}
