// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem::transmute;

use crate::{
    ffi, gobject_ffi,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    Object, RustClosure, SignalGroup, Value,
};

impl SignalGroup {
    #[doc(alias = "g_signal_group_new")]
    pub fn new<T: IsA<Object>>() -> Self {
        Self::with_type(T::static_type())
    }
    #[doc(alias = "g_signal_group_connect_closure")]
    pub fn connect_closure(&self, signal_name: &str, after: bool, closure: RustClosure) {
        unsafe {
            gobject_ffi::g_signal_group_connect_closure(
                self.to_glib_none().0,
                signal_name.to_glib_none().0,
                closure.as_ref().to_glib_none().0,
                after.into_glib(),
            );
        }
    }

    #[doc(alias = "g_signal_group_connect")]
    #[inline]
    pub fn connect<F>(&self, signal_name: &str, after: bool, callback: F)
    where
        F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static,
    {
        self.connect_closure(signal_name, after, RustClosure::new(callback));
    }

    // rustdoc-stripper-ignore-next
    /// Like [`Self::connect`] but doesn't require a `Send+Sync` closure. Signal emission will
    /// panic if the signal on the current target is emitted from a different thread from the
    /// thread that connected the signal.
    #[inline]
    pub fn connect_local<F>(&self, signal_name: &str, after: bool, callback: F)
    where
        F: Fn(&[Value]) -> Option<Value> + 'static,
    {
        self.connect_closure(signal_name, after, RustClosure::new_local(callback));
    }

    #[inline]
    pub fn connect_notify<F>(&self, name: Option<&str>, callback: F)
    where
        F: Fn(&crate::Object, &crate::ParamSpec) + Send + Sync + 'static,
    {
        let signal_name = if let Some(name) = name {
            format!("notify::{name}")
        } else {
            "notify".into()
        };

        let closure = crate::RustClosure::new(move |values| {
            let obj = values[0].get().unwrap();
            let pspec = values[1].get().unwrap();
            callback(obj, pspec);

            None
        });

        self.connect_closure(&signal_name, false, closure);
    }

    #[inline]
    pub fn connect_notify_local<F>(&self, name: Option<&str>, callback: F)
    where
        F: Fn(&crate::Object, &crate::ParamSpec) + 'static,
    {
        let signal_name = if let Some(name) = name {
            format!("notify::{name}")
        } else {
            "notify".into()
        };

        let closure = crate::RustClosure::new_local(move |values| {
            let obj = values[0].get().unwrap();
            let pspec = values[1].get().unwrap();
            callback(obj, pspec);

            None
        });

        self.connect_closure(&signal_name, false, closure);
    }

    unsafe fn connect_bind_unsafe<F: Fn(&Self, &Object)>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn bind_trampoline<F: Fn(&SignalGroup, &Object)>(
            this: *mut crate::gobject_ffi::GSignalGroup,
            instance: *mut crate::gobject_ffi::GObject,
            f: ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(instance))
        }
        let f: Box<F> = Box::new(f);
        connect_raw(
            self.as_ptr() as *mut _,
            b"bind\0".as_ptr() as *const _,
            Some(transmute::<*const (), unsafe extern "C" fn()>(
                bind_trampoline::<F> as *const (),
            )),
            Box::into_raw(f),
        )
    }

    unsafe fn connect_unbind_unsafe<F: Fn(&Self)>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn unbind_trampoline<F: Fn(&SignalGroup)>(
            this: *mut crate::gobject_ffi::GSignalGroup,
            f: ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        let f: Box<F> = Box::new(f);
        connect_raw(
            self.as_ptr() as *mut _,
            b"unbind\0".as_ptr() as *const _,
            Some(transmute::<*const (), unsafe extern "C" fn()>(
                unbind_trampoline::<F> as *const (),
            )),
            Box::into_raw(f),
        )
    }

    #[doc(alias = "bind")]
    pub fn connect_bind<F: Fn(&Self, &Object) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe { self.connect_bind_unsafe(f) }
    }

    // rustdoc-stripper-ignore-next
    /// Like [`Self::connect_bind`] but doesn't require a `Send+Sync` closure. Signal emission will
    /// panic if the signal is emitted from a different thread from the thread that connected the
    /// signal.
    pub fn connect_bind_local<F: Fn(&Self, &Object) + 'static>(&self, f: F) -> SignalHandlerId {
        let f = crate::thread_guard::ThreadGuard::new(f);

        unsafe {
            self.connect_bind_unsafe(move |s, o| {
                (f.get_ref())(s, o);
            })
        }
    }

    #[doc(alias = "unbind")]
    pub fn connect_unbind<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe { self.connect_unbind_unsafe(f) }
    }

    // rustdoc-stripper-ignore-next
    /// Like [`Self::connect_unbind`] but doesn't require a `Send+Sync` closure. Signal emission
    /// will panic if the signal is emitted from a different thread from the thread that connected
    /// the signal.
    pub fn connect_unbind_local<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        let f = crate::thread_guard::ThreadGuard::new(f);

        unsafe {
            self.connect_unbind_unsafe(move |s| {
                (f.get_ref())(s);
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc, sync::OnceLock};

    use super::*;
    use crate as glib;

    mod imp {
        use super::*;
        use crate::subclass::{prelude::*, Signal};

        #[derive(Default)]
        pub struct SignalObject {}

        #[glib::object_subclass]
        impl ObjectSubclass for SignalObject {
            const NAME: &'static str = "SignalObject";
            type Type = super::SignalObject;
        }

        impl ObjectImpl for SignalObject {
            fn signals() -> &'static [Signal] {
                static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
                SIGNALS.get_or_init(|| {
                    vec![
                        Signal::builder("sig-with-args")
                            .param_types([u32::static_type(), String::static_type()])
                            .build(),
                        Signal::builder("sig-with-ret")
                            .return_type::<String>()
                            .build(),
                    ]
                })
            }
        }
    }

    wrapper! {
        pub struct SignalObject(ObjectSubclass<imp::SignalObject>);
    }

    #[test]
    fn group_emit() {
        let group = SignalGroup::new::<SignalObject>();

        let obj = Object::new::<SignalObject>();
        let store = Rc::new(RefCell::new(String::new()));
        group.connect_closure(
            "sig-with-args",
            false,
            glib::closure_local!(
                #[watch]
                obj,
                #[strong]
                store,
                move |o: &SignalObject, a: u32, b: &str| {
                    assert_eq!(o, obj);
                    store.replace(format!("a {a} b {b}"));
                }
            ),
        );
        group.connect_closure(
            "sig-with-ret",
            false,
            glib::closure_local!(
                #[watch]
                obj,
                move |o: &SignalObject| -> &'static crate::GStr {
                    assert_eq!(o, obj);
                    crate::gstr!("Hello")
                }
            ),
        );
        group.set_target(Some(&obj));
        obj.emit_by_name::<()>("sig-with-args", &[&5u32, &"World"]);
        assert_eq!(*store.borrow(), "a 5 b World");
        let ret = obj.emit_by_name::<crate::GString>("sig-with-ret", &[]);
        assert_eq!(ret, "Hello");
        group.set_target(Object::NONE);
        let ret = obj.emit_by_name::<Option<String>>("sig-with-ret", &[]);
        assert_eq!(ret, None);
    }
}
