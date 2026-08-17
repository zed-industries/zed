// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUNCTIONAL_BIND_INTERNAL_H_
#define BASE_FUNCTIONAL_BIND_INTERNAL_H_

#include <stddef.h>

#include <concepts>
#include <functional>
#include <memory>
#include <tuple>
#include <type_traits>
#include <utility>

#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/functional/callback_internal.h"
#include "base/functional/unretained_traits.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ref.h"
#include "base/memory/weak_ptr.h"
#include "base/types/always_false.h"
#include "base/types/is_complete.h"
#include "base/types/is_instantiation.h"
#include "base/types/to_address.h"
#include "build/build_config.h"
#include "third_party/abseil-cpp/absl/functional/function_ref.h"

#if PA_BUILDFLAG(USE_ASAN_BACKUP_REF_PTR)
#include "base/memory/raw_ptr_asan_bound_arg_tracker.h"
#endif

// See docs/callback.md for user documentation.
//
// Concepts:
//  Functor -- A movable type representing something that should be called.
//             All function pointers and `Callback<>` are functors even if the
//             invocation syntax differs.
//  RunType -- A function type (as opposed to function _pointer_ type) for
//             a `Callback<>::Run()`.  Usually just a convenience typedef.
//  (Bound)Args -- A set of types that stores the arguments.
//
// Types:
//  `ForceVoidReturn<>` -- Helper class for translating function signatures to
//                         equivalent forms with a `void` return type.
//  `FunctorTraits<>` -- Type traits used to determine the correct RunType and
//                       invocation manner for a Functor.  This is where
//                       function signature adapters are applied.
//  `StorageTraits<>` -- Type traits that determine how a bound argument is
//                       stored in `BindState<>`.
//  `InvokeHelper<>` -- Takes a Functor + arguments and actually invokes it.
//                      Handles the differing syntaxes needed for `WeakPtr<>`
//                      support.  This is separate from `Invoker<>` to avoid
//                      creating multiple versions of `Invoker<>`.
//  `Invoker<>` -- Unwraps the curried parameters and executes the Functor.
//  `BindState<>` -- Stores the curried parameters, and is the main entry point
//                   into the `Bind()` system.

#if BUILDFLAG(IS_WIN)
namespace Microsoft {
namespace WRL {
template <typename>
class ComPtr;
}  // namespace WRL
}  // namespace Microsoft
#endif

namespace base {

template <typename T>
struct IsWeakReceiver;

template <typename>
struct BindUnwrapTraits;

template <typename Functor, typename BoundArgsTuple>
struct CallbackCancellationTraits;

template <typename Signature>
class FunctionRef;

// A tag type to return when `Bind()` calls fail. In this case we intentionally
// don't return `void`, since that would produce spurious errors like "variable
// has incomplete type 'void'" when assigning the result of
// `Bind{Once,Repeating}()` to an `auto`.
struct BindFailedCheckPreviousErrors {};

namespace unretained_traits {

// `UnretainedWrapper` will check and report if pointer is dangling upon
// invocation.
struct MayNotDangle {};
// `UnretainedWrapper` won't check if pointer is dangling upon invocation. For
// extra safety, the receiver must be of type `MayBeDangling<>`.
struct MayDangle {};
// `UnretainedWrapper` won't check if pointer is dangling upon invocation. The
// receiver doesn't have to be a `raw_ptr<>`. This is just a temporary state, to
// allow dangling pointers that would otherwise crash if `MayNotDangle` was
// used. It should be replaced ASAP with `MayNotDangle` (after fixing the
// dangling pointers) or with `MayDangle` if there is really no other way (after
// making receivers `MayBeDangling<>`).
struct MayDangleUntriaged {};

}  // namespace unretained_traits

namespace internal {

template <typename T,
          typename UnretainedTrait,
          RawPtrTraits PtrTraits = RawPtrTraits::kEmpty>
class UnretainedWrapper {
  // Note that if `PtrTraits` already includes `MayDangle`, `DanglingRawPtrType`
  // will be identical to `raw_ptr<T, PtrTraits>`.
  using DanglingRawPtrType = MayBeDangling<T, PtrTraits>;

 public:
  // We want the getter type to match the receiver parameter that it is passed
  // into, to minimize `raw_ptr<T>` <-> `T*` conversions. We also would like to
  // match `StorageType`, but sometimes we can't have both, as shown in
  // https://docs.google.com/document/d/1dLM34aKqbNBfRdOYxxV_T-zQU4J5wjmXwIBJZr7JvZM/edit
  // When we can't have both, prefer the former, mostly because
  // `GetPtrType`=`raw_ptr<T>` would break if e.g. `UnretainedWrapper()` is
  // constructed using `char*`, but the receiver is of type `std::string&`.
  // This is enforced by `static_assert()`s in `ParamCanBeBound`.
  using GetPtrType = std::conditional_t<
      raw_ptr_traits::IsSupportedType<T>::value &&
          std::same_as<UnretainedTrait, unretained_traits::MayDangle>,
      DanglingRawPtrType,
      T*>;

  // Raw pointer makes sense only if there are no `PtrTrait`s. If there are,
  // it means that a `raw_ptr` is being passed, so use the ctors below instead.
  explicit UnretainedWrapper(T* o)
    requires(PtrTraits == RawPtrTraits::kEmpty)
      : ptr_(o) {
    VerifyPreconditions();
  }

  explicit UnretainedWrapper(const raw_ptr<T, PtrTraits>& o)
    requires(raw_ptr_traits::IsSupportedType<T>::value)
      : ptr_(o) {
    VerifyPreconditions();
  }

  explicit UnretainedWrapper(raw_ptr<T, PtrTraits>&& o)
    requires(raw_ptr_traits::IsSupportedType<T>::value)
      : ptr_(std::move(o)) {
    VerifyPreconditions();
  }

  GetPtrType get() const { return GetInternal(ptr_); }

  // True if this type is valid. When this is false, a `static_assert` will have
  // been fired explaining why.
  static constexpr bool value = SupportsUnretained<T>;

 private:
  // `ptr_` is either a `raw_ptr` or a regular C++ pointer.
  template <typename U>
    requires std::same_as<T, U>
  static GetPtrType GetInternal(U* ptr) {
    return ptr;
  }
  template <typename U, RawPtrTraits Traits>
    requires std::same_as<T, U>
  static GetPtrType GetInternal(const raw_ptr<U, Traits>& ptr) {
    if constexpr (std::same_as<UnretainedTrait,
                               unretained_traits::MayNotDangle>) {
      ptr.ReportIfDangling();
    }
    return ptr;
  }

  // `Unretained()` arguments often dangle by design (a common design pattern
  // is to manage an object's lifetime inside the callback itself, using
  // stateful information), so disable direct dangling pointer detection
  // of `ptr_`.
  //
  // If the callback is invoked, dangling pointer detection will be triggered
  // before invoking the bound functor (unless stated otherwise, see
  // `UnsafeDangling()` and `UnsafeDanglingUntriaged()`), when retrieving the
  // pointer value via `get()` above.
  using StorageType =
      std::conditional_t<raw_ptr_traits::IsSupportedType<T>::value,
                         DanglingRawPtrType,
                         T*>;
  // Avoid converting between different `raw_ptr` types when calling `get()`.
  // It is allowable to convert `raw_ptr<T>` -> `T*`, but not in the other
  // direction. See the comment by `GetPtrType` describing for more details.
  static_assert(std::is_pointer_v<GetPtrType> ||
                std::same_as<GetPtrType, StorageType>);

  // Forces `value` to be materialized, performing a compile-time check of the
  // preconditions if it hasn't already occurred. This is called from every
  // constructor so the wrappers in bind.h don't have to each check it, and so
  // no one can go around them and construct this underlying type directly.
  static constexpr void VerifyPreconditions() {
    // Using `static_assert(value);` here would work but fire an extra error.
    std::ignore = value;
  }

  StorageType ptr_;
};

// Storage type for `std::reference_wrapper` so `BindState` can internally store
// unprotected references using `raw_ref`.
//
// `std::reference_wrapper<T>` and `T&` do not work, since the reference
// lifetime is not safely protected by MiraclePtr.
//
// `UnretainedWrapper<T>` and `raw_ptr<T>` do not work, since `BindUnwrapTraits`
// would try to pass by `T*` rather than `T&`.
template <typename T,
          typename UnretainedTrait,
          RawPtrTraits PtrTraits = RawPtrTraits::kEmpty>
class UnretainedRefWrapper {
 public:
  // Raw reference makes sense only if there are no `PtrTrait`s. If there are,
  // it means that a `raw_ref` is being passed, so use the ctors below instead.
  explicit UnretainedRefWrapper(T& o)
    requires(PtrTraits == RawPtrTraits::kEmpty)
      : ref_(o) {
    VerifyPreconditions();
  }

  explicit UnretainedRefWrapper(const raw_ref<T, PtrTraits>& o)
    requires(raw_ptr_traits::IsSupportedType<T>::value)
      : ref_(o) {
    VerifyPreconditions();
  }

  explicit UnretainedRefWrapper(raw_ref<T, PtrTraits>&& o)
    requires(raw_ptr_traits::IsSupportedType<T>::value)
      : ref_(std::move(o)) {
    VerifyPreconditions();
  }

  T& get() const { return GetInternal(ref_); }

  // See comments in `UnretainedWrapper` regarding this and
  // `VerifyPreconditions()`.
  static constexpr bool value = SupportsUnretained<T>;

