// Take a look at the license at the top of the repository in the LICENSE file.

use std::rc::Rc;

use glib::clone;

#[test]
fn clone() {
    let _ = clone!(move || {});
    let fut = clone!(async move {});
    drop(fut);

    let x = 1;
    let _ = clone!(move || {
        println!("foo {x}");
        1
    });

    let x = 1;
    let y = String::from("123");
    let v = Rc::new(1);
    let _ = clone!(
        #[strong]
        v,
        move || {
            println!("foo {x} {y} {v}");
            1
        }
    );

    let v = Rc::new(1);
    let _ = clone!(
        #[strong(rename_to = y)]
        v,
        move || {
            println!("foo {y}");
            1
        }
    );

    let v = Rc::new(1);
    let _ = clone!(
        #[strong(rename_to = y)]
        Rc::strong_count(&v),
        move || {
            println!("foo {y}");
            1
        }
    );

    let v = Rc::new(1);
    let _ = clone!(
        #[strong]
        v,
        move |a: i32, b: &str| {
            println!("foo {a} {b} {v}");
            1
        }
    );

    let x = 1;
    let y = String::from("123");
    let v = Rc::new(1);
    let fut = clone!(
        #[strong]
        v,
        async move {
            println!("foo {x} {y} {v}");
            1
        }
    );
    drop(fut);

    let v = Rc::new(1);
    let _ = clone!(
        #[weak]
        v,
        move || {
            println!("foo {v}");
        }
    );

    let v = Rc::new(1);
    let _ = clone!(
        #[weak]
        v,
        #[upgrade_or_else]
        || None::<i32>,
        move || {
            println!("foo {v}");
            Some(1)
        }
    );

    let v = Rc::new(1);
    let _ = clone!(
        #[weak]
        v,
        #[upgrade_or]
        None::<i32>,
        move || {
            println!("foo {v}");
            Some(1)
        }
    );

    let v = Rc::new(1);
    let _ = clone!(
        #[weak]
        v,
        #[upgrade_or_default]
        move || {
            println!("foo {v}");
            Some(1)
        }
    );

    let v = Rc::new(1);
    let w = Rc::new(2);
    let x = Rc::new(3);
    let _ = clone!(
        #[weak]
        v,
        #[weak]
        w,
        #[upgrade_or_else]
        || {
            let x: Rc<i32> = x;
            Some(*x)
        },
        move || {
            println!("foo {v} {w}");
            Some(1)
        }
    );

    let v = Rc::new(1);
    let _ = clone!(
        #[weak]
        v,
        #[upgrade_or_panic]
        move || {
            println!("foo {v}");
            Some(1)
        }
    );

    let v = Rc::new(1);
    let _ = clone!(
        #[weak_allow_none]
        v,
        move || {
            println!("foo {}", v.unwrap());
            Some(1)
        }
    );

    let v = "123";
    let _ = clone!(
        #[to_owned]
        v,
        move || {
            println!("foo {v}");
            1
        }
    );
}

