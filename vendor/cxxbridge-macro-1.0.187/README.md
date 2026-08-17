This directory contains CXX's Rust code generator, which is a procedural macro.
Users won't depend on this crate directly. Instead they'll invoke its macro
through the reexport in the main `cxx` crate.
