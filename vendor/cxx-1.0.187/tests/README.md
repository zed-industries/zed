# Testing

This document provides an outline of different kinds of tests used by the `cxx`
project.

## Errors from proc macro

We want to verify that the `#[cxx::bridge]` macro reports expected error
messages when invoked by `rustc` on certain inputs.

Such verification is handled by test cases underneath **tests/ui** directory and
driven by **tests/compiletest.rs**. The test cases consist of a pair of files:

* **foo.rs** is the input
* **foo.stderr** is the expected Rust compiler diagnostic

## Errors from C++ compiler

We want to verify that cxx's generated C++ code triggers expected C++ compiler
diagnostics on certain inputs.

Such verification is covered by **tests/cpp_ui_tests.rs**.

## End-to-end functionality

End-to-end functional tests are structured as follows:

* The code under test is contained underneath **tests/ffi** directory which
  contains:
    - Rust code under test &emdash; the `cxx-test-suite` crate (**lib.rs** and
      **module.rs**) with:
        - A few `#[cxx::bridge]` invocations
        - Rust types (e.g. `struct R`)
        - Rust functions and methods (e.g. `fn r_return_primitive`)
    - C/C++ code under test (**tests.h** and **tests.cc**)
        - C++ types (e.g. `class C`)
        - C++ functions and methods (e.g. `c_return_primitive`)
* The testcases can be found in:
    - Rust calling into C++: **tests/test.rs**.
    - C++ calling into Rust: **tests/ffi/test.cc**. These tests are manually
      dispatched from the `cxx_run_test` function in **tests/ffi/test.cc**.
