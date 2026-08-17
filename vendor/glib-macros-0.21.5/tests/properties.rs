// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::ParamFlags;

#[cfg(test)]
mod base {
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib_macros::Properties;
    use std::marker::PhantomData;

    pub mod imp {
        use super::*;

        #[derive(Properties, Default)]
        #[properties(wrapper_type = super::Base, ext_trait)]
        pub struct Base {
            #[property(get = Self::not_overridden)]
            overridden: PhantomData<u32>,
            #[property(get = Self::not_overridden)]
            not_overridden: PhantomData<u32>,
        }

        #[glib::derived_properties]
        impl ObjectImpl for Base {}

        #[glib::object_subclass]
        impl ObjectSubclass for Base {
            const NAME: &'static str = "MyBase";
            type Type = super::Base;
        }

        impl Base {
            fn not_overridden(&self) -> u32 {
                42
            }
        }
    }

    glib::wrapper! {
        pub struct Base(ObjectSubclass<imp::Base>);
    }

    unsafe impl<T: ObjectImpl> IsSubclassable<T> for Base {}
}

#[cfg(test)]
mod foo {
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib_macros::{Properties, ValueDelegate};
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::marker::PhantomData;
    use std::sync::Mutex;

    use super::base::Base;

    #[derive(ValueDelegate, Default, Debug, PartialEq)]
    pub struct MyPropertyValue(pub i32);

    #[derive(Clone, Default, Debug, PartialEq, Eq, glib::Boxed)]
    #[boxed_type(name = "SimpleBoxedString")]
    pub struct SimpleBoxedString(pub String);

    #[derive(Copy, Default, Clone, Debug, PartialEq, Eq, glib::Enum)]
    #[enum_type(name = "SimpleEnum")]
    pub enum SimpleEnum {
        #[default]
        One,
        Two,
    }

    #[derive(Default, Clone)]
    struct Author {
        name: String,
        nick: String,
    }

    pub mod imp {
        use std::{cell::OnceCell, rc::Rc};

        use super::*;

        #[derive(Properties, Default)]
        #[properties(wrapper_type = super::Foo)]
        pub struct Foo {
            #[property(get, set)]
            bar: Mutex<String>,
            #[property(get, set)]
            double: RefCell<f64>,
            #[property(get, set)]
            string_vec: RefCell<Vec<String>>,
            #[property(get, set, builder(glib::VariantTy::DOUBLE))]
            variant: RefCell<Option<glib::Variant>>,
            #[property(get, set, builder(&<Option<i32>>::static_variant_type()))]
            variant2: RefCell<Option<glib::Variant>>,
            #[property(get = |_| 42.0, set)]
            infer_inline_type: RefCell<f64>,
            // The following property doesn't store any data. The value of the property is calculated
            // when the value is accessed.
            #[property(get = Self::hello_world)]
            _buzz: PhantomData<String>,
            #[property(get, set)]
            my_property_value: RefCell<MyPropertyValue>,
            #[property(get, set = Self::set_fizz, name = "fizz", nick = "fizz-nick",
                blurb = "short description stored in the GLib type system"
            )]
            fizz: RefCell<String>,
            #[property(name = "author-name", get, set, type = String, member = name)]
            #[property(name = "author-nick", get, set, type = String, member = nick)]
            author: RefCell<Author>,
            #[property(
                type = String,
                get = |t: &Self| t.author.borrow().name.to_owned(),
                set = Self::set_author_name)]
            fake_field: PhantomData<String>,
            #[property(get)]
            read_only_text: String,
            #[property(get, set, explicit_notify, lax_validation)]
            custom_flags: RefCell<String>,
            #[property(get, set, default = "hello")]
            with_default: RefCell<String>,
            #[property(get, set, builder())]
            simple_builder: RefCell<u32>,
            #[property(get, set, builder().minimum(0).maximum(5))]
            numeric_builder: RefCell<u32>,
            #[property(get, set, minimum = 0, maximum = 5)]
            builder_fields_without_builder: RefCell<u32>,
            #[property(get, set, builder('c'))]
            builder_with_required_param: RefCell<char>,
            #[property(get, set, default)]
            char_default: RefCell<char>,
            #[property(get, set)]
            boxed: RefCell<SimpleBoxedString>,
            #[property(get, set, builder(SimpleEnum::Two))]
            fenum: RefCell<SimpleEnum>,
            #[property(get, set, default)]
            fenum_default: RefCell<SimpleEnum>,
            #[property(get, set, nullable)]
            object: RefCell<Option<glib::Object>>,
            #[property(get, set, nullable)]
            optional: RefCell<Option<String>>,
            #[property(get, set)]
            smart_pointer: Rc<RefCell<String>>,
            #[property(get, set)]
            once_cell: OnceCell<u8>,
            #[property(get, set)]
            cell: Cell<u8>,
            #[property(get = Self::overridden, override_class = Base)]
            overridden: PhantomData<u32>,
            #[property(get, set)]
            weak_ref_prop: glib::WeakRef<glib::Object>,
            #[property(get, set)]
            send_weak_ref_prop: glib::SendWeakRef<glib::Object>,
            #[property(get, default_value = 0, construct_only)]
            construct_only_cell: OnceCell<u32>,
            #[property(get, set = Self::set_construct_only_custom, construct_only)]
            construct_only_custom_setter: OnceCell<Option<String>>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for Foo {
            const NAME: &'static str = "MyFoo";
            type Type = super::Foo;
            type ParentType = Base;
        }

        #[glib::derived_properties]
        impl ObjectImpl for Foo {}

        impl Foo {
            fn set_author_name(&self, value: String) {
                self.author.borrow_mut().name = value;
            }
            fn hello_world(&self) -> String {
                String::from("Hello world!")
            }
            fn set_fizz(&self, value: String) {
                *self.fizz.borrow_mut() = format!("custom set: {value}");
            }
            fn overridden(&self) -> u32 {
                43
            }
            fn set_construct_only_custom(&self, value: Option<String>) {
                self.construct_only_custom_setter
                    .set(value.map(|v| format!("custom set: {v}")))
                    .expect("Setter to be only called once");
            }
        }

        /// Checks that `Properties` does not pollute the scope with
        /// trait imports, as it did in older versions.
        #[test]
        fn no_import_leaks() {
            // `vec.get` can match these methods (in order of precedence):
            // (1) `<Vec<String> as PropertyGet>::get`
            // (2) `<[String]>::get` through deref of `Vec<String>`
            // Had the macro introduced `PropertyGet` into the scope, it would
            // resolve to (1), which we do not want.
            let vec: Vec<String> = vec![String::new(); 2];
            assert_eq!(vec.get(1), Some(&String::new()));
        }
    }

    glib::wrapper! {
        pub struct Foo(ObjectSubclass<imp::Foo>) @extends Base;
    }
}

