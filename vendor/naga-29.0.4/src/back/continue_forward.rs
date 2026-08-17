//! Workarounds for platform bugs and limitations in switches and loops.
//!
//! In these docs, we use CamelCase links for Naga IR concepts, and ordinary
//! `code` formatting for HLSL or GLSL concepts.
//!
//! ## Avoiding `continue` within `switch`
//!
//! As described in <https://github.com/gfx-rs/wgpu/issues/4485>, the FXC HLSL
//! compiler doesn't allow `continue` statements within `switch` statements, but
//! Naga IR does. We work around this by introducing synthetic boolean local
//! variables and branches.
//!
//! Specifically:
//!
//! - We generate code for [`Continue`] statements within [`SwitchCase`]s that
//!   sets an introduced `bool` local to `true` and does a `break`, jumping to
//!   immediately after the generated `switch`.
//!
//! - When generating code for a [`Switch`] statement, we conservatively assume
//!   it might contain such a [`Continue`] statement, so:
//!
//!   - If it's the outermost such [`Switch`] within a [`Loop`], we declare the
//!     `bool` local ahead of the switch, initialized to `false`. Immediately
//!     after the `switch`, we check the local and do a `continue` if it's set.
//!
//!   - If the [`Switch`] is nested within other [`Switch`]es, then after the
//!     generated `switch`, we check the local (which we know was declared
//!     before the surrounding `switch`) and do a `break` if it's set.
//!
//!   - As an optimization, we only generate the check of the local if a
//!     [`Continue`] statement is encountered within the [`Switch`]. This may
//!     help drivers more easily identify that the `bool` is unused.
//!
//! So while we "weaken" the [`Continue`] statement by rendering it as a `break`
//! statement, we also place checks immediately at the locations to which those
//! `break` statements will jump, until we can be sure we've reached the
//! intended target of the original [`Continue`].
//!
//! In the case of nested [`Loop`] and [`Switch`] statements, there may be
//! multiple introduced `bool` locals in scope, but there's no problem knowing
//! which one to operate on. At any point, there is at most one [`Loop`]
//! statement that could be targeted by a [`Continue`] statement, so the correct
//! `bool` local to set and test is always the one introduced for the innermost
//! enclosing [`Loop`]'s outermost [`Switch`].
//!
//! # Avoiding single body `switch` statements
//!
//! As described in <https://github.com/gfx-rs/wgpu/issues/4514>, some language
//! front ends miscompile `switch` statements where all cases branch to the same
//! body. Our HLSL and GLSL backends render [`Switch`] statements with a single
//! [`SwitchCase`] as `do {} while(false);` loops.
//!
//! However, this rewriting introduces a new loop that could "capture"
//! `continue` statements in its body. To avoid doing so, we apply the
//! [`Continue`]-to-`break` transformation described above.
//!
//! [`Continue`]: crate::Statement::Continue
//! [`Loop`]: crate::Statement::Loop
//! [`Switch`]: crate::Statement::Switch
//! [`SwitchCase`]: crate::SwitchCase

use alloc::{rc::Rc, string::String, vec::Vec};

use crate::proc::Namer;

/// A summary of the code surrounding a statement.
enum Nesting {
    /// Currently nested in at least one [`Loop`] statement.
    ///
    /// [`Continue`] should apply to the innermost loop.
    ///
    /// When this entry is on the top of the stack:
    ///
    /// * When entering an inner [`Loop`] statement, push a [`Loop`][nl] state
    ///   onto the stack.
    ///
    /// * When entering a nested [`Switch`] statement, push a [`Switch`][ns]
    ///   state onto the stack with a new variable name. Before the generated
    ///   `switch`, introduce a `bool` local with that name, initialized to
    ///   `false`.
    ///
    /// When exiting the [`Loop`] for which this entry was pushed, pop it from
    /// the stack.
    ///
    /// [`Continue`]: crate::Statement::Continue
    /// [`Loop`]: crate::Statement::Loop
    /// [`Switch`]: crate::Statement::Switch
    /// [ns]: Nesting::Switch
    /// [nl]: Nesting::Loop
    Loop,

