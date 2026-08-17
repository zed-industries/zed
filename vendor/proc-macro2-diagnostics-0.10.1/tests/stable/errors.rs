use diagnostic_example::{diagnostic_item, diagnostic_expr};

fn main() {
    diagnostic_item! {
        error: just an item error,
        _note: see an item,
        help: I can help with that,
    }

    diagnostic_expr! {
        error: just an expression error item context,
        _note: see an exp,
        _warning: kind of weird,
        help: I can help with that,
    }

    let x = diagnostic_expr! {
        error: just an expression error expr content,
        _warning: oh no,
    };
}

diagnostic_item! {
    error: this is an item error message,
}

diagnostic_item! {
    error: this is an error message,
    _note: but it has a note,
}

diagnostic_item! {
    error: hello,
    warning: just wanted to warn you,
    note: just jotting things down,
    _help: please help on that note,
    _note: notes on notes,
}

diagnostic_item! {
    warning: hello not found,
    _note: just a note to say hi,
    _help: its a good idea to say hello,
}

diagnostic_item! {
    hello: hello,
}

diagnostic_item! {
    help: this is standalone help,
    _note: but we add things to it,
    _help: like more help,
}
