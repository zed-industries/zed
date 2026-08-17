//! # Octocrab: A modern, extensible GitHub API client.
//! Octocrab is an third party GitHub API client, allowing you to easily build
//! your own GitHub integrations or bots. `octocrab` comes with two primary
//! set of APIs for communicating with GitHub, a high level strongly typed
//! semantic API, and a lower level HTTP API for extending behaviour.
//!
//! ## Semantic API
//! The semantic API provides strong typing around GitHub's API, as well as a
//! set of [`models`] that maps to GitHub's types. Currently the following
//! modules are available.
//!
//! - [`actions`] GitHub Actions
//! - [`activity`] GitHub Activity
//! - [`apps`] GitHub Apps
//! - [`checks`] GitHub Checks
//! - [`code_scannings`] Code Scanning
//! - [`commits`] GitHub Commits
//! - [`current`] Information about the current user.
//! - [`events`] GitHub Events
//! - [`gists`] Gists
//! - [`gitignore`] Gitignore templates
//! - [`Octocrab::graphql`] GraphQL.
//! - [`issues`] Issues and related items, e.g. comments, labels, etc.
//! - [`licenses`] License Metadata.
//! - [`markdown`] Rendering Markdown with GitHub
//! - [`orgs`] GitHub Organisations
//! - [`projects`] GitHub Projects
//! - [`pulls`] Pull Requests
//! - [`ratelimit`] Rate Limiting
//! - [`repos`] Repositories
//! - [`repos::forks`] Repository forks
//! - [`repos::releases`] Repository releases
//! - [`search`] Using GitHub's search.
//! - [`teams`] Teams
//! - [`users`] Users
//! - [`classroom`] GitHub Classroom
//! - [`workflows`] GitHub Workflows
//!
//! #### Getting a Pull Request
//! ```no_run
//! # async fn run() -> octocrab::Result<()> {
//! // Get pull request #404 from `octocrab/repo`.
//! let pr = octocrab::instance().pulls("octocrab", "repo").get(404).await?;
//! # Ok(())
//! # }
//! ```
//!
//! All methods with multiple optional parameters are built as `Builder`
//! structs, allowing you to easily specify parameters.
//!
//! #### Listing issues
//! ```no_run
//! # async fn run() -> octocrab::Result<()> {
//! use octocrab::{models, params};
//!
//! let octocrab = octocrab::instance();
//! // Returns the first page of all issues.
//! let mut page = octocrab.issues("octocrab", "repo")
//!     .list()
//!     // Optional Parameters
//!     .creator("octocrab")
//!     .state(params::State::All)
//!     .per_page(50)
//!     .send()
//!     .await?;
//!
//! // Go through every page of issues. Warning: There's no rate limiting so
//! // be careful.
//! let results = octocrab.all_pages::<models::issues::Issue>(page).await?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## HTTP API
//! The typed API currently doesn't cover all of GitHub's API at this time, and
//! even if it did GitHub is in active development and this library will
//! likely always be somewhat behind GitHub at some points in time. However that
//! shouldn't mean that in order to use those features that you have to now fork
//! or replace `octocrab` with your own solution.
//!
//! Instead `octocrab` exposes a suite of HTTP methods allowing you to easily
//! extend `Octocrab`'s existing behaviour. Using these HTTP methods allows you
//! to keep using the same authentication and configuration, while having
//! control over the request and response. There is a method for each HTTP
//! method `get`, `post`, `patch`, `put`, `delete`, all of which accept a
//! relative route and a optional body.
//!
//! ```no_run
//! # async fn run() -> octocrab::Result<()> {
//! let user: octocrab::models::Author = octocrab::instance()
//!     .get("/user", None::<&()>)
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! Each of the HTTP methods expects a body, formats the URL with the base
//! URL, and errors if GitHub doesn't return a successful status, but this isn't
//! always desired when working with GitHub's API, sometimes you need to check
//! the response status or headers. As such there are companion methods `_get`,
//! `_post`, etc. that perform no additional pre or post-processing to
//! the request.
//!
//! ```no_run
//! # use http::Uri;
//! # async fn run() -> octocrab::Result<()> {
//! let octocrab = octocrab::instance();
//! let response = octocrab
//!     ._get("https://api.github.com/organizations")
//!     .await?;
//!
//! // You can also use `Uri::builder().authority("<my custom base>").path_and_query("<my custom path>")` if you want to customize the base uri and path.
//! let response =  octocrab
//!     ._get(Uri::builder().path_and_query("/organizations").build().expect("valid uri"))
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! You can use the those HTTP methods to easily create your own extensions to
//! `Octocrab`'s typed API. (Requires `async_trait`).
//! ```
//! use octocrab::{Octocrab, Page, Result, models};
//!
//! #[async_trait::async_trait]
//! trait OrganisationExt {
//!   async fn list_every_organisation(&self) -> Result<Page<models::orgs::Organization>>;
//! }
//!
//! #[async_trait::async_trait]
//! impl OrganisationExt for Octocrab {
//!   async fn list_every_organisation(&self) -> Result<Page<models::orgs::Organization>> {
//!     self.get("organizations", None::<&()>).await
//!   }
//! }
//! ```
//!
//! You can also easily access new properties that aren't available in the
//! current models using `serde`.
//!
//! ```no_run
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct RepositoryWithVisibility {
//!     #[serde(flatten)]
//!     inner: octocrab::models::Repository,
//!     visibility: String,
//! }
//!
//!
//! # async fn run() -> octocrab::Result<()> {
//! let my_repo = octocrab::instance()
//!     .get::<RepositoryWithVisibility, _, _>("https://api.github.com/repos/XAMPPRocky/octocrab", None::<&()>)
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//!
//!
//! ## Static API
//! `octocrab` also provides a statically reference count version of its API,
//! allowing you to easily plug it into existing systems without worrying
//! about having to integrate and pass around the client.
//!
//! ```
//! // Initialises the static instance with your configuration and returns an
//! // instance of the client.
//! # use octocrab::Octocrab;
//! tokio_test::block_on(async {
//! octocrab::initialise(Octocrab::default());
//! // Gets a instance of `Octocrab` from the static API. If you call this
//! // without first calling `octocrab::initialise` a default client will be
//! // initialised and returned instead.
//! octocrab::instance();
//! # })
//! ```
//!
//! ## GitHub webhook application support
//!
//! `octocrab` provides [deserializable datatypes](crate::models::webhook_events)
//! for the payloads received by a GitHub application [responding to
//! webhooks](https://docs.github.com/en/apps/creating-github-apps/writing-code-for-a-github-app/building-a-github-app-that-responds-to-webhook-events).
//! This allows you to write a typesafe application using Rust with
//! pattern-matching/enum-dispatch to respond to events.
//!
//! **Note**: Webhook support in `octocrab` is still beta, not all known webhook events are
//! strongly typed.
//!
//! ```no_run
//! # use http::request::Request;
//! # use tracing::{warn, info};
//! # use octocrab::models::webhook_events::*;
//! # let request_from_github = Request::post("https://my-webhook-url.com").body(vec![0_u8]).unwrap();
//! // request_from_github is the HTTP request your webhook handler received
//! let (parts, body) = request_from_github.into_parts();
//! let header = parts.headers.get("X-GitHub-Event").unwrap().to_str().unwrap();
//!
//! let event = WebhookEvent::try_from_header_and_body(header, &body).unwrap();
//! // Now you can match on event type and call any specific handling logic
//! match event.kind {
//!     WebhookEventType::Ping => info!("Received a ping"),
//!     WebhookEventType::PullRequest => info!("Received a pull request event"),
//!     // ...
//!     _ => warn!("Ignored event"),
//! };
//! ```
#![cfg_attr(test, recursion_limit = "512")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod api;
mod body;
mod error;
mod from_response;
mod page;

pub mod auth;
pub mod etag;
pub mod models;
pub mod params;
pub mod service;

use api::repos::RepoRef;
use api::users::UserRef;
use body::OctoBody;
use chrono::{DateTime, Utc};
use http::{HeaderMap, HeaderValue, Method, Uri};
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use service::middleware::auth_header::AuthHeaderLayer;
use service::middleware::cache::{CacheStorage, HttpCacheLayer};
use std::convert::{Infallible, TryInto};
use std::fmt;
use std::future::Future;
use std::io::Write;
use std::marker::PhantomData;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use web_time::Duration;

use http::{header::HeaderName, StatusCode};
use hyper::{Request, Response};

use once_cell::sync::Lazy;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use snafu::*;
use tower::{buffer::Buffer, util::BoxService, BoxError, Layer, Service, ServiceExt};

use bytes::Bytes;
use http::header::USER_AGENT;
use http::request::Builder;
#[cfg(feature = "opentls")]
use hyper_tls::HttpsConnector;

#[cfg(feature = "rustls")]
use hyper_rustls::HttpsConnectorBuilder;

#[cfg(feature = "retry")]
use tower::retry::{Retry, RetryLayer};

#[cfg(feature = "timeout")]
use hyper_timeout::TimeoutConnector;

use tower_http::{classify::ServerErrorsFailureClass, map_response_body::MapResponseBodyLayer};

#[cfg(feature = "tracing")]
use {tower_http::trace::TraceLayer, tracing::Span};

use crate::api::codes_of_conduct;
use crate::error::{
    HttpSnafu, HyperSnafu, InvalidUtf8Snafu, SerdeSnafu, SerdeUrlEncodedSnafu, ServiceSnafu,
    UriParseError, UriParseSnafu, UriSnafu,
};

use crate::service::middleware::base_uri::BaseUriLayer;
use crate::service::middleware::extra_headers::ExtraHeadersLayer;

#[cfg(feature = "retry")]
use crate::service::middleware::retry::RetryConfig;

use auth::{AppAuth, Auth};
use models::{AppId, InstallationId, InstallationToken, RepositoryId, UserId};

pub use self::{
    api::{
        actions, activity, apps, checks, classroom, code_scannings, commits, current, events,
        gists, gitignore, hooks, issues, licenses, markdown, orgs, projects, pulls, ratelimit,
        repos, search, teams, users, workflows,
    },
    error::{Error, GitHubError},
    from_response::FromResponse,
    page::Page,
};

#[cfg(all(feature = "jwt-rust-crypto", feature = "jwt-aws-lc-rs"))]
compile_error!(
    "feature \"jwt-rust-crypto\" and feature \"jwt-aws-lc-rs\" cannot be enabled at the same time"
);

#[cfg(not(any(feature = "jwt-rust-crypto", feature = "jwt-aws-lc-rs")))]
compile_error!("at least one of the features \"jwt-rust-crypto\" and feature \"jwt-aws-lc-rs\" must be enabled");

/// A convenience type with a default error type of [`Error`].
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

const GITHUB_BASE_URI: &str = "https://api.github.com";
const GITHUB_BASE_UPLOAD_URI: &str = "https://uploads.github.com";

// This `include!` gives us pub const _SET_HEADERS_MAP: [(&str, &str)]
// generated from Cargo.toml `[package.metadata.github-api].request-headers` array, like
// ```
// [package.metadata.github-api]
//
// request-headers = ["X-GitHub-Api-Version: 2022-11-28", ]
// ```
include!(concat!(env!("OUT_DIR"), "/headers_metadata.rs"));

#[cfg(feature = "default-client")]
static STATIC_INSTANCE: Lazy<arc_swap::ArcSwap<Octocrab>> =
    Lazy::new(|| arc_swap::ArcSwap::from_pointee(Octocrab::default()));

/// Formats a GitHub preview from it's name into the full value for the
/// `Accept` header.
/// ```
/// assert_eq!(octocrab::format_preview("machine-man"), "application/vnd.github.machine-man-preview");
/// ```
pub fn format_preview(preview: impl AsRef<str>) -> String {
    format!("application/vnd.github.{}-preview", preview.as_ref())
}

/// Formats a media type from it's name into the full value for the
/// `Accept` header.
/// ```
/// assert_eq!(octocrab::format_media_type("html"), "application/vnd.github.v3.html+json");
/// assert_eq!(octocrab::format_media_type("json"), "application/vnd.github.v3.json");
/// assert_eq!(octocrab::format_media_type("patch"), "application/vnd.github.v3.patch");
/// ```
pub fn format_media_type(media_type: impl AsRef<str>) -> String {
    let media_type = media_type.as_ref();
    let json_suffix = match media_type {
        "raw" | "text" | "html" | "full" => "+json",
        _ => "",
    };

    format!("application/vnd.github.v3.{media_type}{json_suffix}")
}

#[derive(Debug, Deserialize)]
struct GitHubErrorBody {
    pub documentation_url: Option<String>,
    pub errors: Option<Vec<serde_json::Value>>,
    pub message: String,
}

/// Maps a GitHub error response into and `Err()` variant if the status is
/// not a success.
pub async fn map_github_error(
    response: http::Response<BoxBody<Bytes, crate::Error>>,
) -> Result<http::Response<BoxBody<Bytes, crate::Error>>> {
    if response.status().is_success() {
        Ok(response)
    } else {
        let (parts, body) = response.into_parts();
        let GitHubErrorBody {
            documentation_url,
            errors,
            message,
        } = serde_json::from_slice(body.collect().await?.to_bytes().as_ref())
            .context(error::SerdeSnafu)?;

        Err(error::Error::GitHub {
            source: Box::new(GitHubError {
                status_code: parts.status,
                documentation_url,
                errors,
                message,
            }),
            backtrace: Backtrace::capture(),
        })
    }
}

/// Initialises the static instance using the configuration set by
/// `builder`.
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let octocrab = octocrab::initialise(octocrab::Octocrab::default());
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "default-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "default-client")))]
pub fn initialise(crab: Octocrab) -> Arc<Octocrab> {
    STATIC_INSTANCE.swap(Arc::from(crab))
}

/// Returns a new instance of [`Octocrab`]. If it hasn't been previously
/// initialised it returns a default instance with no authentication set.
/// ```
/// #[tokio::main]
/// async fn main() -> () {
/// let octocrab = octocrab::instance();
/// }
/// ```
#[cfg(feature = "default-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "default-client")))]
pub fn instance() -> Arc<Octocrab> {
    STATIC_INSTANCE.load().clone()
}

type Executor = Box<dyn Fn(Pin<Box<dyn Future<Output = ()>>>)>;

/// A builder struct for `Octocrab`, allowing you to configure the client, such
/// as using GitHub previews, the github instance, authentication, etc.
///
/// ```
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let octocrab = octocrab::OctocrabBuilder::default()
///     .add_preview("machine-man")
///     .base_uri("https://github.example.com")?
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
/// OctocrabBuilder can be extended with a custom config, see [DefaultOctocrabBuilderConfig] for an example
pub struct OctocrabBuilder<Svc, Config, Auth, LayerReady> {
    service: Svc,
    auth: Auth,
    config: Config,
    _layer_ready: PhantomData<LayerReady>,
    executor: Option<Executor>,
}

//Indicates weather the builder supports config
pub struct NoConfig {}

//Indicates weather the builder supports service that is already inside builder
pub struct NoSvc {}

//Indicates weather builder supports with_layer(This is somewhat redundant given NoSvc exists, but we have to use this until specialization is stable)
pub struct NotLayerReady {}
pub struct LayerReady {}

//Indicates weather the builder supports auth
pub struct NoAuth {}

impl OctocrabBuilder<NoSvc, NoConfig, NoAuth, NotLayerReady> {
    pub fn new_empty() -> Self {
        OctocrabBuilder {
            service: NoSvc {},
            auth: NoAuth {},
            config: NoConfig {},
            _layer_ready: PhantomData,
            executor: None,
        }
    }
}

impl OctocrabBuilder<NoSvc, DefaultOctocrabBuilderConfig, NoAuth, NotLayerReady> {
    pub fn new() -> Self {
        OctocrabBuilder::default()
    }
}

impl<Config, Auth> OctocrabBuilder<NoSvc, Config, Auth, NotLayerReady> {
    pub fn with_service<Svc>(self, service: Svc) -> OctocrabBuilder<Svc, Config, Auth, LayerReady> {
        OctocrabBuilder {
            service,
            auth: self.auth,
            config: self.config,
            _layer_ready: PhantomData,
            executor: None,
        }
    }
}

impl<Svc, Config, Auth, B> OctocrabBuilder<Svc, Config, Auth, LayerReady>
where
    Svc: Service<Request<OctoBody>, Response = Response<B>> + Send + 'static,
    Svc::Future: Send + 'static,
    Svc::Error: Into<BoxError>,
    B: http_body::Body<Data = bytes::Bytes> + Send + 'static,
    B::Error: Into<BoxError>,
{
    pub fn with_executor(
        self,
        executor: Executor,
    ) -> OctocrabBuilder<Svc, Config, Auth, LayerReady> {
        OctocrabBuilder {
            service: self.service,
            auth: self.auth,
            config: self.config,
            _layer_ready: PhantomData,
            executor: Some(executor),
        }
    }
}

impl<Svc, Config, Auth, B> OctocrabBuilder<Svc, Config, Auth, LayerReady>
where
    Svc: Service<Request<OctoBody>, Response = Response<B>> + Send + 'static,
    Svc::Future: Send + 'static,
    Svc::Error: Into<BoxError>,
    B: http_body::Body<Data = bytes::Bytes> + Send + 'static,
    B::Error: Into<BoxError>,
{
    /// Add a [`Layer`] to the current [`Service`] stack.
    pub fn with_layer<L: Layer<Svc>>(
        self,
        layer: &L,
    ) -> OctocrabBuilder<L::Service, Config, Auth, LayerReady> {
        let Self {
            service: stack,
            auth,
            config,
            executor,
            ..
        } = self;
        OctocrabBuilder {
            service: layer.layer(stack),
            auth,
            config,
            executor,
            _layer_ready: PhantomData,
        }
    }
}

impl Default for OctocrabBuilder<NoSvc, DefaultOctocrabBuilderConfig, NoAuth, NotLayerReady> {
    fn default() -> OctocrabBuilder<NoSvc, DefaultOctocrabBuilderConfig, NoAuth, NotLayerReady> {
        OctocrabBuilder::new_empty().with_config(DefaultOctocrabBuilderConfig::default())
    }
}

impl<Svc, Auth, LayerState> OctocrabBuilder<Svc, NoConfig, Auth, LayerState> {
    fn with_config<Config>(self, config: Config) -> OctocrabBuilder<Svc, Config, Auth, LayerState> {
        OctocrabBuilder {
            service: self.service,
            auth: self.auth,
            executor: self.executor,
            config,
            _layer_ready: PhantomData,
        }
    }
}

impl<Svc, B, LayerState> OctocrabBuilder<Svc, NoConfig, AuthState, LayerState>
where
    Svc: Service<Request<OctoBody>, Response = Response<B>> + Send + 'static,
    Svc::Future: Send + 'static,
    Svc::Error: Into<BoxError>,
    B: http_body::Body<Data = bytes::Bytes> + Send + Sync + 'static,
    B::Error: Into<BoxError>,
{
    /// Build a [`Client`](OctocrabService) instance with the current [`Service`] stack.
    pub fn build(self) -> Result<Octocrab, Infallible> {
        // Transform response body to `BoxBody<Bytes, crate::Error>` and use type erased error to avoid type parameters.
        let service = MapResponseBodyLayer::new(|b: B| {
            b.map_err(|e| ServiceSnafu.into_error(e.into())).boxed()
        })
        .layer(self.service)
        .map_err(|e| e.into());

        if let Some(executor) = self.executor {
            return Ok(Octocrab::new_with_executor(service, self.auth, executor));
        }

        Ok(Octocrab::new(service, self.auth))
    }
}

impl<Svc, Config, LayerState> OctocrabBuilder<Svc, Config, NoAuth, LayerState> {
    pub fn with_auth<Auth>(self, auth: Auth) -> OctocrabBuilder<Svc, Config, Auth, LayerState> {
        OctocrabBuilder {
            service: self.service,
            auth,
            config: self.config,
            executor: self.executor,
            _layer_ready: PhantomData,
        }
    }
}

impl OctocrabBuilder<NoSvc, DefaultOctocrabBuilderConfig, NoAuth, NotLayerReady> {
    /// Set the retry configuration
    #[cfg(feature = "retry")]
    #[cfg_attr(docsrs, doc(cfg(feature = "retry")))]
    pub fn add_retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.config.retry_config = retry_config;
        self
    }

    /// Set the connect timeout.
    #[cfg(feature = "timeout")]
    #[cfg_attr(docsrs, doc(cfg(feature = "timeout")))]
    pub fn set_connect_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.config.connect_timeout = timeout;
        self
    }

    /// Set the read timeout.
    #[cfg(feature = "timeout")]
    #[cfg_attr(docsrs, doc(cfg(feature = "timeout")))]
    pub fn set_read_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.config.read_timeout = timeout;
        self
    }

    /// Set the write timeout.
    #[cfg(feature = "timeout")]
    #[cfg_attr(docsrs, doc(cfg(feature = "timeout")))]
    pub fn set_write_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.config.write_timeout = timeout;
        self
    }

    /// Enable a GitHub preview.
    pub fn add_preview(mut self, preview: &'static str) -> Self {
        self.config.previews.push(preview);
        self
    }

    /// Add an additional header to include with every request.
    pub fn add_header(mut self, key: HeaderName, value: String) -> Self {
        self.config.extra_headers.push((key, value));
        self
    }

    /// Add a personal token to use for authentication.
    pub fn personal_token<S: Into<SecretString>>(mut self, token: S) -> Self {
        self.config.auth = Auth::PersonalToken(token.into());
        self
    }

    /// Authenticate as a Github App.
    /// `key`: RSA private key in DER or PEM formats.
    pub fn app(mut self, app_id: AppId, key: jsonwebtoken::EncodingKey) -> Self {
        self.config.auth = Auth::App(AppAuth { app_id, key });
        self
    }

    /// Authenticate as a Basic Auth
    /// username and password
    pub fn basic_auth(mut self, username: String, password: String) -> Self {
        self.config.auth = Auth::Basic { username, password };
        self
    }

    /// Authenticate with an OAuth token.
    pub fn oauth(mut self, oauth: auth::OAuth) -> Self {
        self.config.auth = Auth::OAuth(oauth);
        self
    }

    /// Authenticate with a user access token.
    pub fn user_access_token<S: Into<SecretString>>(mut self, token: S) -> Self {
        self.config.auth = Auth::UserAccessToken(token.into());
        self
    }

    /// Set the base url for `Octocrab`.
    pub fn base_uri(mut self, base_uri: impl TryInto<Uri>) -> Result<Self> {
        self.config.base_uri = Some(
            base_uri
                .try_into()
                .map_err(|_| UriParseError {})
                .context(UriParseSnafu)?,
        );
        Ok(self)
    }

    /// Set the base upload url for `Octocrab`.
    pub fn upload_uri(mut self, upload_uri: impl TryInto<Uri>) -> Result<Self> {
        self.config.upload_uri = Some(
            upload_uri
                .try_into()
                .map_err(|_| UriParseError {})
                .context(UriParseSnafu)?,
        );
        Ok(self)
    }

    pub fn cache<C>(mut self, cache: C) -> Self
    where
        C: CacheStorage + 'static,
    {
        self.config.cache_storage = Some(Arc::new(cache));
        self
    }

    #[cfg(feature = "retry")]
    #[cfg_attr(docsrs, doc(cfg(feature = "retry")))]
    pub fn set_connector_retry_service<S>(
        &self,
        connector: hyper_util::client::legacy::Client<S, OctoBody>,
    ) -> Retry<RetryConfig, hyper_util::client::legacy::Client<S, OctoBody>> {
        let retry_layer = RetryLayer::new(self.config.retry_config.clone());

        retry_layer.layer(connector)
    }

    #[cfg(feature = "timeout")]
    #[cfg_attr(docsrs, doc(cfg(feature = "timeout")))]
    pub fn set_connect_timeout_service<T>(&self, connector: T) -> TimeoutConnector<T>
    where
        T: Service<Uri> + Send,
        T::Response: hyper::rt::Read + hyper::rt::Write + Send + Unpin,
        T::Future: Send + 'static,
        T::Error: Into<BoxError>,
    {
        let mut connector = TimeoutConnector::new(connector);
        // Set the timeouts for the client
        connector.set_connect_timeout(self.config.connect_timeout);
        connector.set_read_timeout(self.config.read_timeout);
        connector.set_write_timeout(self.config.write_timeout);
        connector
    }

    /// Build a [`Client`](hyper_util::client::legacy::Client) instance with the current [`Service`] stack.
    #[cfg(feature = "default-client")]
    #[cfg_attr(docsrs, doc(cfg(feature = "default-client")))]
    pub fn build(self) -> Result<Octocrab> {
        let client: hyper_util::client::legacy::Client<_, OctoBody> = {
            #[cfg(all(not(feature = "opentls"), not(feature = "rustls")))]
            let mut connector = hyper::client::conn::http1::HttpConnector::new();

            #[cfg(all(feature = "rustls", not(feature = "opentls")))]
            let connector = {
                let builder = HttpsConnectorBuilder::new();
                #[cfg(feature = "rustls-webpki-tokio")]
                let builder = builder.with_webpki_roots();
                #[cfg(not(feature = "rustls-webpki-tokio"))]
                let builder = builder
                    .with_native_roots()
                    .map_err(Into::into)
                    .context(error::OtherSnafu)?; // enabled the `rustls-native-certs` feature in hyper-rustls

                builder
                    .https_or_http() //  Disable .https_only() during tests until: https://github.com/LukeMathWalker/wiremock-rs/issues/58 is resolved. Alternatively we can use conditional compilation to only enable this feature in tests, but it becomes rather ugly with integration tests.
                    .enable_http1()
                    .build()
            };

            #[cfg(all(feature = "opentls", not(feature = "rustls")))]
            let connector = HttpsConnector::new();

            #[cfg(feature = "timeout")]
            let connector = self.set_connect_timeout_service(connector);

            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(connector)
        };

        #[cfg(feature = "retry")]
        let client = self.set_connector_retry_service(client);

        #[cfg(feature = "tracing")]
        let client = TraceLayer::new_for_http()
            .make_span_with(|req: &Request<OctoBody>| {
                tracing::debug_span!(
                    "HTTP",
                     http.method = %req.method(),
                     http.url = %req.uri(),
                     http.status_code = tracing::field::Empty,
                     otel.name = req.extensions().get::<&'static str>().unwrap_or(&"HTTP"),
                     otel.kind = "client",
                     otel.status_code = tracing::field::Empty,
                )
            })
            .on_request(|_req: &Request<OctoBody>, _span: &Span| {
                tracing::debug!("requesting");
            })
            .on_response(
                |res: &Response<hyper::body::Incoming>, _latency: Duration, span: &Span| {
                    let status = res.status();
                    span.record("http.status_code", status.as_u16());
                    if status.is_client_error() || status.is_server_error() {
                        span.record("otel.status_code", "ERROR");
                    }
                },
            )
            // Explicitly disable `on_body_chunk`. The default does nothing.
            .on_body_chunk(())
            .on_eos(|_: Option<&HeaderMap>, _duration: Duration, _span: &Span| {
                tracing::debug!("stream closed");
            })
            .on_failure(
                |ec: ServerErrorsFailureClass, _latency: Duration, span: &Span| {
                    // Called when
                    // - Calling the inner service errored
                    // - Polling `Body` errored
                    // - the response was classified as failure (5xx)
                    // - End of stream was classified as failure
                    span.record("otel.status_code", "ERROR");
                    match ec {
                        ServerErrorsFailureClass::StatusCode(status) => {
                            span.record("http.status_code", status.as_u16());
                            tracing::error!("failed with status {}", status)
                        }
                        ServerErrorsFailureClass::Error(err) => {
                            tracing::error!("failed with error {}", err)
                        }
                    }
                },
            )
            .layer(client);

        #[cfg(feature = "follow-redirect")]
        let client = tower_http::follow_redirect::FollowRedirectLayer::new().layer(client);

        let mut hmap: Vec<(HeaderName, HeaderValue)> = vec![];

        // Add the user agent header required by GitHub
        hmap.push((USER_AGENT, HeaderValue::from_str("octocrab").unwrap()));

        for preview in &self.config.previews {
            hmap.push((
                http::header::ACCEPT,
                HeaderValue::from_str(crate::format_preview(preview).as_str()).unwrap(),
            ));
        }

        let (auth_header, auth_state): (Option<HeaderValue>, _) = match self.config.auth {
            Auth::None => (None, AuthState::None),
            Auth::Basic { username, password } => {
                (None, AuthState::BasicAuth { username, password })
            }
            Auth::PersonalToken(token) => (
                Some(format!("Bearer {}", token.expose_secret()).parse().unwrap()),
                AuthState::None,
            ),
            Auth::UserAccessToken(token) => (
                Some(format!("Bearer {}", token.expose_secret()).parse().unwrap()),
                AuthState::None,
            ),
            Auth::App(app_auth) => (None, AuthState::App(app_auth)),
            Auth::OAuth(device) => (
                Some(
                    format!(
                        "{} {}",
                        device.token_type,
                        &device.access_token.expose_secret()
                    )
                    .parse()
                    .unwrap(),
                ),
                AuthState::None,
            ),
        };

        for (key, value) in self.config.extra_headers.iter() {
            hmap.push((
                key.clone(),
                HeaderValue::from_str(value.as_str())
                    .map_err(http::Error::from)
                    .context(HttpSnafu)?,
            ));
        }

        let client = ExtraHeadersLayer::new(Arc::new(hmap)).layer(client);

        let client = MapResponseBodyLayer::new(|body| {
            BodyExt::map_err(body, |e| HyperSnafu.into_error(e)).boxed()
        })
        .layer(client);

        let base_uri = self
            .config
            .base_uri
            .clone()
            .unwrap_or_else(|| Uri::from_str(GITHUB_BASE_URI).unwrap());

        let upload_uri = self
            .config
            .upload_uri
            .clone()
            .unwrap_or_else(|| Uri::from_str(GITHUB_BASE_UPLOAD_URI).unwrap());

        let client = BaseUriLayer::new(base_uri.clone()).layer(client);

        let client = AuthHeaderLayer::new(auth_header, base_uri, upload_uri).layer(client);

        let client = HttpCacheLayer::new(self.config.cache_storage.clone()).layer(client);

        if let Some(executor) = self.executor {
            return Ok(Octocrab::new_with_executor(client, auth_state, executor));
        }

        Ok(Octocrab::new(client, auth_state))
    }
}

