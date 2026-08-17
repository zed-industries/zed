use runtime::Imp;

extern {
    fn objc_msgSend();

    fn objc_msgSendSuper();
}

pub fn msg_send_fn<R>() -> Imp {
    // stret is not even available in arm64.
    // <https://twitter.com/gparker/status/378079715824660480>

    objc_msgSend
}

pub fn msg_send_super_fn<R>() -> Imp {
    objc_msgSendSuper
}
