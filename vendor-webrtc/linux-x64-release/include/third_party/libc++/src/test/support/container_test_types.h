//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_CONTAINER_TEST_TYPES_H
#define SUPPORT_CONTAINER_TEST_TYPES_H

// container_test_types.h - A set of types used for testing STL containers.
// The types container within this header are used to test the requirements in
// [container.requirements.general]. The header is made up of 3 main components:
//
// * test-types: 'CopyInsertable', 'MoveInsertable' and 'EmplaceConstructible' -
//    These test types are used to test the container requirements of the same
//    name. These test types use the global 'AllocatorConstructController' to
//    assert that they are only constructed by the containers allocator.
//
// * test-allocator: 'ContainerTestAllocator' - This test allocator is used to
//    test the portions of [container.requirements.general] that pertain to the
//    containers allocator. The three primary jobs of the test allocator are:
//      1. Enforce that 'a.construct(...)' and 'a.destroy(...)' are only ever
//         instantiated for 'Container::value_type'.
//      2. Provide a mechanism of checking calls to 'a.construct(Args...)'.
//         Including controlling when and with what types 'a.construct(...)'
//         may be called with.
//      3. Support the test types internals by controlling the global
//        'AllocatorConstructController' object.
//
// * 'AllocatorConstructController' - This type defines an interface for testing
//   the construction of types using an allocator. This type is used to communicate
//   between the test author, the containers allocator, and the types
//   being constructed by the container.
//   The controller's primary functions are:
//     1. Allow calls to 'a.construct(p, args...)' to be checked by a test.
//        The test uses 'cc->expect<Args...>()' to specify that the allocator
//        should expect one call to 'a.construct' with the specified argument
//        types.
//     2. Controlling the value of 'cc->isInAllocatorConstruct()' within the
//        'construct' method. The test-types use this value to assert that
//         they are being constructed by the allocator.
//
//   'AllocatorConstructController' enforces the Singleton pattern since the
//    test-types, test-allocator and test need to share the same controller
//    object. A pointer to the global controller is returned by
//   'getConstructController()'.
//
//----------------------------------------------------------------------------
/*
 * Usage: The following example checks that 'unordered_map::emplace(Args&&...)'
 *        with 'Args = [CopyInsertable<1> const&, CopyInsertable<2>&&]'
 *        calls 'alloc.construct(value_type*, Args&&...)' with the same types.
 *
 * // Typedefs for container
 * using Key = CopyInsertable<1>;
 * using Value = CopyInsertable<2>;
 * using ValueTp = std::pair<const Key, Value>;
 * using Alloc = ContainerTestAllocator<ValueTp, ValueTp>;
 * using Map = std::unordered_map<Key, Value, std::hash<Key>, std::equal_to<Key>, Alloc>;
 *
 * // Get the global controller, reset it, and construct an allocator with
 * // the controller.
 * ConstructController* cc = getConstructController();
 * cc->reset();
 *
 * // Create a Map and a Key and Value to insert. Note that the test-allocator
 * // does not need to be given 'cc'.
 * Map m;
 * const Key k(1);
 * Value v(1);
 *
 * // Tell the controller to expect a construction from the specified types.
 * cc->expect<Key const&, Value&&>();
 *
 * // Emplace the objects into the container. 'Alloc.construct(p, UArgs...)'
 * // will assert 'cc->check<UArgs&&>()' is true which will consume
 * // the call to 'cc->expect<...>()'.
 * m.emplace(k, std::move(v));
 *
 * // Assert that the "expect" was consumed by a matching "check" call within
 * // Alloc.
 * assert(!cc->unexpected());
 *
 */

#include <cassert>
#include <cstddef>
#include <deque>
#include <functional>
#include <list>
#include <map>
#include <new>
#include <set>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

#include "test_macros.h"

#if TEST_STD_VER < 11
#error This header requires C++11 or greater
#endif

