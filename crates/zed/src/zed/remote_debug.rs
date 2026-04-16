use workspace::Workspace;
use zed_actions::remote_debug::{SimulateDisconnect, SimulateTimeout, SimulateTimeoutExhausted};

pub fn init(cx: &mut gpui::App) {
    cx.observe_new(|workspace: &mut Workspace, _, cx| {
        let project = workspace.project().read(cx);
        let Some(remote_client) = project.remote_client() else {
            return;
        };

        workspace.register_action({
            let remote_client = remote_client.downgrade();
            move |_, _: &SimulateDisconnect, _window, cx| {
                let Some(remote_client) = remote_client.upgrade() else {
                    return;
                };

                log::info!("SimulateDisconnect: forcing disconnect from remote server");
                remote_client.update(cx, |client, cx| {
                    client.force_disconnect(cx).detach_and_log_err(cx);
                });
            }
        });

        workspace.register_action({
            let remote_client = remote_client.downgrade();
            move |_, _: &SimulateTimeout, _window, cx| {
                let Some(remote_client) = remote_client.upgrade() else {
                    return;
                };

                log::info!("SimulateTimeout: forcing heartbeat timeout on remote connection");
                remote_client.update(cx, |client, cx| {
                    client.force_heartbeat_timeout(0, cx);
                });
            }
        });

        let remote_client = remote_client.downgrade();
        workspace.register_action(move |_, _: &SimulateTimeoutExhausted, _window, cx| {
            let Some(remote_client) = remote_client.upgrade() else {
                return;
            };

            log::info!("SimulateTimeout: forcing heartbeat timeout on remote connection");
            remote_client.update(cx, |client, cx| {
                client.force_heartbeat_timeout(remote::remote_client::MAX_RECONNECT_ATTEMPTS, cx);
            });
        });
    })
    .detach();
}
