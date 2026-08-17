use derive_setters::Setters;

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(no_std, prefix = "set_", into)]
struct AllFlagsStruct {
    a: u32,
    #[setters(strip_option, into = false)]
    b: Option<u32>,
    #[setters(bool, rename = "c_flag")]
    c: bool,
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
struct FieldFlagsStruct {
    #[setters(rename = "custom_a", into)]
    a: u32,
    
    #[setters(skip)]
    b: u32,
    
    #[setters(strip_option)]
    c: Option<u32>,
    
    #[setters(bool)]
    d: bool,

    #[setters(borrow_self)]
    e: u32,
    
    #[setters(generate)]
    f: u32,
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(generate = false)]
struct NoGenerateStruct {
    #[setters(generate)]
    a: u32,
    b: u32,
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(generate_private = false)]
pub struct PrivateGenerateStruct {
    pub a: u32,
    b: u32,
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(generate_public = false)]
pub struct PublicGenerateStruct {
    pub a: u32,
    b: u32,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct DelegateTarget {
    inner: AllFlagsStruct,
}

impl DelegateTarget {
    fn get_inner_mut(&mut self) -> &mut AllFlagsStruct {
        &mut self.inner
    }
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(generate_delegates(ty = "DelegateTarget", field = "inner", prefix = "delegate_field_"))]
#[setters(generate_delegates(ty = "DelegateTarget", method = "get_inner_mut", prefix = "delegate_method_"))]
struct DelegatedStruct {
    a: u32,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct GenericDelegateTarget<T> {
    inner: T,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct GenericDelegateTargetOther<T: Default> {
    unrelated: T,
    inner: GenericDelegatedStruct,
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(generate_delegates(ty = "GenericDelegateTarget<GenericDelegatedStruct>", field = "inner"))]
#[setters(generate_delegates(ty = "GenericDelegateTargetOther<T>", generics = "<T: Default>", field = "inner"))]
struct GenericDelegatedStruct {
    a: u32,
}

fn main() {
    let s1 = AllFlagsStruct::default().set_a(10u8).set_b(20).c_flag();
    assert_eq!(s1.a, 10);
    assert_eq!(s1.b, Some(20));
    assert_eq!(s1.c, true);

    let s2 = FieldFlagsStruct::default();
    let s2 = s2.custom_a(30u8);
    let s2 = s2.c(50);
    let s2 = s2.d();
    let mut s2 = s2;
    s2.e(60);
    let s2 = s2.f(70);
    assert_eq!(s2.a, 30);
    assert_eq!(s2.c, Some(50));
    assert_eq!(s2.d, true);
    assert_eq!(s2.e, 60);
    assert_eq!(s2.f, 70);

    let s3 = NoGenerateStruct::default().a(80);
    assert_eq!(s3.a, 80);

    let s4 = PrivateGenerateStruct::default().a(100);
    assert_eq!(s4.a, 100);

    let s5 = PublicGenerateStruct::default().b(120);
    assert_eq!(s5.b, 120);

    let value = DelegateTarget::default();
    let value = value.delegate_field_a(140);
    let value = value.delegate_method_a(150);
    assert_eq!(value.inner.a, 150);

    let value = GenericDelegateTarget::<GenericDelegatedStruct>::default();
    let value = value.a(160);
    assert_eq!(value.inner.a, 160);

    let value = GenericDelegateTargetOther::<u32>::default();
    let value = value.a(160);
    assert_eq!(value.inner.a, 160);
}
