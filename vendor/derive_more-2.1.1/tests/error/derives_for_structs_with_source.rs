#![allow(dead_code)] // some code is tested for type checking only

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use super::*;

type RenamedOption<T> = Option<T>;

// Asserts that `derive(Error)` macro expansion is hygienic enough to not conflict
// with `.as_dyn_error()` method name.
pub trait ErrorExt {
    fn as_dyn_error<'a>(&self) -> &(dyn core::error::Error + 'a)
    where
        Self: 'a;
}
impl<E: core::error::Error> ErrorExt for E {
    fn as_dyn_error<'a>(&self) -> &(dyn core::error::Error + 'a)
    where
        Self: 'a,
    {
        self
    }
}

#[test]
fn unit() {
    assert!(SimpleErr.source().is_none());
}

#[test]
fn named_implicit_no_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        field: i32,
    }

    assert!(TestErr::default().source().is_none());
}

#[test]
fn named_implicit_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        source: SimpleErr,
        field: i32,
    }

    let err = TestErr::default();

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_implicit_boxed_source() {
    derive_display!(TestErr);
    #[derive(Debug, Error)]
    struct TestErr {
        source: Box<dyn Error + Send + 'static>,
        field: i32,
    }

    let err = TestErr {
        source: Box::new(SimpleErr),
        field: 0,
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_implicit_optional_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        source: Option<SimpleErr>,
        field: i32,
    }

    let err = TestErr {
        source: Some(SimpleErr),
        ..TestErr::default()
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_implicit_optional_boxed_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        source: Option<Box<dyn Error + Send + 'static>>,
        field: i32,
    }

    let err = TestErr {
        source: Some(Box::new(SimpleErr)),
        ..TestErr::default()
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_no_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        #[error(not(source))]
        source: SimpleErr,
        field: i32,
    }

    assert!(TestErr::default().source().is_none());
}

#[test]
fn named_explicit_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        #[error(source)]
        explicit_source: SimpleErr,
        field: i32,
    }

    let err = TestErr::default();

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_boxed_source() {
    derive_display!(TestErr);
    #[derive(Debug, Error)]
    struct TestErr {
        #[error(source)]
        explicit_source: Box<dyn Error + Send + 'static>,
        field: i32,
    }

    let err = TestErr {
        explicit_source: Box::new(SimpleErr),
        field: 0,
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_optional_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        #[error(source)]
        explicit_source: Option<SimpleErr>,
        field: i32,
    }

    let err = TestErr {
        explicit_source: Some(SimpleErr),
        ..TestErr::default()
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_optional_boxed_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        #[error(source)]
        explicit_source: Option<Box<dyn Error + Send + 'static>>,
        field: i32,
    }

    let err = TestErr {
        explicit_source: Some(Box::new(SimpleErr)),
        ..TestErr::default()
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_renamed_optional_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        #[error(source(optional))]
        explicit_source: RenamedOption<SimpleErr>,
        field: i32,
    }

    let err = TestErr {
        explicit_source: Some(SimpleErr),
        ..TestErr::default()
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_renamed_optional_boxed_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        #[error(source(optional))]
        explicit_source: RenamedOption<Box<dyn Error + Send + 'static>>,
        field: i32,
    }

    let err = TestErr {
        explicit_source: Some(Box::new(SimpleErr)),
        ..TestErr::default()
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_no_source_redundant() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        #[error(not(source))]
        field: i32,
    }

    assert!(TestErr::default().source().is_none());
}

#[test]
fn named_explicit_source_redundant() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        #[error(source)]
        source: SimpleErr,
        field: i32,
    }

    let err = TestErr::default();

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_suppresses_implicit() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        source: i32,
        #[error(source)]
        field: SimpleErr,
    }

    let err = TestErr::default();

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_optional_suppresses_implicit() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        source: i32,
        #[error(source)]
        field: Option<SimpleErr>,
    }

    let err = TestErr {
        field: Some(SimpleErr),
        ..TestErr::default()
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_implicit_no_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr(i32, i32);

    assert!(TestErr::default().source().is_none());
}

#[test]
fn unnamed_implicit_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr(SimpleErr);

    let err = TestErr::default();

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_implicit_optional_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr(Option<SimpleErr>);

    let err = TestErr(Some(SimpleErr));

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_explicit_no_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr(#[error(not(source))] SimpleErr);

    assert!(TestErr::default().source().is_none());
}

#[test]
fn unnamed_explicit_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr(#[error(source)] SimpleErr, i32);

    let err = TestErr::default();

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_explicit_optional_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr(#[error(source)] Option<SimpleErr>, i32);

    let err = TestErr {
        0: Some(SimpleErr),
        ..TestErr::default()
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_explicit_renamed_optional_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr(#[error(source(optional))] RenamedOption<SimpleErr>, i32);

    let err = TestErr {
        0: Some(SimpleErr),
        ..TestErr::default()
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_explicit_renamed_optional_boxed_source() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr(
        #[error(source(optional))] RenamedOption<Box<dyn Error + Send + 'static>>,
        i32,
    );

    let err = TestErr {
        0: Some(Box::new(SimpleErr)),
        ..TestErr::default()
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_explicit_no_source_redundant() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr(#[error(not(source))] i32, #[error(not(source))] i32);

    assert!(TestErr::default().source().is_none());
}

#[test]
fn unnamed_explicit_source_redundant() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr(#[error(source)] SimpleErr);

    let err = TestErr::default();

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_ignore() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        #[error(ignore)]
        source: SimpleErr,
        field: i32,
    }

    assert!(TestErr::default().source().is_none());
}

#[test]
fn unnamed_ignore() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr(#[error(ignore)] SimpleErr);

    assert!(TestErr::default().source().is_none());
}

#[test]
fn named_ignore_redundant() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr {
        #[error(ignore)]
        field: i32,
    }

    assert!(TestErr::default().source().is_none());
}

#[test]
fn unnamed_ignore_redundant() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    struct TestErr(#[error(ignore)] i32, #[error(ignore)] i32);

    assert!(TestErr::default().source().is_none());
}

#[test]
fn named_struct_ignore() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    #[error(ignore)]
    struct TestErr {
        source: SimpleErr,
        field: i32,
    }

    assert!(TestErr::default().source().is_none())
}

#[test]
fn unnamed_struct_ignore() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    #[error(ignore)]
    struct TestErr(SimpleErr);

    assert!(TestErr::default().source().is_none())
}

#[test]
fn named_struct_ignore_redundant() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    #[error(ignore)]
    struct TestErr {
        field: i32,
    }

    assert!(TestErr::default().source().is_none())
}

#[test]
fn unnamed_struct_ignore_redundant() {
    derive_display!(TestErr);
    #[derive(Default, Debug, Error)]
    #[error(ignore)]
    struct TestErr(i32, i32);

    assert!(TestErr::default().source().is_none())
}
