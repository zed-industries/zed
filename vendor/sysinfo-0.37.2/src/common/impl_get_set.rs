// Take a look at the license at the top of the repository in the LICENSE file.

macro_rules! impl_get_set {
    ($ty_name:ident, $name:ident, $with:ident, $without:ident $(, $extra_doc:literal)? $(,)?) => {
        #[doc = concat!("Returns the value of the \"", stringify!($name), "\" refresh kind.")]
        $(#[doc = concat!("
", $extra_doc, "
")])?
        #[doc = concat!("
```
use sysinfo::", stringify!($ty_name), ";

let r = ", stringify!($ty_name), "::nothing();

let r = r.with_", stringify!($name), "();
assert_eq!(r.", stringify!($name), "(), true);

let r = r.without_", stringify!($name), "();
assert_eq!(r.", stringify!($name), "(), false);
```")]
        pub fn $name(&self) -> bool {
            self.$name
        }

        #[doc = concat!("Sets the value of the \"", stringify!($name), "\" refresh kind to `true`.

```
use sysinfo::", stringify!($ty_name), ";

let r = ", stringify!($ty_name), "::nothing();

let r = r.with_", stringify!($name), "();
assert_eq!(r.", stringify!($name), "(), true);
```")]
        #[must_use]
        pub fn $with(mut self) -> Self {
            self.$name = true;
            self
        }

        #[doc = concat!("Sets the value of the \"", stringify!($name), "\" refresh kind to `false`.

```
use sysinfo::", stringify!($ty_name), ";

let r = ", stringify!($ty_name), "::everything();
assert_eq!(r.", stringify!($name), "(), true);

let r = r.without_", stringify!($name), "();
assert_eq!(r.", stringify!($name), "(), false);
```")]
        #[must_use]
        pub fn $without(mut self) -> Self {
            self.$name = false;
            self
        }
    };

    // To handle `UpdateKind`.
    ($ty_name:ident, $name:ident, $with:ident, $without:ident, UpdateKind $(, $extra_doc:literal)? $(,)?) => {
        #[doc = concat!("Returns the value of the \"", stringify!($name), "\" refresh kind.")]
        $(#[doc = concat!("
", $extra_doc, "
")])?
        #[doc = concat!("
```
use sysinfo::{", stringify!($ty_name), ", UpdateKind};

let r = ", stringify!($ty_name), "::nothing();
assert_eq!(r.", stringify!($name), "(), UpdateKind::Never);

let r = r.with_", stringify!($name), "(UpdateKind::OnlyIfNotSet);
assert_eq!(r.", stringify!($name), "(), UpdateKind::OnlyIfNotSet);

let r = r.without_", stringify!($name), "();
assert_eq!(r.", stringify!($name), "(), UpdateKind::Never);
```")]
        pub fn $name(&self) -> UpdateKind {
            self.$name
        }

        #[doc = concat!("Sets the value of the \"", stringify!($name), "\" refresh kind.

```
use sysinfo::{", stringify!($ty_name), ", UpdateKind};

let r = ", stringify!($ty_name), "::nothing();
assert_eq!(r.", stringify!($name), "(), UpdateKind::Never);

let r = r.with_", stringify!($name), "(UpdateKind::OnlyIfNotSet);
assert_eq!(r.", stringify!($name), "(), UpdateKind::OnlyIfNotSet);
```")]
        #[must_use]
        pub fn $with(mut self, kind: UpdateKind) -> Self {
            self.$name = kind;
            self
        }

        #[doc = concat!("Sets the value of the \"", stringify!($name), "\" refresh kind to `UpdateKind::Never`.

```
use sysinfo::{", stringify!($ty_name), ", UpdateKind};

let r = ", stringify!($ty_name), "::everything();
assert_eq!(r.", stringify!($name), "(), UpdateKind::OnlyIfNotSet);

let r = r.without_", stringify!($name), "();
assert_eq!(r.", stringify!($name), "(), UpdateKind::Never);
```")]
        #[must_use]
        pub fn $without(mut self) -> Self {
            self.$name = UpdateKind::Never;
            self
        }
    };

    // To handle `*RefreshKind`.
    ($ty_name:ident, $name:ident, $with:ident, $without:ident, $typ:ty $(,)?) => {
        #[doc = concat!("Returns the value of the \"", stringify!($name), "\" refresh kind.

```
use sysinfo::{", stringify!($ty_name), ", ", stringify!($typ), "};

let r = ", stringify!($ty_name), "::nothing();
assert_eq!(r.", stringify!($name), "().is_some(), false);

let r = r.with_", stringify!($name), "(", stringify!($typ), "::everything());
assert_eq!(r.", stringify!($name), "().is_some(), true);

let r = r.without_", stringify!($name), "();
assert_eq!(r.", stringify!($name), "().is_some(), false);
```")]
        pub fn $name(&self) -> Option<$typ> {
            self.$name
        }

        #[doc = concat!("Sets the value of the \"", stringify!($name), "\" refresh kind to `Some(...)`.

```
use sysinfo::{", stringify!($ty_name), ", ", stringify!($typ), "};

let r = ", stringify!($ty_name), "::nothing();
assert_eq!(r.", stringify!($name), "().is_some(), false);

let r = r.with_", stringify!($name), "(", stringify!($typ), "::everything());
assert_eq!(r.", stringify!($name), "().is_some(), true);
```")]
        #[must_use]
        pub fn $with(mut self, kind: $typ) -> Self {
            self.$name = Some(kind);
            self
        }

        #[doc = concat!("Sets the value of the \"", stringify!($name), "\" refresh kind to `None`.

```
use sysinfo::", stringify!($ty_name), ";

let r = ", stringify!($ty_name), "::everything();
assert_eq!(r.", stringify!($name), "().is_some(), true);

let r = r.without_", stringify!($name), "();
assert_eq!(r.", stringify!($name), "().is_some(), false);
```")]
        #[must_use]
        pub fn $without(mut self) -> Self {
            self.$name = None;
            self
        }
    };
}

pub(crate) use impl_get_set;
