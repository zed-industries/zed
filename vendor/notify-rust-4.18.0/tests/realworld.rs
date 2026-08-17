#![allow(unused_must_use)]
extern crate notify_rust;

#[cfg(test)]
mod realworld {

    use std::time::Duration;

    #[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
    use notify_rust::Image;
    use notify_rust::*;

    static _INIT: std::sync::LazyLock<()> = std::sync::LazyLock::new(|| {
        color_backtrace::install();
    });

    #[test]
    fn burst() {
        for msg in &[
            "These should each",
            "come in their own pop up.",
            "If they don't than",
            "I will have to complain about it.",
        ] {
            Notification::new()
                .summary("burst")
                .appname(msg)
                .body(msg)
                .icon("media-floppy")
                .show()
                .unwrap();
        }

        for msg in &[
            "These may be grouped",
            "together by the server.",
            "that is because the all have the same",
            "appname.",
        ] {
            Notification::new()
                .summary("merged burst")
                .body(msg)
                .icon("applications-toys")
                .show()
                .unwrap();
        }
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn closing() {
        Notification::new()
            .summary("You see me")
            .body("you don't see me!")
            .show()
            .unwrap()
            .close();
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn capabilities() {
        let capabilities: Vec<String> = get_capabilities().unwrap();
        for capability in capabilities {
            Notification::new()
                .summary("capability")
                .body(&capability)
                .show()
                .unwrap();
        }
    }

    #[test]
    fn build_pattern() {
        let notification = Notification::new().summary("foo").finalize();
        assert_eq!(notification.summary, "foo");

        let mut notification = Notification::new();
        notification.body = "foo".to_string();
        assert_eq!(notification.body, "foo");

        let mut notification = Notification::new();
        notification.icon = "foo".to_string();
        assert_eq!(notification.icon, "foo");

        let mut notification = Notification::new();
        notification.summary = "foo".to_string();
        assert_eq!(notification.summary, "foo");

        let mut notification = Notification::new();
        notification.timeout = Timeout::Milliseconds(42);
        assert_eq!(notification.timeout, Timeout::Milliseconds(21 * 2));

        let mut notification = Notification::new();
        notification.summary = "foo".to_string();
        assert_eq!(notification.summary, "foo");
    }

    #[test]
    fn init() {
        let mut message = Notification::new();
        message.summary("invocation type 2");
        message.body("your <b>body</b> is a <u>wonderland</u>");
        message.show().unwrap();

        Notification::new()
            .summary("this is the summary")
            .summary("invocation type 3")
            .body("this is the body\nnewline<br/>linebreak")
            .show()
            .unwrap();
    }

    #[test]
    fn should_allow_timeout_with_duration() {
        let mut notification = Notification::new();
        notification.timeout(Duration::from_secs(15));
        assert!(matches!(
            notification.timeout,
            Timeout::Milliseconds(15_000)
        ));
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn urgency() {
        // use it this way
        use Urgency::*;
        for urgency in &[
            Hint::Urgency(Low),
            Hint::Urgency(Normal),
            Hint::Urgency(Critical),
        ] {
            Notification::new()
                .summary(&format!("Urgency {:?}", urgency))
                .hint(urgency.clone())
                .show()
                .unwrap();
        }
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn category() {
        Notification::new()
            .appname("thunderbird")
            .summary("Category:email")
            .icon("thunderbird")
            .hint(Hint::Category("email".to_string()))
            .show()
            .unwrap();
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn persistent() {
        Notification::new()
            .summary("Incoming Call: Your Mom!")
            .body("Resident:True")
            .icon("call-start")
            .hint(Hint::Resident(true))
            .show()
            .unwrap();

        Notification::new()
            .summary("Incoming Call: Your Mom!")
            .body("Resident:False, but Timeout=0")
            .icon("call-start")
            .hint(Hint::Resident(false))
            .timeout(0)
            .show()
            .unwrap();
    }

    #[test]
    #[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
    fn imagedata() {
        let mut data: Vec<u8> = vec![0 as u8; 64 * 64 * 3];
        for x in 0..64 {
            for y in 0..64 {
                let offset = (y * 64 + x) * 3;
                data[offset] = (x + y) as u8;
                data[offset + 1] = x as u8;
                data[offset + 2] = y as u8;
            }
        }

        Notification::new()
            .summary("I can haz image data!")
            .hint(Hint::ImageData(Image::from_rgb(64, 64, data).unwrap()))
            .show()
            .unwrap();
    }
}
