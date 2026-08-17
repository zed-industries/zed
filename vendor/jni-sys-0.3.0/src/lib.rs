#![doc(html_root_url="https://docs.rs/jni-sys/0.3.0")]
#![allow(non_snake_case, non_camel_case_types)]

use std::os::raw::c_void;
use std::os::raw::c_char;

// FIXME is this sufficiently correct?
pub type va_list = *mut c_void;

pub type jint = i32;
pub type jlong = i64;
pub type jbyte = i8;
pub type jboolean = u8;
pub type jchar = u16;
pub type jshort = i16;
pub type jfloat = f32;
pub type jdouble = f64;
pub type jsize = jint;

pub enum _jobject {}
pub type jobject = *mut _jobject;
pub type jclass = jobject;
pub type jthrowable = jobject;
pub type jstring = jobject;
pub type jarray = jobject;
pub type jbooleanArray = jarray;
pub type jbyteArray = jarray;
pub type jcharArray = jarray;
pub type jshortArray = jarray;
pub type jintArray = jarray;
pub type jlongArray = jarray;
pub type jfloatArray = jarray;
pub type jdoubleArray = jarray;
pub type jobjectArray = jarray;
pub type jweak = jobject;

#[repr(C)]
#[derive(Copy)]
pub union jvalue {
    pub z: jboolean,
    pub b: jbyte,
    pub c: jchar,
    pub s: jshort,
    pub i: jint,
    pub j: jlong,
    pub f: jfloat,
    pub d: jdouble,
    pub l: jobject,
}

impl Clone for jvalue {
    fn clone(&self) -> Self {
        *self
    }
}

pub enum _jfieldID {}
pub type jfieldID = *mut _jfieldID;
pub enum _jmethodID {}
pub type jmethodID = *mut _jmethodID;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jobjectRefType {
    JNIInvalidRefType = 0,
    JNILocalRefType = 1,
    JNIGlobalRefType = 2,
    JNIWeakGlobalRefType = 3,
}

pub const JNI_FALSE: jboolean = 0;
pub const JNI_TRUE: jboolean = 1;

pub const JNI_OK: jint = 0;
pub const JNI_ERR: jint = -1;
pub const JNI_EDETACHED: jint = -2;
pub const JNI_EVERSION: jint = -3;
pub const JNI_ENOMEM: jint = -4;
pub const JNI_EEXIST: jint = -5;
pub const JNI_EINVAL: jint = -6;

pub const JNI_COMMIT: jint = 1;
pub const JNI_ABORT: jint = 2;

pub const JNI_VERSION_1_1: jint = 0x00010001;
pub const JNI_VERSION_1_2: jint = 0x00010002;
pub const JNI_VERSION_1_4: jint = 0x00010004;
pub const JNI_VERSION_1_6: jint = 0x00010006;
pub const JNI_VERSION_1_8: jint = 0x00010008;

#[repr(C)]
#[derive(Copy)]
pub struct JNINativeMethod {
    pub name: *mut c_char,
    pub signature: *mut c_char,
    pub fnPtr: *mut c_void,
}

impl Clone for JNINativeMethod {
    fn clone(&self) -> Self {
        *self
    }
}

pub type JNIEnv = *const JNINativeInterface_;
pub type JavaVM = *const JNIInvokeInterface_;

