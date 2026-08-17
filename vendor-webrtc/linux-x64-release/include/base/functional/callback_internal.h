// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains utility functions and classes that help the
// implementation, and management of the Callback objects.

#ifndef BASE_FUNCTIONAL_CALLBACK_INTERNAL_H_
#define BASE_FUNCTIONAL_CALLBACK_INTERNAL_H_

#include <type_traits>
#include <utility>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/functional/callback_forward.h"
#include "base/memory/ref_counted.h"

namespace base {

struct FakeBindState;

namespace internal {

class BindStateBase;

template <bool is_method,
          bool is_nullable,
          bool is_callback,
          typename Functor,
          typename... BoundArgs>
struct BindState;

struct BASE_EXPORT BindStateBaseRefCountTraits {
  static void Destruct(const BindStateBase*);
};

template <typename T>
using PassingType = std::conditional_t<std::is_scalar_v<T>, T, T&&>;

// BindStateBase is used to provide an opaque handle that the Callback
// class can use to represent a function object with bound arguments.  It
// behaves as an existential type that is used by a corresponding
// DoInvoke function to perform the function execution.  This allows
// us to shield the Callback class from the types of the bound argument via
// "type erasure."
// At the base level, the only task is to add reference counting data. Avoid
// using or inheriting any virtual functions. Creating a vtable for every
// BindState template instantiation results in a lot of bloat. Its only task is
// to call the destructor which can be done with a function pointer.
class BASE_EXPORT BindStateBase
    : public RefCountedThreadSafe<BindStateBase, BindStateBaseRefCountTraits> {
 public:
  REQUIRE_ADOPTION_FOR_REFCOUNTED_TYPE();

  // What kind of cancellation query the call to the cancellation traits is
  // making. This enum could be removed, at the cost of storing an extra
  // function pointer.
  enum class CancellationQueryMode : bool {
    kIsCancelled = false,
    kMaybeValid = true,
  };

  using InvokeFuncStorage = void (*)();

  BindStateBase(const BindStateBase&) = delete;
  BindStateBase& operator=(const BindStateBase&) = delete;

 private:
  using DestructorPtr = void (*)(const BindStateBase*);
  using QueryCancellationTraitsPtr = bool (*)(const BindStateBase*,
                                              CancellationQueryMode mode);

  BindStateBase(InvokeFuncStorage polymorphic_invoke, DestructorPtr destructor);
  BindStateBase(InvokeFuncStorage polymorphic_invoke,
                DestructorPtr destructor,
                QueryCancellationTraitsPtr query_cancellation_traits);
  ~BindStateBase() = default;

  friend struct BindStateBaseRefCountTraits;
  friend class RefCountedThreadSafe<BindStateBase, BindStateBaseRefCountTraits>;

  friend class BindStateHolder;

  // Allowlist subclasses that access the destructor of BindStateBase.
  template <bool is_method,
            bool is_nullable,
            bool is_callback,
            typename Functor,
            typename... BoundArgs>
  friend struct BindState;
  friend struct ::base::FakeBindState;

  bool IsCancelled() const {
    return query_cancellation_traits_(this,
                                      CancellationQueryMode::kIsCancelled);
  }

  bool MaybeValid() const {
    return query_cancellation_traits_(this, CancellationQueryMode::kMaybeValid);
  }

  // In C++, it is safe to cast function pointers to function pointers of
  // another type. It is not okay to use void*. We create a InvokeFuncStorage
  // that that can store our function pointer, and then cast it back to
  // the original type on usage.
  InvokeFuncStorage polymorphic_invoke_;

  // Pointer to a function that will properly destroy |this|.
  DestructorPtr destructor_;
  QueryCancellationTraitsPtr query_cancellation_traits_;
};

// Minimal wrapper around a `scoped_refptr<BindStateBase>`. It allows more
// expensive operations (such as ones that destroy `BindStateBase` or manipulate
// refcounts) to be defined out-of-line to reduce binary size.
class BASE_EXPORT TRIVIAL_ABI BindStateHolder {
 public:
  using InvokeFuncStorage = BindStateBase::InvokeFuncStorage;

  // Used to construct a null callback.
  inline constexpr BindStateHolder() noexcept;

  // Used to construct a callback by `base::BindOnce()`/`base::BindRepeating().
  inline explicit BindStateHolder(BindStateBase* bind_state);

  // BindStateHolder is always copyable so it can be used by `OnceCallback` and
  // `RepeatingCallback`. `OnceCallback` restricts copies so a `BindStateHolder`
  // used with a `OnceCallback will never be copied.
  BindStateHolder(const BindStateHolder&);
  BindStateHolder& operator=(const BindStateHolder&);

