use std::sync::{Arc, Mutex};

use glib::*;

// Funny thing: we can't put those two tests in two different functions, otherwise they might
// conflict with the results of the other one (or it would be mandatory to run the tests on only
// one thread).
#[test]
fn check_print_handler() {
    //
    // g_print check part
    //
    let count = Arc::new(Mutex::new(0));
    set_print_handler(clone!(
        #[weak]
        count,
        move |_| {
            // we don't care about the message in here!
            *count.lock().expect("failed to lock 1") += 1;
        }
    ));
    g_print!("test");
    assert_eq!(*count.lock().expect("failed to lock 2"), 1);
    g_printerr!("one");
    assert_eq!(*count.lock().expect("failed to lock 3"), 1);
    g_print!("another");
    assert_eq!(*count.lock().expect("failed to lock 4"), 2);
    unset_print_handler();
    g_print!("tadam");
    assert_eq!(*count.lock().expect("failed to lock 5"), 2);
    g_printerr!("toudoum");
    assert_eq!(*count.lock().expect("failed to lock 6"), 2);

    //
    // g_printerr check part
    //
    let count = Arc::new(Mutex::new(0));
    set_printerr_handler(clone!(
        #[weak]
        count,
        move |_| {
            // we don't care about the message in here!
            *count.lock().expect("failed to lock a") += 1;
        }
    ));
    g_printerr!("test");
    assert_eq!(*count.lock().expect("failed to lock b"), 1);
    g_print!("one");
    assert_eq!(*count.lock().expect("failed to lock c"), 1);
    g_printerr!("another");
    assert_eq!(*count.lock().expect("failed to lock d"), 2);
    unset_printerr_handler();
    g_printerr!("tadam");
    assert_eq!(*count.lock().expect("failed to lock e"), 2);
    g_print!("toudoum");
    assert_eq!(*count.lock().expect("failed to lock f"), 2);
}
