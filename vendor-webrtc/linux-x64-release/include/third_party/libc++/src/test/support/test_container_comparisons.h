//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//  Utility functions to test comparisons on containers.

#ifndef TEST_CONTAINER_COMPARISONS
#define TEST_CONTAINER_COMPARISONS

#include <functional>
#include <set>

#include "test_allocator.h"
#include "test_comparisons.h"

// Implementation detail of `test_sequence_container_spaceship`
template <template <typename...> typename Container, typename Elem, typename Allocator, typename Order>
constexpr void test_sequence_container_spaceship_with_type() {
  // Empty containers
  {
    Container<Elem, Allocator> l1;
    Container<Elem, Allocator> l2;
    assert(testOrder(l1, l2, Order::equivalent));
  }
  // Identical contents
  {
    Container<Elem, Allocator> l1{1, 1};
    Container<Elem, Allocator> l2{1, 1};
    assert(testOrder(l1, l2, Order::equivalent));
  }
  // Less, due to contained values
  {
    Container<Elem, Allocator> l1{1, 1};
    Container<Elem, Allocator> l2{1, 2};
    assert(testOrder(l1, l2, Order::less));
  }
  // Greater, due to contained values
  {
    Container<Elem, Allocator> l1{1, 3};
    Container<Elem, Allocator> l2{1, 2};
    assert(testOrder(l1, l2, Order::greater));
  }
  // Shorter list
  {
    Container<Elem, Allocator> l1{1};
    Container<Elem, Allocator> l2{1, 2};
    assert(testOrder(l1, l2, Order::less));
  }
  // Longer list
  {
    Container<Elem, Allocator> l1{1, 2};
    Container<Elem, Allocator> l2{1};
    assert(testOrder(l1, l2, Order::greater));
  }
  // Unordered
  if constexpr (std::is_same_v<Elem, PartialOrder>) {
    Container<Elem, Allocator> l1{1, std::numeric_limits<int>::min()};
    Container<Elem, Allocator> l2{1, 2};
    assert(testOrder(l1, l2, Order::unordered));
  }
}

// Tests the `operator<=>` on sequence containers
template <template <typename...> typename Container>
constexpr bool test_sequence_container_spaceship() {
  // The container should fulfill `std::three_way_comparable`
  static_assert(std::three_way_comparable<Container<int>>);

  // Test different comparison categories
  test_sequence_container_spaceship_with_type<Container, int, std::allocator<int>, std::strong_ordering>();
  test_sequence_container_spaceship_with_type<Container,
                                              StrongOrder,
                                              test_allocator<StrongOrder>,
                                              std::strong_ordering>();
  test_sequence_container_spaceship_with_type<Container, WeakOrder, std::allocator<WeakOrder>, std::weak_ordering>();
  test_sequence_container_spaceship_with_type<Container,
                                              PartialOrder,
                                              test_allocator<PartialOrder>,
                                              std::partial_ordering>();

  // `LessAndEqComp` does not have `operator<=>`. Ordering is synthesized based on `operator<`
  test_sequence_container_spaceship_with_type<Container,
                                              LessAndEqComp,
                                              std::allocator<LessAndEqComp>,
                                              std::weak_ordering>();

  // Thanks to SFINAE, the following is not a compiler error but returns `false`
  struct NonComparable {};
  static_assert(!std::three_way_comparable<Container<NonComparable>>);

  return true;
}

// Implementation detail of `test_sequence_container_adaptor_spaceship`
template <template <typename...> typename ContainerAdaptor,
          template <typename...>
          typename Container,
          typename Elem,
          typename Order>