pub struct DefaultOctocrabBuilderConfig {
    auth: Auth,
    previews: Vec<&'static str>,
    extra_headers: Vec<(HeaderName, String)>,
    #[cfg(feature = "timeout")]
    connect_timeout: Option<Duration>,
    #[cfg(feature = "timeout")]
    read_timeout: Option<Duration>,
    #[cfg(feature = "timeout")]
    write_timeout: Option<Duration>,
    base_uri: Option<Uri>,
    upload_uri: Option<Uri>,
    #[cfg(feature = "retry")]
    retry_config: RetryConfig,
    cache_storage: Option<Arc<dyn CacheStorage>>,
}

impl Default for DefaultOctocrabBuilderConfig {
    fn default() -> Self {
        Self {
            auth: Auth::None,
            previews: Vec::new(),
            extra_headers: Vec::new(),
            #[cfg(feature = "timeout")]
            connect_timeout: None,
            #[cfg(feature = "timeout")]
            read_timeout: None,
            #[cfg(feature = "timeout")]
            write_timeout: None,
            base_uri: None,
            upload_uri: None,
            #[cfg(feature = "retry")]
            retry_config: RetryConfig::Simple(3),
            cache_storage: None,
        }
    }
}

impl DefaultOctocrabBuilderConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
struct CachedTokenInner {
    expiration: Option<DateTime<Utc>>,
    secret: SecretString,
}

