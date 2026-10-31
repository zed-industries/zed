//! # Accessibility in GPUI
//!
//! "Accessibility" refers to the ability of your application to be used by all
//! users, regardless of disability status. There are many aspects, all important, including:
//! - Ensuring sufficient text contrast.
//! - Providing a mechanism to disable animations.
//! - Providing a mechanism to increase text sizes.
//! - etc.
//!
//! This guide is focused on **programmatic accessibility**. This allows
//! assistive technology, such as screen readers or Braille displays, to inspect
//! and interact with your app. Docs for contributors working on accessibility
//! support can be found in the `a11y` module's doc comment.
//!
//! GPUI integrates with [AccessKit] to provide programmatic accessibility
//! features (referred to as simply "accessibility" for the rest of this guide).
//!
//! A minimal example can be found in the `examples/a11y` directory.
//!
//! ## Background
//!
//! Accessibility support is based on two key capabilities:
//! - Exposing information about the current UI state to assistive technology.
//! - Responding to actions requested by assistive technology.
//!
//! For example, a screen reader might want to announce to the user that a new
//! button has appeared. The user may then want to use a voice control program
//! to press that button.
//!
//! ### IDs in GPUI - [`ElementId`] and [`GlobalElementId`]
//!
//! In GPUI, each [`Element`] can have an [`id`][Element::id]:
//! ```rust
//! # use gpui::*;
//! let div_with_id = div().id("my-id").child(text!("hello"));
//!
//! // IDs are optional
//! let div_without_id = div().child(text!("hello"));
//! ```
//!
//! [`Element`]s with IDs are also assigned a [`GlobalElementId`]. This global
//! ID is formed by composing all the non-`None` IDs of its ancestors. For
//! example:
//! ```rust
//! # use gpui::*;
//! let inner = div().id("inner-id");
//! let middle = div().child(inner);  // no ID
//! let outer = div().id("outer-id").child(middle);
//! ```
//! In this example, `inner`s global ID is (roughly speaking) `["outer-id",
//! "inner-id"]`.
//!
//! Since `middle` doesn't have an ID itself, it has no global ID.
//!
//! [`GlobalElementId`]s should be unique per-frame. Duplicate global IDs in the
//! same frame will likely cause bugs.
//!
//! ### IDs and accessibility
//!
//! When GPUI renders a frame, it walks your UI tree, and finds nodes with
//! global IDs, and informs assistive technology about this node.
//!
//! In order for nodes to be reported, they must also have a non-`None`
//! [`role`][Element::a11y_role]. This is used to inform assistive technology
//! what *sort* of node it is (button, label, table, etc.). You can use
//! [`div().id(...).role()`][StatefulInteractiveElement::role] to set the role.
//!
//! Nodes with the same global ID *across frames* are considered to be "the
//! same" node. For example:
//! ```rust
//! # use gpui::*;
//! // The UI in frame 1
//! let frame_1 = div()
//!     .id("parent")
//!     .role(Role::Button)
//!     .child(
//!       div()
//!         .id("id-1")
//!         .role(Role::Label)
//!         .child(text!("hello"))
//!     );
//!
//! // The UI on the next frame
//! let frame_2 = div()
//!     .id("parent")
//!     .role(Role::Button)
//!     .child(
//!       div()
//!         .id("id-2")  // <- different ID
//!         .role(Role::Label)
//!         .child(text!("hello"))
//!     );
//! ```
//! Logically, the UI has not changed. But the screen reader has no way of
//! knowing that both child [`div`]s are "the same". So assistive technology
//! will interpret this as one node being removed, and another node being added.
//! This can be very disorienting for users, since announcements typically only
//! happen when something has *meaningfully* changed.
//!
//! In other words, by controlling the ID of an element, you can control whether
//! a change to a UI element is considered meaningful. You can also control
//! whether elements are reported to assistive technology *at all* by setting
//! the [`role`][Element::a11y_role], since nodes with no role are not reported.
//!
//! #### IDs and text
//!
//! Special care must be taken when dealing with text.
//!
//! GPUI provides the [`text!`] macro, which wraps strings in the [`Text`] type,
//! but automatically derives an ID. Usually, this is what you want. However,
//! the way it generates its ID is subtle and perhaps surprising.
//!
//! The ID of an invocation of the [`text!`] macro is derived from the
//! **location in the source code of that invocation**. For example:
//!
//! ```rust
//! # use gpui::*;
//! let a = text!("a");
//! let b = text!("b");
//!
//! // Different source locations, different IDs
//! assert_ne!(a.id(), b.id());
//!
//! // However:
//!
//! fn make_text(s: &str) -> Text { text!(s) }
//!
//! let a = make_text("a");
//! let b = make_text("b");
//!
//! // Both `a` and `b` are produced by the same `text!` invocation, so the IDs
//! // are the same
//! assert_eq!(a.id(), b.id());
//! ```
//! This can produce surprising behaviour. For example, this footgun:
//! ```rust
//! # use gpui::*;
//! let todos = vec!["eat lunch", "drink water", "go to gym"];
//! let todo_divs = todos.into_iter().map(|todo| {
//!     text!(todo)
//! });
//!
//! div()
//!     .id("todo-list")
//!     .role(Role::Document)
//!     .children(todo_divs);  // ERROR: multiple nodes with the same global ID
//! ```
//!
//! Here, when we map the iterator, since we have only written [`text!`] once,
//! there is only one ID. And since they have the same ancestors and the same
//! ID, they will have the same global ID. In release builds, this will mean
//! some nodes get silently dropped!
//!
//! To fix this, you can set an ID:
//! ```rust
//! # use gpui::*;
//! let todos = vec!["eat lunch", "drink water", "go to gym"];
//! let todo_divs = todos.into_iter().enumerate().map(|(index, todo)| {
//!     text!(todo).with_id(index)  // OR `text(id = index, todo)`
//! });
//!
//! div()
//!     .id("todo-list")
//!     .role(Role::Document)
//!     .children(todo_divs);
//! ```
//! Another possible solution is to wrap the [`text!`] in another node that
//! *does* have a unique global ID. For example:
//! ```rust
//! # use gpui::*;
//! let todos = vec!["eat lunch", "drink water", "go to gym"];
//! let todo_divs = todos.into_iter().enumerate().map(|(index, todo)| {
//!     div().id(index).child(text!(todo))
//! });
//!
//! div()
//!     .id("todo-list")
//!     .role(Role::Document)
//!     .children(todo_divs);
//! ```
//! Since the AccessKit [`NodeId`][accesskit::NodeId] is derived from the global
//! ID, and the global ID takes into account the IDs of all ancestors, this
//! works too.
//!
//! Occasionally, you will need to create a [`Text`] element with *no* ID. You
//! can achieve this with [`Text::new_inaccessible`]. If you are creating a
//! custom UI component (e.g. a button), you may want this so that you can set a
//! label property on a parent [`div`] without duplicating the text in the
//! accessibility tree.
//!
//! ### Handling actions
//!
//! Assistive technology can dispatch actions to the UI. While many users of
//! assistive technology use traditional input devices (e.g. a keyboard), some
//! use more specialized systems. For example, users with limited mobility may
//! use voice control to interact with your app.
//!
//! When a user dispatches an action, it is dispatched *to a specific node*. It
//! is your responsibility to tell the UI elements how they should respond when
//! a request comes in.
//!
//! Note, these actions are **totally unrelated** to GPUI's [`Action`] trait.
//! AccessKit exposes [`accesskit::Action`]. In GPUI, this is re-exported as
//! [`AccessibleAction`].
//!
//! To respond to an accessible action, use
//! [`div().on_a11y_action()`][InteractiveElement::on_a11y_action]:
//! ```rust,ignore
//! div()
//!     .id("my-slider")
//!     .role(Role::Slider)
//!     .on_a11y_action(AccessibleAction::Increment, |_extra, _window, _cx| {
//!         position += 1;
//!         cx.notify();
//!     })
//!     .child(my_cool_slider());
//! ```
//!
//! Note that some common actions are automatically registered. For example,
//! [`.on_click()`][StatefulInteractiveElement::on_click] adds an
//! [`AccessibleAction::Click`] handler that calls the click handler.
//!
//! ## Synthetic children
//!
//! Sometimes, a custom [`Element`] may want to appear as if it is really made
//! of multiple nodes. For example, a totally hypothetical custom text editor
//! element may want to have [`Role::TextInput`], while presenting children
//! consisting of [`Role::TextRun`]s.
//!
//! This is possible using [`Element::a11y_synthetic_children`]. For example:
//! ```rust,ignore
//! # use gpui::*;
//! impl Element for MyCustomTextField {
//!
//!     // ...
//!     
//!     fn a11y_role(&self) -> Option<Role> {
//!         Some(Role::TextInput)
//!     }
//!     
//!     fn a11y_synthetic_children(
//!         &mut self,
//!         _prepaint: &mut Self::PrepaintState,
//!         builder: &mut A11ySubtreeBuilder,
//!     ) {
//!         // Create the synthetic child node
//!         let mut run = accesskit::Node::new(Role::TextRun);
//!         run.set_value(self.text.clone());
//!         run.set_character_lengths(
//!             self.text.chars().map(|c| c.len_utf8() as u8).collect::<Vec<_>>(),
//!         );
//!
//!         // Insert it as a child of `MyCustomTextField`
//!         let run_id = builder.synthetic_node_id(0);
//!         builder.push_child(run_id, run);
//!
//!         // You can also mutate the parent (i.e. the `MyCustomTextField`)
//!         let caret = accesskit::TextPosition {
//!             node: run_id,
//!             character_index: self.cursor,
//!         };
//!         builder.parent_node().set_text_selection(accesskit::TextSelection {
//!             anchor: caret,
//!             focus: caret,
//!         });
//!     }
//! }
//! ```
//!
//! Notably, synthetic children are added *after* an element is
//! [prepainted][Element::prepaint], so prepaint state can be used (for example,
//! to determine what is visible on screen).
//!
//! ## Further reading
//!
//! Designing high-quality accessible interfaces can be challenging, in the same
//! way that designing high-quality traditional interfaces can be. The
//! following pages have useful information:
//!
//! - [AccessKit]: The cross-platform accessibility toolkit GPUI uses
//!   internally.
//! - [MDN WAI-ARIA basics][mdn-aria]: Introduction to roles, properties, and
//!   states.
//! - [ARIA Authoring Practices Guide][apg]: W3C patterns for accessible
//!   widgets.
//!
//! Note that, while GPUI mimics web APIs, it doesn't necessarily behave
//! *exactly* as a web browser would with the same attributes.
//!
//! [AccessKit]: https://accesskit.dev/
//! [mdn-aria]: https://developer.mozilla.org/en-US/docs/Learn_web_development/Core/Accessibility/WAI-ARIA_basics
//! [apg]: https://www.w3.org/WAI/ARIA/apg/

#[cfg(doc)]
use crate::*; // so I don't have to qualify every type :)
