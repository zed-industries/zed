/*!
Functionality for declaring Objective-C classes.

Classes can be declared using the `ClassDecl` struct. Instance variables and
methods can then be added before the class is ultimately registered.

# Example

The following example demonstrates declaring a class named `MyNumber` that has
one ivar, a `u32` named `_number` and a `number` method that returns it:

``` no_run
# #[macro_use] extern crate objc;
# use objc::declare::ClassDecl;
# use objc::runtime::{Class, Object, Sel};
# fn main() {
let superclass = class!(NSObject);
let mut decl = ClassDecl::new("MyNumber", superclass).unwrap();

// Add an instance variable
decl.add_ivar::<u32>("_number");

// Add an ObjC method for getting the number
extern fn my_number_get(this: &Object, _cmd: Sel) -> u32 {
    unsafe { *this.get_ivar("_number") }
}
unsafe {
    decl.add_method(sel!(number),
        my_number_get as extern fn(&Object, Sel) -> u32);
}

decl.register();
# }
```
*/

use std::ffi::CString;
use std::mem;
use std::ptr;

use runtime::{BOOL, Class, Imp, NO, Object, Protocol, Sel, self};
use {Encode, EncodeArguments, Encoding, Message};

/// Types that can be used as the implementation of an Objective-C method.
pub trait MethodImplementation {
    /// The callee type of the method.
    type Callee: Message;
    /// The return type of the method.
    type Ret: Encode;
    /// The argument types of the method.
    type Args: EncodeArguments;

    /// Returns self as an `Imp` of a method.
    fn imp(self) -> Imp;
}

macro_rules! method_decl_impl {
    (-$s:ident, $r:ident, $f:ty, $($t:ident),*) => (
        impl<$s, $r $(, $t)*> MethodImplementation for $f
                where $s: Message, $r: Encode $(, $t: Encode)* {
            type Callee = $s;
            type Ret = $r;
            type Args = ($($t,)*);

            fn imp(self) -> Imp {
                unsafe { mem::transmute(self) }
            }
        }
    );
    ($($t:ident),*) => (
        method_decl_impl!(-T, R, extern fn(&T, Sel $(, $t)*) -> R, $($t),*);
        method_decl_impl!(-T, R, extern fn(&mut T, Sel $(, $t)*) -> R, $($t),*);
    );
}

method_decl_impl!();
method_decl_impl!(A);
method_decl_impl!(A, B);
method_decl_impl!(A, B, C);
method_decl_impl!(A, B, C, D);
method_decl_impl!(A, B, C, D, E);
method_decl_impl!(A, B, C, D, E, F);
method_decl_impl!(A, B, C, D, E, F, G);
method_decl_impl!(A, B, C, D, E, F, G, H);
method_decl_impl!(A, B, C, D, E, F, G, H, I);
method_decl_impl!(A, B, C, D, E, F, G, H, I, J);
method_decl_impl!(A, B, C, D, E, F, G, H, I, J, K);
method_decl_impl!(A, B, C, D, E, F, G, H, I, J, K, L);

fn count_args(sel: Sel) -> usize {
    sel.name().chars().filter(|&c| c == ':').count()
}

fn method_type_encoding(ret: &Encoding, args: &[Encoding]) -> CString {
    let mut types = ret.as_str().to_owned();
    // First two arguments are always self and the selector
    types.push_str(<*mut Object>::encode().as_str());
    types.push_str(Sel::encode().as_str());
    types.extend(args.iter().map(|e| e.as_str()));
    CString::new(types).unwrap()
}

fn log2_align_of<T>() -> u8 {
    let align = mem::align_of::<T>();
    // Alignments are required to be powers of 2
    debug_assert!(align.count_ones() == 1);
    // log2 of a power of 2 is the number of trailing zeros
    align.trailing_zeros() as u8
}

/// A type for declaring a new class and adding new methods and ivars to it
/// before registering it.
pub struct ClassDecl {
    cls: *mut Class,
}

