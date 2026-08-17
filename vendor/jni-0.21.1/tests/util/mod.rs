use std::sync::{Arc, Once};

use jni::{
    errors::Result, objects::JValue, sys::jint, AttachGuard, InitArgsBuilder, JNIEnv, JNIVersion,
    JavaVM,
};

mod example_proxy;
pub use self::example_proxy::AtomicIntegerProxy;

pub fn jvm() -> &'static Arc<JavaVM> {
    static mut JVM: Option<Arc<JavaVM>> = None;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option("-Xcheck:jni")
            .build()
            .unwrap_or_else(|e| panic!("{:#?}", e));

        let jvm = JavaVM::new(jvm_args).unwrap_or_else(|e| panic!("{:#?}", e));

        unsafe {
            JVM = Some(Arc::new(jvm));
        }
    });

    unsafe { JVM.as_ref().unwrap() }
}

#[allow(dead_code)]
pub fn call_java_abs(env: &mut JNIEnv, value: i32) -> i32 {
    env.call_static_method(
        "java/lang/Math",
        "abs",
        "(I)I",
        &[JValue::from(value as jint)],
    )
    .unwrap()
    .i()
    .unwrap()
}

#[allow(dead_code)]
pub fn attach_current_thread() -> AttachGuard<'static> {
    jvm()
        .attach_current_thread()
        .expect("failed to attach jvm thread")
}

#[allow(dead_code)]
pub fn attach_current_thread_as_daemon() -> JNIEnv<'static> {
    jvm()
        .attach_current_thread_as_daemon()
        .expect("failed to attach jvm daemon thread")
}

#[allow(dead_code)]
pub fn attach_current_thread_permanently() -> JNIEnv<'static> {
    jvm()
        .attach_current_thread_permanently()
        .expect("failed to attach jvm thread permanently")
}

#[allow(dead_code)]
pub unsafe fn detach_current_thread() {
    jvm().detach_current_thread()
}

pub fn print_exception(env: &JNIEnv) {
    let exception_occurred = env.exception_check().unwrap_or_else(|e| panic!("{:?}", e));
    if exception_occurred {
        env.exception_describe()
            .unwrap_or_else(|e| panic!("{:?}", e));
    }
}

#[allow(dead_code)]
pub fn unwrap<T>(res: Result<T>, env: &JNIEnv) -> T {
    res.unwrap_or_else(|e| {
        print_exception(env);
        panic!("{:#?}", e);
    })
}
