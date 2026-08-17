use async_recursion::async_recursion;
use futures_executor::block_on;

pub trait ThirtySeven {
    fn thirty_seven(&self) -> u64 {
        37
    }

    fn descend(&mut self) -> bool;
}

struct Silly {
    counter: usize,
}

impl ThirtySeven for Silly {
    fn descend(&mut self) -> bool {
        if self.counter == 0 {
            false
        } else {
            self.counter -= 1;
            true
        }
    }
}

#[async_recursion]
pub async fn generic_parameter<S: ThirtySeven + Send>(mut x: S) -> u64 {
    if x.descend() {
        generic_parameter(x).await
    } else {
        x.thirty_seven()
    }
}

#[async_recursion(?Send)]
pub async fn generic_parameter_no_send<T>(x: T, y: u64) -> u64 {
    if y > 0 {
        generic_parameter_no_send(x, y - 1).await
    } else {
        111
    }
}

#[test]
fn generic_parameter_is_send() {
    fn assert_is_send(_: impl Send) {}

    assert_is_send(generic_parameter(Silly { counter: 10 }));
}

#[test]
fn generic_parameter_bounds() {
    block_on(async move {
        let s = Silly { counter: 45 };
        assert_eq!(generic_parameter(s).await, 37);
        assert_eq!(
            generic_parameter_no_send(Silly { counter: 999 }, 10).await,
            111
        );
    });
}
