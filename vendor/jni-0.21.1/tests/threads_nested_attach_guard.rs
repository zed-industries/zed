#![cfg(feature = "invocation")]

mod util;
use util::{
    attach_current_thread, attach_current_thread_as_daemon, attach_current_thread_permanently,
    call_java_abs, jvm,
};

#[test]
pub fn nested_attaches_should_not_detach_guarded_thread() {
    assert_eq!(jvm().threads_attached(), 0);
    let mut env = attach_current_thread();
    let val = call_java_abs(&mut env, -1);
    assert_eq!(val, 1);
    assert_eq!(jvm().threads_attached(), 1);

    // Create nested AttachGuard.
    {
        let mut env_nested = attach_current_thread();
        let val = call_java_abs(&mut env_nested, -2);
        assert_eq!(val, 2);
        assert_eq!(jvm().threads_attached(), 1);
    }

    // Call a Java method after nested guard has been dropped to check that
    // this thread has not been detached.
    let val = call_java_abs(&mut env, -3);
    assert_eq!(val, 3);
    assert_eq!(jvm().threads_attached(), 1);

    // Nested attach_permanently is a no-op.
    {
        let mut env_nested = attach_current_thread_permanently();
        let val = call_java_abs(&mut env_nested, -4);
        assert_eq!(val, 4);
        assert_eq!(jvm().threads_attached(), 1);
    }
    assert_eq!(jvm().threads_attached(), 1);

    // Nested attach_as_daemon is a no-op.
    {
        let mut env_nested = attach_current_thread_as_daemon();
        let val = call_java_abs(&mut env_nested, -5);
        assert_eq!(val, 5);
        assert_eq!(jvm().threads_attached(), 1);
    }
    assert_eq!(jvm().threads_attached(), 1);

    // Check that after guard is dropped the thread is properly detached
    // despite nested "permanent" attaches.
    drop(env);
    assert_eq!(jvm().threads_attached(), 0);
    assert!(jvm().get_env().is_err());
}
