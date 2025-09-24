use gpui::Global;
use std::{collections::BTreeMap, convert::Infallible, sync::Arc};

use convex_sync_types::{AuthenticationToken, UdfPath, UserIdentityAttributes};
#[cfg(doc)]
use futures::Stream;
use futures::StreamExt;
use tokio::{
    sync::{broadcast, mpsc, oneshot},
    task::JoinHandle,
};
use tokio_stream::wrappers::BroadcastStream;
use url::Url;

use self::worker::AuthenticateRequest;
#[cfg(doc)]
use crate::SubscriberId;
use crate::{
    base_client::{BaseConvexClient, QueryResults},
    client::{
        subscription::{QuerySetSubscription, QuerySubscription},
        worker::{worker, ActionRequest, ClientRequest, MutationRequest, SubscribeRequest},
    },
    sync::{web_socket_manager::WebSocketManager, SyncProtocol, WebSocketState},
    value::Value,
    FunctionResult,
};

pub mod subscription;
mod worker;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

/// An asynchronous client to interact with a specific project to perform
/// mutations and manage query subscriptions using [`tokio`].
///
/// The Convex client requires a deployment url,
/// which can be found in the [dashboard](https://dashboard.convex.dev/) settings tab.
///
/// ```no_run
/// use convex::ConvexClient;
/// use futures::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let mut client = ConvexClient::new("https://cool-music-123.convex.cloud").await?;
///     let mut sub = client.subscribe("listMessages", maplit::btreemap!{}).await?;
///     while let Some(result) = sub.next().await {
///         println!("{result:?}");
///     }
///     Ok(())
/// }
/// ```
///
/// The [`ConvexClient`] internally holds a connection and a [`tokio`]
/// background task to manage it. It is advised that you create one and
/// **reuse** it. You can safely clone with [`ConvexClient::clone()`] to share
/// the connection and outstanding subscriptions.
///
/// ## Examples
/// For example code, please refer to the examples directory.
pub struct ConvexClient {
    listen_handle: Option<Arc<JoinHandle<Infallible>>>,
    request_sender: mpsc::UnboundedSender<ClientRequest>,
    watch_receiver: broadcast::Receiver<QueryResults>,
}

/// Clone the [`ConvexClient`], sharing the connection and outstanding
/// subscriptions.
impl Clone for ConvexClient {
    fn clone(&self) -> Self {
        Self {
            listen_handle: self.listen_handle.clone(),
            request_sender: self.request_sender.clone(),
            watch_receiver: self.watch_receiver.resubscribe(),
        }
    }
}

/// Drop the [`ConvexClient`]. When the final reference to the [`ConvexClient`]
/// is dropped, the connection is cleaned up.
impl Drop for ConvexClient {
    fn drop(&mut self) {
        if let Ok(j_handle) = Arc::try_unwrap(
            self.listen_handle
                .take()
                .expect("INTERNAL BUG: listen handle should never be none"),
        ) {
            j_handle.abort()
        }
    }
}

impl Global for ConvexClient {}