constexpr void test_sequence_container_adaptor_spaceship_with_type() {
  // Empty containers
  {
    Container<Elem> l1;
    ContainerAdaptor<Elem, Container<Elem>> ca1{l1};
    Container<Elem> l2;
    ContainerAdaptor<Elem, Container<Elem>> ca2{l2};
    assert(testOrder(ca1, ca2, Order::equivalent));
  }
  // Identical contents
  {
    Container<Elem> l1{1, 1};
    ContainerAdaptor<Elem, Container<Elem>> ca1{l1};
    Container<Elem> l2{1, 1};
    ContainerAdaptor<Elem, Container<Elem>> ca2{l2};
    assert(testOrder(ca1, ca2, Order::equivalent));
  }
  // Less, due to contained values
  {
    Container<Elem> l1{1, 1};
    ContainerAdaptor<Elem, Container<Elem>> ca1{l1};
    Container<Elem> l2{1, 2};
    ContainerAdaptor<Elem, Container<Elem>> ca2{l2};
    assert(testOrder(ca1, ca2, Order::less));
  }
  // Greater, due to contained values
  {
    Container<Elem> l1{1, 3};
    ContainerAdaptor<Elem, Container<Elem>> ca1{l1};
    Container<Elem> l2{1, 2};
    ContainerAdaptor<Elem, Container<Elem>> ca2{l2};
    assert(testOrder(ca1, ca2, Order::greater));
  }
  // Shorter list
  {
    Container<Elem> l1{1};
    ContainerAdaptor<Elem, Container<Elem>> ca1{l1};
    Container<Elem> l2{1, 2};
    ContainerAdaptor<Elem, Container<Elem>> ca2{l2};
    assert(testOrder(ca1, ca2, Order::less));
  }
  // Longer list
  {
    Container<Elem> l1{1, 2};
    ContainerAdaptor<Elem, Container<Elem>> ca1{l1};
    Container<Elem> l2{1};
    ContainerAdaptor<Elem, Container<Elem>> ca2{l2};
    assert(testOrder(ca1, ca2, Order::greater));
  }
  // Unordered
  if constexpr (std::is_same_v<Elem, PartialOrder>) {
    Container<Elem> l1{1, std::numeric_limits<int>::min()};
    ContainerAdaptor<Elem, Container<Elem>> ca1{l1};
    Container<Elem> l2{1, 2};
    ContainerAdaptor<Elem, Container<Elem>> ca2{l2};
    assert(testOrder(ca1, ca2, Order::unordered));
  }
}

// Tests the `operator<=>` on sequence container adaptors
template <template <typename...> typename ContainerAdaptor, template <typename...> typename Container>
constexpr bool test_sequence_container_adaptor_spaceship() {
  // Thanks to SFINAE, the following is not a compiler error but returns `false`
  struct NonComparable {};
  static_assert(!std::three_way_comparable<ContainerAdaptor<NonComparable>>);

  // The container should fulfill `std::three_way_comparable`
  static_assert(std::three_way_comparable<ContainerAdaptor<int, Container<int>>>);

  // Test different comparison categories
  test_sequence_container_adaptor_spaceship_with_type<ContainerAdaptor, Container, int, std::strong_ordering>();
  test_sequence_container_adaptor_spaceship_with_type<ContainerAdaptor, Container, StrongOrder, std::strong_ordering>();
  test_sequence_container_adaptor_spaceship_with_type<ContainerAdaptor, Container, WeakOrder, std::weak_ordering>();
  test_sequence_container_adaptor_spaceship_with_type<ContainerAdaptor,
                                                      Container,
                                                      PartialOrder,
                                                      std::partial_ordering>();

  // `LessAndEqComp` does not have `operator<=>`. Ordering is synthesized based on `operator<`
  test_sequence_container_adaptor_spaceship_with_type<ContainerAdaptor, Container, LessAndEqComp, std::weak_ordering>();

  return true;
}

// Implementation detail of `test_ordered_map_container_spaceship`
template <template <typename...> typename Container,
          typename Key,
          typename Val,
          typename Allocator,
          typename Order,
          typename Compare>
