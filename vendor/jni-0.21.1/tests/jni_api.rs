#![cfg(feature = "invocation")]
use std::{convert::TryFrom, str::FromStr};

use jni::{
    descriptors::Desc,
    errors::Error,
    objects::{
        AutoElements, AutoLocal, JByteBuffer, JList, JObject, JString, JThrowable, JValue,
        ReleaseMode,
    },
    signature::{JavaType, Primitive, ReturnType},
    strings::JNIString,
    sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jobject, jshort, jsize},
    JNIEnv,
};

mod util;
use util::{attach_current_thread, unwrap};

static ARRAYLIST_CLASS: &str = "java/util/ArrayList";
static EXCEPTION_CLASS: &str = "java/lang/Exception";
static ARITHMETIC_EXCEPTION_CLASS: &str = "java/lang/ArithmeticException";
static RUNTIME_EXCEPTION_CLASS: &str = "java/lang/RuntimeException";
static INTEGER_CLASS: &str = "java/lang/Integer";
static MATH_CLASS: &str = "java/lang/Math";
static STRING_CLASS: &str = "java/lang/String";
static MATH_ABS_METHOD_NAME: &str = "abs";
static MATH_TO_INT_METHOD_NAME: &str = "toIntExact";
static MATH_ABS_SIGNATURE: &str = "(I)I";
static MATH_TO_INT_SIGNATURE: &str = "(J)I";
static TEST_EXCEPTION_MESSAGE: &str = "Default exception thrown";
static TESTING_OBJECT_STR: &str = "TESTING OBJECT";

#[test]
pub fn call_method_returning_null() {
    let mut env = attach_current_thread();
    // Create an Exception with no message
    let obj = AutoLocal::new(
        unwrap(env.new_object(EXCEPTION_CLASS, "()V", &[]), &env),
        &env,
    );
    // Call Throwable#getMessage must return null
    let message = unwrap(
        env.call_method(&obj, "getMessage", "()Ljava/lang/String;", &[]),
        &env,
    );
    let message_ref = env.auto_local(unwrap(message.l(), &env));

    assert!(message_ref.is_null());
}

#[test]
pub fn is_instance_of_same_class() {
    let mut env = attach_current_thread();
    let obj = AutoLocal::new(
        unwrap(env.new_object(EXCEPTION_CLASS, "()V", &[]), &env),
        &env,
    );
    assert!(unwrap(env.is_instance_of(&obj, EXCEPTION_CLASS), &env));
}

#[test]
pub fn is_instance_of_superclass() {
    let mut env = attach_current_thread();
    let obj = AutoLocal::new(
        unwrap(env.new_object(ARITHMETIC_EXCEPTION_CLASS, "()V", &[]), &env),
        &env,
    );
    assert!(unwrap(env.is_instance_of(&obj, EXCEPTION_CLASS), &env));
}

#[test]
pub fn is_instance_of_subclass() {
    let mut env = attach_current_thread();
    let obj = AutoLocal::new(
        unwrap(env.new_object(EXCEPTION_CLASS, "()V", &[]), &env),
        &env,
    );
    assert!(!unwrap(
        env.is_instance_of(&obj, ARITHMETIC_EXCEPTION_CLASS),
        &env,
    ));
}

#[test]
pub fn is_instance_of_not_superclass() {
    let mut env = attach_current_thread();
    let obj = AutoLocal::new(
        unwrap(env.new_object(ARITHMETIC_EXCEPTION_CLASS, "()V", &[]), &env),
        &env,
    );
    assert!(!unwrap(env.is_instance_of(&obj, ARRAYLIST_CLASS), &env));
}

#[test]
pub fn is_instance_of_null() {
    let mut env = attach_current_thread();
    let obj = JObject::null();
    assert!(unwrap(env.is_instance_of(&obj, ARRAYLIST_CLASS), &env));
    assert!(unwrap(env.is_instance_of(&obj, EXCEPTION_CLASS), &env));
    assert!(unwrap(
        env.is_instance_of(&obj, ARITHMETIC_EXCEPTION_CLASS),
        &env,
    ));
}

#[test]
pub fn is_same_object_diff_references() {
    let env = attach_current_thread();
    let string = env.new_string(TESTING_OBJECT_STR).unwrap();
    let ref_from_string = unwrap(env.new_local_ref(&string), &env);
    assert!(unwrap(env.is_same_object(&string, &ref_from_string), &env));
    unwrap(env.delete_local_ref(ref_from_string), &env);
}

#[test]
pub fn is_same_object_same_reference() {
    let env = attach_current_thread();
    let string = env.new_string(TESTING_OBJECT_STR).unwrap();
    assert!(unwrap(env.is_same_object(&string, &string), &env));
}

#[test]
pub fn is_not_same_object() {
    let env = attach_current_thread();
    let string = env.new_string(TESTING_OBJECT_STR).unwrap();
    let same_src_str = env.new_string(TESTING_OBJECT_STR).unwrap();
    assert!(!unwrap(env.is_same_object(string, same_src_str), &env));
}