    /// Currently nested in at least one [`Switch`] that may need to forward
    /// [`Continue`]s.
    ///
    /// This includes [`Switch`]es rendered as `do {} while(false)` loops, but
    /// doesn't need to include regular [`Switch`]es in backends that can
    /// support `continue` within switches.
    ///
    /// [`Continue`] should be forwarded to the innermost surrounding [`Loop`].
    ///
    /// When this entry is on the top of the stack:
    ///
    /// * When entering a nested [`Loop`], push a [`Loop`][nl] state onto the
    ///   stack.
    ///
    /// * When entering a nested [`Switch`], push a [`Switch`][ns] state onto
    ///   the stack with a clone of the introduced `bool` variable's name.
    ///
    /// * When encountering a [`Continue`] statement, render it as code to set
    ///   the introduced `bool` local (whose name is held in [`variable`]) to
    ///   `true`, and then `break`. Set [`continue_encountered`] to `true` to
    ///   record that the [`Switch`] contains a [`Continue`].
    ///
    /// * When exiting this [`Switch`], pop its entry from the stack. If
    ///   [`continue_encountered`] is set, then we have rendered [`Continue`]
    ///   statements as `break` statements that jump to this point. Generate
    ///   code to check `variable`, and if it is `true`:
    ///
    ///     * If there is another [`Switch`][ns] left on top of the stack, set
    ///       its `continue_encountered` flag, and generate a `break`. (Both
    ///       [`Switch`][ns]es are within the same [`Loop`] and share the same
    ///       introduced variable, so there's no need to set another flag to
    ///       continue to exit the `switch`es.)
    ///
    ///     * Otherwise, `continue`.
    ///
    /// When we exit the [`Switch`] for which this entry was pushed, pop it.
    ///
    /// [`Continue`]: crate::Statement::Continue
    /// [`Loop`]: crate::Statement::Loop
    /// [`Switch`]: crate::Statement::Switch
    /// [`variable`]: Nesting::Switch::variable
    /// [`continue_encountered`]: Nesting::Switch::continue_encountered
    /// [ns]: Nesting::Switch
    /// [nl]: Nesting::Loop
    Switch {
        variable: Rc<String>,

        /// Set if we've generated code for a [`Continue`] statement with this
        /// entry on the top of the stack.
        ///
        /// If this is still clear when we finish rendering the [`Switch`], then
        /// we know we don't need to generate branch forwarding code. Omitting
        /// that may make it easier for drivers to tell that the `bool` we
        /// introduced ahead of the [`Switch`] is actually unused.
        ///
        /// [`Continue`]: crate::Statement::Continue
        /// [`Switch`]: crate::Statement::Switch
        continue_encountered: bool,
    },
}

/// A micro-IR for code a backend should generate after a [`Switch`].
///
/// [`Switch`]: crate::Statement::Switch
pub(super) enum ExitControlFlow {
    None,
    /// Emit `if (continue_variable) { continue; }`
    Continue {
        variable: Rc<String>,
    },
    /// Emit `if (continue_variable) { break; }`
    ///
    /// Used after a [`Switch`] to exit from an enclosing [`Switch`].
    ///
    /// After the enclosing switch, its associated check will consult this same
    /// variable, see that it is set, and exit early.
    ///
    /// [`Switch`]: crate::Statement::Switch
    Break {
        variable: Rc<String>,
    },
}

/// Utility for tracking nesting of loops and switches to orchestrate forwarding
/// of continue statements inside of a switch to the enclosing loop.
///
/// See [module docs](self) for why we need this.
#[derive(Default)]
pub(super) struct ContinueCtx {
    stack: Vec<Nesting>,
}

impl ContinueCtx {
    /// Resets internal state.
    ///
    /// Use this to reuse memory between writing sessions.
    #[allow(dead_code, reason = "only used by some backends")]
    pub fn clear(&mut self) {
        self.stack.clear();
    }

    /// Updates internal state to record entering a [`Loop`] statement.
    ///
    /// [`Loop`]: crate::Statement::Loop
    pub fn enter_loop(&mut self) {
        self.stack.push(Nesting::Loop);
    }

