//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef MAP_ALLOCATOR_REQUIREMENT_TEST_TEMPLATES_H
#define MAP_ALLOCATOR_REQUIREMENT_TEST_TEMPLATES_H

// <map>
// <unordered_map>

// class map
// class unordered_map

// insert(...);
// emplace(...);
// emplace_hint(...);

// UNSUPPORTED: c++03

#include <cassert>
#include <iterator>

#include "test_macros.h"
#include "count_new.h"
#include "container_test_types.h"

template <class Container>
void testMapInsert() {
  typedef typename Container::value_type ValueTp;
  ConstructController* cc = getConstructController();
  cc->reset();
  {
    // Testing C::insert(const value_type&)
    Container c;
    const ValueTp v(42, 1);
    cc->expect<const ValueTp&>();
    assert(c.insert(v).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42, 1);
      assert(c.insert(v2).second == false);
    }
  }
  {
    // Testing C::insert(value_type&)
    Container c;
    ValueTp v(42, 1);
    cc->expect<const ValueTp&>();
    assert(c.insert(v).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42, 1);
      assert(c.insert(v2).second == false);
    }
  }
  {
    // Testing C::insert(value_type&&)
    Container c;
    ValueTp v(42, 1);
    cc->expect<ValueTp&&>();
    assert(c.insert(std::move(v)).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42, 1);
      assert(c.insert(std::move(v2)).second == false);
    }
  }
  {
    // Testing C::insert(const value_type&&)
    Container c;
    const ValueTp v(42, 1);
    cc->expect<const ValueTp&>();
    assert(c.insert(std::move(v)).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42, 1);
      assert(c.insert(std::move(v2)).second == false);
    }
  }
  {
    // Testing C::insert({key, value})
    Container c;
    cc->expect<ValueTp&&>();
    assert(c.insert({42, 1}).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42, 1);
      assert(c.insert(std::move(v2)).second == false);
    }
  }
  {
    // Testing C::insert(std::initializer_list<ValueTp>)
    Container c;
    std::initializer_list<ValueTp> il = {ValueTp(1, 1), ValueTp(2, 1)};
    cc->expect<ValueTp const&>(2);
    c.insert(il);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      c.insert(il);
    }
  }
  {
    // Testing C::insert(Iter, Iter) for *Iter = value_type const&
    Container c;
    const ValueTp ValueList[] = {ValueTp(1, 1), ValueTp(2, 1), ValueTp(3, 1)};
    cc->expect<ValueTp const&>(3);
    c.insert(std::begin(ValueList), std::end(ValueList));
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      c.insert(std::begin(ValueList), std::end(ValueList));
    }
  }
  {
    // Testing C::insert(Iter, Iter) for *Iter = value_type&&
    Container c;
    ValueTp ValueList[] = {ValueTp(1, 1), ValueTp(2, 1), ValueTp(3, 1)};
    cc->expect<ValueTp&&>(3);
    c.insert(std::move_iterator<ValueTp*>(std::begin(ValueList)), std::move_iterator<ValueTp*>(std::end(ValueList)));
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp ValueList2[] = {ValueTp(1, 1), ValueTp(2, 1), ValueTp(3, 1)};
      c.insert(std::move_iterator<ValueTp*>(std::begin(ValueList2)),
               std::move_iterator<ValueTp*>(std::end(ValueList2)));
    }
  }
  {
    // Testing C::insert(Iter, Iter) for *Iter = value_type&
    Container c;
    ValueTp ValueList[] = {ValueTp(1, 1), ValueTp(2, 1), ValueTp(3, 1)};
    cc->expect<ValueTp const&>(3);
    c.insert(std::begin(ValueList), std::end(ValueList));
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      c.insert(std::begin(ValueList), std::end(ValueList));
    }
  }
}

