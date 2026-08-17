use std::ops::{Deref, DerefMut};
use std::os::raw::c_char;
use std::sync::{Once, ONCE_INIT};

use declare::{ClassDecl, ProtocolDecl};
use runtime::{Class, Object, Protocol, Sel, self};
use {Encode, Encoding};

pub struct CustomObject {
    obj: *mut Object,
}

impl CustomObject {
    fn new(class: &Class) -> Self {
        let obj = unsafe {
            runtime::class_createInstance(class, 0)
        };
        CustomObject { obj: obj }
    }
}

impl Deref for CustomObject {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe { &*self.obj }
    }
}

impl DerefMut for CustomObject {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe { &mut *self.obj }
    }
}

impl Drop for CustomObject {
    fn drop(&mut self) {
        unsafe {
            runtime::object_dispose(self.obj);
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct CustomStruct {
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub d: u64,
}

unsafe impl Encode for CustomStruct {
    fn encode() -> Encoding {
        let mut code = "{CustomStruct=".to_owned();
        for _ in 0..4 {
            code.push_str(u64::encode().as_str());
        }
        code.push_str("}");
        unsafe {
            Encoding::from_str(&code)
        }
    }
}

pub fn custom_class() -> &'static Class {
    static REGISTER_CUSTOM_CLASS: Once = ONCE_INIT;

    REGISTER_CUSTOM_CLASS.call_once(|| {
        // The runtime will call this method, so it has to be implemented
        extern fn custom_obj_class_initialize(_this: &Class, _cmd: Sel) { }

        let mut decl = ClassDecl::root("CustomObject", custom_obj_class_initialize).unwrap();
        let proto = custom_protocol();

        decl.add_protocol(proto);
        decl.add_ivar::<u32>("_foo");

        extern fn custom_obj_set_foo(this: &mut Object, _cmd: Sel, foo: u32) {
            unsafe { this.set_ivar::<u32>("_foo", foo); }
        }

        extern fn custom_obj_get_foo(this: &Object, _cmd: Sel) -> u32 {
            unsafe { *this.get_ivar::<u32>("_foo") }
        }

        extern fn custom_obj_get_struct(_this: &Object, _cmd: Sel) -> CustomStruct {
            CustomStruct { a: 1, b: 2, c: 3, d: 4 }
        }

        extern fn custom_obj_class_method(_this: &Class, _cmd: Sel) -> u32 {
            7
        }

        extern fn custom_obj_set_bar(this: &mut Object, _cmd: Sel, bar: u32) {
            unsafe { this.set_ivar::<u32>("_foo", bar) ;}
        }

        extern fn custom_obj_add_number_to_number(_this: &Class, _cmd: Sel, fst: i32, snd: i32) -> i32 {
            fst + snd
        }

        unsafe {
            let set_foo: extern fn(&mut Object, Sel, u32) = custom_obj_set_foo;
            decl.add_method(sel!(setFoo:), set_foo);
            let get_foo: extern fn(&Object, Sel) -> u32 = custom_obj_get_foo;
            decl.add_method(sel!(foo), get_foo);
            let get_struct: extern fn(&Object, Sel) -> CustomStruct = custom_obj_get_struct;
            decl.add_method(sel!(customStruct), get_struct);
            let class_method: extern fn(&Class, Sel) -> u32 = custom_obj_class_method;
            decl.add_class_method(sel!(classFoo), class_method);

            let protocol_instance_method: extern fn(&mut Object, Sel, u32) = custom_obj_set_bar;
            decl.add_method(sel!(setBar:), protocol_instance_method);
            let protocol_class_method: extern fn(&Class, Sel, i32, i32) -> i32 = custom_obj_add_number_to_number;
            decl.add_class_method(sel!(addNumber:toNumber:), protocol_class_method);
        }

        decl.register();
    });

    class!(CustomObject)
}

pub fn custom_protocol() -> &'static Protocol {
    static REGISTER_CUSTOM_PROTOCOL: Once = ONCE_INIT;

    REGISTER_CUSTOM_PROTOCOL.call_once(|| {
        let mut decl = ProtocolDecl::new("CustomProtocol").unwrap();

        decl.add_method_description::<(i32,), ()>(sel!(setBar:), true);
        decl.add_method_description::<(), *const c_char>(sel!(getName), false);
        decl.add_class_method_description::<(i32, i32), i32>(sel!(addNumber:toNumber:), true);

        decl.register();
    });

    Protocol::get("CustomProtocol").unwrap()
}

pub fn custom_subprotocol() -> &'static Protocol {
    static REGISTER_CUSTOM_SUBPROTOCOL: Once = ONCE_INIT;

    REGISTER_CUSTOM_SUBPROTOCOL.call_once(|| {
        let super_proto = custom_protocol();
        let mut decl = ProtocolDecl::new("CustomSubProtocol").unwrap();

        decl.add_protocol(super_proto);
        decl.add_method_description::<(u32,), u32>(sel!(calculateFoo:), true);

        decl.register();
    });

    Protocol::get("CustomSubProtocol").unwrap()
}

pub fn custom_object() -> CustomObject {
    CustomObject::new(custom_class())
}

pub fn custom_subclass() -> &'static Class {
    static REGISTER_CUSTOM_SUBCLASS: Once = ONCE_INIT;

    REGISTER_CUSTOM_SUBCLASS.call_once(|| {
        let superclass = custom_class();
        let mut decl = ClassDecl::new("CustomSubclassObject", superclass).unwrap();

        extern fn custom_subclass_get_foo(this: &Object, _cmd: Sel) -> u32 {
            let foo: u32 = unsafe {
                msg_send![super(this, custom_class()), foo]
            };
            foo + 2
        }

        unsafe {
            let get_foo: extern fn(&Object, Sel) -> u32 = custom_subclass_get_foo;
            decl.add_method(sel!(foo), get_foo);
        }

        decl.register();
    });

    class!(CustomSubclassObject)
}

pub fn custom_subclass_object() -> CustomObject {
    CustomObject::new(custom_subclass())
}
