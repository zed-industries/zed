/**
 * \file wasmtime/func.hh
 */

#ifndef WASMTIME_FUNC_HH
#define WASMTIME_FUNC_HH

#include <array>
#include <wasmtime/error.hh>
#include <wasmtime/extern_declare.hh>
#include <wasmtime/func.h>
#include <wasmtime/span.hh>
#include <wasmtime/store.hh>
#include <wasmtime/trap.hh>
#include <wasmtime/types/func.hh>
#include <wasmtime/types/val.hh>
#include <wasmtime/val.hh>

namespace wasmtime {

/**
 * \brief Structure provided to host functions to lookup caller information or
 * acquire a `Store::Context`.
 *
 * This structure is passed to all host functions created with `Func`. It can be
 * used to create a `Store::Context`.
 */
class Caller {
  friend class Func;
  friend class Store;
  wasmtime_caller_t *ptr;
  Caller(wasmtime_caller_t *ptr) : ptr(ptr) {}

public:
  /// Attempts to load an exported item from the calling instance.
  ///
  /// For more information see the Rust documentation -
  /// https://docs.wasmtime.dev/api/wasmtime/struct.Caller.html#method.get_export
  std::optional<Extern> get_export(std::string_view name);

  /// Explicitly acquire a `Store::Context` from this `Caller`.
  Store::Context context() { return this; }
};

inline Store::Context::Context(Caller &caller)
    : Context(wasmtime_caller_context(caller.ptr)) {}
inline Store::Context::Context(Caller *caller) : Context(*caller) {}

namespace detail {

/// A "trait" for native types that correspond to WebAssembly types for use with
/// `Func::wrap` and `TypedFunc::call`
template <typename T> struct WasmType {
  static const bool valid = false;
};

/// Helper macro to define `WasmType` definitions for primitive types like
/// int32_t and such.
// NOLINTNEXTLINE
#define NATIVE_WASM_TYPE(native, valkind, field)                               \
  template <> struct WasmType<native> {                                        \
    static const bool valid = true;                                            \
    static const ValKind kind = ValKind::valkind;                              \
    static void store(Store::Context cx, wasmtime_val_raw_t *p,                \
                      const native &t) {                                       \
      p->field = t;                                                            \
    }                                                                          \
    static native load(Store::Context cx, wasmtime_val_raw_t *p) {             \
      return p->field;                                                         \
    }                                                                          \
  };

NATIVE_WASM_TYPE(int32_t, I32, i32)
NATIVE_WASM_TYPE(uint32_t, I32, i32)
NATIVE_WASM_TYPE(int64_t, I64, i64)
NATIVE_WASM_TYPE(uint64_t, I64, i64)
NATIVE_WASM_TYPE(float, F32, f32)
NATIVE_WASM_TYPE(double, F64, f64)

#undef NATIVE_WASM_TYPE

/// Type information for `externref`, represented on the host as an optional
/// `ExternRef`.
template <> struct WasmType<std::optional<ExternRef>> {
  static const bool valid = true;
  static const ValKind kind = ValKind::ExternRef;
  static void store(Store::Context cx, wasmtime_val_raw_t *p,
                    const std::optional<ExternRef> &ref) {
    if (ref) {
      p->externref = wasmtime_externref_to_raw(cx.raw_context(), ref->raw());
    } else {
      p->externref = 0;
    }
  }
  static std::optional<ExternRef> load(Store::Context cx,
                                       wasmtime_val_raw_t *p) {
    if (p->externref == 0) {
      return std::nullopt;
    }
    wasmtime_externref_t val;
    wasmtime_externref_from_raw(cx.raw_context(), p->externref, &val);
    return ExternRef(val);
  }
};

/// Type information for the `V128` host value used as a wasm value.
template <> struct WasmType<V128> {
  static const bool valid = true;
  static const ValKind kind = ValKind::V128;
  static void store(Store::Context cx, wasmtime_val_raw_t *p, const V128 &t) {
    memcpy(&p->v128[0], &t.v128[0], sizeof(wasmtime_v128));
  }
  static V128 load(Store::Context cx, wasmtime_val_raw_t *p) { return p->v128; }
};

/// A "trait" for a list of types and operations on them, used for `Func::wrap`
/// and `TypedFunc::call`
///
/// The base case is a single type which is a list of one element.
template <typename T> struct WasmTypeList {
  static const bool valid = WasmType<T>::valid;
  static const size_t size = 1;
  static bool matches(ValType::ListRef types) {
    return WasmTypeList<std::tuple<T>>::matches(types);
  }
  static void store(Store::Context cx, wasmtime_val_raw_t *storage,
                    const T &t) {
    WasmType<T>::store(cx, storage, t);
  }
  static T load(Store::Context cx, wasmtime_val_raw_t *storage) {
    return WasmType<T>::load(cx, storage);
  }
  static std::vector<ValType> types() { return {WasmType<T>::kind}; }
};

/// std::monostate translates to an empty list of types.
template <> struct WasmTypeList<std::monostate> {
  static const bool valid = true;
  static const size_t size = 0;
  static bool matches(ValType::ListRef types) { return types.size() == 0; }
  static void store(Store::Context cx, wasmtime_val_raw_t *storage,
                    const std::monostate &t) {}
  static std::monostate load(Store::Context cx, wasmtime_val_raw_t *storage) {
    return std::monostate();
  }
  static std::vector<ValType> types() { return {}; }
};

/// std::tuple<> translates to the corresponding list of types
template <typename... T> struct WasmTypeList<std::tuple<T...>> {
  static const bool valid = (WasmType<T>::valid && ...);
  static const size_t size = sizeof...(T);
  static bool matches(ValType::ListRef types) {
    if (types.size() != size) {
      return false;
    }
    size_t n = 0;
    return ((WasmType<T>::kind == types.begin()[n++].kind()) && ...);
  }
  static void store(Store::Context cx, wasmtime_val_raw_t *storage,
                    const std::tuple<T...> &t) {
    size_t n = 0;
    std::apply(
        [&](const auto &...val) {
          (WasmType<T>::store(cx, &storage[n++], val), ...); // NOLINT
        },
        t);
  }
  static std::tuple<T...> load(Store::Context cx, wasmtime_val_raw_t *storage) {
    size_t n = 0;
    return std::tuple<T...>{WasmType<T>::load(cx, &storage[n++])...}; // NOLINT
  }
  static std::vector<ValType> types() { return {WasmType<T>::kind...}; }
};

/// A "trait" for what can be returned from closures specified to `Func::wrap`.
///
/// The base case here is a bare return value like `int32_t`.
template <typename R> struct WasmHostRet {
  using Results = WasmTypeList<R>;

  template <typename F, typename... A>
  static std::optional<Trap> invoke(F f, Caller cx, wasmtime_val_raw_t *raw,
                                    A... args) {
    auto ret = f(args...);
    Results::store(cx, raw, ret);
    return std::nullopt;
  }
};

/// Host functions can return nothing
template <> struct WasmHostRet<void> {
  using Results = WasmTypeList<std::tuple<>>;

  template <typename F, typename... A>
  static std::optional<Trap> invoke(F f, Caller cx, wasmtime_val_raw_t *raw,
                                    A... args) {
    f(args...);
    return std::nullopt;
  }
};

// Alternative method of returning "nothing" (also enables `std::monostate` in
// the `R` type of `Result` below)
template <> struct WasmHostRet<std::monostate> : public WasmHostRet<void> {};

/// Host functions can return a result which allows them to also possibly return
/// a trap.
template <typename R> struct WasmHostRet<Result<R, Trap>> {
  using Results = WasmTypeList<R>;

  template <typename F, typename... A>
  static std::optional<Trap> invoke(F f, Caller cx, wasmtime_val_raw_t *raw,
                                    A... args) {
    Result<R, Trap> ret = f(args...);
    if (!ret) {
      return ret.err();
    }
    Results::store(cx, raw, ret.ok());
    return std::nullopt;
  }
};

template <typename F, typename = void> struct WasmHostFunc;

/// Base type information for host free-function pointers being used as wasm
/// functions
template <typename R, typename... A> struct WasmHostFunc<R (*)(A...)> {
  using Params = WasmTypeList<std::tuple<A...>>;
  using Results = typename WasmHostRet<R>::Results;

  template <typename F>
  static std::optional<Trap> invoke(F &f, Caller cx, wasmtime_val_raw_t *raw) {
    auto params = Params::load(cx, raw);
    return std::apply(
        [&](const auto &...val) {
          return WasmHostRet<R>::invoke(f, cx, raw, val...);
        },
        params);
  }
};

/// Function type information, but with a `Caller` first parameter
template <typename R, typename... A>
struct WasmHostFunc<R (*)(Caller, A...)> : public WasmHostFunc<R (*)(A...)> {
  // Override `invoke` here to pass the `cx` as the first parameter
  template <typename F>
  static std::optional<Trap> invoke(F &f, Caller cx, wasmtime_val_raw_t *raw) {
    auto params = WasmTypeList<std::tuple<A...>>::load(cx, raw);
    return std::apply(
        [&](const auto &...val) {
          return WasmHostRet<R>::invoke(f, cx, raw, cx, val...);
        },
        params);
  }
};

/// Function type information, but with a class method.
template <typename R, typename C, typename... A>
struct WasmHostFunc<R (C::*)(A...)> : public WasmHostFunc<R (*)(A...)> {};

/// Function type information, but with a const class method.
template <typename R, typename C, typename... A>
struct WasmHostFunc<R (C::*)(A...) const> : public WasmHostFunc<R (*)(A...)> {};

/// Function type information, but as a host method with a `Caller` first
/// parameter.
template <typename R, typename C, typename... A>
struct WasmHostFunc<R (C::*)(Caller, A...)>
    : public WasmHostFunc<R (*)(Caller, A...)> {};

/// Function type information, but as a host const method with a `Caller`
/// first parameter.
template <typename R, typename C, typename... A>
struct WasmHostFunc<R (C::*)(Caller, A...) const>
    : public WasmHostFunc<R (*)(Caller, A...)> {};

/// Base type information for host callables being used as wasm
/// functions
template <typename T>
struct WasmHostFunc<T, std::void_t<decltype(&T::operator())>>
    : public WasmHostFunc<decltype(&T::operator())> {};

} // namespace detail

using namespace detail;

// forward-declaration for `Func::typed` below.
template <typename Params, typename Results> class TypedFunc;

/**
 * \brief Representation of a WebAssembly function.
 *
 * This class represents a WebAssembly function, either created through
 * instantiating a module or a host function.
 *
 * Note that this type does not itself own any resources. It points to resources
 * owned within a `Store` and the `Store` must be passed in as the first
 * argument to the functions defined on `Func`. Note that if the wrong `Store`
 * is passed in then the process will be aborted.
 */
class Func {
  friend class Val;
  friend class Instance;
  friend class Linker;
  template <typename Params, typename Results> friend class TypedFunc;

