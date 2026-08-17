mod test;

struct Context;
impl test::Context for Context {
    fn get_input(&mut self, x: u32) -> Option<test::A> {
        Some(test::A::A1 { x: x + 1 })
    }
}

fn main() {
    test::constructor_Lower(&mut Context, &test::A::A1 { x: 42 });
}