impl CachedTokenInner {
    fn new(secret: SecretString, expiration: Option<DateTime<Utc>>) -> Self {
        Self { secret, expiration }
    }

    fn expose_secret(&self) -> &str {
        self.secret.expose_secret()
    }
}

/// A cached API access token (which may be None)
pub struct CachedToken(RwLock<Option<CachedTokenInner>>);

impl CachedToken {
    fn clear(&self) {
        *self.0.write().unwrap() = None;
    }

    /// Returns a valid token if it exists and is not expired or if there is no expiration date.
    fn valid_token_with_buffer(&self, buffer: chrono::Duration) -> Option<SecretString> {
        let inner = self.0.read().unwrap();

        if let Some(token) = inner.as_ref() {
            if let Some(exp) = token.expiration {
                if exp - Utc::now() > buffer {
                    return Some(token.secret.clone());
                }
            } else {
                return Some(token.secret.clone());
            }
        }

        None
    }

    fn valid_token(&self) -> Option<SecretString> {
        self.valid_token_with_buffer(chrono::Duration::seconds(30))
    }

    fn set<S: Into<SecretString>>(&self, token: S, expiration: Option<DateTime<Utc>>) {
        *self.0.write().unwrap() = Some(CachedTokenInner::new(token.into(), expiration));
    }
}