template <class Container>
void testMapInsertHint() {
  typedef typename Container::value_type ValueTp;
  typedef typename Container::key_type Key;
  typedef typename Container::mapped_type Mapped;
  typedef typename std::pair<Key, Mapped> NonConstKeyPair;
  typedef Container C;
  typedef typename C::iterator It;
  ConstructController* cc = getConstructController();
  cc->reset();
  {
    // Testing C::insert(p, const value_type&)
    Container c;
    const ValueTp v(42, 1);
    cc->expect<const ValueTp&>();
    It ret = c.insert(c.end(), v);
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42, 1);
      It ret2 = c.insert(c.begin(), v2);
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::insert(p, value_type&)
    Container c;
    ValueTp v(42, 1);
    cc->expect<ValueTp const&>();
    It ret = c.insert(c.end(), v);
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42, 1);
      It ret2 = c.insert(c.begin(), v2);
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::insert(p, value_type&&)
    Container c;
    ValueTp v(42, 1);
    cc->expect<ValueTp&&>();
    It ret = c.insert(c.end(), std::move(v));
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42, 1);
      It ret2 = c.insert(c.begin(), std::move(v2));
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::insert(p, {key, value})
    Container c;
    cc->expect<ValueTp&&>();
    It ret = c.insert(c.end(), {42, 1});
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      It ret2 = c.insert(c.begin(), {42, 1});
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::insert(p, const value_type&&)
    Container c;
    const ValueTp v(42, 1);
    cc->expect<const ValueTp&>();
    It ret = c.insert(c.end(), std::move(v));
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42, 1);
      It ret2 = c.insert(c.begin(), std::move(v2));
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::insert(p, pair<Key, Mapped> const&)
    Container c;
    const NonConstKeyPair v(42, 1);
    cc->expect<const NonConstKeyPair&>();
    It ret = c.insert(c.end(), v);
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const NonConstKeyPair v2(42, 1);
      It ret2 = c.insert(c.begin(), v2);
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::insert(p, pair<Key, Mapped>&&)
    Container c;
    NonConstKeyPair v(42, 1);
    cc->expect<NonConstKeyPair&&>();
    It ret = c.insert(c.end(), std::move(v));
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      NonConstKeyPair v2(42, 1);
      It ret2 = c.insert(c.begin(), std::move(v2));
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
}

template <class Container>
void testMapEmplace() {
  typedef typename Container::value_type ValueTp;
  typedef typename Container::key_type Key;
  typedef typename Container::mapped_type Mapped;
  typedef typename std::pair<Key, Mapped> NonConstKeyPair;
  ConstructController* cc = getConstructController();
  cc->reset();
  {
    // Testing C::emplace(const value_type&)
    Container c;
    const ValueTp v(42, 1);
    cc->expect<const ValueTp&>();
    assert(c.emplace(v).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42, 1);
      assert(c.emplace(v2).second == false);
    }
  }
  {
    // Testing C::emplace(value_type&)
    Container c;
    ValueTp v(42, 1);
    cc->expect<ValueTp&>();
    assert(c.emplace(v).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42, 1);
      assert(c.emplace(v2).second == false);
    }
  }
  {
    // Testing C::emplace(value_type&&)
    Container c;
    ValueTp v(42, 1);
    cc->expect<ValueTp&&>();
    assert(c.emplace(std::move(v)).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42, 1);
      assert(c.emplace(std::move(v2)).second == false);
    }
  }
  {
    // Testing C::emplace(const value_type&&)
    Container c;
    const ValueTp v(42, 1);
    cc->expect<const ValueTp&&>();
    assert(c.emplace(std::move(v)).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42, 1);
      assert(c.emplace(std::move(v2)).second == false);
    }
  }
  {
    // Testing C::emplace(pair<Key, Mapped> const&)
    Container c;
    const NonConstKeyPair v(42, 1);
    cc->expect<const NonConstKeyPair&>();
    assert(c.emplace(v).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const NonConstKeyPair v2(42, 1);
      assert(c.emplace(v2).second == false);
    }
  }
  {
    // Testing C::emplace(pair<Key, Mapped> &&)
    Container c;
    NonConstKeyPair v(42, 1);
    cc->expect<NonConstKeyPair&&>();
    assert(c.emplace(std::move(v)).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      NonConstKeyPair v2(42, 1);
      assert(c.emplace(std::move(v2)).second == false);
    }
  }
  {
    // Testing C::emplace(const Key&, ConvertibleToMapped&&)
    Container c;
    const Key k(42);
    cc->expect<Key const&, int&&>();
    assert(c.emplace(k, 1).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const Key k2(42);
      assert(c.emplace(k2, 2).second == false);
    }
  }
  {
    // Testing C::emplace(Key&, Mapped&)
    Container c;
    Key k(42);
    Mapped m(1);
    cc->expect<Key&, Mapped&>();
    assert(c.emplace(k, m).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      Key k2(42);
      assert(c.emplace(k2, m).second == false);
    }
  }
  {
    // Testing C::emplace(Key&&, Mapped&&)
    Container c;
    Key k(42);
    Mapped m(1);
    cc->expect<Key&&, Mapped&&>();
    assert(c.emplace(std::move(k), std::move(m)).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      Key k2(42);
      Mapped m2(2);
      assert(c.emplace(std::move(k2), std::move(m2)).second == false);
    }
  }
  {
    // Testing C::emplace(ConvertibleToKey&&, ConvertibleToMapped&&)
    Container c;
    cc->expect<int&&, int&&>();
    assert(c.emplace(42, 1).second);
    assert(!cc->unchecked());
    {
      // test that emplacing a duplicate item allocates. We cannot optimize
      // this case because int&& does not match the type of key exactly.
      cc->expect<int&&, int&&>();
      assert(c.emplace(42, 1).second == false);
      assert(!cc->unchecked());
    }
  }
}