constexpr void test_ordered_map_container_spaceship_with_type(Compare comp) {
  // Empty containers
  {
    Container<Key, Val, Compare, Allocator> l1{{}, comp};
    Container<Key, Val, Compare, Allocator> l2{{}, comp};
    assert(testOrder(l1, l2, Order::equivalent));
  }
  // Identical contents
  {
    Container<Key, Val, Compare, Allocator> l1{{{1, 1}, {2, 1}}, comp};
    Container<Key, Val, Compare, Allocator> l2{{{1, 1}, {2, 1}}, comp};
    assert(testOrder(l1, l2, Order::equivalent));
  }
  // Less, due to contained values
  {
    Container<Key, Val, Compare, Allocator> l1{{{1, 1}, {2, 1}}, comp};
    Container<Key, Val, Compare, Allocator> l2{{{1, 1}, {2, 2}}, comp};
    assert(testOrder(l1, l2, Order::less));
  }
  // Greater, due to contained values
  {
    Container<Key, Val, Compare, Allocator> l1{{{1, 1}, {2, 3}}, comp};
    Container<Key, Val, Compare, Allocator> l2{{{1, 1}, {2, 2}}, comp};
    assert(testOrder(l1, l2, Order::greater));
  }
  // Shorter list
  {
    Container<Key, Val, Compare, Allocator> l1{{{1, 1}}, comp};
    Container<Key, Val, Compare, Allocator> l2{{{1, 1}, {2, 2}}, comp};
    assert(testOrder(l1, l2, Order::less));
  }
  // Longer list
  {
    Container<Key, Val, Compare, Allocator> l1{{{1, 2}, {2, 2}}, comp};
    Container<Key, Val, Compare, Allocator> l2{{{1, 1}}, comp};
    assert(testOrder(l1, l2, Order::greater));
  }
  // Unordered
  if constexpr (std::is_same_v<Val, PartialOrder>) {
    Container<Key, Val, Compare, Allocator> l1{{{1, 1}, {2, std::numeric_limits<int>::min()}}, comp};
    Container<Key, Val, Compare, Allocator> l2{{{1, 1}, {2, 2}}, comp};
    assert(testOrder(l1, l2, Order::unordered));
  }

  // Identical contents
  {
    Container<Key, Val, Compare, Allocator> l1{{{1, 1}, {2, 1}, {2, 2}}, comp};
    Container<Key, Val, Compare, Allocator> l2{{{1, 1}, {2, 1}, {2, 2}}, comp};
    assert(testOrder(l1, l2, Order::equivalent));

    Container<Key, Val, Compare, Allocator> l3{{{1, 1}, {2, 1}, {2, 2}}, comp};
    Container<Key, Val, Compare, Allocator> l4{{{2, 1}, {2, 2}, {1, 1}}, comp};
    assert(testOrder(l3, l4, Order::equivalent));
  }
  // Less, due to contained values
  {
    Container<Key, Val, Compare, Allocator> l1{{{1, 1}, {2, 1}, {2, 1}}, comp};
    Container<Key, Val, Compare, Allocator> l2{{{1, 1}, {2, 2}, {2, 2}}, comp};
    assert(testOrder(l1, l2, Order::less));

    Container<Key, Val, Compare, Allocator> l3{{{1, 1}, {2, 1}, {2, 1}}, comp};
    Container<Key, Val, Compare, Allocator> l4{{{2, 2}, {2, 2}, {1, 1}}, comp};
    assert(testOrder(l3, l4, Order::less));
  }
  // Greater, due to contained values
  {
    Container<Key, Val, Compare, Allocator> l1{{{1, 1}, {2, 3}, {2, 3}}, comp};
    Container<Key, Val, Compare, Allocator> l2{{{1, 1}, {2, 2}, {2, 2}}, comp};
    assert(testOrder(l1, l2, Order::greater));

    Container<Key, Val, Compare, Allocator> l3{{{1, 1}, {2, 3}, {2, 3}}, comp};
    Container<Key, Val, Compare, Allocator> l4{{{2, 2}, {2, 2}, {1, 1}}, comp};
    assert(testOrder(l3, l4, Order::greater));
  }
  // Shorter list
  {
    Container<Key, Val, Compare, Allocator> l1{{{1, 1}, {2, 2}}, comp};
    Container<Key, Val, Compare, Allocator> l2{{{1, 1}, {2, 2}, {2, 2}, {3, 1}}, comp};
    assert(testOrder(l1, l2, Order::less));

    Container<Key, Val, Compare, Allocator> l3{{{1, 1}, {2, 2}}, comp};
    Container<Key, Val, Compare, Allocator> l4{{{3, 1}, {2, 2}, {2, 2}, {1, 1}}, comp};
    assert(testOrder(l3, l4, Order::less));
  }
  // Longer list
  {
    Container<Key, Val, Compare, Allocator> l1{{{1, 2}, {2, 2}, {2, 2}, {3, 1}}, comp};
    Container<Key, Val, Compare, Allocator> l2{{{1, 1}, {2, 2}}, comp};
    assert(testOrder(l1, l2, Order::greater));

    Container<Key, Val, Compare, Allocator> l3{{{1, 2}, {2, 2}, {2, 2}, {3, 1}}, comp};
    Container<Key, Val, Compare, Allocator> l4{{{2, 2}, {1, 1}}, comp};
    assert(testOrder(l3, l4, Order::greater));
  }
  // Unordered
  if constexpr (std::is_same_v<Val, PartialOrder>) {
    Container<Key, Val, Compare, Allocator> l1{{{1, 1}, {2, std::numeric_limits<int>::min()}, {2, 3}}, comp};
    Container<Key, Val, Compare, Allocator> l2{{{1, 1}, {2, 2}, {2, 3}}, comp};
    assert(testOrder(l1, l2, Order::unordered));

    Container<Key, Val, Compare, Allocator> l3{{{1, 1}, {2, std::numeric_limits<int>::min()}, {2, 3}}, comp};
    Container<Key, Val, Compare, Allocator> l4{{{2, 3}, {2, 2}, {1, 1}}, comp};
    assert(testOrder(l3, l4, Order::unordered));
  }
}

