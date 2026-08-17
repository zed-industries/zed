/**
 * \mainpage Wasmtime C/C++ API
 *
 * This documentation is an overview and API reference for the C and C++ API of
 * Wasmtime. The C API is spread between three different header files:
 *
 * * \ref wasmtime.h
 * * \ref wasi.h
 * * \ref wasm.h
 *
 * The C++ API builds on the C API and thus is represented in just header files:
 *
 * * \ref wasmtime.hh
 *
 * The \ref wasmtime.h header file includes all the other header files and is
 * the main header file you'll likely be using. The \ref wasm.h header file
 * comes directly from the
 * [WebAssembly/wasm-c-api](https://github.com/WebAssembly/wasm-c-api)
 * repository, and at this time the upstream header file does not have
 * documentation so Wasmtime provides documentation here. It should be noted
 * some semantics may be Wasmtime-specific and may not be portable to other
 * engines.
 *
 * ## Installing the C API
 *
 * To install the C API from precompiled binaries you can download the
 * appropriate binary from the [releases page of
 * Wasmtime](https://github.com/bytecodealliance/wasmtime/releases). Artifacts
 * for the C API all end in "-c-api" for the filename.
 *
 * Each archive contains an `include` directory with necessary headers, as well
 * as a `lib` directory with both a static archive and a dynamic library of
 * Wasmtime. You can link to either of them as you see fit.
 *
 * ## Installing the C API through CMake
 *
 * CMake can be used to make the process of linking and compiling easier. An
 * example of this if you have wasmtime as a git submodule at
 * `third_party/wasmtime`:
 * ```
 * add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/third_party/wasmtime/crates/c-api
 * ${CMAKE_CURRENT_BINARY_DIR}/wasmtime)
 * ...
 * target_include_directories(YourProject PUBLIC wasmtime)
 * target_link_libraries(YourProject PUBLIC wasmtime)
 * ```
 * `BUILD_SHARED_LIBS` is provided as a define if you would like to build a
 * shared library instead. You must distribute the appropriate shared library
 * for your platform if you do this.
 *
 * ## Linking against the C API
 *
 * You'll want to arrange the `include` directory of the C API to be in your
 * compiler's header path (e.g. the `-I` flag). If you're compiling for Windows
 * and you're using the static library then you'll also need to pass
 * `-DWASM_API_EXTERN=` and `-DWASI_API_EXTERN=` to disable dllimport.
 *
 * Your final artifact can then be linked with `-lwasmtime`. If you're linking
 * against the static library you may need to pass other system libraries
 * depending on your platform:
 *
 * * Linux - `-lpthread -ldl -lm`
 * * macOS - no extra flags needed
 * * Windows - `ws2_32.lib advapi32.lib userenv.lib ntdll.lib shell32.lib
 * ole32.lib bcrypt.lib`
 *
 * ## Building from Source
 *
 * The C API is located in the
 * [`crates/c-api`](https://github.com/bytecodealliance/wasmtime/tree/main/crates/c-api)
 * directory of the [Wasmtime
 * repository](https://github.com/bytecodealliance/wasmtime). To build from
 * source you'll need a Rust compiler and a checkout of the `wasmtime` project.
 * Afterwards you can execute:
 *
 * ```
 * $ cargo build --release -p wasmtime-c-api
 * ```
 *
 * This will place the final artifacts in `target/release`, with names depending
 * on what platform you're compiling for.
 *
 * ## Other resources
 *
 * Some other handy resources you might find useful when exploring the C API
 * documentation are:
 *
 * * [Rust `wasmtime` crate
 *   documentation](https://bytecodealliance.github.io/wasmtime/api/wasmtime/) -
 *   although this documentation is for Rust and not C, you'll find that many
 *   functions mirror one another and there may be extra documentation in Rust
 *   you find helpful. If you find yourself having to frequently do this,
 *   though, please feel free to [file an
 *   issue](https://github.com/bytecodealliance/wasmtime/issues/new).
 *
 * * [C embedding
 *   examples](https://bytecodealliance.github.io/wasmtime/lang-c.html)
 *   are available online and are tested from the Wasmtime repository itself.
 *
 * * [Contribution documentation for
 *   Wasmtime](https://bytecodealliance.github.io/wasmtime/contributing.html) in
 *   case you're interested in helping out!
 */