#[repr(C)]
#[derive(Copy)]
pub struct JNINativeInterface_ {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub reserved3: *mut c_void,
    pub GetVersion: Option<unsafe extern "system" fn(env: *mut JNIEnv) -> jint>,
    pub DefineClass: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  name: *const c_char,
                                  loader: jobject,
                                  buf: *const jbyte,
                                  len: jsize)
                                  -> jclass,
    >,
    pub FindClass:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, name: *const c_char) -> jclass>,
    pub FromReflectedMethod:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, method: jobject) -> jmethodID>,
    pub FromReflectedField:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, field: jobject) -> jfieldID>,
    pub ToReflectedMethod: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  cls: jclass,
                                  methodID: jmethodID,
                                  isStatic: jboolean)
                                  -> jobject,
    >,
    pub GetSuperclass: Option<unsafe extern "system" fn(env: *mut JNIEnv, sub: jclass) -> jclass>,
    pub IsAssignableFrom:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, sub: jclass, sup: jclass) -> jboolean>,
    pub ToReflectedField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  cls: jclass,
                                  fieldID: jfieldID,
                                  isStatic: jboolean)
                                  -> jobject,
    >,
    pub Throw: Option<unsafe extern "system" fn(env: *mut JNIEnv, obj: jthrowable) -> jint>,
    pub ThrowNew: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  msg: *const c_char)
                                  -> jint,
    >,
    pub ExceptionOccurred: Option<unsafe extern "system" fn(env: *mut JNIEnv) -> jthrowable>,
    pub ExceptionDescribe: Option<unsafe extern "system" fn(env: *mut JNIEnv)>,
    pub ExceptionClear: Option<unsafe extern "system" fn(env: *mut JNIEnv)>,
    pub FatalError: Option<unsafe extern "system" fn(env: *mut JNIEnv, msg: *const c_char) -> !>,
    pub PushLocalFrame: Option<unsafe extern "system" fn(env: *mut JNIEnv, capacity: jint) -> jint>,
    pub PopLocalFrame:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, result: jobject) -> jobject>,
    pub NewGlobalRef: Option<unsafe extern "system" fn(env: *mut JNIEnv, lobj: jobject) -> jobject>,
    pub DeleteGlobalRef: Option<unsafe extern "system" fn(env: *mut JNIEnv, gref: jobject)>,
    pub DeleteLocalRef: Option<unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject)>,
    pub IsSameObject: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj1: jobject,
                                  obj2: jobject)
                                  -> jboolean,
    >,
    pub NewLocalRef: Option<unsafe extern "system" fn(env: *mut JNIEnv, ref_: jobject) -> jobject>,
    pub EnsureLocalCapacity:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, capacity: jint) -> jint>,
    pub AllocObject: Option<unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass) -> jobject>,
    pub NewObject: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jobject,
    >,
    pub NewObjectV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jobject,
    >,
    pub NewObjectA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jobject,
    >,
    pub GetObjectClass: Option<unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> jclass>,
    pub IsInstanceOf:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, clazz: jclass) -> jboolean>,
    pub GetMethodID: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  name: *const c_char,
                                  sig: *const c_char)
                                  -> jmethodID,
    >,
    pub CallObjectMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             methodID: jmethodID,
                             ...)
                             -> jobject,
    >,
    pub CallObjectMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jobject,
    >,
    pub CallObjectMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jobject,
    >,
    pub CallBooleanMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             methodID: jmethodID,
                             ...)
                             -> jboolean,
    >,
    pub CallBooleanMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jboolean,
    >,
    pub CallBooleanMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jboolean,
    >,
    pub CallByteMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             methodID: jmethodID,
                             ...)
                             -> jbyte,
    >,
    pub CallByteMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jbyte,
    >,
    pub CallByteMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jbyte,
    >,
    pub CallCharMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             methodID: jmethodID,
                             ...)
                             -> jchar,
    >,
    pub CallCharMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jchar,
    >,
    pub CallCharMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jchar,
    >,
    pub CallShortMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             methodID: jmethodID,
                             ...)
                             -> jshort,
    >,
    pub CallShortMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jshort,
    >,
    pub CallShortMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jshort,
    >,
    pub CallIntMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             methodID: jmethodID,
                             ...)
                             -> jint,
    >,
    pub CallIntMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jint,
    >,
    pub CallIntMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jint,
    >,
    pub CallLongMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             methodID: jmethodID,
                             ...)
                             -> jlong,
    >,
    pub CallLongMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jlong,
    >,
    pub CallLongMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jlong,
    >,
    pub CallFloatMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             methodID: jmethodID,
                             ...)
                             -> jfloat,
    >,
    pub CallFloatMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jfloat,
    >,
    pub CallFloatMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jfloat,
    >,
    pub CallDoubleMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             methodID: jmethodID,
                             ...)
                             -> jdouble,
    >,
    pub CallDoubleMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jdouble,
    >,
    pub CallDoubleMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jdouble,
    >,
    pub CallVoidMethod:
        Option<unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...)>,
    pub CallVoidMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: va_list),
    >,
    pub CallVoidMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  methodID: jmethodID,
                                  args: *const jvalue),
    >,
    pub CallNonvirtualObjectMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jobject,
    >,
    pub CallNonvirtualObjectMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jobject,
    >,
    pub CallNonvirtualObjectMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jobject,
    >,
    pub CallNonvirtualBooleanMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jboolean,
    >,
    pub CallNonvirtualBooleanMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jboolean,
    >,
    pub CallNonvirtualBooleanMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jboolean,
    >,
    pub CallNonvirtualByteMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jbyte,
    >,
    pub CallNonvirtualByteMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jbyte,
    >,
    pub CallNonvirtualByteMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jbyte,
    >,
    pub CallNonvirtualCharMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jchar,
    >,
    pub CallNonvirtualCharMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jchar,
    >,
    pub CallNonvirtualCharMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jchar,
    >,
    pub CallNonvirtualShortMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jshort,
    >,
    pub CallNonvirtualShortMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jshort,
    >,
    pub CallNonvirtualShortMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jshort,
    >,
    pub CallNonvirtualIntMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jint,
    >,
    pub CallNonvirtualIntMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jint,
    >,
    pub CallNonvirtualIntMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jint,
    >,
    pub CallNonvirtualLongMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jlong,
    >,
    pub CallNonvirtualLongMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jlong,
    >,
    pub CallNonvirtualLongMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jlong,
    >,
    pub CallNonvirtualFloatMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jfloat,
    >,
    pub CallNonvirtualFloatMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jfloat,
    >,
    pub CallNonvirtualFloatMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jfloat,
    >,
    pub CallNonvirtualDoubleMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jdouble,
    >,
    pub CallNonvirtualDoubleMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jdouble,
    >,
    pub CallNonvirtualDoubleMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jdouble,
    >,
    pub CallNonvirtualVoidMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             obj: jobject,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...),
    >,
    pub CallNonvirtualVoidMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list),
    >,
    pub CallNonvirtualVoidMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue),
    >,
    pub GetFieldID: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  name: *const c_char,
                                  sig: *const c_char)
                                  -> jfieldID,
    >,
    pub GetObjectField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID)
                                  -> jobject,
    >,
    pub GetBooleanField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID)
                                  -> jboolean,
    >,
    pub GetByteField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID)
                                  -> jbyte,
    >,
    pub GetCharField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID)
                                  -> jchar,
    >,
    pub GetShortField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID)
                                  -> jshort,
    >,
    pub GetIntField:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jint>,
    pub GetLongField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID)
                                  -> jlong,
    >,
    pub GetFloatField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID)
                                  -> jfloat,
    >,
    pub GetDoubleField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID)
                                  -> jdouble,
    >,
    pub SetObjectField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID,
                                  val: jobject),
    >,
    pub SetBooleanField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID,
                                  val: jboolean),
    >,
    pub SetByteField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID,
                                  val: jbyte),
    >,
    pub SetCharField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID,
                                  val: jchar),
    >,
    pub SetShortField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID,
                                  val: jshort),
    >,
    pub SetIntField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID,
                                  val: jint),
    >,
    pub SetLongField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID,
                                  val: jlong),
    >,
    pub SetFloatField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID,
                                  val: jfloat),
    >,
    pub SetDoubleField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  obj: jobject,
                                  fieldID: jfieldID,
                                  val: jdouble),
    >,
    pub GetStaticMethodID: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  name: *const c_char,
                                  sig: *const c_char)
                                  -> jmethodID,
    >,
    pub CallStaticObjectMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jobject,
    >,
    pub CallStaticObjectMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jobject,
    >,
    pub CallStaticObjectMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jobject,
    >,
    pub CallStaticBooleanMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jboolean,
    >,
    pub CallStaticBooleanMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jboolean,
    >,
    pub CallStaticBooleanMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jboolean,
    >,
    pub CallStaticByteMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jbyte,
    >,
    pub CallStaticByteMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jbyte,
    >,
    pub CallStaticByteMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jbyte,
    >,
    pub CallStaticCharMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jchar,
    >,
    pub CallStaticCharMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jchar,
    >,
    pub CallStaticCharMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jchar,
    >,
    pub CallStaticShortMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jshort,
    >,
    pub CallStaticShortMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jshort,
    >,
    pub CallStaticShortMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jshort,
    >,
    pub CallStaticIntMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jint,
    >,
    pub CallStaticIntMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jint,
    >,
    pub CallStaticIntMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jint,
    >,
    pub CallStaticLongMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jlong,
    >,
    pub CallStaticLongMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jlong,
    >,
    pub CallStaticLongMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jlong,
    >,
    pub CallStaticFloatMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jfloat,
    >,
    pub CallStaticFloatMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jfloat,
    >,
    pub CallStaticFloatMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jfloat,
    >,
    pub CallStaticDoubleMethod: Option<
        unsafe extern "C" fn(env: *mut JNIEnv,
                             clazz: jclass,
                             methodID: jmethodID,
                             ...)
                             -> jdouble,
    >,
    pub CallStaticDoubleMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: va_list)
                                  -> jdouble,
    >,
    pub CallStaticDoubleMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue)
                                  -> jdouble,
    >,
    pub CallStaticVoidMethod:
        Option<unsafe extern "C" fn(env: *mut JNIEnv, cls: jclass, methodID: jmethodID, ...)>,
    pub CallStaticVoidMethodV: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  cls: jclass,
                                  methodID: jmethodID,
                                  args: va_list),
    >,
    pub CallStaticVoidMethodA: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  cls: jclass,
                                  methodID: jmethodID,
                                  args: *const jvalue),
    >,
    pub GetStaticFieldID: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  name: *const c_char,
                                  sig: *const c_char)
                                  -> jfieldID,
    >,
    pub GetStaticObjectField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID)
                                  -> jobject,
    >,
    pub GetStaticBooleanField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID)
                                  -> jboolean,
    >,
    pub GetStaticByteField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID)
                                  -> jbyte,
    >,
    pub GetStaticCharField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID)
                                  -> jchar,
    >,
    pub GetStaticShortField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID)
                                  -> jshort,
    >,
    pub GetStaticIntField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID)
                                  -> jint,
    >,
    pub GetStaticLongField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID)
                                  -> jlong,
    >,
    pub GetStaticFloatField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID)
                                  -> jfloat,
    >,
    pub GetStaticDoubleField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID)
                                  -> jdouble,
    >,
    pub SetStaticObjectField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID,
                                  value: jobject),
    >,
    pub SetStaticBooleanField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID,
                                  value: jboolean),
    >,
    pub SetStaticByteField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID,
                                  value: jbyte),
    >,
    pub SetStaticCharField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID,
                                  value: jchar),
    >,
    pub SetStaticShortField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID,
                                  value: jshort),
    >,
    pub SetStaticIntField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID,
                                  value: jint),
    >,
    pub SetStaticLongField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID,
                                  value: jlong),
    >,
    pub SetStaticFloatField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID,
                                  value: jfloat),
    >,
    pub SetStaticDoubleField: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  fieldID: jfieldID,
                                  value: jdouble),
    >,
    pub NewString: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  unicode: *const jchar,
                                  len: jsize)
                                  -> jstring,
    >,
    pub GetStringLength: Option<unsafe extern "system" fn(env: *mut JNIEnv, str: jstring) -> jsize>,
    pub GetStringChars: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  str: jstring,
                                  isCopy: *mut jboolean)
                                  -> *const jchar,
    >,
    pub ReleaseStringChars:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, str: jstring, chars: *const jchar)>,
    pub NewStringUTF:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, utf: *const c_char) -> jstring>,
    pub GetStringUTFLength:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, str: jstring) -> jsize>,
    pub GetStringUTFChars: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  str: jstring,
                                  isCopy: *mut jboolean)
                                  -> *const c_char,
    >,
    pub ReleaseStringUTFChars:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, str: jstring, chars: *const c_char)>,
    pub GetArrayLength: Option<unsafe extern "system" fn(env: *mut JNIEnv, array: jarray) -> jsize>,
    pub NewObjectArray: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  len: jsize,
                                  clazz: jclass,
                                  init: jobject)
                                  -> jobjectArray,
    >,
    pub GetObjectArrayElement: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jobjectArray,
                                  index: jsize)
                                  -> jobject,
    >,
    pub SetObjectArrayElement: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jobjectArray,
                                  index: jsize,
                                  val: jobject),
    >,
    pub NewBooleanArray:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jbooleanArray>,
    pub NewByteArray: Option<unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jbyteArray>,
    pub NewCharArray: Option<unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jcharArray>,
    pub NewShortArray:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jshortArray>,
    pub NewIntArray: Option<unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jintArray>,
    pub NewLongArray: Option<unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jlongArray>,
    pub NewFloatArray:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jfloatArray>,
    pub NewDoubleArray:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, len: jsize) -> jdoubleArray>,
    pub GetBooleanArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jbooleanArray,
                                  isCopy: *mut jboolean)
                                  -> *mut jboolean,
    >,
    pub GetByteArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jbyteArray,
                                  isCopy: *mut jboolean)
                                  -> *mut jbyte,
    >,
    pub GetCharArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jcharArray,
                                  isCopy: *mut jboolean)
                                  -> *mut jchar,
    >,
    pub GetShortArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jshortArray,
                                  isCopy: *mut jboolean)
                                  -> *mut jshort,
    >,
    pub GetIntArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jintArray,
                                  isCopy: *mut jboolean)
                                  -> *mut jint,
    >,
    pub GetLongArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jlongArray,
                                  isCopy: *mut jboolean)
                                  -> *mut jlong,
    >,
    pub GetFloatArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jfloatArray,
                                  isCopy: *mut jboolean)
                                  -> *mut jfloat,
    >,
    pub GetDoubleArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jdoubleArray,
                                  isCopy: *mut jboolean)
                                  -> *mut jdouble,
    >,
    pub ReleaseBooleanArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jbooleanArray,
                                  elems: *mut jboolean,
                                  mode: jint),
    >,
    pub ReleaseByteArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jbyteArray,
                                  elems: *mut jbyte,
                                  mode: jint),
    >,
    pub ReleaseCharArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jcharArray,
                                  elems: *mut jchar,
                                  mode: jint),
    >,
    pub ReleaseShortArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jshortArray,
                                  elems: *mut jshort,
                                  mode: jint),
    >,
    pub ReleaseIntArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jintArray,
                                  elems: *mut jint,
                                  mode: jint),
    >,
    pub ReleaseLongArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jlongArray,
                                  elems: *mut jlong,
                                  mode: jint),
    >,
    pub ReleaseFloatArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jfloatArray,
                                  elems: *mut jfloat,
                                  mode: jint),
    >,
    pub ReleaseDoubleArrayElements: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jdoubleArray,
                                  elems: *mut jdouble,
                                  mode: jint),
    >,
    pub GetBooleanArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jbooleanArray,
                                  start: jsize,
                                  l: jsize,
                                  buf: *mut jboolean),
    >,
    pub GetByteArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jbyteArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *mut jbyte),
    >,
    pub GetCharArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jcharArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *mut jchar),
    >,
    pub GetShortArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jshortArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *mut jshort),
    >,
    pub GetIntArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jintArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *mut jint),
    >,
    pub GetLongArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jlongArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *mut jlong),
    >,
    pub GetFloatArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jfloatArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *mut jfloat),
    >,
    pub GetDoubleArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jdoubleArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *mut jdouble),
    >,
    pub SetBooleanArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jbooleanArray,
                                  start: jsize,
                                  l: jsize,
                                  buf: *const jboolean),
    >,
    pub SetByteArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jbyteArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *const jbyte),
    >,
    pub SetCharArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jcharArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *const jchar),
    >,
    pub SetShortArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jshortArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *const jshort),
    >,
    pub SetIntArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jintArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *const jint),
    >,
    pub SetLongArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jlongArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *const jlong),
    >,
    pub SetFloatArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jfloatArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *const jfloat),
    >,
    pub SetDoubleArrayRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jdoubleArray,
                                  start: jsize,
                                  len: jsize,
                                  buf: *const jdouble),
    >,
    pub RegisterNatives: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  clazz: jclass,
                                  methods: *const JNINativeMethod,
                                  nMethods: jint)
                                  -> jint,
    >,
    pub UnregisterNatives:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, clazz: jclass) -> jint>,
    pub MonitorEnter: Option<unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> jint>,
    pub MonitorExit: Option<unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> jint>,
    pub GetJavaVM:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, vm: *mut *mut JavaVM) -> jint>,
    pub GetStringRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  str: jstring,
                                  start: jsize,
                                  len: jsize,
                                  buf: *mut jchar),
    >,
    pub GetStringUTFRegion: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  str: jstring,
                                  start: jsize,
                                  len: jsize,
                                  buf: *mut c_char),
    >,
    pub GetPrimitiveArrayCritical: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jarray,
                                  isCopy: *mut jboolean)
                                  -> *mut c_void,
    >,
    pub ReleasePrimitiveArrayCritical: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  array: jarray,
                                  carray: *mut c_void,
                                  mode: jint),
    >,
    pub GetStringCritical: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  string: jstring,
                                  isCopy: *mut jboolean)
                                  -> *const jchar,
    >,
    pub ReleaseStringCritical:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, string: jstring, cstring: *const jchar)>,
    pub NewWeakGlobalRef:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> jweak>,
    pub DeleteWeakGlobalRef: Option<unsafe extern "system" fn(env: *mut JNIEnv, ref_: jweak)>,
    pub ExceptionCheck: Option<unsafe extern "system" fn(env: *mut JNIEnv) -> jboolean>,
    pub NewDirectByteBuffer: Option<
        unsafe extern "system" fn(env: *mut JNIEnv,
                                  address: *mut c_void,
                                  capacity: jlong)
                                  -> jobject,
    >,
    pub GetDirectBufferAddress:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, buf: jobject) -> *mut c_void>,
    pub GetDirectBufferCapacity:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, buf: jobject) -> jlong>,
    pub GetObjectRefType:
        Option<unsafe extern "system" fn(env: *mut JNIEnv, obj: jobject) -> jobjectRefType>,
}