impl ConvexClient {
    /// Constructs a new client for communicating with `deployment_url`.
    ///
    /// ```no_run
    /// # use convex::ConvexClient;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = ConvexClient::new("https://cool-music-123.convex.cloud").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(deployment_url: &str) -> anyhow::Result<Self> {
        ConvexClient::new_from_builder(ConvexClientBuilder::new(deployment_url)).await
    }

    #[doc(hidden)]
    pub async fn new_from_builder(builder: ConvexClientBuilder) -> anyhow::Result<Self> {
        let client_id = builder
            .client_id
            .unwrap_or_else(|| format!("rust-{}", VERSION.unwrap_or("unknown")));
        let ws_url = deployment_to_ws_url(builder.deployment_url.as_str().try_into()?)?;

        // Channels for the `listen` background thread
        let (response_sender, response_receiver) = mpsc::channel(1);
        let (request_sender, request_receiver) = mpsc::unbounded_channel();

        // Listener for when each transaction completes
        let (watch_sender, watch_receiver) = broadcast::channel(1);

        let base_client = BaseConvexClient::new();

        let protocol = WebSocketManager::open(
            ws_url,
            response_sender,
            builder.on_state_change,
            client_id.as_str(),
        )
        .await?;

        let listen_handle = tokio::spawn(worker(
            response_receiver,
            request_receiver,
            watch_sender,
            base_client,
            protocol,
        ));
        let client = ConvexClient {
            listen_handle: Some(Arc::new(listen_handle)),
            request_sender,
            watch_receiver,
        };
        Ok(client)
    }

    /// Subscribe to the results of query `name` called with `args`.
    ///
    /// Returns a [`QuerySubscription`] which implements [`Stream`]<
    /// [`FunctionResult`]>. A new value appears on the stream each
    /// time the query function produces a new result.
    ///
    /// The subscription is automatically unsubscribed when it is dropped.
    ///
    /// ```no_run
    /// # use convex::ConvexClient;
    /// # use futures::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let mut client = ConvexClient::new("https://cool-music-123.convex.cloud").await?;
    /// let mut sub = client.subscribe("listMessages", maplit::btreemap!{}).await?;
    /// while let Some(result) = sub.next().await {
    ///     println!("{result:?}");
    /// }
    /// # Ok(())
    /// # }
    pub async fn subscribe(
        &mut self,
        name: &str,
        args: BTreeMap<String, Value>,
    ) -> anyhow::Result<QuerySubscription> {
        let (tx, rx) = oneshot::channel();

        let udf_path = name.parse()?;
        let request = SubscribeRequest { udf_path, args };

        self.request_sender.send(ClientRequest::Subscribe(
            request,
            tx,
            self.request_sender.clone(),
        ))?;

        let res = rx.await?;
        Ok(res)
    }

    /// Make a oneshot request to a query `name` with `args`.
    ///
    /// Returns a [`FunctionResult`] representing the result of the query.
    ///
    /// This method is syntactic sugar for waiting for a single result on
    /// a subscription.
    /// It is equivalent to `client.subscribe(name,
    /// args).await?.next().unwrap()`
    ///
    /// ```no_run
    /// # use convex::ConvexClient;
    /// # use futures::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let mut client = ConvexClient::new("https://cool-music-123.convex.cloud").await?;
    /// let result = client.query("listMessages", maplit::btreemap!{}).await?;
    /// println!("{result:?}");
    /// # Ok(())
    /// # }
    pub async fn query(
        &mut self,
        name: &str,
        args: BTreeMap<String, Value>,
    ) -> anyhow::Result<FunctionResult> {
        Ok(self
            .subscribe(name, args)
            .await?
            .next()
            .await
            .expect("INTERNAL BUG: Convex Client dropped prematurely."))
    }

    /// Perform a mutation `name` with `args` and return a future
    /// containing the return value of the mutation once it completes.
    ///
    /// ```no_run
    /// # use convex::ConvexClient;
    /// # use futures::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let mut client = ConvexClient::new("https://cool-music-123.convex.cloud").await?;
    /// let result = client.mutation("sendMessage", maplit::btreemap!{
    ///     "body".into() => "Let it be.".into(),
    ///     "author".into() => "The Beatles".into(),
    /// }).await?;
    /// println!("{result:?}");
    /// # Ok(())
    /// # }
    pub async fn mutation(
        &mut self,
        name: &str,
        args: BTreeMap<String, Value>,
    ) -> anyhow::Result<FunctionResult> {
        let (tx, rx) = oneshot::channel();

        let udf_path: UdfPath = name.parse()?;
        let request = MutationRequest { udf_path, args };

        self.request_sender
            .send(ClientRequest::Mutation(request, tx))?;

        let res = rx.await?;
        Ok(res.await?)
    }

    /// Perform an action `name` with `args` and return a future
    /// containing the return value of the action once it completes.
    ///
    /// ```no_run
    /// # use convex::ConvexClient;
    /// # use futures::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let mut client = ConvexClient::new("https://cool-music-123.convex.cloud").await?;
    /// let result = client.action("sendGif", maplit::btreemap!{
    ///     "body".into() => "Tatooine Sunrise.".into(),
    ///     "author".into() => "Luke Skywalker".into(),
    /// }).await?;
    /// println!("{result:?}");
    /// # Ok(())
    /// # }
    pub async fn action(
        &mut self,
        name: &str,
        args: BTreeMap<String, Value>,
    ) -> anyhow::Result<FunctionResult> {
        let (tx, rx) = oneshot::channel();

        let udf_path: UdfPath = name.parse()?;
        let request = ActionRequest { udf_path, args };

        self.request_sender
            .send(ClientRequest::Action(request, tx))?;

        let res = rx.await?;
        Ok(res.await?)
    }

    /// Get a consistent view of the results of multiple queries (query set).
    ///
    /// Returns a [`QuerySetSubscription`] which
    /// implements [`Stream`]<[`QueryResults`]>.
    /// Each item in the stream contains a consistent view
    /// of the results of all the queries in the query set.
    ///
    /// Queries can be added to the query set via [`ConvexClient::subscribe`].
    /// Queries can be removed from the query set via dropping the
    /// [`QuerySubscription`] token returned by [`ConvexClient::subscribe`].
    ///
    ///
    /// [`QueryResults`] is a copy-on-write mapping from [`SubscriberId`] to
    /// its latest result [`Value`].
    ///
    /// ```no_run
    /// # use convex::ConvexClient;
    /// # use futures::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let mut client = ConvexClient::new("https://cool-music-123.convex.cloud").await?;
    /// let mut watch = client.watch_all();
    /// let sub1 = client.subscribe("listMessages", maplit::btreemap!{
    ///     "channel".into() => 1.into(),
    /// }).await?;
    /// let sub2 = client.subscribe("listMessages", maplit::btreemap!{
    ///     "channel".into() => 1.into(),
    /// }).await?;
    /// # Ok(())
    /// # }
    pub fn watch_all(&self) -> QuerySetSubscription {
        QuerySetSubscription::new(BroadcastStream::new(self.watch_receiver.resubscribe()))
    }

    /// Set auth for use when calling Convex functions.
    ///
    /// Set it with a token that you get from your auth provider via their login
    /// flow. If `None` is passed as the token, then auth is unset (logging
    /// out).
    pub async fn set_auth(&mut self, token: Option<String>) {
        let req = AuthenticateRequest {
            token: match token {
                None => AuthenticationToken::None,
                Some(token) => AuthenticationToken::User(token),
            },
        };
        self.request_sender
            .send(ClientRequest::Authenticate(req))
            .expect("INTERNAL BUG: Worker has gone away");
    }

    /// Set admin auth for use when calling Convex functions as a deployment
    /// admin. Not typically required.
    ///
    /// You can get a deploy_key from the Convex dashboard's deployment settings
    /// page. Deployment admins can act as users as part of their
    /// development flow to see how a function would act.
    #[doc(hidden)]
    pub async fn set_admin_auth(
        &mut self,
        deploy_key: String,
        acting_as: Option<UserIdentityAttributes>,
    ) {
        let req = AuthenticateRequest {
            token: AuthenticationToken::Admin(deploy_key, acting_as),
        };
        self.request_sender
            .send(ClientRequest::Authenticate(req))
            .expect("INTERNAL BUG: Worker has gone away");
    }
}

