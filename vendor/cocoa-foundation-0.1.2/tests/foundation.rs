#[macro_use]
extern crate objc;
extern crate block;
extern crate cocoa_foundation;

#[cfg(test)]
mod foundation {
    mod nsstring {
        use cocoa_foundation::base::nil;
        use cocoa_foundation::foundation::NSString;
        use std::slice;
        use std::str;

        #[test]
        fn test_utf8() {
            let expected = "Iñtërnâtiônàlizætiøn";
            unsafe {
                let built = NSString::alloc(nil).init_str(expected);
                let bytes = built.UTF8String() as *const u8;
                let objc_string =
                    str::from_utf8(slice::from_raw_parts(bytes, built.len())).unwrap();
                assert_eq!(objc_string.len(), expected.len());
                assert_eq!(objc_string, expected);
            }
        }

        #[test]
        fn test_string() {
            let expected = "Hello World!";
            unsafe {
                let built = NSString::alloc(nil).init_str(expected);
                let bytes = built.UTF8String() as *const u8;
                let objc_string =
                    str::from_utf8(slice::from_raw_parts(bytes, built.len())).unwrap();
                assert_eq!(objc_string.len(), expected.len());
                assert_eq!(objc_string, expected);
            }
        }

        #[test]
        fn test_length() {
            let expected = "Hello!";
            unsafe {
                let built = NSString::alloc(nil).init_str(expected);
                assert_eq!(built.len(), expected.len());
            }
        }

        #[test]
        fn test_append_by_appending_string() {
            let initial_str = "Iñtërnâtiônàlizætiøn";
            let to_append = "_more_strings";
            let expected = concat!("Iñtërnâtiônàlizætiøn", "_more_strings");
            unsafe {
                let built = NSString::alloc(nil).init_str(initial_str);
                let built_to_append = NSString::alloc(nil).init_str(to_append);
                let append_string = built.stringByAppendingString_(built_to_append);
                let bytes = append_string.UTF8String() as *const u8;
                let objc_string =
                    str::from_utf8(slice::from_raw_parts(bytes, append_string.len())).unwrap();
                assert_eq!(objc_string, expected);
            }
        }
    }

    mod nsfastenumeration {
        use cocoa_foundation::base::{id, nil};
        use cocoa_foundation::foundation::{NSFastEnumeration, NSString};
        use std::slice;
        use std::str;

        #[test]
        fn test_iter() {
            unsafe {
                let string = NSString::alloc(nil).init_str("this is a test string");
                let separator = NSString::alloc(nil).init_str(" ");
                let components: id = msg_send![string, componentsSeparatedByString: separator];

                let combined = components
                    .iter()
                    .map(|s| {
                        let bytes = s.UTF8String() as *const u8;
                        str::from_utf8(slice::from_raw_parts(bytes, s.len())).unwrap()
                    })
                    .fold(String::new(), |mut acc, s| {
                        acc.push_str(s);
                        acc
                    });

                assert_eq!(combined, "thisisateststring");
            }
        }

        #[test]
        #[should_panic]
        fn test_mutation() {
            unsafe {
                let string = NSString::alloc(nil).init_str("this is a test string");
                let separator = NSString::alloc(nil).init_str(" ");
                let components: id = msg_send![string, componentsSeparatedByString: separator];
                let mut_components: id = msg_send![components, mutableCopy];
                let mut iter = mut_components.iter();
                iter.next();
                let () = msg_send![mut_components, removeObjectAtIndex:1];
                iter.next();
            }
        }
    }

    mod nsdictionary {
        use block::ConcreteBlock;
        use cocoa_foundation::base::{id, nil};
        use cocoa_foundation::foundation::{
            NSArray, NSComparisonResult, NSDictionary, NSFastEnumeration, NSString,
        };

        #[test]
        fn test_get() {
            const KEY: &str = "The key";
            const VALUE: &str = "Some value";
            unsafe {
                let key = NSString::alloc(nil).init_str(KEY);
                let value = NSString::alloc(nil).init_str(VALUE);
                let dict = NSDictionary::dictionaryWithObject_forKey_(nil, value, key);

                let retrieved_value = dict.objectForKey_(key);
                assert!(retrieved_value.isEqualToString(VALUE));
            }
        }

        #[test]
        fn test_iter() {
            let mkstr = |s| unsafe { NSString::alloc(nil).init_str(s) };
            let keys = vec!["a", "b", "c", "d", "e", "f"];
            let objects = vec!["1", "2", "3", "4", "5", "6"];
            unsafe {
                use std::cmp::{Ord, Ordering};
                use std::{slice, str};

                let keys_raw_vec = keys.clone().into_iter().map(&mkstr).collect::<Vec<_>>();
                let objs_raw_vec = objects.clone().into_iter().map(&mkstr).collect::<Vec<_>>();

                let keys_array = NSArray::arrayWithObjects(nil, &keys_raw_vec);
                let objs_array = NSArray::arrayWithObjects(nil, &objs_raw_vec);

                let dict =
                    NSDictionary::dictionaryWithObjects_forKeys_(nil, objs_array, keys_array);

                // NSDictionary does not store its contents in order of insertion, so ask for
                // sorted iterators to ensure that each item is the same as its counterpart in
                // the vector.

                fn compare_function(s0: &id, s1: &id) -> Ordering {
                    unsafe {
                        let (bytes0, len0) = (s0.UTF8String() as *const u8, s0.len());
                        let (bytes1, len1) = (s1.UTF8String() as *const u8, s1.len());
                        let (s0, s1) = (
                            str::from_utf8(slice::from_raw_parts(bytes0, len0)).unwrap(),
                            str::from_utf8(slice::from_raw_parts(bytes1, len1)).unwrap(),
                        );
                        let (c0, c1) = (s0.chars().next().unwrap(), s1.chars().next().unwrap());
                        c0.cmp(&c1)
                    }
                }

                // First test cocoa sorting...
                let mut comparator =
                    ConcreteBlock::new(|s0: id, s1: id| match compare_function(&s0, &s1) {
                        Ordering::Less => NSComparisonResult::NSOrderedAscending,
                        Ordering::Equal => NSComparisonResult::NSOrderedSame,
                        Ordering::Greater => NSComparisonResult::NSOrderedDescending,
                    });

                let associated_iter = keys.iter().zip(objects.iter());
                for (k_id, (k, v)) in dict
                    .keysSortedByValueUsingComparator_(&mut *comparator)
                    .iter()
                    .zip(associated_iter)
                {
                    assert!(k_id.isEqualToString(k));
                    let v_id = dict.objectForKey_(k_id);
                    assert!(v_id.isEqualToString(v));
                }

                // Then use rust sorting
                let mut keys_arr = dict.allKeys().iter().collect::<Vec<_>>();
                keys_arr.sort_by(compare_function);
                for (k0, k1) in keys_arr.into_iter().zip(keys.iter()) {
                    assert!(k0.isEqualToString(k1));
                }

                let mut objects_arr = dict.allValues().iter().collect::<Vec<_>>();
                objects_arr.sort_by(compare_function);
                for (v0, v1) in objects_arr.into_iter().zip(objects.iter()) {
                    assert!(v0.isEqualToString(v1));
                }
            }
        }
    }
}