namespace detail {
// TypeID - Represent a unique identifier for a type. TypeID allows equality
// comparisons between different types.
struct TypeID {
  friend bool operator==(TypeID const& LHS, TypeID const& RHS)
  {return LHS.m_id == RHS.m_id; }
  friend bool operator!=(TypeID const& LHS, TypeID const& RHS)
  {return LHS.m_id != RHS.m_id; }
private:
  explicit constexpr TypeID(const int* xid) : m_id(xid) {}
  const int* const m_id;
  template <class T> friend class TypeInfo;
};

// TypeInfo - Represent information for the specified type 'T', including a
// unique TypeID.
template <class T>
class TypeInfo {
public:
  typedef T value_type;
  typedef TypeID ID;
  static  ID const& GetID() { static ID id(&dummy_addr); return id; }

private:
  static const int dummy_addr;
};

template <class L, class R>
inline bool operator==(TypeInfo<L> const&, TypeInfo<R> const&)
{ return std::is_same<L, R>::value; }

template <class L, class R>
inline bool operator!=(TypeInfo<L> const& lhs, TypeInfo<R> const& rhs)
{ return !(lhs == rhs); }

template <class T>
const int TypeInfo<T>::dummy_addr = 42;

// makeTypeID - Return the TypeID for the specified type 'T'.
template <class T>
inline constexpr TypeID const& makeTypeID() { return TypeInfo<T>::GetID(); }

template <class ...Args>
struct ArgumentListID {};

// makeArgumentID - Create and return a unique identifier for a given set
// of arguments.
template <class ...Args>
inline constexpr TypeID const& makeArgumentID() {
  return makeTypeID<ArgumentListID<Args...>>();
}

} // namespace detail

//===----------------------------------------------------------------------===//
//                        AllocatorConstructController
//===----------------------------------------------------------------------===//

struct AllocatorConstructController {
  const detail::TypeID* m_expected_args;
  bool m_allow_constructions;
  bool m_allow_unchecked;
  int m_expected_count;

  void clear() {
    m_expected_args = nullptr;
    m_expected_count = -1;
  }

  // Check for and consume an expected construction added by 'expect'.
  // Return true if the construction was expected and false otherwise.
  // This should only be called by 'Allocator.construct'.
  bool check(detail::TypeID const& tid) {
    if (!m_expected_args) {
      assert(m_allow_unchecked);
      return m_allow_unchecked;
    }
    bool res = *m_expected_args == tid;
    if (m_expected_count == -1 || --m_expected_count == -1)
      m_expected_args = nullptr;
    return res;
  }

  // Return true iff there is an unchecked construction expression.
  bool unchecked() {
    return m_expected_args != nullptr;
  }

  // Expect a call to Allocator::construct with Args that match 'tid'.
  void expect(detail::TypeID const& tid) {
    assert(!unchecked());
    m_expected_args = &tid;
  }

  template <class ...Args>
  void expect(int times = 1) {
    assert(!unchecked());
    assert(times > 0);
    m_expected_count = times - 1;
    m_expected_args = &detail::makeArgumentID<Args...>();
  }
  template <class ...Args>
  bool check() {
    return check(detail::makeArgumentID<Args...>());
  }


  // Return true iff the program is currently within a call to "Allocator::construct"
  bool isInAllocatorConstruct() const {
    return m_allow_constructions;
  }

  void inAllocatorConstruct(bool value = true) {
    m_allow_constructions = value;
  }

  void allowUnchecked(bool value = true) {
    m_allow_unchecked = value;
  }

  void reset() {
    m_allow_constructions = false;
    m_expected_args = nullptr;
    m_allow_unchecked = false;
    m_expected_count = -1;
  }

private:
  friend AllocatorConstructController* getConstructController();
  AllocatorConstructController()  { reset(); }
  AllocatorConstructController(AllocatorConstructController const&);
  AllocatorConstructController& operator=(AllocatorConstructController const&);
};

typedef AllocatorConstructController ConstructController;

// getConstructController - Return the global allocator construction controller.
inline ConstructController* getConstructController() {
  static ConstructController c;
  return &c;
}

template <class ...Args>
struct ExpectConstructGuard {
  ExpectConstructGuard(int N)  {
    auto CC = getConstructController();
    assert(!CC->unchecked());
    CC->expect<Args...>(N);
  }

