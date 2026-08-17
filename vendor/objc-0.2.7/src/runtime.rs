//! A Rust interface for the functionality of the Objective-C runtime.
//!
//! For more information on foreign functions, see Apple's documentation:
//! <https://developer.apple.com/library/mac/documentation/Cocoa/Reference/ObjCRuntimeRef/index.html>

use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;
use std::str;
use malloc_buf::MallocBuffer;

use encode;
use {Encode, Encoding};

/// The Objective-C `BOOL` type.
///
/// To convert an Objective-C `BOOL` into a Rust `bool`, compare it with `NO`.
#[cfg(not(target_arch = "aarch64"))]
pub type BOOL = ::std::os::raw::c_schar;
/// The equivalent of true for Objective-C's `BOOL` type.
#[cfg(not(target_arch = "aarch64"))]
pub const YES: BOOL = 1;
/// The equivalent of false for Objective-C's `BOOL` type.
#[cfg(not(target_arch = "aarch64"))]
pub const NO: BOOL = 0;

#[cfg(target_arch = "aarch64")]
pub type BOOL = bool;
#[cfg(target_arch = "aarch64")]
pub const YES: BOOL = true;
#[cfg(target_arch = "aarch64")]
pub const NO: BOOL = false;

/// A type that represents a method selector.
#[repr(C)]
pub struct Sel {
    ptr: *const c_void,
}

/// A marker type to be embedded into other types just so that they cannot be
/// constructed externally.
type PrivateMarker = [u8; 0];

/// A type that represents an instance variable.
#[repr(C)]
pub struct Ivar {
    _priv: PrivateMarker,
}

/// A type that represents a method in a class definition.
#[repr(C)]
pub struct Method {
    _priv: PrivateMarker,
}

/// A type that represents an Objective-C class.
#[repr(C)]
pub struct Class {
    _priv: PrivateMarker,
}

/// A type that represents an Objective-C protocol.
#[repr(C)]
pub struct Protocol {
    _priv: PrivateMarker
}

/// A type that represents an instance of a class.
#[repr(C)]
pub struct Object {
    _priv: PrivateMarker,
}

/// A pointer to the start of a method implementation.
pub type Imp = unsafe extern fn();

