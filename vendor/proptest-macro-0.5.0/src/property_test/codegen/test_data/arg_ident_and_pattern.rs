// Make sure that code for choosing names for the generated struct is robust against mixtures of
// ident and non-ident parameter patterns

fn foo(
    a: i32,
    (b, c): (i32, i32),
    d: i32,
    Wrapper(e): Wrapper<i32>,
    [f, g]: [i32; 2],
    h: i32,
    Point { x, y }: Point,
) {}
