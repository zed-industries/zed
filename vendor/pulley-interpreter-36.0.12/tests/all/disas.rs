//! Disassembly tests.

use pulley_interpreter::*;

fn encoded(ops: &[Op]) -> Vec<u8> {
    let mut encoded = vec![];
    for op in ops {
        op.encode(&mut encoded);
    }
    log::trace!("encoded: {encoded:?}");
    encoded
}

#[track_caller]
fn assert_disas_with_disassembler(dis: &mut disas::Disassembler<'_>, expected: &str) {
    let expected = expected.trim();
    eprintln!("=== expected ===\n{expected}");

    decode::Decoder::decode_all(dis).expect("decoding should succeed");

    let actual = dis.disas().trim();
    eprintln!("=== actual ===\n{actual}");

    assert_eq!(expected, actual);
}

#[track_caller]
fn assert_disas(ops: &[Op], expected: &str) {
    let bytecode = encoded(ops);
    assert_disas_with_disassembler(
        &mut disas::Disassembler::new(&bytecode).hexdump(false),
        expected,
    );
}

#[test]
fn simple() {
    assert_disas(
        &[
            // Prologue.
            Op::PushFrame(PushFrame {}),
            // Function body.
            Op::Xadd32(Xadd32 {
                operands: BinaryOperands {
                    dst: XReg::x0,
                    src1: XReg::x0,
                    src2: XReg::x1,
                },
            }),
            // Epilogue.
            Op::PopFrame(PopFrame {}),
            Op::Ret(Ret {}),
        ],
        r#"
       0: push_frame
       1: xadd32 x0, x0, x1
       4: pop_frame
       5: ret
        "#,
    );
}

#[test]
fn no_offsets() {
    let bytecode = encoded(&[
        // Prologue.
        Op::PushFrame(PushFrame {}),
        // Function body.
        Op::Xadd32(Xadd32 {
            operands: BinaryOperands {
                dst: XReg::x0,
                src1: XReg::x0,
                src2: XReg::x1,
            },
        }),
        // Epilogue.
        Op::PopFrame(PopFrame {}),
        Op::Ret(Ret {}),
    ]);

    assert_disas_with_disassembler(
        disas::Disassembler::new(&bytecode)
            .offsets(false)
            .hexdump(false),
        r#"
push_frame
xadd32 x0, x0, x1
pop_frame
ret
        "#,
    );
}

#[test]
fn no_offsets_or_hexdump() {
    let bytecode = encoded(&[
        // Prologue.
        Op::PushFrame(PushFrame {}),
        // Function body.
        Op::Xadd32(Xadd32 {
            operands: BinaryOperands {
                dst: XReg::x0,
                src1: XReg::x0,
                src2: XReg::x1,
            },
        }),
        // Epilogue.
        Op::PopFrame(PopFrame {}),
        Op::Ret(Ret {}),
    ]);

    assert_disas_with_disassembler(
        disas::Disassembler::new(&bytecode)
            .offsets(false)
            .hexdump(false),
        r#"
push_frame
xadd32 x0, x0, x1
pop_frame
ret
        "#,
    );
}

#[test]
fn disassemble_br_table() {
    let mut bytecode = Vec::new();
    PushFrame {}.encode(&mut bytecode);
    BrTable32 {
        idx: XReg::x1,
        amt: 4,
    }
    .encode(&mut bytecode);
    bytecode.extend_from_slice(&0_i32.to_le_bytes());
    bytecode.extend_from_slice(&0_i32.to_le_bytes());
    bytecode.extend_from_slice(&0_i32.to_le_bytes());
    bytecode.extend_from_slice(&0_i32.to_le_bytes());
    PopFrame {}.encode(&mut bytecode);

    assert_disas_with_disassembler(
        disas::Disassembler::new(&bytecode)
            .hexdump(false)
            .offsets(false),
        r#"
push_frame
br_table32 x1, 4
0x0    // target = 0x7
0x0    // target = 0xb
0x0    // target = 0xf
0x0    // target = 0x13
pop_frame
        "#,
    );
}