    /// Updates internal state to record exiting a [`Loop`] statement.
    ///
    /// [`Loop`]: crate::Statement::Loop
    pub fn exit_loop(&mut self) {
        if !matches!(self.stack.pop(), Some(Nesting::Loop)) {
            unreachable!("ContinueCtx stack out of sync");
        }
    }

    /// Updates internal state to record entering a [`Switch`] statement.
    ///
    /// Return `Some(variable)` if this [`Switch`] is nested within a [`Loop`],
    /// and the caller should introcue a new `bool` local variable named
    /// `variable` above the `switch`, for forwarding [`Continue`] statements.
    ///
    /// `variable` is guaranteed not to conflict with any names used by the
    /// program itself.
    ///
    /// [`Continue`]: crate::Statement::Continue
    /// [`Loop`]: crate::Statement::Loop
    /// [`Switch`]: crate::Statement::Switch
    pub fn enter_switch(&mut self, namer: &mut Namer) -> Option<Rc<String>> {
        match self.stack.last() {
            // If the stack is empty, we are not in loop, so we don't need to
            // forward continue statements within this `Switch`. We can leave
            // the stack empty.
            None => None,
            Some(&Nesting::Loop) => {
                let variable = Rc::new(namer.call("should_continue"));
                self.stack.push(Nesting::Switch {
                    variable: Rc::clone(&variable),
                    continue_encountered: false,
                });
                Some(variable)
            }
            Some(&Nesting::Switch { ref variable, .. }) => {
                self.stack.push(Nesting::Switch {
                    variable: Rc::clone(variable),
                    continue_encountered: false,
                });
                // We have already declared the variable before some enclosing
                // `Switch`.
                None
            }
        }
    }

    /// Update internal state to record leaving a [`Switch`] statement.
    ///
    /// Return an [`ExitControlFlow`] value indicating what code should be
    /// introduced after the generated `switch` to forward continues.
    ///
    /// [`Switch`]: crate::Statement::Switch
    pub fn exit_switch(&mut self) -> ExitControlFlow {
        match self.stack.pop() {
            // This doesn't indicate a problem: we don't start pushing entries
            // for `Switch` statements unless we have an enclosing `Loop`.
            None => ExitControlFlow::None,
            Some(Nesting::Loop) => {
                unreachable!("Unexpected loop state when exiting switch");
            }
            Some(Nesting::Switch {
                variable,
                continue_encountered: inner_continue,
            }) => {
                if !inner_continue {
                    // No `Continue` statement was encountered, so we didn't
                    // introduce any `break`s jumping to this point.
                    ExitControlFlow::None
                } else if let Some(&mut Nesting::Switch {
                    continue_encountered: ref mut outer_continue,
                    ..
                }) = self.stack.last_mut()
                {
                    // This is nested in another `Switch`. Propagate upwards
                    // that there is a continue statement present.
                    *outer_continue = true;
                    ExitControlFlow::Break { variable }
                } else {
                    ExitControlFlow::Continue { variable }
                }
            }
        }
    }

    /// Determine what to generate for a [`Continue`] statement.
    ///
    /// If we can generate an ordinary `continue` statement, return `None`.
    ///
    /// Otherwise, we're enclosed by a [`Switch`] that is itself enclosed by a
    /// [`Loop`]. Return `Some(variable)` to indicate that the [`Continue`]
    /// should be rendered as setting `variable` to `true`, and then doing a
    /// `break`.
    ///
    /// This also notes that we've encountered a [`Continue`] statement, so that
    /// we can generate the right code to forward the branch following the
    /// enclosing `switch`.
    ///
    /// [`Continue`]: crate::Statement::Continue
    /// [`Loop`]: crate::Statement::Loop
    /// [`Switch`]: crate::Statement::Switch
    pub fn continue_encountered(&mut self) -> Option<&str> {
        if let Some(&mut Nesting::Switch {
            ref variable,
            ref mut continue_encountered,
        }) = self.stack.last_mut()
        {
            *continue_encountered = true;
            Some(variable)
        } else {
            None
        }
    }
}