#[test]
pub fn is_not_same_object_null() {
    let env = attach_current_thread();
    assert!(unwrap(
        env.is_same_object(JObject::null(), JObject::null()),
        &env,
    ));
}

#[test]
pub fn get_static_public_field() {
    let mut env = attach_current_thread();

    let min_int_value = env
        .get_static_field(INTEGER_CLASS, "MIN_VALUE", "I")
        .unwrap()
        .i()
        .unwrap();

    assert_eq!(min_int_value, i32::min_value());
}

#[test]
pub fn get_static_public_field_by_id() {
    let mut env = attach_current_thread();

    // One can't pass a JavaType::Primitive(Primitive::Int) to
    //   `get_static_field_id` unfortunately: #137
    let field_type = "I";
    let field_id = env
        .get_static_field_id(INTEGER_CLASS, "MIN_VALUE", field_type)
        .unwrap();

    let field_type = JavaType::from_str(field_type).unwrap();
    let min_int_value = env
        .get_static_field_unchecked(INTEGER_CLASS, field_id, field_type)
        .unwrap()
        .i()
        .unwrap();

    assert_eq!(min_int_value, i32::min_value());
}

#[test]
pub fn pop_local_frame_pending_exception() {
    let mut env = attach_current_thread();

    env.push_local_frame(16).unwrap();

    env.throw_new(RUNTIME_EXCEPTION_CLASS, "Test Exception")
        .unwrap();

    // Pop the local frame with a pending exception
    unsafe { env.pop_local_frame(&JObject::null()) }
        .expect("JNIEnv#pop_local_frame must work in case of pending exception");

    env.exception_clear().unwrap();
}

#[test]
pub fn push_local_frame_pending_exception() {
    let mut env = attach_current_thread();

    env.throw_new(RUNTIME_EXCEPTION_CLASS, "Test Exception")
        .unwrap();

    // Push a new local frame with a pending exception
    env.push_local_frame(16)
        .expect("JNIEnv#push_local_frame must work in case of pending exception");

    env.exception_clear().unwrap();

    unsafe { env.pop_local_frame(&JObject::null()) }.unwrap();
}

#[test]
pub fn push_local_frame_too_many_refs() {
    let env = attach_current_thread();

    // Try to push a new local frame with a ridiculous size
    let frame_size = i32::max_value();
    env.push_local_frame(frame_size)
        .expect_err("push_local_frame(2B) must Err");

    unsafe { env.pop_local_frame(&JObject::null()) }.unwrap();
}

#[test]
pub fn with_local_frame() {
    let mut env = attach_current_thread();

    let s = env
        .with_local_frame_returning_local::<_, jni::errors::Error>(16, |env| {
            let res = env.new_string("Test")?;
            Ok(res.into())
        })
        .unwrap()
        .into();

    let s = env
        .get_string(&s)
        .expect("The object returned from the local frame must remain valid");
    assert_eq!(s.to_str().unwrap(), "Test");
}

#[test]
pub fn with_local_frame_pending_exception() {
    let mut env = attach_current_thread();

    env.throw_new(RUNTIME_EXCEPTION_CLASS, "Test Exception")
        .unwrap();

    // Try to allocate a frame of locals
    env.with_local_frame(16, |_| -> Result<_, Error> { Ok(()) })
        .expect("JNIEnv#with_local_frame must work in case of pending exception");

    env.exception_clear().unwrap();
}

#[test]
pub fn call_method_ok() {
    let mut env = attach_current_thread();

    let s = env.new_string(TESTING_OBJECT_STR).unwrap();

    let v: jint = env
        .call_method(s, "indexOf", "(I)I", &[JValue::Int('S' as i32)])
        .expect("JNIEnv#call_method should return JValue")
        .i()
        .unwrap();

    assert_eq!(v, 2);
}

#[test]
pub fn call_method_with_bad_args_errs() {
    let mut env = attach_current_thread();

    let s = env.new_string(TESTING_OBJECT_STR).unwrap();

    let is_bad_typ = env
        .call_method(
            &s,
            "indexOf",
            "(I)I",
            &[JValue::Float(std::f32::consts::PI)],
        )
        .map_err(|error| matches!(error, Error::InvalidArgList(_)))
        .expect_err("JNIEnv#callmethod with bad arg type should err");

    assert!(
        is_bad_typ,
        "ErrorKind::InvalidArgList expected when passing bad value type"
    );

    let is_bad_len = env
        .call_method(
            &s,
            "indexOf",
            "(I)I",
            &[JValue::Int('S' as i32), JValue::Long(3)],
        )
        .map_err(|error| matches!(error, Error::InvalidArgList(_)))
        .expect_err("JNIEnv#call_method with bad arg lengths should err");

    assert!(
        is_bad_len,
        "ErrorKind::InvalidArgList expected when passing bad argument lengths"
    );
}

