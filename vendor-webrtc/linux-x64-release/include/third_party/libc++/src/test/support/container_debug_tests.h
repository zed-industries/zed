//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_CONTAINER_DEBUG_TESTS_H
#define TEST_SUPPORT_CONTAINER_DEBUG_TESTS_H

#include "test_macros.h"

#ifndef _LIBCPP_VERSION
#error This header may only be used for libc++ tests
#endif

#if _LIBCPP_HARDENING_MODE != _LIBCPP_HARDENING_MODE_DEBUG
#error The library must be built with the debug mode enabled in order to use this header
#endif

#include <utility>
#include <cstddef>
#include <cstdlib>
#include <cassert>

#include "check_assertion.h"
#include "test_allocator.h"

// These test make use of 'if constexpr'.
#if TEST_STD_VER <= 14
#error This header may only be used in C++17 and greater
#endif

namespace IteratorDebugChecks {

enum ContainerType {
  CT_None,
  CT_String,
  CT_Vector,
  CT_VectorBool,
  CT_List,
  CT_Deque,
  CT_ForwardList,
  CT_Map,
  CT_Set,
  CT_MultiMap,
  CT_MultiSet,
  CT_UnorderedMap,
  CT_UnorderedSet,
  CT_UnorderedMultiMap,
  CT_UnorderedMultiSet
};

constexpr bool isSequential(ContainerType CT) {
  return CT >= CT_Vector && CT <= CT_ForwardList;
}

constexpr bool isAssociative(ContainerType CT) {
  return CT >= CT_Map && CT <= CT_MultiSet;
}

constexpr bool isUnordered(ContainerType CT) {
  return CT >= CT_UnorderedMap && CT <= CT_UnorderedMultiSet;
}

constexpr bool isSet(ContainerType CT) {
  return CT == CT_Set
      || CT == CT_MultiSet
      || CT == CT_UnorderedSet
      || CT == CT_UnorderedMultiSet;
}

constexpr bool isMap(ContainerType CT) {
  return CT == CT_Map
      || CT == CT_MultiMap
      || CT == CT_UnorderedMap
      || CT == CT_UnorderedMultiMap;
}

constexpr bool isMulti(ContainerType CT) {
  return CT == CT_MultiMap
      || CT == CT_MultiSet
      || CT == CT_UnorderedMultiMap
      || CT == CT_UnorderedMultiSet;
}

template <class Container, class ValueType = typename Container::value_type>
struct ContainerDebugHelper {
  static_assert(std::is_constructible<ValueType, int>::value,
                "must be constructible from int");

  static ValueType makeValueType(int val = 0, int = 0) {
    return ValueType(val);
  }
};

template <class Container>
struct ContainerDebugHelper<Container, char> {
  static char makeValueType(int = 0, int = 0) {
    return 'A';
  }
};

template <class Container, class Key, class Value>
struct ContainerDebugHelper<Container, std::pair<const Key, Value> > {
  using ValueType = std::pair<const Key, Value>;
  static_assert(std::is_constructible<Key, int>::value,
                "must be constructible from int");
  static_assert(std::is_constructible<Value, int>::value,
                "must be constructible from int");

  static ValueType makeValueType(int key = 0, int val = 0) {
    return ValueType(key, val);
  }
};

template <class Container, ContainerType CT,
    class Helper = ContainerDebugHelper<Container> >
struct BasicContainerChecks {
  using value_type = typename Container::value_type;
  using iterator = typename Container::iterator;
  using const_iterator = typename Container::const_iterator;
  using allocator_type = typename Container::allocator_type;
  using traits = std::iterator_traits<iterator>;
  using category = typename traits::iterator_category;

  static_assert(std::is_same<test_allocator<value_type>, allocator_type>::value,
                "the container must use a test allocator");

  static constexpr bool IsBiDir =
      std::is_convertible<category, std::bidirectional_iterator_tag>::value;

 public:
  static void run() {
    run_iterator_tests();
    run_container_tests();
    run_allocator_aware_tests();
  }

  static void run_iterator_tests() {
    TestNullIterators<iterator>();
    TestNullIterators<const_iterator>();
    if constexpr (IsBiDir) { DecrementBegin(); }
    IncrementEnd();
    DerefEndIterator();
  }

  static void run_container_tests() {
    CopyInvalidatesIterators();
    MoveInvalidatesIterators();
    if constexpr (CT != CT_ForwardList) {
      EraseIter();
      EraseIterIter();
    }
  }

  static void run_allocator_aware_tests() {
    SwapNonEqualAllocators();
    if constexpr (CT != CT_ForwardList ) {
      // FIXME: This should work for both forward_list and string
      SwapInvalidatesIterators();
    }
  }

  static Container makeContainer(int size, allocator_type A = allocator_type()) {
    Container C(A);
    if constexpr (CT == CT_ForwardList) {
      for (int i = 0; i < size; ++i)
        C.insert_after(C.before_begin(), Helper::makeValueType(i));
    } else {
      for (int i = 0; i < size; ++i)
        C.insert(C.end(), Helper::makeValueType(i));
      assert(C.size() == static_cast<std::size_t>(size));
    }
    return C;
  }

  static value_type makeValueType(int value) {
    return Helper::makeValueType(value);
  }

