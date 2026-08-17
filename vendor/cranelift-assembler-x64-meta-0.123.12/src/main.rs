//! Print all known instructions.

use cranelift_assembler_x64_meta::instructions;

fn main() {
    let insts = instructions::list();
    for inst in &insts {
        println!("{inst}");
    }
}