impl ClassDecl {
    fn with_superclass(name: &str, superclass: Option<&Class>)
            -> Option<ClassDecl> {
        let name = CString::new(name).unwrap();
        let super_ptr = superclass.map_or(ptr::null(), |c| c);
        let cls = unsafe {
            runtime::objc_allocateClassPair(super_ptr, name.as_ptr(), 0)
        };
        if cls.is_null() {
            None
        } else {
            Some(ClassDecl { cls: cls })
        }
    }

    /// Constructs a `ClassDecl` with the given name and superclass.
    /// Returns `None` if the class couldn't be allocated.
    pub fn new(name: &str, superclass: &Class) -> Option<ClassDecl> {
        ClassDecl::with_superclass(name, Some(superclass))
    }

    /**
    Constructs a `ClassDecl` declaring a new root class with the given name.
    Returns `None` if the class couldn't be allocated.

    An implementation for `+initialize` must also be given; the runtime calls
    this method for all classes, so it must be defined on root classes.

    Note that implementing a root class is not a simple endeavor.
    For example, your class probably cannot be passed to Cocoa code unless
    the entire `NSObject` protocol is implemented.
    Functionality it expects, like implementations of `-retain` and `-release`
    used by ARC, will not be present otherwise.
    */
    pub fn root(name: &str, intitialize_fn: extern fn(&Class, Sel))
            -> Option<ClassDecl> {
        let mut decl = ClassDecl::with_superclass(name, None);
        if let Some(ref mut decl) = decl {
            unsafe {
                decl.add_class_method(sel!(initialize), intitialize_fn);
            }
        }
        decl
    }

    /// Adds a method with the given name and implementation to self.
    /// Panics if the method wasn't sucessfully added
    /// or if the selector and function take different numbers of arguments.
    /// Unsafe because the caller must ensure that the types match those that
    /// are expected when the method is invoked from Objective-C.
    pub unsafe fn add_method<F>(&mut self, sel: Sel, func: F)
            where F: MethodImplementation<Callee=Object> {
        let encs = F::Args::encodings();
        let encs = encs.as_ref();
        let sel_args = count_args(sel);
        assert!(sel_args == encs.len(),
            "Selector accepts {} arguments, but function accepts {}",
            sel_args, encs.len(),
        );

        let types = method_type_encoding(&F::Ret::encode(), encs);
        let success = runtime::class_addMethod(self.cls, sel, func.imp(),
            types.as_ptr());
        assert!(success != NO, "Failed to add method {:?}", sel);
    }

    /// Adds a class method with the given name and implementation to self.
    /// Panics if the method wasn't sucessfully added
    /// or if the selector and function take different numbers of arguments.
    /// Unsafe because the caller must ensure that the types match those that
    /// are expected when the method is invoked from Objective-C.
    pub unsafe fn add_class_method<F>(&mut self, sel: Sel, func: F)
            where F: MethodImplementation<Callee=Class> {
        let encs = F::Args::encodings();
        let encs = encs.as_ref();
        let sel_args = count_args(sel);
        assert!(sel_args == encs.len(),
            "Selector accepts {} arguments, but function accepts {}",
            sel_args, encs.len(),
        );

        let types = method_type_encoding(&F::Ret::encode(), encs);
        let metaclass = (*self.cls).metaclass() as *const _ as *mut _;
        let success = runtime::class_addMethod(metaclass, sel, func.imp(),
            types.as_ptr());
        assert!(success != NO, "Failed to add class method {:?}", sel);
    }

    /// Adds an ivar with type `T` and the provided name to self.
    /// Panics if the ivar wasn't successfully added.
    pub fn add_ivar<T>(&mut self, name: &str) where T: Encode {
        let c_name = CString::new(name).unwrap();
        let encoding = CString::new(T::encode().as_str()).unwrap();
        let size = mem::size_of::<T>();
        let align = log2_align_of::<T>();
        let success = unsafe {
            runtime::class_addIvar(self.cls, c_name.as_ptr(), size, align,
                encoding.as_ptr())
        };
        assert!(success != NO, "Failed to add ivar {}", name);
    }

