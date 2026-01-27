use std::sync::Arc;

use gpui::{BackgroundExecutor, TestAppContext};
use notifications::NotificationEvent;
use parking_lot::Mutex;
use pretty_assertions::assert_eq;
use rpc::{Notification, proto};

use crate::TestServer;

#[gpui::test]
async fn test_notifications(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    // Wait for authentication/connection to Collab to be established.
    executor.run_until_parked();

    let notification_events_a = Arc::new(Mutex::new(Vec::new()));
    let notification_events_b = Arc::new(Mutex::new(Vec::new()));
    client_a.notification_store().update(cx_a, |_, cx| {
        let events = notification_events_a.clone();
        cx.subscribe(&cx.entity(), move |_, _, event, _| {
            events.lock().push(event.clone());
        })
        .detach()
    });
    client_b.notification_store().update(cx_b, |_, cx| {
        let events = notification_events_b.clone();
        cx.subscribe(&cx.entity(), move |_, _, event, _| {
            events.lock().push(event.clone());
        })
        .detach()
    });

    // Client A sends a contact request to client B.
    client_a
        .user_store()
        .update(cx_a, |store, cx| store.request_contact(client_b.id(), cx))
        .await
        .unwrap();

    // Client B receives a contact request notification and responds to the
    // request, accepting it.
    executor.run_until_parked();
    client_b.notification_store().update(cx_b, |store, cx| {
        assert_eq!(store.notification_count(), 1);
        assert_eq!(store.unread_notification_count(), 1);

        let entry = store.notification_at(0).unwrap();
        assert_eq!(
            entry.notification,
            Notification::ContactRequest {
                sender_id: client_a.id()
            }
        );
        assert!(!entry.is_read);
        assert_eq!(
            &notification_events_b.lock()[0..],
            &[
                NotificationEvent::NewNotification {
                    entry: entry.clone(),
                },
                NotificationEvent::NotificationsUpdated {
                    old_range: 0..0,
                    new_count: 1
                }
            ]
        );

        store.respond_to_notification(entry.notification.clone(), true, cx);
    });

    // Client B sees the notification is now read, and that they responded.
    executor.run_until_parked();
    client_b.notification_store().read_with(cx_b, |store, _| {
        assert_eq!(store.notification_count(), 1);
        assert_eq!(store.unread_notification_count(), 0);

        let entry = store.notification_at(0).unwrap();
        assert!(entry.is_read);
        assert_eq!(entry.response, Some(true));
        assert_eq!(
            &notification_events_b.lock()[2..],
            &[
                NotificationEvent::NotificationRead {
                    entry: entry.clone(),
                },
                NotificationEvent::NotificationsUpdated {
                    old_range: 0..1,
                    new_count: 1
                }
            ]
        );
    });

    // Client A receives a notification that client B accepted their request.
    client_a.notification_store().read_with(cx_a, |store, _| {
        assert_eq!(store.notification_count(), 1);
        assert_eq!(store.unread_notification_count(), 1);

        let entry = store.notification_at(0).unwrap();
        assert_eq!(
            entry.notification,
            Notification::ContactRequestAccepted {
                responder_id: client_b.id()
            }
        );
        assert!(!entry.is_read);
    });

    // Client A creates a channel and invites client B to be a member.
    let channel_id = client_a
        .channel_store()
        .update(cx_a, |store, cx| {
            store.create_channel("the-channel", None, cx)
        })
        .await
        .unwrap();
    client_a
        .channel_store()
        .update(cx_a, |store, cx| {
            store.invite_member(channel_id, client_b.id(), proto::ChannelRole::Member, cx)
        })
        .await
        .unwrap();

    // Client B receives a channel invitation notification and responds to the
    // invitation, accepting it.
    executor.run_until_parked();
    client_b.notification_store().update(cx_b, |store, cx| {
        assert_eq!(store.notification_count(), 2);
        assert_eq!(store.unread_notification_count(), 1);

        let entry = store.notification_at(0).unwrap();
        assert_eq!(
            entry.notification,
            Notification::ChannelInvitation {
                channel_id: channel_id.0,
                channel_name: "the-channel".to_string(),
                inviter_id: client_a.id()
            }
        );
        assert!(!entry.is_read);

        store.respond_to_notification(entry.notification.clone(), true, cx);
    });

    // Client B sees the notification is now read, and that they responded.
    executor.run_until_parked();
    client_b.notification_store().read_with(cx_b, |store, _| {
        assert_eq!(store.notification_count(), 2);
        assert_eq!(store.unread_notification_count(), 0);

        let entry = store.notification_at(0).unwrap();
        assert!(entry.is_read);
        assert_eq!(entry.response, Some(true));
    });
}
