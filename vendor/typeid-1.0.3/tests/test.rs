#[test]
fn test_non_static_type_id() {
    assert_eq!(typeid::of::<usize>(), typeid::of::<usize>());
    assert_eq!(typeid::of::<&str>(), typeid::of::<&'static str>());

    assert_ne!(typeid::of::<u32>(), typeid::of::<[u8; 4]>());
    assert_ne!(typeid::of::<u32>(), typeid::of::<[u32; 2]>());

    assert_ne!(typeid::of::<usize>(), typeid::of::<isize>());
    assert_ne!(typeid::of::<usize>(), typeid::of::<&usize>());
    assert_ne!(typeid::of::<&usize>(), typeid::of::<&&usize>());
    assert_ne!(typeid::of::<&usize>(), typeid::of::<&mut usize>());

    assert_ne!(typeid::of::<fn(&str)>(), typeid::of::<fn(&'static str)>());

    trait Trait<'a> {}
    assert_ne!(
        typeid::of::<dyn for<'a> Trait<'a>>(),
        typeid::of::<dyn Trait<'static>>(),
    );

    struct A;
    struct B;
    assert_ne!(typeid::of::<A>(), typeid::of::<B>());
}
