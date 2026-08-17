use std::sync::{Arc, Mutex};

use glib::*;

#[derive(Default)]
struct Counters {
    criticals: usize,
    warnings: usize,
    messages: usize,
    infos: usize,
    debugs: usize,
}

fn assert_counts(
    count: &Arc<Mutex<Counters>>,
    criticals: usize,
    warnings: usize,
    messages: usize,
    infos: usize,
    debugs: usize,
) {
    let count = count.lock().expect("failed to lock 1");
    assert_eq!(count.criticals, criticals);
    assert_eq!(count.warnings, warnings);
    assert_eq!(count.messages, messages);
    assert_eq!(count.infos, infos);
    assert_eq!(count.debugs, debugs);
}

#[test]
fn check_log_handlers() {
    // We set the fatal level explicitly in case it's set outside of the test.
    log_set_fatal_mask(Some("domain"), LogLevels::LEVEL_ERROR);

    //
    // log_set_default_handler check part
    //
    let count = Arc::new(Mutex::new(Counters::default()));
    log_set_default_handler(clone!(
        #[weak]
        count,
        move |_, level, _| {
            match level {
                LogLevel::Critical => {
                    count.lock().expect("failed to lock 3").criticals += 1;
                }
                LogLevel::Warning => {
                    count.lock().expect("failed to lock 4").warnings += 1;
                }
                LogLevel::Message => {
                    count.lock().expect("failed to lock 5").messages += 1;
                }
                LogLevel::Info => {
                    count.lock().expect("failed to lock 6").infos += 1;
                }
                LogLevel::Debug => {
                    count.lock().expect("failed to lock 7").debugs += 1;
                }
                _ => unreachable!(),
            }
        }
    ));
    assert_counts(&count, 0, 0, 0, 0, 0);
    g_warning!(Some("domain"), "hello");
    assert_counts(&count, 0, 1, 0, 0, 0);
    g_warning!(Some("domain"), "hello");
    g_critical!(Some("domain"), "hello");
    g_warning!(Some("domain"), "hello");
    g_message!(Some("domain"), "hello");
    g_info!(Some("domain"), "hello");
    g_debug!(Some("domain"), "hello");
    g_info!(Some("domain"), "hello");
    assert_counts(&count, 1, 3, 1, 2, 1);

    // We now unset our callback and check if it has really been unset.
    log_unset_default_handler();
    g_info!(Some("domain"), "hello");
    g_debug!(Some("domain"), "hello");
    assert_counts(&count, 1, 3, 1, 2, 1);

    //
    // log_set_handler check part
    //
    let count = Arc::new(Mutex::new(Counters::default()));
    // We set the handler for both warning and debug.
    let handler_id = log_set_handler(
        Some("domain"),
        LogLevels::LEVEL_WARNING | LogLevels::LEVEL_DEBUG,
        false,
        false,
        clone!(
            #[weak]
            count,
            move |_, level, _| {
                match level {
                    LogLevel::Warning => {
                        count.lock().expect("failed to lock 4").warnings += 1;
                    }
                    LogLevel::Debug => {
                        count.lock().expect("failed to lock 7").debugs += 1;
                    }
                    _ => unreachable!(),
                }
            }
        ),
    );
    assert_counts(&count, 0, 0, 0, 0, 0);
    g_warning!(Some("domain"), "hello");
    assert_counts(&count, 0, 1, 0, 0, 0);
    g_critical!(Some("domain"), "hello");
    g_message!(Some("domain"), "hello");
    g_info!(Some("domain"), "hello");
    assert_counts(&count, 0, 1, 0, 0, 0);
    g_debug!(Some("domain"), "hello");
    assert_counts(&count, 0, 1, 0, 0, 1);
    // We check that only "domain" messages are calling our callback.
    g_debug!(Some("not-domain"), "hello");
    g_warning!(Some("not-domain"), "hello");
    assert_counts(&count, 0, 1, 0, 0, 1);

    log_remove_handler(Some("domain"), handler_id);
    g_critical!(Some("domain"), "hello");
    g_message!(Some("domain"), "hello");
    g_info!(Some("domain"), "hello");
    g_debug!(Some("domain"), "hello");
    g_warning!(Some("domain"), "hello");
    assert_counts(&count, 0, 1, 0, 0, 1);
}