 private:
  // `ref_` is either a `raw_ref` or a regular C++ reference.
  template <typename U>
    requires std::same_as<T, U>
  static T& GetInternal(U& ref) {
    return ref;
  }
  template <typename U, RawPtrTraits Traits>
    requires std::same_as<T, U>
  static T& GetInternal(const raw_ref<U, Traits>& ref) {
    // The ultimate goal is to crash when a callback is invoked with a
    // dangling pointer. This is checked here. For now, it is configured to
    // either crash, DumpWithoutCrashing or be ignored. This depends on the
    // `PartitionAllocUnretainedDanglingPtr` feature.
    if constexpr (std::is_same_v<UnretainedTrait,
                                 unretained_traits::MayNotDangle>) {
      ref.ReportIfDangling();
    }
    // We can't use `operator*` here, we need to use `raw_ptr`'s
    // `GetForExtraction` instead of `GetForDereference`. If we did use
    // `GetForDereference` then we'd crash in ASAN builds on calling a bound
    // callback with a dangling reference parameter even if that parameter is
    // not used. This could hide a later unprotected issue that would be reached
    // in release builds.
    return ref.get();
  }

  // `Unretained()` arguments often dangle by design (a common design pattern
  // is to manage an object's lifetime inside the callback itself, using
  // stateful information), so disable direct dangling pointer detection
  // of `ref_`.
  //
  // If the callback is invoked, dangling pointer detection will be triggered
  // before invoking the bound functor (unless stated otherwise, see
  // `UnsafeDangling()` and `UnsafeDanglingUntriaged()`), when retrieving the
  // pointer value via `get()` above.
  using StorageType =
      std::conditional_t<raw_ptr_traits::IsSupportedType<T>::value,
                         raw_ref<T, DisableDanglingPtrDetection>,
                         T&>;

  static constexpr void VerifyPreconditions() { std::ignore = value; }

  StorageType ref_;
};

// Can't use `is_instantiation` to detect the unretained wrappers, since they
// have non-type template params.
template <template <typename, typename, RawPtrTraits> typename WrapperT,
          typename T>
inline constexpr bool kIsUnretainedWrapper = false;

template <template <typename, typename, RawPtrTraits> typename WrapperT,
          typename T,
          typename UnretainedTrait,
          RawPtrTraits PtrTraits>
inline constexpr bool
    kIsUnretainedWrapper<WrapperT, WrapperT<T, UnretainedTrait, PtrTraits>> =
        true;

// The class is used to wrap `UnretainedRefWrapper` when the latter is used as
// a method receiver (a reference on `this` argument). This is needed because
// the internal callback mechanism expects the receiver to have the type
// `MyClass*` and to have `operator*`.
// This is used as storage.
template <typename T, typename UnretainedTrait, RawPtrTraits PtrTraits>
class UnretainedRefWrapperReceiver {
 public:
  // NOLINTNEXTLINE(google-explicit-constructor)
  UnretainedRefWrapperReceiver(
      UnretainedRefWrapper<T, UnretainedTrait, PtrTraits>&& obj)
      : obj_(std::move(obj)) {}

  T& operator*() const { return obj_.get(); }
  T* operator->() const { return &obj_.get(); }

 private:
  UnretainedRefWrapper<T, UnretainedTrait, PtrTraits> obj_;
};

// `MethodReceiverStorage` converts the current receiver type to its stored
// type. For instance, it converts pointers to `scoped_refptr`, and wraps
// `UnretainedRefWrapper` to make it compliant with the internal callback
// invocation mechanism.
template <typename T>
struct MethodReceiverStorage {
  using Type = std::
      conditional_t<IsPointerOrRawPtr<T>, scoped_refptr<RemovePointerT<T>>, T>;
};

template <typename T, typename UnretainedTrait, RawPtrTraits PtrTraits>
struct MethodReceiverStorage<
    UnretainedRefWrapper<T, UnretainedTrait, PtrTraits>> {
  // We can't use `UnretainedRefWrapper` as a receiver directly (see
  // `UnretainedRefWrapperReceiver` for why).
  using Type = UnretainedRefWrapperReceiver<T, UnretainedTrait, PtrTraits>;
};

template <typename T>
class RetainedRefWrapper {
 public:
  explicit RetainedRefWrapper(T* o) : ptr_(o) {}
  explicit RetainedRefWrapper(scoped_refptr<T> o) : ptr_(std::move(o)) {}
  T* get() const { return ptr_.get(); }

 private:
  scoped_refptr<T> ptr_;
};

template <typename T>
struct IgnoreResultHelper {
  explicit IgnoreResultHelper(T functor) : functor_(std::move(functor)) {}
  explicit operator bool() const { return !!functor_; }

  T functor_;
};

template <typename T, typename Deleter = std::default_delete<T>>
class OwnedWrapper {
 public:
  explicit OwnedWrapper(T* o) : ptr_(o) {}
  explicit OwnedWrapper(std::unique_ptr<T, Deleter>&& ptr)
      : ptr_(std::move(ptr)) {}
  T* get() const { return ptr_.get(); }

 private:
  std::unique_ptr<T, Deleter> ptr_;
};

template <typename T>
class OwnedRefWrapper {
 public:
  explicit OwnedRefWrapper(const T& t) : t_(t) {}
  explicit OwnedRefWrapper(T&& t) : t_(std::move(t)) {}
  T& get() const { return t_; }

 private:
  mutable T t_;
};

// `PassedWrapper` is a copyable adapter for a scoper that ignores `const`.
//
// It is needed to get around the fact that `Bind()` takes a const reference to
// all its arguments.  Because `Bind()` takes a const reference to avoid
// unnecessary copies, it is incompatible with movable-but-not-copyable
// types; doing a destructive "move" of the type into `Bind()` would violate
// the const correctness.
//
// This conundrum cannot be solved without either rvalue references or an O(2^n)
// blowup of `Bind()` templates to handle each combination of regular types and
// movable-but-not-copyable types.  Thus we introduce a wrapper type that is
// copyable to transmit the correct type information down into `BindState<>`.
// Ignoring `const` in this type makes sense because it is only created when we
// are explicitly trying to do a destructive move.
//
// Two notes:
//  1) `PassedWrapper` supports any type that has a move constructor, however
//     the type will need to be specifically allowed in order for it to be
//     bound to a `Callback`. We guard this explicitly at the call of `Passed()`
//     to make for clear errors. Things not given to `Passed()` will be
//     forwarded and stored by value which will not work for general move-only
//     types.
//  2) `is_valid_` is distinct from `nullptr` because it is valid to bind a null
//     scoper to a `Callback` and allow the `Callback` to execute once.
//
// TODO(crbug.com/40840557): We have rvalue references and such now. Remove.
template <typename T>
class PassedWrapper {
 public:
  explicit PassedWrapper(T&& scoper) : scoper_(std::move(scoper)) {}
  PassedWrapper(PassedWrapper&& other)
      : is_valid_(other.is_valid_), scoper_(std::move(other.scoper_)) {}
  T Take() const {
    CHECK(is_valid_);
    is_valid_ = false;
    return std::move(scoper_);
  }

