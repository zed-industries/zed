//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_ARCHETYPES_H
#define TEST_SUPPORT_ARCHETYPES_H

#include <type_traits>
#include <cassert>
#include <initializer_list>

#include "test_macros.h"
#include "test_workarounds.h"

#if TEST_STD_VER >= 11

namespace ArchetypeBases {

template <bool, class T>
struct DepType : T {};

struct NullBase {
#ifndef TEST_WORKAROUND_MSVC_BROKEN_ZA_CTOR_CHECK
protected:
#endif // !TEST_WORKAROUND_MSVC_BROKEN_ZA_CTOR_CHECK
  NullBase() = default;
  NullBase(NullBase const&) = default;
  NullBase& operator=(NullBase const&) = default;
  NullBase(NullBase &&) = default;
  NullBase& operator=(NullBase &&) = default;
};

template <class Derived, bool Explicit = false>
struct TestBase {
    static int alive;
    static int constructed;
    static int value_constructed;
    static int default_constructed;
    static int copy_constructed;
    static int move_constructed;
    static int assigned;
    static int value_assigned;
    static int copy_assigned;
    static int move_assigned;
    static int destroyed;

    static void reset() {
        assert(alive == 0);
        alive = 0;
        reset_constructors();
    }

    static void reset_constructors() {
      constructed = value_constructed = default_constructed =
        copy_constructed = move_constructed = 0;
      assigned = value_assigned = copy_assigned = move_assigned = destroyed = 0;
    }

