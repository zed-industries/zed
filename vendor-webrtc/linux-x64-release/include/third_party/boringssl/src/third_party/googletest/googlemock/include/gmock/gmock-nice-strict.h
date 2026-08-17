// Copyright 2008, Google Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google Inc. nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// Implements class templates NiceMock, NaggyMock, and StrictMock.
//
// Given a mock class MockFoo that is created using Google Mock,
// NiceMock<MockFoo> is a subclass of MockFoo that allows
// uninteresting calls (i.e. calls to mock methods that have no
// EXPECT_CALL specs), NaggyMock<MockFoo> is a subclass of MockFoo
// that prints a warning when an uninteresting call occurs, and
// StrictMock<MockFoo> is a subclass of MockFoo that treats all
// uninteresting calls as errors.
//
// Currently a mock is naggy by default, so MockFoo and
// NaggyMock<MockFoo> behave like the same.  However, we will soon
// switch the default behavior of mocks to be nice, as that in general
// leads to more maintainable tests.  When that happens, MockFoo will
// stop behaving like NaggyMock<MockFoo> and start behaving like
// NiceMock<MockFoo>.
//
// NiceMock, NaggyMock, and StrictMock "inherit" the constructors of
// their respective base class.  Therefore you can write
// NiceMock<MockFoo>(5, "a") to construct a nice mock where MockFoo
// has a constructor that accepts (int, const char*), for example.
//
// A known limitation is that NiceMock<MockFoo>, NaggyMock<MockFoo>,
// and StrictMock<MockFoo> only works for mock methods defined using
// the MOCK_METHOD* family of macros DIRECTLY in the MockFoo class.
// If a mock method is defined in a base class of MockFoo, the "nice"
// or "strict" modifier may not affect it, depending on the compiler.
// In particular, nesting NiceMock, NaggyMock, and StrictMock is NOT
// supported.

// IWYU pragma: private, include "gmock/gmock.h"
// IWYU pragma: friend gmock/.*

#ifndef GOOGLEMOCK_INCLUDE_GMOCK_GMOCK_NICE_STRICT_H_
#define GOOGLEMOCK_INCLUDE_GMOCK_GMOCK_NICE_STRICT_H_

#include <cstdint>
#include <type_traits>

#include "gmock/gmock-spec-builders.h"
#include "gmock/internal/gmock-port.h"

namespace testing {
template <class MockClass>
class NiceMock;
template <class MockClass>
class NaggyMock;
template <class MockClass>
class StrictMock;

namespace internal {
template <typename T>
std::true_type StrictnessModifierProbe(const NiceMock<T>&);
template <typename T>
std::true_type StrictnessModifierProbe(const NaggyMock<T>&);
template <typename T>
std::true_type StrictnessModifierProbe(const StrictMock<T>&);
std::false_type StrictnessModifierProbe(...);

template <typename T>
constexpr bool HasStrictnessModifier() {
  return decltype(StrictnessModifierProbe(std::declval<const T&>()))::value;
}

// Base classes that register and deregister with testing::Mock to alter the
// default behavior around uninteresting calls. Inheriting from one of these
// classes first and then MockClass ensures the MockClass constructor is run
// after registration, and that the MockClass destructor runs before
// deregistration. This guarantees that MockClass's constructor and destructor
// run with the same level of strictness as its instance methods.

#if defined(GTEST_OS_WINDOWS) && !defined(GTEST_OS_WINDOWS_MINGW) && \
    (defined(_MSC_VER) || defined(__clang__))
// We need to mark these classes with this declspec to ensure that
// the empty base class optimization is performed.
#define GTEST_INTERNAL_EMPTY_BASE_CLASS __declspec(empty_bases)
#else
#define GTEST_INTERNAL_EMPTY_BASE_CLASS
#endif

template <typename Base>
class NiceMockImpl {
 public:
  NiceMockImpl() {
    ::testing::Mock::AllowUninterestingCalls(reinterpret_cast<uintptr_t>(this));
  }

  ~NiceMockImpl() {
    ::testing::Mock::UnregisterCallReaction(reinterpret_cast<uintptr_t>(this));
  }
};

template <typename Base>
class NaggyMockImpl {
 public:
  NaggyMockImpl() {
    ::testing::Mock::WarnUninterestingCalls(reinterpret_cast<uintptr_t>(this));
  }

  ~NaggyMockImpl() {
    ::testing::Mock::UnregisterCallReaction(reinterpret_cast<uintptr_t>(this));
  }
};

template <typename Base>
class StrictMockImpl {
 public:
  StrictMockImpl() {
    ::testing::Mock::FailUninterestingCalls(reinterpret_cast<uintptr_t>(this));
  }

