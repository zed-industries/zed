use crate::prelude::*;

#[test]
fn test_function() {
    #[builder]
    fn sut(state: u32, member_state: u32, unset: u32, empty: u32) {
        let _ = (state, member_state, unset, empty);
    }

    sut().state(1).member_state(2).unset(3).empty(4).call();
}

#[test]
fn test_struct() {
    #[derive(Builder)]
    #[allow(dead_code)]
    struct Sut {
        state: u32,
        member_state: u32,
        unset: u32,
        empty: u32,
    }

    let _ = Sut::builder()
        .state(1)
        .member_state(2)
        .unset(3)
        .empty(4)
        .build();
}

#[test]
fn test_method() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder]
        fn sut(state: u32, member_state: u32, unset: u32, empty: u32) {
            let _ = (state, member_state, unset, empty);
        }
    }

    Sut::sut().state(1).member_state(2).unset(3).empty(4).call();
}
