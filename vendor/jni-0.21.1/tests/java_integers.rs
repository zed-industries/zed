#![cfg(feature = "invocation")]

use jni::{errors::Error, objects::JValue};

mod util;
use util::{attach_current_thread, print_exception};

#[test]
fn test_java_integers() {
    let mut env = attach_current_thread();

    let array_length = 50;

    for value in -10..10 {
        env.with_local_frame(16, |env| -> Result<_, Error> {
            let integer_value =
                env.new_object("java/lang/Integer", "(I)V", &[JValue::Int(value)])?;

            let values_array =
                env.new_object_array(array_length, "java/lang/Integer", &integer_value)?;

            let result = env
                .call_static_method(
                    "java/util/Arrays",
                    "binarySearch",
                    "([Ljava/lang/Object;Ljava/lang/Object;)I",
                    &[
                        JValue::Object(&values_array),
                        JValue::Object(&integer_value),
                    ],
                )?
                .i()?;

            assert!(0 <= result && result < array_length);

            Ok(())
        })
        .unwrap_or_else(|e| {
            print_exception(&env);
            panic!("{:#?}", e);
        })
    }
}
