//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef ANY_HELPERS_H
#define ANY_HELPERS_H

#include <typeinfo>
#include <type_traits>
#include <cassert>

namespace std { namespace experimental {} }

#include "test_macros.h"
#include "type_id.h"

#if !defined(TEST_HAS_NO_RTTI)
#define RTTI_ASSERT(X) assert(X)
#else
#define RTTI_ASSERT(X)
#endif

template <class T>
  struct IsSmallObject
    : public std::integral_constant<bool
        , sizeof(T) <= sizeof(std::any) - sizeof(void*)
          && std::alignment_of<void*>::value
             % std::alignment_of<T>::value == 0
          && std::is_nothrow_move_constructible<T>::value
        >
  {};

template <class T>
bool containsType(std::any const& a) {
#if !defined(TEST_HAS_NO_RTTI)
    return a.type() == typeid(T);
#else
    return a.has_value() && std::any_cast<T>(&a) != nullptr;
#endif
}

// Return 'true' if 'Type' will be considered a small type by 'any'
template <class Type>
bool isSmallType() {
    return IsSmallObject<Type>::value;
}

// Assert that an object is empty. If the object used to contain an object
// of type 'LastType' check that it can no longer be accessed.
template <class LastType = int>
void assertEmpty(std::any const& a) {
    assert(!a.has_value());
    RTTI_ASSERT(a.type() == typeid(void));
    assert(std::any_cast<LastType const>(&a) == nullptr);
}

template <class Type>
constexpr auto has_value_member(int) -> decltype(std::declval<Type&>().value, true)
{ return true; }
template <class> constexpr bool has_value_member(long) { return false; }


// Assert that an 'any' object stores the specified 'Type' and 'value'.
template <class Type>
std::enable_if_t<has_value_member<Type>(0)>
_LIBCPP_AVAILABILITY_THROW_BAD_ANY_CAST
assertContains(std::any const& a, int value) {
    assert(a.has_value());
    assert(containsType<Type>(a));
    assert(std::any_cast<Type const &>(a).value == value);
}

template <class Type, class Value>
std::enable_if_t<!has_value_member<Type>(0)>
_LIBCPP_AVAILABILITY_THROW_BAD_ANY_CAST
assertContains(std::any const& a, Value value) {
    assert(a.has_value());
    assert(containsType<Type>(a));
    assert(std::any_cast<Type const &>(a) == value);
}


// Modify the value of a "test type" stored within an any to the specified
// 'value'.
template <class Type>
_LIBCPP_AVAILABILITY_THROW_BAD_ANY_CAST
void modifyValue(std::any& a, int value) {
    assert(a.has_value());
    assert(containsType<Type>(a));
    std::any_cast<Type&>(a).value = value;
}

// A test type that will trigger the small object optimization within 'any'.
template <int Dummy = 0>
struct small_type
{
    static int count;
    static int copied;
    static int moved;
    static int const_copied;
    static int non_const_copied;

    static void reset() {
        small_type::copied = 0;
        small_type::moved = 0;
        small_type::const_copied = 0;
        small_type::non_const_copied = 0;
    }

    int value;

    explicit small_type(int val = 0) : value(val) {
        ++count;
    }
    explicit small_type(int, int val, int) : value(val) {
        ++count;
    }
    small_type(std::initializer_list<int> il) : value(*il.begin()) {
        ++count;
    }

    small_type(small_type const & other) noexcept {
        value = other.value;
        ++count;
        ++copied;
        ++const_copied;
    }

    small_type(small_type& other) noexcept {
        value = other.value;
        ++count;
        ++copied;
        ++non_const_copied;
    }

    small_type(small_type && other) noexcept {
        value = other.value;
        other.value = 0;
        ++count;
        ++moved;
    }

    ~small_type() {
        value = -1;
        --count;
    }

private:
    small_type& operator=(small_type const&) = delete;
    small_type& operator=(small_type&&) = delete;
};

template <int Dummy>
int small_type<Dummy>::count = 0;

template <int Dummy>
int small_type<Dummy>::copied = 0;

template <int Dummy>
int small_type<Dummy>::moved = 0;

template <int Dummy>
int small_type<Dummy>::const_copied = 0;

template <int Dummy>
int small_type<Dummy>::non_const_copied = 0;

typedef small_type<> small;
typedef small_type<1> small1;
typedef small_type<2> small2;


// A test type that will NOT trigger the small object optimization in any.
template <int Dummy = 0>
struct large_type
{
    static int count;
    static int copied;
    static int moved;
    static int const_copied;
    static int non_const_copied;

    static void reset() {
        large_type::copied = 0;
        large_type::moved  = 0;
        large_type::const_copied = 0;
        large_type::non_const_copied = 0;
    }

    int value;

    large_type(int val = 0) : value(val) {
        ++count;
        data[0] = 0;
    }
    large_type(int, int val, int) : value(val) {
        ++count;
        data[0] = 0;
    }
    large_type(std::initializer_list<int> il) : value(*il.begin()) {
        ++count;
    }
    large_type(large_type const & other) {
        value = other.value;
        ++count;
        ++copied;
        ++const_copied;
    }

    large_type(large_type & other) {
        value = other.value;
        ++count;
        ++copied;
        ++non_const_copied;
    }

    large_type(large_type && other) {
        value = other.value;
        other.value = 0;
        ++count;
        ++moved;
    }

    ~large_type()  {
        value = 0;
        --count;
    }

private:
    large_type& operator=(large_type const&) = delete;
    large_type& operator=(large_type &&) = delete;
    int data[10];
};

template <int Dummy>
int large_type<Dummy>::count = 0;

template <int Dummy>
int large_type<Dummy>::copied = 0;

