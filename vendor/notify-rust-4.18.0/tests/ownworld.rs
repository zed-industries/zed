#![cfg(feature = "server")]
#![allow(unused_must_use)]
#![cfg(all(unix, not(target_os = "macos")))]
extern crate notify_rust;

#[cfg(test)]
mod ownworld {

    use notify_rust::server::*;
    use notify_rust::*;
    use std::thread;

    #[test]
    #[ignore]
    fn server_can_be_stopped() {
        let thread_handle = thread::spawn(move || {
            let server = NotificationServer::create();
            NotificationServer::start(&server, |_| {})
        });

        stop_server();
        assert!(thread_handle.join().is_ok());
    }

    #[test]
    #[ignore]
    fn actions_vec() {
        let thread_handle = thread::spawn(move || {
            let server = NotificationServer::create();
            NotificationServer::start(&server, |notification| {
                assert_eq!(notification.actions[0], "actions_vec0");
                assert_eq!(notification.actions[1], "actions_vec1");
                assert_eq!(notification.actions[2], "actions_vec2");
                assert_eq!(notification.actions[3], "actions_vec3");
            })
        });

        #[allow(deprecated)]
        Notification::new()
            .summary("Notification with actions")
            .body("action1=\"Action One\", something_else=\"Something Else\"")
            .icon("dialog-information")
            .actions(vec![
                "actions_vec0".into(),
                "actions_vec1".into(),
                "actions_vec2".into(),
                "actions_vec3".into(),
            ])
            .show();

        stop_server();
        assert!(thread_handle.join().is_ok());
    }

    #[test]
    #[ignore]
    fn actions_automatic() {
        let server = NotificationServer::create();
        let thread_handle = thread::spawn(move || {
            NotificationServer::start(&server, |notification| {
                assert_eq!(notification.actions[0], "actions_built0");
                assert_eq!(notification.actions[1], "actions_built1");
                assert_eq!(notification.actions[2], "actions_built2");
                assert_eq!(notification.actions[3], "actions_built3");
                assert_eq!(notification.timeout, Timeout::Milliseconds(6000));
            })
        });

        Notification::new()
            .summary("Another notification with actions")
            .body("action0=\"Press me please\", action1=\"firefox\"")
            .icon("dialog-information")
            .timeout(6000) //miliseconds
            .action("actions_built0", "actions_built1")
            .action("actions_built2", "actions_built3")
            .show();

        stop_server();
        assert!(thread_handle.join().is_ok());
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn no_server() {
        let server = NotificationServer::create();
        thread::spawn(move || {
            NotificationServer::start(&server, |notification| println!("{:#?}", notification))
        });

        stop_server();
        Notification::new()
            .summary("Another notification with actions")
            .body("action0=\"Press me please\", action1=\"firefox\"")
            .show()
            .unwrap();
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn join_failed() {
        let thread_handle = thread::spawn(|| {
            let server = NotificationServer::create();
            NotificationServer::start(&server, |notification| {
                assert_eq!(notification.timeout, Timeout::Milliseconds(6000));
                assert_eq!(notification.actions[0], "this is no action");
            })
        });

        Notification::new()
            .summary("Notification without timeout")
            .show();
        stop_server();
        assert!(thread_handle.join().is_ok());
    }
}
