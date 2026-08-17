//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_ITERATOR_TRAITS_ITERATOR_TRAITS_CPP17_ITERATORS
#define TEST_SUPPORT_ITERATOR_TRAITS_ITERATOR_TRAITS_CPP17_ITERATORS

struct iterator_traits_cpp17_iterator {
  int& operator*();
  iterator_traits_cpp17_iterator& operator++();
  iterator_traits_cpp17_iterator operator++(int);
};

struct iterator_traits_cpp17_proxy_iterator {
  int operator*();
  iterator_traits_cpp17_proxy_iterator& operator++();

  // this returns legacy_iterator, not iterator_traits_cpp17_proxy_iterator
  iterator_traits_cpp17_iterator operator++(int);
};

struct iterator_traits_cpp17_input_iterator {
  using difference_type = int;
  using value_type = long;

  int& operator*();
  iterator_traits_cpp17_input_iterator& operator++();
  iterator_traits_cpp17_input_iterator operator++(int);

  bool operator==(iterator_traits_cpp17_input_iterator const&) const;
};

struct iterator_traits_cpp17_proxy_input_iterator {
  using difference_type = int;
  using value_type = long;

  int operator*();
  iterator_traits_cpp17_proxy_input_iterator& operator++();

  // this returns legacy_input_iterator, not iterator_traits_cpp17_proxy_input_iterator
  iterator_traits_cpp17_input_iterator operator++(int);

  bool operator==(iterator_traits_cpp17_proxy_input_iterator const&) const;
};

struct iterator_traits_cpp17_forward_iterator {
  using difference_type = int;
  using value_type = int;

  int& operator*();
  iterator_traits_cpp17_forward_iterator& operator++();
  iterator_traits_cpp17_forward_iterator operator++(int);

  bool operator==(iterator_traits_cpp17_forward_iterator const&) const;
};

struct iterator_traits_cpp17_bidirectional_iterator {
  using difference_type = int;
  using value_type = int;

  int& operator*();
  iterator_traits_cpp17_bidirectional_iterator& operator++();
  iterator_traits_cpp17_bidirectional_iterator operator++(int);
  iterator_traits_cpp17_bidirectional_iterator& operator--();
  iterator_traits_cpp17_bidirectional_iterator operator--(int);

  bool operator==(iterator_traits_cpp17_bidirectional_iterator const&) const;
};

struct iterator_traits_cpp17_random_access_iterator {
  using difference_type = int;
  using value_type = int;

  int& operator*();
  int& operator[](difference_type);
  iterator_traits_cpp17_random_access_iterator& operator++();
  iterator_traits_cpp17_random_access_iterator operator++(int);
  iterator_traits_cpp17_random_access_iterator& operator--();
  iterator_traits_cpp17_random_access_iterator operator--(int);

  bool operator==(iterator_traits_cpp17_random_access_iterator const&) const;
  bool operator<(iterator_traits_cpp17_random_access_iterator const&) const;
  bool operator>(iterator_traits_cpp17_random_access_iterator const&) const;
  bool operator<=(iterator_traits_cpp17_random_access_iterator const&) const;
  bool operator>=(iterator_traits_cpp17_random_access_iterator const&) const;

  iterator_traits_cpp17_random_access_iterator& operator+=(difference_type);
  iterator_traits_cpp17_random_access_iterator& operator-=(difference_type);

  friend iterator_traits_cpp17_random_access_iterator operator+(iterator_traits_cpp17_random_access_iterator,
                                                                difference_type);
  friend iterator_traits_cpp17_random_access_iterator operator+(difference_type,
                                                                iterator_traits_cpp17_random_access_iterator);
  friend iterator_traits_cpp17_random_access_iterator operator-(iterator_traits_cpp17_random_access_iterator,
                                                                difference_type);
  friend difference_type operator-(iterator_traits_cpp17_random_access_iterator,
                                   iterator_traits_cpp17_random_access_iterator);
};

#endif // TEST_SUPPORT_ITERATOR_TRAITS_ITERATOR_TRAITS_CPP17_ITERATORS