// Tests the `operator<=>` on ordered map containers
template <template <typename...> typename Container>
constexpr bool test_ordered_map_container_spaceship() {
  // Thanks to SFINAE, the following is not a compiler error but returns `false`
  struct NonComparable {};
  static_assert(!std::three_way_comparable<Container<int, NonComparable>>);

  // The container should fulfill `std::three_way_comparable`
  static_assert(std::three_way_comparable<Container<int, int>>);

  // Test different comparison categories
  test_ordered_map_container_spaceship_with_type<Container,
                                                 int,
                                                 int,
                                                 std::allocator<std::pair<const int, int>>,
                                                 std::strong_ordering>(std::less{});
  test_ordered_map_container_spaceship_with_type<Container,
                                                 int,
                                                 int,
                                                 test_allocator<std::pair<const int, int>>,
                                                 std::strong_ordering>(std::greater{});
  test_ordered_map_container_spaceship_with_type<Container,
                                                 int,
                                                 StrongOrder,
                                                 std::allocator<std::pair<const int, StrongOrder>>,
                                                 std::strong_ordering>(std::less{});
  test_ordered_map_container_spaceship_with_type<Container,
                                                 int,
                                                 StrongOrder,
                                                 test_allocator<std::pair<const int, StrongOrder>>,
                                                 std::strong_ordering>(std::greater{});
  test_ordered_map_container_spaceship_with_type<Container,
                                                 int,
                                                 WeakOrder,
                                                 std::allocator<std::pair<const int, WeakOrder>>,
                                                 std::weak_ordering>(std::less{});
  test_ordered_map_container_spaceship_with_type<Container,
                                                 int,
                                                 WeakOrder,
                                                 test_allocator<std::pair<const int, WeakOrder>>,
                                                 std::weak_ordering>(std::greater{});
  test_ordered_map_container_spaceship_with_type<Container,
                                                 int,
                                                 PartialOrder,
                                                 std::allocator<std::pair<const int, PartialOrder>>,
                                                 std::partial_ordering>(std ::less{});
  test_ordered_map_container_spaceship_with_type<Container,
                                                 int,
                                                 PartialOrder,
                                                 test_allocator<std::pair<const int, PartialOrder>>,
                                                 std::partial_ordering>(std ::greater{});

  // `LessAndEqComp` does not have `operator<=>`. Ordering is synthesized based on `operator<`
  test_ordered_map_container_spaceship_with_type<Container,
                                                 int,
                                                 LessAndEqComp,
                                                 std::allocator<std::pair<const int, LessAndEqComp>>,
                                                 std::weak_ordering>(std::less{});

  return true;
}

// Implementation detail of `test_ordered_set_container_spaceship`
template <template <typename...> typename Container,
          typename Elem,
          typename Allocator,
          typename Order,
          typename Compare>
