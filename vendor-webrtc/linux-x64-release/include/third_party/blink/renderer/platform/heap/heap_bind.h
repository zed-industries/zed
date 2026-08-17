// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_BIND_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_BIND_H_

#include "base/functional/function_ref.h"
#include "base/types/is_instantiation.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/trace_traits.h"

// Synopsys:
//
// HeapCallback<Signature> HeapBind(Functor, Args...)
//
// Example:
//
// class A : publlic GarbageCollected<A> {
//  public:
//   void Frobnicate(const String& s, int a);
//   void Trace(Visitor* visitor) const;
// };
//
// HeapCallback<void(int)> callback =
//     HeapBind(&A::Frobnicate, MakeGarbageCollected<A>(), "foo");
// ...
// callback.Run(42);
//
// Notes and limitations:
// - HeapCallback<Signature> DISALLOW_NEW() (similar to Member<>) and must
//    be traced;
// - There's no Persistent<> counterpart to HeapCallback<>. If you want to
//    use it in a non-GC class, just use regular base::Bind();
// - Supported Callables are standalone functions, pointer to members,
//    captureless lambdas and HeapCallbacks.
// - There's no support for weak receivers (yet);
// - Methods must accept GarbageCollected classes by pointers if these are to be
// bound;
// - Only pointers to GarbageCollected types can be bound;
// - base::Unretained() is not supported;

namespace blink::bindings {

// Implementation notes:
// `HeapCallback<>` is essentially a `Member<>`-like wrapper for a pointer
// to `HeapCallback::Closure<>`, which is a pure interface implemented by
// `HeapCallbackClosureImpl<>.` The latter stores actual functor and state,
// and is the only one that knows the type of bound arguments. The former
// two are specialized by callback _signature_, which is essentially a
// function signature of the `HeapCallback::Run()` method.
namespace internal {
template <typename Functor, typename BoundArgs, typename FreeArgs>
class HeapCallbackClosureImpl;
}  // namespace internal

template <typename CallbackSignature>
class HeapCallback;

template <typename Ret, typename... Args>
class HeapCallback<Ret(Args...)> final {
  DISALLOW_NEW();

  class Closure;

  template <typename Functor, typename BoundArgs, typename FreeArgs>
  friend class internal::HeapCallbackClosureImpl;

 public:
  HeapCallback() = default;
  explicit HeapCallback(Closure* closure) : closure_(closure) {}
  explicit operator bool() const { return !!closure_; }

  bool operator==(const HeapCallback& r) const = default;
  bool operator!=(const HeapCallback& r) const = default;

  Ret Run(Args... args) {
    DCHECK(closure_);
    return closure_->Run(std::forward<Args>(args)...);
  }
  void Trace(Visitor* visitor) const { visitor->Trace(closure_); }

 private:
  Member<Closure> closure_;
};

template <typename Ret, typename... Args>
class HeapCallback<Ret(Args...)>::Closure
    : public GarbageCollected<HeapCallback<Ret(Args...)>::Closure> {
 public:
  using SignatureType = Ret(Args...);

  virtual Ret Run(Args... args) = 0;
  virtual void Trace(Visitor* visitor) const {}
  // Required for properly destroying bound non-trivially destructable types.
  virtual ~Closure() = default;
};

namespace internal {

// A trivial wrapper around individual bound argument, which just selects an
// appropriate backing type (e.g. a Member<> for GC'ed types) and traces it
// (if required).
template <typename Arg>
class ArgStorage final {
  DISALLOW_NEW();

 private:
  static_assert(!std::is_reference_v<Arg>);  // Caller should remove_cvref.
  using StorageType = std::conditional_t<std::is_pointer_v<Arg>,
                                         Member<std::remove_pointer_t<Arg>>,
                                         Arg>;
  using PassType = std::conditional_t<std::is_pointer_v<Arg>, Arg, Arg&>;

 public:
  static_assert(!IsGarbageCollectedType<std::remove_pointer_t<Arg>>::value ||
                    std::is_pointer_v<Arg>,
                "GarbageCollected classes should be bound as pointers");
  static_assert(!std::is_pointer_v<Arg> ||
                    IsGarbageCollectedType<std::remove_pointer_t<Arg>>::value,
                "Only pointers to GarbageCollected types may be bound");