template <class Container>
void testMapEmplaceHint() {
  typedef typename Container::value_type ValueTp;
  typedef typename Container::key_type Key;
  typedef typename Container::mapped_type Mapped;
  typedef typename std::pair<Key, Mapped> NonConstKeyPair;
  typedef Container C;
  typedef typename C::iterator It;
  ConstructController* cc = getConstructController();
  cc->reset();
  {
    // Testing C::emplace_hint(p, const value_type&)
    Container c;
    const ValueTp v(42, 1);
    cc->expect<const ValueTp&>();
    It ret = c.emplace_hint(c.end(), v);
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42, 1);
      It ret2 = c.emplace_hint(c.begin(), v2);
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::emplace_hint(p, value_type&)
    Container c;
    ValueTp v(42, 1);
    cc->expect<ValueTp&>();
    It ret = c.emplace_hint(c.end(), v);
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42, 1);
      It ret2 = c.emplace_hint(c.begin(), v2);
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::emplace_hint(p, value_type&&)
    Container c;
    ValueTp v(42, 1);
    cc->expect<ValueTp&&>();
    It ret = c.emplace_hint(c.end(), std::move(v));
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42, 1);
      It ret2 = c.emplace_hint(c.begin(), std::move(v2));
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::emplace_hint(p, const value_type&&)
    Container c;
    const ValueTp v(42, 1);
    cc->expect<const ValueTp&&>();
    It ret = c.emplace_hint(c.end(), std::move(v));
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42, 1);
      It ret2 = c.emplace_hint(c.begin(), std::move(v2));
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::emplace_hint(p, pair<Key, Mapped> const&)
    Container c;
    const NonConstKeyPair v(42, 1);
    cc->expect<const NonConstKeyPair&>();
    It ret = c.emplace_hint(c.end(), v);
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const NonConstKeyPair v2(42, 1);
      It ret2 = c.emplace_hint(c.begin(), v2);
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::emplace_hint(p, pair<Key, Mapped>&&)
    Container c;
    NonConstKeyPair v(42, 1);
    cc->expect<NonConstKeyPair&&>();
    It ret = c.emplace_hint(c.end(), std::move(v));
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      NonConstKeyPair v2(42, 1);
      It ret2 = c.emplace_hint(c.begin(), std::move(v2));
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::emplace_hint(p, const Key&, ConvertibleToMapped&&)
    Container c;
    const Key k(42);
    cc->expect<Key const&, int&&>();
    It ret = c.emplace_hint(c.end(), k, 42);
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const Key k2(42);
      It ret2 = c.emplace_hint(c.begin(), k2, 1);
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::emplace_hint(p, Key&, Mapped&)
    Container c;
    Key k(42);
    Mapped m(1);
    cc->expect<Key&, Mapped&>();
    It ret = c.emplace_hint(c.end(), k, m);
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      Key k2(42);
      Mapped m2(2);
      It ret2 = c.emplace_hint(c.begin(), k2, m2);
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::emplace_hint(p, Key&&, Mapped&&)
    Container c;
    Key k(42);
    Mapped m(1);
    cc->expect<Key&&, Mapped&&>();
    It ret = c.emplace_hint(c.end(), std::move(k), std::move(m));
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      Key k2(42);
      Mapped m2(2);
      It ret2 = c.emplace_hint(c.begin(), std::move(k2), std::move(m2));
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::emplace_hint(p, ConvertibleToKey&&, ConvertibleToMapped&&)
    Container c;
    cc->expect<int&&, int&&>();
    It ret = c.emplace_hint(c.end(), 42, 1);
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      cc->expect<int&&, int&&>();
      It ret2 = c.emplace_hint(c.begin(), 42, 2);
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
      assert(!cc->unchecked());
    }
  }
}