const TESTS: &[(&str, &str)] = &[
    ("clone!()", "expected a closure or async block"),
    (
        "clone!(#[weak] a, #[weak] b, |x| {})",
        r#"error: closures need to capture variables by move. Please add the `move` keyword
 --> test_1.rs:1:88
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[weak] a, #[weak] b, |x| {}); }
  |                                                                                        ^^^^^^"#,
    ),
    (
        "clone!(#[strong] self, move |x| {})",
        r#"error: capture attribute for `self` requires usage of the `rename_to` attribute property
 --> test_2.rs:1:66
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[strong] self, move |x| {}); }
  |                                                                  ^^^^^^^^^"#,
    ),
    (
        "clone!(#[strong] self.v, move |x| {})",
        r#"error: capture attribute for an expression requires usage of the `rename_to` attribute property
 --> test_3.rs:1:66
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[strong] self.v, move |x| {}); }
  |                                                                  ^^^^^^^^^"#,
    ),
    (
        "clone!(#[strong(rename_to = x, rename_to = y)] self.v, move || {})",
        r#"error: multiple `rename_to` properties are not allowed
 --> test_4.rs:1:90
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[strong(rename_to = x, rename_to = y)] self.v, move || {}); }
  |                                                                                          ^^^^^^^^^^^^^"#,
    ),
    (
        "clone!(#[strong(stronk)] self.v, move || {})",
        r#"error: unsupported capture attribute property `stronk`: only `rename_to` is supported
 --> test_5.rs:1:75
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[strong(stronk)] self.v, move || {}); }
  |                                                                           ^^^^^^"#,
    ),
    (
        "clone!(#[strong(rename_to = \"a\")] self.v, move || {})",
        r#"error: expected identifier
 --> test_6.rs:1:87
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[strong(rename_to = "a")] self.v, move || {}); }
  |                                                                                       ^^^"#,
    ),
    (
        "clone!(#[weak] v, #[upgrade_or_else] false, move || {})",
        r#"error: expected `|`
 --> test_7.rs:1:96
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[weak] v, #[upgrade_or_else] false, move || {}); }
  |                                                                                                ^^^^^"#,
    ),
    (
        "clone!(#[weak] v, #[upgrade_or(abort)] move || {})",
        r#"error: unexpected token in attribute
 --> test_8.rs:1:89
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[weak] v, #[upgrade_or(abort)] move || {}); }
  |                                                                                         ^"#,
    ),
    (
        "clone!(#[yolo] v, move || {})",
        r#"error: unsupported attribute `yolo`: only `strong`, `weak`, `weak_allow_none`, `to_owned`, `upgrade_or`, `upgrade_or_else`, `upgrade_or_default` and `upgrade_or_panic` are supported
 --> test_9.rs:1:66
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[yolo] v, move || {}); }
  |                                                                  ^^^^^^^"#,
    ),
    (
        "clone!(#[watch] v, move || {})",
        r#"error: watch variable captures are not supported
 --> test_10.rs:1:66
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[watch] v, move || {}); }
  |                                                                  ^^^^^^^^"#,
    ),
    (
        "clone!(#[strong]#[strong] v, move || {})",
        r#"error: variable capture attributes must be followed by an identifier
 --> test_11.rs:1:75
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[strong]#[strong] v, move || {}); }
  |                                                                           ^^^^^^^^^"#,
    ),
    (
        "clone!(v, move || {})",
        r#"error: only closures and async blocks are supported
 --> test_12.rs:1:66
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(v, move || {}); }
  |                                                                  ^"#,
    ),
    (
        "clone!(#[upgrade_or_else] || lol, #[strong] v, move || {println!(\"foo\");})",
        r#"error: upgrade failure attribute must not be followed by any other attributes. Found 1 more attribute
 --> test_13.rs:1:93
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[upgrade_or_else] || lol, #[strong] v, move || {println!("foo");}); }
  |                                                                                             ^^^^^^^^^"#,
    ),
    (
        "clone!(#[upgrade_or_else] |x| lol, #[strong] v, move || {println!(\"foo\");})",
        r#"error: `upgrade_or_else` closure must not have any parameters
 --> test_14.rs:1:85
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[upgrade_or_else] |x| lol, #[strong] v, move || {println!("foo");}); }
  |                                                                                     ^^^^^^^"#,
    ),
    (
        "clone!(#[upgrade_or_else] async || lol, #[strong] v, move || {println!(\"foo\");})",
        r#"error: `upgrade_or_else` closure needs to be a non-async closure
 --> test_15.rs:1:85
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[upgrade_or_else] async || lol, #[strong] v, move || {println!("foo");}...
  |                                                                                     ^^^^^^^^^^^^"#,
    ),
    (
        "clone!(#[upgrade_or_panic] #[strong] v, move || {println!(\"foo\");})",
        r#"error: upgrade failure attribute must not be followed by any other attributes. Found 1 more attribute
 --> test_16.rs:1:86
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[upgrade_or_panic] #[strong] v, move || {println!("foo");}); }
  |                                                                                      ^^^^^^^^^"#,
    ),
    (
        "clone!(#[strong] v, #[upgrade_or_panic] move || {println!(\"foo\");})",
        r#"error: upgrade failure attribute can only be used together with weak variable captures
 --> test_17.rs:1:79
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[strong] v, #[upgrade_or_panic] move || {println!("foo");}); }
  |                                                                               ^"#,
    ),
    // The async part!
    (
        "clone!(#[strong] v, async {println!(\"foo\");})",
        r#"error: async blocks need to capture variables by move. Please add the `move` keyword
 --> test_18.rs:1:79
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[strong] v, async {println!("foo");}); }
  |                                                                               ^^^^^^^^^^^^^^^^^^^^^^^^"#,
    ),
    (
        "clone!(#[strong] v, {println!(\"foo\");})",
        r#"error: only closures and async blocks are supported
 --> test_19.rs:1:79
  |
1 | fn main() { use glib::clone; let v = std::rc::Rc::new(1); clone!(#[strong] v, {println!("foo");}); }
  |                                                                               ^^^^^^^^^^^^^^^^^^"#,
    ),
];

#[test]
fn clone_failures() {
    let t = trybuild2::TestCases::new();

    for (index, (expr, err)) in TESTS.iter().enumerate() {
        let prefix = "fn main() { use glib::clone; let v = std::rc::Rc::new(1); ";
        let suffix = "; }";
        let output = format!("{prefix}{expr}{suffix}");

        t.compile_fail_inline_check_sub(&format!("test_{index}"), &output, err);
    }
}

const NO_WARNING: &[&str] = &[
    "let _ = clone!(#[weak] v, #[upgrade_or] (), move || println!(\"{}\", v))",
    "let _ = clone!(#[weak] v, #[upgrade_or_else] || (), move || println!(\"{}\", v))",
    "let _ = clone!(#[weak] v, #[upgrade_or_else] || (()), move || println!(\"{}\", v))",
    "let _ = clone!(#[weak] v, #[upgrade_or_else] || ( () ), move || println!(\"{}\", v))",
    "let _ = clone!(#[weak] v, #[upgrade_or_else] || (  ), move || println!(\"{}\", v))",
];

// Ensures that no warning are emitted if the return value is a unit tuple.
#[test]
fn clone_unit_tuple_return() {
    let t = trybuild2::TestCases::new();

    for (index, expr) in NO_WARNING.iter().enumerate() {
        let prefix = "fn main() { use glib::clone; let v = std::rc::Rc::new(1); ";
        let suffix = "; }";
        let output = format!("{prefix}{expr}{suffix}");

        t.pass_inline(&format!("test_{index}"), &output);
    }
}