  template <typename PassedType>
  explicit ArgStorage(PassedType&& arg)
      : storage_(std::forward<PassedType>(arg)) {}

  void Trace(Visitor* visitor) const {
    blink::TraceIfNeeded<StorageType>::Trace(visitor, storage_);
  }

  PassType Unwrap() {
    if constexpr (std::is_pointer_v<Arg>) {
      return storage_.Get();
    } else {
      return storage_;
    }
  }

 private:
  StorageType storage_;
};

// A tuple of ArgStorage for all arguments.
template <bool arg0_is_nullable, typename Tuple, typename IndexSequence>
class BoundState;

template <bool arg0_is_nullable, typename... Args, size_t... index>
class BoundState<arg0_is_nullable,
                 std::tuple<Args...>,
                 std::integer_sequence<size_t, index...>>
    final {
  DISALLOW_NEW();

 public:
  template <typename... PassedArgs>
  BoundState(PassedArgs&&... args)
      : storage_(std::forward<PassedArgs>(args)...) {
    if constexpr (!arg0_is_nullable && sizeof...(args) > 0) {
      CHECK(std::get<0>(storage_).Unwrap())
          << "Receiver argument must not be null";
    }
  }

  void Trace(Visitor* visitor) const {
    (...,
     blink::TraceIfNeeded<typename std::tuple_element<
         index, StorageType>::type>::Trace(visitor, std::get<index>(storage_)));
  }

  template <typename Functor, typename... FreeArgs>
  auto Run(Functor&& functor, FreeArgs&&... free_args) {
    if constexpr (base::is_instantiation<HeapCallback,
                                         std::remove_cvref_t<Functor>>) {
      return std::forward<Functor>(functor).Run(
          std::get<index>(storage_).Unwrap()...,
          std::forward<FreeArgs>(free_args)...);
    } else {
      return std::invoke(std::forward<Functor>(functor),
                         std::get<index>(storage_).Unwrap()...,
                         std::forward<FreeArgs>(free_args)...);
    }
  }

 private:
  using StorageType = std::tuple<ArgStorage<std::remove_cvref_t<Args>>...>;
  StorageType storage_;
};

// `FunctorTraits<>` are internally specialized for different functors and
// help with properly extracting types for return value and arguments, as well
// as some other properties (such as whether we should null-check receiver
// args). The supported types include standalone functions, methods and
// callabcle classes, including captureless lambdas.
template <typename Functor>
struct FunctorTraits;

template <typename R, typename... Args>
struct FunctorTraits<R (*)(Args...)> {
  using return_t = R;
  using args_t = std::tuple<Args...>;
  static constexpr bool is_first_arg_nullable = true;
};
template <typename R, typename C, typename... Args>
struct FunctorTraits<R (C::*)(Args...)> {
  using return_t = R;
  using args_t = std::tuple<C*, Args...>;
  static constexpr bool is_first_arg_nullable = false;
};
template <typename R, typename C, typename... Args>
struct FunctorTraits<R (C::*)(Args...) const> {
  using return_t = R;
  using args_t = std::tuple<const C*, Args...>;
  static constexpr bool is_first_arg_nullable = false;
};
template <typename R, typename... Args>
struct FunctorTraits<HeapCallback<R(Args...)>> {
  using return_t = R;
  using args_t = std::tuple<Args...>;
  static constexpr bool is_first_arg_nullable = true;
};

template <typename Functor>
struct FunctorTraitsForCallable;
template <typename R, typename C, typename... Args>
struct FunctorTraitsForCallable<R (C::*)(Args...) const>
    : public FunctorTraits<R (*)(Args...)> {
  static_assert(!base::is_instantiation<base::FunctionRef, C>,
                "base::FunctionRef<> can't be bound");
  static_assert(std::is_empty_v<C>, "Capturing lambdas can't be bound");
};

// This covers everything with a non-overloaded operator(), including
// lambdas.
template <typename C>
concept IsCallable = requires { decltype (&C::operator())(); };

template <typename C>
  requires(IsCallable<C> && !std::is_function_v<C>)
struct FunctorTraits<C>
    : public FunctorTraitsForCallable<decltype(&C::operator())> {};

// HeapCallbackClosureImpl carries actual functor and the bound arguments.
template <typename Functor, typename BoundArgs, typename FreeArgs>
class HeapCallbackClosureImpl;

template <typename Functor, typename... BoundArgs, typename... FreeArgs>
class HeapCallbackClosureImpl<Functor,
                              std::tuple<BoundArgs...>,
                              std::tuple<FreeArgs...>>
    final : public HeapCallback<typename FunctorTraits<Functor>::return_t(
                FreeArgs...)>::Closure {
 private:
  using return_t = typename FunctorTraits<Functor>::return_t;

 public:
  template <typename... PassedArgs>
  HeapCallbackClosureImpl(Functor functor, PassedArgs&&... args)
      : functor_(functor), state_(std::forward<PassedArgs>(args)...) {
    CHECK(functor_);
  }

  HeapCallbackClosureImpl(const HeapCallbackClosureImpl& r) = delete;
  HeapCallbackClosureImpl(HeapCallbackClosureImpl&& r) = delete;

  return_t Run(FreeArgs... args) final {
    return state_.Run(functor_, std::move(args)...);
  }

  void Trace(Visitor* visitor) const final {
    blink::TraceIfNeeded<Functor>::Trace(visitor, functor_);
    state_.Trace(visitor);
  }

 private:
  Functor functor_;
  BoundState<FunctorTraits<Functor>::is_first_arg_nullable,
             std::tuple<BoundArgs...>,
             std::make_index_sequence<sizeof...(BoundArgs)>>
      state_;
};

// SplitAtN<> takes a tuple and splits it two tuples after Nth element
// (first N elements go to head_t, the rest go into tail_t).
template <size_t N, typename Head, typename Tail>
struct SplitAtN;

template <typename... HeadArgs, typename... TailArgs>
struct SplitAtN<0, std::tuple<HeadArgs...>, std::tuple<TailArgs...>> {
  using head_t = std::tuple<HeadArgs...>;
  using tail_t = std::tuple<TailArgs...>;
};

template <size_t N, typename... HeadArgs, typename CAR, typename... TailArgs>
  requires(N > 0)
struct SplitAtN<N, std::tuple<HeadArgs...>, std::tuple<CAR, TailArgs...>>
    : public SplitAtN<N - 1,
                      std::tuple<HeadArgs..., CAR>,
                      std::tuple<TailArgs...>> {};

// SplitArgs<> takes a functor and a pack of bound args and utilizes
// SplitAtN<> to split functor args into bound and free ones.
template <typename Functor, typename... BoundArgs>
class SplitArgs {
 private:
  using split_args_t = SplitAtN<sizeof...(BoundArgs),
                                std::tuple<>,
                                typename FunctorTraits<Functor>::args_t>;

