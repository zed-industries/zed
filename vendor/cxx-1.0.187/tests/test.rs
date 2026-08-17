#![allow(
    clippy::assertions_on_constants,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::float_cmp,
    clippy::needless_pass_by_value,
    clippy::ptr_cast_constness,
    clippy::unit_cmp
)]

use cxx::{CxxVector, SharedPtr, UniquePtr};
use cxx_test_suite::module::ffi2;
use cxx_test_suite::{cast, ffi, R};
use std::cell::Cell;
use std::ffi::CStr;
use std::panic::{self, RefUnwindSafe, UnwindSafe};
use std::ptr;

thread_local! {
    static CORRECT: Cell<bool> = const { Cell::new(false) };
}

#[no_mangle]
extern "C" fn cxx_test_suite_set_correct() {
    CORRECT.with(|correct| correct.set(true));
}

macro_rules! check {
    ($run:expr) => {{
        CORRECT.with(|correct| correct.set(false));
        $run;
        assert!(CORRECT.with(Cell::get), "{}", stringify!($run));
    }};
}

#[test]
fn test_c_return() {
    let shared = ffi::Shared { z: 2020 };
    let ns_shared = ffi::AShared { z: 2020 };
    let nested_ns_shared = ffi::ABShared { z: 2020 };

    assert_eq!(2020, ffi::c_return_primitive());
    assert_eq!(2020, ffi::c_return_shared().z);
    assert_eq!(2020, ffi::c_return_box().0);
    ffi::c_return_unique_ptr();
    ffi2::c_return_ns_unique_ptr();
    assert_eq!(2020, *ffi::c_return_ref(&shared));
    assert_eq!(2020, *ffi::c_return_ns_ref(&ns_shared));
    assert_eq!(2020, *ffi::c_return_nested_ns_ref(&nested_ns_shared));
    assert_eq!("2020", ffi::c_return_str(&shared));
    assert_eq!(
        b"2020\0",
        cast::c_char_to_unsigned(ffi::c_return_slice_char(&shared)),
    );
    assert_eq!("2020", ffi::c_return_rust_string());
    assert_eq!("Hello \u{fffd}World", ffi::c_return_rust_string_lossy());
    assert_eq!("2020", ffi::c_return_unique_ptr_string().to_str().unwrap());
    assert_eq!(c"2020", ffi::c_return_unique_ptr_string().as_c_str());
    assert_eq!(4, ffi::c_return_unique_ptr_vector_u8().len());
    assert!(4 <= ffi::c_return_unique_ptr_vector_u8().capacity());
    assert_eq!(
        200_u8,
        ffi::c_return_unique_ptr_vector_u8().into_iter().sum(),
    );
    assert_eq!(
        200.5_f64,
        ffi::c_return_unique_ptr_vector_f64().into_iter().sum(),
    );
    assert_eq!(2, ffi::c_return_unique_ptr_vector_shared().len());
    assert!(2 <= ffi::c_return_unique_ptr_vector_shared().capacity());
    assert_eq!(
        2021_usize,
        ffi::c_return_unique_ptr_vector_shared()
            .into_iter()
            .map(|o| o.z)
            .sum(),
    );
    assert_eq!(b"\x02\0\x02\0"[..], ffi::c_return_rust_vec_u8());
    assert_eq!([true, true, false][..], ffi::c_return_rust_vec_bool());
    assert_eq!(2020, ffi::c_return_identity(2020));
    assert_eq!(2021, ffi::c_return_sum(2020, 1));
    match ffi::c_return_enum(0) {
        enm @ ffi::Enum::AVal => assert_eq!(0, enm.repr),
        _ => assert!(false),
    }
    match ffi::c_return_enum(1) {
        enm @ ffi::Enum::BVal => assert_eq!(2020, enm.repr),
        _ => assert!(false),
    }
    match ffi::c_return_enum(2021) {
        enm @ ffi::Enum::LastVal => assert_eq!(2021, enm.repr),
        _ => assert!(false),
    }
    match ffi::c_return_ns_enum(0) {
        enm @ ffi::AEnum::AAVal => assert_eq!(0, enm.repr),
        _ => assert!(false),
    }
    match ffi::c_return_nested_ns_enum(2021) {
        enm @ ffi::ABEnum::ABCVal => assert_eq!(i32::MIN, enm.repr),
        _ => assert!(false),
    }
}