template <int Dummy>
int large_type<Dummy>::moved = 0;

template <int Dummy>
int large_type<Dummy>::const_copied = 0;

template <int Dummy>
int large_type<Dummy>::non_const_copied = 0;

typedef large_type<> large;
typedef large_type<1> large1;
typedef large_type<2> large2;

// The exception type thrown by 'small_throws_on_copy', 'large_throws_on_copy'
// and 'throws_on_move'.
struct my_any_exception {};

void throwMyAnyExpression() {
#if !defined(TEST_HAS_NO_EXCEPTIONS)
        throw my_any_exception();
#else
        assert(false && "Exceptions are disabled");
#endif
}

// A test type that will trigger the small object optimization within 'any'.
// this type throws if it is copied.
struct small_throws_on_copy
{
    static int count;
    static int copied;
    static int moved;
    static void reset() { count = copied = moved = 0; }
    int value;

    explicit small_throws_on_copy(int val = 0) : value(val) {
        ++count;
    }
    explicit small_throws_on_copy(int, int val, int) : value(val) {
        ++count;
    }
    small_throws_on_copy(small_throws_on_copy const &) {
        throwMyAnyExpression();
    }

    small_throws_on_copy(small_throws_on_copy && other) throw() {
        value = other.value;
        ++count; ++moved;
    }

    ~small_throws_on_copy() {
        --count;
    }
private:
    small_throws_on_copy& operator=(small_throws_on_copy const&) = delete;
    small_throws_on_copy& operator=(small_throws_on_copy &&) = delete;
};

int small_throws_on_copy::count = 0;
int small_throws_on_copy::copied = 0;
int small_throws_on_copy::moved = 0;


// A test type that will NOT trigger the small object optimization within 'any'.
// this type throws if it is copied.
struct large_throws_on_copy
{
    static int count;
    static int copied;
    static int moved;
    static void reset() { count = copied = moved = 0; }
    int value = 0;

    explicit large_throws_on_copy(int val = 0) : value(val) {
        data[0] = 0;
        ++count;
    }
    explicit large_throws_on_copy(int, int val, int) : value(val) {
        data[0] = 0;
        ++count;
    }
    large_throws_on_copy(large_throws_on_copy const &) {
         throwMyAnyExpression();
    }

    large_throws_on_copy(large_throws_on_copy && other) throw() {
        value = other.value;
        ++count; ++moved;
    }

    ~large_throws_on_copy() {
        --count;
    }

private:
    large_throws_on_copy& operator=(large_throws_on_copy const&) = delete;
    large_throws_on_copy& operator=(large_throws_on_copy &&) = delete;
    int data[10];
};

int large_throws_on_copy::count = 0;
int large_throws_on_copy::copied = 0;
int large_throws_on_copy::moved = 0;

// A test type that throws when it is moved. This object will NOT trigger
// the small object optimization in 'any'.
struct throws_on_move
{
    static int count;
    static int copied;
    static int moved;
    static void reset() { count = copied = moved = 0; }
    int value;

    explicit throws_on_move(int val = 0) : value(val) { ++count; }
    explicit throws_on_move(int, int val, int) : value(val) { ++count; }
    throws_on_move(throws_on_move const & other) {
        value = other.value;
        ++count; ++copied;
    }

    throws_on_move(throws_on_move &&) {
        throwMyAnyExpression();
    }

    ~throws_on_move() {
        --count;
    }
private:
    throws_on_move& operator=(throws_on_move const&) = delete;
    throws_on_move& operator=(throws_on_move &&) = delete;
};

int throws_on_move::count = 0;
int throws_on_move::copied = 0;
int throws_on_move::moved = 0;

struct small_tracked_t {
  small_tracked_t()
      : arg_types(&makeArgumentID<>()) {}
  small_tracked_t(small_tracked_t const&) noexcept
      : arg_types(&makeArgumentID<small_tracked_t const&>()) {}
  small_tracked_t(small_tracked_t &&) noexcept
      : arg_types(&makeArgumentID<small_tracked_t &&>()) {}
  template <class ...Args>
  explicit small_tracked_t(Args&&...)
      : arg_types(&makeArgumentID<Args...>()) {}
  template <class ...Args>
  explicit small_tracked_t(std::initializer_list<int>, Args&&...)
      : arg_types(&makeArgumentID<std::initializer_list<int>, Args...>()) {}

  TypeID const* arg_types;
};
static_assert(IsSmallObject<small_tracked_t>::value, "must be small");

struct large_tracked_t {
  large_tracked_t()
      : arg_types(&makeArgumentID<>()) { dummy[0] = 42; }
  large_tracked_t(large_tracked_t const&) noexcept
      : arg_types(&makeArgumentID<large_tracked_t const&>()) {}
  large_tracked_t(large_tracked_t &&) noexcept
      : arg_types(&makeArgumentID<large_tracked_t &&>()) {}
  template <class ...Args>
  explicit large_tracked_t(Args&&...)
      : arg_types(&makeArgumentID<Args...>()) {}
  template <class ...Args>
  explicit large_tracked_t(std::initializer_list<int>, Args&&...)
      : arg_types(&makeArgumentID<std::initializer_list<int>, Args...>()) {}

  TypeID const* arg_types;
  int dummy[sizeof(std::any) / sizeof(int) + 1];
};

static_assert(!IsSmallObject<large_tracked_t>::value, "must not be small");


template <class Type, class ...Args>
void assertArgsMatch(std::any const& a) {
    assert(a.has_value());
    assert(containsType<Type>(a));
    assert(std::any_cast<Type const &>(a).arg_types == &makeArgumentID<Args...>());
};


#endif