  ~StrictMockImpl() {
    ::testing::Mock::UnregisterCallReaction(reinterpret_cast<uintptr_t>(this));
  }
};

}  // namespace internal

template <class MockClass>
class GTEST_INTERNAL_EMPTY_BASE_CLASS NiceMock
    : private internal::NiceMockImpl<MockClass>,
      public MockClass {
 public:
  static_assert(!internal::HasStrictnessModifier<MockClass>(),
                "Can't apply NiceMock to a class hierarchy that already has a "
                "strictness modifier. See "
                "https://google.github.io/googletest/"
                "gmock_cook_book.html#NiceStrictNaggy");
  NiceMock() : MockClass() {
    static_assert(sizeof(*this) == sizeof(MockClass),
                  "The impl subclass shouldn't introduce any padding");
  }

  // Ideally, we would inherit base class's constructors through a using
  // declaration, which would preserve their visibility. However, many existing
  // tests rely on the fact that current implementation reexports protected
  // constructors as public. These tests would need to be cleaned up first.

  // Single argument constructor is special-cased so that it can be
  // made explicit.
  template <typename A>
  explicit NiceMock(A&& arg) : MockClass(std::forward<A>(arg)) {
    static_assert(sizeof(*this) == sizeof(MockClass),
                  "The impl subclass shouldn't introduce any padding");
  }

  template <typename TArg1, typename TArg2, typename... An>
  NiceMock(TArg1&& arg1, TArg2&& arg2, An&&... args)
      : MockClass(std::forward<TArg1>(arg1), std::forward<TArg2>(arg2),
                  std::forward<An>(args)...) {
    static_assert(sizeof(*this) == sizeof(MockClass),
                  "The impl subclass shouldn't introduce any padding");
  }

 private:
  NiceMock(const NiceMock&) = delete;
  NiceMock& operator=(const NiceMock&) = delete;
};

template <class MockClass>
class GTEST_INTERNAL_EMPTY_BASE_CLASS NaggyMock
    : private internal::NaggyMockImpl<MockClass>,
      public MockClass {
  static_assert(!internal::HasStrictnessModifier<MockClass>(),
                "Can't apply NaggyMock to a class hierarchy that already has a "
                "strictness modifier. See "
                "https://google.github.io/googletest/"
                "gmock_cook_book.html#NiceStrictNaggy");

 public:
  NaggyMock() : MockClass() {
    static_assert(sizeof(*this) == sizeof(MockClass),
                  "The impl subclass shouldn't introduce any padding");
  }

  // Ideally, we would inherit base class's constructors through a using
  // declaration, which would preserve their visibility. However, many existing
  // tests rely on the fact that current implementation reexports protected
  // constructors as public. These tests would need to be cleaned up first.

  // Single argument constructor is special-cased so that it can be
  // made explicit.
  template <typename A>
  explicit NaggyMock(A&& arg) : MockClass(std::forward<A>(arg)) {
    static_assert(sizeof(*this) == sizeof(MockClass),
                  "The impl subclass shouldn't introduce any padding");
  }

  template <typename TArg1, typename TArg2, typename... An>
  NaggyMock(TArg1&& arg1, TArg2&& arg2, An&&... args)
      : MockClass(std::forward<TArg1>(arg1), std::forward<TArg2>(arg2),
                  std::forward<An>(args)...) {
    static_assert(sizeof(*this) == sizeof(MockClass),
                  "The impl subclass shouldn't introduce any padding");
  }

 private:
  NaggyMock(const NaggyMock&) = delete;
  NaggyMock& operator=(const NaggyMock&) = delete;
};

template <class MockClass>
class GTEST_INTERNAL_EMPTY_BASE_CLASS StrictMock
    : private internal::StrictMockImpl<MockClass>,
      public MockClass {
 public:
  static_assert(
      !internal::HasStrictnessModifier<MockClass>(),
      "Can't apply StrictMock to a class hierarchy that already has a "
      "strictness modifier. See "
      "https://google.github.io/googletest/"
      "gmock_cook_book.html#NiceStrictNaggy");
  StrictMock() : MockClass() {
    static_assert(sizeof(*this) == sizeof(MockClass),
                  "The impl subclass shouldn't introduce any padding");
  }

  // Ideally, we would inherit base class's constructors through a using
  // declaration, which would preserve their visibility. However, many existing
  // tests rely on the fact that current implementation reexports protected
  // constructors as public. These tests would need to be cleaned up first.

  // Single argument constructor is special-cased so that it can be
  // made explicit.
  template <typename A>
  explicit StrictMock(A&& arg) : MockClass(std::forward<A>(arg)) {
    static_assert(sizeof(*this) == sizeof(MockClass),
                  "The impl subclass shouldn't introduce any padding");
  }

  template <typename TArg1, typename TArg2, typename... An>
  StrictMock(TArg1&& arg1, TArg2&& arg2, An&&... args)
      : MockClass(std::forward<TArg1>(arg1), std::forward<TArg2>(arg2),
                  std::forward<An>(args)...) {
    static_assert(sizeof(*this) == sizeof(MockClass),
                  "The impl subclass shouldn't introduce any padding");
  }

 private:
  StrictMock(const StrictMock&) = delete;
  StrictMock& operator=(const StrictMock&) = delete;
};

#undef GTEST_INTERNAL_EMPTY_BASE_CLASS

}  // namespace testing

#endif  // GOOGLEMOCK_INCLUDE_GMOCK_GMOCK_NICE_STRICT_H_