impl fmt::Debug for CachedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.read().unwrap().fmt(f)
    }
}

impl fmt::Display for CachedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let option = self.0.read().unwrap();
        option
            .as_ref()
            .map(|s| s.expose_secret().fmt(f))
            .unwrap_or_else(|| write!(f, "<none>"))
    }
}

impl Clone for CachedToken {
    fn clone(&self) -> CachedToken {
        CachedToken(RwLock::new(self.0.read().unwrap().clone()))
    }
}

impl Default for CachedToken {
    fn default() -> CachedToken {
        CachedToken(RwLock::new(None))
    }
}

/// State used for authenticate to Github
#[derive(Debug, Clone)]
pub enum AuthState {
    /// No state, although Auth::PersonalToken may have caused
    /// an Authorization HTTP header to be set to provide authentication.
    None,
    /// Basic Auth HTTP. (username:password)
    BasicAuth {
        /// The username
        username: String,
        /// The password
        password: String,
    },
    /// Github App authentication with the given app data
    App(AppAuth),
    /// Authentication via a Github App repo-specific installation
    Installation {
        /// The app authentication data (app ID and private key)
        app: AppAuth,
        /// The installation ID
        installation: InstallationId,
        /// The cached access token, if any
        token: CachedToken,
    },
    /// Access token based authentication.
    AccessToken {
        /// The access token
        token: SecretString,
    },
}