#[test]
fn props() {
    use crate::foo::SimpleEnum;

    let myfoo: foo::Foo = glib::object::Object::new();

    // Read values
    let bar: String = myfoo.property("bar");
    assert_eq!(bar, "".to_string());
    let string_vec: Vec<String> = myfoo.property("string-vec");
    assert!(string_vec.is_empty());
    let my_property_value: foo::MyPropertyValue = myfoo.property("my-property-value");
    assert_eq!(my_property_value, foo::MyPropertyValue(0));
    let var: Option<glib::Variant> = myfoo.property("variant");
    assert!(var.is_none());

    // Set values
    myfoo.set_property("bar", "epic".to_value());
    let bar: String = myfoo.property("bar");
    assert_eq!(bar, "epic".to_string());
    myfoo.set_property("string-vec", ["epic", "more epic"].to_value());
    let string_vec: Vec<String> = myfoo.property("string-vec");
    assert_eq!(
        string_vec,
        vec!["epic".to_string(), "more epic".to_string()]
    );
    let myv = Some(2.0f64.to_variant());
    myfoo.set_property("variant", &myv);
    let var: Option<glib::Variant> = myfoo.property("variant");
    assert_eq!(var, myv);

    // Custom getter
    let buzz: String = myfoo.property("buzz");
    assert_eq!(buzz, "Hello world!".to_string());

    // Custom setter
    myfoo.set_property("fizz", "test");
    let fizz: String = myfoo.property("fizz");
    assert_eq!(fizz, "custom set: test".to_string());

    // Multiple props on the same field
    myfoo.set_property("author-name", "freddy".to_value());
    let author_name: String = myfoo.property("author-name");
    assert_eq!(author_name, "freddy".to_string());

    myfoo.set_property("author-nick", "freddy-nick".to_value());
    let author_name: String = myfoo.property("author-nick");
    assert_eq!(author_name, "freddy-nick".to_string());

    // read_only
    assert_eq!(
        myfoo.find_property("read_only_text").unwrap().flags(),
        ParamFlags::READABLE
    );

    // custom flags
    assert_eq!(
        myfoo.find_property("custom_flags").unwrap().flags(),
        ParamFlags::EXPLICIT_NOTIFY | ParamFlags::READWRITE | ParamFlags::LAX_VALIDATION
    );

    // default value
    assert_eq!(
        myfoo
            .find_property("with_default")
            .unwrap()
            .default_value()
            .get::<String>()
            .unwrap(),
        "hello".to_string()
    );

    assert_eq!(
        myfoo
            .find_property("fenum")
            .unwrap()
            .default_value()
            .get::<SimpleEnum>()
            .unwrap(),
        SimpleEnum::Two
    );
    assert_eq!(
        myfoo
            .find_property("fenum_default")
            .unwrap()
            .default_value()
            .get::<SimpleEnum>()
            .unwrap(),
        SimpleEnum::One
    );

    // numeric builder
    assert_eq!(
        myfoo
            .find_property("numeric_builder")
            .unwrap()
            .downcast::<glib::ParamSpecUInt>()
            .unwrap()
            .maximum(),
        5
    );

    assert_eq!(
        {
            let spec = myfoo
                .find_property("builder_fields_without_builder")
                .unwrap()
                .downcast::<glib::ParamSpecUInt>()
                .unwrap();
            (spec.minimum(), spec.maximum())
        },
        (0, 5)
    );

    // builder with required param
    assert_eq!(
        myfoo
            .find_property("builder_with_required_param")
            .unwrap()
            .default_value()
            .get::<char>()
            .unwrap(),
        'c'
    );

    // boxed type
    assert_eq!(
        myfoo.property::<foo::SimpleBoxedString>("boxed"),
        foo::SimpleBoxedString("".into())
    );

    // Test `FooPropertiesExt`
    // getters
    {
        // simple
        let bar = myfoo.bar();
        assert_eq!(bar, myfoo.property::<String>("bar"));

        // custom
        let buzz = myfoo.buzz();
        assert_eq!(buzz, myfoo.property::<String>("buzz"));

        // member of struct field
        let author_nick = myfoo.author_nick();
        assert_eq!(author_nick, myfoo.property::<String>("author-nick"));
    }

    // setters
    {
        // simple
        myfoo.set_bar("setter working");
        assert_eq!(
            myfoo.property::<String>("bar"),
            "setter working".to_string()
        );

        myfoo.set_double(0.1);
        assert_eq!(myfoo.property::<f64>("double"), 0.1);

        myfoo.set_infer_inline_type(42.0);
        assert_eq!(myfoo.property::<f64>("infer-inline-type"), 42.0);

        // simple with various String types
        myfoo.set_bar(String::from("setter working"));
        myfoo.set_bar(glib::GString::from("setter working"));
        assert_eq!(
            myfoo.property::<String>("bar"),
            "setter working".to_string()
        );

        // custom
        myfoo.set_fake_field("fake setter");
        assert_eq!(
            myfoo.property::<String>("author-name"),
            "fake setter".to_string()
        );

        // member of struct field
        myfoo.set_author_nick("setter nick");
        assert_eq!(
            myfoo.property::<String>("author-nick"),
            "setter nick".to_string()
        );
    }

    // overrides
    {
        let overridden: u32 = myfoo.property("overridden");
        assert_eq!(overridden, 43);
        let not_overridden: u32 = myfoo.property("not-overridden");
        assert_eq!(not_overridden, 42);
    }

    // optional
    myfoo.set_optional(Some("Hello world"));
    assert_eq!(myfoo.optional(), Some("Hello world".to_string()));
    myfoo.connect_optional_notify(|_| println!("notified"));

    // object subclass
    let myobj = glib::BoxedAnyObject::new("");
    myfoo.set_object(Some(myobj.upcast_ref()));
    assert_eq!(myfoo.object(), Some(myobj.upcast()));

    // construct_only
    let myfoo: foo::Foo = glib::object::Object::builder()
        .property("construct-only-cell", 1u32)
        .build();
    assert_eq!(myfoo.construct_only_cell(), 1u32);

    // construct_only with custom setter
    let myfoo: foo::Foo = glib::object::Object::builder()
        .property("construct-only-custom-setter", "foo")
        .build();
    assert_eq!(
        myfoo.construct_only_custom_setter(),
        Some("custom set: foo".to_owned())
    );
}

