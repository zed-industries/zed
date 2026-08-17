macro_rules! get_string {
    // The `expr` designator is used for expressions.
    ($obj:ident, $name: ident) => {{
        let string_ptr: *const NSString = msg_send![$obj, $name];
        if string_ptr.is_null() {
            None
        } else {
            Some((*string_ptr).as_str().to_owned())
        }
    }};
}

pub(crate) use get_string;

macro_rules! declare_ref_type {
    ($name:ident) => {
        declare_ref_type!($name,);
    };
    ($name:ident, $($t:ident),*) => {
        #[derive(Debug)]
        #[repr(C)]
        pub struct $name {
            _priv: [u8; 0],
        }
        unsafe impl objc::Message for $name {}
    };
}

pub(crate) use declare_ref_type;
