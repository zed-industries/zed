//! Cranelift IR builder library.
//!
//! Provides a straightforward way to create a Cranelift IR function and fill it with instructions
//! corresponding to your source program written in another language.
//!
//! To get started, create an [`FunctionBuilderContext`](struct.FunctionBuilderContext.html) and
//! pass it as an argument to a [`FunctionBuilder`].
//!
//! # Mutable variables and Cranelift IR values
//!
//! The most interesting feature of this API is that it provides a single way to deal with all your
//! variable problems. Indeed, the [`FunctionBuilder`] struct has a
//! type `Variable` that should be an index of your source language variables. Then, through
//! calling the functions
//! [`declare_var`](FunctionBuilder::declare_var), [`def_var`](FunctionBuilder::def_var) and
//! [`use_var`](FunctionBuilder::use_var), the [`FunctionBuilder`] will create for you all the
//! Cranelift IR values corresponding to your variables.
//!
//! This API has been designed to help you translate your mutable variables into
//! [`SSA`](https://en.wikipedia.org/wiki/Static_single_assignment_form) form.
//! [`use_var`](FunctionBuilder::use_var) will return the Cranelift IR value
//! that corresponds to your mutable variable at a precise point in the program. However, if you know
//! beforehand that one of your variables is defined only once, for instance if it is the result
//! of an intermediate expression in an expression-based language, then you can translate it
//! directly by the Cranelift IR value returned by the instruction builder. Using the
//! [`use_var`](FunctionBuilder::use_var) API for such an immutable variable
//! would also work but with a slight additional overhead (the SSA algorithm does not know
//! beforehand if a variable is immutable or not).
//!
//! The moral is that you should use these three functions to handle all your mutable variables,
//! even those that are not present in the source code but artifacts of the translation. It is up
//! to you to keep a mapping between the mutable variables of your language and their [`Variable`]
//! index that is used by Cranelift. Caution: as the [`Variable`] is used by Cranelift to index an
//! array containing information about your mutable variables, when you create a new [`Variable`]
//! with `Variable::new(var_index)` you should make sure that `var_index`
//! is provided by a counter incremented by 1 each time you encounter a new mutable variable.
//!
//! # Example
//!
//! Here is a pseudo-program we want to transform into Cranelift IR:
//!
//! ```clif
//! function(x) {
//! x, y, z : i32
//! block0:
//!    y = 2;
//!    z = x + y;
//!    jump block1
//! block1:
//!    z = z + y;
//!    brif y, block3, block2
//! block2:
//!    z = z - x;
//!    return y
//! block3:
//!    y = y - x
//!    jump block1
//! }
//! ```
//!
//! Here is how you build the corresponding Cranelift IR function using [`FunctionBuilderContext`]:
//!
//! ```rust
//! use cranelift_codegen::entity::EntityRef;
//! use cranelift_codegen::ir::types::*;
//! use cranelift_codegen::ir::{AbiParam, UserFuncName, Function, InstBuilder, Signature};
//! use cranelift_codegen::isa::CallConv;
//! use cranelift_codegen::settings;
//! use cranelift_codegen::verifier::verify_function;
//! use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
//!
//! let mut sig = Signature::new(CallConv::SystemV);
//! sig.returns.push(AbiParam::new(I32));
//! sig.params.push(AbiParam::new(I32));
//! let mut fn_builder_ctx = FunctionBuilderContext::new();
//! let mut func = Function::with_name_signature(UserFuncName::user(0, 0), sig);
//! {
//!     let mut builder = FunctionBuilder::new(&mut func, &mut fn_builder_ctx);
//!
//!     let block0 = builder.create_block();
//!     let block1 = builder.create_block();
//!     let block2 = builder.create_block();
//!     let block3 = builder.create_block();
//!     let x = builder.declare_var(I32);
//!     let y = builder.declare_var(I32);
//!     let z = builder.declare_var(I32);
//!     builder.append_block_params_for_function_params(block0);
//!
//!     builder.switch_to_block(block0);
//!     builder.seal_block(block0);
//!     {
//!         let tmp = builder.block_params(block0)[0]; // the first function parameter
//!         builder.def_var(x, tmp);
//!     }
//!     {
//!         let tmp = builder.ins().iconst(I32, 2);
//!         builder.def_var(y, tmp);
//!     }
//!     {
//!         let arg1 = builder.use_var(x);
//!         let arg2 = builder.use_var(y);
//!         let tmp = builder.ins().iadd(arg1, arg2);
//!         builder.def_var(z, tmp);
//!     }
//!     builder.ins().jump(block1, &[]);
//!
//!     builder.switch_to_block(block1);
//!     {
//!         let arg1 = builder.use_var(y);
//!         let arg2 = builder.use_var(z);
//!         let tmp = builder.ins().iadd(arg1, arg2);
//!         builder.def_var(z, tmp);
//!     }
//!     {
//!         let arg = builder.use_var(y);
//!         builder.ins().brif(arg, block3, &[], block2, &[]);
//!     }
//!
//!     builder.switch_to_block(block2);
//!     builder.seal_block(block2);
//!     {
//!         let arg1 = builder.use_var(z);
//!         let arg2 = builder.use_var(x);
//!         let tmp = builder.ins().isub(arg1, arg2);
//!         builder.def_var(z, tmp);
//!     }
//!     {
//!         let arg = builder.use_var(y);
//!         builder.ins().return_(&[arg]);
//!     }
//!
//!     builder.switch_to_block(block3);
//!     builder.seal_block(block3);
//!
//!     {
//!         let arg1 = builder.use_var(y);
//!         let arg2 = builder.use_var(x);
//!         let tmp = builder.ins().isub(arg1, arg2);
//!         builder.def_var(y, tmp);
//!     }
//!     builder.ins().jump(block1, &[]);
//!     builder.seal_block(block1);
//!
//!     builder.finalize();
//! }
//!
//! let flags = settings::Flags::new(settings::builder());
//! let res = verify_function(&func, &flags);
//! println!("{}", func.display());
//! if let Err(errors) = res {
//!     panic!("{}", errors);
//! }
//! ```

#![deny(missing_docs)]
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(not(feature = "std"))]
use hashbrown::{HashMap, HashSet};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

pub use crate::frontend::{FuncInstBuilder, FunctionBuilder, FunctionBuilderContext};
pub use crate::switch::Switch;
pub use crate::variable::Variable;

#[cfg(test)]
macro_rules! assert_eq_output {
    ( $left:expr, $right:expr $(,)? ) => {{
        let left = $left;
        let left = left.trim();

        let right = $right;
        let right = right.trim();

        assert_eq!(
            left,
            right,
            "assertion failed, output not equal:\n\
             \n\
             =========== Diff ===========\n\
             {}\n\
             =========== Left ===========\n\
             {left}\n\
             =========== Right ===========\n\
             {right}\n\
             ",
            similar::TextDiff::from_lines(left, right)
                .unified_diff()
                .header("left", "right")
        )
    }};
}

mod frontend;
mod ssa;
mod switch;
mod variable;

/// Version number of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
