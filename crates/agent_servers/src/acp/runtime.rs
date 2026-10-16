use super::{
    AcpDebugLog, ClientContext, ForegroundWork, drain_foreground_queue,
    exited_load_error_after_drain, settle_after_disconnect,
};
use acp_thread::LoadError;
use agent_client_protocol::{Error, Result as RpcResult};
use anyhow::Result;
use futures::{
    Future, FutureExt as _, StreamExt as _,
    channel::{mpsc, oneshot},
    future::{BoxFuture, LocalBoxFuture, Shared},
};
use gpui::{AppContext as _, AsyncApp, Task};
use std::{process::ExitStatus, time::Duration};
use util::ResultExt as _;

type DisconnectCleanup = Shared<BoxFuture<'static, RpcResult<()>>>;
pub(super) type ProcessStatus = LocalBoxFuture<'static, Result<ExitStatus>>;

pub(super) struct ClientRuntime {
    dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
    io_task: Option<Task<()>>,
    dispatch_task: Option<Task<()>>,
    shutdown: Option<oneshot::Sender<()>>,
}

impl ClientRuntime {
    pub(super) fn new(context: ClientContext, cx: &mut AsyncApp) -> (Self, oneshot::Receiver<()>) {
        let (dispatch_tx, mut dispatch_rx) = mpsc::unbounded::<ForegroundWork>();
        let dispatch_task = cx.spawn(async move |cx| {
            while let Some(work) = dispatch_rx.next().await {
                work.run(cx, &context);
            }
        });
        let (shutdown, received) = oneshot::channel();
        (
            Self {
                dispatch_tx,
                io_task: None,
                dispatch_task: Some(dispatch_task),
                shutdown: Some(shutdown),
            },
            received,
        )
    }

    pub(super) fn sender(&self) -> mpsc::UnboundedSender<ForegroundWork> {
        self.dispatch_tx.clone()
    }

