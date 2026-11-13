use super::*;
use futures::FutureExt as _;

#[gpui::test]
async fn test_notification_commands_do_not_wait_for_the_service(cx: &mut gpui::TestAppContext) {
    let (sender, receiver) = channel();
    let mut state = SystemNotificationState {
        initialized: true,
        commands: Some(sender),
        ..SystemNotificationState::new()
    };
    let notification = SystemNotification {
        tag: "thread".into(),
        title: "Finished".into(),
        body: "Ready for input".into(),
        actions: vec![],
    };

    // No service consumes commands until after every UI call has returned.
    let mut tasks = [
        state.send(
            &cx.foreground_executor,
            NotificationCommand::Show(notification.clone()),
        ),
        state.send(
            &cx.foreground_executor,
            NotificationCommand::Dismiss("thread".into()),
        ),
        state.send(
            &cx.foreground_executor,
            NotificationCommand::Show(notification.clone()),
        ),
    ];
    for task in &mut tasks {
        assert!(task.now_or_never().is_none());
    }

    let (commands, completions): (Vec<_>, Vec<_>) = receiver.try_iter().unzip();
    assert_eq!(
        commands,
        vec![
            NotificationCommand::Show(notification.clone()),
            NotificationCommand::Dismiss("thread".into()),
            NotificationCommand::Show(notification),
        ]
    );
    for completed in completions {
        completed
            .send(())
            .expect("caller is waiting for completion");
    }
    for task in tasks {
        task.await;
    }
}
