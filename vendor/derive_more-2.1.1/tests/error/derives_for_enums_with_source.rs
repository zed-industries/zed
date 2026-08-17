#![allow(dead_code)] // some code is tested for type checking only

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use super::*;

type RenamedOption<T> = Option<T>;

derive_display!(TestErr);
#[derive(Debug, Error)]
enum TestErr {
    Unit,
    NamedImplicitNoSource {
        field: i32,
    },
    NamedImplicitSource {
        source: SimpleErr,
        field: i32,
    },
    NamedImplicitBoxedSource {
        source: Box<dyn Error + Send + 'static>,
        field: i32,
    },
    NamedImplicitOptionalSource {
        source: Option<SimpleErr>,
        field: i32,
    },
    NamedImplicitOptionalBoxedSource {
        source: Option<Box<dyn Error + Send + 'static>>,
        field: i32,
    },
    NamedExplicitNoSource {
        #[error(not(source))]
        source: SimpleErr,
        field: i32,
    },
    NamedExplicitSource {
        #[error(source)]
        explicit_source: SimpleErr,
        field: i32,
    },
    NamedExplicitOptionalSource {
        #[error(source)]
        explicit_source: Option<SimpleErr>,
        field: i32,
    },
    NamedExplicitRenamedOptionalSource {
        #[error(source(optional))]
        explicit_source: RenamedOption<SimpleErr>,
        field: i32,
    },
    NamedExplicitRenamedOptionalBoxedSource {
        #[error(source(optional))]
        explicit_source: RenamedOption<Box<dyn Error + Send + 'static>>,
        field: i32,
    },
    NamedExplicitNoSourceRedundant {
        #[error(not(source))]
        field: i32,
    },
    NamedExplicitSourceRedundant {
        #[error(source)]
        source: SimpleErr,
        field: i32,
    },
    NamedExplicitSuppressesImplicit {
        source: i32,
        #[error(source)]
        field: SimpleErr,
    },
    NamedExplicitOptionalSuppressesImplicit {
        source: i32,
        #[error(source)]
        field: Option<SimpleErr>,
    },
    UnnamedImplicitNoSource(i32, i32),
    UnnamedImplicitSource(SimpleErr),
    UnnamedImplicitOptionalSource(Option<SimpleErr>),
    UnnamedExplicitNoSource(#[error(not(source))] SimpleErr),
    UnnamedExplicitSource(#[error(source)] SimpleErr, i32),
    UnnamedExplicitOptionalSource(#[error(source)] Option<SimpleErr>, i32),
    UnnamedExplicitRenamedOptionalSource(
        #[error(source(optional))] RenamedOption<SimpleErr>,
        i32,
    ),
    UnnamedExplicitRenamedOptionalBoxedSource(
        #[error(source(optional))] RenamedOption<Box<dyn Error + Send + 'static>>,
        i32,
    ),
    UnnamedExplicitNoSourceRedundant(
        #[error(not(source))] i32,
        #[error(not(source))] i32,
    ),
    UnnamedExplicitSourceRedundant(#[error(source)] SimpleErr),
    NamedIgnore {
        #[error(ignore)]
        source: SimpleErr,
        field: i32,
    },
    UnnamedIgnore(#[error(ignore)] SimpleErr),
    NamedIgnoreRedundant {
        #[error(ignore)]
        field: i32,
    },
    UnnamedIgnoreRedundant(#[error(ignore)] i32, #[error(ignore)] i32),
    #[error(ignore)]
    NamedVariantIgnore {
        source: SimpleErr,
        field: i32,
    },
    #[error(ignore)]
    UnnamedVariantIgnore(SimpleErr),
    #[error(ignore)]
    NamedVariantIgnoreRedundant {
        field: i32,
    },
    #[error(ignore)]
    UnnamedVariantIgnoreRedundant(i32, i32),
}

#[test]
fn unit() {
    assert!(TestErr::Unit.source().is_none());
}

#[test]
fn named_implicit_no_source() {
    let err = TestErr::NamedImplicitNoSource { field: 0 };

    assert!(err.source().is_none());
}

#[test]
fn named_implicit_source() {
    let err = TestErr::NamedImplicitSource {
        source: SimpleErr,
        field: 0,
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_implicit_optional_source() {
    let err = TestErr::NamedImplicitOptionalSource {
        source: Some(SimpleErr),
        field: 0,
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_implicit_boxed_source() {
    let err = TestErr::NamedImplicitBoxedSource {
        source: Box::new(SimpleErr),
        field: 0,
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_implicit_optional_boxed_source() {
    let err = TestErr::NamedImplicitOptionalBoxedSource {
        source: Some(Box::new(SimpleErr)),
        field: 0,
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_no_source() {
    let err = TestErr::NamedExplicitNoSource {
        source: SimpleErr,
        field: 0,
    };

    assert!(err.source().is_none());
}

#[test]
fn named_explicit_source() {
    let err = TestErr::NamedExplicitSource {
        explicit_source: SimpleErr,
        field: 0,
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_optional_source() {
    let err = TestErr::NamedExplicitOptionalSource {
        explicit_source: Some(SimpleErr),
        field: 0,
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_renamed_optional_source() {
    let err = TestErr::NamedExplicitRenamedOptionalSource {
        explicit_source: Some(SimpleErr),
        field: 0,
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_renamed_optional_boxed_source() {
    let err = TestErr::NamedExplicitRenamedOptionalBoxedSource {
        explicit_source: Some(Box::new(SimpleErr)),
        field: 0,
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_no_source_redundant() {
    let err = TestErr::NamedExplicitNoSourceRedundant { field: 0 };

    assert!(err.source().is_none());
}

#[test]
fn named_explicit_source_redundant() {
    let err = TestErr::NamedExplicitSourceRedundant {
        source: SimpleErr,
        field: 0,
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_suppresses_implicit() {
    let err = TestErr::NamedExplicitSuppressesImplicit {
        source: 0,
        field: SimpleErr,
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_explicit_optional_suppresses_implicit() {
    let err = TestErr::NamedExplicitOptionalSuppressesImplicit {
        source: 0,
        field: Some(SimpleErr),
    };

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_implicit_no_source() {
    assert!(TestErr::UnnamedImplicitNoSource(0, 0).source().is_none());
}

#[test]
fn unnamed_implicit_source() {
    let err = TestErr::UnnamedImplicitSource(SimpleErr);

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_implicit_optional_source() {
    let err = TestErr::UnnamedImplicitOptionalSource(Some(SimpleErr));

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_explicit_no_source() {
    let err = TestErr::UnnamedExplicitNoSource(SimpleErr);

    assert!(err.source().is_none());
}

#[test]
fn unnamed_explicit_source() {
    let err = TestErr::UnnamedExplicitSource(SimpleErr, 0);

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_explicit_optional_source() {
    let err = TestErr::UnnamedExplicitOptionalSource(Some(SimpleErr), 0);

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_explicit_renamed_optional_source() {
    let err = TestErr::UnnamedExplicitRenamedOptionalSource(Some(SimpleErr), 0);

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_explicit_renamed_optional_boxed_source() {
    let err = TestErr::UnnamedExplicitRenamedOptionalBoxedSource(
        Some(Box::new(SimpleErr)),
        0,
    );

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn unnamed_explicit_no_source_redundant() {
    let err = TestErr::UnnamedExplicitNoSourceRedundant(0, 0);

    assert!(err.source().is_none());
}

#[test]
fn unnamed_explicit_source_redundant() {
    let err = TestErr::UnnamedExplicitSourceRedundant(SimpleErr);

    assert!(err.source().is_some());
    assert!(err.source().unwrap().is::<SimpleErr>());
}

#[test]
fn named_ignore() {
    let err = TestErr::NamedIgnore {
        source: SimpleErr,
        field: 0,
    };

    assert!(err.source().is_none());
}

#[test]
fn unnamed_ignore() {
    let err = TestErr::UnnamedIgnore(SimpleErr);

    assert!(err.source().is_none());
}

#[test]
fn named_ignore_redundant() {
    let err = TestErr::NamedIgnoreRedundant { field: 0 };

    assert!(err.source().is_none());
}

#[test]
fn unnamed_ignore_redundant() {
    let err = TestErr::UnnamedIgnoreRedundant(0, 0);

    assert!(err.source().is_none());
}

#[test]
fn named_variant_ignore() {
    let err = TestErr::NamedVariantIgnore {
        source: SimpleErr,
        field: 0,
    };

    assert!(err.source().is_none());
}

#[test]
fn unnamed_variant_ignore() {
    let err = TestErr::UnnamedVariantIgnore(SimpleErr);

    assert!(err.source().is_none())
}

#[test]
fn named_variant_ignore_redundant() {
    let err = TestErr::NamedVariantIgnoreRedundant { field: 0 };

    assert!(err.source().is_none());
}

#[test]
fn unnamed_variant_ignore_redundant() {
    let err = TestErr::UnnamedVariantIgnoreRedundant(0, 0);

    assert!(err.source().is_none())
}