#[link(name = "objc", kind = "dylib")]
extern {
    pub fn sel_registerName(name: *const c_char) -> Sel;
    pub fn sel_getName(sel: Sel) -> *const c_char;

    pub fn class_getName(cls: *const Class) -> *const c_char;
    pub fn class_getSuperclass(cls: *const Class) -> *const Class;
    pub fn class_getInstanceSize(cls: *const Class) -> usize;
    pub fn class_getInstanceMethod(cls: *const Class, sel: Sel) -> *const Method;
    pub fn class_getInstanceVariable(cls: *const Class, name: *const c_char) -> *const Ivar;
    pub fn class_copyMethodList(cls: *const Class, outCount: *mut c_uint) -> *mut *const Method;
    pub fn class_copyIvarList(cls: *const Class, outCount: *mut c_uint) -> *mut *const Ivar;
    pub fn class_addMethod(cls: *mut Class, name: Sel, imp: Imp, types: *const c_char) -> BOOL;
    pub fn class_addIvar(cls: *mut Class, name: *const c_char, size: usize, alignment: u8, types: *const c_char) -> BOOL;
    pub fn class_addProtocol(cls: *mut Class, proto: *const Protocol) -> BOOL;
    pub fn class_conformsToProtocol(cls: *const Class, proto: *const Protocol) -> BOOL;
    pub fn class_copyProtocolList(cls: *const Class, outCount: *mut c_uint) -> *mut *const Protocol;

    pub fn objc_allocateClassPair(superclass: *const Class, name: *const c_char, extraBytes: usize) -> *mut Class;
    pub fn objc_disposeClassPair(cls: *mut Class);
    pub fn objc_registerClassPair(cls: *mut Class);

    pub fn class_createInstance(cls: *const Class, extraBytes: usize) -> *mut Object;
    pub fn object_dispose(obj: *mut Object) -> *mut Object;
    pub fn object_getClass(obj: *const Object) -> *const Class;

    pub fn objc_getClassList(buffer: *mut *const Class, bufferLen: c_int) -> c_int;
    pub fn objc_copyClassList(outCount: *mut c_uint) -> *mut *const Class;
    pub fn objc_getClass(name: *const c_char) -> *const Class;
    pub fn objc_getProtocol(name: *const c_char) -> *const Protocol;
    pub fn objc_copyProtocolList(outCount: *mut c_uint) -> *mut *const Protocol;
    pub fn objc_allocateProtocol(name: *const c_char) -> *mut Protocol;
    pub fn objc_registerProtocol(proto: *mut Protocol);

    pub fn objc_autoreleasePoolPush() -> *mut c_void;
    pub fn objc_autoreleasePoolPop(context: *mut c_void);

    pub fn protocol_addMethodDescription(proto: *mut Protocol, name: Sel, types: *const c_char, isRequiredMethod: BOOL,
                                         isInstanceMethod: BOOL);
    pub fn protocol_addProtocol(proto: *mut Protocol, addition: *const Protocol);
    pub fn protocol_getName(proto: *const Protocol) -> *const c_char;
    pub fn protocol_isEqual(proto: *const Protocol, other: *const Protocol) -> BOOL;
    pub fn protocol_copyProtocolList(proto: *const Protocol, outCount: *mut c_uint) -> *mut *const Protocol;
    pub fn protocol_conformsToProtocol(proto: *const Protocol, other: *const Protocol) -> BOOL;

    pub fn ivar_getName(ivar: *const Ivar) -> *const c_char;
    pub fn ivar_getOffset(ivar: *const Ivar) -> isize;
    pub fn ivar_getTypeEncoding(ivar: *const Ivar) -> *const c_char;

    pub fn method_getName(method: *const Method) -> Sel;
    pub fn method_getImplementation(method: *const Method) -> Imp;
    pub fn method_copyReturnType(method: *const Method) -> *mut c_char;
    pub fn method_copyArgumentType(method: *const Method, index: c_uint) -> *mut c_char;
    pub fn method_getNumberOfArguments(method: *const Method) -> c_uint;
    pub fn method_setImplementation(method: *mut Method, imp: Imp) -> Imp;
    pub fn method_exchangeImplementations(m1: *mut Method, m2: *mut Method);

    pub fn objc_retain(obj: *mut Object) -> *mut Object;
    pub fn objc_release(obj: *mut Object);
    pub fn objc_autorelease(obj: *mut Object);

    pub fn objc_loadWeakRetained(location: *mut *mut Object) -> *mut Object;
    pub fn objc_initWeak(location: *mut *mut Object, obj: *mut Object) -> *mut Object;
    pub fn objc_destroyWeak(location: *mut *mut Object);
    pub fn objc_copyWeak(to: *mut *mut Object, from: *mut *mut Object);
}

impl Sel {
    /// Registers a method with the Objective-C runtime system,
    /// maps the method name to a selector, and returns the selector value.
    pub fn register(name: &str) -> Sel {
        let name = CString::new(name).unwrap();
        unsafe {
            sel_registerName(name.as_ptr())
        }
    }

    /// Returns the name of the method specified by self.
    pub fn name(&self) -> &str {
        let name = unsafe {
            CStr::from_ptr(sel_getName(*self))
        };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// Wraps a raw pointer to a selector into a `Sel` object.
    ///
    /// This is almost never what you want; use `Sel::register()` instead.
    #[inline]
    pub unsafe fn from_ptr(ptr: *const c_void) -> Sel {
        Sel {
            ptr: ptr,
        }
    }

    /// Returns a pointer to the raw selector.
    #[inline]
    pub fn as_ptr(&self) -> *const c_void {
        self.ptr
    }
}

impl PartialEq for Sel {
    fn eq(&self, other: &Sel) -> bool {
        self.ptr == other.ptr
    }
}

impl Eq for Sel { }

// Sel is safe to share across threads because it is immutable
unsafe impl Sync for Sel { }
unsafe impl Send for Sel { }

impl Copy for Sel { }

impl Clone for Sel {
    fn clone(&self) -> Sel { *self }
}

impl fmt::Debug for Sel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Ivar {
    /// Returns the name of self.
    pub fn name(&self) -> &str {
        let name = unsafe {
            CStr::from_ptr(ivar_getName(self))
        };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// Returns the offset of self.
    pub fn offset(&self) -> isize {
        let offset = unsafe {
            ivar_getOffset(self)
        };
        offset as isize
    }

    /// Returns the `Encoding` of self.
    pub fn type_encoding(&self) -> Encoding {
        let encoding = unsafe {
            CStr::from_ptr(ivar_getTypeEncoding(self))
        };
        let s = str::from_utf8(encoding.to_bytes()).unwrap();
        encode::from_str(s)
    }
}

impl Method {
    /// Returns the name of self.
    pub fn name(&self) -> Sel {
        unsafe {
            method_getName(self)
        }
    }