#[test]
pub fn call_static_method_ok() {
    let mut env = attach_current_thread();

    let x = JValue::from(-10);
    let val: jint = env
        .call_static_method(MATH_CLASS, MATH_ABS_METHOD_NAME, MATH_ABS_SIGNATURE, &[x])
        .expect("JNIEnv#call_static_method should return JValue")
        .i()
        .unwrap();

    assert_eq!(val, 10);
}

#[test]
pub fn call_static_method_unchecked_ok() {
    let mut env = attach_current_thread();

    let x = JValue::from(-10);
    let math_class = env.find_class(MATH_CLASS).unwrap();
    let abs_method_id = env
        .get_static_method_id(&math_class, MATH_ABS_METHOD_NAME, MATH_ABS_SIGNATURE)
        .unwrap();
    let val: jint = unsafe {
        env.call_static_method_unchecked(
            &math_class,
            abs_method_id,
            ReturnType::Primitive(Primitive::Int),
            &[x.as_jni()],
        )
    }
    .expect("JNIEnv#call_static_method_unchecked should return JValue")
    .i()
    .unwrap();

    assert_eq!(val, 10);
}

#[test]
pub fn call_new_object_unchecked_ok() {
    let mut env = attach_current_thread();

    let test_str = env.new_string(TESTING_OBJECT_STR).unwrap();
    let string_class = env.find_class(STRING_CLASS).unwrap();

    let ctor_method_id = env
        .get_method_id(&string_class, "<init>", "(Ljava/lang/String;)V")
        .unwrap();
    let val: JObject = unsafe {
        env.new_object_unchecked(
            &string_class,
            ctor_method_id,
            &[JValue::from(&test_str).as_jni()],
        )
    }
    .expect("JNIEnv#new_object_unchecked should return JValue");

    let jstr = JString::try_from(val).expect("asd");
    let javastr = env.get_string(&jstr).unwrap();
    let rstr = javastr.to_str().unwrap();
    assert_eq!(rstr, TESTING_OBJECT_STR);
}

#[test]
pub fn call_new_object_with_bad_args_errs() {
    let mut env = attach_current_thread();

    let string_class = env.find_class(STRING_CLASS).unwrap();

    let is_bad_typ = env
        .new_object(&string_class, "(Ljava/lang/String;)V", &[JValue::Int(2)])
        .map_err(|error| matches!(error, Error::InvalidArgList(_)))
        .expect_err("JNIEnv#new_object with bad arg type should err");

    assert!(
        is_bad_typ,
        "ErrorKind::InvalidArgList expected when passing bad value type"
    );

    let s = env.new_string(TESTING_OBJECT_STR).unwrap();

    let is_bad_len = env
        .new_object(
            &string_class,
            "(Ljava/lang/String;)V",
            &[JValue::from(&s), JValue::Int(2)],
        )
        .map_err(|error| matches!(error, Error::InvalidArgList(_)))
        .expect_err("JNIEnv#new_object with bad arg type should err");

    assert!(
        is_bad_len,
        "ErrorKind::InvalidArgList expected when passing bad argument lengths"
    );
}

/// Check that we get a runtime error if trying to instantiate with an array class.
///
/// Although the JNI spec for `NewObjectA` states that the class "must not refer to an array class"
/// (and could therefor potentially trigger undefined behaviour if that rule is violated) we
/// expect that `JNIEnv::new_object()` shouldn't ever get as far as calling `NewObjectA` since
/// it will first fail (with a safe, runtime error) to lookup a method ID for any constructor.
/// (consistent with how [getConstructors()](https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/Class.html#getConstructors())
/// doesn't expose constructors for array classes)
#[test]
pub fn call_new_object_with_array_class() {
    let mut env = attach_current_thread();

    let byte_array = env.new_byte_array(16).unwrap();
    let array_class = env.get_object_class(byte_array).unwrap();
    // We just make up a plausible constructor signature
    let result = env.new_object(&array_class, "(I)[B", &[JValue::Int(16)]);

    assert!(result.is_err())
}

#[test]
pub fn call_static_method_throws() {
    let mut env = attach_current_thread();

    let x = JValue::Long(4_000_000_000);
    let is_java_exception = env
        .call_static_method(
            MATH_CLASS,
            MATH_TO_INT_METHOD_NAME,
            MATH_TO_INT_SIGNATURE,
            &[x],
        )
        .map_err(|error| matches!(error, Error::JavaException))
        .expect_err("JNIEnv#call_static_method_unsafe should return error");

    // Throws a java.lang.ArithmeticException: integer overflow
    assert!(
        is_java_exception,
        "ErrorKind::JavaException expected as error"
    );
    assert_pending_java_exception(&mut env);
}