 private:
  mutable bool is_valid_ = true;
  mutable T scoper_;
};

template <typename T>
using Unwrapper = BindUnwrapTraits<std::decay_t<T>>;

template <typename T>
decltype(auto) Unwrap(T&& o) {
  return Unwrapper<T>::Unwrap(std::forward<T>(o));
}

// `kIsWeakMethod` is a helper that determines if we are binding a `WeakPtr<>`
// to a method. It is used internally by `Bind()` to select the correct
// `InvokeHelper` that will no-op itself in the event the `WeakPtr<>` for the
// target object is invalidated.
//
// The first argument should be the type of the object that will be received by
// the method.
template <bool is_method, typename... Args>
inline constexpr bool kIsWeakMethod = false;

template <typename T, typename... Args>
inline constexpr bool kIsWeakMethod<true, T, Args...> =
    IsWeakReceiver<T>::value;

// Packs a list of types to hold them in a single type.
template <typename... Types>
struct TypeList {};

// Implements `DropTypeListItem`.
template <size_t n, typename List>
  requires is_instantiation<TypeList, List>
struct DropTypeListItemImpl {
  using Type = List;
};

template <size_t n, typename T, typename... List>
  requires(n > 0)
struct DropTypeListItemImpl<n, TypeList<T, List...>>
    : DropTypeListItemImpl<n - 1, TypeList<List...>> {};

// A type-level function that drops `n` list items from a given `TypeList`.
template <size_t n, typename List>
using DropTypeListItem = typename DropTypeListItemImpl<n, List>::Type;

// Implements `TakeTypeListItem`.
template <size_t n, typename List, typename... Accum>
  requires is_instantiation<TypeList, List>
struct TakeTypeListItemImpl {
  using Type = TypeList<Accum...>;
};

template <size_t n, typename T, typename... List, typename... Accum>
  requires(n > 0)
struct TakeTypeListItemImpl<n, TypeList<T, List...>, Accum...>
    : TakeTypeListItemImpl<n - 1, TypeList<List...>, Accum..., T> {};

// A type-level function that takes the first `n` items from a given `TypeList`;
// e.g. `TakeTypeListItem<3, TypeList<A, B, C, D>>` -> `TypeList<A, B, C>`.
template <size_t n, typename List>
using TakeTypeListItem = typename TakeTypeListItemImpl<n, List>::Type;

// Implements `MakeFunctionType`.
template <typename R, typename ArgList>
struct MakeFunctionTypeImpl;

template <typename R, typename... Args>
struct MakeFunctionTypeImpl<R, TypeList<Args...>> {
  using Type = R(Args...);
};

// A type-level function that constructs a function type that has `R` as its
// return type and has a `TypeList`'s items as its arguments.
template <typename R, typename ArgList>
using MakeFunctionType = typename MakeFunctionTypeImpl<R, ArgList>::Type;

// Implements `ExtractArgs` and `ExtractReturnType`.
template <typename Signature>
struct ExtractArgsImpl;

template <typename R, typename... Args>
struct ExtractArgsImpl<R(Args...)> {
  using ReturnType = R;
  using ArgsList = TypeList<Args...>;
};

// A type-level function that extracts function arguments into a `TypeList`;
// e.g. `ExtractArgs<R(A, B, C)>` -> `TypeList<A, B, C>`.
template <typename Signature>
using ExtractArgs = typename ExtractArgsImpl<Signature>::ArgsList;

// A type-level function that extracts the return type of a function.
// e.g. `ExtractReturnType<R(A, B, C)>` -> `R`.
template <typename Signature>
using ExtractReturnType = typename ExtractArgsImpl<Signature>::ReturnType;

template <typename Callable,
          typename Signature = decltype(&Callable::operator())>
struct ExtractCallableRunTypeImpl;

#define BIND_INTERNAL_EXTRACT_CALLABLE_RUN_TYPE_WITH_QUALS(quals)     \
  template <typename Callable, typename R, typename... Args>          \
  struct ExtractCallableRunTypeImpl<Callable,                         \
                                    R (Callable::*)(Args...) quals> { \
    using Type = R(Args...);                                          \
  }

BIND_INTERNAL_EXTRACT_CALLABLE_RUN_TYPE_WITH_QUALS();
BIND_INTERNAL_EXTRACT_CALLABLE_RUN_TYPE_WITH_QUALS(const);
BIND_INTERNAL_EXTRACT_CALLABLE_RUN_TYPE_WITH_QUALS(noexcept);
BIND_INTERNAL_EXTRACT_CALLABLE_RUN_TYPE_WITH_QUALS(const noexcept);

#undef BIND_INTERNAL_EXTRACT_CALLABLE_RUN_TYPE_WITH_QUALS

// Evaluated to the RunType of the given callable type; e.g.
// `ExtractCallableRunType<decltype([](int, char*) { return 0.1; })>` ->
//     `double(int, char*)`.
template <typename Callable>
using ExtractCallableRunType =
    typename ExtractCallableRunTypeImpl<Callable>::Type;

// True when `Functor` has a non-overloaded `operator()()`, e.g.:
//   struct S1 {
//     int operator()(int);
//   };
//   static_assert(HasNonOverloadedCallOp<S1>);
//
//   int i = 0;
//   auto f = [i] {};
//   static_assert(HasNonOverloadedCallOp<decltype(f)>);
//
//   struct S2 {
//     int operator()(int);
//     std::string operator()(std::string);
//   };
//   static_assert(!HasNonOverloadedCallOp<S2>);
//
//   static_assert(!HasNonOverloadedCallOp<void(*)()>);
//
//   struct S3 {};
//   static_assert(!HasNonOverloadedCallOp<S3>);
// ```
template <typename Functor>
concept HasNonOverloadedCallOp = requires { &Functor::operator(); };

template <typename T>
inline constexpr bool IsObjCArcBlockPointer = false;

#if __OBJC__ && HAS_FEATURE(objc_arc)
template <typename R, typename... Args>
inline constexpr bool IsObjCArcBlockPointer<R (^)(Args...)> = true;
#endif

// True when `Functor` has an overloaded `operator()()` that can be invoked with
// the provided `BoundArgs`.
//
// Do not decay `Functor` before testing this, lest it give an incorrect result
// for overloads with different ref-qualifiers.
template <typename Functor, typename... BoundArgs>
concept HasOverloadedCallOp = requires {
  // The functor must be invocable with the bound args.
  requires requires(Functor&& f, BoundArgs&&... args) {
    std::forward<Functor>(f)(std::forward<BoundArgs>(args)...);
  };
  // Now exclude invocables that are not cases of overloaded `operator()()`s:
  // * `operator()()` exists, but isn't overloaded
  requires !HasNonOverloadedCallOp<std::decay_t<Functor>>;
  // * Function pointer (doesn't have `operator()()`)
  requires !std::is_pointer_v<std::decay_t<Functor>>;
  // * Block pointer (doesn't have `operator()()`)
  requires !IsObjCArcBlockPointer<std::decay_t<Functor>>;
};

// `ForceVoidReturn<>` converts a signature to have a `void` return type.
template <typename Sig>
struct ForceVoidReturn;

template <typename R, typename... Args>
struct ForceVoidReturn<R(Args...)> {
  using RunType = void(Args...);
};

// `FunctorTraits<>`
//
// See description at top of file. This must be declared here so it can be
// referenced in `DecayedFunctorTraits`.
template <typename Functor, typename... BoundArgs>
struct FunctorTraits;

// Provides functor traits for pre-decayed functor types.
template <typename Functor, typename... BoundArgs>
struct DecayedFunctorTraits;

// Callable types.
// This specialization handles lambdas (captureless and capturing) and functors
// with a call operator. Capturing lambdas and stateful functors are explicitly
// disallowed by `BindHelper<>::Bind()`; e.g.:
// ```
//   // Captureless lambda: Allowed
//   [] { return 42; };
//
//   // Capturing lambda: Disallowed
//   int x;
//   [x] { return x; };
//
//   // Empty class with `operator()()`: Allowed
//   struct Foo {
//     void operator()() const {}
//     // No non-`static` member variables and no virtual functions.
//   };
// ```
template <typename Functor, typename... BoundArgs>
  requires HasNonOverloadedCallOp<Functor>
struct DecayedFunctorTraits<Functor, BoundArgs...> {
  using RunType = ExtractCallableRunType<Functor>;
  static constexpr bool is_method = false;
  static constexpr bool is_nullable = false;
  static constexpr bool is_callback = false;
  static constexpr bool is_stateless = std::is_empty_v<Functor>;

  template <typename RunFunctor, typename... RunArgs>
  static ExtractReturnType<RunType> Invoke(RunFunctor&& functor,
                                           RunArgs&&... args) {
    return std::forward<RunFunctor>(functor)(std::forward<RunArgs>(args)...);
  }
};

// Functions.
template <typename R, typename... Args, typename... BoundArgs>
struct DecayedFunctorTraits<R (*)(Args...), BoundArgs...> {
  using RunType = R(Args...);
  static constexpr bool is_method = false;
  static constexpr bool is_nullable = true;
  static constexpr bool is_callback = false;
  static constexpr bool is_stateless = true;

  template <typename Function, typename... RunArgs>
  static R Invoke(Function&& function, RunArgs&&... args) {
    return std::forward<Function>(function)(std::forward<RunArgs>(args)...);
  }
};

template <typename R, typename... Args, typename... BoundArgs>
struct DecayedFunctorTraits<R (*)(Args...) noexcept, BoundArgs...>
    : DecayedFunctorTraits<R (*)(Args...), BoundArgs...> {};

#if BUILDFLAG(IS_WIN) && !defined(ARCH_CPU_64_BITS)

// `__stdcall` and `__fastcall` functions.
#define BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_WITH_CONV_AND_QUALS(conv, quals) \
  template <typename R, typename... Args, typename... BoundArgs>              \
  struct DecayedFunctorTraits<R(conv*)(Args...) quals, BoundArgs...>          \
      : DecayedFunctorTraits<R (*)(Args...) quals, BoundArgs...> {}

BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_WITH_CONV_AND_QUALS(__stdcall, );
BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_WITH_CONV_AND_QUALS(__stdcall, noexcept);
BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_WITH_CONV_AND_QUALS(__fastcall, );
BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_WITH_CONV_AND_QUALS(__fastcall, noexcept);

#undef BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_WITH_CONV_AND_QUALS
#endif  // BUILDFLAG(IS_WIN) && !defined(ARCH_CPU_64_BITS)

#if __OBJC__ && HAS_FEATURE(objc_arc)

// Objective-C blocks. Blocks can be bound as the compiler will ensure their
// lifetimes will be correctly managed.
template <typename R, typename... Args, typename... BoundArgs>
struct DecayedFunctorTraits<R (^)(Args...), BoundArgs...> {
  using RunType = R(Args...);
  static constexpr bool is_method = false;
  static constexpr bool is_nullable = true;
  static constexpr bool is_callback = false;
  static constexpr bool is_stateless = true;

  template <typename BlockType, typename... RunArgs>
  static R Invoke(BlockType&& block, RunArgs&&... args) {
    // According to LLVM documentation (§ 6.3), "local variables of automatic
    // storage duration do not have precise lifetime." Use
    // `objc_precise_lifetime` to ensure that the Objective-C block is not
    // deallocated until it has finished executing even if the `Callback<>` is
    // destroyed during the block execution.
    // https://clang.llvm.org/docs/AutomaticReferenceCounting.html#precise-lifetime-semantics
    __attribute__((objc_precise_lifetime)) R (^scoped_block)(Args...) = block;
    return scoped_block(std::forward<RunArgs>(args)...);
  }
};

#endif  // __OBJC__ && HAS_FEATURE(objc_arc)

// Methods.
template <typename R,
          typename Receiver,
          typename... Args,
          typename... BoundArgs>
struct DecayedFunctorTraits<R (Receiver::*)(Args...), BoundArgs...> {
  using RunType = R(Receiver*, Args...);
  static constexpr bool is_method = true;
  static constexpr bool is_nullable = true;
  static constexpr bool is_callback = false;
  static constexpr bool is_stateless = true;

