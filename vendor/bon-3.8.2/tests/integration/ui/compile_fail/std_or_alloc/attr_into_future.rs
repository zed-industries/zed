use bon::{bon, builder};
use core::future::IntoFuture;

fn _non_send() {
    struct Sut;

    fn assert_send(_: &dyn Send) {}

    #[bon]
    impl Sut {
        #[builder(derive(IntoFuture(Box, ?Send)))]
        async fn sut(&self, value: u32) -> u32 {
            value * 2
        }
    }

    assert_send(&Sut.sut().value(21).into_future());

    #[builder(derive(IntoFuture(Box, ?Send)))]
    async fn sut(value: u32) -> u32 {
        value * 2
    }

    assert_send(&sut().value(21).into_future());
}

fn main() {}