#[test]
pub fn call_static_method_with_bad_args_errs() {
    let mut env = attach_current_thread();

    let x = JValue::Double(4.567_891_23);
    let is_bad_typ = env
        .call_static_method(
            MATH_CLASS,
            MATH_TO_INT_METHOD_NAME,
            MATH_TO_INT_SIGNATURE,
            &[x],
        )
        .map_err(|error| matches!(error, Error::InvalidArgList(_)))
        .expect_err("JNIEnv#call_static_method with bad arg type should err");

    assert!(
        is_bad_typ,
        "ErrorKind::InvalidArgList expected when passing bad value type"
    );

    let is_bad_len = env
        .call_static_method(
            MATH_CLASS,
            MATH_TO_INT_METHOD_NAME,
            MATH_TO_INT_SIGNATURE,
            &[JValue::Int(2), JValue::Int(3)],
        )
        .map_err(|error| matches!(error, Error::InvalidArgList(_)))
        .expect_err("JNIEnv#call_static_method with bad arg lengths should err");

    assert!(
        is_bad_len,
        "ErrorKind::InvalidArgList expected when passing bad argument lengths"
    );
}

#[test]
pub fn java_byte_array_from_slice() {
    let env = attach_current_thread();
    let buf: &[u8] = &[1, 2, 3];
    let java_array = AutoLocal::new(
        env.byte_array_from_slice(buf)
            .expect("JNIEnv#byte_array_from_slice must create a java array from slice"),
        &env,
    );

    assert!(!java_array.is_null());
    let mut res: [i8; 3] = [0; 3];
    env.get_byte_array_region(&java_array, 0, &mut res).unwrap();
    assert_eq!(res[0], 1);
    assert_eq!(res[1], 2);
    assert_eq!(res[2], 3);
}