  template <typename Method, typename ReceiverPtr, typename... RunArgs>
  static R Invoke(Method method,
                  ReceiverPtr&& receiver_ptr,
                  RunArgs&&... args) {
    return ((*receiver_ptr).*method)(std::forward<RunArgs>(args)...);
  }
};

template <typename R,
          typename Receiver,
          typename... Args,
          typename... BoundArgs>
struct DecayedFunctorTraits<R (Receiver::*)(Args...) const, BoundArgs...>
    : DecayedFunctorTraits<R (Receiver::*)(Args...), BoundArgs...> {
  using RunType = R(const Receiver*, Args...);
};

#define BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_WITH_CONST_AND_QUALS(constqual, \
                                                                  quals)     \
  template <typename R, typename Receiver, typename... Args,                 \
            typename... BoundArgs>                                           \
  struct DecayedFunctorTraits<R (Receiver::*)(Args...) constqual quals,      \
                              BoundArgs...>                                  \
      : DecayedFunctorTraits<R (Receiver::*)(Args...) constqual,             \
                             BoundArgs...> {}

BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_WITH_CONST_AND_QUALS(, noexcept);
BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_WITH_CONST_AND_QUALS(const, noexcept);

#undef BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_WITH_CONST_AND_QUALS

#if BUILDFLAG(IS_WIN) && !defined(ARCH_CPU_64_BITS)

// `__stdcall` methods.
#define BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_STDCALL_WITH_QUALS(quals)  \
  template <typename R, typename Receiver, typename... Args,            \
            typename... BoundArgs>                                      \
  struct DecayedFunctorTraits<R (__stdcall Receiver::*)(Args...) quals, \
                              BoundArgs...>                             \
      : public DecayedFunctorTraits<R (Receiver::*)(Args...) quals,     \
                                    BoundArgs...> {}

BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_STDCALL_WITH_QUALS();
BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_STDCALL_WITH_QUALS(const);
BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_STDCALL_WITH_QUALS(noexcept);
BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_STDCALL_WITH_QUALS(const noexcept);

#undef BIND_INTERNAL_DECAYED_FUNCTOR_TRAITS_STDCALL_WITH_QUALS

#endif  // BUILDFLAG(IS_WIN) && !defined(ARCH_CPU_64_BITS)

// `IgnoreResult`s.
template <typename T, typename... BoundArgs>
struct DecayedFunctorTraits<IgnoreResultHelper<T>, BoundArgs...>
    : FunctorTraits<T, BoundArgs...> {
  using RunType = typename ForceVoidReturn<
      typename FunctorTraits<T, BoundArgs...>::RunType>::RunType;

  template <typename IgnoreResultType, typename... RunArgs>
  static void Invoke(IgnoreResultType&& ignore_result_helper,
                     RunArgs&&... args) {
    FunctorTraits<T, BoundArgs...>::Invoke(
        std::forward<IgnoreResultType>(ignore_result_helper).functor_,
        std::forward<RunArgs>(args)...);
  }
};

// `OnceCallback`s.
template <typename R, typename... Args, typename... BoundArgs>
struct DecayedFunctorTraits<OnceCallback<R(Args...)>, BoundArgs...> {
  using RunType = R(Args...);
  static constexpr bool is_method = false;
  static constexpr bool is_nullable = true;
  static constexpr bool is_callback = true;
  static constexpr bool is_stateless = true;

  template <typename CallbackType, typename... RunArgs>
  static R Invoke(CallbackType&& callback, RunArgs&&... args) {
    DCHECK(!callback.is_null());
    return std::forward<CallbackType>(callback).Run(
        std::forward<RunArgs>(args)...);
  }
};

// `RepeatingCallback`s.
template <typename R, typename... Args, typename... BoundArgs>
struct DecayedFunctorTraits<RepeatingCallback<R(Args...)>, BoundArgs...> {
  using RunType = R(Args...);
  static constexpr bool is_method = false;
  static constexpr bool is_nullable = true;
  static constexpr bool is_callback = true;
  static constexpr bool is_stateless = true;

  template <typename CallbackType, typename... RunArgs>
  static R Invoke(CallbackType&& callback, RunArgs&&... args) {
    DCHECK(!callback.is_null());
    return std::forward<CallbackType>(callback).Run(
        std::forward<RunArgs>(args)...);
  }
};

// For most functors, the traits should not depend on how the functor is passed,
// so decay the functor.
template <typename Functor, typename... BoundArgs>
// This requirement avoids "implicit instantiation of undefined template" errors
// when the underlying `DecayedFunctorTraits<>` cannot be instantiated. Instead,
// this template will also not be instantiated, and the caller can detect and
// handle that.
  requires IsComplete<DecayedFunctorTraits<std::decay_t<Functor>, BoundArgs...>>
struct FunctorTraits<Functor, BoundArgs...>
    : DecayedFunctorTraits<std::decay_t<Functor>, BoundArgs...> {};

// For `overloaded operator()()`s, it's possible the ref qualifiers of the
// functor matter, so be careful to use the undecayed type.
template <typename Functor, typename... BoundArgs>
  requires HasOverloadedCallOp<Functor, BoundArgs...>
struct FunctorTraits<Functor, BoundArgs...> {
  // For an overloaded operator()(), it is not possible to resolve the
  // actual declared type. Since it is invocable with the bound args, make up a
  // signature based on their types.
  using RunType = decltype(std::declval<Functor>()(
      std::declval<BoundArgs>()...))(std::decay_t<BoundArgs>...);
  static constexpr bool is_method = false;
  static constexpr bool is_nullable = false;
  static constexpr bool is_callback = false;
  static constexpr bool is_stateless = std::is_empty_v<std::decay_t<Functor>>;

  template <typename RunFunctor, typename... RunArgs>
  static ExtractReturnType<RunType> Invoke(RunFunctor&& functor,
                                           RunArgs&&... args) {
    return std::forward<RunFunctor>(functor)(std::forward<RunArgs>(args)...);
  }
};

// `StorageTraits<>`
//
// See description at top of file.
template <typename T>
struct StorageTraits {
  // The type to use for storing the bound arg inside `BindState`.
  using Type = T;

  // True iff all compile-time preconditions for using this specialization are
  // satisfied. Specializations that set this to `false` should ensure a
  // `static_assert()` explains why.
  static constexpr bool value = true;
};

// For `T*`, store as `UnretainedWrapper<T>` for safety, as it internally uses
// `raw_ptr<T>` (when possible).
template <typename T>
struct StorageTraits<T*> {
  using Type = UnretainedWrapper<T, unretained_traits::MayNotDangle>;
  static constexpr bool value = Type::value;
};

// For `raw_ptr<T>`, store as `UnretainedWrapper<T>` for safety. This may seem
// contradictory, but this ensures guaranteed protection for the pointer even
// during execution of callbacks with parameters of type `raw_ptr<T>`.
template <typename T, RawPtrTraits PtrTraits>
struct StorageTraits<raw_ptr<T, PtrTraits>> {
  using Type = UnretainedWrapper<T, unretained_traits::MayNotDangle, PtrTraits>;
  static constexpr bool value = Type::value;
};

// Unwrap `std::reference_wrapper` and store it in a custom wrapper so that
// references are also protected with `raw_ptr<T>`.
template <typename T>
struct StorageTraits<std::reference_wrapper<T>> {
  using Type = UnretainedRefWrapper<T, unretained_traits::MayNotDangle>;
  static constexpr bool value = Type::value;
};

template <typename T>
using ValidateStorageTraits = StorageTraits<std::decay_t<T>>;

// `InvokeHelper<>`
//
// There are 2 logical `InvokeHelper<>` specializations: normal, weak.
//
// The normal type just calls the underlying runnable.
//
// Weak calls need special syntax that is applied to the first argument to check
// if they should no-op themselves.
template <bool is_weak_call,
          typename Traits,
          typename ReturnType,
          size_t... indices>
struct InvokeHelper;

template <typename Traits, typename ReturnType, size_t... indices>
struct InvokeHelper<false, Traits, ReturnType, indices...> {
  template <typename Functor, typename BoundArgsTuple, typename... RunArgs>
  static inline ReturnType MakeItSo(Functor&& functor,
                                    BoundArgsTuple&& bound,
                                    RunArgs&&... args) {
    return Traits::Invoke(
        Unwrap(std::forward<Functor>(functor)),
        Unwrap(std::get<indices>(std::forward<BoundArgsTuple>(bound)))...,
        std::forward<RunArgs>(args)...);
  }
};

template <typename Traits,
          typename ReturnType,
          size_t index_target,
          size_t... index_tail>
struct InvokeHelper<true, Traits, ReturnType, index_target, index_tail...> {
  template <typename Functor, typename BoundArgsTuple, typename... RunArgs>
  static inline void MakeItSo(Functor&& functor,
                              BoundArgsTuple&& bound,
                              RunArgs&&... args) {
    static_assert(index_target == 0);
    // Note the validity of the weak pointer should be tested _after_ it is
    // unwrapped, otherwise it creates a race for weak pointer implementations
    // that allow cross-thread usage and perform `Lock()` in `Unwrap()` traits.
    const auto& target = Unwrap(std::get<0>(bound));
    if (!target) {
      return;
    }
    Traits::Invoke(
        Unwrap(std::forward<Functor>(functor)), target,
        Unwrap(std::get<index_tail>(std::forward<BoundArgsTuple>(bound)))...,
        std::forward<RunArgs>(args)...);
  }
};

// `Invoker<>`
//
// See description at the top of the file.
template <typename Traits, typename StorageType, typename UnboundRunType>
struct Invoker;

template <typename Traits,
          typename StorageType,
          typename R,
          typename... UnboundArgs>
struct Invoker<Traits, StorageType, R(UnboundArgs...)> {
 private:
  using Indices = std::make_index_sequence<
      std::tuple_size_v<decltype(StorageType::bound_args_)>>;

 public:
  static R RunOnce(BindStateBase* base,
                   PassingType<UnboundArgs>... unbound_args) {
    auto* const storage = static_cast<StorageType*>(base);
    return RunImpl(std::move(storage->functor_),
                   std::move(storage->bound_args_), Indices(),
                   std::forward<UnboundArgs>(unbound_args)...);
  }

  static R Run(BindStateBase* base, PassingType<UnboundArgs>... unbound_args) {
    auto* const storage = static_cast<const StorageType*>(base);
    return RunImpl(storage->functor_, storage->bound_args_, Indices(),
                   std::forward<UnboundArgs>(unbound_args)...);
  }

