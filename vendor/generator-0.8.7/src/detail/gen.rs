use crate::rt::ContextStack;
use crate::stack::{overflow, Func};
use crate::yield_::yield_now;
use crate::Error;
use std::any::Any;
use std::panic;

/// don't print panic info for Done/Cancel
fn catch_unwind_filter<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> std::thread::Result<R> {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            // this is not an error at all, ignore it
            if let Some(Error::Cancel | Error::Done) = info.payload().downcast_ref::<Error>() {
                return;
            }
            prev_hook(info);
        }));
    });

    panic::catch_unwind(f)
}

/// the init function passed to reg_context
#[inline]
pub fn gen_init_impl(_: usize, f: *mut usize) -> ! {
    overflow::init_once();

    let clo = move || {
        // consume self.f
        let f: &mut Option<Func> = unsafe { &mut *(f as *mut _) };
        let func = f.take().unwrap();
        func.call_once();
    };

    fn check_err(cause: Box<dyn Any + Send + 'static>) {
        // this is not an error at all, ignore it
        if let Some(Error::Cancel | Error::Done) = cause.downcast_ref::<Error>() {
            return;
        }

        error!("set panic inside generator");
        ContextStack::current().top().err = Some(cause);
    }

    // we can't panic inside the generator context
    // need to propagate the panic to the main thread
    if let Err(cause) = catch_unwind_filter(clo) {
        check_err(cause);
    }

    yield_now();

    unreachable!("Should never come back");
}
