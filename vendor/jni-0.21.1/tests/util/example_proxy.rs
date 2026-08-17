#![allow(dead_code)]

use jni::{
    errors::*,
    objects::{GlobalRef, JValue},
    sys::jint,
    Executor, JNIEnv,
};

/// A test example of a native-to-JNI proxy
#[derive(Clone)]
pub struct AtomicIntegerProxy {
    exec: Executor,
    obj: GlobalRef,
}

impl AtomicIntegerProxy {
    /// Creates a new instance of `AtomicIntegerProxy`
    pub fn new(exec: Executor, init_value: jint) -> Result<Self> {
        let obj = exec.with_attached(|env: &mut JNIEnv| {
            let i = env.new_object(
                "java/util/concurrent/atomic/AtomicInteger",
                "(I)V",
                &[JValue::from(init_value)],
            )?;
            env.new_global_ref(i)
        })?;
        Ok(AtomicIntegerProxy { exec, obj })
    }

    /// Gets a current value from java object
    pub fn get(&mut self) -> Result<jint> {
        self.exec
            .with_attached(|env| env.call_method(&self.obj, "get", "()I", &[])?.i())
    }

    /// Increments a value of java object and then gets it
    pub fn increment_and_get(&mut self) -> Result<jint> {
        self.exec.with_attached(|env| {
            env.call_method(&self.obj, "incrementAndGet", "()I", &[])?
                .i()
        })
    }

    /// Adds some value to the value of java object and then gets a resulting value
    pub fn add_and_get(&mut self, delta: jint) -> Result<jint> {
        let delta = JValue::from(delta);
        self.exec.with_attached(|env| {
            env.call_method(&self.obj, "addAndGet", "(I)I", &[delta])?
                .i()
        })
    }
}
