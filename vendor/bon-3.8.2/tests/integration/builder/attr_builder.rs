mod multiple_attributes {
    use crate::prelude::*;

    #[test]
    fn test_struct() {
        #[derive(Builder)]
        #[builder(start_fn = start)]
        #[builder(finish_fn = finish)]
        struct Sut {}

        let _ = Sut::start().finish();
    }

    #[test]
    fn test_function() {
        #[builder]
        #[builder]
        #[builder(start_fn = start)]
        #[builder(finish_fn = finish)]
        fn sut() {}

        start().finish();
    }

    #[test]
    fn test_method() {
        struct Sut {}

        #[bon]
        impl Sut {
            #[builder]
            #[builder]
            #[builder(start_fn = start)]
            #[builder(finish_fn = finish)]
            fn method() {}
        }

        Sut::start().finish();
    }
}