    pub(super) fn start_io(
        &mut self,
        future: impl Future<Output = RpcResult<()>> + Send + 'static,
        cx: &AsyncApp,
    ) {
        self.io_task = Some(cx.background_spawn(async move {
            future.await.log_err();
        }));
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(super) fn from_test_tasks(
        dispatch_tx: mpsc::UnboundedSender<ForegroundWork>,
        io_task: Task<()>,
        dispatch_task: Task<()>,
    ) -> Self {
        Self {
            dispatch_tx,
            io_task: Some(io_task),
            dispatch_task: Some(dispatch_task),
            shutdown: None,
        }
    }
}

impl Drop for ClientRuntime {
    fn drop(&mut self) {
        // Raw test drivers have no cooperative shutdown callback and retain cancel-on-drop.
        if self.shutdown.take().is_some() {
            // SDK close callbacks must settle foreground owners before the dispatch task exits.
            if let Some(task) = self.io_task.take() {
                task.detach();
            }
            if let Some(task) = self.dispatch_task.take() {
                task.detach();
            }
        }
    }
}

pub(super) fn disconnect_cleanup(
    sender: mpsc::UnboundedSender<ForegroundWork>,
) -> DisconnectCleanup {
    settle_after_disconnect(sender).boxed().shared()
}

pub(super) async fn publish_connection<Connection>(
    connection: Connection,
    sender: oneshot::Sender<Connection>,
    shutdown: impl Future<Output = ()>,
) -> RpcResult<()> {
    if sender.send(connection).is_err() {
        return Err(Error::internal_error().data("ACP connection receiver dropped"));
    }
    shutdown.await;
    Ok(())
}

pub(super) async fn finish_connection(
    future: impl Future<Output = RpcResult<()>>,
    cleanup: DisconnectCleanup,
) -> RpcResult<()> {
    // Transport failures can bypass on_close; both paths await this one shared cleanup.
    let result = future.await;
    let cleanup_result = cleanup.await;
    if let Err(error) = &cleanup_result {
        log::warn!("ACP disconnect cleanup failed: {error}");
    }
    result?;
    cleanup_result
}

pub(super) async fn shutdown_requested(received: oneshot::Receiver<()>) {
    if received.await.is_err() {
        log::debug!("ACP connection owner dropped");
    }
}

pub(super) async fn race_startup<Value, Drain: Future<Output = ()>>(
    initialized: impl Future<Output = Result<Value>>,
    status: ProcessStatus,
    drained: Drain,
    debug_log: &AcpDebugLog,
    cx: &AsyncApp,
) -> Result<(Value, ProcessStatus, Drain)> {
    futures::pin_mut!(initialized);
    match futures::future::select(initialized, status).await {
        futures::future::Either::Left((Ok(value), status)) => Ok((value, status, drained)),
        futures::future::Either::Left((Err(error), status)) => {
            let timeout = cx.background_executor().timer(Duration::from_millis(250));
            futures::pin_mut!(timeout);
            if let futures::future::Either::Left((status, _)) =
                futures::future::select(status, timeout).await
            {
                return Err(
                    exited_load_error_after_drain(status?, drained, debug_log, cx)
                        .await
                        .into(),
                );
            }
            Err(error)
        }
        futures::future::Either::Right((status, _)) => Err(exited_load_error_after_drain(
            status?, drained, debug_log, cx,
        )
        .await
        .into()),
    }
}

pub(super) async fn drain_transport(
    incoming_closed: impl Future<Output = ()>,
    sender: mpsc::UnboundedSender<ForegroundWork>,
    stderr: Shared<Task<()>>,
) {
    let incoming = async move {
        incoming_closed.await;
        drain_foreground_queue(&sender).await.log_err();
    };
    futures::join!(incoming, stderr);
}

pub(super) fn watch_process(
    status: ProcessStatus,
    drained: impl Future<Output = ()> + 'static,
    debug_log: AcpDebugLog,
    on_exit: impl FnOnce(LoadError, &mut AsyncApp) + 'static,
    cx: &AsyncApp,
) -> Task<()> {
    cx.spawn(async move |cx| match status.await {
        Ok(status) => {
            let error = exited_load_error_after_drain(status, drained, &debug_log, cx).await;
            on_exit(error, cx);
        }
        Err(error) => log::error!("ACP process wait failed: {error}"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acp::v1::connect_client_future;
    use acp_thread::{ElicitationStatus, ElicitationStore};
    use agent_client_protocol::schema::v2 as acp;
    use collections::HashMap;
    use gpui::TestAppContext;
    use std::{cell::RefCell, rc::Rc};

    #[gpui::test]
    async fn shared_runtime_v1_owner_drop_settles_retained_input(cx: &mut TestAppContext) {
        let store = cx.new(|_| ElicitationStore::default());
        let weak_store = store.downgrade();
        let context = ClientContext {
            sessions: Rc::new(RefCell::new(HashMap::default())),
            session_list: Rc::new(RefCell::new(None)),
            request_elicitations: store.clone(),
            active_request_scopes: None,
        };
        let (mut runtime, shutdown) = ClientRuntime::new(context, &mut cx.to_async());
        let (transport, peer) = agent_client_protocol::Channel::duplex();
        let (sender, received) = oneshot::channel();
        runtime.start_io(
            connect_client_future(
                "runtime-test",
                transport,
                runtime.sender(),
                sender,
                shutdown_requested(shutdown),
            ),
            &cx.to_async(),
        );
        let connection = received.await.expect("SDK handle");
        let (entry, response) = store.update(cx, |store, cx| {
            store
                .request_elicitation_with_id(
                    acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(
                            acp::ElicitationRequestScope::new(acp::RequestId::Number(1)),
                            acp::ElicitationSchema::new().string("name", true),
                        ),
                        "Retained input",
                    ),
                    cx,
                )
                .expect("retained input")
        });
        drop(runtime);
        let timeout = cx.background_executor.timer(Duration::from_secs(5));
        futures::pin_mut!(response, timeout);
        let response = match futures::future::select(response, timeout).await {
            futures::future::Either::Left((response, _)) => response,
            futures::future::Either::Right(_) => panic!("owner drop did not settle input"),
        };
        assert_eq!(response.action, acp::ElicitationAction::Cancel);
        store.read_with(cx, |store, _| {
            assert!(matches!(
                store.elicitation(&entry).expect("retained entry").1.status,
                ElicitationStatus::Canceled
            ));
        });
        drop(connection);
        drop(peer);
        drop(store);
        cx.run_until_parked();
        assert!(weak_store.upgrade().is_none(), "foreground owner leaked");
    }

    #[test]
    fn shared_runtime_abandoned_handle_rejects_without_waiting_for_shutdown() {
        let (sender, received) = oneshot::channel();
        drop(received);
        let error =
            futures::executor::block_on(publish_connection((), sender, futures::future::pending()))
                .expect_err("abandoned handle");
        assert_eq!(
            error.data,
            Some(serde_json::json!("ACP connection receiver dropped"))
        );
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn shared_runtime_startup_exit_drains_stderr_in_both_arrival_orders(
        cx: &mut TestAppContext,
    ) {
        use crate::acp::debug_log::AcpDebugMessageDirection;
        use std::os::unix::process::ExitStatusExt as _;

        for initialize_fails_first in [false, true] {
            let debug_log = AcpDebugLog::default();
            let drained = {
                let debug_log = debug_log.clone();
                async move {
                    debug_log.record_line(AcpDebugMessageDirection::Stderr, "runtime final stderr");
                }
            };
            let initialized = async move {
                if initialize_fails_first {
                    Err::<(), _>(anyhow::anyhow!("initialize failed"))
                } else {
                    futures::future::pending().await
                }
            };
            let status = async { Ok(ExitStatus::from_raw(7 << 8)) }.boxed_local();
            let error = race_startup(initialized, status, drained, &debug_log, &cx.to_async())
                .await
                .err()
                .expect("startup exit");
            let LoadError::Exited { status, stderr } = error
                .downcast_ref::<LoadError>()
                .expect("process exit supersedes initialization error")
            else {
                panic!("process exit error");
            };
            assert_eq!(status.code(), Some(7));
            assert_eq!(stderr.as_deref(), Some("runtime final stderr"));
        }
    }

    #[gpui::test]
    async fn shared_runtime_startup_preserves_error_while_process_remains_live(
        cx: &mut TestAppContext,
    ) {
        let error = race_startup(
            async { Err::<(), _>(anyhow::anyhow!("initialize rejected")) },
            futures::future::pending().boxed_local(),
            futures::future::pending(),
            &AcpDebugLog::default(),
            &cx.to_async(),
        )
        .await
        .err()
        .expect("initialization failure");
        assert_eq!(error.to_string(), "initialize rejected");
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn shared_runtime_child_guard_kills_an_abandoned_process(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let mut command = std::process::Command::new("/bin/sh");
        command.args(["-c", "IFS= read -r request"]);
        let child = util::process::Child::spawn(
            command,
            std::process::Stdio::piped(),
            std::process::Stdio::null(),
            std::process::Stdio::null(),
        )
        .expect("spawn waiting child");
        let mut child = crate::acp::transport::OwnedChild(child);
        let status = child.0.status();
        drop(child);
        let timeout = cx.background_executor.timer(Duration::from_secs(5));
        futures::pin_mut!(status, timeout);
        let status = match futures::future::select(status, timeout).await {
            futures::future::Either::Left((status, _)) => status.expect("wait for killed child"),
            futures::future::Either::Right(_) => panic!("child survived its owner"),
        };
        use std::os::unix::process::ExitStatusExt as _;
        assert_eq!(status.signal(), Some(9));
    }
}
