//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_UNIQUE_PTR_TEST_HELPER_H
#define TEST_SUPPORT_UNIQUE_PTR_TEST_HELPER_H

#include <memory>
#include <type_traits>

#include "test_macros.h"
#include "deleter_types.h"

struct A {
  static int count;
  TEST_CONSTEXPR_CXX23 A() {
    if (!TEST_IS_CONSTANT_EVALUATED)
      ++count;
  }
  TEST_CONSTEXPR_CXX23 A(const A&) {
    if (!TEST_IS_CONSTANT_EVALUATED)
      ++count;
  }
  TEST_CONSTEXPR_CXX23 virtual ~A() {
    if (!TEST_IS_CONSTANT_EVALUATED)
      --count;
  }
};

int A::count = 0;

struct B : public A {
  static int count;
  TEST_CONSTEXPR_CXX23 B() {
    if (!TEST_IS_CONSTANT_EVALUATED)
      ++count;
  }
  TEST_CONSTEXPR_CXX23 B(const B& other) : A(other) {
    if (!TEST_IS_CONSTANT_EVALUATED)
      ++count;
  }
  TEST_CONSTEXPR_CXX23 virtual ~B() {
    if (!TEST_IS_CONSTANT_EVALUATED)
      --count;
  }
};

int B::count = 0;

template <class T>
TEST_CONSTEXPR_CXX23 typename std::enable_if<!std::is_array<T>::value, T*>::type newValue(int num_elements) {
  assert(num_elements == 1);
  return new T;
}

template <class T>
TEST_CONSTEXPR_CXX23 typename std::enable_if<std::is_array<T>::value, typename std::remove_all_extents<T>::type*>::type
newValue(int num_elements) {
  typedef typename std::remove_all_extents<T>::type VT;
  assert(num_elements >= 1);
  return new VT[num_elements];
}

struct IncompleteType;

void checkNumIncompleteTypeAlive(int i);
int getNumIncompleteTypeAlive();
IncompleteType* getNewIncomplete();
IncompleteType* getNewIncompleteArray(int size);

#if TEST_STD_VER >= 11
template <class ThisT, class... Args>
struct args_is_this_type : std::false_type {};

template <class ThisT, class A1>
struct args_is_this_type<ThisT, A1> : std::is_same<ThisT, typename std::decay<A1>::type> {};
#endif

template <class IncompleteT = IncompleteType, class Del = std::default_delete<IncompleteT> >
struct StoresIncomplete {
  static_assert(
      (std::is_same<IncompleteT, IncompleteType>::value || std::is_same<IncompleteT, IncompleteType[]>::value), "");

  std::unique_ptr<IncompleteT, Del> m_ptr;

#if TEST_STD_VER >= 11
  StoresIncomplete(StoresIncomplete const&) = delete;
  StoresIncomplete(StoresIncomplete&&)      = default;

  template <class... Args>
  StoresIncomplete(Args&&... args) : m_ptr(std::forward<Args>(args)...) {
    static_assert(!args_is_this_type<StoresIncomplete, Args...>::value, "");
  }
#else

private:
  StoresIncomplete();
  StoresIncomplete(StoresIncomplete const&);

public:
#endif

  ~StoresIncomplete();

  IncompleteType* get() const { return m_ptr.get(); }
  Del& get_deleter() { return m_ptr.get_deleter(); }
};

#if TEST_STD_VER >= 11
template <class IncompleteT = IncompleteType, class Del = std::default_delete<IncompleteT>, class... Args>
void doIncompleteTypeTest(int expect_alive, Args&&... ctor_args) {
  checkNumIncompleteTypeAlive(expect_alive);
  {
    StoresIncomplete<IncompleteT, Del> sptr(std::forward<Args>(ctor_args)...);
    checkNumIncompleteTypeAlive(expect_alive);
    if (expect_alive == 0)
      assert(sptr.get() == nullptr);
    else
      assert(sptr.get() != nullptr);
  }
  checkNumIncompleteTypeAlive(0);
}
#endif

#define INCOMPLETE_TEST_EPILOGUE()                                                                                     \
  int is_incomplete_test_anchor = is_incomplete_test();                                                                \
                                                                                                                       \
  struct IncompleteType {                                                                                              \
    static int count;                                                                                                  \
    IncompleteType() { ++count; }                                                                                      \
    ~IncompleteType() { --count; }                                                                                     \
  };                                                                                                                   \
                                                                                                                       \
  int IncompleteType::count = 0;                                                                                       \
                                                                                                                       \
  void checkNumIncompleteTypeAlive(int i) { assert(IncompleteType::count == i); }                                      \
  int getNumIncompleteTypeAlive() { return IncompleteType::count; }                                                    \
  IncompleteType* getNewIncomplete() { return new IncompleteType; }                                                    \
  IncompleteType* getNewIncompleteArray(int size) { return new IncompleteType[size]; }                                 \
                                                                                                                       \
  template <class IncompleteT, class Del>                                                                              \
  StoresIncomplete<IncompleteT, Del>::~StoresIncomplete() {}
#

#if TEST_STD_VER >= 11
#  define DEFINE_AND_RUN_IS_INCOMPLETE_TEST(...)                                                                       \
    static int is_incomplete_test() { __VA_ARGS__ return 0; }                                                          \
    INCOMPLETE_TEST_EPILOGUE()
#else
#  define DEFINE_AND_RUN_IS_INCOMPLETE_TEST(...)                                                                       \
    static int is_incomplete_test() { return 0; }                                                                      \
    INCOMPLETE_TEST_EPILOGUE()
#endif

#endif // TEST_SUPPORT_UNIQUE_PTR_TEST_HELPER_H
