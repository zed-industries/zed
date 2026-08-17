//! A single-test executable which only tests `Engine::unload_process_handlers`
//! is possible.
//!
//! It's not safe for this binary to contain any other tests.

#![cfg(not(miri))]
#![cfg(has_native_signals)]

use wasmtime::*;

#[test]
fn test_unload_engine() {
    for _ in 0..3 {
        std::thread::spawn(|| {
            let engine = Engine::default();
            {
                let module =
                    Module::new(&engine, r#"(module (func (export "x") unreachable))"#).unwrap();
                let mut store = Store::new(&engine, ());
                let instance = Instance::new(&mut store, &module, &[]).unwrap();
                let func = instance.get_typed_func::<(), ()>(&mut store, "x").unwrap();
                assert!(func.call(&mut store, ()).unwrap_err().is::<Trap>());
            }
            unsafe {
                engine.unload_process_handlers();
            }
        })
        .join()
        .unwrap();
    }
}