#[test]
fn test_c_try_return() {
    assert_eq!((), ffi::c_try_return_void().unwrap());
    assert_eq!(2020, ffi::c_try_return_primitive().unwrap());
    assert_eq!(
        "logic error",
        ffi::c_fail_return_primitive().unwrap_err().what(),
    );
    assert_eq!(2020, ffi::c_try_return_box().unwrap().0);
    assert_eq!("2020", *ffi::c_try_return_ref(&"2020".to_owned()).unwrap());
    assert_eq!("2020", ffi::c_try_return_str("2020").unwrap());
    assert_eq!(b"2020", ffi::c_try_return_sliceu8(b"2020").unwrap());
    assert_eq!("2020", ffi::c_try_return_rust_string().unwrap());
    assert_eq!("2020", &*ffi::c_try_return_unique_ptr_string().unwrap());
}

#[test]
fn test_c_take() {
    let unique_ptr = ffi::c_return_unique_ptr();
    let unique_ptr_ns = ffi2::c_return_ns_unique_ptr();

    check!(ffi::c_take_primitive(2020));
    check!(ffi::c_take_shared(ffi::Shared { z: 2020 }));
    check!(ffi::c_take_ns_shared(ffi::AShared { z: 2020 }));
    check!(ffi::ns_c_take_ns_shared(ffi::AShared { z: 2020 }));
    check!(ffi::c_take_nested_ns_shared(ffi::ABShared { z: 2020 }));
    check!(ffi::c_take_box(Box::new(R(2020))));
    check!(ffi::c_take_ref_c(&unique_ptr));
    check!(ffi2::c_take_ref_ns_c(&unique_ptr_ns));
    check!(cxx_test_suite::module::ffi::c_take_unique_ptr(unique_ptr));
    check!(ffi::c_take_str("2020"));
    check!(ffi::c_take_slice_char(cast::unsigned_to_c_char(b"2020")));
    check!(ffi::c_take_slice_shared(&[
        ffi::Shared { z: 2020 },
        ffi::Shared { z: 2021 },
    ]));
    let shared_sort_slice = &mut [
        ffi::Shared { z: 2 },
        ffi::Shared { z: 0 },
        ffi::Shared { z: 7 },
        ffi::Shared { z: 4 },
    ];
    check!(ffi::c_take_slice_shared_sort(shared_sort_slice));
    assert_eq!(shared_sort_slice[0].z, 0);
    assert_eq!(shared_sort_slice[1].z, 2);
    assert_eq!(shared_sort_slice[2].z, 4);
    assert_eq!(shared_sort_slice[3].z, 7);
    let r_sort_slice = &mut [R(2020), R(2050), R(2021)];
    check!(ffi::c_take_slice_r(r_sort_slice));
    check!(ffi::c_take_slice_r_sort(r_sort_slice));
    assert_eq!(r_sort_slice[0].0, 2020);
    assert_eq!(r_sort_slice[1].0, 2021);
    assert_eq!(r_sort_slice[2].0, 2050);
    check!(ffi::c_take_rust_string("2020".to_owned()));
    check!(ffi::c_take_unique_ptr_string(
        ffi::c_return_unique_ptr_string()
    ));
    let mut vector = ffi::c_return_unique_ptr_vector_u8();
    assert_eq!(vector.pin_mut().pop(), Some(9));
    check!(ffi::c_take_unique_ptr_vector_u8(vector));
    let mut vector = ffi::c_return_unique_ptr_vector_f64();
    vector.pin_mut().extend(Some(9.0));
    assert!(vector.pin_mut().capacity() >= 1);
    vector.pin_mut().reserve(100);
    assert!(vector.pin_mut().capacity() >= 101);
    check!(ffi::c_take_unique_ptr_vector_f64(vector));
    let mut vector = ffi::c_return_unique_ptr_vector_shared();
    vector.pin_mut().push(ffi::Shared { z: 9 });
    check!(ffi::c_take_unique_ptr_vector_shared(vector));
    check!(ffi::c_take_ref_vector(&ffi::c_return_unique_ptr_vector_u8()));
    let test_vec = [86_u8, 75_u8, 30_u8, 9_u8].to_vec();
    check!(ffi::c_take_rust_vec(test_vec.clone()));
    check!(ffi::c_take_rust_vec_index(test_vec.clone()));
    let shared_test_vec = vec![ffi::Shared { z: 1010 }, ffi::Shared { z: 1011 }];
    check!(ffi::c_take_rust_vec_shared(shared_test_vec.clone()));
    check!(ffi::c_take_rust_vec_shared_index(shared_test_vec.clone()));
    check!(ffi::c_take_rust_vec_shared_push(shared_test_vec.clone()));
    check!(ffi::c_take_rust_vec_shared_truncate(
        shared_test_vec.clone()
    ));
    check!(ffi::c_take_rust_vec_shared_clear(shared_test_vec.clone()));
    check!(ffi::c_take_rust_vec_shared_forward_iterator(
        shared_test_vec,
    ));
    let shared_sort_vec = vec![
        ffi::Shared { z: 2 },
        ffi::Shared { z: 0 },
        ffi::Shared { z: 7 },
        ffi::Shared { z: 4 },
    ];
    check!(ffi::c_take_rust_vec_shared_sort(shared_sort_vec));
    check!(ffi::c_take_ref_rust_vec(&test_vec));
    check!(ffi::c_take_ref_rust_vec_index(&test_vec));
    check!(ffi::c_take_ref_rust_vec_copy(&test_vec));
    check!(ffi::c_take_ref_shared_string(&ffi::SharedString {
        msg: "2020".to_owned()
    }));
    let ns_shared_test_vec = vec![ffi::AShared { z: 1010 }, ffi::AShared { z: 1011 }];
    check!(ffi::c_take_rust_vec_ns_shared(ns_shared_test_vec));
    let nested_ns_shared_test_vec = vec![ffi::ABShared { z: 1010 }, ffi::ABShared { z: 1011 }];
    check!(ffi::c_take_rust_vec_nested_ns_shared(
        nested_ns_shared_test_vec
    ));

    check!(ffi::c_take_enum(ffi::Enum::AVal));
    check!(ffi::c_take_ns_enum(ffi::AEnum::AAVal));
    check!(ffi::c_take_nested_ns_enum(ffi::ABEnum::ABAVal));
}