pub type OctocrabService = Buffer<
    http::Request<OctoBody>,
    <BoxService<http::Request<OctoBody>, http::Response<BoxBody<Bytes, Error>>, BoxError> as tower::Service<http::Request<OctoBody>>>::Future
>;

/// The GitHub API client.
#[derive(Clone)]
pub struct Octocrab {
    client: OctocrabService,
    auth_state: AuthState,
}

impl fmt::Debug for Octocrab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Octocrab")
            .field("auth_state", &self.auth_state)
            .finish()
    }
}

/// Defaults for Octocrab:
/// - `base_uri`: `https://api.github.com`
/// - `auth`: `None`
/// - `client`: http client with the `octocrab` user agent.
#[cfg(feature = "default-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "default-client")))]
impl Default for Octocrab {
    fn default() -> Self {
        OctocrabBuilder::default().build().unwrap()
    }
}

/// # Constructors
impl Octocrab {
    /// Returns a new `OctocrabBuilder`.
    pub fn builder() -> OctocrabBuilder<NoSvc, DefaultOctocrabBuilderConfig, NoAuth, NotLayerReady>
    {
        OctocrabBuilder::new_empty().with_config(DefaultOctocrabBuilderConfig::default())
    }

    /// Creates a new `Octocrab`.
    fn new<S>(service: S, auth_state: AuthState) -> Self
    where
        S: Service<Request<OctoBody>, Response = Response<BoxBody<Bytes, crate::Error>>>
            + Send
            + 'static,
        S::Future: Send + 'static,
        S::Error: Into<BoxError>,
    {
        let service = Buffer::new(BoxService::new(service.map_err(Into::into)), 1024);

        Self {
            client: service,
            auth_state,
        }
    }

    /// Creates a new `Octocrab` with a custom executor
    fn new_with_executor<S>(service: S, auth_state: AuthState, executor: Executor) -> Self
    where
        S: Service<Request<OctoBody>, Response = Response<BoxBody<Bytes, crate::Error>>>
            + Send
            + 'static,
        S::Future: Send + 'static,
        S::Error: Into<BoxError>,
    {
        // Use Buffer pair to return the background worker
        let (service, worker) = Buffer::pair(BoxService::new(service.map_err(Into::into)), 1024);

        // Execute the background worker with the custom executor
        executor(Box::pin(worker));

        Self {
            client: service,
            auth_state,
        }
    }

    /// Returns a new `Octocrab` based on the current builder but
    /// authorizing via a specific installation ID.
    /// Typically you will first construct an `Octocrab` using
    /// `OctocrabBuilder::app` to authenticate as your Github App,
    /// then obtain an installation ID, and then pass that here to
    /// obtain a new `Octocrab` with which you can make API calls
    /// with the permissions of that installation.
    pub fn installation(&self, id: InstallationId) -> Result<Octocrab> {
        let app_auth = if let AuthState::App(ref app_auth) = self.auth_state {
            app_auth.clone()
        } else {
            return Err(Error::Installation {
                backtrace: Backtrace::capture(),
            });
        };
        Ok(Octocrab {
            client: self.client.clone(),
            auth_state: AuthState::Installation {
                app: app_auth,
                installation: id,
                token: CachedToken::default(),
            },
        })
    }

    /// Similar to `installation`, but also eagerly caches the installation
    /// token and returns the token. The returned token can be used to make
    /// https git requests to e.g. clone repositories that the installation
    /// has access to.
    ///
    /// See also <https://docs.github.com/en/developers/apps/building-github-apps/authenticating-with-github-apps#http-based-git-access-by-an-installation>
    pub async fn installation_and_token(
        &self,
        id: InstallationId,
    ) -> Result<(Octocrab, SecretString)> {
        let crab = self.installation(id)?;
        let token = crab.request_installation_auth_token().await?;
        Ok((crab, token))
    }

    /// Acquire a GitHub App installation access token that does not expire for
    /// at least 30 seconds. A cached token will be used if its expiration is
    /// far enough in the future. Otherwise, a new token will be acquired and
    /// cached.
    pub async fn installation_token(&self) -> Result<SecretString> {
        self.installation_token_with_buffer(chrono::Duration::seconds(30))
            .await
    }

    /// Acquire a GitHub App installation access token that does not expire for
    /// at least the duration specified by [`buffer`]. A cached token will be
    /// used if its expiration is far enough in the future. Otherwise, a new
    /// token will be acquired and cached.
    pub async fn installation_token_with_buffer(
        &self,
        buffer: chrono::Duration,
    ) -> Result<SecretString> {
        let token = if let AuthState::Installation { ref token, .. } = self.auth_state {
            token
        } else {
            return Err(Error::InstallationTokenInvalidAuth {
                backtrace: Backtrace::capture(),
            });
        };

        let token = match token.valid_token_with_buffer(buffer) {
            Some(token) => token.into(),
            None => self.request_installation_auth_token().await?,
        };

        Ok(token)
    }

    /// Returns a new `Octocrab` based on the current builder but
    /// authorizing via an access token.
    ///
    /// See also <https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app>
    pub fn user_access_token<S: Into<SecretString>>(&self, token: S) -> Result<Self> {
        Ok(Octocrab {
            client: self.client.clone(),
            auth_state: AuthState::AccessToken {
                token: token.into(),
            },
        })
    }
}

