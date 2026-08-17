mod let_shadowing;

struct Context;

impl let_shadowing::Context for Context {}

fn main() {
    let mut ctx = Context;

    assert_eq!(20, let_shadowing::constructor_test1(&mut ctx, 20));
    assert_eq!(97, let_shadowing::constructor_test1(&mut ctx, 97));

    assert_eq!(20, let_shadowing::constructor_test2(&mut ctx, 20));
    assert_eq!(97, let_shadowing::constructor_test2(&mut ctx, 97));

    assert_eq!(20, let_shadowing::constructor_test3(&mut ctx, 20));
    assert_eq!(97, let_shadowing::constructor_test3(&mut ctx, 97));

    assert_eq!(23, let_shadowing::constructor_test4(&mut ctx, 20));
    assert_eq!(23, let_shadowing::constructor_test4(&mut ctx, 97));

    assert_eq!(20, let_shadowing::constructor_test5(&mut ctx, 20));
    assert_eq!(97, let_shadowing::constructor_test5(&mut ctx, 97));

    assert_eq!(20, let_shadowing::constructor_test6(&mut ctx, 20));
    assert_eq!(97, let_shadowing::constructor_test6(&mut ctx, 97));
}
