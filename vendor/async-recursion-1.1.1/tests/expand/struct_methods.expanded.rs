use async_recursion::async_recursion;
struct S;
impl S {
    #[must_use]
    pub fn all_of_the_above<'a, 'b, 'life0, 'life1, 'life_self, 'async_recursion, S, T>(
        &'life_self self,
        _x: &'life0 S,
        _y: &'b T,
        _w: S,
        _z: T,
        _p: &'life1 usize,
        _q: &'a u64,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = (),
            > + 'async_recursion + ::core::marker::Send,
        >,
    >
    where
        S: 'async_recursion,
        T: 'async_recursion,
        'life0: 'async_recursion,
        'b: 'async_recursion,
        'life1: 'async_recursion,
        'a: 'async_recursion,
        'life_self: 'async_recursion,
    {
        Box::pin(async move {})
    }
}
