//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef USES_ALLOC_TYPES_H
#define USES_ALLOC_TYPES_H

#include <cassert>
#include <cstdlib>
#include <memory>

#include "test_macros.h"
#include "test_workarounds.h"
#include "type_id.h"

// There are two forms of uses-allocator construction:
//   (1) UA_AllocArg: 'T(allocator_arg_t, Alloc const&, Args&&...)'
//   (2) UA_AllocLast: 'T(Args&&..., Alloc const&)'
// 'UA_None' represents non-uses allocator construction.
enum class UsesAllocatorType { UA_None = 0, UA_AllocArg = 2, UA_AllocLast = 4 };
constexpr UsesAllocatorType UA_None      = UsesAllocatorType::UA_None;
constexpr UsesAllocatorType UA_AllocArg  = UsesAllocatorType::UA_AllocArg;
constexpr UsesAllocatorType UA_AllocLast = UsesAllocatorType::UA_AllocLast;

inline const char* toString(UsesAllocatorType UA) {
  switch (UA) {
  case UA_None:
    return "UA_None";
  case UA_AllocArg:
    return "UA_AllocArg";
  case UA_AllocLast:
    return "UA_AllocLast";
  default:
    std::abort();
  }
}

template <class Alloc, std::size_t N>
class UsesAllocatorV1;
// Implements form (1) of uses-allocator construction from the specified
// 'Alloc' type and exactly 'N' additional arguments. It also provides
// non-uses allocator construction from 'N' arguments. This test type
// blows up when form (2) of uses-allocator is even considered.

template <class Alloc, std::size_t N>
class UsesAllocatorV2;
// Implements form (2) of uses-allocator construction from the specified
// 'Alloc' type and exactly 'N' additional arguments. It also provides
// non-uses allocator construction from 'N' arguments.

template <class Alloc, std::size_t N>
class UsesAllocatorV3;
// Implements both form (1) and (2) of uses-allocator construction from
// the specified 'Alloc' type and exactly 'N' additional arguments. It also
// provides non-uses allocator construction from 'N' arguments.

template <class Alloc, std::size_t>
class NotUsesAllocator;
// Implements both form (1) and (2) of uses-allocator construction from
// the specified 'Alloc' type and exactly 'N' additional arguments. It also
// provides non-uses allocator construction from 'N' arguments. However
// 'NotUsesAllocator' never provides a 'allocator_type' typedef so it is
// never automatically uses-allocator constructed.

template <class... ArgTypes, class TestType>
bool checkConstruct(TestType& value, UsesAllocatorType form, typename TestType::CtorAlloc const& alloc)
// Check that 'value' was constructed using the specified 'form' of
// construction and with the specified 'ArgTypes...'. Additionally
// check that 'value' was constructed using the specified 'alloc'.
{
  if (form == UA_None) {
    return value.template checkConstruct<ArgTypes&&...>(form);
  } else {
    return value.template checkConstruct<ArgTypes&&...>(form, alloc);
  }
}

template <class... ArgTypes, class TestType>
bool checkConstruct(TestType& value, UsesAllocatorType form) {
  return value.template checkConstruct<ArgTypes&&...>(form);
}

template <class TestType>
bool checkConstructionEquiv(TestType& T, TestType& U)
// check that 'T' and 'U' where initialized in the exact same manner.
{
  return T.checkConstructEquiv(U);
}

////////////////////////////////////////////////////////////////////////////////
namespace detail {

template <bool IsZero, std::size_t N, class ArgList, class... Args>
struct TakeNImp;

template <class ArgList, class... Args>
struct TakeNImp<true, 0, ArgList, Args...> {
  typedef ArgList type;
};

template <std::size_t N, class... A1, class F, class... R>
struct TakeNImp<false, N, ArgumentListID<A1...>, F, R...>
    : TakeNImp<N - 1 == 0, N - 1, ArgumentListID<A1..., F>, R...> {};

template <std::size_t N, class... Args>
struct TakeNArgs : TakeNImp<N == 0, N, ArgumentListID<>, Args...> {};

template <class T>
struct Identity {
  typedef T type;
};

template <class T>
using IdentityT = typename Identity<T>::type;

template <bool Value>
using EnableIfB = typename std::enable_if<Value, bool>::type;

} // namespace detail

using detail::EnableIfB;

struct AllocLastTag {};

template <class Alloc, bool = std::is_default_constructible<Alloc>::value>
struct UsesAllocatorTestBaseStorage {
  Alloc allocator;
  UsesAllocatorTestBaseStorage() = default;
  UsesAllocatorTestBaseStorage(Alloc const& a) : allocator(a) {}
  const Alloc* get_allocator() const { return &allocator; }
};

