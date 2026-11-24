use super::ClientContext;
use agent_client_protocol::{v1::schema as v1, v2::schema as v2};
use gpui::{App, AsyncApp, Task};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub(super) struct ConnectionAuthStatus {
    snapshots: RefCell<watch::Sender<Option<v2::AuthStatusResponse>>>,
    generation: Cell<u64>,
    query: RefCell<Option<Task<()>>>,
}

impl ConnectionAuthStatus {
    pub(super) fn new() -> Rc<Self> {
        Rc::new(Self {
            snapshots: RefCell::new(watch::channel(None).0),
            generation: Cell::new(0),
            query: RefCell::new(None),
        })
    }

    pub(super) fn receiver(&self) -> watch::Receiver<Option<v2::AuthStatusResponse>> {
        self.snapshots.borrow().receiver()
    }

    pub(super) fn query(
        self: &Rc<Self>,
        response: impl Future<Output = Option<v2::AuthStatusResponse>> + 'static,
        cx: &App,
    ) {
        let generation = self.generation.get();
        let status = Rc::downgrade(self);
        *self.query.borrow_mut() = Some(cx.foreground_executor().spawn(async move {
            let response = response.await;
            if let Some(status) = status.upgrade()
                && status.generation.get() == generation
            {
                // An optional probe failing isn't a failed login or a connection error.
                status.publish(
                    response.unwrap_or_else(|| v2::AuthStatusResponse::new(v2::AuthState::Unknown)),
                );
            }
        }));
    }

    fn publish(&self, snapshot: v2::AuthStatusResponse) {
        // Even an identical notification is a newer observation than an outstanding query.
        self.generation.set(self.generation.get() + 1);
        // The latest value is retained even when no view is watching.
        self.snapshots.borrow_mut().send(Some(snapshot)).ok();
    }

    pub(super) fn disconnect(&self) {
        self.query.borrow_mut().take();
        self.publish(v2::AuthStatusResponse::new(v2::AuthState::Unknown));
    }
}

pub(super) fn from_v1(
    state: v1::AuthState,
    accounts: Vec<v1::AuthAccount>,
) -> v2::AuthStatusResponse {
    let state = match state {
        v1::AuthState::Authenticated => v2::AuthState::Authenticated,
        v1::AuthState::Unauthenticated => v2::AuthState::Unauthenticated,
        v1::AuthState::NotRequired => v2::AuthState::NotRequired,
        _ => v2::AuthState::Unknown,
    };
    v2::AuthStatusResponse::new(state).accounts(
        accounts
            .into_iter()
            .map(|account| {
                v2::AuthAccount::new(account.label)
                    .active(account.active)
                    .description(account.description)
                    .name(account.name)
                    .email(account.email)
                    .organization(account.organization)
            })
            .collect::<Vec<_>>(),
    )
}

pub(super) fn update_v1(
    update: v1::AuthStatusUpdateNotification,
    _: &mut AsyncApp,
    context: &ClientContext,
) {
    if let Some(status) = context.auth_status.upgrade() {
        status.publish(from_v1(update.state, update.accounts));
    }
}