  ~ExpectConstructGuard() {
    assert(!getConstructController()->unchecked());
  }
};

//===----------------------------------------------------------------------===//
//                       ContainerTestAllocator
//===----------------------------------------------------------------------===//

// ContainerTestAllocator - A STL allocator type that only allows 'construct'
// and 'destroy' to be called for 'AllowConstructT' types. ContainerTestAllocator
// uses the 'AllocatorConstructionController' interface.
template <class T, class AllowConstructT>
class ContainerTestAllocator
{
  struct InAllocatorConstructGuard {
    ConstructController *m_cc;
    bool m_old;
    InAllocatorConstructGuard(ConstructController* cc) : m_cc(cc) {
      if (m_cc) {
        m_old = m_cc->isInAllocatorConstruct();
        m_cc->inAllocatorConstruct(true);
      }
    }
    ~InAllocatorConstructGuard() {
      if (m_cc) m_cc->inAllocatorConstruct(m_old);
    }
  private:
    InAllocatorConstructGuard(InAllocatorConstructGuard const&);
    InAllocatorConstructGuard& operator=(InAllocatorConstructGuard const&);
  };

public:
    typedef T value_type;

    int construct_called;
    int destroy_called;
    ConstructController* controller;

    ContainerTestAllocator() TEST_NOEXCEPT
        : controller(getConstructController()) {}

    explicit ContainerTestAllocator(ConstructController* c)
       : controller(c)
    {}

    template <class U>
    ContainerTestAllocator(ContainerTestAllocator<U, AllowConstructT> other) TEST_NOEXCEPT
      : controller(other.controller)
    {}

    T* allocate(std::size_t n)
    {
        return static_cast<T*>(::operator new(n*sizeof(T)));
    }

    void deallocate(T* p, std::size_t)
    {
        return ::operator delete(static_cast<void*>(p));
    }

    template <class Up, class ...Args>
    void construct(Up* p, Args&&... args) {
      static_assert((std::is_same<Up, AllowConstructT>::value),
                    "Only allowed to construct Up");
      assert(controller->check<Args&&...>());
      {
        InAllocatorConstructGuard g(controller);
        ::new ((void*)p) Up(std::forward<Args>(args)...);
      }
    }

    template <class Up>
    void destroy(Up* p) {
      static_assert((std::is_same<Up, AllowConstructT>::value),
                    "Only allowed to destroy Up");
      {
        InAllocatorConstructGuard g(controller);
        p->~Up();
      }
    }

    friend bool operator==(ContainerTestAllocator, ContainerTestAllocator) {return true;}
    friend bool operator!=(ContainerTestAllocator x, ContainerTestAllocator y) {return !(x == y);}
};


namespace test_detail {
typedef ContainerTestAllocator<int, int> A1;
typedef std::allocator_traits<A1> A1T;
typedef ContainerTestAllocator<float, int> A2;
typedef std::allocator_traits<A2> A2T;

static_assert(std::is_same<A1T::rebind_traits<float>, A2T>::value, "");
static_assert(std::is_same<A2T::rebind_traits<int>, A1T>::value, "");
} // namespace test_detail

//===----------------------------------------------------------------------===//
//  'CopyInsertable', 'MoveInsertable' and 'EmplaceConstructible' test types
//===----------------------------------------------------------------------===//

template <int Dummy = 0>
struct CopyInsertable {
  int data;
  mutable bool copied_once;
  bool constructed_under_allocator;

  explicit CopyInsertable(int val) : data(val), copied_once(false),
                                     constructed_under_allocator(false) {
    if (getConstructController()->isInAllocatorConstruct()) {
      copied_once = true;
      constructed_under_allocator = true;
    }
  }

  CopyInsertable() : data(0), copied_once(false), constructed_under_allocator(true)
  {
    assert(getConstructController()->isInAllocatorConstruct());
  }

  CopyInsertable(CopyInsertable const& other) : data(other.data),
                                                copied_once(true),
                                                constructed_under_allocator(true) {
    assert(getConstructController()->isInAllocatorConstruct());
    assert(other.copied_once == false);
    other.copied_once = true;
  }