    /// Returns the `Encoding` of self's return type.
    pub fn return_type(&self) -> Encoding {
        unsafe {
            let encoding = method_copyReturnType(self);
            encode::from_malloc_str(encoding)
        }
    }

    /// Returns the `Encoding` of a single parameter type of self, or
    /// `None` if self has no parameter at the given index.
    pub fn argument_type(&self, index: usize) -> Option<Encoding> {
        unsafe {
            let encoding = method_copyArgumentType(self, index as c_uint);
            if encoding.is_null() {
                None
            } else {
                Some(encode::from_malloc_str(encoding))
            }
        }
    }

    /// Returns the number of arguments accepted by self.
    pub fn arguments_count(&self) -> usize {
        unsafe {
            method_getNumberOfArguments(self) as usize
        }
    }

    /// Returns the implementation of self.
    pub fn implementation(&self) -> Imp {
        unsafe {
            method_getImplementation(self)
        }
    }
}

impl Class {
    /// Returns the class definition of a specified class, or `None` if the
    /// class is not registered with the Objective-C runtime.
    pub fn get(name: &str) -> Option<&'static Class> {
        let name = CString::new(name).unwrap();
        unsafe {
            let cls = objc_getClass(name.as_ptr());
            if cls.is_null() { None } else { Some(&*cls) }
        }
    }

    /// Obtains the list of registered class definitions.
    pub fn classes() -> MallocBuffer<&'static Class> {
        unsafe {
            let mut count: c_uint = 0;
            let classes = objc_copyClassList(&mut count);
            MallocBuffer::new(classes as *mut _, count as usize).unwrap()
        }
    }

    /// Returns the total number of registered classes.
    pub fn classes_count() -> usize {
        unsafe {
            objc_getClassList(ptr::null_mut(), 0) as usize
        }
    }