  // Subtle: since `this` is marked as TRIVIAL_ABI, the move operations must
  // leave a moved-from `BindStateHolder` in a trivially destructible state.
  inline BindStateHolder(BindStateHolder&&) noexcept;
  BindStateHolder& operator=(BindStateHolder&&) noexcept;

  ~BindStateHolder();

  bool is_null() const { return !bind_state_; }
  explicit operator bool() const { return !is_null(); }

  bool IsCancelled() const;
  bool MaybeValid() const;

  void Reset();

  friend bool operator==(const BindStateHolder&,
                         const BindStateHolder&) = default;

  const scoped_refptr<BindStateBase>& bind_state() const { return bind_state_; }

  InvokeFuncStorage polymorphic_invoke() const {
    return bind_state_->polymorphic_invoke_;
  }

 private:
  scoped_refptr<BindStateBase> bind_state_;
};

constexpr BindStateHolder::BindStateHolder() noexcept = default;

// TODO(dcheng): Try plumbing a scoped_refptr all the way through, since
// scoped_refptr is marked as TRIVIAL_ABI.
BindStateHolder::BindStateHolder(BindStateBase* bind_state)
    : bind_state_(AdoptRef(bind_state)) {}

// Unlike the copy constructor, copy assignment operator, and move assignment
// operator, the move constructor is defaulted in the header because it
// generates minimal code: move construction does not change any refcounts, nor
// does it potentially destroy `BindStateBase`.
BindStateHolder::BindStateHolder(BindStateHolder&&) noexcept = default;

// Helpers for the `Then()` implementation.
template <typename OriginalCallback, typename ThenCallback>
struct ThenHelper;

// Specialization when original callback returns `void`.
template <template <typename> class OriginalCallback,
          template <typename>
          class ThenCallback,
          typename... OriginalArgs,
          typename ThenR,
          typename... ThenArgs>
struct ThenHelper<OriginalCallback<void(OriginalArgs...)>,
                  ThenCallback<ThenR(ThenArgs...)>> {
 private:
  // For context on this "templated struct with a lambda that asserts" pattern,
  // see comments in `Invoker<>`.
  template <bool v = sizeof...(ThenArgs) == 0>
  struct CorrectNumberOfArgs {
    static constexpr bool value = [] {
      static_assert(v,
                    "|then| callback cannot accept parameters if |this| has a "
                    "void return type.");
      return v;
    }();
  };

 public:
  static auto CreateTrampoline() {
    return [](OriginalCallback<void(OriginalArgs...)> c1,
              ThenCallback<ThenR(ThenArgs...)> c2,
              OriginalArgs... c1_args) -> ThenR {
      if constexpr (CorrectNumberOfArgs<>::value) {
        std::move(c1).Run(std::forward<OriginalArgs>(c1_args)...);
        return std::move(c2).Run();
      }
    };
  }
};

// Specialization when original callback returns a non-void type.
template <template <typename> class OriginalCallback,
          template <typename>
          class ThenCallback,
          typename OriginalR,
          typename... OriginalArgs,
          typename ThenR,
          typename... ThenArgs>
struct ThenHelper<OriginalCallback<OriginalR(OriginalArgs...)>,
                  ThenCallback<ThenR(ThenArgs...)>> {
 private:
  template <bool v = sizeof...(ThenArgs) == 1>
  struct CorrectNumberOfArgs {
    static constexpr bool value = [] {
      static_assert(
          v,
          "|then| callback must accept exactly one parameter if |this| has a "
          "non-void return type.");
      return v;
    }();
  };

  template <bool v =
                // TODO(dcheng): This should probably check is_convertible as
                // well (same with `AssertBindArgsValidity`).
            std::is_constructible_v<ThenArgs..., OriginalR&&>>
  struct ArgsAreConvertible {
    static constexpr bool value = [] {
      static_assert(v,
                    "|then| callback's parameter must be constructible from "
                    "return type of |this|.");
      return v;
    }();
  };

 public:
  static auto CreateTrampoline() {
    return [](OriginalCallback<OriginalR(OriginalArgs...)> c1,
              ThenCallback<ThenR(ThenArgs...)> c2,
              OriginalArgs... c1_args) -> ThenR {
      if constexpr (std::conjunction_v<CorrectNumberOfArgs<>,
                                       ArgsAreConvertible<>>) {
        return std::move(c2).Run(
            std::move(c1).Run(std::forward<OriginalArgs>(c1_args)...));
      }
    };
  }
};

}  // namespace internal
}  // namespace base

#endif  // BASE_FUNCTIONAL_CALLBACK_INTERNAL_H_