    /// Adds a protocol to self. Panics if the protocol wasn't successfully
    /// added
    pub fn add_protocol(&mut self, proto: &Protocol) {
        let success = unsafe { runtime::class_addProtocol(self.cls, proto) };
        assert!(success != NO, "Failed to add protocol {:?}", proto);
    }

    /// Registers self, consuming it and returning a reference to the
    /// newly registered `Class`.
    pub fn register(self) -> &'static Class {
        unsafe {
            let cls = self.cls;
            runtime::objc_registerClassPair(cls);
            // Forget self otherwise the class will be disposed in drop
            mem::forget(self);
            &*cls
        }
    }
}

impl Drop for ClassDecl {
    fn drop(&mut self) {
        unsafe {
            runtime::objc_disposeClassPair(self.cls);
        }
    }
}

/// A type for declaring a new protocol and adding new methods to it
/// before registering it.
pub struct ProtocolDecl {
    proto: *mut Protocol
}

impl ProtocolDecl {
    /// Constructs a `ProtocolDecl` with the given name. Returns `None` if the
    /// protocol couldn't be allocated.
    pub fn new(name: &str) -> Option<ProtocolDecl> {
        let c_name = CString::new(name).unwrap();
        let proto = unsafe {
            runtime::objc_allocateProtocol(c_name.as_ptr())
        };
        if proto.is_null() {
            None
        } else {
            Some(ProtocolDecl { proto: proto })
        }
    }

    fn add_method_description_common<Args, Ret>(&mut self, sel: Sel, is_required: bool,
            is_instance_method: bool)
            where Args: EncodeArguments,
                  Ret: Encode {
        let encs = Args::encodings();
        let encs = encs.as_ref();
        let sel_args = count_args(sel);
        assert!(sel_args == encs.len(),
            "Selector accepts {} arguments, but function accepts {}",
            sel_args, encs.len(),
        );
        let types = method_type_encoding(&Ret::encode(), encs);
        unsafe {
            runtime::protocol_addMethodDescription(
                self.proto, sel, types.as_ptr(), is_required as BOOL, is_instance_method as BOOL);
        }
    }

    /// Adds an instance method declaration with a given description to self.
    pub fn add_method_description<Args, Ret>(&mut self, sel: Sel, is_required: bool)
            where Args: EncodeArguments,
                  Ret: Encode {
        self.add_method_description_common::<Args, Ret>(sel, is_required, true)
    }

    /// Adds a class method declaration with a given description to self.
    pub fn add_class_method_description<Args, Ret>(&mut self, sel: Sel, is_required: bool)
            where Args: EncodeArguments,
                  Ret: Encode {
        self.add_method_description_common::<Args, Ret>(sel, is_required, false)
    }

    /// Adds a requirement on another protocol.
    pub fn add_protocol(&mut self, proto: &Protocol) {
        unsafe {
            runtime::protocol_addProtocol(self.proto, proto);
        }
    }

    /// Registers self, consuming it and returning a reference to the
    /// newly registered `Protocol`.
    pub fn register(self) -> &'static Protocol {
        unsafe {
            runtime::objc_registerProtocol(self.proto);
            &*self.proto
        }
    }
}

#[cfg(test)]
mod tests {
    use test_utils;

    #[test]
    fn test_custom_class() {
        // Registering the custom class is in test_utils
        let obj = test_utils::custom_object();
        unsafe {
            let _: () = msg_send![obj, setFoo:13u32];
            let result: u32 = msg_send![obj, foo];
            assert!(result == 13);
        }
    }

    #[test]
    fn test_class_method() {
        let cls = test_utils::custom_class();
        unsafe {
            let result: u32 = msg_send![cls, classFoo];
            assert!(result == 7);
        }
    }
}