macro_rules! test_auto_array_read_write {
    ( $test_name:tt, $jni_type:ty, $new_array:tt, $get_array:tt, $set_array:tt ) => {
        #[test]
        pub fn $test_name() {
            let env = attach_current_thread();

            // Create original Java array
            let buf: &[$jni_type] = &[0 as $jni_type, 1 as $jni_type];
            let java_array = env
                .$new_array(2)
                .expect(stringify!(JNIEnv#$new_array must create a Java $jni_type array with given size));

            // Insert array elements
            let _ = env.$set_array(&java_array, 0, buf);

            // Use a scope to test Drop
            {
                // Get byte array elements auto wrapper
                let mut auto_ptr: AutoElements<$jni_type> = unsafe {
                    // Make sure the lifetime is tied to the environment,
                    // not the particular JNIEnv reference
                    let mut temporary_env: JNIEnv = env.unsafe_clone();
                    temporary_env.get_array_elements(&java_array, ReleaseMode::CopyBack).unwrap()
                };

                // Check array size
                assert_eq!(auto_ptr.len(), 2);

                // Check pointer access
                let ptr = auto_ptr.as_ptr();
                assert_eq!(unsafe { *ptr.offset(0) } as i32, 0);
                assert_eq!(unsafe { *ptr.offset(1) } as i32, 1);

                // Check pointer From access
                let ptr: *mut $jni_type = std::convert::From::from(&auto_ptr);
                assert_eq!(unsafe { *ptr.offset(0) } as i32, 0);
                assert_eq!(unsafe { *ptr.offset(1) } as i32, 1);

                // Check pointer into() access
                let ptr: *mut $jni_type = (&auto_ptr).into();
                assert_eq!(unsafe { *ptr.offset(0) } as i32, 0);
                assert_eq!(unsafe { *ptr.offset(1) } as i32, 1);

                // Check slice access
                //
                // # Safety
                //
                // We make sure that the slice is dropped before also testing access via `Deref`
                // (to ensure we don't have aliased references)
                unsafe {
                    let slice = std::slice::from_raw_parts(auto_ptr.as_ptr(), auto_ptr.len());
                    assert_eq!(slice[0] as i32, 0);
                    assert_eq!(slice[1] as i32, 1);
                }

                // Check access via Deref
                assert_eq!(auto_ptr[0] as i32, 0);
                assert_eq!(auto_ptr[1] as i32, 1);

                // Modify via DerefMut
                let tmp = auto_ptr[1];
                auto_ptr[1] = auto_ptr[0];
                auto_ptr[0] = tmp;

                // Commit would be necessary here, if there were no closure
                //auto_ptr.commit().unwrap();
            }

            // Confirm modification of original Java array
            let mut res: [$jni_type; 2] = [0 as $jni_type; 2];
            env.$get_array(&java_array, 0, &mut res).unwrap();
            assert_eq!(res[0] as i32, 1);
            assert_eq!(res[1] as i32, 0);
        }
    };
}

// Test generic get_array_elements
test_auto_array_read_write!(
    get_array_elements,
    jint,
    new_int_array,
    get_int_array_region,
    set_int_array_region
);

// Test type-specific array accessors
test_auto_array_read_write!(
    get_int_array_elements,
    jint,
    new_int_array,
    get_int_array_region,
    set_int_array_region
);

test_auto_array_read_write!(
    get_long_array_elements,
    jlong,
    new_long_array,
    get_long_array_region,
    set_long_array_region
);

test_auto_array_read_write!(
    get_byte_array_elements,
    jbyte,
    new_byte_array,
    get_byte_array_region,
    set_byte_array_region
);

test_auto_array_read_write!(
    get_boolean_array_elements,
    jboolean,
    new_boolean_array,
    get_boolean_array_region,
    set_boolean_array_region
);

test_auto_array_read_write!(
    get_char_array_elements,
    jchar,
    new_char_array,
    get_char_array_region,
    set_char_array_region
);

test_auto_array_read_write!(
    get_short_array_elements,
    jshort,
    new_short_array,
    get_short_array_region,
    set_short_array_region
);

test_auto_array_read_write!(
    get_float_array_elements,
    jfloat,
    new_float_array,
    get_float_array_region,
    set_float_array_region
);

test_auto_array_read_write!(
    get_double_array_elements,
    jdouble,
    new_double_array,
    get_double_array_region,
    set_double_array_region
);

#[test]
#[ignore] // Disabled until issue #283 is resolved
pub fn get_long_array_elements_commit() {
    let mut env = attach_current_thread();

    // Create original Java array
    let buf: &[i64] = &[1, 2, 3];
    let java_array = env
        .new_long_array(3)
        .expect("JNIEnv#new_long_array must create a java array with given size");

    // Insert array elements
    let _ = env.set_long_array_region(&java_array, 0, buf);

    // Get long array elements auto wrapper
    let mut auto_ptr = unsafe {
        env.get_array_elements(&java_array, ReleaseMode::CopyBack)
            .unwrap()
    };

    // Copying the array depends on the VM vendor/version/GC combinations.
    // If the wrapped array is not being copied, we can skip the test.
    if !auto_ptr.is_copy() {
        return;
    }

    // Check pointer access
    let ptr = auto_ptr.as_ptr();

    // Modify
    unsafe {
        *ptr.offset(0) += 1;
        *ptr.offset(1) += 1;
        *ptr.offset(2) += 1;
    }

    // Check that original Java array is unmodified
    let mut res: [i64; 3] = [0; 3];
    env.get_long_array_region(&java_array, 0, &mut res).unwrap();
    assert_eq!(res[0], 1);
    assert_eq!(res[1], 2);
    assert_eq!(res[2], 3);

    auto_ptr.commit().unwrap();

    // Confirm modification of original Java array
    env.get_long_array_region(&java_array, 0, &mut res).unwrap();
    assert_eq!(res[0], 2);
    assert_eq!(res[1], 3);
    assert_eq!(res[2], 4);
}

#[test]
pub fn get_array_elements_critical() {
    let mut env = attach_current_thread();

    // Create original Java array
    let buf: &[u8] = &[1, 2, 3];
    let java_array = env
        .byte_array_from_slice(buf)
        .expect("JNIEnv#byte_array_from_slice must create a java array from slice");

    // Use a scope to test Drop
    {
        // Get primitive array elements auto wrapper
        let mut auto_ptr = unsafe {
            env.get_array_elements_critical(&java_array, ReleaseMode::CopyBack)
                .unwrap()
        };

        // Check array size
        assert_eq!(auto_ptr.len(), 3);

        // Convert void pointer to a &[i8] slice, without copy
        //
        // # Safety
        //
        // We make sure that the slice is dropped before also testing access via `Deref`
        // (to ensure we don't have aliased references)
        unsafe {
            let slice = std::slice::from_raw_parts(auto_ptr.as_ptr(), auto_ptr.len());
            assert_eq!(slice[0], 1);
            assert_eq!(slice[1], 2);
            assert_eq!(slice[2], 3);
        }

        // Also check access via `Deref`
        assert_eq!(auto_ptr[0], 1);
        assert_eq!(auto_ptr[1], 2);
        assert_eq!(auto_ptr[2], 3);

        // Modify via `DerefMut`
        auto_ptr[0] += 1;
        auto_ptr[1] += 1;
        auto_ptr[2] += 1;
    }

    // Confirm modification of original Java array
    let mut res: [i8; 3] = [0; 3];
    env.get_byte_array_region(&java_array, 0, &mut res).unwrap();
    assert_eq!(res[0], 2);
    assert_eq!(res[1], 3);
    assert_eq!(res[2], 4);
}

#[test]
pub fn get_object_class() {
    let env = attach_current_thread();
    let string = env.new_string("test").unwrap();
    let result = env.get_object_class(string);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_null());
}

#[test]
pub fn get_object_class_null_arg() {
    let env = attach_current_thread();
    let null_obj = JObject::null();
    let result = env
        .get_object_class(null_obj)
        .map_err(|error| matches!(error, Error::NullPtr(_)))
        .expect_err("JNIEnv#get_object_class should return error for null argument");
    assert!(result, "ErrorKind::NullPtr expected as error");
}

#[test]
pub fn new_direct_byte_buffer() {
    let mut env = attach_current_thread();
    let vec: Vec<u8> = vec![0, 1, 2, 3];
    let (addr, len) = {
        // (would use buf.into_raw_parts() on nightly)
        let buf = vec.leak();
        (buf.as_mut_ptr(), buf.len())
    };
    let result = unsafe { env.new_direct_byte_buffer(addr, len) };
    assert!(result.is_ok());
    assert!(!result.unwrap().is_null());
}

#[test]
pub fn new_direct_byte_buffer_invalid_addr() {
    let mut env = attach_current_thread();
    let result = unsafe { env.new_direct_byte_buffer(std::ptr::null_mut(), 5) };
    assert!(result.is_err());
}

#[test]
pub fn get_direct_buffer_capacity_ok() {
    let mut env = attach_current_thread();
    let vec: Vec<u8> = vec![0, 1, 2, 3];
    let (addr, len) = {
        // (would use buf.into_raw_parts() on nightly)
        let buf = vec.leak();
        (buf.as_mut_ptr(), buf.len())
    };
    let result = unsafe { env.new_direct_byte_buffer(addr, len) }.unwrap();
    assert!(!result.is_null());

    let capacity = env.get_direct_buffer_capacity(&result).unwrap();
    assert_eq!(capacity, 4);
}

#[test]
pub fn get_direct_buffer_capacity_wrong_arg() {
    let env = attach_current_thread();
    let wrong_obj = unsafe { JByteBuffer::from_raw(env.new_string("wrong").unwrap().into_raw()) };
    let capacity = env.get_direct_buffer_capacity(&wrong_obj);
    assert!(capacity.is_err());
}

#[test]
pub fn get_direct_buffer_capacity_null_arg() {
    let env = attach_current_thread();
    let result = env.get_direct_buffer_capacity(&JObject::null().into());
    assert!(result.is_err());
}

#[test]
pub fn get_direct_buffer_address_ok() {
    let mut env = attach_current_thread();
    let vec: Vec<u8> = vec![0, 1, 2, 3];
    let (addr, len) = {
        // (would use buf.into_raw_parts() on nightly)
        let buf = vec.leak();
        (buf.as_mut_ptr(), buf.len())
    };
    let result = unsafe { env.new_direct_byte_buffer(addr, len) }.unwrap();
    assert!(!result.is_null());

    let dest_buffer = env.get_direct_buffer_address(&result).unwrap();
    assert_eq!(addr, dest_buffer);
}

#[test]
pub fn get_direct_buffer_address_wrong_arg() {
    let env = attach_current_thread();
    let wrong_obj: JObject = env.new_string("wrong").unwrap().into();
    let result = env.get_direct_buffer_address(&wrong_obj.into());
    assert!(result.is_err());
}

#[test]
pub fn get_direct_buffer_address_null_arg() {
    let env = attach_current_thread();
    let result = env.get_direct_buffer_address(&JObject::null().into());
    assert!(result.is_err());
}

// Group test for testing the family of new_PRIMITIVE_array functions with correct arguments
#[test]
pub fn new_primitive_array_ok() {
    let env = attach_current_thread();
    const SIZE: jsize = 16;

    let result = env.new_boolean_array(SIZE);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_null());

    let result = env.new_byte_array(SIZE);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_null());

    let result = env.new_char_array(SIZE);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_null());

    let result = env.new_short_array(SIZE);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_null());

    let result = env.new_int_array(SIZE);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_null());

    let result = env.new_long_array(SIZE);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_null());

    let result = env.new_float_array(SIZE);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_null());

    let result = env.new_double_array(SIZE);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_null());
}

