#[cfg(feature = "client")]
mod async_body;
#[cfg(feature = "client")]
pub mod github;
#[cfg(feature = "client")]
pub mod github_download;
#[cfg(feature = "client")]
mod http_client;

pub use anyhow::{Result, anyhow};
pub use http::{self, Method, Request, Response, StatusCode, Uri, request::Builder};
pub use url::{Host, Url};

#[cfg(feature = "client")]
pub use crate::async_body::{AsyncBody, Inner};
#[cfg(feature = "client")]
pub use crate::http_client::*;