template <class Container>
void testMultimapInsert() {
  typedef typename Container::value_type ValueTp;
  ConstructController* cc = getConstructController();
  cc->reset();
  {
    // Testing C::insert(const value_type&)
    Container c;
    const ValueTp v(42, 1);
    cc->expect<const ValueTp&>();
    c.insert(v);
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(value_type&)
    Container c;
    ValueTp v(42, 1);
    cc->expect<ValueTp&>();
    c.insert(v);
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(value_type&&)
    Container c;
    ValueTp v(42, 1);
    cc->expect<ValueTp&&>();
    c.insert(std::move(v));
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert({key, value})
    Container c;
    cc->expect<ValueTp&&>();
    c.insert({42, 1});
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(std::initializer_list<ValueTp>)
    Container c;
    std::initializer_list<ValueTp> il = {ValueTp(1, 1), ValueTp(2, 1)};
    cc->expect<ValueTp const&>(2);
    c.insert(il);
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(Iter, Iter) for *Iter = value_type const&
    Container c;
    const ValueTp ValueList[] = {ValueTp(1, 1), ValueTp(2, 1), ValueTp(3, 1)};
    cc->expect<ValueTp const&>(3);
    c.insert(std::begin(ValueList), std::end(ValueList));
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(Iter, Iter) for *Iter = value_type&&
    Container c;
    ValueTp ValueList[] = {ValueTp(1, 1), ValueTp(2, 1), ValueTp(3, 1)};
    cc->expect<ValueTp&&>(3);
    c.insert(std::move_iterator<ValueTp*>(std::begin(ValueList)), std::move_iterator<ValueTp*>(std::end(ValueList)));
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(Iter, Iter) for *Iter = value_type&
    Container c;
    ValueTp ValueList[] = {ValueTp(1, 1), ValueTp(2, 1), ValueTp(3, 1)};
    cc->expect<ValueTp&>(3);
    c.insert(std::begin(ValueList), std::end(ValueList));
    assert(!cc->unchecked());
  }
}

template <class Container>
void testMultimapInsertHint() {
  typedef typename Container::value_type ValueTp;
  ConstructController* cc = getConstructController();
  cc->reset();
  {
    // Testing C::insert(p, const value_type&)
    Container c;
    const ValueTp v(42, 1);
    cc->expect<const ValueTp&>();
    c.insert(c.begin(), v);
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(p, value_type&)
    Container c;
    ValueTp v(42, 1);
    cc->expect<ValueTp&>();
    c.insert(c.begin(), v);
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(p, value_type&&)
    Container c;
    ValueTp v(42, 1);
    cc->expect<ValueTp&&>();
    c.insert(c.begin(), std::move(v));
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(p, {key, value})
    Container c;
    cc->expect<ValueTp&&>();
    c.insert(c.begin(), {42, 1});
    assert(!cc->unchecked());
  }
}

#endif