/// # GitHub API Methods
impl Octocrab {
    /// Creates a new [`actions::ActionsHandler`] for accessing information from
    /// GitHub Actions.
    pub fn actions(&self) -> actions::ActionsHandler<'_> {
        actions::ActionsHandler::new(self)
    }

    /// Creates a [`current::CurrentAuthHandler`] that allows you to access
    /// information about the current authenticated user.
    pub fn current(&self) -> current::CurrentAuthHandler<'_> {
        current::CurrentAuthHandler::new(self)
    }

    /// Creates a [`activity::ActivityHandler`] for the current authenticated user.
    pub fn activity(&self) -> activity::ActivityHandler<'_> {
        activity::ActivityHandler::new(self)
    }

    /// Creates a new [`apps::AppsRequestHandler`] for the currently authenticated app.
    pub fn apps(&self) -> apps::AppsRequestHandler<'_> {
        apps::AppsRequestHandler::new(self)
    }

    /// Creates a [`gitignore::GitignoreHandler`] for accessing information
    /// about `gitignore`.
    pub fn gitignore(&self) -> gitignore::GitignoreHandler<'_> {
        gitignore::GitignoreHandler::new(self)
    }

    /// Creates a [`issues::IssueHandler`] for the repo specified at `owner/repo`,
    /// that allows you to access GitHub's issues API.
    pub fn issues(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> issues::IssueHandler<'_> {
        issues::IssueHandler::new(self, RepoRef::ByOwnerAndName(owner.into(), repo.into()))
    }

    /// Creates a [`issues::IssueHandler`] for the repo specified at repository ID,
    /// that allows you to access GitHub's issues API.
    pub fn issues_by_id(&self, id: impl Into<RepositoryId>) -> issues::IssueHandler<'_> {
        issues::IssueHandler::new(self, RepoRef::ById(id.into()))
    }

    /// Creates a [`code_scannings::CodeScanningHandler`] for the repo specified at `owner/repo`,
    /// that allows you to access GitHub's Code scanning API.
    pub fn code_scannings(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> code_scannings::CodeScanningHandler<'_> {
        code_scannings::CodeScanningHandler::new(self, owner.into(), Option::from(repo.into()))
    }

    /// Creates a [`code_scannings::CodeScanningHandler`] for the org specified at `owner`,
    /// that allows you to access GitHub's Code scanning API.
    pub fn code_scannings_organisation(
        &self,
        owner: impl Into<String>,
    ) -> code_scannings::CodeScanningHandler<'_> {
        code_scannings::CodeScanningHandler::new(self, owner.into(), None)
    }

    /// Creates a [`commits::CommitHandler`] for the repo specified at `owner/repo`,
    pub fn commits(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> commits::CommitHandler<'_> {
        commits::CommitHandler::new(self, owner.into(), repo.into())
    }

    /// Creates a [`licenses::LicenseHandler`].
    pub fn licenses(&self) -> licenses::LicenseHandler<'_> {
        licenses::LicenseHandler::new(self)
    }

    /// Creates a [`markdown::MarkdownHandler`].
    pub fn markdown(&self) -> markdown::MarkdownHandler<'_> {
        markdown::MarkdownHandler::new(self)
    }

    /// Creates an [`orgs::OrgHandler`] for the specified organization,
    /// that allows you to access GitHub's organization API.
    pub fn orgs(&self, owner: impl Into<String>) -> orgs::OrgHandler<'_> {
        orgs::OrgHandler::new(self, owner.into())
    }

    /// Creates a [`pulls::PullRequestHandler`] for the repo specified at
    /// `owner/repo`, that allows you to access GitHub's pull request API.
    pub fn pulls(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> pulls::PullRequestHandler<'_> {
        pulls::PullRequestHandler::new(self, owner.into(), repo.into())
    }

    /// Creates a [`repos::RepoHandler`] for the repo specified at `owner/repo`,
    /// that allows you to access GitHub's repository API.
    pub fn repos(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> repos::RepoHandler<'_> {
        repos::RepoHandler::new(self, RepoRef::ByOwnerAndName(owner.into(), repo.into()))
    }

    /// Creates a [`repos::RepoHandler`] for the repo specified at repository ID,
    /// that allows you to access GitHub's repository API.
    pub fn repos_by_id(&self, id: impl Into<RepositoryId>) -> repos::RepoHandler<'_> {
        repos::RepoHandler::new(self, RepoRef::ById(id.into()))
    }

    /// Creates a [`projects::ProjectHandler`] that allows you to access GitHub's
    /// projects API (classic).
    pub fn projects(&self) -> projects::ProjectHandler<'_> {
        projects::ProjectHandler::new(self)
    }

    /// Creates a [`search::SearchHandler`] that allows you to construct general queries
    /// to GitHub's API.
    pub fn search(&self) -> search::SearchHandler<'_> {
        search::SearchHandler::new(self)
    }

    /// Creates a [`teams::TeamHandler`] for the specified organization that allows
    /// you to access GitHub's teams API.
    pub fn teams(&self, owner: impl Into<String>) -> teams::TeamHandler<'_> {
        teams::TeamHandler::new(self, owner.into())
    }

    /// Creates a [`users::UserHandler`] for the specified user using the user name
    pub fn users(&self, user: impl Into<String>) -> users::UserHandler<'_> {
        users::UserHandler::new(self, UserRef::ByString(user.into()))
    }

    /// Creates a [`users::UserHandler`] for the specified user using the user ID
    pub fn users_by_id(&self, user: impl Into<UserId>) -> users::UserHandler<'_> {
        users::UserHandler::new(self, UserRef::ById(user.into()))
    }

    /// Creates a [`workflows::WorkflowsHandler`] for the specified repository that allows
    /// you to access GitHub's workflows API.
    pub fn workflows(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> workflows::WorkflowsHandler<'_> {
        workflows::WorkflowsHandler::new(self, owner.into(), repo.into())
    }

    /// Creates an [`events::EventsBuilder`] that allows you to access
    /// GitHub's events API.
    pub fn events(&self) -> events::EventsBuilder<'_> {
        events::EventsBuilder::new(self)
    }

    /// Creates a [`gists::GistsHandler`] that allows you to access
    /// GitHub's Gists API.
    pub fn gists(&self) -> gists::GistsHandler<'_> {
        gists::GistsHandler::new(self)
    }

    /// Creates a [`checks::ChecksHandler`] that allows to access the Checks API.
    pub fn checks(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> checks::ChecksHandler<'_> {
        checks::ChecksHandler::new(self, owner.into(), repo.into())
    }

    /// Creates a [`ratelimit::RateLimitHandler`] that returns the API rate limit.
    pub fn ratelimit(&self) -> ratelimit::RateLimitHandler<'_> {
        ratelimit::RateLimitHandler::new(self)
    }

    /// Creates a [`hooks::HooksHandler`] that returns the API hooks
    pub fn hooks(&self, owner: impl Into<String>) -> hooks::HooksHandler<'_> {
        hooks::HooksHandler::new(self, owner.into())
    }

    /// Creates a [`classroom::AssignmentsHandler`] providing the GitHub Classroom _Assignments_ API
    pub fn assignments(&self) -> classroom::AssignmentsHandler<'_> {
        classroom::AssignmentsHandler::new(self)
    }

    /// Creates a [`classroom::ClassroomHandler`] providing the GitHub Classroom _Classrooms_ API
    pub fn classrooms(&self) -> classroom::ClassroomHandler<'_> {
        classroom::ClassroomHandler::new(self)
    }

    /// Creates a [`codes_of_conduct::CodesOfConductHandler`] providing the GitHub Codes of Codes of Conduct API
    pub fn codes_of_conduct(&self) -> codes_of_conduct::CodesOfConductHandler<'_> {
        codes_of_conduct::CodesOfConductHandler::new(self)
    }
}

/// # GraphQL API.
impl Octocrab {
    /// Sends a graphql query to GitHub, and deserialises the response
    /// from JSON.
    /// ```no_run
    ///# async fn run() -> octocrab::Result<()> {
    /// let response: serde_json::Value = octocrab::instance()
    ///     .graphql(&serde_json::json!({ "query": "{ viewer { login }}" }))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub async fn graphql<R: crate::FromResponse>(
        &self,
        payload: &(impl serde::Serialize + ?Sized),
    ) -> crate::Result<R> {
        self.post("/graphql", Some(&serde_json::json!(payload)))
            .await
    }
}

/// # HTTP Methods
/// A collection of different of HTTP methods to use with Octocrab's
/// configuration (Authenication, etc.). All of the HTTP methods (`get`, `post`,
/// etc.) perform some amount of pre-processing such as making relative urls
/// absolute, and post processing such as mapping any potential GitHub errors
/// into `Err()` variants, and deserializing the response body.
///
/// This isn't always ideal when working with GitHub's API and as such there are
/// additional methods available prefixed with `_` (e.g.  `_get`, `_post`,
/// etc.) that perform no pre or post processing and directly return the
/// `http::Response` struct.
impl Octocrab {
    /// Send a `POST` request to `route` with an optional body, returning the body
    /// of the response.
    pub async fn post<P: Serialize + ?Sized, R: FromResponse>(
        &self,
        route: impl AsRef<str>,
        body: Option<&P>,
    ) -> Result<R> {
        let response = self
            ._post(self.parameterized_uri(route, None::<&()>)?, body)
            .await?;
        R::from_response(crate::map_github_error(response).await?).await
    }

    /// Send a `POST` request with no additional pre/post-processing.
    pub async fn _post<P: Serialize + ?Sized>(
        &self,
        uri: impl TryInto<http::Uri>,
        body: Option<&P>,
    ) -> Result<http::Response<BoxBody<Bytes, crate::Error>>> {
        let uri = uri
            .try_into()
            .map_err(|_| UriParseError {})
            .context(UriParseSnafu)?;
        let request = Builder::new().method(Method::POST).uri(uri);
        let request = self.build_request(request, body)?;
        self.execute(request).await
    }

    /// Send a `GET` request to `route` with optional query parameters, returning
    /// the body of the response.
    pub async fn get<R, A, P>(&self, route: A, parameters: Option<&P>) -> Result<R>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        R: FromResponse,
    {
        self.get_with_headers(route, parameters, None).await
    }

    /// Send a `GET` request with no additional post-processing.
    pub async fn _get(
        &self,
        uri: impl TryInto<Uri>,
    ) -> Result<http::Response<BoxBody<Bytes, crate::Error>>> {
        self._get_with_headers(uri, None).await
    }

