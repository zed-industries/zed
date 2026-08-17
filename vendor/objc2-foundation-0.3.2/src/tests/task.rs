#![cfg(feature = "NSTask")]
use crate::{NSObjectProtocol, NSTask};

use objc2::{sel, ClassType};

#[test]
#[cfg_attr(not(target_vendor = "apple"), ignore = "only on Apple")]
fn class_cluster_and_wait_method() {
    // This method happens to only be available on the concrete subclass NSConcreteTask.
    let sel = sel!(waitUntilExit);

    let method = NSTask::class().instance_method(sel);
    assert!(method.is_none(), "class does not have method");

    let task = NSTask::new();
    assert!(task.respondsToSelector(sel), "object has method");
}