#[test]
fn ext_trait() {
    use base::imp::BasePropertiesExt;
    let base: base::Base = glib::object::Object::builder().build();
    assert_eq!(BasePropertiesExt::overridden(&base), 42);

    let foo_obj: foo::Foo = glib::object::Object::builder().build();
    assert_eq!(BasePropertiesExt::overridden(&foo_obj), 43);
    assert_eq!(foo_obj.overridden(), 43);
}

#[test]
fn keyword_propnames() {
    mod kw_names {
        mod imp {

            use std::cell::Cell;

            use glib::{prelude::*, subclass::prelude::*};
            use glib_macros::Properties;

            #[derive(Properties, Default)]
            #[properties(wrapper_type = super::KwNames)]
            pub struct KwNames {
                // Some of the strict keywords
                #[property(get, set)]
                r#loop: Cell<u8>,
                #[property(get, set)]
                r#move: Cell<u8>,
                #[property(get, set)]
                r#type: Cell<u8>,

                // Lexer 2018+ strict keywords
                #[property(get, set)]
                r#async: Cell<u8>,
                #[property(get, set)]
                r#await: Cell<u8>,
                #[property(get, set)]
                r#dyn: Cell<u8>,

                // Some of the reserved keywords
                #[property(get, set)]
                r#become: Cell<u8>,
                #[property(get, set)]
                r#macro: Cell<u8>,
                #[property(get, set)]
                r#unsized: Cell<u8>,

                // Lexer 2018+ reserved keywords
                #[property(get, set)]
                r#try: Cell<u8>,
            }

            #[glib::object_subclass]
            impl ObjectSubclass for KwNames {
                const NAME: &'static str = "MyKwNames";
                type Type = super::KwNames;
            }

            #[glib::derived_properties]
            impl ObjectImpl for KwNames {}
        }

        glib::wrapper! {
            pub struct KwNames(ObjectSubclass<imp::KwNames>);
        }
    }

    let mykwnames: kw_names::KwNames = glib::Object::new();

    // make sure all 10 properties are registered
    assert_eq!(mykwnames.list_properties().len(), 10);

    // getting property values
    assert_eq!(mykwnames.r#loop(), 0);
    assert_eq!(mykwnames.r#move(), 0);
    assert_eq!(mykwnames.r#type(), 0);
    assert_eq!(mykwnames.r#async(), 0);
    assert_eq!(mykwnames.r#await(), 0);
    assert_eq!(mykwnames.r#try(), 0);

    // getting property by name
    assert_eq!(mykwnames.property::<u8>("loop"), 0);
    assert_eq!(mykwnames.property::<u8>("move"), 0);
    assert_eq!(mykwnames.property::<u8>("type"), 0);
    assert_eq!(mykwnames.property::<u8>("async"), 0);
    assert_eq!(mykwnames.property::<u8>("await"), 0);
    assert_eq!(mykwnames.property::<u8>("try"), 0);

    // setting property values
    mykwnames.set_loop(128_u8);
    assert_eq!(mykwnames.r#loop(), 128_u8);
    mykwnames.set_move(128_u8);
    assert_eq!(mykwnames.r#move(), 128_u8);
    mykwnames.set_type(128_u8);
    assert_eq!(mykwnames.r#type(), 128_u8);
    mykwnames.set_async(128_u8);
    assert_eq!(mykwnames.r#async(), 128_u8);
    mykwnames.set_await(128_u8);
    assert_eq!(mykwnames.r#await(), 128_u8);
    mykwnames.set_try(128_u8);
    assert_eq!(mykwnames.r#try(), 128_u8);

    // setting property by name
    mykwnames.set_property("loop", 255_u8);
    assert_eq!(mykwnames.r#loop(), 255_u8);
    mykwnames.set_property("move", 255_u8);
    assert_eq!(mykwnames.r#loop(), 255_u8);
    mykwnames.set_property("type", 255_u8);
    assert_eq!(mykwnames.r#loop(), 255_u8);
    mykwnames.set_property("async", 255_u8);
    assert_eq!(mykwnames.r#async(), 255_u8);
    mykwnames.set_property("await", 255_u8);
    assert_eq!(mykwnames.r#await(), 255_u8);
    mykwnames.set_property("try", 255_u8);
    assert_eq!(mykwnames.r#try(), 255_u8);
}

/// This struct is intentionally left empty.
///
/// Ensure the code compiles even when no properties are specified at all.
/// This is useful for refactoring.
#[test]
#[allow(unreachable_code)]
fn empty_struct() {
    mod empty {
        mod imp {
            use glib::subclass::prelude::*;
            use glib_macros::Properties;

            #[derive(Properties, Default)]
            #[properties(wrapper_type = super::Empty)]
            pub struct Empty {}

            #[glib::object_subclass]
            impl ObjectSubclass for Empty {
                const NAME: &'static str = "Empty";
                type Type = super::Empty;
            }

            #[glib::derived_properties]
            impl ObjectImpl for Empty {}
        }

        glib::wrapper! {
            pub struct Empty(ObjectSubclass<imp::Empty>);
        }
    }
}