    /// Returns the name of self.
    pub fn name(&self) -> &str {
        let name = unsafe {
            CStr::from_ptr(class_getName(self))
        };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// Returns the superclass of self, or `None` if self is a root class.
    pub fn superclass(&self) -> Option<&Class> {
        unsafe {
            let superclass = class_getSuperclass(self);
            if superclass.is_null() { None } else { Some(&*superclass) }
        }
    }

    /// Returns the metaclass of self.
    pub fn metaclass(&self) -> &Class {
        unsafe {
            let self_ptr: *const Class = self;
            &*object_getClass(self_ptr as *const Object)
        }
    }

    /// Returns the size of instances of self.
    pub fn instance_size(&self) -> usize {
        unsafe {
            class_getInstanceSize(self) as usize
        }
    }

    /// Returns a specified instance method for self, or `None` if self and
    /// its superclasses do not contain an instance method with the
    /// specified selector.
    pub fn instance_method(&self, sel: Sel) -> Option<&Method> {
        unsafe {
            let method = class_getInstanceMethod(self, sel);
            if method.is_null() { None } else { Some(&*method) }
        }
    }

    /// Returns the ivar for a specified instance variable of self, or `None`
    /// if self has no ivar with the given name.
    pub fn instance_variable(&self, name: &str) -> Option<&Ivar> {
        let name = CString::new(name).unwrap();
        unsafe {
            let ivar = class_getInstanceVariable(self, name.as_ptr());
            if ivar.is_null() { None } else { Some(&*ivar) }
        }
    }

    /// Describes the instance methods implemented by self.
    pub fn instance_methods(&self) -> MallocBuffer<&Method> {
        unsafe {
            let mut count: c_uint = 0;
            let methods = class_copyMethodList(self, &mut count);
            MallocBuffer::new(methods as *mut _, count as usize).unwrap()
        }

    }

    /// Checks whether this class conforms to the specified protocol.
    pub fn conforms_to(&self, proto: &Protocol) -> bool {
        unsafe { class_conformsToProtocol(self, proto) == YES }
    }

    /// Get a list of the protocols to which this class conforms.
    pub fn adopted_protocols(&self) -> MallocBuffer<&Protocol> {
        unsafe {
            let mut count: c_uint = 0;
            let protos = class_copyProtocolList(self, &mut count);
            MallocBuffer::new(protos as *mut _, count as usize).unwrap()
        }
    }

    /// Describes the instance variables declared by self.
    pub fn instance_variables(&self) -> MallocBuffer<&Ivar> {
        unsafe {
            let mut count: c_uint = 0;
            let ivars = class_copyIvarList(self, &mut count);
            MallocBuffer::new(ivars as *mut _, count as usize).unwrap()
        }
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Class) -> bool {
        let self_ptr: *const Class = self;
        let other_ptr: *const Class = other;
        self_ptr == other_ptr
    }
}

impl Eq for Class { }

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Protocol {
    /// Returns the protocol definition of a specified protocol, or `None` if the
    /// protocol is not registered with the Objective-C runtime.
    pub fn get(name: &str) -> Option<&'static Protocol> {
        let name = CString::new(name).unwrap();
        unsafe {
            let proto = objc_getProtocol(name.as_ptr());
            if proto.is_null() { None } else { Some(&*proto) }
        }
    }

    /// Obtains the list of registered protocol definitions.
    pub fn protocols() -> MallocBuffer<&'static Protocol> {
        unsafe {
            let mut count: c_uint = 0;
            let protocols = objc_copyProtocolList(&mut count);
            MallocBuffer::new(protocols as *mut _, count as usize).unwrap()
        }
    }

    /// Get a list of the protocols to which this protocol conforms.
    pub fn adopted_protocols(&self) -> MallocBuffer<&Protocol> {
        unsafe {
            let mut count: c_uint = 0;
            let protocols = protocol_copyProtocolList(self, &mut count);
            MallocBuffer::new(protocols as *mut _, count as usize).unwrap()
        }
    }

    /// Checks whether this protocol conforms to the specified protocol.
    pub fn conforms_to(&self, proto: &Protocol) -> bool {
        unsafe { protocol_conformsToProtocol(self, proto) == YES }
    }

    /// Returns the name of self.
    pub fn name(&self) -> &str {
        let name = unsafe {
            CStr::from_ptr(protocol_getName(self))
        };
        str::from_utf8(name.to_bytes()).unwrap()
    }
}

impl PartialEq for Protocol {
    fn eq(&self, other: &Protocol) -> bool {
        unsafe { protocol_isEqual(self, other) == YES }
    }
}

impl Eq for Protocol { }

impl fmt::Debug for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Object {
    /// Returns the class of self.
    pub fn class(&self) -> &Class {
        unsafe {
            &*object_getClass(self)
        }
    }

    /// Returns a reference to the ivar of self with the given name.
    /// Panics if self has no ivar with the given name.
    /// Unsafe because the caller must ensure that the ivar is actually
    /// of type `T`.
    pub unsafe fn get_ivar<T>(&self, name: &str) -> &T where T: Encode {
        let offset = {
            let cls = self.class();
            match cls.instance_variable(name) {
                Some(ivar) => {
                    assert!(ivar.type_encoding() == T::encode());
                    ivar.offset()
                }
                None => panic!("Ivar {} not found on class {:?}", name, cls),
            }
        };
        let ptr = {
            let self_ptr: *const Object = self;
            (self_ptr as *const u8).offset(offset) as *const T
        };
        &*ptr
    }