constexpr void test_ordered_set_spaceship_with_type(Compare comp) {
  // Empty containers
  {
    Container<Elem, Compare, Allocator> l1{{}, comp};
    Container<Elem, Compare, Allocator> l2{{}, comp};
    assert(testOrder(l1, l2, Order::equivalent));
  }
  // Identical contents
  {
    Container<Elem, Compare, Allocator> l1{{1, 1, 2}, comp};
    Container<Elem, Compare, Allocator> l2{{1, 1, 2}, comp};
    assert(testOrder(l1, l2, Order::equivalent));
  }
  // Less, due to contained values
  {
    Container<Elem, Compare, Allocator> l1{{1, 1, 2, 3}, comp};
    Container<Elem, Compare, Allocator> l2{{1, 2, 2, 4}, comp};
    assert(testOrder(l1, l2, Order::less));
  }
  // Greater, due to contained values
  {
    Container<Elem, Compare, Allocator> l1{{1, 2, 2, 4}, comp};
    Container<Elem, Compare, Allocator> l2{{1, 1, 2, 3}, comp};
    assert(testOrder(l1, l2, Order::greater));
  }
  // Shorter list
  {
    Container<Elem, Compare, Allocator> l1{{1, 1, 2, 2}, comp};
    Container<Elem, Compare, Allocator> l2{{1, 1, 2, 2, 3}, comp};
    assert(testOrder(l1, l2, Order::less));
  }
  // Longer list
  {
    Container<Elem, Compare, Allocator> l1{{1, 1, 2, 2, 3}, comp};
    Container<Elem, Compare, Allocator> l2{{1, 1, 2, 2}, comp};
    assert(testOrder(l1, l2, Order::greater));
  }
  // Unordered
  if constexpr (std::is_same_v< Container<Elem>, std::multiset<PartialOrder>>) {
    if constexpr (std::is_same_v<Elem, PartialOrder> && std::is_same_v<Compare, decltype(std::less{})>) {
      Container<Elem, Compare, Allocator> l1{{1, std::numeric_limits<int>::min()}, comp};
      Container<Elem, Compare, Allocator> l2{{1, 2}, comp};
      assert(testOrder(l1, l2, Order::unordered));
    }
    if constexpr (std::is_same_v<Elem, PartialOrder> && std::is_same_v<Compare, decltype(std::less{})>) {
      Container<Elem, Compare, Allocator> l1{{1, std::numeric_limits<int>::max()}, comp};
      Container<Elem, Compare, Allocator> l2{{1, 2}, comp};
      assert(testOrder(l1, l2, Order::unordered));
    }
  }
  if constexpr (std::is_same_v< Container<Elem>, std::set<PartialOrder>>) {
    // Unordered values are not supported for `set`
    if constexpr (std::is_same_v<Elem, PartialOrder> && std::is_same_v<Compare, decltype(std::less{})>) {
      Container<Elem, Compare, Allocator> l1{{1, std::numeric_limits<int>::min()}, comp};
      Container<Elem, Compare, Allocator> l2{{1, 2}, comp};
      assert(testOrder(l1, l2, Order::less));
    }
    if constexpr (std::is_same_v<Elem, PartialOrder> && std::is_same_v<Compare, decltype(std::less{})>) {
      Container<Elem, Compare, Allocator> l1{{1, std::numeric_limits<int>::max()}, comp};
      Container<Elem, Compare, Allocator> l2{{1, 2}, comp};
      assert(testOrder(l1, l2, Order::less));
    }
  }
  if constexpr (std::is_same_v<Elem, PartialOrder> && std::is_same_v<Compare, decltype(std::greater{})>) {
    Container<Elem, Compare, Allocator> l1{{1, std::numeric_limits<int>::min()}, comp};
    Container<Elem, Compare, Allocator> l2{{1, 2}, comp};
    assert(testOrder(l1, l2, Order::less));
  }
  if constexpr (std::is_same_v<Elem, PartialOrder> && std::is_same_v<Compare, decltype(std::greater{})>) {
    Container<Elem, Compare, Allocator> l1{{1, std::numeric_limits<int>::max()}, comp};
    Container<Elem, Compare, Allocator> l2{{1, 2}, comp};
    assert(testOrder(l1, l2, Order::less));
  }
}

// Tests the `operator<=>` on ordered set containers
template <template <typename...> typename Container>
constexpr bool test_ordered_set_container_spaceship() {
  // Thanks to SFINAE, the following is not a compiler error but returns `false`
  struct NonComparable {};
  static_assert(!std::three_way_comparable<Container<NonComparable>>);

  // The container should fulfill `std::three_way_comparable`
  static_assert(std::three_way_comparable<Container<int>>);

  // Test different comparison categories
  test_ordered_set_spaceship_with_type<Container, int, std::allocator<int>, std::strong_ordering>(std::less{});
  test_ordered_set_spaceship_with_type<Container, int, test_allocator<int>, std::strong_ordering>(std::greater{});
  test_ordered_set_spaceship_with_type<Container, StrongOrder, std::allocator<StrongOrder>, std::strong_ordering>(
      std::less{});
  test_ordered_set_spaceship_with_type<Container, StrongOrder, test_allocator<StrongOrder>, std::strong_ordering>(
      std::greater{});
  test_ordered_set_spaceship_with_type<Container, WeakOrder, std::allocator<WeakOrder>, std::weak_ordering>(
      std::less{});
  test_ordered_set_spaceship_with_type<Container, WeakOrder, test_allocator<WeakOrder>, std::weak_ordering>(
      std::greater{});
  test_ordered_set_spaceship_with_type<Container, PartialOrder, std::allocator<PartialOrder>, std::partial_ordering>(
      std::less{});
  test_ordered_set_spaceship_with_type<Container, PartialOrder, test_allocator<PartialOrder>, std::partial_ordering>(
      std::greater{});

  // `LessAndEqComp` does not have `operator<=>`. Ordering is synthesized based on `operator<`
  test_ordered_set_spaceship_with_type<Container, LessAndEqComp, std::allocator<LessAndEqComp>, std::weak_ordering>(
      std::less{});

  return true;
}

#endif
