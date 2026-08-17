/**
 * This project is a C++ API for
 * [Wasmtime](https://github.com/bytecodealliance/wasmtime). Support for the
 * C++ API is exclusively built on the [C API of
 * Wasmtime](https://docs.wasmtime.dev/c-api/), so the C++ support for this is
 * just a set of header files. Like the C API the C++ API is split into
 * separate headers to be included on an as-needed basis. Types shouldn't
 * need to use the C API, but if something is missing please feel free to file
 * an issue.
 *
 * Examples can be [found
 * online](https://github.com/bytecodealliance/wasmtime/tree/main/examples)
 * and otherwise be sure to check out the
 * [README](https://github.com/bytecodealliance/wasmtime/blob/main/crates/c-api/README.md)
 * for simple usage instructions. Otherwise you can dive right in to the
 * reference documentation of \ref wasmtime.hh
 *
 * \example hello.cc
 * \example gcd.cc
 * \example linking.cc
 * \example memory.cc
 * \example interrupt.cc
 * \example externref.cc
 */

/**
 * \file wasmtime.hh
 */

#ifndef WASMTIME_HH
#define WASMTIME_HH

#include <wasmtime/config.hh>
#include <wasmtime/engine.hh>
#include <wasmtime/error.hh>
#include <wasmtime/extern.hh>
#include <wasmtime/func.hh>
#include <wasmtime/global.hh>
#include <wasmtime/instance.hh>
#include <wasmtime/linker.hh>
#include <wasmtime/memory.hh>
#include <wasmtime/module.hh>
#include <wasmtime/store.hh>
#include <wasmtime/table.hh>
#include <wasmtime/trap.hh>
#include <wasmtime/types.hh>
#include <wasmtime/val.hh>
#include <wasmtime/wasi.hh>

#endif // WASMTIME_HH
