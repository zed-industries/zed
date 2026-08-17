mod multi_extractor;

use multi_extractor::{ContextIter, IntoContextIter};

const MAX_ISLE_RETURNS: usize = 100;

#[derive(Clone)]
pub enum A {
    B,
    C,
}

#[derive(Default)]
struct It {
    i: u32,
    arg: u32,
}

impl ContextIter for It {
    type Context = Context;
    type Output = (A, u32);
    fn next(&mut self, _ctx: &mut Self::Context) -> Option<Self::Output> {
        if self.i >= 32 {
            None
        } else {
            let idx = self.i;
            self.i += 1;
            let a = if self.arg & (1u32 << idx) != 0 {
                A::B
            } else {
                A::C
            };
            Some((a, idx))
        }
    }
}

impl IntoContextIter for It {
    type Context = Context;
    type IntoIter = It;
    type Output = (A, u32);
    fn into_context_iter(self) -> It {
        self
    }
}

struct Context;
impl multi_extractor::Context for Context {
    type e1_etor_returns = It;
    fn e1_etor(&mut self, arg0: u32, returns: &mut It) {
        returns.i = 0;
        returns.arg = arg0;
    }
}

fn main() {
    let mut ctx = Context;
    let mut x = vec![];
    multi_extractor::constructor_Rule(&mut ctx, 0xf0, &mut x);
    let mut y = vec![];
    multi_extractor::constructor_Rule(&mut ctx, 0, &mut y);
    println!("x = {:?} y = {:?}", x, y);
}