    TestBase() noexcept : value(0) {
        ++alive; ++constructed; ++default_constructed;
    }
    template <bool Dummy = true, typename std::enable_if<Dummy && Explicit, bool>::type = true>
    explicit TestBase(int x) noexcept : value(x) {
        ++alive; ++constructed; ++value_constructed;
    }
    template <bool Dummy = true, typename std::enable_if<Dummy && !Explicit, bool>::type = true>
    TestBase(int x) noexcept : value(x) {
        ++alive; ++constructed; ++value_constructed;
    }
    template <bool Dummy = true, typename std::enable_if<Dummy && Explicit, bool>::type = true>
    explicit TestBase(int, int y) noexcept : value(y) {
        ++alive; ++constructed; ++value_constructed;
    }
    template <bool Dummy = true, typename std::enable_if<Dummy && !Explicit, bool>::type = true>
    TestBase(int, int y) noexcept : value(y) {
        ++alive; ++constructed; ++value_constructed;
    }
    template <bool Dummy = true, typename std::enable_if<Dummy && Explicit, bool>::type = true>
    explicit TestBase(std::initializer_list<int>& il, int = 0) noexcept
      : value(static_cast<int>(il.size())) {
        ++alive; ++constructed; ++value_constructed;
    }
    template <bool Dummy = true, typename std::enable_if<Dummy && !Explicit, bool>::type = true>
    explicit TestBase(std::initializer_list<int>& il, int = 0) noexcept : value(static_cast<int>(il.size())) {
        ++alive; ++constructed; ++value_constructed;
    }
    TestBase& operator=(int xvalue) noexcept {
      value = xvalue;
      ++assigned; ++value_assigned;
      return *this;
    }
#ifndef TEST_WORKAROUND_MSVC_BROKEN_ZA_CTOR_CHECK
protected:
#endif // !TEST_WORKAROUND_MSVC_BROKEN_ZA_CTOR_CHECK
    ~TestBase() {
      assert(value != -999); assert(alive > 0);
      --alive; ++destroyed; value = -999;
    }
    explicit TestBase(TestBase const& o) noexcept : value(o.value) {
        assert(o.value != -1); assert(o.value != -999);
        ++alive; ++constructed; ++copy_constructed;
    }
    explicit TestBase(TestBase && o) noexcept : value(o.value) {
        assert(o.value != -1); assert(o.value != -999);
        ++alive; ++constructed; ++move_constructed;
        o.value = -1;
    }
    TestBase& operator=(TestBase const& o) noexcept {
      assert(o.value != -1); assert(o.value != -999);
      ++assigned; ++copy_assigned;
      value = o.value;
      return *this;
    }
    TestBase& operator=(TestBase&& o) noexcept {
        assert(o.value != -1); assert(o.value != -999);
        ++assigned; ++move_assigned;
        value = o.value;
        o.value = -1;
        return *this;
    }
public:
    int value;
};

template <class D, bool E> int TestBase<D, E>::alive = 0;
template <class D, bool E> int TestBase<D, E>::constructed = 0;
template <class D, bool E> int TestBase<D, E>::value_constructed = 0;
template <class D, bool E> int TestBase<D, E>::default_constructed = 0;
template <class D, bool E> int TestBase<D, E>::copy_constructed = 0;
template <class D, bool E> int TestBase<D, E>::move_constructed = 0;
template <class D, bool E> int TestBase<D, E>::assigned = 0;
template <class D, bool E> int TestBase<D, E>::value_assigned = 0;
template <class D, bool E> int TestBase<D, E>::copy_assigned = 0;
template <class D, bool E> int TestBase<D, E>::move_assigned = 0;
template <class D, bool E> int TestBase<D, E>::destroyed = 0;

template <bool Explicit = false>
struct ValueBase {
    template <bool Dummy = true, typename std::enable_if<Dummy && Explicit, bool>::type = true>
    explicit constexpr ValueBase(int x) : value(x) {}
    template <bool Dummy = true, typename std::enable_if<Dummy && !Explicit, bool>::type = true>
    constexpr ValueBase(int x) : value(x) {}
    template <bool Dummy = true, typename std::enable_if<Dummy && Explicit, bool>::type = true>
    explicit constexpr ValueBase(int, int y) : value(y) {}
    template <bool Dummy = true, typename std::enable_if<Dummy && !Explicit, bool>::type = true>
    constexpr ValueBase(int, int y) : value(y) {}
    template <bool Dummy = true, typename std::enable_if<Dummy && Explicit, bool>::type = true>
    explicit constexpr ValueBase(std::initializer_list<int>& il, int = 0) : value(static_cast<int>(il.size())) {}
    template <bool Dummy = true, typename std::enable_if<Dummy && !Explicit, bool>::type = true>
    constexpr ValueBase(std::initializer_list<int>& il, int = 0) : value(static_cast<int>(il.size())) {}
    TEST_CONSTEXPR_CXX14 ValueBase& operator=(int xvalue) noexcept {
        value = xvalue;
        return *this;
    }
    //~ValueBase() { assert(value != -999); value = -999; }
    int value;
#ifndef TEST_WORKAROUND_MSVC_BROKEN_ZA_CTOR_CHECK
protected:
#endif // !TEST_WORKAROUND_MSVC_BROKEN_ZA_CTOR_CHECK
    constexpr static int check_value(int const& val) {
#if TEST_STD_VER < 14
      return val == -1 || val == 999 ? (TEST_THROW(42), 0) : val;
#else
      assert(val != -1); assert(val != 999);
      return val;
#endif
    }
    constexpr static int check_value(int& val, int val_cp = 0) {
#if TEST_STD_VER < 14
      return val_cp = val, val = -1, (val_cp == -1 || val_cp == 999 ? (TEST_THROW(42), 0) : val_cp);
#else
      assert(val != -1); assert(val != 999);
      val_cp = val;
      val = -1;
      return val_cp;
#endif
    }
    constexpr ValueBase() noexcept : value(0) {}
    constexpr ValueBase(ValueBase const& o) noexcept : value(check_value(o.value)) {
    }
    constexpr ValueBase(ValueBase && o) noexcept : value(check_value(o.value)) {
    }
    TEST_CONSTEXPR_CXX14 ValueBase& operator=(ValueBase const& o) noexcept {
        assert(o.value != -1); assert(o.value != -999);
        value = o.value;
        return *this;
    }
    TEST_CONSTEXPR_CXX14 ValueBase& operator=(ValueBase&& o) noexcept {
        assert(o.value != -1); assert(o.value != -999);
        value = o.value;
        o.value = -1;
        return *this;
    }
};


template <bool Explicit = false>
struct TrivialValueBase {
    template <bool Dummy = true, typename std::enable_if<Dummy && Explicit, bool>::type = true>
    explicit constexpr TrivialValueBase(int x) : value(x) {}
    template <bool Dummy = true, typename std::enable_if<Dummy && !Explicit, bool>::type = true>
    constexpr TrivialValueBase(int x) : value(x) {}
    template <bool Dummy = true, typename std::enable_if<Dummy && Explicit, bool>::type = true>
    explicit constexpr TrivialValueBase(int, int y) : value(y) {}
    template <bool Dummy = true, typename std::enable_if<Dummy && !Explicit, bool>::type = true>
    constexpr TrivialValueBase(int, int y) : value(y) {}
    template <bool Dummy = true, typename std::enable_if<Dummy && Explicit, bool>::type = true>
    explicit constexpr TrivialValueBase(std::initializer_list<int>& il, int = 0) : value(static_cast<int>(il.size())) {}
    template <bool Dummy = true, typename std::enable_if<Dummy && !Explicit, bool>::type = true>
    constexpr TrivialValueBase(std::initializer_list<int>& il, int = 0) : value(static_cast<int>(il.size())) {}
    int value;
#ifndef TEST_WORKAROUND_MSVC_BROKEN_ZA_CTOR_CHECK
protected:
#endif // !TEST_WORKAROUND_MSVC_BROKEN_ZA_CTOR_CHECK
    constexpr TrivialValueBase() noexcept : value(0) {}
};

}