  wasmtime_func_t func;

  template <typename F>
  static wasm_trap_t *raw_callback(void *env, wasmtime_caller_t *caller,
                                   const wasmtime_val_t *args, size_t nargs,
                                   wasmtime_val_t *results, size_t nresults) {
    static_assert(alignof(Val) == alignof(wasmtime_val_t));
    static_assert(sizeof(Val) == sizeof(wasmtime_val_t));
    F *func = reinterpret_cast<F *>(env);                          // NOLINT
    Span<const Val> args_span(reinterpret_cast<const Val *>(args), // NOLINT
                              nargs);
    Span<Val> results_span(reinterpret_cast<Val *>(results), // NOLINT
                           nresults);
    Result<std::monostate, Trap> result =
        (*func)(Caller(caller), args_span, results_span);
    if (!result) {
      return result.err().ptr.release();
    }
    return nullptr;
  }

  template <typename F>
  static wasm_trap_t *
  raw_callback_unchecked(void *env, wasmtime_caller_t *caller,
                         wasmtime_val_raw_t *args_and_results,
                         size_t nargs_and_results) {
    using HostFunc = WasmHostFunc<F>;
    Caller cx(caller);
    F *func = reinterpret_cast<F *>(env); // NOLINT
    auto trap = HostFunc::invoke(*func, cx, args_and_results);
    if (trap) {
      return trap->ptr.release();
    }
    return nullptr;
  }

