use std::mem::size_of_val;
use itertools::assert_equal;
use any_vec::{AnyVec, SatisfyTraits};
use any_vec::traits::*;

#[test]
pub fn test_default(){
    let _any_vec: AnyVec = AnyVec::new::<String>();
    // should not compile
    //fn t(_: impl Sync){}
    //t(any_vec);
}

#[test]
pub fn test_sync(){
    let any_vec: AnyVec<dyn Cloneable + Sync + Send> = AnyVec::new::<String>();
    fn t(_: impl Sync){}
    t(any_vec);
}

#[test]
pub fn test_clone(){
    fn do_test<Traits: ?Sized + Cloneable + Trait>()
        where String: SatisfyTraits<Traits>
    {
        let mut any_vec: AnyVec<Traits> = AnyVec::new::<String>();
        {
            let mut vec = any_vec.downcast_mut::<String>().unwrap();
            vec.push(String::from("0"));
            vec.push(String::from("1"));
            vec.push(String::from("2"));
        }

        let any_vec2 = any_vec.clone();
        assert_equal(
            any_vec.downcast_ref::<String>().unwrap().as_slice(),
            any_vec2.downcast_ref::<String>().unwrap().as_slice()
        );
    }

    do_test::<dyn Cloneable>();
    do_test::<dyn Cloneable + Sync>();
    do_test::<dyn Cloneable + Send>();
    do_test::<dyn Cloneable + Sync + Send>();
}

#[test]
pub fn type_check_test(){
    fn fn_send(_: &impl Send){}
    fn fn_sync(_: &impl Sync){}

    {
        let any_vec: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
        let _ = any_vec.clone();
    }
    {
        let any_vec: AnyVec<dyn Send> = AnyVec::new::<String>();
        fn_send(&any_vec);
    }
    {
        let any_vec: AnyVec<dyn Sync> = AnyVec::new::<String>();
        fn_sync(&any_vec);
    }
    {
        let any_vec: AnyVec<dyn Send + Sync> = AnyVec::new::<String>();
        fn_send(&any_vec);
        fn_sync(&any_vec);
    }
    {
        let any_vec: AnyVec<dyn Send + Sync + Cloneable> = AnyVec::new::<String>();
        fn_send(&any_vec);
        fn_sync(&any_vec);
        let _ = any_vec.clone();
    }
}

/*// Should not compile
#[test]
pub fn type_check_fail_test(){
    let any_vec: AnyVec = AnyVec::new::<String>();
    any_vec.clone();
}*/

#[test]
fn any_vec_cloneable_zst_test(){
    let v1: AnyVec<dyn Cloneable> = AnyVec::new::<usize>();
    let v2: AnyVec<dyn Sync> = AnyVec::new::<usize>();

    let s1 = size_of_val(&v1);
    let s2 = size_of_val(&v2);

    assert!(s1 > s2);
}