pub(super) fn update_v2(
    update: v2::AuthStatusUpdateNotification,
    _: &mut AsyncApp,
    context: &ClientContext,
) {
    if let Some(status) = context.auth_status.upgrade() {
        status.publish(v2::AuthStatusResponse::new(update.state).accounts(update.accounts));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn notification_supersedes_pending_auth_status_query(cx: &mut TestAppContext) {
        let status = ConnectionAuthStatus::new();
        let (reply, pending) = futures::channel::oneshot::channel();
        cx.update(|cx| status.query(async move { pending.await.ok() }, cx));
        let updated = v2::AuthStatusResponse::new(v2::AuthState::Unauthenticated);
        status.publish(updated.clone());
        reply
            .send(v2::AuthStatusResponse::new(v2::AuthState::Authenticated))
            .expect("pending query");
        cx.run_until_parked();
        assert_eq!(*status.receiver().borrow(), Some(updated));
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn auth_status_stdio_is_negotiated_and_connection_scoped(cx: &mut TestAppContext) {
        use crate::acp::{connect, set_acp_beta};
        use collections::HashMap;
        use gpui::{FutureExt as _, WeakEntity};
        use project::{
            AgentId, Project,
            agent_server_store::{AgentServerCommand, ResolvedAgentServerCommand},
        };
        use serde_json::json;
        use std::time::Duration;

        cx.update(|cx| {
            let mut settings = settings::SettingsStore::test(cx);
            settings.register_setting::<feature_flags::FeatureFlagsSettings>();
            cx.set_global(settings);
            release_channel::init("0.0.0".parse().expect("version"), cx);
        });
        cx.executor().allow_parking();
        let directory = tempfile::tempdir().expect("project");
        let project = Project::example([directory.path()], &mut cx.to_async()).await;
        for (version, beta, advertised) in [
            (1, true, true),
            (2, true, true),
            (1, false, true),
            (2, true, false),
        ] {
            cx.update(|cx| set_acp_beta(beta, cx));
            let mut auth = json!({"logout": {}});
            if advertised {
                auth["status"] = json!({});
            }
            let init = if version == 1 {
                json!({"protocolVersion":1,"agentCapabilities":{"auth":auth},"authMethods":[{"id":"login","name":"Login"}]})
            } else {
                json!({"protocolVersion":2,"info":{"name":"test","version":"1"},"capabilities":{"auth":auth},"authMethods":[{"type":"agent","methodId":"login","name":"Login"}]})
            };
            let command = AgentServerCommand {
                path: "/bin/sh".into(),
                args: vec!["-c".into(), r#"
queries=0
while IFS= read -r request; do
    id=${request#*\"id\":}; id=${id%%,*}
    case "$request" in
        *'"method":"initialize"'*) result=$INIT ;;
        *'"method":"auth/status"'*) queries=$((queries+1)); result='{"state":"unauthenticated"}' ;;
        *'"method":"logout"'*|*'"method":"auth/logout"'*)
            [ "$queries" = "$EXPECTED_QUERIES" ] || exit 42
            printf '%s\n' '{"jsonrpc":"2.0","method":"auth/status_update","params":{"state":"authenticated","accounts":[{"label":"Environment key","active":true}]}}'
            result='{}' ;;
        *) exit 43 ;;
    esac
    printf '{"jsonrpc":"2.0","id":%s,"result":%s}\n' "$id" "$result"
done
"#.into()],
                env: Some(HashMap::from_iter([
                    ("INIT".into(), init.to_string()),
                    ("EXPECTED_QUERIES".into(), usize::from(beta && advertised).to_string()),
                ])),
            };
            let connection = cx
                .spawn({
                    let project = project.clone();
                    async move |mut cx| {
                        connect(
                            AgentId::new("auth-status-test"),
                            project,
                            ResolvedAgentServerCommand::without_installation_lease(command),
                            WeakEntity::new_invalid(),
                            None,
                            HashMap::default(),
                            &mut cx,
                        )
                        .await
                    }
                })
                .await
                .expect("connect");
            let mut reports = connection.auth_status();
            assert_eq!(reports.is_some(), beta && advertised);
            if let Some(reports) = &mut reports {
                if reports.borrow().is_none() {
                    reports
                        .changed()
                        .with_timeout(Duration::from_secs(5), &cx.executor())
                        .await
                        .expect("query timeout")
                        .expect("status");
                }
                assert_eq!(
                    *reports.borrow(),
                    Some(v2::AuthStatusResponse::new(v2::AuthState::Unauthenticated))
                );
            }
            cx.update(|cx| connection.logout(cx)).await.expect("logout");
            if let Some(reports) = &mut reports {
                reports
                    .changed()
                    .with_timeout(Duration::from_secs(5), &cx.executor())
                    .await
                    .expect("notification timeout")
                    .expect("status");
                assert_eq!(
                    *reports.borrow(),
                    Some(
                        v2::AuthStatusResponse::new(v2::AuthState::Authenticated)
                            .accounts(vec![v2::AuthAccount::new("Environment key").active(true)])
                    )
                );
            }
            connection.shutdown();
        }
    }
}