  template <typename F> static void raw_finalize(void *env) {
    std::unique_ptr<F> ptr(reinterpret_cast<F *>(env)); // NOLINT
  }

public:
  /// Creates a new function from the raw underlying C API representation.
  Func(wasmtime_func_t func) : func(func) {}

  /**
   * \brief Creates a new host-defined function.
   *
   * This constructor is used to create a host function within the store
   * provided. This is how WebAssembly can call into the host and make use of
   * external functionality.
   *
   * > **Note**: host functions created this way are more flexible but not
   * > as fast to call as those created by `Func::wrap`.
   *
   * \param cx the store to create the function within
   * \param ty the type of the function that will be created
   * \param f the host callback to be executed when this function is called.
   *
   * The parameter `f` is expected to be a lambda (or a lambda lookalike) which
   * takes three parameters:
   *
   * * The first parameter is a `Caller` to get recursive access to the store
   *   and other caller state.
   * * The second parameter is a `Span<const Val>` which is the list of
   *   parameters to the function. These parameters are guaranteed to be of the
   *   types specified by `ty` when constructing this function.
   * * The last argument is `Span<Val>` which is where to write the return
   *   values of the function. The function must produce the types of values
   *   specified by `ty` or otherwise a trap will be raised.
   *
   * The parameter `f` is expected to return `Result<std::monostate, Trap>`.
   * This allows `f` to raise a trap if desired, or otherwise return no trap and
   * finish successfully. If a trap is raised then the results pointer does not
   * need to be written to.
   */
  template <typename F,
            std::enable_if_t<
                std::is_invocable_r_v<Result<std::monostate, Trap>, F, Caller,
                                      Span<const Val>, Span<Val>>,
                bool> = true>
  Func(Store::Context cx, const FuncType &ty, F f) : func({}) {
    wasmtime_func_new(cx.ptr, ty.ptr.get(), raw_callback<F>,
                      std::make_unique<F>(f).release(), raw_finalize<F>, &func);
  }

