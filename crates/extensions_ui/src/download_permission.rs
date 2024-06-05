use std::collections::VecDeque;

use futures::{channel::mpsc, StreamExt};
use gpui::{Model, VisualContext};
use ui::{Context, SharedString, ViewContext};
use workspace::{
    notifications::{simple_message_notification, NotificationId},
    Workspace,
};

pub(crate) fn download_permission(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
    let mut request_rx = workspace.app_state().languages.download_request_rx();
    let notification_queue: Model<VecDeque<(String, mpsc::UnboundedSender<bool>)>> =
        cx.new_model(|_| VecDeque::new());

    cx.observe(&notification_queue, |workspace, queue, cx| {
        if queue.read(cx).is_empty() {
            return;
        }

        while let Some((url, response_tx)) = queue.update(cx, |queue, cx| {
            let request = queue.pop_front();
            cx.notify();
            request
        }) {
            struct DownloadRequestNotification;

            let notification_id = NotificationId::identified::<DownloadRequestNotification>(
                SharedString::from(url.clone()),
            );
            workspace.show_notification(notification_id, cx, |cx| {
                let response_tx_clone = response_tx.clone();
                cx.new_view(|_cx| {
                    simple_message_notification::MessageNotification::new(format!(
                        "Allow '{url}' to be downloaded?"
                    ))
                    .with_click_message("Yes")
                    .on_click(move |_| {
                        response_tx.unbounded_send(true).unwrap();
                    })
                    .with_secondary_click_message("No")
                    .on_secondary_click(move |_| {
                        response_tx_clone.unbounded_send(false).unwrap();
                    })
                })
            });
        }
    })
    .detach();

    cx.spawn(|_workspace, mut cx| async move {
        while let Some((url, response_tx)) = request_rx.next().await {
            notification_queue
                .update(&mut cx, |queue, cx| {
                    queue.push_back((url, response_tx));
                    cx.notify();
                })
                .unwrap();
        }
    })
    .detach();
}