 private:
  // The "templated struct with a lambda that asserts" pattern below is used
  // repeatedly in Bind/Callback code to verify compile-time preconditions. The
  // goal is to print only the root cause failure when users violate a
  // precondition, and not also a host of resulting compile errors.
  //
  // There are three key aspects:
  //   1. By placing the assertion inside a lambda that initializes a variable,
  //      the assertion will not be verified until the compiler tries to read
  //      the value of that variable. This allows the containing types to be
  //      complete. As a result, code that needs to know if the assertion failed
  //      can read the variable's value and get the right answer. (If we instead
  //      placed the assertion at struct scope, the resulting type would be
  //      incomplete when the assertion failed; in practice, reading a
  //      `constexpr` member of an incomplete type seems to return the default
  //      value regardless of what the code tried to set the value to, which
  //      makes it impossible for other code to check whether the assertion
  //      failed.)
  //   2. Code that will not successfully compile unless the assertion holds is
  //      guarded by a constexpr if that checks the variable.
  //   3. By placing the variable inside an independent, templated struct and
  //      naming it `value`, we allow checking multiple conditions via
  //      `std::conjunction_v<>`. This short-circuits type instantiation, so
  //      that when one condition fails, the others are never examined and thus
  //      never assert. As a result, we can verify dependent conditions without
  //      worrying that "if one fails, we'll get errors from several others".
  //      (This would not be true if we simply checked all the values with `&&`,
  //      which would instantiate all the types before evaluating the
  //      expression.)
  //
  // For caller convenience and to avoid potential repetition, the actual
  // condition to be checked is always used as the default value of a template
  // argument, so callers can simply instantiate the struct with no template
  // params to verify the condition.

  // Weak calls are only supported for functions with a `void` return type.
  // Otherwise, the desired function result would be unclear if the `WeakPtr<>`
  // is invalidated. In theory, we could support default-constructible return
  // types (and return the default value) or allow callers to specify a default
  // return value via a template arg. It's not clear these are necessary.
  template <bool is_weak_call, bool v = !is_weak_call || std::is_void_v<R>>
  struct WeakCallReturnsVoid {
    static constexpr bool value = [] {
      static_assert(v,
                    "WeakPtrs can only bind to methods without return values.");
      return v;
    }();
  };

  template <typename Functor, typename BoundArgsTuple, size_t... indices>
  static inline R RunImpl(Functor&& functor,
                          BoundArgsTuple&& bound,
                          std::index_sequence<indices...>,
                          UnboundArgs&&... unbound_args) {
#if PA_BUILDFLAG(USE_ASAN_BACKUP_REF_PTR)
    RawPtrAsanBoundArgTracker raw_ptr_asan_bound_arg_tracker;
    raw_ptr_asan_bound_arg_tracker.AddArgs(
        std::get<indices>(std::forward<BoundArgsTuple>(bound))...,
        std::forward<UnboundArgs>(unbound_args)...);
#endif  // PA_BUILDFLAG(USE_ASAN_BACKUP_REF_PTR)

    using DecayedArgsTuple = std::decay_t<BoundArgsTuple>;
    static constexpr bool kIsWeakCall =
        kIsWeakMethod<Traits::is_method,
                      std::tuple_element_t<indices, DecayedArgsTuple>...>;
    if constexpr (WeakCallReturnsVoid<kIsWeakCall>::value) {
      // Do not `Unwrap()` here, as that immediately triggers dangling pointer
      // detection. Dangling pointer detection should only be triggered if the
      // callback is not cancelled, but cancellation status is not determined
      // until later inside the `InvokeHelper::MakeItSo()` specialization for
      // weak calls.
      //
      // Dangling pointers when invoking a cancelled callback are not considered
      // a memory safety error because protecting raw pointers usage with weak
      // receivers (where the weak receiver usually own the pointed objects) is
      // a common and broadly used pattern in the codebase.
      return InvokeHelper<kIsWeakCall, Traits, R, indices...>::MakeItSo(
          std::forward<Functor>(functor), std::forward<BoundArgsTuple>(bound),
          std::forward<UnboundArgs>(unbound_args)...);
    }
  }
};

// Allow binding a method call with no receiver.
// TODO(crbug.com/41484339): Remove or make safe.
template <typename... Unused>
void VerifyMethodReceiver(Unused&&...) {}

template <typename Receiver, typename... Unused>
void VerifyMethodReceiver(Receiver&& receiver, Unused&&...) {
  // Asserts that a callback is not the first owner of a ref-counted receiver.
  if constexpr (IsPointerOrRawPtr<std::decay_t<Receiver>> &&
                IsRefCountedType<RemovePointerT<std::decay_t<Receiver>>>) {
    DCHECK(receiver);

    // It's error prone to make the implicit first reference to ref-counted
    // types. In the example below, `BindOnce()` would make the implicit first
    // reference to the ref-counted `Foo`. If `PostTask()` failed or the posted
    // task ran fast enough, the newly created instance could be destroyed
    // before `oo` makes another reference.
    // ```
    //   Foo::Foo() {
    //     ThreadPool::PostTask(FROM_HERE, BindOnce(&Foo::Bar, this));
    //   }
    //
    //   scoped_refptr<Foo> oo = new Foo();
    // ```
    //
    // Hence, `Bind()` refuses to create the first reference to ref-counted
    // objects, and `DCHECK()`s otherwise. As above, that typically happens
    // around `PostTask()` in their constructors, and such objects can be
    // destroyed before `new` returns if the tasks resolve fast enough.
    //
    // Instead, consider adding a static factory, and keeping the first
    // reference alive explicitly.
    // ```
    //   // static
    //   scoped_refptr<Foo> Foo::Create() {
    //     auto foo = base::WrapRefCounted(new Foo());
    //     ThreadPool::PostTask(FROM_HERE, BindOnce(&Foo::Bar, foo));
    //     return foo;
    //   }
    //
    //   scoped_refptr<Foo> oo = Foo::Create();
    // ```
    DCHECK(receiver->HasAtLeastOneRef());
  }
}

// `BindState<>`
//
// This stores all the state passed into `Bind()`.
template <bool is_method,
          bool is_nullable,
          bool is_callback,
          typename Functor,
          typename... BoundArgs>
struct BindState final : BindStateBase {
 private:
  using BoundArgsTuple = std::tuple<BoundArgs...>;

 public:
  template <typename ForwardFunctor, typename... ForwardBoundArgs>
  static BindState* Create(BindStateBase::InvokeFuncStorage invoke_func,
                           ForwardFunctor&& functor,
                           ForwardBoundArgs&&... bound_args) {
    if constexpr (is_method) {
      VerifyMethodReceiver(bound_args...);
    }
    return new BindState(invoke_func, std::forward<ForwardFunctor>(functor),
                         std::forward<ForwardBoundArgs>(bound_args)...);
  }

  Functor functor_;
  BoundArgsTuple bound_args_;

 private:
  using CancellationTraits =
      CallbackCancellationTraits<Functor, BoundArgsTuple>;

  template <typename ForwardFunctor, typename... ForwardBoundArgs>
    requires CancellationTraits::is_cancellable
  explicit BindState(BindStateBase::InvokeFuncStorage invoke_func,
                     ForwardFunctor&& functor,
                     ForwardBoundArgs&&... bound_args)
      : BindStateBase(invoke_func, &Destroy, &QueryCancellationTraits),
        functor_(std::forward<ForwardFunctor>(functor)),
        bound_args_(std::forward<ForwardBoundArgs>(bound_args)...) {
    CheckFunctorIsNonNull();
  }

  template <typename ForwardFunctor, typename... ForwardBoundArgs>
    requires(!CancellationTraits::is_cancellable)
  explicit BindState(BindStateBase::InvokeFuncStorage invoke_func,
                     ForwardFunctor&& functor,
                     ForwardBoundArgs&&... bound_args)
      : BindStateBase(invoke_func, &Destroy),
        functor_(std::forward<ForwardFunctor>(functor)),
        bound_args_(std::forward<ForwardBoundArgs>(bound_args)...) {
    CheckFunctorIsNonNull();
  }

  ~BindState() = default;

  static bool QueryCancellationTraits(
      const BindStateBase* base,
      BindStateBase::CancellationQueryMode mode) {
    auto* const storage = static_cast<const BindState*>(base);
    static constexpr std::make_index_sequence<sizeof...(BoundArgs)> kIndices;
    return (mode == BindStateBase::CancellationQueryMode::kIsCancelled)
               ? storage->IsCancelled(kIndices)
               : storage->MaybeValid(kIndices);
  }

  static void Destroy(const BindStateBase* self) {
    delete static_cast<const BindState*>(self);
  }

  // Helpers to do arg tuple expansion.
  template <size_t... indices>
  bool IsCancelled(std::index_sequence<indices...>) const {
    return CancellationTraits::IsCancelled(functor_,
                                           std::get<indices>(bound_args_)...);
  }

  template <size_t... indices>
  bool MaybeValid(std::index_sequence<indices...>) const {
    return CancellationTraits::MaybeValid(functor_,
                                          std::get<indices>(bound_args_)...);
  }

  void CheckFunctorIsNonNull() const {
    if constexpr (is_nullable) {
      // Check the validity of `functor_` to avoid hard-to-diagnose crashes.
      // Ideally we'd do this unconditionally, but release builds limit this to
      // the case of nested callbacks (e.g. `Bind(callback, ...)`) to limit
      // binary size impact.
      if constexpr (is_callback) {
        CHECK(!!functor_);
      } else {
        DCHECK(!!functor_);
      }
    }
  }
};

template <typename... BoundArgs>
struct ValidateBindStateTypeCommonChecks {
 private:
  // Refcounted parameters must be passed as `scoped_refptr` instead of raw
  // pointers, to ensure they are not deleted before use.
  // TODO(danakj): Ban native references and `std::reference_wrapper` too.
  template <typename T,
            bool v =
                (IsRawRef<T> && IsRefCountedType<base::RemoveRawRefT<T>>) ||
                (IsPointerOrRawPtr<T> &&
                 IsRefCountedType<base::RemovePointerT<T>>)>
  struct RefCountedTypeNotPassedByRawPointer {
    static constexpr bool value = [] {
      static_assert(
          !v, "A parameter is a refcounted type and needs scoped_refptr.");
      return !v;
    }();
  };

