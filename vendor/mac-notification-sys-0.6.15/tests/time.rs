use mac_notification_sys::*;

#[test]
fn dont_schedule_in_past() {
    let stamp = time::OffsetDateTime::now_utc().unix_timestamp() as f64 - 5.;
    let sent = send_notification(
        "Danger",
        Some("Will Robinson"),
        "Run away as fast as you can",
        Some(
            Notification::new()
                .sound("Blow")
                .delivery_date(stamp)
                .asynchronous(true),
        ),
    );
    assert!(sent.is_err())
}
