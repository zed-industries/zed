use glib::{prelude::*, value::FromValue, Value, ValueDelegate};

#[test]
fn into_value() {
    fn test_func(_: impl Into<Value>) {}

    #[derive(ValueDelegate)]
    pub struct Test(i32);

    #[derive(ValueDelegate)]
    #[value_delegate(nullable)]
    pub struct TestNullable(String);

    #[derive(ValueDelegate)]
    #[value_delegate(from = i64)]
    pub struct TestManualFrom(i32);

    impl From<i64> for TestManualFrom {
        fn from(v: i64) -> Self {
            Self(v as i32)
        }
    }
    impl<'a> From<&'a TestManualFrom> for i64 {
        fn from(v: &'a TestManualFrom) -> Self {
            v.0 as i64
        }
    }
    impl From<TestManualFrom> for i64 {
        fn from(v: TestManualFrom) -> Self {
            v.0 as i64
        }
    }

    test_func(Test(123));
    test_func(Test(123));

    test_func(TestManualFrom(123));
    test_func(TestManualFrom(123));

    test_func(TestNullable("foo".to_string()));
    test_func(TestNullable("foo".to_string()));
    test_func(Some(&TestNullable("foo".to_string())));
    test_func(Some(TestNullable("foo".to_string())));
    test_func(Some(TestNullable("foo".to_string())));

    assert_eq!(glib::Value::from(Test(123)).get::<Test>().unwrap().0, 123);
    assert_eq!(glib::Value::from(123).get::<Test>().unwrap().0, 123);
    assert_eq!(glib::Value::from(Test(123)).get::<i32>().unwrap(), 123);

    assert_eq!(
        glib::Value::from(TestManualFrom(123))
            .get::<TestManualFrom>()
            .unwrap()
            .0,
        123
    );
    assert_eq!(
        glib::Value::from(123_i64)
            .get::<TestManualFrom>()
            .unwrap()
            .0,
        123
    );
    assert_eq!(
        glib::Value::from(TestManualFrom(123)).get::<i64>().unwrap(),
        123
    );

    // From TestNullable
    assert_eq!(
        glib::Value::from(TestNullable("foo".to_string()))
            .get::<Option<TestNullable>>()
            .unwrap()
            .unwrap()
            .0,
        "foo"
    );
    assert_eq!(
        glib::Value::from("foo")
            .get::<Option<TestNullable>>()
            .unwrap()
            .unwrap()
            .0,
        "foo"
    );
    assert_eq!(
        glib::Value::from(TestNullable("foo".to_string()))
            .get::<Option<String>>()
            .unwrap()
            .unwrap(),
        "foo"
    );
    // From Option<TestNullable> Some
    assert_eq!(
        glib::Value::from(Some(TestNullable("foo".to_string())))
            .get::<Option<TestNullable>>()
            .unwrap()
            .unwrap()
            .0,
        "foo"
    );
    assert_eq!(
        glib::Value::from(Some("foo"))
            .get::<Option<TestNullable>>()
            .unwrap()
            .unwrap()
            .0,
        "foo"
    );
    assert_eq!(
        glib::Value::from(Some(TestNullable("foo".to_string())))
            .get::<Option<String>>()
            .unwrap()
            .unwrap(),
        "foo"
    );
    // From Option<TestNullable> None
    assert!(glib::Value::from(None::<TestNullable>)
        .get::<Option<TestNullable>>()
        .unwrap()
        .is_none());
    assert!(glib::Value::from(None::<String>)
        .get::<Option<TestNullable>>()
        .unwrap()
        .is_none());
    assert!(glib::Value::from(None::<TestNullable>)
        .get::<Option<String>>()
        .unwrap()
        .is_none());
}