template <class Alloc>
struct UsesAllocatorTestBaseStorage<Alloc, false> {
  union {
    char dummy;
    Alloc alloc;
  };
  bool has_alloc = false;

  UsesAllocatorTestBaseStorage() : dummy(), has_alloc(false) {}
  UsesAllocatorTestBaseStorage(Alloc const& a) : alloc(a), has_alloc(true) {}
  ~UsesAllocatorTestBaseStorage() {
    if (has_alloc)
      alloc.~Alloc();
  }

  Alloc const* get_allocator() const {
    if (!has_alloc)
      return nullptr;
    return &alloc;
  }
};

template <class Self, class Alloc>
struct UsesAllocatorTestBase {
public:
  using CtorAlloc = Alloc;

  template <class... ArgTypes>
  bool checkConstruct(UsesAllocatorType expectType) const {
    auto expectArgs = &makeArgumentID<ArgTypes...>();
    if (expectType != constructor_called)
      return false;
    if (args_id != expectArgs)
      return false;
    return true;
  }

  template <class... ArgTypes>
  bool checkConstruct(UsesAllocatorType expectType, CtorAlloc const& expectAlloc) const {
    auto ExpectID = &makeArgumentID<ArgTypes...>();
    if (expectType != constructor_called)
      return false;
    if (args_id != ExpectID)
      return false;
    if (!has_alloc() || expectAlloc != *get_alloc())
      return false;
    return true;
  }

  bool checkConstructEquiv(UsesAllocatorTestBase& O) const {
    if (has_alloc() != O.has_alloc())
      return false;
    if (constructor_called != O.constructor_called)
      return false;
    if (args_id != O.args_id)
      return false;
    if (has_alloc() && *get_alloc() != *O.get_alloc())
      return false;
    return true;
  }

protected:
  explicit UsesAllocatorTestBase(const TypeID* aid) : args_id(aid), constructor_called(UA_None), alloc_store() {}

  UsesAllocatorTestBase(UsesAllocatorTestBase const&)
      : args_id(&makeArgumentID<Self const&>()), constructor_called(UA_None), alloc_store() {}

  UsesAllocatorTestBase(UsesAllocatorTestBase&&)
      : args_id(&makeArgumentID<Self&&>()), constructor_called(UA_None), alloc_store() {}

  template <class... Args>
  UsesAllocatorTestBase(std::allocator_arg_t, CtorAlloc const& a, Args&&...)
      : args_id(&makeArgumentID<Args&&...>()), constructor_called(UA_AllocArg), alloc_store(a) {}

  template <class... Args, class ArgsIDL = detail::TakeNArgs<sizeof...(Args) - 1, Args&&...>>
  UsesAllocatorTestBase(AllocLastTag, Args&&... args)
      : args_id(&makeTypeIDImp<typename ArgsIDL::type>()),
        constructor_called(UA_AllocLast),
        alloc_store(
            UsesAllocatorTestBase::getAllocatorFromPack(typename ArgsIDL::type{}, std::forward<Args>(args)...)) {}

private:
  template <class... LArgs, class... Args>
  static CtorAlloc getAllocatorFromPack(ArgumentListID<LArgs...>, Args&&... args) {
    return UsesAllocatorTestBase::getAllocatorFromPackImp<LArgs const&...>(args...);
  }

  template <class... LArgs>
  static CtorAlloc getAllocatorFromPackImp(typename detail::Identity<LArgs>::type..., CtorAlloc const& alloc) {
    return alloc;
  }

  bool has_alloc() const { return alloc_store.get_allocator() != nullptr; }
  const CtorAlloc* get_alloc() const { return alloc_store.get_allocator(); }

public:
  const TypeID* args_id;
  UsesAllocatorType constructor_called = UA_None;
  UsesAllocatorTestBaseStorage<CtorAlloc> alloc_store;
};

template <class Alloc, std::size_t Arity>
class UsesAllocatorV1 : public UsesAllocatorTestBase<UsesAllocatorV1<Alloc, Arity>, Alloc> {
public:
  typedef Alloc allocator_type;

  using Base      = UsesAllocatorTestBase<UsesAllocatorV1, Alloc>;
  using CtorAlloc = typename Base::CtorAlloc;

  UsesAllocatorV1() : Base(&makeArgumentID<>()) {}

  UsesAllocatorV1(UsesAllocatorV1 const&) : Base(&makeArgumentID<UsesAllocatorV1 const&>()) {}
  UsesAllocatorV1(UsesAllocatorV1&&) : Base(&makeArgumentID<UsesAllocatorV1&&>()) {}
  // Non-Uses Allocator Ctor
  template <class... Args, EnableIfB<sizeof...(Args) == Arity> = false>
  UsesAllocatorV1(Args&&...) : Base(&makeArgumentID<Args&&...>()) {}

