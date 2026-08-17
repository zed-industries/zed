// Take a look at the license at the top of the repository in the LICENSE file.

const TESTS: &[(&str, &str)] = &[
    ("closure!()", "expected a closure"),
    (
        "closure!(#[weak] a, #[weak] b, |x| {})",
        r#"error: closures need to capture variables by move. Please add the `move` keyword
 --> test_1.rs:1:92
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[weak] a, #[weak] b, |x| {}); }
  |                                                                                            ^^^^^^"#,
    ),
    (
        "closure!(#[strong] self, move |x| {})",
        r#"error: capture attribute for `self` requires usage of the `rename_to` attribute property
 --> test_2.rs:1:70
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[strong] self, move |x| {}); }
  |                                                                      ^^^^^^^^^"#,
    ),
    (
        "closure!(#[strong] self.v, move |x| {})",
        r#"error: capture attribute for an expression requires usage of the `rename_to` attribute property
 --> test_3.rs:1:70
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[strong] self.v, move |x| {}); }
  |                                                                      ^^^^^^^^^"#,
    ),
    (
        "closure!(#[strong(rename_to = x, rename_to = y)] self.v, move || {})",
        r#"error: multiple `rename_to` properties are not allowed
 --> test_4.rs:1:94
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[strong(rename_to = x, rename_to = y)] self.v, move || {}); }
  |                                                                                              ^^^^^^^^^^^^^"#,
    ),
    (
        "closure!(#[strong(stronk)] self.v, move || {})",
        r#"error: unsupported capture attribute property `stronk`: only `rename_to` is supported
 --> test_5.rs:1:79
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[strong(stronk)] self.v, move || {}); }
  |                                                                               ^^^^^^"#,
    ),
    (
        "closure!(#[strong(rename_to = \"a\")] self.v, move || {})",
        r#"error: expected identifier
 --> test_6.rs:1:91
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[strong(rename_to = "a")] self.v, move || {}); }
  |                                                                                           ^^^"#,
    ),
    (
        "closure!(#[weak] v, #[upgrade_or_else] false, move || {})",
        r#"error: expected `|`
 --> test_7.rs:1:100
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[weak] v, #[upgrade_or_else] false, move || {}); }
  |                                                                                                    ^^^^^"#,
    ),
    (
        "closure!(#[weak] v, #[upgrade_or(abort)] move || {})",
        r#"error: unexpected token in attribute
 --> test_8.rs:1:93
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[weak] v, #[upgrade_or(abort)] move || {}); }
  |                                                                                             ^"#,
    ),
    (
        "closure!(#[yolo] v, move || {})",
        r#"error: unsupported attribute `yolo`: only `watch`, `strong`, `weak`, `weak_allow_none`, `to_owned`, `upgrade_or`, `upgrade_or_else`, `upgrade_or_default` and `upgrade_or_panic` are supported
 --> test_9.rs:1:70
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[yolo] v, move || {}); }
  |                                                                      ^^^^^^^"#,
    ),
    (
        "closure!(#[watch] v, #[watch] v, move || {})",
        r#"error: only one `watch` capture is allowed per closure
 --> test_10.rs:1:82
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[watch] v, #[watch] v, move || {}); }
  |                                                                                  ^^^^^^^^"#,
    ),
    (
        "closure!(#[strong]#[strong] v, move || {})",
        r#"error: variable capture attributes must be followed by an identifier
 --> test_11.rs:1:79
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[strong]#[strong] v, move || {}); }
  |                                                                               ^^^^^^^^^"#,
    ),
    (
        "closure!(v, move || {})",
        r#"error: expected `|`
 --> test_12.rs:1:70
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(v, move || {}); }
  |                                                                      ^"#,
    ),
    (
        "closure!(#[upgrade_or_else] || lol, #[strong] v, move || {println!(\"foo\");})",
        r#"error: upgrade failure attribute must not be followed by any other attributes. Found 1 more attribute
 --> test_13.rs:1:97
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[upgrade_or_else] || lol, #[strong] v, move || {println!("foo");}); }
  |                                                                                                 ^^^^^^^^^"#,
    ),
    (
        "closure!(#[upgrade_or_else] |x| lol, #[strong] v, move || {println!(\"foo\");})",
        r#"error: `upgrade_or_else` closure must not have any parameters
 --> test_14.rs:1:89
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[upgrade_or_else] |x| lol, #[strong] v, move || {println!("foo");}); }
  |                                                                                         ^^^^^^^"#,
    ),
    (
        "closure!(#[upgrade_or_else] async || lol, #[strong] v, move || {println!(\"foo\");})",
        r#"error: `upgrade_or_else` closure needs to be a non-async closure
 --> test_15.rs:1:89
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[upgrade_or_else] async || lol, #[strong] v, move || {println!("foo...
  |                                                                                         ^^^^^^^^^^^^"#,
    ),
    (
        "closure!(#[upgrade_or_panic] #[strong] v, move || {println!(\"foo\");})",
        r#"error: upgrade failure attribute must not be followed by any other attributes. Found 1 more attribute
 --> test_16.rs:1:90
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[upgrade_or_panic] #[strong] v, move || {println!("foo");}); }
  |                                                                                          ^^^^^^^^^"#,
    ),
    (
        "closure!(#[strong] v, async {println!(\"foo\");})",
        r#"error: expected `|`
 --> test_17.rs:1:89
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[strong] v, async {println!("foo");}); }
  |                                                                                         ^"#,
    ),
    (
        "closure!(#[strong] v, {println!(\"foo\");})",
        r#"error: expected `|`
 --> test_18.rs:1:83
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[strong] v, {println!("foo");}); }
  |                                                                                   ^"#,
    ),
    (
        "closure!(#[strong] v, #[upgrade_or_panic] move || {println!(\"foo\");})",
        r#"error: upgrade failure attribute can only be used together with weak variable captures
 --> test_19.rs:1:83
  |
1 | fn main() { use glib::closure; let v = std::rc::Rc::new(1); closure!(#[strong] v, #[upgrade_or_panic] move || {println!("foo");}); }
  |                                                                                   ^"#,
    ),
];

#[test]
fn closure_failures() {
    let t = trybuild2::TestCases::new();

    for (index, (expr, err)) in TESTS.iter().enumerate() {
        let prefix = "fn main() { use glib::closure; let v = std::rc::Rc::new(1); ";
        let suffix = "; }";
        let output = format!("{prefix}{expr}{suffix}");

        t.compile_fail_inline_check_sub(&format!("test_{index}"), &output, err);
    }
}