#[test]
fn test_c_callback() {
    fn callback(s: String) -> usize {
        if s == "2020" {
            cxx_test_suite_set_correct();
        }
        0
    }

    #[allow(clippy::ptr_arg)]
    fn callback_ref(s: &String) {
        if s == "2020" {
            cxx_test_suite_set_correct();
        }
    }

    fn callback_mut(s: &mut String) {
        if s == "2020" {
            cxx_test_suite_set_correct();
        }
    }

    check!(ffi::c_take_callback(callback));
    check!(ffi::c_take_callback_ref(callback_ref));
    check!(ffi::c_take_callback_ref_lifetime(callback_ref));
    check!(ffi::c_take_callback_mut(callback_mut));
}

#[test]
fn test_c_call_r() {
    fn cxx_run_test() {
        extern "C" {
            fn cxx_run_test() -> *const i8;
        }
        let failure = unsafe { cxx_run_test() };
        if !failure.is_null() {
            let msg = unsafe { CStr::from_ptr(failure as *mut std::os::raw::c_char) };
            eprintln!("{}", msg.to_string_lossy());
        }
    }
    check!(cxx_run_test());
}

#[test]
fn test_c_method_calls() {
    let mut unique_ptr = ffi::c_return_unique_ptr();

    let old_value = unique_ptr.get();
    assert_eq!(2020, old_value);
    assert_eq!(2021, unique_ptr.pin_mut().set(2021));
    assert_eq!(2021, unique_ptr.get());
    assert_eq!(2021, unique_ptr.get2());
    assert_eq!(2021, *unique_ptr.getRef());
    assert_eq!(2021, unsafe { &mut *unique_ptr.as_mut_ptr() }.get());
    assert_eq!(2021, unsafe { &*unique_ptr.as_ptr() }.get());
    assert_eq!(2021, *unique_ptr.pin_mut().getMut());
    assert_eq!(2022, unique_ptr.pin_mut().set_succeed(2022).unwrap());
    assert!(unique_ptr.pin_mut().get_fail().is_err());
    assert_eq!(2021, ffi::Shared { z: 0 }.c_method_on_shared());
    assert_eq!(2022, *ffi::Shared { z: 2022 }.c_method_ref_on_shared());
    assert_eq!(2023, *ffi::Shared { z: 2023 }.c_method_mut_on_shared());
    assert_eq!(2025, ffi::Shared::c_static_method_on_shared());
    assert_eq!(2026, ffi::C::c_static_method());

    let val = 42;
    let mut array = ffi::WithArray {
        a: [0, 0, 0, 0],
        b: ffi::Buffer::default(),
    };
    array.c_set_array(val);
    assert_eq!(array.a.len() as i32 * val, array.r_get_array_sum());

    R(2020).c_member_function_on_rust_type();
}