 public:
  using bound_args_t = typename split_args_t::head_t;
  using free_args_t = typename split_args_t::tail_t;
};

}  // namespace internal

template <typename Functor, typename... BoundArgs>
[[nodiscard]] auto HeapBind(Functor functor, BoundArgs&&... args) {
  using SplitArgs = internal::SplitArgs<Functor, BoundArgs...>;
  using ClosureType =
      internal::HeapCallbackClosureImpl<Functor,
                                        typename SplitArgs::bound_args_t,
                                        typename SplitArgs::free_args_t>;
  // force-instantiate closure type to trigger all possible asserts there.
  static_assert(std::is_function_v<typename ClosureType::SignatureType>);
  using SignatureType = typename ClosureType::SignatureType;
  using CallbackType = HeapCallback<SignatureType>;
  // Don't do extra wrapping if there's nothing to bind.
  if constexpr (sizeof...(BoundArgs) == 0 &&
                std::is_same_v<std::remove_cvref_t<Functor>, CallbackType>) {
    return functor;
  }

  return CallbackType(MakeGarbageCollected<ClosureType>(
      functor, std::forward<BoundArgs>(args)...));
}

template <typename Ret, typename... Preargs, typename... Args>
HeapCallback<Ret(Preargs..., Args...)> IgnoreArgs(
    HeapCallback<Ret(Args...)> callback) {
  return callback ? HeapBind(
                        [](HeapCallback<Ret(Args...)> callback, Preargs...,
                           Args&&... args) {
                          return callback.Run(std::forward<Args>(args)...);
                        },
                        callback)
                  : HeapCallback<Ret(Preargs..., Args...)>();
}

}  // namespace blink::bindings

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_BIND_H_
