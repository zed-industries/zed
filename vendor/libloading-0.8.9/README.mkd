# libloading

Bindings around the platform's dynamic library loading primitives with greatly improved memory
safety. The most important safety guarantee of this library is the prevention of dangling `Symbol`s
that may occur after a `Library` is unloaded.

Using this library allows the loading of dynamic libraries, also known as shared libraries, as well
as the use of the functions and static variables that these libraries may contain.

* [Documentation][docs]
* [Changelog][changelog]

[docs]: https://docs.rs/libloading/
[changelog]: https://docs.rs/libloading/*/libloading/changelog/index.html

libloading is available to use under ISC (MIT-like) license.