  /**
   * \brief Creates a new host function from the provided callback `f`,
   * inferring the WebAssembly function type from the host signature.
   *
   * This function is akin to the `Func` constructor except that the WebAssembly
   * type does not need to be specified and additionally the signature of `f`
   * is different. The main goal of this function is to enable WebAssembly to
   * call the function `f` as-fast-as-possible without having to validate any
   * types or such.
   *
   * The function `f` can optionally take a `Caller` as its first parameter,
   * but otherwise its arguments are translated to WebAssembly types:
   *
   * * `int32_t`, `uint32_t` - `i32`
   * * `int64_t`, `uint64_t` - `i64`
   * * `float` - `f32`
   * * `double` - `f64`
   * * `std::optional<Func>` - `funcref`
   * * `std::optional<ExternRef>` - `externref`
   * * `wasmtime::V128` - `v128`
   *
   * The function may only take these arguments and if it takes any other kinds
   * of arguments then it will fail to compile.
   *
   * The function may return a few different flavors of return values:
   *
   * * `void` - interpreted as returning nothing
   * * Any type above - interpreted as a singular return value.
   * * `std::tuple<T...>` where `T` is one of the valid argument types -
   *   interpreted as returning multiple values.
   * * `Result<T, Trap>` where `T` is another valid return type - interpreted as
   *   a function that returns `T` to wasm but is optionally allowed to also
   *   raise a trap.
   *
   * It's recommended, if possible, to use this function over the `Func`
   * constructor since this is generally easier to work with and also enables
   * a faster path for WebAssembly to call this function.
   */
  template <typename F,
            std::enable_if_t<WasmHostFunc<F>::Params::valid, bool> = true,
            std::enable_if_t<WasmHostFunc<F>::Results::valid, bool> = true>
  static Func wrap(Store::Context cx, F f) {
    using HostFunc = WasmHostFunc<F>;
    auto params = HostFunc::Params::types();
    auto results = HostFunc::Results::types();
    auto ty = FuncType::from_iters(params, results);
    wasmtime_func_t func;
    wasmtime_func_new_unchecked(cx.ptr, ty.ptr.get(), raw_callback_unchecked<F>,
                                std::make_unique<F>(f).release(),
                                raw_finalize<F>, &func);
    return func;
  }