 public:
  using CommonCheckResult = std::conjunction<
      RefCountedTypeNotPassedByRawPointer<std::decay_t<BoundArgs>>...,
      ValidateStorageTraits<BoundArgs>...>;
};

// Used to determine and validate the appropriate `BindState`. The
// specializations below cover all cases. The members are similar in intent to
// those in `StorageTraits`; see comments there.
template <bool is_method,
          bool is_nullable,
          bool is_callback,
          typename Functor,
          typename... BoundArgs>
struct ValidateBindStateType;

template <bool is_nullable,
          bool is_callback,
          typename Functor,
          typename... BoundArgs>
struct ValidateBindStateType<false,
                             is_nullable,
                             is_callback,
                             Functor,
                             BoundArgs...> {
  using Type = BindState<false,
                         is_nullable,
                         is_callback,
                         std::decay_t<Functor>,
                         typename ValidateStorageTraits<BoundArgs>::Type...>;
  static constexpr bool value =
      ValidateBindStateTypeCommonChecks<BoundArgs...>::CommonCheckResult::value;
};

template <bool is_nullable, bool is_callback, typename Functor>
struct ValidateBindStateType<true, is_nullable, is_callback, Functor> {
  using Type = BindState<true, is_nullable, is_callback, std::decay_t<Functor>>;
  static constexpr bool value = true;
};

template <bool is_nullable,
          bool is_callback,
          typename Functor,
          typename Receiver,
          typename... BoundArgs>
struct ValidateBindStateType<true,
                             is_nullable,
                             is_callback,
                             Functor,
                             Receiver,
                             BoundArgs...> {
 private:
  using DecayedReceiver = std::decay_t<Receiver>;
  using ReceiverStorageType =
      typename MethodReceiverStorage<DecayedReceiver>::Type;

  template <bool v = !std::is_array_v<std::remove_reference_t<Receiver>>>
  struct FirstBoundArgIsNotArray {
    static constexpr bool value = [] {
      static_assert(v, "First bound argument to a method cannot be an array.");
      return v;
    }();
  };

  template <bool v = !IsRawRef<DecayedReceiver>>
  struct ReceiverIsNotRawRef {
    static constexpr bool value = [] {
      static_assert(v, "Receivers may not be raw_ref<T>. If using a raw_ref<T> "
                       "here is safe and has no lifetime concerns, use "
                       "base::Unretained() and document why it's safe.");
      return v;
    }();
  };

  template <bool v = !IsPointerOrRawPtr<DecayedReceiver> ||
                     IsRefCountedType<RemovePointerT<DecayedReceiver>>>
  struct ReceiverIsNotRawPtr {
    static constexpr bool value = [] {
      static_assert(v,
                    "Receivers may not be raw pointers. If using a raw pointer "
                    "here is safe and has no lifetime concerns, use "
                    "base::Unretained() and document why it's safe.");
      return v;
    }();
  };

 public:
  using Type = BindState<true,
                         is_nullable,
                         is_callback,
                         std::decay_t<Functor>,
                         ReceiverStorageType,
                         typename ValidateStorageTraits<BoundArgs>::Type...>;
  static constexpr bool value =
      std::conjunction_v<FirstBoundArgIsNotArray<>,
                         ReceiverIsNotRawRef<>,
                         ReceiverIsNotRawPtr<>,
                         typename ValidateBindStateTypeCommonChecks<
                             BoundArgs...>::CommonCheckResult>;
};

// Transforms `T` into an unwrapped type, which is passed to the target
// function; e.g.:
// * `is_once` cases:
// ** `TransformToUnwrappedType<true, int&&>` -> `int&&`
// ** `TransformToUnwrappedType<true, const int&>` -> `int&&`
// ** `TransformToUnwrappedType<true, OwnedWrapper<int>&>` -> `int*&&`
// * `!is_once` cases:
// ** `TransformToUnwrappedType<false, int&&>` -> `const int&`
// ** `TransformToUnwrappedType<false, const int&>` -> `const int&`
// ** `TransformToUnwrappedType<false, OwnedWrapper<int>&>` -> `int* const &`
template <bool is_once,
          typename T,
          typename StoredType = std::decay_t<T>,
          typename ForwardedType =
              std::conditional_t<is_once, StoredType&&, const StoredType&>>
using TransformToUnwrappedType =
    decltype(Unwrap(std::declval<ForwardedType>()));

// Used to convert `this` arguments to underlying pointer types; e.g.:
//   `int*` -> `int*`
//   `std::unique_ptr<int>` -> `int*`
//   `int` -> (assertion failure; `this` must be a pointer-like object)
template <typename T>
struct ValidateReceiverType {
 private:
  // Pointer-like receivers use a different specialization, so this never
  // succeeds.
  template <bool v = AlwaysFalse<T>>
  struct ReceiverMustBePointerLike {
    static constexpr bool value = [] {
      static_assert(v,
                    "Cannot convert `this` argument to address. Method calls "
                    "must be bound using a pointer-like `this` argument.");
      return v;
    }();
  };

 public:
  // These members are similar in intent to those in `StorageTraits`; see
  // comments there.
  using Type = T;
  static constexpr bool value = ReceiverMustBePointerLike<>::value;
};

template <typename T>
  requires requires(T&& t) { base::to_address(t); }
struct ValidateReceiverType<T> {
  using Type = decltype(base::to_address(std::declval<T>()));
  static constexpr bool value = true;
};

// Transforms `Args` into unwrapped types, and packs them into a `TypeList`. If
// `is_method` is true, tries to dereference the first argument to support smart
// pointers.
template <bool is_once, bool is_method, typename... Args>
struct ValidateUnwrappedTypeList {
  // These members are similar in intent to those in `StorageTraits`; see
  // comments there.
  using Type = TypeList<TransformToUnwrappedType<is_once, Args>...>;
  static constexpr bool value = true;
};

template <bool is_once, typename Receiver, typename... Args>
struct ValidateUnwrappedTypeList<is_once, true, Receiver, Args...> {
 private:
  using ReceiverStorageType =
      typename MethodReceiverStorage<std::decay_t<Receiver>>::Type;
  using UnwrappedReceiver =
      TransformToUnwrappedType<is_once, ReceiverStorageType>;
  using ValidatedReceiver = ValidateReceiverType<UnwrappedReceiver>;

 public:
  using Type = TypeList<typename ValidatedReceiver::Type,
                        TransformToUnwrappedType<is_once, Args>...>;
  static constexpr bool value = ValidatedReceiver::value;
};

// `IsUnretainedMayDangle` is true iff `StorageType` is marked with
// `unretained_traits::MayDangle`. Note that it is false for
// `unretained_traits::MayDangleUntriaged`.
template <typename StorageType>
inline constexpr bool IsUnretainedMayDangle = false;

template <typename T, RawPtrTraits PtrTraits>
inline constexpr bool IsUnretainedMayDangle<
    UnretainedWrapper<T, unretained_traits::MayDangle, PtrTraits>> = true;

// `UnretainedAndRawPtrHaveCompatibleTraits` is true iff `StorageType` is marked
// with `unretained_traits::MayDangle`, `FunctionParamType` is a `raw_ptr`, and
// `StorageType::GetPtrType` is the same type as `FunctionParamType`.
template <typename StorageType, typename FunctionParamType>
inline constexpr bool UnretainedAndRawPtrHaveCompatibleTraits = false;

template <typename T,
          RawPtrTraits PtrTraitsInUnretained,
          RawPtrTraits PtrTraitsInReceiver>
inline constexpr bool UnretainedAndRawPtrHaveCompatibleTraits<
    UnretainedWrapper<T, unretained_traits::MayDangle, PtrTraitsInUnretained>,
    raw_ptr<T, PtrTraitsInReceiver>> =
    std::same_as<typename UnretainedWrapper<T,
                                            unretained_traits::MayDangle,
                                            PtrTraitsInUnretained>::GetPtrType,
                 raw_ptr<T, PtrTraitsInReceiver>>;

// Helpers to make error messages slightly more readable.
template <int i>
struct BindArgument {
  template <typename ForwardingType>
  struct ForwardedAs {
    template <typename FunctorParamType>
    struct ToParamWithType {
      static constexpr bool kRawPtr = IsRawPtr<FunctorParamType>;
      static constexpr bool kRawPtrMayBeDangling =
          IsRawPtrMayDangle<FunctorParamType>;
      static constexpr bool kCanBeForwardedToBoundFunctor =
          std::is_convertible_v<ForwardingType, FunctorParamType>;

      // If the bound type can't be forwarded, then test if `FunctorParamType`
      // is a non-const lvalue reference and a reference to the unwrapped type
      // could have been successfully forwarded.
      static constexpr bool kIsUnwrappedForwardableNonConstReference =
          std::is_lvalue_reference_v<FunctorParamType> &&
          !std::is_const_v<std::remove_reference_t<FunctorParamType>> &&
          std::is_convertible_v<std::decay_t<ForwardingType>&,
                                FunctorParamType>;

      // Also intentionally drop the `const` qualifier from `ForwardingType`, to
      // test if it could have been successfully forwarded if `Passed()` had
      // been used.
      static constexpr bool kWouldBeForwardableWithPassed =
          std::is_convertible_v<std::decay_t<ForwardingType>&&,
                                FunctorParamType>;
    };
  };

  template <typename BoundAsType>
  struct BoundAs {
    template <typename StorageType>
    struct StoredAs {
      static constexpr bool kBindArgumentCanBeCaptured =
          std::constructible_from<StorageType, BoundAsType>;

      // If the argument can't be captured, intentionally drop the `const`
      // qualifier from `BoundAsType`, to test if it could have been
      // successfully captured if `std::move()` had been used.
      static constexpr bool kWouldBeCapturableWithStdMove =
          std::constructible_from<StorageType, std::decay_t<BoundAsType>&&>;
    };
  };

  template <typename FunctionParamType>
  struct ToParamWithType {
    template <typename StorageType>
    struct StoredAs {
      static constexpr bool kBoundPtrMayDangle =
          IsUnretainedMayDangle<StorageType>;