// Group test for testing the family of new_PRIMITIVE_array functions with wrong arguments
#[test]
pub fn new_primitive_array_wrong() {
    let mut env = attach_current_thread();
    const WRONG_SIZE: jsize = -1;

    let result = env.new_boolean_array(WRONG_SIZE).map(|arr| arr.as_raw());
    assert_exception(&result, "JNIEnv#new_boolean_array should throw exception");
    assert_pending_java_exception(&mut env);

    let result = env.new_byte_array(WRONG_SIZE).map(|arr| arr.as_raw());
    assert_exception(&result, "JNIEnv#new_byte_array should throw exception");
    assert_pending_java_exception(&mut env);

    let result = env.new_char_array(WRONG_SIZE).map(|arr| arr.as_raw());
    assert_exception(&result, "JNIEnv#new_char_array should throw exception");
    assert_pending_java_exception(&mut env);

    let result = env.new_short_array(WRONG_SIZE).map(|arr| arr.as_raw());
    assert_exception(&result, "JNIEnv#new_short_array should throw exception");
    assert_pending_java_exception(&mut env);

    let result = env.new_int_array(WRONG_SIZE).map(|arr| arr.as_raw());
    assert_exception(&result, "JNIEnv#new_int_array should throw exception");
    assert_pending_java_exception(&mut env);

    let result = env.new_long_array(WRONG_SIZE).map(|arr| arr.as_raw());
    assert_exception(&result, "JNIEnv#new_long_array should throw exception");
    assert_pending_java_exception(&mut env);

    let result = env.new_float_array(WRONG_SIZE).map(|arr| arr.as_raw());
    assert_exception(&result, "JNIEnv#new_float_array should throw exception");
    assert_pending_java_exception(&mut env);

    let result = env.new_double_array(WRONG_SIZE).map(|arr| arr.as_raw());
    assert_exception(&result, "JNIEnv#new_double_array should throw exception");
    assert_pending_java_exception(&mut env);
}

