use async_recursion::async_recursion;

struct S;

impl S {
    #[async_recursion]
    pub async fn all_of_the_above<'a, 'b, S, T>(
        &self,
        // Some references with / without lifetimes to generic parameters
        _x: &S,
        _y: &'b T,
        // Some generic parameters passed by value
        _w: S,
        _z: T,
        // A reference to a concrete type without a lifetime
        _p: &usize,
        // A reference to a concrete type with a lifetime
        _q: &'a u64,
    ) {
    }
}