      static constexpr bool kMayDangleAndMayBeDanglingHaveMatchingTraits =
          UnretainedAndRawPtrHaveCompatibleTraits<StorageType,
                                                  FunctionParamType>;
    };
  };
};

// Helper to assert that parameter `i` of type `Arg` can be bound, which means:
// * `Arg` can be retained internally as `Storage`
// * `Arg` can be forwarded as `Unwrapped` to `Param`
template <int i,
          bool is_method,
          typename Arg,
          typename Storage,
          typename Unwrapped,
          typename Param>
struct ParamCanBeBound {
 private:
  using UnwrappedParam = BindArgument<i>::template ForwardedAs<
      Unwrapped>::template ToParamWithType<Param>;
  using ParamStorage = BindArgument<i>::template ToParamWithType<
      Param>::template StoredAs<Storage>;
  using BoundStorage =
      BindArgument<i>::template BoundAs<Arg>::template StoredAs<Storage>;

  template <bool v = !UnwrappedParam::kRawPtr ||
                     UnwrappedParam::kRawPtrMayBeDangling>
  struct NotRawPtr {
    static constexpr bool value = [] {
      static_assert(
          v, "Use T* or T& instead of raw_ptr<T> for function parameters, "
             "unless you must mark the parameter as MayBeDangling<T>.");
      return v;
    }();
  };

  template <bool v = !ParamStorage::kBoundPtrMayDangle ||
                     UnwrappedParam::kRawPtrMayBeDangling ||
                     // Exempt `this` pointer as it is not passed as a regular
                     // function argument.
                     (is_method && i == 0)>
  struct MayBeDanglingPtrPassedCorrectly {
    static constexpr bool value = [] {
      static_assert(v, "base::UnsafeDangling() pointers should only be passed "
                       "to parameters marked MayBeDangling<T>.");
      return v;
    }();
  };

  template <bool v =
                !UnwrappedParam::kRawPtrMayBeDangling ||
                (ParamStorage::kBoundPtrMayDangle &&
                 ParamStorage::kMayDangleAndMayBeDanglingHaveMatchingTraits)>
  struct MayDangleAndMayBeDanglingHaveMatchingTraits {
    static constexpr bool value = [] {
      static_assert(
          v, "Pointers passed to MayBeDangling<T> parameters must be created "
             "by base::UnsafeDangling() with the same RawPtrTraits.");
      return v;
    }();
  };

  // With `BindRepeating()`, there are two decision points for how to handle a
  // move-only type:
  //
  // 1. Whether the move-only argument should be moved into the internal
  //    `BindState`. Either `std::move()` or `Passed()` is sufficient to trigger
  //    move-only semantics.
  // 2. Whether or not the bound, move-only argument should be moved to the
  //    bound functor when invoked. When the argument is bound with `Passed()`,
  //    invoking the callback will destructively move the bound, move-only
  //    argument to the bound functor. In contrast, if the argument is bound
  //    with `std::move()`, `RepeatingCallback` will attempt to call the bound
  //    functor with a constant reference to the bound, move-only argument. This
  //    will fail if the bound functor accepts that argument by value, since the
  //    argument cannot be copied. It is this latter case that this
  //    assertion aims to catch.
  //
  // In contrast, `BindOnce()` only has one decision point. Once a move-only
  // type is captured by value into the internal `BindState`, the bound,
  // move-only argument will always be moved to the functor when invoked.
  // Failure to use `std::move()` will simply fail the
  // `MoveOnlyTypeMustUseStdMove` assertion below instead.
  //
  // Note: `Passed()` is a legacy of supporting move-only types when repeating
  // callbacks were the only callback type. A `RepeatingCallback` with a
  // `Passed()` argument is really a `OnceCallback` and should eventually be
  // migrated.
  template <bool v = UnwrappedParam::kCanBeForwardedToBoundFunctor ||
                     !UnwrappedParam::kWouldBeForwardableWithPassed>
  struct MoveOnlyTypeMustUseBasePassed {
    static constexpr bool value = [] {
      static_assert(v,
                    "base::BindRepeating() argument is a move-only type. Use "
                    "base::Passed() instead of std::move() to transfer "
                    "ownership from the callback to the bound functor.");
      return v;
    }();
  };

  template <bool v = UnwrappedParam::kCanBeForwardedToBoundFunctor ||
                     !UnwrappedParam::kIsUnwrappedForwardableNonConstReference>
  struct NonConstRefParamMustBeWrapped {
    static constexpr bool value = [] {
      static_assert(v,
                    "Bound argument for non-const reference parameter must be "
                    "wrapped in std::ref() or base::OwnedRef().");
      return v;
    }();
  };

  // Generic failed-to-forward message for cases that didn't match one of the
  // two assertions above.
  template <bool v = UnwrappedParam::kCanBeForwardedToBoundFunctor>
  struct CanBeForwardedToBoundFunctor {
    static constexpr bool value = [] {
      static_assert(v,
                    "Type mismatch between bound argument and bound functor's "
                    "parameter.");
      return v;
    }();
  };

  // The most common reason for failing to capture a parameter is attempting to
  // pass a move-only type as an lvalue.
  template <bool v = BoundStorage::kBindArgumentCanBeCaptured ||
                     !BoundStorage::kWouldBeCapturableWithStdMove>
  struct MoveOnlyTypeMustUseStdMove {
    static constexpr bool value = [] {
      static_assert(v,
                    "Attempting to bind a move-only type. Use std::move() to "
                    "transfer ownership to the created callback.");
      return v;
    }();
  };

  // Any other reason the parameter could not be captured.
  template <bool v = BoundStorage::kBindArgumentCanBeCaptured>
  struct BindArgumentCanBeCaptured {
    static constexpr bool value = [] {
      // In practice, failing this precondition should be rare, as the storage
      // type is deduced from the arguments passed to `Bind()`.
      static_assert(
          v, "Cannot capture argument: is the argument copyable or movable?");
      return v;
    }();
  };

 public:
  static constexpr bool value =
      std::conjunction_v<NotRawPtr<>,
                         MayBeDanglingPtrPassedCorrectly<>,
                         MayDangleAndMayBeDanglingHaveMatchingTraits<>,
                         MoveOnlyTypeMustUseBasePassed<>,
                         NonConstRefParamMustBeWrapped<>,
                         CanBeForwardedToBoundFunctor<>,
                         MoveOnlyTypeMustUseStdMove<>,
                         BindArgumentCanBeCaptured<>>;
};

// Takes three same-length `TypeList`s, and checks `ParamCanBeBound` for each
// triple.
template <bool is_method,
          typename Index,
          typename Args,
          typename UnwrappedTypeList,
          typename ParamsList>
struct ParamsCanBeBound {
  static constexpr bool value = false;
};

template <bool is_method,
          size_t... Ns,
          typename... Args,
          typename... UnwrappedTypes,
          typename... Params>
struct ParamsCanBeBound<is_method,
                        std::index_sequence<Ns...>,
                        TypeList<Args...>,
                        TypeList<UnwrappedTypes...>,
                        TypeList<Params...>> {
  static constexpr bool value =
      std::conjunction_v<ParamCanBeBound<Ns,
                                         is_method,
                                         Args,
                                         std::decay_t<Args>,
                                         UnwrappedTypes,
                                         Params>...>;
};

// Core implementation of `Bind()`, which checks common preconditions before
// returning an appropriate callback.
template <template <typename> class CallbackT>
struct BindHelper {
 private:
  static constexpr bool kIsOnce =
      is_instantiation<OnceCallback, CallbackT<void()>>;

  template <typename Traits, bool v = IsComplete<Traits>>
  struct TraitsAreInstantiable {
    static constexpr bool value = [] {
      static_assert(
          v, "Could not determine how to invoke functor. If this functor has "
             "an overloaded operator()(), bind all arguments to it, and ensure "
             "the result will select a unique overload.");
      return v;
    }();
  };

  template <typename Functor,
            bool v = !is_instantiation<OnceCallback, std::decay_t<Functor>> ||
                     (kIsOnce && std::is_rvalue_reference_v<Functor&&> &&
                      !std::is_const_v<std::remove_reference_t<Functor>>)>
  struct OnceCallbackFunctorIsValid {
    static constexpr bool value = [] {
      if constexpr (kIsOnce) {
        static_assert(v,
                      "BindOnce() requires non-const rvalue for OnceCallback "
                      "binding, i.e. base::BindOnce(std::move(callback)).");
      } else {
        static_assert(v, "BindRepeating() cannot bind OnceCallback. Use "
                         "BindOnce() with std::move().");
      }
      return v;
    }();
  };

  template <typename... Args>
  struct NoBindArgToOnceCallbackIsBasePassed {
    static constexpr bool value = [] {
      // Can't use a defaulted template param since it can't come after `Args`.
      constexpr bool v =
          !kIsOnce ||
          (... && !is_instantiation<PassedWrapper, std::decay_t<Args>>);
      static_assert(
          v,
          "Use std::move() instead of base::Passed() with base::BindOnce().");
      return v;
    }();
  };

  template <
      typename Functor,
      bool v =
          !is_instantiation<FunctionRef, std::remove_cvref_t<Functor>> &&
          !is_instantiation<absl::FunctionRef, std::remove_cvref_t<Functor>>>
  struct NotFunctionRef {
    static constexpr bool value = [] {
      static_assert(
          v,
          "Functor may not be a FunctionRef, since that is a non-owning "
          "reference that may go out of scope before the callback executes.");
      return v;
    }();
  };

  template <typename Traits, bool v = Traits::is_stateless>
  struct IsStateless {
    static constexpr bool value = [] {
      static_assert(
          v, "Capturing lambdas and stateful functors are intentionally not "
             "supported. Use a non-capturing lambda or stateless functor (i.e. "
             "has no non-static data members) and bind arguments directly.");
      return v;
    }();
  };