#[allow(clippy::unnecessary_literal_unwrap)]
#[test]
fn higher_level_types() {
    #[derive(Debug, ValueDelegate)]
    pub struct MyVec(Vec<String>);

    #[derive(Debug, ValueDelegate)]
    #[value_delegate(nullable)]
    pub struct MyString(Box<str>);

    #[derive(Debug, ValueDelegate)]
    #[value_delegate(from = Option<String>)]
    struct MyVecManualFrom(Vec<String>);

    impl From<Option<String>> for MyVecManualFrom {
        fn from(v: Option<String>) -> Self {
            Self(v.into_iter().collect::<Vec<_>>())
        }
    }
    impl<'a> From<&'a MyVecManualFrom> for Option<String> {
        fn from(v: &'a MyVecManualFrom) -> Self {
            v.0.first().cloned()
        }
    }
    impl From<MyVecManualFrom> for Option<String> {
        fn from(v: MyVecManualFrom) -> Self {
            v.0.into_iter().next()
        }
    }

    let vec = vec!["foo".to_string(), "bar".to_string()];
    let vec_value = vec.to_value();
    let my_vec_value = MyVec(vec).to_value();
    assert_eq!(MyVec::static_type(), Vec::<String>::static_type());
    assert_eq!(
        vec_value.get::<Vec<String>>().unwrap(),
        my_vec_value.get::<Vec<String>>().unwrap(),
    );
    assert_eq!(vec_value.value_type(), my_vec_value.value_type());
    assert_eq!(unsafe { Vec::<String>::from_value(&vec_value) }, unsafe {
        MyVec::from_value(&vec_value).0
    });
    assert_eq!(
        unsafe { Vec::<String>::from_value(&my_vec_value) },
        unsafe { MyVec::from_value(&my_vec_value).0 }
    );

    let string = "foo".to_string();
    let string_value = string.to_value();
    let my_string_value = MyString(string.into()).to_value();
    assert_eq!(MyString::static_type(), Box::<str>::static_type());
    assert_eq!(
        string_value.get::<Box<str>>().unwrap(),
        my_string_value.get::<Box<str>>().unwrap(),
    );
    assert_eq!(string_value.value_type(), my_string_value.value_type());
    assert_eq!(unsafe { Box::<str>::from_value(&string_value) }, unsafe {
        MyString::from_value(&string_value).0
    });
    assert_eq!(
        unsafe { Box::<str>::from_value(&my_string_value) },
        unsafe { MyString::from_value(&my_string_value).0 }
    );

    let string_some = Some("foo".to_string());
    let string_some_value = string_some.to_value();
    let string_none_value = None::<String>.to_value();
    let my_string_some_value = MyString(string_some.unwrap().into()).to_value();
    let my_string_none_value = None::<MyString>.to_value();
    assert_eq!(
        Option::<MyString>::static_type(),
        Option::<Box<str>>::static_type()
    );
    assert_eq!(
        string_some_value
            .get::<Option<Box<str>>>()
            .unwrap()
            .unwrap(),
        my_string_some_value
            .get::<Option<Box<str>>>()
            .unwrap()
            .unwrap(),
    );
    assert_eq!(
        string_none_value
            .get::<Option<Box<str>>>()
            .unwrap()
            .is_none(),
        my_string_none_value
            .get::<Option<Box<str>>>()
            .unwrap()
            .is_none(),
    );
    assert_eq!(
        string_some_value.value_type(),
        my_string_some_value.value_type()
    );
    assert_eq!(
        string_none_value.value_type(),
        my_string_none_value.value_type()
    );
    assert_eq!(
        unsafe { Option::<Box<str>>::from_value(&string_some_value).unwrap() },
        unsafe {
            Option::<MyString>::from_value(&string_some_value)
                .unwrap()
                .0
        }
    );
    assert_eq!(
        unsafe { Option::<Box<str>>::from_value(&string_none_value).is_none() },
        unsafe { Option::<MyString>::from_value(&string_none_value).is_none() }
    );
    assert_eq!(
        unsafe { Option::<Box<str>>::from_value(&my_string_some_value).unwrap() },
        unsafe {
            Option::<MyString>::from_value(&my_string_some_value)
                .unwrap()
                .0
        }
    );
    assert_eq!(
        unsafe { Option::<Box<str>>::from_value(&my_string_none_value).is_none() },
        unsafe { Option::<MyString>::from_value(&my_string_none_value).is_none() }
    );

    let opt = Some("foo".to_string());
    let opt_value = opt.to_value();
    let my_vec_manual_from_value = MyVecManualFrom::from(opt).to_value();
    assert_eq!(
        MyVecManualFrom::static_type(),
        Option::<String>::static_type()
    );
    assert_eq!(
        opt_value.get::<Option<String>>().unwrap(),
        my_vec_manual_from_value.get::<Option<String>>().unwrap(),
    );
    assert_eq!(
        opt_value.value_type(),
        my_vec_manual_from_value.value_type()
    );
    assert_eq!(
        unsafe {
            Option::<String>::from_value(&opt_value)
                .into_iter()
                .collect::<Vec<_>>()
        },
        unsafe { MyVecManualFrom::from_value(&opt_value).0 }
    );
    assert_eq!(
        unsafe {
            Option::<String>::from_value(&my_vec_manual_from_value)
                .into_iter()
                .collect::<Vec<_>>()
        },
        unsafe { MyVecManualFrom::from_value(&my_vec_manual_from_value).0 }
    );
}
