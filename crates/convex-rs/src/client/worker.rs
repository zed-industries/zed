use std::{
    collections::BTreeMap,
    convert::Infallible,
    time::Duration,
};

use convex_sync_types::{
    backoff::Backoff,
    AuthenticationToken,
    UdfPath,
};
use futures::{
    stream::FusedStream,
    StreamExt,
};
use tokio::sync::{
    broadcast,
    mpsc,
    oneshot,
};
use tokio_stream::wrappers::{
    BroadcastStream,
    ReceiverStream,
    UnboundedReceiverStream,
};

use crate::{
    base_client::{
        BaseConvexClient,
        SubscriberId,
    },
    client::{
        QueryResults,
        QuerySubscription,
    },
    sync::{
        ProtocolResponse,
        ReconnectProtocolReason,
        ReconnectRequest,
        SyncProtocol,
    },
    value::Value,
    FunctionResult,
};

const INITIAL_BACKOFF: Duration = Duration::from_millis(100);
const MAX_BACKOFF: Duration = Duration::from_secs(15);

pub enum ClientRequest {
    Mutation(
        MutationRequest,
        oneshot::Sender<oneshot::Receiver<FunctionResult>>,
    ),
    Action(
        ActionRequest,
        oneshot::Sender<oneshot::Receiver<FunctionResult>>,
    ),
    Subscribe(
        SubscribeRequest,
        oneshot::Sender<QuerySubscription>,
        mpsc::UnboundedSender<ClientRequest>,
    ),
    Unsubscribe(UnsubscribeRequest),
    Authenticate(AuthenticateRequest),
}

pub struct MutationRequest {
    pub udf_path: UdfPath,
    pub args: BTreeMap<String, Value>,
}

pub struct ActionRequest {
    pub udf_path: UdfPath,
    pub args: BTreeMap<String, Value>,
}

pub struct SubscribeRequest {
    pub udf_path: UdfPath,
    pub args: BTreeMap<String, Value>,
}

pub struct AuthenticateRequest {
    pub token: AuthenticationToken,
}

#[derive(Debug)]
pub struct UnsubscribeRequest {
    pub subscriber_id: SubscriberId,
}

pub async fn worker<T: SyncProtocol>(
    protocol_response_receiver: mpsc::Receiver<ProtocolResponse>,
    client_request_receiver: mpsc::UnboundedReceiver<ClientRequest>,
    mut watch_sender: broadcast::Sender<QueryResults>,
    mut base_client: BaseConvexClient,
    mut protocol_manager: T,
) -> Infallible {
    let mut backoff = Backoff::new(INITIAL_BACKOFF, MAX_BACKOFF);
    let mut protocol_response_stream = ReceiverStream::new(protocol_response_receiver).fuse();
    let mut client_request_stream = UnboundedReceiverStream::new(client_request_receiver).fuse();
    loop {
        let e = loop {
            match _worker_once(
                &mut protocol_response_stream,
                &mut client_request_stream,
                &mut watch_sender,
                &mut base_client,
                &mut protocol_manager,
            )
            .await
            {
                Ok(()) => backoff.reset(),
                Err(e) => break e,
            }
        };

        let delay = backoff.fail(&mut rand::rng());
        tracing::error!(
            "Convex Client Worker failed: {e:?}. Backing off for {delay:?} and retrying."
        );
        // Tell the sync protocol to reconnect followed by an immediate resend of
        // ongoing queries/mutations. It's important these happen together to
        // ensure mutation ordering.
        protocol_manager
            .reconnect(ReconnectRequest {
                reason: e,
                max_observed_timestamp: base_client.max_observed_timestamp(),
            })
            .await;
        base_client.resend_ongoing_queries_mutations();
        flush_messages(&mut base_client, &mut protocol_manager).await;
        tokio::time::sleep(delay).await;
    }
}

async fn _worker_once<T: SyncProtocol>(
    protocol_response_stream: impl FusedStream<Item = ProtocolResponse>,
    client_request_stream: impl FusedStream<Item = ClientRequest>,
    watch_sender: &mut broadcast::Sender<QueryResults>,
    base_client: &mut BaseConvexClient,
    protocol_manager: &mut T,
) -> Result<(), ReconnectProtocolReason> {
    tokio::pin!(protocol_response_stream);
    tokio::pin!(client_request_stream);
    tokio::select! {
        Some(protocol_response) = protocol_response_stream.next(),
        if !protocol_response_stream.is_terminated() => {
            match protocol_response {
                ProtocolResponse::ServerMessage(msg) => {
                    if let Some(subscriber_id_to_latest_value) = base_client.receive_message(msg)? {
                        // Notify watchers of the new consistent query results at new timestamp
                        let _ = watch_sender.send(subscriber_id_to_latest_value);
                    }
                },
                ProtocolResponse::Failure => {
                    return Err("ProtocolFailure".into());
                },
            }
        }
        Some(client_request) = client_request_stream.next(),
        if !client_request_stream.is_terminated() => {
            match client_request {
                ClientRequest::Subscribe(query, tx, request_sender) => {
                    let watch = watch_sender.subscribe();
                    let SubscribeRequest {
                        udf_path,
                        args,
                    } =  query;
                    let subscriber_id = base_client.subscribe(udf_path, args);
                    flush_messages(base_client, protocol_manager).await;

                    let watch = BroadcastStream::new(watch);
                    let subscription = QuerySubscription {
                        subscriber_id,
                        request_sender,
                        watch,
                        initial: base_client.latest_results().get(&subscriber_id).cloned(),
                    };
                    let _ = tx.send(subscription);
                },
                ClientRequest::Mutation(mutation, tx) => {
                    let MutationRequest {
                        udf_path,
                        args,
                    } = mutation;
                    let result_receiver = base_client
                        .mutation(udf_path, args);
                    flush_messages(base_client, protocol_manager).await;
                    let _ = tx.send(result_receiver);
                },
                ClientRequest::Action(action, tx) => {
                    let ActionRequest {
                        udf_path,
                        args,
                    } = action;
                    let result_receiver = base_client
                        .action(udf_path, args);
                    flush_messages(base_client, protocol_manager).await;
                    let _ = tx.send(result_receiver);
                },
                ClientRequest::Unsubscribe(unsubscribe) => {
                    let UnsubscribeRequest {subscriber_id} = unsubscribe;
                    base_client.unsubscribe(subscriber_id);
                    flush_messages(base_client, protocol_manager).await;
                },
                ClientRequest::Authenticate(authenticate) => {
                    base_client.set_auth(authenticate.token);
                    flush_messages(base_client, protocol_manager).await;
                },
            }
        },
        else => (),
    }
    Ok(())
}

/// Flush all messages to the protocol
async fn flush_messages<P: SyncProtocol>(base_client: &mut BaseConvexClient, protocol: &mut P) {
    while let Some(modification) = base_client.pop_next_message() {
        let _ = protocol.send(modification).await;
    }
}