#[test]
fn test_shared_ptr_weak_ptr() {
    let shared_ptr = ffi::c_return_shared_ptr();
    let weak_ptr = SharedPtr::downgrade(&shared_ptr);
    assert_eq!(1, ffi::c_get_use_count(&weak_ptr));

    assert!(!weak_ptr.upgrade().is_null());
    assert_eq!(1, ffi::c_get_use_count(&weak_ptr));

    drop(shared_ptr);
    assert_eq!(0, ffi::c_get_use_count(&weak_ptr));
    assert!(weak_ptr.upgrade().is_null());
}

#[test]
fn test_unique_to_shared_ptr_string() {
    let unique = ffi::c_return_unique_ptr_string();
    let ptr = ptr::addr_of!(*unique);
    let shared = SharedPtr::from(unique);
    assert_eq!(ptr::addr_of!(*shared), ptr);
    assert_eq!(*shared, *"2020");
}

#[test]
fn test_unique_to_shared_ptr_cpp_type() {
    let unique = ffi::c_return_unique_ptr();
    let ptr = ptr::addr_of!(*unique);
    let shared = SharedPtr::from(unique);
    assert_eq!(ptr::addr_of!(*shared), ptr);
}

#[test]
fn test_unique_to_shared_ptr_null() {
    let unique = UniquePtr::<ffi::C>::null();
    assert!(unique.is_null());
    let shared = SharedPtr::from(unique);
    assert!(shared.is_null());
}

#[test]
fn test_shared_ptr_from_raw() {
    let shared = unsafe { SharedPtr::<ffi::C>::from_raw(ptr::null_mut()) };
    assert!(shared.is_null());
}

#[test]
#[should_panic = "tests::Undefined is not destructible"]
fn test_shared_ptr_from_raw_undefined() {
    unsafe { SharedPtr::<ffi::Undefined>::from_raw(ptr::null_mut()) };
}

#[test]
#[should_panic = "tests::Private is not destructible"]
fn test_shared_ptr_from_raw_private() {
    unsafe { SharedPtr::<ffi::Private>::from_raw(ptr::null_mut()) };
}

#[test]
#[should_panic = "tests::Unmovable is not move constructible"]
fn test_vector_reserve_unmovable() {
    let mut vector = CxxVector::<ffi::Unmovable>::new();
    vector.pin_mut().reserve(10);
}

#[test]
fn test_c_ns_method_calls() {
    let unique_ptr = ffi2::ns_c_return_unique_ptr_ns();

    let old_value = unique_ptr.get();
    assert_eq!(1000, old_value);
}

#[test]
fn test_enum_representations() {
    assert_eq!(0, ffi::Enum::AVal.repr);
    assert_eq!(2020, ffi::Enum::BVal.repr);
    assert_eq!(2021, ffi::Enum::LastVal.repr);
}

#[test]
fn test_enum_default() {
    assert_eq!(ffi::Enum::BVal, ffi::Enum::default());
}

#[test]
fn test_struct_repr_align() {
    assert_eq!(4, std::mem::align_of::<ffi::OveralignedStruct>());
}

