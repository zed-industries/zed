//! Primitive support for debugging Pulley
//!
//! This `Debug` visitor defined in this module is what's actually used as part
//! of the interpreter loop in Pulley. Due to the code size impact of always
//! including this and the runtime overhead of always checking a flag this is
//! enabled/disabled via a `const DEBUG` below. This is currently only really
//! suitable for one-off debugging while developing locally.
//!
//! The hope is that this'll eventually evolve into something more useful, but
//! for now it's a quick-and-easy way to dump all the instructions that are
//! executed as well as the values in various registers.
//!
//! If debugging is disabled, or in `#[no_std]` mode, then this module should
//! compile away (e.g. a "zero cost abstraction").

use super::Interpreter;
use crate::decode::{ExtendedOpVisitor, OpVisitor};
use crate::imms::*;
use crate::regs::*;
use alloc::string::ToString;

// Whether or not debugging is enabled at all.
const DEBUG: bool = false;

// Whether or not these registers are dumped between each instruction.
const DEBUG_X_REGS: bool = true;
const DEBUG_F_REGS: bool = false;

#[cfg(not(feature = "std"))]
macro_rules! print {
    ($($t:tt)*) => ({ let _ = format_args!($($t)*); })
}
#[cfg(not(feature = "std"))]
macro_rules! println {
    () => ();
    ($($t:tt)*) => ({ let _ = format_args!($($t)*); })
}

#[repr(transparent)]
pub(super) struct Debug<'a>(pub Interpreter<'a>);

macro_rules! debug_then_delegate {
    (
        $(
            $( #[$attr:meta] )*
                $snake_name:ident = $name:ident $( {
                $(
                    $( #[$field_attr:meta] )*
                    $field:ident : $field_ty:ty
                ),*
            } )? ;
        )*
    ) => {
        $(
            $( #[$attr] )*
            fn $snake_name(&mut self $( $( , $field : $field_ty )* )? ) -> Self::Return {
                if DEBUG {
                    println!(
                        concat!(
                            stringify!($snake_name),
                            $(
                                $(
                                    " ",
                                    stringify!($field),
                                    "={:?}",
                                )*
                            )?
                        ),
                        $($($field),*)?
                    );
                }
                self.0.$snake_name($( $($field),* )?)
            }
        )*
    }
}

impl<'a> OpVisitor for Debug<'a> {
    type BytecodeStream = <Interpreter<'a> as OpVisitor>::BytecodeStream;
    type Return = <Interpreter<'a> as OpVisitor>::Return;

    fn bytecode(&mut self) -> &mut Self::BytecodeStream {
        self.0.bytecode()
    }

    fn before_visit(&mut self) {
        self.0.record_executing_pc_for_profiling();
        if !DEBUG {
            return;
        }
        print!("\t{:?}\t", self.bytecode().as_ptr());
    }

    fn after_visit(&mut self) {
        if !DEBUG {
            return;
        }
        if DEBUG_X_REGS {
            for (i, regs) in self.0.state.x_regs.chunks(4).enumerate() {
                print!("\t\t");
                for (j, reg) in regs.iter().enumerate() {
                    let n = i * 4 + j;
                    let val = reg.get_u64();
                    let reg = XReg::new(n as u8).unwrap().to_string();
                    print!(" {reg:>3}={val:#018x}");
                }
                println!();
            }
        }
        if DEBUG_F_REGS {
            for (i, regs) in self.0.state.f_regs.chunks(4).enumerate() {
                print!("\t\t");
                for (j, reg) in regs.iter().enumerate() {
                    let n = i * 4 + j;
                    let val = reg.get_f64().to_bits();
                    let reg = FReg::new(n as u8).unwrap().to_string();
                    print!(" {reg:>3}={val:#018x}");
                }
                println!();
            }
        }
    }

    for_each_op!(debug_then_delegate);
}

impl<'a> ExtendedOpVisitor for Debug<'a> {
    for_each_extended_op!(debug_then_delegate);
}
