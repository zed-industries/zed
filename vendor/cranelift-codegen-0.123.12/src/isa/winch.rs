use crate::machinst::{ABIArg, ABIArgSlot, ArgsAccumulator};

// Winch writes the first result to the highest offset, so we need to iterate through the
// args and adjust the offsets down.
pub(super) fn reverse_stack(mut args: ArgsAccumulator, next_stack: u32, uses_extension: bool) {
    for arg in args.args_mut() {
        if let ABIArg::Slots { slots, .. } = arg {
            for slot in slots.iter_mut() {
                if let ABIArgSlot::Stack { offset, ty, .. } = slot {
                    let size = if uses_extension {
                        i64::from(std::cmp::max(ty.bytes(), 8))
                    } else {
                        i64::from(ty.bytes())
                    };
                    *offset = i64::from(next_stack) - *offset - size;
                }
            }
        } else {
            unreachable!("Winch cannot handle {arg:?}");
        }
    }
}
