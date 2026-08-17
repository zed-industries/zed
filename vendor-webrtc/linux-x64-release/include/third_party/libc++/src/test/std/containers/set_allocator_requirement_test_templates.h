//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SET_ALLOCATOR_REQUIREMENT_TEST_TEMPLATES_H
#define SET_ALLOCATOR_REQUIREMENT_TEST_TEMPLATES_H

// <set>
// <unordered_set>

// class set
// class unordered_set

// insert(...);
// emplace(...);
// emplace_hint(...);

#include <cassert>
#include <iterator>

#include "test_macros.h"
#include "count_new.h"
#include "container_test_types.h"

template <class Container>
void testSetInsert() {
  typedef typename Container::value_type ValueTp;
  ConstructController* cc = getConstructController();
  cc->reset();
  {
    // Testing C::insert(const value_type&)"
    Container c;
    const ValueTp v(42);
    cc->expect<const ValueTp&>();
    assert(c.insert(v).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42);
      assert(c.insert(v2).second == false);
    }
  }
  {
    // Testing C::insert(value_type&)"
    Container c;
    ValueTp v(42);
    cc->expect<const ValueTp&>();
    assert(c.insert(v).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42);
      assert(c.insert(v2).second == false);
    }
  }
  {
    // Testing C::insert(value_type&&)"
    Container c;
    ValueTp v(42);
    cc->expect<ValueTp&&>();
    assert(c.insert(std::move(v)).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42);
      assert(c.insert(std::move(v2)).second == false);
    }
  }
  {
    // Testing C::insert(const value_type&&)"
    Container c;
    const ValueTp v(42);
    cc->expect<const ValueTp&>();
    assert(c.insert(std::move(v)).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42);
      assert(c.insert(std::move(v2)).second == false);
    }
  }
  {
    // Testing C::insert(std::initializer_list<ValueTp>)"
    Container c;
    std::initializer_list<ValueTp> il = {ValueTp(1), ValueTp(2)};
    cc->expect<ValueTp const&>(2);
    c.insert(il);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      c.insert(il);
    }
  }
  {
    // Testing C::insert(Iter, Iter) for *Iter = value_type const&"
    Container c;
    const ValueTp ValueList[] = {ValueTp(1), ValueTp(2), ValueTp(3)};
    cc->expect<ValueTp const&>(3);
    c.insert(std::begin(ValueList), std::end(ValueList));
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      c.insert(std::begin(ValueList), std::end(ValueList));
    }
  }
  {
    // Testing C::insert(Iter, Iter) for *Iter = value_type&&"
    Container c;
    ValueTp ValueList[] = {ValueTp(1), ValueTp(2), ValueTp(3)};
    cc->expect<ValueTp&&>(3);
    c.insert(std::move_iterator<ValueTp*>(std::begin(ValueList)), std::move_iterator<ValueTp*>(std::end(ValueList)));
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp ValueList2[] = {ValueTp(1), ValueTp(2), ValueTp(3)};
      c.insert(std::move_iterator<ValueTp*>(std::begin(ValueList2)),
               std::move_iterator<ValueTp*>(std::end(ValueList2)));
    }
  }
  {
    // Testing C::insert(Iter, Iter) for *Iter = value_type&"
    Container c;
    ValueTp ValueList[] = {ValueTp(1), ValueTp(2), ValueTp(3)};
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
void testSetEmplace() {
  typedef typename Container::value_type ValueTp;
  ConstructController* cc = getConstructController();
  cc->reset();
  {
    // Testing C::emplace(const value_type&)"
    Container c;
    const ValueTp v(42);
    cc->expect<const ValueTp&>();
    assert(c.emplace(v).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42);
      assert(c.emplace(v2).second == false);
    }
  }
  {
    // Testing C::emplace(value_type&)"
    Container c;
    ValueTp v(42);
    cc->expect<ValueTp&>();
    assert(c.emplace(v).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42);
      assert(c.emplace(v2).second == false);
    }
  }
  {
    // Testing C::emplace(value_type&&)"
    Container c;
    ValueTp v(42);
    cc->expect<ValueTp&&>();
    assert(c.emplace(std::move(v)).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42);
      assert(c.emplace(std::move(v2)).second == false);
    }
  }
  {
    // Testing C::emplace(const value_type&&)"
    Container c;
    const ValueTp v(42);
    cc->expect<const ValueTp&&>();
    assert(c.emplace(std::move(v)).second);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42);
      assert(c.emplace(std::move(v2)).second == false);
    }
  }
}

