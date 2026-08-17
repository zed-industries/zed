// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_VARIANT_TEST_HELPERS_H
#define SUPPORT_VARIANT_TEST_HELPERS_H

#include <type_traits>
#include <utility>
#include <cassert>

#include "test_macros.h"
#include "type_id.h"

#if TEST_STD_VER <= 14
#error This file requires C++17
#endif

#ifndef TEST_HAS_NO_EXCEPTIONS
struct CopyThrows {
  CopyThrows() = default;
  CopyThrows(CopyThrows const&) { throw 42; }
  CopyThrows& operator=(CopyThrows const&) { throw 42; }
};

struct MoveThrows {
  static int alive;
  MoveThrows() { ++alive; }
  MoveThrows(MoveThrows const&) {++alive;}
  MoveThrows(MoveThrows&&) {  throw 42; }
  MoveThrows& operator=(MoveThrows const&) { return *this; }
  MoveThrows& operator=(MoveThrows&&) { throw 42; }
  ~MoveThrows() { --alive; }
};

int MoveThrows::alive = 0;

struct MakeEmptyT {
  static int alive;
  MakeEmptyT() { ++alive; }
  MakeEmptyT(MakeEmptyT const&) {
      ++alive;
      // Don't throw from the copy constructor since variant's assignment
      // operator performs a copy before committing to the assignment.
  }
  MakeEmptyT(MakeEmptyT &&) {
      throw 42;
  }
  MakeEmptyT& operator=(MakeEmptyT const&) {
      throw 42;
  }
  MakeEmptyT& operator=(MakeEmptyT&&) {
      throw 42;
  }
   ~MakeEmptyT() { --alive; }
};
static_assert(std::is_swappable_v<MakeEmptyT>, ""); // required for test

int MakeEmptyT::alive = 0;

template <class Variant>
void makeEmpty(Variant& v) {
    Variant v2(std::in_place_type<MakeEmptyT>);
    try {
        v = std::move(v2);
        assert(false);
    } catch (...) {
        assert(v.valueless_by_exception());
    }
}
#endif // TEST_HAS_NO_EXCEPTIONS

enum CallType : unsigned {
  CT_None,
  CT_NonConst = 1,
  CT_Const = 2,
  CT_LValue = 4,
  CT_RValue = 8
};

inline constexpr CallType operator|(CallType LHS, CallType RHS) {
  return static_cast<CallType>(static_cast<unsigned>(LHS) |
                               static_cast<unsigned>(RHS));
}

struct ForwardingCallObject {

  template <class... Args>
  ForwardingCallObject& operator()(Args&&...) & {
    set_call<Args &&...>(CT_NonConst | CT_LValue);
    return *this;
  }

  template <class... Args>
  const ForwardingCallObject& operator()(Args&&...) const & {
    set_call<Args &&...>(CT_Const | CT_LValue);
    return *this;
  }

  template <class... Args>
  ForwardingCallObject&& operator()(Args&&...) && {
    set_call<Args &&...>(CT_NonConst | CT_RValue);
    return std::move(*this);
  }

  template <class... Args>
  const ForwardingCallObject&& operator()(Args&&...) const && {
    set_call<Args &&...>(CT_Const | CT_RValue);
    return std::move(*this);
  }

  template <class... Args> static void set_call(CallType type) {
    assert(last_call_type == CT_None);
    assert(last_call_args == nullptr);
    last_call_type = type;
    last_call_args = std::addressof(makeArgumentID<Args...>());
  }

  template <class... Args> static bool check_call(CallType type) {
    bool result = last_call_type == type && last_call_args &&
                  *last_call_args == makeArgumentID<Args...>();
    last_call_type = CT_None;
    last_call_args = nullptr;
    return result;
  }

  // To check explicit return type for visit<R>
  constexpr operator int() const
  {
    return 0;
  }

  static CallType last_call_type;
  static const TypeID *last_call_args;
};

CallType ForwardingCallObject::last_call_type = CT_None;
const TypeID *ForwardingCallObject::last_call_args = nullptr;

struct ReturnFirst {
  template <class... Args> constexpr int operator()(int f, Args &&...) const {
    return f;
  }
};

struct ReturnArity {
  template <class... Args> constexpr int operator()(Args &&...) const {
    return sizeof...(Args);
  }
};

#endif // SUPPORT_VARIANT_TEST_HELPERS_H