#[test]
fn test_debug() {
    assert_eq!("Shared { z: 1 }", format!("{:?}", ffi::Shared { z: 1 }));
    assert_eq!("BVal", format!("{:?}", ffi::Enum::BVal));
    assert_eq!("Enum(9)", format!("{:?}", ffi::Enum { repr: 9 }));
}

#[no_mangle]
extern "C" fn cxx_test_suite_get_box() -> *mut R {
    Box::into_raw(Box::new(R(2020usize)))
}

#[no_mangle]
unsafe extern "C" fn cxx_test_suite_r_is_correct(r: *const R) -> bool {
    (*r).0 == 2020
}

#[test]
fn test_rust_name_attribute() {
    assert_eq!("2020", ffi::i32_overloaded_function(2020));
    assert_eq!("2020", ffi::str_overloaded_function("2020"));
    let unique_ptr = ffi::c_return_unique_ptr();
    assert_eq!("2020", unique_ptr.i32_overloaded_method(2020));
    assert_eq!("2020", unique_ptr.str_overloaded_method("2020"));
}

#[test]
fn test_extern_trivial() {
    let mut d = ffi2::c_return_trivial();
    check!(ffi2::c_take_trivial_ref(&d));
    check!(d.c_take_trivial_ref_method());
    check!(d.c_take_trivial_mut_ref_method());
    check!(ffi2::c_take_trivial(d));
    let mut d = ffi2::c_return_trivial_ptr();
    check!(d.c_take_trivial_ref_method());
    check!(d.c_take_trivial_mut_ref_method());
    check!(ffi2::c_take_trivial_ptr(d));
    cxx::UniquePtr::new(ffi2::D { d: 42 });
    let d = ffi2::ns_c_return_trivial();
    check!(ffi2::ns_c_take_trivial(d));

    let g = ffi2::c_return_trivial_ns();
    check!(ffi2::c_take_trivial_ns_ref(&g));
    check!(ffi2::c_take_trivial_ns(g));
    let g = ffi2::c_return_trivial_ns_ptr();
    check!(ffi2::c_take_trivial_ns_ptr(g));
    cxx::UniquePtr::new(ffi2::G { g: 42 });
}

#[test]
fn test_extern_opaque() {
    let mut e = ffi2::c_return_opaque_ptr();
    check!(ffi2::c_take_opaque_ref(e.as_ref().unwrap()));
    check!(e.c_take_opaque_ref_method());
    check!(e.pin_mut().c_take_opaque_mut_ref_method());
    check!(ffi2::c_take_opaque_ptr(e));

    let f = ffi2::c_return_ns_opaque_ptr();
    check!(ffi2::c_take_opaque_ns_ref(f.as_ref().unwrap()));
    check!(ffi2::c_take_opaque_ns_ptr(f));
}

#[test]
fn test_raw_ptr() {
    let c = ffi::c_return_mut_ptr(2023);
    let mut c_unique = unsafe { cxx::UniquePtr::from_raw(c) };
    assert_eq!(2023, c_unique.pin_mut().set_succeed(2023).unwrap());
    // c will be dropped as it's now in a UniquePtr

    let c2 = ffi::c_return_mut_ptr(2024);
    assert_eq!(2024, unsafe { ffi::c_take_const_ptr(c2) });
    assert_eq!(2024, unsafe { ffi::c_take_mut_ptr(c2) }); // deletes c2

    let c3 = ffi::c_return_const_ptr(2025);
    assert_eq!(2025, unsafe { ffi::c_take_const_ptr(c3) });
    assert_eq!(2025, unsafe { ffi::c_take_mut_ptr(c3 as *mut ffi::C) }); // deletes c3
}

#[test]
#[allow(clippy::items_after_statements, clippy::no_effect_underscore_binding)]
fn test_unwind_safe() {
    fn inspect(_c: &ffi::C) {}
    let _unwind_safe = |c: UniquePtr<ffi::C>| panic::catch_unwind(|| drop(c));
    let _ref_unwind_safe = |c: &ffi::C| panic::catch_unwind(|| inspect(c));

    fn require_unwind_safe<T: UnwindSafe>() {}
    require_unwind_safe::<ffi::C>();

    fn require_ref_unwind_safe<T: RefUnwindSafe>() {}
    require_ref_unwind_safe::<ffi::C>();
}