/**
 * \file wasmtime.h
 *
 * \brief Wasmtime's C API
 *
 * This file is the central inclusion point for Wasmtime's C API. There are a
 * number of sub-header files but this file includes them all. The C API is
 * based on \ref wasm.h but there are many Wasmtime-specific APIs which are
 * tailored to Wasmtime's implementation.
 *
 * The #wasm_config_t and #wasm_engine_t types are used from \ref wasm.h.
 * Additionally all type-level information (like #wasm_functype_t) is also
 * used from \ref wasm.h. Otherwise, though, all wasm objects (like
 * #wasmtime_store_t or #wasmtime_func_t) are used from this header file.
 *
 * ### Thread Safety
 *
 * The multithreading story of the C API very closely follows the
 * multithreading story of the Rust API for Wasmtime. All objects are safe to
 * send to other threads so long as user-specific data is also safe to send to
 * other threads. Functions are safe to call from any thread but some functions
 * cannot be called concurrently. For example, functions which correspond to
 * `&T` in Rust can be called concurrently with any other methods that take
 * `&T`. Functions that take `&mut T` in Rust, however, cannot be called
 * concurrently with any other function (but can still be invoked on any
 * thread).
 *
 * This generally equates to mutation of internal state. Functions which don't
 * mutate anything, such as learning type information through
 * #wasmtime_func_type, can be called concurrently. Functions which do require
 * mutation, for example #wasmtime_func_call, cannot be called concurrently.
 * This is conveyed in the C API with either `const wasmtime_context_t*`
 * (concurrency is ok as it's read-only) or `wasmtime_context_t*` (concurrency
 * is not ok, mutation may happen).
 *
 * When in doubt assume that functions cannot be called concurrently with
 * aliasing objects.
 *
 * ### Aliasing
 *
 * The C API for Wasmtime is intended to be a relatively thin layer over the
 * Rust API for Wasmtime. Rust has much more strict rules about aliasing than C
 * does, and the Rust API for Wasmtime is designed around these rules to be
 * used safely. These same rules must be upheld when using the C API of
 * Wasmtime.
 *
 * The main consequence of this is that the #wasmtime_context_t pointer into
 * the #wasmtime_store_t must be carefully used. Since the context is an
 * internal pointer into the store it must be used carefully to ensure you're
 * not doing something that Rust would otherwise forbid at compile time. A
 * #wasmtime_context_t can only be used when you would otherwise have been
 * provided access to it. For example in a host function created with
 * #wasmtime_func_new you can use #wasmtime_context_t in the host function
 * callback. This is because an argument, a #wasmtime_caller_t, provides access
 * to #wasmtime_context_t.
 *
 * ### Stores
 *
 * A foundational construct in this API is the #wasmtime_store_t. A store is a
 * collection of host-provided objects and instantiated wasm modules. Stores are
 * often treated as a "single unit" and items within a store are all allowed to
 * reference one another. References across stores cannot currently be created.
 * For example you cannot pass a function from one store into another store.
 *
 * A store is not intended to be a global long-lived object. Stores provide no
 * means of internal garbage collections of wasm objects (such as instances),
 * meaning that no memory from a store will be deallocated until you call
 * #wasmtime_store_delete. If you're working with a web server, for example,
 * then it's recommended to think of a store as a "one per request" sort of
 * construct. Globally you'd have one #wasm_engine_t and a cache of
 * #wasmtime_module_t instances compiled into that engine. Each request would
 * create a new #wasmtime_store_t and then instantiate a #wasmtime_module_t
 * into the store. This process of creating a store and instantiating a module
 * is expected to be quite fast. When the request is finished you'd delete the
 * #wasmtime_store_t keeping memory usage reasonable for the lifetime of the
 * server.
 */

#ifndef WASMTIME_API_H
#define WASMTIME_API_H

#include <wasi.h>
#include <wasmtime/conf.h>
// clang-format off
// IWYU pragma: begin_exports
#include <wasmtime/config.h>
#include <wasmtime/engine.h>
#include <wasmtime/error.h>
#include <wasmtime/extern.h>
#include <wasmtime/func.h>
#include <wasmtime/global.h>
#include <wasmtime/instance.h>
#include <wasmtime/linker.h>
#include <wasmtime/memory.h>
#include <wasmtime/module.h>
#include <wasmtime/profiling.h>
#include <wasmtime/sharedmemory.h>
#include <wasmtime/store.h>
#include <wasmtime/table.h>
#include <wasmtime/trap.h>
#include <wasmtime/val.h>
#include <wasmtime/async.h>
#include <wasmtime/component.h>
#include <wasmtime/wasip2.h>
#include <wasmtime/wat.h>
// IWYU pragma: end_exports
// clang-format on

/**
 * \brief Wasmtime version string.
 */
#define WASMTIME_VERSION "36.0.9"
/**
 * \brief Wasmtime major version number.
 */
#define WASMTIME_VERSION_MAJOR 36
/**
 * \brief Wasmtime minor version number.
 */
#define WASMTIME_VERSION_MINOR 0
/**
 * \brief Wasmtime patch version number.
 */
#define WASMTIME_VERSION_PATCH 9

#endif // WASMTIME_API_H