 private:
  // Iterator tests
  template <class Iter>
  static void TestNullIterators() {
    // testing null iterator
    Iter it;
    EXPECT_DEATH( ++it );
    EXPECT_DEATH( it++ );
    EXPECT_DEATH( *it );
    if constexpr (CT != CT_VectorBool) {
      EXPECT_DEATH( it.operator->() );
    }
    if constexpr (IsBiDir) {
      EXPECT_DEATH( --it );
      EXPECT_DEATH( it-- );
    }
  }

  static void DecrementBegin() {
    // testing decrement on begin
    Container C = makeContainer(1);
    iterator i = C.end();
    const_iterator ci = C.cend();
    --i;
    --ci;
    assert(i == C.begin());
    EXPECT_DEATH( --i );
    EXPECT_DEATH( i-- );
    EXPECT_DEATH( --ci );
    EXPECT_DEATH( ci-- );
  }

  static void IncrementEnd() {
    // testing increment on end
    Container C = makeContainer(1);
    iterator i = C.begin();
    const_iterator ci = C.begin();
    ++i;
    ++ci;
    assert(i == C.end());
    EXPECT_DEATH( ++i );
    EXPECT_DEATH( i++ );
    EXPECT_DEATH( ++ci );
    EXPECT_DEATH( ci++ );
  }

  static void DerefEndIterator() {
    // testing deref end iterator
    Container C = makeContainer(1);
    iterator i = C.begin();
    const_iterator ci = C.cbegin();
    (void)*i; (void)*ci;
    if constexpr (CT != CT_VectorBool) {
      i.operator->();
      ci.operator->();
    }
    ++i; ++ci;
    assert(i == C.end());
    EXPECT_DEATH( *i );
    EXPECT_DEATH( *ci );
    if constexpr (CT != CT_VectorBool) {
      EXPECT_DEATH( i.operator->() );
      EXPECT_DEATH( ci.operator->() );
    }
  }

  // Container tests
  static void CopyInvalidatesIterators() {
    // copy invalidates iterators
    Container C1 = makeContainer(3);
    iterator i = C1.begin();
    Container C2 = C1;
    if constexpr (CT == CT_ForwardList) {
      iterator i_next = i;
      ++i_next;
      (void)*i_next;
      EXPECT_DEATH( C2.erase_after(i) );
      C1.erase_after(i);
      EXPECT_DEATH( *i_next );
    } else {
      EXPECT_DEATH( C2.erase(i) );
      (void)*i;
      C1.erase(i);
      EXPECT_DEATH( *i );
    }
  }

  static void MoveInvalidatesIterators() {
    // copy move invalidates iterators
    Container C1 = makeContainer(3);
    iterator i = C1.begin();
    Container C2 = std::move(C1);
    (void) *i;
    if constexpr (CT == CT_ForwardList) {
      EXPECT_DEATH( C1.erase_after(i) );
      C2.erase_after(i);
    } else {
      EXPECT_DEATH( C1.erase(i) );
      C2.erase(i);
      EXPECT_DEATH(*i);
    }
  }

  static void EraseIter() {
    // testing erase invalidation
    Container C1 = makeContainer(2);
    iterator it1 = C1.begin();
    iterator it1_next = it1;
    ++it1_next;
    Container C2 = C1;
    EXPECT_DEATH( C2.erase(it1) ); // wrong container
    EXPECT_DEATH( C2.erase(C2.end()) ); // erase with end
    C1.erase(it1_next);
    EXPECT_DEATH( C1.erase(it1_next) ); // invalidated iterator
    C1.erase(it1);
    EXPECT_DEATH( C1.erase(it1) ); // invalidated iterator
  }

  static void EraseIterIter() {
    // testing erase iter iter invalidation
    Container C1 = makeContainer(2);
    iterator it1 = C1.begin();
    iterator it1_next = it1;
    ++it1_next;
    Container C2 = C1;
    iterator it2 = C2.begin();
    iterator it2_next = it2;
    ++it2_next;
    EXPECT_DEATH( C2.erase(it1, it1_next) ); // begin from wrong container
    EXPECT_DEATH( C2.erase(it1, it2_next) ); // end   from wrong container
    EXPECT_DEATH( C2.erase(it2, it1_next) ); // both  from wrong container
    C2.erase(it2, it2_next);
  }

  // Allocator aware tests
  static void SwapInvalidatesIterators() {
    // testing swap invalidates iterators
    Container C1 = makeContainer(3);
    Container C2 = makeContainer(3);
    iterator it1 = C1.begin();
    iterator it2 = C2.begin();
    swap(C1, C2);
    EXPECT_DEATH( C1.erase(it1) );
    if (CT == CT_String) {
      EXPECT_DEATH(C1.erase(it2));
    } else
      C1.erase(it2);
    //C2.erase(it1);
    EXPECT_DEATH( C1.erase(it1) );
  }

  static void SwapNonEqualAllocators() {
    // testing swap with non-equal allocators
    Container C1 = makeContainer(3, allocator_type(1));
    Container C2 = makeContainer(1, allocator_type(2));
    Container C3 = makeContainer(2, allocator_type(2));
    swap(C2, C3);
    EXPECT_DEATH( swap(C1, C2) );
  }

 private:
  BasicContainerChecks() = delete;
};

} // namespace IteratorDebugChecks

#endif // TEST_SUPPORT_CONTAINER_DEBUG_TESTS_H
