mod borrows;

#[derive(Clone)]
pub enum A {
    B { x: u32, y: u32 },
}

struct Context(A);
impl borrows::Context for Context {
    fn get_a(&mut self, _: u32) -> Option<A> {
        Some(self.0.clone())
    }

    fn u32_pure(&mut self, value: u32) -> Option<u32> {
        Some(value + 1)
    }
}

fn main() {
    borrows::constructor_entry(&mut Context(A::B { x: 1, y: 2 }), 42);
}
