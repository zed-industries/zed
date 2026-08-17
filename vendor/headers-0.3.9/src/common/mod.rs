//! A Collection of Header implementations for common HTTP Headers.
//!
//! ## Mime
//!
//! Several header fields use MIME values for their contents. Keeping with the
//! strongly-typed theme, the [mime](https://docs.rs/mime) crate
//! is used, such as `ContentType(pub Mime)`.

//pub use self::accept_charset::AcceptCharset;
//pub use self::accept_encoding::AcceptEncoding;
//pub use self::accept_language::AcceptLanguage;
pub use self::accept_ranges::AcceptRanges;
//pub use self::accept::Accept;
pub use self::access_control_allow_credentials::AccessControlAllowCredentials;
pub use self::access_control_allow_headers::AccessControlAllowHeaders;
pub use self::access_control_allow_methods::AccessControlAllowMethods;
pub use self::access_control_allow_origin::AccessControlAllowOrigin;
pub use self::access_control_expose_headers::AccessControlExposeHeaders;
pub use self::access_control_max_age::AccessControlMaxAge;
pub use self::access_control_request_headers::AccessControlRequestHeaders;
pub use self::access_control_request_method::AccessControlRequestMethod;
pub use self::age::Age;
pub use self::allow::Allow;
pub use self::authorization::Authorization;
pub use self::cache_control::CacheControl;
pub use self::connection::Connection;
pub use self::content_disposition::ContentDisposition;
pub use self::content_encoding::ContentEncoding;
//pub use self::content_language::ContentLanguage;
pub use self::content_length::ContentLength;
pub use self::content_location::ContentLocation;
pub use self::content_range::ContentRange;
pub use self::content_type::ContentType;
pub use self::cookie::Cookie;
pub use self::date::Date;
pub use self::etag::ETag;
pub use self::expect::Expect;
pub use self::expires::Expires;
//pub use self::from::From;
pub use self::host::Host;
pub use self::if_match::IfMatch;
pub use self::if_modified_since::IfModifiedSince;
pub use self::if_none_match::IfNoneMatch;
pub use self::if_range::IfRange;
pub use self::if_unmodified_since::IfUnmodifiedSince;
//pub use self::last_event_id::LastEventId;
pub use self::last_modified::LastModified;
//pub use self::link::{Link, LinkValue, RelationType, MediaDesc};
pub use self::location::Location;
pub use self::origin::Origin;
pub use self::pragma::Pragma;
//pub use self::prefer::{Prefer, Preference};
//pub use self::preference_applied::PreferenceApplied;
pub use self::proxy_authorization::ProxyAuthorization;
pub use self::range::Range;
pub use self::referer::Referer;
pub use self::referrer_policy::ReferrerPolicy;
pub use self::retry_after::RetryAfter;
pub use self::sec_websocket_accept::SecWebsocketAccept;
pub use self::sec_websocket_key::SecWebsocketKey;
pub use self::sec_websocket_version::SecWebsocketVersion;
pub use self::server::Server;
pub use self::set_cookie::SetCookie;
pub use self::strict_transport_security::StrictTransportSecurity;
pub use self::te::Te;
pub use self::transfer_encoding::TransferEncoding;
pub use self::upgrade::Upgrade;
pub use self::user_agent::UserAgent;
pub use self::vary::Vary;
//pub use self::warning::Warning;

#[cfg(test)]
fn test_decode<T: ::Header>(values: &[&str]) -> Option<T> {
    use HeaderMapExt;
    let mut map = ::http::HeaderMap::new();
    for val in values {
        map.append(T::name(), val.parse().unwrap());
    }
    map.typed_get()
}

#[cfg(test)]
fn test_encode<T: ::Header>(header: T) -> ::http::HeaderMap {
    use HeaderMapExt;
    let mut map = ::http::HeaderMap::new();
    map.typed_insert(header);
    map
}

#[cfg(test)]
macro_rules! bench_header {
    ($mod:ident, $ty:ident, $value:expr) => {
        #[cfg(feature = "nightly")]
        mod $mod {
            use super::$ty;
            use HeaderMapExt;

            #[bench]
            fn bench_decode(b: &mut ::test::Bencher) {
                let mut map = ::http::HeaderMap::new();
                map.append(
                    <$ty as ::Header>::name(),
                    $value.parse().expect("HeaderValue::from_str($value)"),
                );
                b.bytes = $value.len() as u64;
                b.iter(|| {
                    map.typed_get::<$ty>().unwrap();
                });
            }

            #[bench]
            fn bench_encode(b: &mut ::test::Bencher) {
                let mut map = ::http::HeaderMap::new();
                map.append(
                    <$ty as ::Header>::name(),
                    $value.parse().expect("HeaderValue::from_str($value)"),
                );
                let typed = map.typed_get::<$ty>().unwrap();
                b.bytes = $value.len() as u64;
                b.iter(|| {
                    map.typed_insert(typed.clone());
                    map.clear();
                });
            }
        }
    };
}

//mod accept;
//mod accept_charset;
//mod accept_encoding;
//mod accept_language;
mod accept_ranges;
mod access_control_allow_credentials;
mod access_control_allow_headers;
mod access_control_allow_methods;
mod access_control_allow_origin;
mod access_control_expose_headers;
mod access_control_max_age;
mod access_control_request_headers;
mod access_control_request_method;
mod age;
mod allow;
pub mod authorization;
mod cache_control;
mod connection;
mod content_disposition;
mod content_encoding;
//mod content_language;
mod content_length;
mod content_location;
mod content_range;
mod content_type;
mod cookie;
mod date;
mod etag;
mod expect;
mod expires;
//mod from;
mod host;
mod if_match;
mod if_modified_since;
mod if_none_match;
mod if_range;
mod if_unmodified_since;
//mod last_event_id;
mod last_modified;
//mod link;
mod location;
mod origin;
mod pragma;
//mod prefer;
//mod preference_applied;
mod proxy_authorization;
mod range;
mod referer;
mod referrer_policy;
mod retry_after;
mod sec_websocket_accept;
mod sec_websocket_key;
mod sec_websocket_version;
mod server;
mod set_cookie;
mod strict_transport_security;
mod te;
mod transfer_encoding;
mod upgrade;
mod user_agent;
mod vary;
//mod warning;
