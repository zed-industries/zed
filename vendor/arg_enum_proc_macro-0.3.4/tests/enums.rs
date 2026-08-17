use arg_enum_proc_macro::ArgEnum;

#[derive(ArgEnum, PartialEq, Debug)]
pub enum Foo {
    Bar,
    /// Foo
    Baz,
}

#[test]
fn parse() {
    let v: Foo = "Baz".parse().unwrap();

    assert_eq!(v, Foo::Baz);
}

#[test]
fn variants() {
    assert_eq!(&Foo::variants(), &["Bar", "Baz"]);
}

mod alias {
    use arg_enum_proc_macro::ArgEnum;

    #[derive(ArgEnum, PartialEq, Debug)]
    pub enum Bar {
        A,
        B,
        #[arg_enum(alias = "Cat")]
        C,
    }

    #[test]
    fn parse() {
        let v: Bar = "Cat".parse().unwrap();

        assert_eq!(v, Bar::C);
    }

    #[test]
    fn variants() {
        assert_eq!(&Bar::variants(), &["A", "B", "C", "Cat"]);
    }
}

mod name {
    use arg_enum_proc_macro::ArgEnum;

    #[derive(ArgEnum, PartialEq, Debug)]
    pub enum Bar {
        A,
        B,
        #[arg_enum(name = "Cat", alias = "Feline")]
        C,
    }

    #[test]
    fn parse() {
        let v: Bar = "Cat".parse().unwrap();

        assert_eq!(v, Bar::C);
    }

    #[test]
    fn variants() {
        assert_eq!(&Bar::variants(), &["A", "B", "Cat", "Feline"]);
    }
}

mod description {
    use arg_enum_proc_macro::ArgEnum;

    #[derive(ArgEnum, PartialEq, Debug)]
    pub enum Bar {
        /// This is A and it's description is a single line
        A,
        /// This is B and it's description contains " for no specific reason
        /// and is in two lines.
        B,
        /// This is C, normally known as "Cat" or "Feline"
        #[arg_enum(name = "Cat", alias = "Feline")]
        C,
    }

    #[test]
    fn descriptions() {
        let expected: [(&'static [&'static str], &'static [&'static str]); 3usize] = [
            (
                &["A"],
                &[" This is A and it's description is a single line"],
            ),
            (
                &["B"],
                &[
                    " This is B and it's description contains \" for no specific reason",
                    " and is in two lines.",
                ],
            ),
            (
                &["Cat", "Feline"],
                &[" This is C, normally known as \"Cat\" or \"Feline\""],
            ),
        ];
        assert_eq!(&Bar::descriptions(), &expected);
    }
}

mod ui {
    #[test]
    fn invalid_applications() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/complex-enum.rs");
        t.compile_fail("tests/ui/derive-not-on-enum.rs");
    }
}
