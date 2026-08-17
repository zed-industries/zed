use async_recursion::async_recursion;
use futures_executor::block_on;

pub struct Empty {}

impl Empty {
    #[async_recursion]
    pub async fn fib(&self, n: u32) -> u64 {
        match n {
            0 => panic!("zero is not a valid argument to fib()!"),
            1 | 2 => 1,
            3 => 2,
            _ => self.fib(n - 1).await + self.fib(n - 2).await,
        }
    }

    #[async_recursion]
    pub async fn empty_string<'a>(&self, some_str: &'a str) -> &'a str {
        if some_str.is_empty() {
            ""
        } else {
            self.empty_string(&some_str[1..]).await
        }
    }

    #[async_recursion]
    pub async fn generic_parameter<T>(&self, _something: &T) -> u64 {
        0
    }

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

#[test]
fn struct_method_fib_works() {
    block_on(async move {
        let e = Empty {};
        assert_eq!(e.fib(6).await, 8);
        assert_eq!(e.fib(5).await, 5);
        assert_eq!(e.fib(7).await, 13);
    });
}

#[test]
fn struct_method_empty_string_works() {
    block_on(async move {
        let e = Empty {};
        assert_eq!(e.empty_string("hello world").await, "");
        assert_eq!(e.empty_string("something else").await, "");
    });
}

#[test]
fn struct_method_with_generic_parameter_works() {
    block_on(async move {
        let e = Empty {};
        assert_eq!(
            e.generic_parameter::<*const u64>(&(0 as *const u64)).await,
            0
        );
    })
}