    /// Convenience method to accept any &str, and attempt to convert it to a Uri.
    /// the method also attempts to serialize any parameters into a query string, and append it to the uri.
    fn parameterized_uri<A, P>(&self, uri: A, parameters: Option<&P>) -> Result<Uri>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
    {
        let mut uri = uri.as_ref().to_string();
        if let Some(parameters) = parameters {
            if uri.contains('?') {
                uri = format!("{uri}&");
            } else {
                uri = format!("{uri}?");
            }
            uri = format!(
                "{}{}",
                uri,
                serde_urlencoded::to_string(parameters)
                    .context(SerdeUrlEncodedSnafu)?
                    .as_str()
            );
        }
        let uri = Uri::from_str(uri.as_str()).context(UriSnafu);
        uri
    }

    pub async fn body_to_string(
        &self,
        res: http::Response<BoxBody<Bytes, crate::Error>>,
    ) -> Result<String> {
        let body_bytes = res.into_body().collect().await?.to_bytes();
        String::from_utf8(body_bytes.to_vec()).context(InvalidUtf8Snafu)
    }

    /// Send a `GET` request to `route` with optional query parameters and headers, returning
    /// the body of the response.
    pub async fn get_with_headers<R, A, P>(
        &self,
        route: A,
        parameters: Option<&P>,
        headers: Option<http::header::HeaderMap>,
    ) -> Result<R>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        R: FromResponse,
    {
        let response = self
            ._get_with_headers(self.parameterized_uri(route, parameters)?, headers)
            .await?;
        R::from_response(crate::map_github_error(response).await?).await
    }

    /// Send a `GET` request including option to set headers, with no additional post-processing.
    pub async fn _get_with_headers(
        &self,
        uri: impl TryInto<Uri>,
        headers: Option<http::header::HeaderMap>,
    ) -> Result<http::Response<BoxBody<Bytes, crate::Error>>> {
        let uri = uri
            .try_into()
            .map_err(|_| UriParseError {})
            .context(UriParseSnafu)?;
        let mut request = Builder::new().method(Method::GET).uri(uri);
        if let Some(headers) = headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }
        let request = self.build_request(request, None::<&()>)?;
        self.execute(request).await
    }

    /// Send a `PATCH` request to `route` with optional query parameters,
    /// returning the body of the response.
    pub async fn patch<R, A, B>(&self, route: A, body: Option<&B>) -> Result<R>
    where
        A: AsRef<str>,
        B: Serialize + ?Sized,
        R: FromResponse,
    {
        let response = self
            ._patch(self.parameterized_uri(route, None::<&()>)?, body)
            .await?;
        R::from_response(crate::map_github_error(response).await?).await
    }

    /// Send a `PATCH` request with no additional post-processing.
    pub async fn _patch<B: Serialize + ?Sized>(
        &self,
        uri: impl TryInto<Uri>,
        body: Option<&B>,
    ) -> Result<http::Response<BoxBody<Bytes, crate::Error>>> {
        let uri = uri
            .try_into()
            .map_err(|_| UriParseError {})
            .context(UriParseSnafu)?;
        let request = Builder::new().method(Method::PATCH).uri(uri);
        let request = self.build_request(request, body)?;
        self.execute(request).await
    }

    /// Send a `PUT` request to `route` with optional query parameters,
    /// returning the body of the response.
    pub async fn put<R, A, B>(&self, route: A, body: Option<&B>) -> Result<R>
    where
        A: AsRef<str>,
        B: Serialize + ?Sized,
        R: FromResponse,
    {
        let response = self
            ._put(self.parameterized_uri(route, None::<&()>)?, body)
            .await?;
        R::from_response(crate::map_github_error(response).await?).await
    }

    /// Send a `PUT` request with no additional post-processing.
    pub async fn _put<B: Serialize + ?Sized>(
        &self,
        uri: impl TryInto<Uri>,
        body: Option<&B>,
    ) -> Result<http::Response<BoxBody<Bytes, crate::Error>>> {
        let uri = uri
            .try_into()
            .map_err(|_| UriParseError {})
            .context(UriParseSnafu)?;
        let request = Builder::new().method(Method::PUT).uri(uri);
        let request = self.build_request(request, body)?;
        self.execute(request).await
    }

    pub fn build_request<B: Serialize + ?Sized>(
        &self,
        mut builder: Builder,
        body: Option<&B>,
    ) -> Result<http::Request<OctoBody>> {
        // Since Octocrab doesn't require streamable bodies(aka, file upload) because it is serde::Serialize),
        // we can just use String body, since it is both http_body::Body(required by Hyper::Client), and Clone(required by BoxService).

        // In case octocrab needs to support cases where body is strictly streamable, it should use something like reqwest::Body,
        // since it differentiates between retryable bodies, and streams(aka, it implements try_clone(), which is needed for middlewares like retry).

        // Add headers specified in Cargo.toml
        // '[package.metadata.github-api].request-headers' section
        for kv in _SET_HEADERS_MAP {
            builder = builder.header(kv.0, kv.1);
        }

        if let Some(body) = body {
            builder = builder.header(http::header::CONTENT_TYPE, "application/json");
            let serialized = serde_json::to_string(body).context(SerdeSnafu)?;
            let body: OctoBody = serialized.into();
            let request = builder.body(body).context(HttpSnafu)?;
            Ok(request)
        } else {
            Ok(builder
                .header(http::header::CONTENT_LENGTH, "0")
                .body(OctoBody::empty())
                .context(HttpSnafu)?)
        }
    }

    /// Send a `DELETE` request to `route` with optional query body,
    /// returning the body of the response.
    pub async fn delete<R, A, B>(&self, route: A, body: Option<&B>) -> Result<R>
    where
        A: AsRef<str>,
        B: Serialize + ?Sized,
        R: FromResponse,
    {
        let response = self
            ._delete(self.parameterized_uri(route, None::<&()>)?, body)
            .await?;
        R::from_response(crate::map_github_error(response).await?).await
    }

    /// Send a `DELETE` request with no additional post-processing.
    pub async fn _delete<B: Serialize + ?Sized>(
        &self,
        uri: impl TryInto<Uri>,
        body: Option<&B>,
    ) -> Result<http::Response<BoxBody<Bytes, crate::Error>>> {
        let uri = uri
            .try_into()
            .map_err(|_| UriParseError {})
            .context(UriParseSnafu)?;
        let request = self.build_request(Builder::new().method(Method::DELETE).uri(uri), body)?;

        self.execute(request).await
    }

    /// Requests a fresh installation auth token and caches it. Returns the token.
    async fn request_installation_auth_token(&self) -> Result<SecretString> {
        let (app, installation, token) = if let AuthState::Installation {
            ref app,
            installation,
            ref token,
        } = self.auth_state
        {
            (app, installation, token)
        } else {
            return Err(Error::Installation {
                backtrace: Backtrace::capture(),
            });
        };
        let mut request = Builder::new();
        let mut sensitive_value =
            HeaderValue::from_str(format!("Bearer {}", app.generate_bearer_token()?).as_str())
                .map_err(http::Error::from)
                .context(HttpSnafu)?;

        let uri = http::Uri::builder()
            .path_and_query(format!("/app/installations/{installation}/access_tokens"))
            .build()
            .context(HttpSnafu)?;

        sensitive_value.set_sensitive(true);
        request = request
            .header(http::header::AUTHORIZATION, sensitive_value)
            .method(http::Method::POST)
            .uri(uri);
        let response = self
            .send(request.body("{}".into()).context(HttpSnafu)?)
            .await?;
        let _status = response.status();

        let token_object =
            InstallationToken::from_response(crate::map_github_error(response).await?).await?;

        let expiration = token_object
            .expires_at
            .map(|time| {
                DateTime::<Utc>::from_str(&time).map_err(|e| error::Error::Other {
                    source: Box::new(e),
                    backtrace: snafu::Backtrace::capture(),
                })
            })
            .transpose()?;

        #[cfg(feature = "tracing")]
        tracing::debug!("Token expires at: {:?}", expiration);

        token.set(token_object.token.clone(), expiration);

        Ok(SecretString::from(token_object.token))
    }

    /// Send the given request to the underlying service
    pub async fn send(
        &self,
        request: Request<OctoBody>,
    ) -> Result<http::Response<BoxBody<Bytes, crate::Error>>> {
        let mut svc = self.client.clone();
        let response: Response<BoxBody<Bytes, crate::Error>> = svc
            .ready()
            .await
            .context(ServiceSnafu)?
            .call(request)
            .await
            .context(ServiceSnafu)?;
        Ok(response)
        //todo: attempt to downcast error to something more specific before returning. (Currently having trouble with this because I am not accustomed with snafu)
        // map_err(|err| {
        //     // Error decorating request
        //     err.downcast::<Error>()
        //         .map(|e| *e)
        //         // Error requesting
        //         .or_else(|err| err.downcast::<hyper::Error>().map(|err| Error::HyperError(*err)))
        //         // Error from another middleware
        //         .unwrap_or_else(|err| Error::Service(err))
        // })?;
    }

    /// Execute the given `request` using octocrab's Client.
    pub async fn execute(
        &self,
        request: http::Request<impl Into<OctoBody>>,
    ) -> Result<http::Response<BoxBody<Bytes, crate::Error>>> {
        let (mut parts, body) = request.into_parts();
        let body: OctoBody = body.into();
        // Saved request that we can retry later if necessary
        let auth_header: Option<HeaderValue> = match self.auth_state {
            AuthState::None => None,
            AuthState::App(ref app) => Some(
                HeaderValue::from_str(format!("Bearer {}", app.generate_bearer_token()?).as_str())
                    .map_err(http::Error::from)
                    .context(HttpSnafu)?,
            ),
            AuthState::BasicAuth {
                ref username,
                ref password,
            } => {
                // Equivalent implementation of: https://github.com/seanmonstar/reqwest/blob/df2b3baadc1eade54b1c22415792b778442673a4/src/util.rs#L3-L23
                use base64::prelude::BASE64_STANDARD;
                use base64::write::EncoderWriter;

                let mut buf = b"Basic ".to_vec();
                {
                    let mut encoder = EncoderWriter::new(&mut buf, &BASE64_STANDARD);
                    write!(encoder, "{username}:{password}").expect("writing to a Vec never fails");
                }
                Some(HeaderValue::from_bytes(&buf).expect("base64 is always valid HeaderValue"))
            }
            AuthState::Installation { ref token, .. } => {
                let token = if let Some(token) = token.valid_token() {
                    token
                } else {
                    self.request_installation_auth_token().await?
                };

                Some(
                    HeaderValue::from_str(format!("Bearer {}", token.expose_secret()).as_str())
                        .map_err(http::Error::from)
                        .context(HttpSnafu)?,
                )
            }
            AuthState::AccessToken { ref token } => Some(
                HeaderValue::from_str(format!("Bearer {}", token.expose_secret()).as_str())
                    .map_err(http::Error::from)
                    .context(HttpSnafu)?,
            ),
        };

        if let Some(mut auth_header) = auth_header {
            // Only set the auth_header if the authority (host) is api.github.com or empty (destined for
            // GitHub). Otherwise, leave it off as we could have been redirected
            // away from GitHub (via follow_location_to_data()), and we don't
            // want to give our credentials to third-party services.
            match parts.uri.authority() {
                None => {
                    auth_header.set_sensitive(true);
                    parts
                        .headers
                        .insert(http::header::AUTHORIZATION, auth_header);
                }
                Some(authority) if authority == "api.github.com" => {
                    auth_header.set_sensitive(true);
                    parts
                        .headers
                        .insert(http::header::AUTHORIZATION, auth_header);
                }
                Some(_) => {
                    // Don't insert auth header.
                }
            }
        }

        let request = http::Request::from_parts(parts, body);

        let response = self.send(request).await?;

        let status = response.status();
        if StatusCode::UNAUTHORIZED == status {
            if let AuthState::Installation { ref token, .. } = self.auth_state {
                token.clear();
            }
        }
        Ok(response)
    }

    pub async fn follow_location_to_data(
        &self,
        response: http::Response<BoxBody<Bytes, Error>>,
    ) -> crate::Result<http::Response<BoxBody<Bytes, crate::Error>>> {
        if let Some(redirect) = response.headers().get(http::header::LOCATION) {
            let location = redirect.to_str().expect("Location URL not valid str");

            self._get(location).await
        } else {
            Ok(response)
        }
    }

    /// Download a file from the given URL with the given content type
    ///
    /// This is a convenience method that sets the `Accept` header to the given
    /// content type and downloads the file into a `Vec<u8>`.
    pub async fn download(
        &self,
        uri: impl TryInto<Uri>,
        content_type: impl TryInto<http::HeaderValue>,
    ) -> crate::Result<Vec<u8>> {
        let uri = uri
            .try_into()
            .map_err(|_| UriParseError {})
            .context(UriParseSnafu)?;
        let content_type = content_type
            .try_into()
            .map_err(|_| UriParseError {})
            .context(UriParseSnafu)?;

        let mut request = Builder::new().method(Method::GET).uri(uri);
        request = request.header(http::header::ACCEPT, content_type);

        let request = self.build_request(request, None::<&()>)?;
        let response = self.execute(request).await?;

        let bytes = response.into_body().collect().await?.to_bytes();
        Ok(bytes.to_vec())
    }

    /// Download a zip file from the given URL into a `Vec<u8>`.
    pub async fn download_zip(&self, uri: impl TryInto<Uri>) -> crate::Result<Vec<u8>> {
        self.download(uri, "application/zip").await
    }
}