//============================================================================//
// Trivial Implicit Test Types
namespace ImplicitTypes {
#include "archetypes.ipp"
}

//============================================================================//
// Trivial Explicit Test Types
namespace ExplicitTypes {
#define DEFINE_EXPLICIT explicit
#include "archetypes.ipp"
}

//============================================================================//
//
namespace NonConstexprTypes {
#define DEFINE_CONSTEXPR
#include "archetypes.ipp"
}

//============================================================================//
// Non-literal implicit test types
namespace NonLiteralTypes {
#define DEFINE_ASSIGN_CONSTEXPR
#define DEFINE_DTOR(Name) ~Name() {}
#include "archetypes.ipp"
}

//============================================================================//
// Non-throwing implicit test types
namespace NonThrowingTypes {
#define DEFINE_NOEXCEPT noexcept
#include "archetypes.ipp"
}

//============================================================================//
// Non-Trivially Copyable Implicit Test Types
namespace NonTrivialTypes {
#define DEFINE_CTOR {}
#define DEFINE_ASSIGN { return *this; }
#include "archetypes.ipp"
}

//============================================================================//
// Implicit counting types
namespace TestTypes {
#define DEFINE_CONSTEXPR
#define DEFINE_BASE(Name) ::ArchetypeBases::TestBase<Name>
#include "archetypes.ipp"

using TestType = AllCtors;

// Add equality operators
template <class Tp>
constexpr bool operator==(Tp const& L, Tp const& R) noexcept {
  return L.value == R.value;
}

template <class Tp>
constexpr bool operator!=(Tp const& L, Tp const& R) noexcept {
  return L.value != R.value;
}

}

//============================================================================//
// Implicit counting types
namespace ExplicitTestTypes {
#define DEFINE_CONSTEXPR
#define DEFINE_EXPLICIT explicit
#define DEFINE_BASE(Name) ::ArchetypeBases::TestBase<Name, true>
#include "archetypes.ipp"

using TestType = AllCtors;

// Add equality operators
template <class Tp>
constexpr bool operator==(Tp const& L, Tp const& R) noexcept {
  return L.value == R.value;
}

template <class Tp>
constexpr bool operator!=(Tp const& L, Tp const& R) noexcept {
  return L.value != R.value;
}

}

//============================================================================//
// Implicit value types
namespace ConstexprTestTypes {
#define DEFINE_BASE(Name) ::ArchetypeBases::ValueBase<>
#include "archetypes.ipp"

using TestType = AllCtors;

// Add equality operators
template <class Tp>
constexpr bool operator==(Tp const& L, Tp const& R) noexcept {
  return L.value == R.value;
}

template <class Tp>
constexpr bool operator!=(Tp const& L, Tp const& R) noexcept {
  return L.value != R.value;
}

} // namespace ConstexprTestTypes


//============================================================================//
//
namespace ExplicitConstexprTestTypes {
#define DEFINE_EXPLICIT explicit
#define DEFINE_BASE(Name) ::ArchetypeBases::ValueBase<true>
#include "archetypes.ipp"

using TestType = AllCtors;

// Add equality operators
template <class Tp>
constexpr bool operator==(Tp const& L, Tp const& R) noexcept {
  return L.value == R.value;
}

template <class Tp>
constexpr bool operator!=(Tp const& L, Tp const& R) noexcept {
  return L.value != R.value;
}

} // namespace ExplicitConstexprTestTypes


//============================================================================//
//
namespace TrivialTestTypes {
#define DEFINE_BASE(Name) ::ArchetypeBases::TrivialValueBase<false>
#include "archetypes.ipp"

using TestType = AllCtors;

// Add equality operators
template <class Tp>
constexpr bool operator==(Tp const& L, Tp const& R) noexcept {
  return L.value == R.value;
}

template <class Tp>
constexpr bool operator!=(Tp const& L, Tp const& R) noexcept {
  return L.value != R.value;
}

} // namespace TrivialTestTypes

//============================================================================//
//
namespace ExplicitTrivialTestTypes {
#define DEFINE_EXPLICIT explicit
#define DEFINE_BASE(Name) ::ArchetypeBases::TrivialValueBase<true>
#include "archetypes.ipp"

using TestType = AllCtors;

// Add equality operators
template <class Tp>
constexpr bool operator==(Tp const& L, Tp const& R) noexcept {
  return L.value == R.value;
}

template <class Tp>
constexpr bool operator!=(Tp const& L, Tp const& R) noexcept {
  return L.value != R.value;
}

} // namespace ExplicitTrivialTestTypes

#endif // TEST_STD_VER >= 11

#endif // TEST_SUPPORT_ARCHETYPES_H
