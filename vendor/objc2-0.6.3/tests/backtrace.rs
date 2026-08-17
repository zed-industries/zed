#![cfg(target_vendor = "apple")] // The test is very Apple centric
use std::ffi::c_void;

use objc2::{define_class, extern_methods, msg_send, ClassType};
use objc2_foundation::{NSException, NSObject};

#[allow(dead_code)]
fn merge_objc_symbols(exc: &NSException) -> Vec<String> {
    // Objective-C and Rust have different mangling schemes, and don't really
    // understand each other. So we use both `NSException`'s backtrace, and
    // `backtrace`' resolving mechanism, to figure out the full list of
    // demangled symbols.
    let mut demangled_symbols = vec![];

    let nssymbols = unsafe { exc.callStackSymbols() };
    let return_addrs = unsafe { exc.callStackReturnAddresses() };

    for (nssymbol, addr) in nssymbols.iter().zip(return_addrs) {
        let addr = addr.as_usize() as *mut c_void;
        let mut call_count = 0;
        backtrace::resolve(addr, |symbol| {
            if let Some(name) = symbol.name() {
                demangled_symbols.push(name.to_string());
            } else {
                demangled_symbols.push(format!("{nssymbol} ({:?})", symbol.addr()));
            }
            call_count += 1;
        });
        if call_count == 0 {
            demangled_symbols.push(nssymbol.to_string());
        }
    }

    demangled_symbols
}

#[test]
#[cfg(feature = "exception")]
#[cfg_attr(feature = "catch-all", ignore = "catch-all interferes with our catch")]
fn array_exception() {
    use objc2::rc::Retained;
    use objc2_foundation::NSArray;

    #[no_mangle]
    fn array_exception_via_msg_send() {
        let arr = NSArray::<NSObject>::new();
        let _: Retained<NSObject> = unsafe { msg_send![&arr, objectAtIndex: 0usize] };
    }
    let expected_msg_send = [
        "__exceptionPreprocess",
        "objc_exception_throw",
        "CFArrayApply",
        "<(A,) as objc2::encode::EncodeArguments>::__invoke",
        "objc2::runtime::message_receiver::msg_send_primitive::send",
        "objc2::runtime::message_receiver::MessageReceiver::send_message",
        "<MethodFamily as objc2::__macro_helpers::msg_send_retained::MsgSend<Receiver,Return>>::send_message",
        "array_exception_via_msg_send",
    ];

    #[no_mangle]
    fn array_exception_via_extern_methods() {
        let arr = NSArray::<NSObject>::new();
        let _ = arr.objectAtIndex(0);
    }
    let expected_extern_methods = [
        "__exceptionPreprocess",
        "objc_exception_throw",
        "CFArrayApply",
        "<(A,) as objc2::encode::EncodeArguments>::__invoke",
        "objc2::runtime::message_receiver::msg_send_primitive::send",
        "objc2::runtime::message_receiver::MessageReceiver::send_message",
        "<MethodFamily as objc2::__macro_helpers::msg_send_retained::MsgSend<Receiver,Return>>::send_message",
        "objc2_foundation::generated::__NSArray::NSArray<ObjectType>::objectAtIndex",
        "array_exception_via_extern_methods",
    ];

    for (fnptr, expected) in [
        (
            array_exception_via_msg_send as fn(),
            &expected_msg_send as &[_],
        ),
        (array_exception_via_extern_methods, &expected_extern_methods),
    ] {
        let res = objc2::exception::catch(fnptr);
        let exc = res.unwrap_err().unwrap();
        let exc = exc.downcast::<NSException>().unwrap();

        let symbols = merge_objc_symbols(&exc);

        // No debug info available, such as when using `--release`.
        if symbols[3] == "__mh_execute_header" {
            continue;
        }

        if symbols.len() < expected.len() {
            panic!("did not find enough symbols: {symbols:?}");
        }

        for (expected, actual) in expected.iter().zip(&symbols) {
            assert!(
                actual.contains(expected),
                "{expected:?} must be in {actual:?}:\n{symbols:#?}",
            );
        }
    }
}

define_class!(
    #[unsafe(super = NSObject)]
    struct Thrower;

    impl Thrower {
        #[unsafe(method(backtrace))]
        fn __backtrace() -> *mut c_void {
            let backtrace = backtrace::Backtrace::new();
            Box::into_raw(Box::new(backtrace)).cast()
        }
    }
);

impl Thrower {
    extern_methods!(
        #[unsafe(method(backtrace))]
        fn backtrace() -> *mut c_void;
    );
}

#[test]
#[cfg_attr(feature = "catch-all", ignore = "catch-all changes the backtrace")]
fn capture_backtrace() {
    #[no_mangle]
    fn rust_backtrace_via_msg_send() -> backtrace::Backtrace {
        let ptr: *mut c_void = unsafe { msg_send![Thrower::class(), backtrace] };
        *unsafe { Box::from_raw(ptr.cast()) }
    }
    let expected_msg_send: &[_] = &[
        "Backtrace::new",
        "Thrower::__backtrace",
        "<() as objc2::encode::EncodeArguments>::__invoke",
        "objc2::runtime::message_receiver::msg_send_primitive::send",
        "objc2::runtime::message_receiver::MessageReceiver::send_message",
        "<MethodFamily as objc2::__macro_helpers::msg_send_retained::MsgSend<Receiver,Return>>::send_message",
        "rust_backtrace_via_msg_send",
    ];

    #[no_mangle]
    fn rust_backtrace_via_extern_methods() -> backtrace::Backtrace {
        let ptr = Thrower::backtrace();
        *unsafe { Box::from_raw(ptr.cast()) }
    }
    let expected_extern_methods: &[_] = &[
        "Backtrace::new",
        "Thrower::__backtrace",
        "<() as objc2::encode::EncodeArguments>::__invoke",
        "objc2::runtime::message_receiver::msg_send_primitive::send",
        "objc2::runtime::message_receiver::MessageReceiver::send_message",
        "<MethodFamily as objc2::__macro_helpers::msg_send_retained::MsgSend<Receiver,Return>>::send_message",
        "Thrower::backtrace",
        "rust_backtrace_via_extern_methods",
    ];

    for (backtrace, expected) in [
        (rust_backtrace_via_msg_send(), expected_msg_send),
        (rust_backtrace_via_extern_methods(), expected_extern_methods),
    ] {
        let symbols: Vec<_> = backtrace
            .frames()
            .iter()
            .flat_map(|frame| frame.symbols())
            .map(|symbol| {
                symbol
                    .name()
                    .map(|name| name.to_string())
                    .unwrap_or_default()
            })
            .skip_while(|name| !name.contains("Backtrace::new"))
            .collect();

        // No debug info available, such as when using `--release`.
        if backtrace.frames()[0].symbols()[0]
            .name()
            .unwrap()
            .to_string()
            == "__mh_execute_header"
        {
            continue;
        }

        if symbols.len() < expected.len() {
            panic!("did not find enough symbols: {backtrace:?}");
        }

        for (expected, actual) in expected.iter().zip(&symbols) {
            assert!(
                actual.contains(expected),
                "{expected:?} must be in {actual:?}:\n{symbols:#?}",
            );
        }
    }
}