  /**
   * \brief Invoke a WebAssembly function.
   *
   * This function will execute this WebAssembly function. This function muts be
   * defined within the `cx`'s store provided. The `params` argument is the list
   * of parameters that are passed to the wasm function, and the types of the
   * values within `params` must match the type signature of this function.
   *
   * This may return one of three values:
   *
   * * First the function could succeed, returning a vector of values
   *   representing the results of the function.
   * * Otherwise a `Trap` might be generated by the WebAssembly function.
   * * Finally an `Error` could be returned indicating that `params` were not of
   *   the right type.
   *
   * > **Note**: for optimized calls into WebAssembly where the function
   * > signature is statically known it's recommended to use `Func::typed` and
   * > `TypedFunc::call`.
   */
  template <typename I>
  TrapResult<std::vector<Val>> call(Store::Context cx, const I &begin,
                                    const I &end) const {
    std::vector<wasmtime_val_t> raw_params;
    raw_params.reserve(end - begin);
    for (auto i = begin; i != end; i++) {
      raw_params.push_back(i->val);
    }
    size_t nresults = this->type(cx)->results().size();
    std::vector<wasmtime_val_t> raw_results(nresults);

    wasm_trap_t *trap = nullptr;
    auto *error =
        wasmtime_func_call(cx.ptr, &func, raw_params.data(), raw_params.size(),
                           raw_results.data(), raw_results.capacity(), &trap);
    if (error != nullptr) {
      return TrapError(Error(error));
    }
    if (trap != nullptr) {
      return TrapError(Trap(trap));
    }

    std::vector<Val> results;
    results.reserve(nresults);
    for (size_t i = 0; i < nresults; i++) {
      results.push_back(raw_results[i]);
    }
    return results;
  }

  /**
   * \brief Helper function for `call(Store::Context cx, const I &begin, const I
   * &end)`
   *
   * \see call(Store::Context cx, const I &begin, const I &end)
   */
  TrapResult<std::vector<Val>> call(Store::Context cx,
                                    const std::vector<Val> &params) const {
    return this->call(cx, params.begin(), params.end());
  }

  /**
   * \brief Helper function for `call(Store::Context cx, const I &begin, const I
   * &end)`
   *
   * \see call(Store::Context cx, const I &begin, const I &end)
   */
  TrapResult<std::vector<Val>>
  call(Store::Context cx, const std::initializer_list<Val> &params) const {
    return this->call(cx, params.begin(), params.end());
  }

  /// Returns the type of this function.
  FuncType type(Store::Context cx) const {
    return wasmtime_func_type(cx.ptr, &func);
  }

