mod arg {
    use cxx::ExternType;
    use std::marker::{PhantomData, PhantomPinned};

    struct Arg(PhantomPinned);

    unsafe impl ExternType for Arg {
        type Id = cxx::type_id!("Arg");
        type Kind = cxx::kind::Opaque;
    }

    struct ArgLife<'a>(PhantomPinned, PhantomData<&'a ()>);

    unsafe impl<'a> ExternType for ArgLife<'a> {
        type Id = cxx::type_id!("ArgLife");
        type Kind = cxx::kind::Opaque;
    }

    #[cxx::bridge]
    mod ffi {
        unsafe extern "C++" {
            type Arg = crate::arg::Arg;
            fn f(arg: &mut Arg);
        }
    }

    #[cxx::bridge]
    mod ffi_life {
        unsafe extern "C++" {
            type ArgLife<'a> = crate::arg::ArgLife<'a>;
            fn fl<'b, 'c>(arg: &'b mut ArgLife<'c>);
        }
    }
}

mod receiver {
    use cxx::ExternType;
    use std::marker::{PhantomData, PhantomPinned};

    struct Receiver(PhantomPinned);

    unsafe impl ExternType for Receiver {
        type Id = cxx::type_id!("Receiver");
        type Kind = cxx::kind::Opaque;
    }

    struct ReceiverLife<'a>(PhantomPinned, PhantomData<&'a ()>);

    unsafe impl<'a> ExternType for ReceiverLife<'a> {
        type Id = cxx::type_id!("ReceiverLife");
        type Kind = cxx::kind::Opaque;
    }

    #[cxx::bridge]
    mod ffi {
        unsafe extern "C++" {
            type Receiver = crate::receiver::Receiver;
            fn g(&mut self);
        }
    }

    #[cxx::bridge]
    mod ffi_life {
        unsafe extern "C++" {
            type ReceiverLife<'a> = crate::receiver::ReceiverLife<'a>;
            fn g<'b>(&'b mut self);
        }
    }
}

mod receiver2 {
    use cxx::ExternType;
    use std::marker::{PhantomData, PhantomPinned};

    struct Receiver2(PhantomPinned);

    unsafe impl ExternType for Receiver2 {
        type Id = cxx::type_id!("Receiver2");
        type Kind = cxx::kind::Opaque;
    }

    struct ReveiverLife2<'a>(PhantomPinned, PhantomData<&'a ()>);

    unsafe impl<'a> ExternType for ReveiverLife2<'a> {
        type Id = cxx::type_id!("ReveiverLife2");
        type Kind = cxx::kind::Opaque;
    }

    #[cxx::bridge]
    mod ffi {
        unsafe extern "C++" {
            type Receiver2 = crate::receiver2::Receiver2;
            fn h(self: &mut Receiver2);
        }
    }

    #[cxx::bridge]
    mod ffi_life {
        unsafe extern "C++" {
            type ReveiverLife2<'a> = crate::receiver2::ReveiverLife2<'a>;
            fn h<'b, 'c>(self: &'b mut ReveiverLife2<'c>);
        }
    }
}

fn main() {}
