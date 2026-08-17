pub mod auth_header;
pub mod base_uri;
pub mod cache;
pub mod extra_headers;
#[cfg(feature = "retry")]
#[cfg_attr(docsrs, doc(cfg(feature = "retry")))]
pub mod retry;