  CopyInsertable(CopyInsertable& other) : data(other.data), copied_once(true),
                                          constructed_under_allocator(true) {
    assert(getConstructController()->isInAllocatorConstruct());
    assert(other.copied_once == false);
    other.copied_once = true;
  }

  CopyInsertable(CopyInsertable&& other) : CopyInsertable(other) {}

  // Forgive pair for not downcasting this to an lvalue in its constructors.
  CopyInsertable(CopyInsertable const && other) : CopyInsertable(other) {}


  template <class ...Args>
  CopyInsertable(Args&&...) {
    assert(false);
  }

  ~CopyInsertable() {
    assert(constructed_under_allocator == getConstructController()->isInAllocatorConstruct());
  }

  void reset(int value) {
    data = value;
    copied_once = false;
    constructed_under_allocator = false;
  }
};

template <int ID>
bool operator==(CopyInsertable<ID> const& L, CopyInsertable<ID> const& R) {
  return L.data == R.data;
}


template <int ID>
bool operator!=(CopyInsertable<ID> const& L, CopyInsertable<ID> const& R) {
  return L.data != R.data;
}

template <int ID>
bool operator <(CopyInsertable<ID> const& L, CopyInsertable<ID> const& R) {
  return L.data < R.data;
}

namespace std {
  template <int ID>
  struct hash< ::CopyInsertable<ID> > {
    typedef ::CopyInsertable<ID> argument_type;
    typedef std::size_t result_type;

    std::size_t operator()(argument_type const& arg) const {
      return arg.data;
    }
  };
}

// TCT - Test container type
namespace TCT {

template <class T = CopyInsertable<1>>
using vector = std::vector<T, ContainerTestAllocator<T, T> >;
template <class T = CopyInsertable<1>>
using deque = std::deque<T, ContainerTestAllocator<T, T> >;
template <class T = CopyInsertable<1>>
using list = std::list<T, ContainerTestAllocator<T, T> >;

template <class Key = CopyInsertable<1>, class Value = CopyInsertable<2>,
          class ValueTp = std::pair<const Key, Value> >
using unordered_map =
      std::unordered_map<Key, Value, std::hash<Key>, std::equal_to<Key>,
                              ContainerTestAllocator<ValueTp, ValueTp> >;

template <class Key = CopyInsertable<1>, class Value = CopyInsertable<2>,
          class ValueTp = std::pair<const Key, Value> >
using map =
      std::map<Key, Value, std::less<Key>,
                              ContainerTestAllocator<ValueTp, ValueTp> >;

template <class Key = CopyInsertable<1>, class Value = CopyInsertable<2>,
          class ValueTp = std::pair<const Key, Value> >
using unordered_multimap =
      std::unordered_multimap<Key, Value, std::hash<Key>, std::equal_to<Key>,
                                   ContainerTestAllocator<ValueTp, ValueTp> >;

template <class Key = CopyInsertable<1>, class Value = CopyInsertable<2>,
          class ValueTp = std::pair<const Key, Value> >
using multimap =
      std::multimap<Key, Value, std::less<Key>,
                              ContainerTestAllocator<ValueTp, ValueTp> >;

template <class Value = CopyInsertable<1> >
using unordered_set =
  std::unordered_set<Value, std::hash<Value>, std::equal_to<Value>,
                               ContainerTestAllocator<Value, Value> >;

template <class Value = CopyInsertable<1> >
using set =
    std::set<Value, std::less<Value>, ContainerTestAllocator<Value, Value> >;

template <class Value = CopyInsertable<1> >
using unordered_multiset =
    std::unordered_multiset<Value, std::hash<Value>, std::equal_to<Value>,
                                    ContainerTestAllocator<Value, Value> >;

template <class Value = CopyInsertable<1> >
using multiset =
    std::multiset<Value, std::less<Value>, ContainerTestAllocator<Value, Value> >;

} // namespace TCT

#endif // SUPPORT_CONTAINER_TEST_TYPES_H