  /**
   * \brief Statically checks this function against the provided types.
   *
   * This function will check whether it takes the statically known `Params`
   * and returns the statically known `Results`. If the type check succeeds then
   * a `TypedFunc` is returned which enables a faster method of invoking
   * WebAssembly functions.
   *
   * The `Params` and `Results` specified as template parameters here are the
   * parameters and results of the wasm function. They can either be a bare
   * type which means that the wasm takes/returns one value, or they can be a
   * `std::tuple<T...>` of types to represent multiple arguments or multiple
   * returns.
   *
   * The valid types for this function are those mentioned as the arguments
   * for `Func::wrap`.
   */
  template <typename Params, typename Results,
            std::enable_if_t<WasmTypeList<Params>::valid, bool> = true,
            std::enable_if_t<WasmTypeList<Results>::valid, bool> = true>
  Result<TypedFunc<Params, Results>, Trap> typed(Store::Context cx) const {
    auto ty = this->type(cx);
    if (!WasmTypeList<Params>::matches(ty->params()) ||
        !WasmTypeList<Results>::matches(ty->results())) {
      return Trap("static type for this function does not match actual type");
    }
    TypedFunc<Params, Results> ret(*this);
    return ret;
  }

  /// Returns the raw underlying C API function this is using.
  const wasmtime_func_t &capi() const { return func; }
};

/**
 * \brief A version of a WebAssembly `Func` where the type signature of the
 * function is statically known.
 */
template <typename Params, typename Results> class TypedFunc {
  friend class Func;
  Func f;
  TypedFunc(Func func) : f(func) {}

public:
  /**
   * \brief Calls this function with the provided parameters.
   *
   * This function is akin to `Func::call` except that since static type
   * information is available it statically takes its parameters and statically
   * returns its results.
   *
   * Note that this function still may return a `Trap` indicating that calling
   * the WebAssembly function failed.
   */
  TrapResult<Results> call(Store::Context cx, Params params) const {
    std::array<wasmtime_val_raw_t, std::max(WasmTypeList<Params>::size,
                                            WasmTypeList<Results>::size)>
        storage;
    wasmtime_val_raw_t *ptr = storage.data();
    if (ptr == nullptr)
      ptr = reinterpret_cast<wasmtime_val_raw_t *>(alignof(wasmtime_val_raw_t));
    WasmTypeList<Params>::store(cx, ptr, params);
    wasm_trap_t *trap = nullptr;
    auto *error = wasmtime_func_call_unchecked(cx.raw_context(), &f.func, ptr,
                                               storage.size(), &trap);
    if (error != nullptr) {
      return TrapError(Error(error));
    }
    if (trap != nullptr) {
      return TrapError(Trap(trap));
    }
    return WasmTypeList<Results>::load(cx, ptr);
  }

  /// Returns the underlying un-typed `Func` for this function.
  const Func &func() const { return f; }
};

inline Val::Val(std::optional<Func> func) : val{} {
  val.kind = WASMTIME_FUNCREF;
  if (func) {
    val.of.funcref = (*func).func;
  } else {
    wasmtime_funcref_set_null(&val.of.funcref);
  }
}

inline Val::Val(Func func) : Val(std::optional(func)) {}
inline Val::Val(ExternRef ptr) : Val(std::optional(ptr)) {}
inline Val::Val(AnyRef ptr) : Val(std::optional(ptr)) {}

inline std::optional<Func> Val::funcref() const {
  if (val.kind != WASMTIME_FUNCREF) {
    std::abort();
  }
  if (val.of.funcref.store_id == 0) {
    return std::nullopt;
  }
  return Func(val.of.funcref);
}

/// Definition for the `funcref` native wasm type
template <> struct detail::WasmType<std::optional<Func>> {
  /// @private
  static const bool valid = true;
  /// @private
  static const ValKind kind = ValKind::FuncRef;
  /// @private
  static void store(Store::Context cx, wasmtime_val_raw_t *p,
                    const std::optional<Func> func) {
    if (func) {
      p->funcref = wasmtime_func_to_raw(cx.raw_context(), &func->capi());
    } else {
      p->funcref = 0;
    }
  }
  /// @private
  static std::optional<Func> load(Store::Context cx, wasmtime_val_raw_t *p) {
    if (p->funcref == 0) {
      return std::nullopt;
    }
    wasmtime_func_t ret;
    wasmtime_func_from_raw(cx.raw_context(), p->funcref, &ret);
    return ret;
  }
};

} // namespace wasmtime

#endif // WASMTIME_FUNC_HH