  // Uses Allocator Arg Ctor
  template <class... Args>
  UsesAllocatorV1(std::allocator_arg_t tag, CtorAlloc const& a, Args&&... args)
      : Base(tag, a, std::forward<Args>(args)...) {}

  // BLOWS UP: Uses Allocator Last Ctor
  template <class First, class... Args, EnableIfB<sizeof...(Args) == Arity> Dummy = false>
  constexpr UsesAllocatorV1(First&&, Args&&...) {
    static_assert(!std::is_same<First, First>::value, "");
  }
};

template <class Alloc, std::size_t Arity>
class UsesAllocatorV2 : public UsesAllocatorTestBase<UsesAllocatorV2<Alloc, Arity>, Alloc> {
public:
  typedef Alloc allocator_type;

  using Base      = UsesAllocatorTestBase<UsesAllocatorV2, Alloc>;
  using CtorAlloc = typename Base::CtorAlloc;

  UsesAllocatorV2() : Base(&makeArgumentID<>()) {}
  UsesAllocatorV2(UsesAllocatorV2 const&) : Base(&makeArgumentID<UsesAllocatorV2 const&>()) {}
  UsesAllocatorV2(UsesAllocatorV2&&) : Base(&makeArgumentID<UsesAllocatorV2&&>()) {}

  // Non-Uses Allocator Ctor
  template <class... Args, EnableIfB<sizeof...(Args) == Arity> = false>
  UsesAllocatorV2(Args&&...) : Base(&makeArgumentID<Args&&...>()) {}

  // Uses Allocator Last Ctor
  template <class... Args, EnableIfB<sizeof...(Args) == Arity + 1> = false>
  UsesAllocatorV2(Args&&... args) : Base(AllocLastTag{}, std::forward<Args>(args)...) {}
};

template <class Alloc, std::size_t Arity>
class UsesAllocatorV3 : public UsesAllocatorTestBase<UsesAllocatorV3<Alloc, Arity>, Alloc> {
public:
  typedef Alloc allocator_type;

  using Base      = UsesAllocatorTestBase<UsesAllocatorV3, Alloc>;
  using CtorAlloc = typename Base::CtorAlloc;

  UsesAllocatorV3() : Base(&makeArgumentID<>()) {}
  UsesAllocatorV3(UsesAllocatorV3 const&) : Base(&makeArgumentID<UsesAllocatorV3 const&>()) {}
  UsesAllocatorV3(UsesAllocatorV3&&) : Base(&makeArgumentID<UsesAllocatorV3&&>()) {}

  // Non-Uses Allocator Ctor
  template <class... Args, EnableIfB<sizeof...(Args) == Arity> = false>
  UsesAllocatorV3(Args&&...) : Base(&makeArgumentID<Args&&...>()) {}

  // Uses Allocator Arg Ctor
  template <class... Args>
  UsesAllocatorV3(std::allocator_arg_t tag, CtorAlloc const& alloc, Args&&... args)
      : Base(tag, alloc, std::forward<Args>(args)...) {}

  // Uses Allocator Last Ctor
  template <class... Args, EnableIfB<sizeof...(Args) == Arity + 1> = false>
  UsesAllocatorV3(Args&&... args) : Base(AllocLastTag{}, std::forward<Args>(args)...) {}
};

template <class Alloc, std::size_t Arity>
class NotUsesAllocator : public UsesAllocatorTestBase<NotUsesAllocator<Alloc, Arity>, Alloc> {
public:
  // no allocator_type typedef provided

  using Base      = UsesAllocatorTestBase<NotUsesAllocator, Alloc>;
  using CtorAlloc = typename Base::CtorAlloc;

  NotUsesAllocator() : Base(&makeArgumentID<>()) {}
  NotUsesAllocator(NotUsesAllocator const&) : Base(&makeArgumentID<NotUsesAllocator const&>()) {}
  NotUsesAllocator(NotUsesAllocator&&) : Base(&makeArgumentID<NotUsesAllocator&&>()) {}
  // Non-Uses Allocator Ctor
  template <class... Args, EnableIfB<sizeof...(Args) == Arity> = false>
  NotUsesAllocator(Args&&...) : Base(&makeArgumentID<Args&&...>()) {}

  // Uses Allocator Arg Ctor
  template <class... Args>
  NotUsesAllocator(std::allocator_arg_t tag, CtorAlloc const& alloc, Args&&... args)
      : Base(tag, alloc, std::forward<Args>(args)...) {}

  // Uses Allocator Last Ctor
  template <class... Args, EnableIfB<sizeof...(Args) == Arity + 1> = false>
  NotUsesAllocator(Args&&... args) : Base(AllocLastTag{}, std::forward<Args>(args)...) {}
};

#endif /* USES_ALLOC_TYPES_H */