#[test]
fn get_super_class_ok() {
    let mut env = attach_current_thread();
    let result = env.get_superclass(ARRAYLIST_CLASS);
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn get_super_class_null() {
    let mut env = attach_current_thread();
    let result = env.get_superclass("java/lang/Object");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn convert_byte_array() {
    let env = attach_current_thread();
    let src: Vec<u8> = vec![1, 2, 3, 4];
    let java_byte_array = env.byte_array_from_slice(&src).unwrap();

    let dest = env.convert_byte_array(java_byte_array);
    assert!(dest.is_ok());
    assert_eq!(dest.unwrap(), src);
}

#[test]
fn local_ref_null() {
    let env = attach_current_thread();
    let null_obj = JObject::null();

    let result = env.new_local_ref::<&JObject>(&null_obj);
    assert!(result.is_ok());
    assert!(result.unwrap().is_null());

    // try to delete null reference
    let result = env.delete_local_ref(null_obj);
    assert!(result.is_ok());
}

#[test]
fn new_global_ref_null() {
    let env = attach_current_thread();
    let null_obj = JObject::null();
    let result = env.new_global_ref(null_obj);
    assert!(result.is_ok());
    assert!(result.unwrap().is_null());
}

#[test]
fn new_weak_ref_null() {
    let env = attach_current_thread();
    let null_obj = JObject::null();
    let result = unwrap(env.new_weak_ref(null_obj), &env);
    assert!(result.is_none());
}

#[test]
fn auto_local_null() {
    let env = attach_current_thread();
    let null_obj = JObject::null();
    {
        let auto_ref = AutoLocal::new(null_obj, &env);
        assert!(auto_ref.is_null());
    }
}

#[test]
fn short_lifetime_with_local_frame() {
    let mut env = attach_current_thread();
    let object = short_lifetime_with_local_frame_sub_fn(&mut env);
    assert!(object.is_ok());
}

fn short_lifetime_with_local_frame_sub_fn<'local>(
    env: &'_ mut JNIEnv<'local>,
) -> Result<JObject<'local>, Error> {
    env.with_local_frame_returning_local(16, |env| {
        env.new_object(INTEGER_CLASS, "(I)V", &[JValue::from(5)])
    })
}

#[test]
fn short_lifetime_list() {
    let mut env = attach_current_thread();
    let first_list_object = short_lifetime_list_sub_fn(&mut env).unwrap();
    let value = env.call_method(first_list_object, "intValue", "()I", &[]);
    assert_eq!(value.unwrap().i().unwrap(), 1);
}

fn short_lifetime_list_sub_fn<'local>(
    env: &'_ mut JNIEnv<'local>,
) -> Result<JObject<'local>, Error> {
    let list_object = env.new_object(ARRAYLIST_CLASS, "()V", &[])?;
    let list = JList::from_env(env, &list_object)?;
    let element = env.new_object(INTEGER_CLASS, "(I)V", &[JValue::from(1)])?;
    list.add(env, &element)?;
    short_lifetime_list_sub_fn_get_first_element(env, &list)
}

fn short_lifetime_list_sub_fn_get_first_element<'local>(
    env: &'_ mut JNIEnv<'local>,
    list: &'_ JList<'local, '_, '_>,
) -> Result<JObject<'local>, Error> {
    let mut iterator = list.iter(env)?;
    Ok(iterator.next(env)?.unwrap())
}

#[test]
fn get_object_array_element() {
    let mut env = attach_current_thread();
    let array = env
        .new_object_array(1, STRING_CLASS, JObject::null())
        .unwrap();
    assert!(!array.is_null());
    assert!(env.get_object_array_element(&array, 0).unwrap().is_null());
    let test_str = env.new_string("test").unwrap();
    env.set_object_array_element(&array, 0, test_str).unwrap();
    assert!(!env.get_object_array_element(&array, 0).unwrap().is_null());
}

#[test]
pub fn throw_new() {
    let mut env = attach_current_thread();

    let result = env.throw_new(RUNTIME_EXCEPTION_CLASS, "Test Exception");
    assert!(result.is_ok());
    assert_pending_java_exception_detailed(
        &mut env,
        Some(RUNTIME_EXCEPTION_CLASS),
        Some("Test Exception"),
    );
}

