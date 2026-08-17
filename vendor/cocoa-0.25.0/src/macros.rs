// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Creates a Cocoa delegate to use e.g. with `NSWindow.setDelegate_`.
/// Adds instance variables and methods to the definition.
///
/// # Example with NSWindowDelegate
/// ``` no_run
/// #[macro_use] extern crate cocoa;
/// #[macro_use] extern crate objc;
///
/// use cocoa::appkit::NSWindow;
/// use cocoa::base::{id, nil};
///
/// use objc::runtime::{Object, Sel};
///
/// # fn main() {
/// unsafe {
///     let my_window: id = NSWindow::alloc(nil);
///
///     extern fn on_enter_fullscreen(this: &Object, _cmd: Sel, _notification: id) {
///         unsafe {
///             let window: id = *this.get_ivar("window");
///             window.setToolbar_(nil);
///         }
///     }
///
///     my_window.setDelegate_(delegate!("MyWindowDelegate", {
///         window: id = my_window, // Declare instance variable(s)
///         (onWindowWillEnterFullscreen:) => on_enter_fullscreen as extern fn(&Object, Sel, id) // Declare function(s)
///     }));
/// }
/// # }
/// ```
#[macro_export]
macro_rules! delegate {
    (
        $name:expr, {
            $( ($($sel:ident :)+) => $func:expr),*
        }
    ) => (
        delegate!($name, {
            ,
            $( ($($sel :)+) => $func),*
        })
    );

    (
        $name:expr, {
            $($var:ident : $var_type:ty = $value:expr),* ,
            $( ($($sel:ident :)+) => $func:expr),*
        }
    ) => ({
        let mut decl = objc::declare::ClassDecl::new($name, class!(NSObject)).unwrap();

        $(
            decl.add_ivar::<$var_type>(stringify!($var));
        )*

        $(
            decl.add_method(sel!($($sel :)+), $func);
        )*

        let cl = decl.register();
        let delegate: id = msg_send![cl, alloc];

        $(
            (*delegate).set_ivar(stringify!($var), $value);
        )*

        delegate
    });
}