/// # Utility Methods
impl Octocrab {
    /// A convenience method to get a page of results (if present).
    pub async fn get_page<R: serde::de::DeserializeOwned>(
        &self,
        uri: &Option<Uri>,
    ) -> crate::Result<Option<Page<R>>> {
        match uri {
            Some(uri) => self.get(uri.to_string(), None::<&()>).await.map(Some),
            None => Ok(None),
        }
    }

    /// A convenience method to get all the results starting at a given
    /// page.
    pub async fn all_pages<R: serde::de::DeserializeOwned>(
        &self,
        mut page: Page<R>,
    ) -> crate::Result<Vec<R>> {
        let mut ret = page.take_items();
        while let Some(mut next_page) = self.get_page(&page.next).await? {
            ret.append(&mut next_page.take_items());
            page = next_page;
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    // tokio runtime seems to be needed for tower: https://users.rust-lang.org/t/no-reactor-running-when-calling-runtime-spawn/81256
    #[tokio::test]
    async fn parametrize_uri_valid() {
        //Previously, invalid characters were handled by url lib's parse function.
        //Todo: should we handle encoding of uri routes ourselves?
        let uri = crate::instance()
            .parameterized_uri("/help%20world", None::<&()>)
            .unwrap();
        assert_eq!(uri.path(), "/help%20world");
    }

    #[tokio::test]
    async fn extra_headers() {
        use http::header::HeaderName;
        use wiremock::{matchers, Mock, MockServer, ResponseTemplate};
        let response = ResponseTemplate::new(304).append_header("etag", "\"abcd\"");
        let mock_server = MockServer::start().await;
        Mock::given(matchers::method("GET"))
            .and(matchers::path_regex(".*"))
            .and(matchers::header("x-test1", "hello"))
            .and(matchers::header("x-test2", "goodbye"))
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;
        crate::OctocrabBuilder::default()
            .base_uri(mock_server.uri())
            .unwrap()
            .add_header(HeaderName::from_static("x-test1"), "hello".to_string())
            .add_header(HeaderName::from_static("x-test2"), "goodbye".to_string())
            .build()
            .unwrap()
            .repos("XAMPPRocky", "octocrab")
            .events()
            .send()
            .await
            .unwrap();
    }

    use super::*;
    use chrono::Duration;

    #[test]
    fn clear_token() {
        let cache = CachedToken(RwLock::new(None));
        cache.set("secret".to_string(), None);
        cache.clear();

        assert!(cache.valid_token().is_none(), "Token was not cleared.");
    }

    #[test]
    fn no_token_when_expired() {
        let cache = CachedToken(RwLock::new(None));
        let expiration = Utc::now() + Duration::seconds(9);
        cache.set("secret".to_string(), Some(expiration));

        assert!(
            cache
                .valid_token_with_buffer(Duration::seconds(10))
                .is_none(),
            "Token should be considered expired due to buffer."
        );
    }

    #[test]
    fn get_valid_token_outside_buffer() {
        let cache = CachedToken(RwLock::new(None));
        let expiration = Utc::now() + Duration::seconds(12);
        cache.set("secret".to_string(), Some(expiration));

        assert!(
            cache
                .valid_token_with_buffer(Duration::seconds(10))
                .is_some(),
            "Token should still be valid outside of buffer."
        );
    }

    #[test]
    fn get_valid_token_without_expiration() {
        let cache = CachedToken(RwLock::new(None));
        cache.set("secret".to_string(), None);

        assert!(
            cache
                .valid_token_with_buffer(Duration::seconds(10))
                .is_some(),
            "Token with no expiration should always be considered valid."
        );
    }
}