template <class Container>
void testSetEmplaceHint() {
  typedef typename Container::value_type ValueTp;
  typedef Container C;
  typedef typename C::iterator It;
  ConstructController* cc = getConstructController();
  cc->reset();
  {
    // Testing C::emplace_hint(p, const value_type&)"
    Container c;
    const ValueTp v(42);
    cc->expect<const ValueTp&>();
    It ret = c.emplace_hint(c.end(), v);
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42);
      It ret2 = c.emplace_hint(c.begin(), v2);
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::emplace_hint(p, value_type&)"
    Container c;
    ValueTp v(42);
    cc->expect<ValueTp&>();
    It ret = c.emplace_hint(c.end(), v);
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42);
      It ret2 = c.emplace_hint(c.begin(), v2);
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::emplace_hint(p, value_type&&)"
    Container c;
    ValueTp v(42);
    cc->expect<ValueTp&&>();
    It ret = c.emplace_hint(c.end(), std::move(v));
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      ValueTp v2(42);
      It ret2 = c.emplace_hint(c.begin(), std::move(v2));
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
  {
    // Testing C::emplace_hint(p, const value_type&&)"
    Container c;
    const ValueTp v(42);
    cc->expect<const ValueTp&&>();
    It ret = c.emplace_hint(c.end(), std::move(v));
    assert(ret != c.end());
    assert(c.size() == 1);
    assert(!cc->unchecked());
    {
      DisableAllocationGuard g;
      const ValueTp v2(42);
      It ret2 = c.emplace_hint(c.begin(), std::move(v2));
      assert(&(*ret2) == &(*ret));
      assert(c.size() == 1);
    }
  }
}

template <class Container>
void testMultisetInsert() {
  typedef typename Container::value_type ValueTp;
  ConstructController* cc = getConstructController();
  cc->reset();
  {
    // Testing C::insert(const value_type&)"
    Container c;
    const ValueTp v(42);
    cc->expect<const ValueTp&>();
    c.insert(v);
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(value_type&)"
    Container c;
    ValueTp v(42);
    cc->expect<const ValueTp&>();
    c.insert(v);
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(value_type&&)"
    Container c;
    ValueTp v(42);
    cc->expect<ValueTp&&>();
    c.insert(std::move(v));
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(std::initializer_list<ValueTp>)"
    Container c;
    std::initializer_list<ValueTp> il = {ValueTp(1), ValueTp(2)};
    cc->expect<ValueTp const&>(2);
    c.insert(il);
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(Iter, Iter) for *Iter = value_type const&"
    Container c;
    const ValueTp ValueList[] = {ValueTp(1), ValueTp(2), ValueTp(3)};
    cc->expect<ValueTp const&>(3);
    c.insert(std::begin(ValueList), std::end(ValueList));
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(Iter, Iter) for *Iter = value_type&&"
    Container c;
    ValueTp ValueList[] = {ValueTp(1), ValueTp(2), ValueTp(3)};
    cc->expect<ValueTp&&>(3);
    c.insert(std::move_iterator<ValueTp*>(std::begin(ValueList)), std::move_iterator<ValueTp*>(std::end(ValueList)));
    assert(!cc->unchecked());
  }
  {
    // Testing C::insert(Iter, Iter) for *Iter = value_type&"
    Container c;
    ValueTp ValueList[] = {ValueTp(1), ValueTp(2), ValueTp(1)};
    cc->expect<ValueTp&>(3);
    c.insert(std::begin(ValueList), std::end(ValueList));
    assert(!cc->unchecked());
  }
}

template <class Container>
void testMultisetEmplace() {
  typedef typename Container::value_type ValueTp;
  ConstructController* cc = getConstructController();
  cc->reset();
  {
    // Testing C::emplace(const value_type&)"
    Container c;
    const ValueTp v(42);
    cc->expect<const ValueTp&>();
    c.emplace(v);
    assert(!cc->unchecked());
  }
  {
    // Testing C::emplace(value_type&)"
    Container c;
    ValueTp v(42);
    cc->expect<ValueTp&>();
    c.emplace(v);
    assert(!cc->unchecked());
  }
  {
    // Testing C::emplace(value_type&&)"
    Container c;
    ValueTp v(42);
    cc->expect<ValueTp&&>();
    c.emplace(std::move(v));
    assert(!cc->unchecked());
  }
  {
    // Testing C::emplace(const value_type&&)"
    Container c;
    const ValueTp v(42);
    cc->expect<const ValueTp&&>();
    c.emplace(std::move(v));
    assert(!cc->unchecked());
  }
}

#endif
