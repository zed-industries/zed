extern crate libc;

#[macro_use]
extern crate lazy_static;

pub mod dynamic_library;

/// Error that can happen while loading the shared library.
#[derive(Debug, Clone)]
pub enum LoadingError {
    /// 
    LibraryNotFound {
        descr: String,
    },

    /// One of the symbols could not be found in the library.
    SymbolNotFound {
        /// The symbol.
        symbol: &'static str,
    }
}

#[macro_export]
macro_rules! shared_library {
    ($struct_name:ident, pub $($rest:tt)+) => {
        shared_library!(__impl $struct_name [] [] [] pub $($rest)+);
    };

    ($struct_name:ident, fn $($rest:tt)+) => {
        shared_library!(__impl $struct_name [] [] [] fn $($rest)+);
    };

    ($struct_name:ident, static $($rest:tt)+) => {
        shared_library!(__impl $struct_name [] [] [] static $($rest)+);
    };

    ($struct_name:ident, $def_path:expr, $($rest:tt)+) => {
        shared_library!(__impl $struct_name [] [$def_path] [] $($rest)+);
    };

    (__impl $struct_name:ident
            [$($p1:tt)*] [$($p2:tt)*] [$($p3:tt)*]
            , $($rest:tt)*
    ) => {
        shared_library!(__impl $struct_name [$($p1)*] [$($p2)*] [$($p3)*] $($rest)*);
    };

    (__impl $struct_name:ident
            [$($p1:tt)*] [$($p2:tt)*] [$($p3:tt)*]
            pub $($rest:tt)*
    ) => {
        shared_library!(__impl $struct_name
                       [$($p1)*] [$($p2)*] [$($p3)* pub] $($rest)*);
    };

    (__impl $struct_name:ident
            [$($p1:tt)*] [$($p2:tt)*] [$($p3:tt)*]
            fn $name:ident($($p:ident:$ty:ty),*) -> $ret:ty, $($rest:tt)*
    ) => {
        shared_library!(__impl $struct_name
                       [$($p1)*, $name:unsafe extern fn($($p:$ty),*) -> $ret]
                       [$($p2)*]
                       [$($p3)*
                           unsafe fn $name($($p:$ty),*) -> $ret {
                               #![allow(dead_code)]
                               ($struct_name::get_static_ref().$name)($($p),*)
                           }
                        ] $($rest)*);
    };

    (__impl $struct_name:ident
            [$($p1:tt)*] [$($p2:tt)*] [$($p3:tt)*]
            static $name:ident:$ty:ty, $($rest:tt)*
    ) => {
        shared_library!(__impl $struct_name
                       [$($p1)*, $name: $ty]
                       [$($p2)*]
                       [$($p3)*] $($rest)*);
    };

    (__impl $struct_name:ident
            [$($p1:tt)*] [$($p2:tt)*] [$($p3:tt)*]
            fn $name:ident($($p:ident:$ty:ty),*), $($rest:tt)*
    ) => {
        shared_library!(__impl $struct_name
                       [$($p1)*] [$($p2)*] [$($p3)*]
                       fn $name($($p:$ty),*) -> (), $($rest)*);
    };

    (__impl $struct_name:ident [$(,$mem_n:ident:$mem_t:ty)+] [$($p2:tt)*] [$($p3:tt)*]) => {
        /// Symbols loaded from a shared library.
        #[allow(non_snake_case)]
        pub struct $struct_name {
            _library_guard: $crate::dynamic_library::DynamicLibrary,
            $(
                pub $mem_n: $mem_t,
            )+
        }

        impl $struct_name {
            /// Tries to open the dynamic library.
            #[allow(non_snake_case)]
            pub fn open(path: &::std::path::Path) -> Result<$struct_name, $crate::LoadingError> {
                use std::mem;

                let dylib = match $crate::dynamic_library::DynamicLibrary::open(Some(path)) {
                    Ok(l) => l,
                    Err(reason) => return Err($crate::LoadingError::LibraryNotFound { descr: reason })
                };

                $(
                    let $mem_n: *mut () = match unsafe { dylib.symbol(stringify!($mem_n)) } {
                        Ok(s) => s,
                        Err(_) => return Err($crate::LoadingError::SymbolNotFound { symbol: stringify!($mem_n) }),
                    };
                )+

                Ok($struct_name {
                    _library_guard: dylib,
                    $(
                        $mem_n: unsafe { mem::transmute($mem_n) },
                    )+
                })
            }
        }

        shared_library!(__write_static_fns $struct_name [] [$($p2)*] [$($p3)*]);
    };

    (__write_static_fns $struct_name:ident [$($p1:tt)*] [] [$($p3:tt)*]) => {
    };

    (__write_static_fns $struct_name:ident [$($p1:tt)*] [$defpath:expr] [$($standalones:item)+]) => {
        impl $struct_name {
            /// This function is used by the regular functions.
            fn get_static_ref() -> &'static $struct_name {
                $struct_name::try_loading().ok()
                                           .expect(concat!("Could not open dynamic \
                                                            library `", stringify!($struct_name),
                                                            "`"))
            }

            /// Try loading the static symbols linked to this library.
            pub fn try_loading() -> Result<&'static $struct_name, $crate::LoadingError> {
                use std::sync::{Mutex, Once, ONCE_INIT};
                use std::mem;

                unsafe {
                    static mut DATA: *const Mutex<Option<$struct_name>> = 0 as *const _;

                    static mut INIT: Once = ONCE_INIT;
                    INIT.call_once(|| {
                        let data = Box::new(Mutex::new(None));
                        DATA = &*data;
                        mem::forget(data);
                    });

                    let data: &Mutex<Option<$struct_name>> = &*DATA;
                    let mut data = data.lock().unwrap();

                    if let Some(ref data) = *data {
                        return Ok(mem::transmute(data));
                    }

                    let path = ::std::path::Path::new($defpath);
                    let result = try!($struct_name::open(path));
                    *data = Some(result);
                    Ok(mem::transmute(data.as_ref().unwrap()))
                }
            }
        }

        $($standalones)+
    };
}
