mod multi_constructor;
use multi_constructor::{ContextIter, IntoContextIter};

struct Context;

const MAX_ISLE_RETURNS: usize = 100;

#[derive(Default)]
struct It {
    i: u32,
    limit: u32,
}

impl ContextIter for It {
    type Context = Context;
    type Output = u32;
    fn next(&mut self, _ctx: &mut Self::Context) -> Option<u32> {
        if self.i >= self.limit {
            None
        } else {
            let i = self.i;
            self.i += 1;
            Some(i)
        }
    }
}

impl IntoContextIter for It {
    type Context = Context;
    type Output = u32;
    type IntoIter = It;
    fn into_context_iter(self) -> It {
        self
    }
}

impl multi_constructor::Context for Context {
    type etor_C_returns = It;
    fn etor_C(&mut self, value: u32, returns: &mut It) {
        returns.i = 0;
        returns.limit = value;
    }

    type ctor_B_returns = multi_constructor::ContextIterWrapper<Vec<u32>, Context>;
    fn ctor_B(&mut self, value: u32, returns: &mut Self::ctor_B_returns) {
        returns.extend((0..value).rev());
    }
}

struct IterWithContext<
    'a,
    Item,
    I: multi_constructor::ContextIter<Output = Item, Context = Context>,
> {
    ctx: &'a mut Context,
    it: I,
}

impl<'a, Item, I: multi_constructor::ContextIter<Output = Item, Context = Context>> Iterator
    for IterWithContext<'a, Item, I>
{
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.it.next(self.ctx)
    }
}

fn main() {
    let mut ctx = Context;

    let mut l1 = multi_constructor::ContextIterWrapper::<Vec<_>, _>::default();
    multi_constructor::constructor_A(&mut ctx, 10, &mut l1);

    let mut l2 = multi_constructor::ContextIterWrapper::<Vec<_>, _>::default();
    multi_constructor::constructor_D(&mut ctx, 5, &mut l2);

    let l1 = IterWithContext {
        ctx: &mut ctx,
        it: l1.into_context_iter(),
    }
    .collect::<Vec<_>>();
    let l2 = IterWithContext {
        ctx: &mut ctx,
        it: l2.into_context_iter(),
    }
    .collect::<Vec<_>>();
    println!("l1 = {:?} l2 = {:?}", l1, l2);
}