#[test]
pub fn throw_new_fail() {
    let mut env = attach_current_thread();

    let result = env.throw_new("java/lang/NonexistentException", "Test Exception");
    assert!(result.is_err());
    // Just to clear the java.lang.NoClassDefFoundError
    assert_pending_java_exception(&mut env);
}

#[test]
pub fn throw_defaults() {
    let mut env = attach_current_thread();

    test_throwable_descriptor_with_default_type(&mut env, TEST_EXCEPTION_MESSAGE);
    test_throwable_descriptor_with_default_type(&mut env, TEST_EXCEPTION_MESSAGE.to_owned());
    test_throwable_descriptor_with_default_type(&mut env, JNIString::from(TEST_EXCEPTION_MESSAGE));
}

#[test]
pub fn test_conversion() {
    let env = attach_current_thread();
    let orig_obj: JObject = env.new_string("Hello, world!").unwrap().into();

    let obj: JObject = unwrap(env.new_local_ref(&orig_obj), &env);
    let string = JString::from(obj);
    let actual = JObject::from(string);
    assert!(unwrap(env.is_same_object(&orig_obj, actual), &env));

    let global_ref = env.new_global_ref(&orig_obj).unwrap();
    assert!(unwrap(env.is_same_object(&orig_obj, global_ref), &env));

    let weak_ref = unwrap(env.new_weak_ref(&orig_obj), &env).expect("weak ref should not be null");
    let actual =
        unwrap(weak_ref.upgrade_local(&env), &env).expect("weak ref should not have been GC'd");
    assert!(unwrap(env.is_same_object(&orig_obj, actual), &env));

    let obj: JObject = unwrap(env.new_local_ref(&orig_obj), &env);
    let auto_local = env.auto_local(obj);
    assert!(unwrap(env.is_same_object(&orig_obj, auto_local), &env));
}

#[test]
pub fn test_null_get_string() {
    let mut env = attach_current_thread();
    let s = unsafe { JString::from_raw(std::ptr::null_mut() as _) };
    let ret = env.get_string(&s);
    assert!(ret.is_err());
}

#[test]
pub fn test_invalid_list_get_string() {
    let mut env = attach_current_thread();

    let class = env.find_class("java/util/List").unwrap();
    let class = JString::from(JObject::from(class));
    let class = env.auto_local(class);

    let ret = env.get_string(&class);
    assert!(ret.is_err());
}

fn test_throwable_descriptor_with_default_type<'local, D>(env: &mut JNIEnv<'local>, descriptor: D)
where
    D: Desc<'local, JThrowable<'local>>,
{
    let result = descriptor.lookup(env);
    assert!(result.is_ok());
    let exception = result.unwrap();
    let exception = exception.as_ref();

    assert_exception_type(env, exception, RUNTIME_EXCEPTION_CLASS);
    assert_exception_message(env, exception, TEST_EXCEPTION_MESSAGE);
}

// Helper method that asserts that result is Error and the cause is JavaException.
fn assert_exception(res: &Result<jobject, Error>, expect_message: &str) {
    assert!(res.is_err());
    assert!(res
        .as_ref()
        .map_err(|error| matches!(error, Error::JavaException))
        .expect_err(expect_message));
}

// Shortcut to `assert_pending_java_exception_detailed()` without checking for expected  type and
// message of exception.
fn assert_pending_java_exception(env: &mut JNIEnv) {
    assert_pending_java_exception_detailed(env, None, None)
}

// Helper method that asserts there is a pending Java exception of `expected_type` with
// `expected_message` and clears it if any.
fn assert_pending_java_exception_detailed(
    env: &mut JNIEnv,
    expected_type: Option<&str>,
    expected_message: Option<&str>,
) {
    assert!(env.exception_check().unwrap());
    let exception = env.exception_occurred().expect("Unable to get exception");
    env.exception_clear().unwrap();

    if let Some(expected_type) = expected_type {
        assert_exception_type(env, &exception, expected_type);
    }

    if let Some(expected_message) = expected_message {
        assert_exception_message(env, &exception, expected_message);
    }
}

// Asserts that exception is of `expected_type` type.
fn assert_exception_type(env: &mut JNIEnv, exception: &JThrowable, expected_type: &str) {
    assert!(env.is_instance_of(exception, expected_type).unwrap());
}

// Asserts that exception's message is `expected_message`.
fn assert_exception_message(env: &mut JNIEnv, exception: &JThrowable, expected_message: &str) {
    let message = env
        .call_method(exception, "getMessage", "()Ljava/lang/String;", &[])
        .unwrap()
        .l()
        .unwrap();
    let msg_rust: String = env.get_string(&message.into()).unwrap().into();
    assert_eq!(msg_rust, expected_message);
}