    /// Returns a mutable reference to the ivar of self with the given name.
    /// Panics if self has no ivar with the given name.
    /// Unsafe because the caller must ensure that the ivar is actually
    /// of type `T`.
    pub unsafe fn get_mut_ivar<T>(&mut self, name: &str) -> &mut T
            where T: Encode {
        let offset = {
            let cls = self.class();
            match cls.instance_variable(name) {
                Some(ivar) => {
                    assert!(ivar.type_encoding() == T::encode());
                    ivar.offset()
                }
                None => panic!("Ivar {} not found on class {:?}", name, cls),
            }
        };
        let ptr = {
            let self_ptr: *mut Object = self;
            (self_ptr as *mut u8).offset(offset) as *mut T
        };
        &mut *ptr
    }

    /// Sets the value of the ivar of self with the given name.
    /// Panics if self has no ivar with the given name.
    /// Unsafe because the caller must ensure that the ivar is actually
    /// of type `T`.
    pub unsafe fn set_ivar<T>(&mut self, name: &str, value: T)
            where T: Encode {
        *self.get_mut_ivar::<T>(name) = value;
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{:?}: {:p}>", self.class(), self)
    }
}

#[cfg(test)]
mod tests {
    use test_utils;
    use Encode;
    use super::{Class, Protocol, Sel};

    #[test]
    fn test_ivar() {
        let cls = test_utils::custom_class();
        let ivar = cls.instance_variable("_foo").unwrap();
        assert!(ivar.name() == "_foo");
        assert!(ivar.type_encoding() == <u32>::encode());
        assert!(ivar.offset() > 0);

        let ivars = cls.instance_variables();
        assert!(ivars.len() > 0);
    }

    #[test]
    fn test_method() {
        let cls = test_utils::custom_class();
        let sel = Sel::register("foo");
        let method = cls.instance_method(sel).unwrap();
        assert!(method.name().name() == "foo");
        assert!(method.arguments_count() == 2);
        assert!(method.return_type() == <u32>::encode());
        assert!(method.argument_type(1).unwrap() == Sel::encode());

        let methods = cls.instance_methods();
        assert!(methods.len() > 0);
    }

    #[test]
    fn test_class() {
        let cls = test_utils::custom_class();
        assert!(cls.name() == "CustomObject");
        assert!(cls.instance_size() > 0);
        assert!(cls.superclass().is_none());

        assert!(Class::get(cls.name()) == Some(cls));

        let metaclass = cls.metaclass();
        // The metaclass of a root class is a subclass of the root class
        assert!(metaclass.superclass().unwrap() == cls);

        let subclass = test_utils::custom_subclass();
        assert!(subclass.superclass().unwrap() == cls);
    }

    #[test]
    fn test_classes() {
        assert!(Class::classes_count() > 0);
        let classes = Class::classes();
        assert!(classes.len() > 0);
    }

    #[test]
    fn test_protocol() {
        let proto = test_utils::custom_protocol();
        assert!(proto.name() == "CustomProtocol");
        let class = test_utils::custom_class();
        assert!(class.conforms_to(proto));
        let class_protocols = class.adopted_protocols();
        assert!(class_protocols.len() > 0);
    }

    #[test]
    fn test_protocol_method() {
        let class = test_utils::custom_class();
        let result: i32 = unsafe {
            msg_send![class, addNumber:1 toNumber:2]
        };
        assert_eq!(result, 3);
    }

    #[test]
    fn test_subprotocols() {
        let sub_proto = test_utils::custom_subprotocol();
        let super_proto = test_utils::custom_protocol();
        assert!(sub_proto.conforms_to(super_proto));
        let adopted_protocols = sub_proto.adopted_protocols();
        assert_eq!(adopted_protocols[0], super_proto);
    }

    #[test]
    fn test_protocols() {
        // Ensure that a protocol has been registered on linux
        let _ = test_utils::custom_protocol();

        let protocols = Protocol::protocols();
        assert!(protocols.len() > 0);
    }

    #[test]
    fn test_object() {
        let mut obj = test_utils::custom_object();
        assert!(obj.class() == test_utils::custom_class());
        let result: u32 = unsafe {
            obj.set_ivar("_foo", 4u32);
            *obj.get_ivar("_foo")
        };
        assert!(result == 4);
    }
}
