// Tests for the `owned_string_into_shared` lint.

#![allow(unused)]

use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    // --- Should warn ---

    // String::from(<literal>).into() into Arc<str>.
    let _a: Arc<str> = String::from("hello").into();

    // String::from(<literal>).into() into Rc<str>.
    let _b: Rc<str> = String::from("world").into();

    // String::from(<literal>).into() into Cow<'_, str>.
    let _c: Cow<'_, str> = String::from("borrowed-or-owned").into();

    // <literal>.to_string().into() into Arc<str>.
    let _d: Arc<str> = "via-to-string".to_string().into();

    // <literal>.to_owned().into() into Arc<str>.
    let _e: Arc<str> = "via-to-owned".to_owned().into();

    // <literal>.to_string().into() into Rc<str>.
    let _f: Rc<str> = "rc-via-to-string".to_string().into();

    // <literal>.to_owned().into() into Cow<'_, str>.
    let _g: Cow<'_, str> = "cow-via-to-owned".to_owned().into();

    // Long literal still flagged the same way.
    let _h: Arc<str> =
        String::from("this literal is definitely longer than twenty three bytes").into();

    // --- Should NOT warn ---

    // Direct construction from the literal — already optimal.
    let _ok1: Arc<str> = Arc::from("hello");
    let _ok2: Rc<str> = Rc::from("world");
    let _ok3: Cow<'_, str> = Cow::Borrowed("borrowed");

    // Producing a plain `String` (not a refcounted destination).
    let _ok4: String = String::from("not refcounted");
    let _ok5: String = "x".to_string();
    let _ok6: String = "x".to_owned();

    // `.into()` from a non-literal `String` — the allocation is unavoidable.
    let dynamic: String = make_string();
    let _ok7: Arc<str> = dynamic.into();

    // `.into()` from a `&str` directly (no owned `String` in between).
    let _ok8: Arc<str> = "direct".into();

    // `.into()` whose destination is not one of the targeted types.
    let _ok9: Box<str> = String::from("box-str").into();

    // `String::new()` is not built from a literal.
    let _ok10: Arc<str> = String::new().into();

    // Method call that is not `into`.
    let _ok11: String = String::from("foo").clone();
}

fn make_string() -> String {
    String::from("dynamic")
}
