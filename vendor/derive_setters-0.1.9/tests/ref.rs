use derive_setters::*;

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(borrow_self)]
struct BasicRefStruct {
    #[setters(rename = "test")]
    a: u32,
    b: u32,
    #[setters(borrow_self = "false")]
    c: u32,
    #[setters(borrow_self = false)]
    d: u32,
}

#[test]
fn basic_ref_struct() {
    let mut a = BasicRefStruct::default().c(34).d(4);
    a.test(1);
    a.b(3);

    assert_eq!(a.a, 1);
    assert_eq!(a.b, 3);
    assert_eq!(a.c, 34);
    assert_eq!(a.d, 4);
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
struct FieldRefStruct {
    a: u32,
    #[setters(borrow_self)]
    b: u32,
}

#[test]
fn field_ref_struct() {
    let mut a = FieldRefStruct::default().a(10);
    a.b(20);

    assert_eq!(a.a, 10);
    assert_eq!(a.b, 20);
}

#[derive(Default, Setters, Debug, PartialEq, Eq)]
#[setters(borrow_self)]
#[setters(generate_delegates(ty = "BasicRefDelegateField", field = "x"))]
#[setters(generate_delegates(
    ty = "BasicRefDelegateFieldGeneric<T>",
    generics = "<T: Default>",
    field = "x"
))]
#[setters(generate_delegates(ty = "BasicRefDelegateMethod", method = "get_x"))]
struct InnerRefDelegateStruct {
    #[setters(rename = "test")]
    a: u32,
    b: u32,
    c: u32,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct BasicRefDelegateField {
    x: InnerRefDelegateStruct,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct BasicRefDelegateFieldGeneric<T: Default> {
    x: InnerRefDelegateStruct,
    y: T,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct BasicRefDelegateMethod {
    x: Option<InnerRefDelegateStruct>,
}
impl BasicRefDelegateMethod {
    fn get_x(&mut self) -> &mut InnerRefDelegateStruct {
        if self.x.is_none() {
            self.x = Some(InnerRefDelegateStruct::default());
        }
        self.x.as_mut().unwrap()
    }
}

#[test]
fn basic_ref_delegate_field() {
    let mut a = BasicRefDelegateField::default();
    a.test(1);
    a.b(3);
    a.c(34);

    assert_eq!(a.x, InnerRefDelegateStruct { a: 1, b: 3, c: 34 });
}

#[test]
fn basic_ref_delegate_field_generic() {
    let mut a: BasicRefDelegateFieldGeneric<u32> = Default::default();
    a.test(1);
    a.b(3);
    a.c(34);

    assert_eq!(a.x, InnerRefDelegateStruct { a: 1, b: 3, c: 34 });
}

#[test]
fn basic_ref_delegate_method() {
    let mut a = BasicRefDelegateMethod::default();
    a.test(1);
    a.b(3);
    a.c(34);

    assert_eq!(a.x, Some(InnerRefDelegateStruct { a: 1, b: 3, c: 34 }));
}
