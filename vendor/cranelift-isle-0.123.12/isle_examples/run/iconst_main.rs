mod iconst;

struct Context;
impl iconst::Context for Context {}

fn main() {
    let mut ctx = Context;

    assert_eq!(iconst::constructor_X(&mut ctx, -1), Some(-2));
    assert_eq!(iconst::constructor_X(&mut ctx, -2), Some(-3));
    assert_eq!(
        iconst::constructor_X(&mut ctx, 0x7fff_ffff_ffff_ffff),
        Some(0x8000_0000_0000_0000u64 as i64)
    );
    assert_eq!(
        iconst::constructor_X(&mut ctx, 0xffff_ffff_ffff_fff0_u64 as i64),
        Some(1)
    );

    assert_eq!(
        iconst::constructor_Y(&mut ctx, 0x1000_0000_0000_0000_1234_5678_9abc_def0),
        Some(-1)
    );
    assert_eq!(
        iconst::constructor_Y(
            &mut ctx,
            0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffffu128 as i128
        ),
        Some(3)
    );
    assert_eq!(
        iconst::constructor_Y(&mut ctx, -0x1000_0000_0000_0000_1234_5678_9abc_def0),
        Some(1)
    );
    assert_eq!(
        iconst::constructor_Y(
            &mut ctx,
            -(0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffffu128 as i128)
        ),
        Some(-3)
    );

    assert_eq!(iconst::constructor_Z(&mut ctx, 0), Some(1));
    assert_eq!(iconst::constructor_Z(&mut ctx, 1), Some(2));
    assert_eq!(iconst::constructor_Z(&mut ctx, 2), Some(3));
    assert_eq!(iconst::constructor_Z(&mut ctx, 3), Some(4));
    assert_eq!(
        iconst::constructor_Z(&mut ctx, 0o7654321),
        Some(0b11_00_11_00)
    );
}