fn deployment_to_ws_url(mut deployment_url: Url) -> anyhow::Result<Url> {
    let ws_scheme = match deployment_url.scheme() {
        "http" | "ws" => "ws",
        "https" | "wss" => "wss",
        scheme => anyhow::bail!("Unknown scheme {scheme}. Expected http or https."),
    };
    deployment_url
        .set_scheme(ws_scheme)
        .expect("Scheme not supported");
    deployment_url.set_path("api/sync");
    Ok(deployment_url)
}

/// A builder for creating a [`ConvexClient`] with custom configuration.
pub struct ConvexClientBuilder {
    deployment_url: String,
    client_id: Option<String>,
    on_state_change: Option<mpsc::Sender<WebSocketState>>,
}

impl ConvexClientBuilder {
    /// Create a new [`ConvexClientBuilder`] with the given deployment URL.
    pub fn new(deployment_url: &str) -> Self {
        Self {
            deployment_url: deployment_url.to_string(),
            client_id: None,
            on_state_change: None,
        }
    }

    /// Set a custom client ID for this client.
    pub fn with_client_id(mut self, client_id: &str) -> Self {
        self.client_id = Some(client_id.to_string());
        self
    }

    /// Set a channel to be notified of changes to the WebSocket connection
    /// state.
    pub fn with_on_state_change(mut self, on_state_change: mpsc::Sender<WebSocketState>) -> Self {
        self.on_state_change = Some(on_state_change);
        self
    }

    /// Build the [`ConvexClient`] with the configured options.
    ///
    /// ```no_run
    /// # use convex::ConvexClientBuilder;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = ConvexClientBuilder::new("https://cool-music-123.convex.cloud").build().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn build(self) -> anyhow::Result<ConvexClient> {
        ConvexClient::new_from_builder(self).await
    }
}