  template <typename Functor, typename... Args>
  static auto BindImpl(Functor&& functor, Args&&... args) {
    // There are a lot of variables and type aliases here. An example will be
    // illustrative. Assume we call:
    // ```
    //   struct S {
    //     double f(int, const std::string&);
    //   } s;
    //   int16_t i;
    //   BindOnce(&S::f, Unretained(&s), i);
    // ```
    // This means our template params are:
    // ```
    //   template <typename> class CallbackT = OnceCallback
    //   typename Functor = double (S::*)(int, const std::string&)
    //   typename... Args =
    //       UnretainedWrapper<S, unretained_traits::MayNotDangle>, int16_t
    // ```
    // And the implementation below is effectively:
    // ```
    //   using Traits = struct {
    //     using RunType = double(S*, int, const std::string&);
    //     static constexpr bool is_method = true;
    //     static constexpr bool is_nullable = true;
    //     static constexpr bool is_callback = false;
    //     static constexpr bool is_stateless = true;
    //     ...
    //   };
    //   using ValidatedUnwrappedTypes = struct {
    //     using Type = TypeList<S*, int16_t>;
    //     static constexpr bool value = true;
    //   };
    //   using BoundArgsList = TypeList<S*, int16_t>;
    //   using RunParamsList = TypeList<S*, int, const std::string&>;
    //   using BoundParamsList = TypeList<S*, int>;
    //   using ValidatedBindState = struct {
    //     using Type =
    //         BindState<double (S::*)(int, const std::string&),
    //                   UnretainedWrapper<S, unretained_traits::MayNotDangle>,
    //                   int16_t>;
    //     static constexpr bool value = true;
    //   };
    //   if constexpr (true) {
    //     using UnboundRunType = double(const std::string&);
    //     using CallbackType = OnceCallback<double(const std::string&)>;
    //     ...
    // ```
    using Traits = FunctorTraits<TransformToUnwrappedType<kIsOnce, Functor&&>,
                                 TransformToUnwrappedType<kIsOnce, Args&&>...>;
    if constexpr (TraitsAreInstantiable<Traits>::value) {
      using ValidatedUnwrappedTypes =
          ValidateUnwrappedTypeList<kIsOnce, Traits::is_method, Args&&...>;
      using BoundArgsList = TypeList<Args...>;
      using RunParamsList = ExtractArgs<typename Traits::RunType>;
      using BoundParamsList = TakeTypeListItem<sizeof...(Args), RunParamsList>;
      using ValidatedBindState =
          ValidateBindStateType<Traits::is_method, Traits::is_nullable,
                                Traits::is_callback, Functor, Args...>;
      // This conditional checks if each of the `args` matches to the
      // corresponding param of the target function. This check does not affect
      // the behavior of `Bind()`, but its error message should be more
      // readable.
      if constexpr (std::conjunction_v<
                        NotFunctionRef<Functor>, IsStateless<Traits>,
                        ValidatedUnwrappedTypes,
                        ParamsCanBeBound<
                            Traits::is_method,
                            std::make_index_sequence<sizeof...(Args)>,
                            BoundArgsList,
                            typename ValidatedUnwrappedTypes::Type,
                            BoundParamsList>,
                        ValidatedBindState>) {
        using UnboundRunType =
            MakeFunctionType<ExtractReturnType<typename Traits::RunType>,
                             DropTypeListItem<sizeof...(Args), RunParamsList>>;
        using CallbackType = CallbackT<UnboundRunType>;

        // Store the invoke func into `PolymorphicInvoke` before casting it to
        // `InvokeFuncStorage`, so that we can ensure its type matches to
        // `PolymorphicInvoke`, to which `CallbackType` will cast back.
        typename CallbackType::PolymorphicInvoke invoke_func;
        using Invoker =
            Invoker<Traits, typename ValidatedBindState::Type, UnboundRunType>;
        if constexpr (kIsOnce) {
          invoke_func = Invoker::RunOnce;
        } else {
          invoke_func = Invoker::Run;
        }

        return CallbackType(ValidatedBindState::Type::Create(
            reinterpret_cast<BindStateBase::InvokeFuncStorage>(invoke_func),
            std::forward<Functor>(functor), std::forward<Args>(args)...));
      }
    }
  }

  // Special cases for binding to a `Callback` without extra bound arguments.

  // `OnceCallback` passed to `OnceCallback`, or `RepeatingCallback` passed to
  // `RepeatingCallback`.
  template <typename T>
    requires is_instantiation<CallbackT, T>
  static T BindImpl(T callback) {
    // Guard against null pointers accidentally ending up in posted tasks,
    // causing hard-to-debug crashes.
    CHECK(callback);
    return callback;
  }

  // `RepeatingCallback` passed to `OnceCallback`. The opposite direction is
  // intentionally not supported.
  template <typename Signature>
    requires is_instantiation<CallbackT, OnceCallback<Signature>>
  static OnceCallback<Signature> BindImpl(
      RepeatingCallback<Signature> callback) {
    return BindImpl(OnceCallback<Signature>(callback));
  }

  // Must be defined after `BindImpl()` since it refers to it.
  template <typename Functor, typename... Args>
  struct BindImplWouldSucceed {
    // Can't use a defaulted template param since it can't come after `Args`.
    //
    // Determining if `BindImpl()` would succeed is not as simple as verifying
    // any conditions it checks directly; those only control when it's safe to
    // call other methods, which in turn may fail. However, ultimately, any
    // failure will result in returning `void`, so check for a non-`void` return
    // type.
    static constexpr bool value =
        !std::same_as<void,
                      decltype(BindImpl(std::declval<Functor&&>(),
                                        std::declval<Args&&>()...))>;
  };

 public:
  template <typename Functor, typename... Args>
  static auto Bind(Functor&& functor, Args&&... args) {
    if constexpr (std::conjunction_v<
                      OnceCallbackFunctorIsValid<Functor>,
                      NoBindArgToOnceCallbackIsBasePassed<Args...>,
                      BindImplWouldSucceed<Functor, Args...>>) {
      return BindImpl(std::forward<Functor>(functor),
                      std::forward<Args>(args)...);
    } else {
      return BindFailedCheckPreviousErrors();
    }
  }
};

}  // namespace internal

// An injection point to control `this` pointer behavior on a method invocation.
// If `IsWeakReceiver<T>::value` is `true` and `T` is used as a method receiver,
// `Bind()` cancels the method invocation if the receiver tests as false.
// ```
//   struct S {
//     void f() {}
//   };
//
//   WeakPtr<S> weak_s = nullptr;
//   BindOnce(&S::f, weak_s).Run();  // `S::f()` is not called.
// ```
template <typename T>
struct IsWeakReceiver : std::bool_constant<is_instantiation<WeakPtr, T>> {};

template <typename T>
struct IsWeakReceiver<std::reference_wrapper<T>> : IsWeakReceiver<T> {};

// An injection point to control how objects are checked for maybe validity,
// which is an optimistic thread-safe check for full validity.
template <typename>
struct MaybeValidTraits {
  template <typename T>
  static bool MaybeValid(const T& o) {
    return o.MaybeValid();
  }
};

// An injection point to control how bound objects passed to the target
// function. `BindUnwrapTraits<>::Unwrap()` is called for each bound object
// right before the target function is invoked.
template <typename>
struct BindUnwrapTraits {
  template <typename T>
  static T&& Unwrap(T&& o) {
    return std::forward<T>(o);
  }
};

template <typename T>
  requires internal::kIsUnretainedWrapper<internal::UnretainedWrapper, T> ||
           internal::kIsUnretainedWrapper<internal::UnretainedRefWrapper, T> ||
           is_instantiation<internal::RetainedRefWrapper, T> ||
           is_instantiation<internal::OwnedWrapper, T> ||
           is_instantiation<internal::OwnedRefWrapper, T>
struct BindUnwrapTraits<T> {
  static decltype(auto) Unwrap(const T& o) { return o.get(); }
};

template <typename T>
struct BindUnwrapTraits<internal::PassedWrapper<T>> {
  static T Unwrap(const internal::PassedWrapper<T>& o) { return o.Take(); }
};

#if BUILDFLAG(IS_WIN)
template <typename T>
struct BindUnwrapTraits<Microsoft::WRL::ComPtr<T>> {
  static T* Unwrap(const Microsoft::WRL::ComPtr<T>& ptr) { return ptr.Get(); }
};
#endif

// `CallbackCancellationTraits` allows customization of `Callback`'s
// cancellation semantics. By default, callbacks are not cancellable. A
// specialization should set `is_cancellable` and implement an `IsCancelled()`
// that returns whether the callback should be cancelled, as well as a
// `MaybeValid()` that returns whether the underlying functor/object is maybe
// valid.
template <typename Functor, typename BoundArgsTuple>
struct CallbackCancellationTraits {
  static constexpr bool is_cancellable = false;
};

// Specialization for a weak receiver.
template <typename Functor, typename... BoundArgs>
  requires internal::kIsWeakMethod<
      internal::FunctorTraits<Functor, BoundArgs...>::is_method,
      BoundArgs...>
struct CallbackCancellationTraits<Functor, std::tuple<BoundArgs...>> {
  static constexpr bool is_cancellable = true;

  template <typename Receiver, typename... Args>
  static bool IsCancelled(const Functor&,
                          const Receiver& receiver,
                          const Args&...) {
    return !receiver;
  }

  template <typename Receiver, typename... Args>
  static bool MaybeValid(const Functor&,
                         const Receiver& receiver,
                         const Args&...) {
    return MaybeValidTraits<Receiver>::MaybeValid(receiver);
  }
};

// Specialization for a nested `Bind()`.
template <typename Functor, typename... BoundArgs>
  requires is_instantiation<OnceCallback, Functor> ||
           is_instantiation<RepeatingCallback, Functor>
struct CallbackCancellationTraits<Functor, std::tuple<BoundArgs...>> {
  static constexpr bool is_cancellable = true;

  static bool IsCancelled(const Functor& functor, const BoundArgs&...) {
    return functor.IsCancelled();
  }

  static bool MaybeValid(const Functor& functor, const BoundArgs&...) {
    return MaybeValidTraits<Functor>::MaybeValid(functor);
  }
};

}  // namespace base

#endif  // BASE_FUNCTIONAL_BIND_INTERNAL_H_