impl Clone for JNINativeInterface_ {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct JNIEnv_ {
    pub functions: *const JNINativeInterface_,
}

impl Clone for JNIEnv_ {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct JavaVMOption {
    pub optionString: *mut c_char,
    pub extraInfo: *mut c_void,
}

impl Clone for JavaVMOption {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct JavaVMInitArgs {
    pub version: jint,
    pub nOptions: jint,
    pub options: *mut JavaVMOption,
    pub ignoreUnrecognized: jboolean,
}

impl Clone for JavaVMInitArgs {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct JavaVMAttachArgs {
    pub version: jint,
    pub name: *mut c_char,
    pub group: jobject,
}

impl Clone for JavaVMAttachArgs {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct JNIInvokeInterface_ {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub DestroyJavaVM: Option<unsafe extern "system" fn(vm: *mut JavaVM) -> jint>,
    pub AttachCurrentThread: Option<
        unsafe extern "system" fn(vm: *mut JavaVM,
                                  penv: *mut *mut c_void,
                                  args: *mut c_void)
                                  -> jint,
    >,
    pub DetachCurrentThread: Option<unsafe extern "system" fn(vm: *mut JavaVM) -> jint>,
    pub GetEnv: Option<
        unsafe extern "system" fn(vm: *mut JavaVM,
                                  penv: *mut *mut c_void,
                                  version: jint)
                                  -> jint,
    >,
    pub AttachCurrentThreadAsDaemon: Option<
        unsafe extern "system" fn(vm: *mut JavaVM,
                                  penv: *mut *mut c_void,
                                  args: *mut c_void)
                                  -> jint,
    >,
}

impl Clone for JNIInvokeInterface_ {
    fn clone(&self) -> Self {
        *self
    }
}

extern "system" {
    pub fn JNI_GetDefaultJavaVMInitArgs(args: *mut c_void) -> jint;
    pub fn JNI_CreateJavaVM(
        pvm: *mut *mut JavaVM,
        penv: *mut *mut c_void,
        args: *mut c_void,
    ) -> jint;
    pub fn JNI_GetCreatedJavaVMs(vmBuf: *mut *mut JavaVM, bufLen: jsize, nVMs: *mut jsize) -> jint;
}
