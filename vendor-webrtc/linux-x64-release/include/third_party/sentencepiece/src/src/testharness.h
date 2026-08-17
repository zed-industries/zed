// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.!

#ifndef TESTHARNESS_H_
#define TESTHARNESS_H_

#include <cmath>
#include <iostream>
#include <sstream>
#include <string>

#include "absl/flags/flag.h"
#include "absl/flags/parse.h"
#include "absl/strings/string_view.h"
#include "common.h"

ABSL_DECLARE_FLAG(std::string, test_tmpdir);
ABSL_DECLARE_FLAG(std::string, test_srcdir);

namespace sentencepiece {
namespace test {
// Run some of the tests registered by the TEST() macro.
// TEST(Foo, Hello) { ... }
// TEST(Foo, World) { ... }
//
// Returns 0 if all tests pass.
// Dies or returns a non-zero value if some test fails.
int RunAllTests();

// An instance of Tester is allocated to hold temporary state during
// the execution of an assertion.
class Tester {
 public:
  Tester(const char* fname, int line) : ok_(true), fname_(fname), line_(line) {}

  ~Tester() {
    if (!ok_) {
      std::cerr << "[       NG ] " << fname_ << ":" << line_ << ":" << ss_.str()
                << std::endl;
      exit(-1);
    }
  }

  Tester& Is(bool b, const char* msg) {
    if (!b) {
      ss_ << " failed: " << msg;
      ok_ = false;
    }
    return *this;
  }

  Tester& IsNear(double val1,
                 double val2,
                 double abs_error,
                 const char* msg1,
                 const char* msg2) {
    const double diff = std::fabs(val1 - val2);
    if (diff > abs_error) {
      ss_ << "The difference between (" << msg1 << ") and (" << msg2 << ") is "
          << diff << ", which exceeds " << abs_error << ", where\n"
          << msg1 << " evaluates to " << val1 << ",\n"
          << msg2 << " evaluates to " << val2;
      ok_ = false;
    }
    return *this;
  }

#define BINARY_OP(name, op)                                                  \
  template <class X, class Y>                                                \
  Tester& name(const X& x, const Y& y, const char* msg1, const char* msg2) { \
    if (!(x op y)) {                                                         \
      ss_ << " failed: " << msg1 << (" " #op " ") << msg2;                   \
      ok_ = false;                                                           \
    }                                                                        \
    return *this;                                                            \
  }

  BINARY_OP(IsEq, ==)
  BINARY_OP(IsNe, !=)
  BINARY_OP(IsGe, >=)
  BINARY_OP(IsGt, >)
  BINARY_OP(IsLe, <=)
  BINARY_OP(IsLt, <)
#undef BINARY_OP

  // Attach the specified value to the error message if an error has occurred
  template <class V>
  Tester& operator<<(const V& value) {
    if (!ok_) {
      ss_ << " " << value;
    }
    return *this;
  }

 private:
  bool ok_;
  const char* fname_;
  int line_;
  std::stringstream ss_;
};

#define EXPECT_TRUE(c) \
  sentencepiece::test::Tester(__FILE__, __LINE__).Is((c), #c)
#define EXPECT_FALSE(c) \
  sentencepiece::test::Tester(__FILE__, __LINE__).Is((!(c)), #c)
#define EXPECT_STREQ(a, b)                        \
  sentencepiece::test::Tester(__FILE__, __LINE__) \
      .IsEq(std::string(a), std::string(b), #a, #b)
#define EXPECT_EQ(a, b) \
  sentencepiece::test::Tester(__FILE__, __LINE__).IsEq((a), (b), #a, #b)
#define EXPECT_NE(a, b) \
  sentencepiece::test::Tester(__FILE__, __LINE__).IsNe((a), (b), #a, #b)
#define EXPECT_GE(a, b) \
  sentencepiece::test::Tester(__FILE__, __LINE__).IsGe((a), (b), #a, #b)
#define EXPECT_GT(a, b) \
  sentencepiece::test::Tester(__FILE__, __LINE__).IsGt((a), (b), #a, #b)
#define EXPECT_LE(a, b) \
  sentencepiece::test::Tester(__FILE__, __LINE__).IsLe((a), (b), #a, #b)
#define EXPECT_LT(a, b) \
  sentencepiece::test::Tester(__FILE__, __LINE__).IsLt((a), (b), #a, #b)
#define EXPECT_NEAR(a, b, c) \
  sentencepiece::test::Tester(__FILE__, __LINE__).IsNear((a), (b), (c), #a, #b)
#define EXPECT_OK(c) EXPECT_EQ(c, ::sentencepiece::util::OkStatus())
#define EXPECT_NOT_OK(c) EXPECT_NE(c, ::sentencepiece::util::OkStatus())

#define EXPECT_DEATH(statement, condition)   \
  {                                          \
    sentencepiece::error::SetTestCounter(1); \
    statement;                               \
    sentencepiece::error::SetTestCounter(0); \
  };

#define ASSERT_TRUE EXPECT_TRUE
#define ASSERT_FALSE EXPECT_FALSE
#define ASSERT_STREQ EXPECT_STREQ
#define ASSERT_EQ EXPECT_EQ
#define ASSERT_NE EXPECT_NE
#define ASSERT_GE EXPECT_GE
#define ASSERT_GT EXPECT_GT
#define ASSERT_LE EXPECT_LE
#define ASSERT_LT EXPECT_LT
#define ASSERT_NEAR EXPECT_NEAR
#define ASSERT_NOT_OK EXPECT_NOT_OK
#define ASSERT_DEATH ASSERT_DEATH

template <typename T>
class TestWithParam {
 public:
  using ParamType = T;
  virtual void SetUp() {}
  virtual void TearDown() {}
  virtual ~TestWithParam() {}
  virtual ParamType GetParam() const { return ParamType(); }
};

template <typename T>
std::vector<T> ValuesIn(const std::vector<T>& v) {
  return v;
}

#define TCONCAT(a, b, c) TCONCAT1(a, b, c)
#define TCONCAT1(a, b, c) a##b##c

#define INSTANTIATE_TEST_SUITE_P(suite_base, base, params)           \
  std::vector<base::ParamType> TCONCAT(base, _get_params_, base)() { \
    return params;                                                   \
  }

#define TEST(base, name)                                                       \
  class TCONCAT(base, _Test_, name) {                                          \
   public:                                                                     \
    void _Run();                                                               \
    static void _RunIt() {                                                     \
      TCONCAT(base, _Test_, name) t;                                           \
      t._Run();                                                                \
    }                                                                          \
  };                                                                           \
  bool TCONCAT(base, _Test_ignored_, name) =                                   \
      sentencepiece::test::RegisterTest(#base, #name,                          \
                                        &TCONCAT(base, _Test_, name)::_RunIt); \
  void TCONCAT(base, _Test_, name)::_Run()

#define TEST_P(base, name)                                          \
  std::vector<base::ParamType> TCONCAT(base, _get_params_, base)(); \
  class TCONCAT(base, _Test_p_, name) : public base {               \
   public:                                                          \
    const std::vector<ParamType> GetParams() const {                \
      return TCONCAT(base, _get_params_, base)();                   \
    }                                                               \
    ParamType param_;                                               \
    void SetParam(const ParamType& param) {                         \
      param_ = param;                                               \
    }                                                               \
    const ParamType GetParam() {                                    \
      return param_;                                                \
    }                                                               \
    void _Run();                                                    \
    static void _RunIt() {                                          \
      TCONCAT(base, _Test_p_, name) t;                              \
      for (const auto& param : t.GetParams()) {                     \
        t.SetParam(param);                                          \
        t.SetUp();                                                  \
        t._Run();                                                   \
        t.TearDown();                                               \
      }                                                             \
    }                                                               \
  };                                                                \
  bool TCONCAT(base, _Test_p_ignored_, name) =                      \
      sentencepiece::test::RegisterTest(                            \
          #base, #name, &TCONCAT(base, _Test_p_, name)::_RunIt);    \
  void TCONCAT(base, _Test_p_, name)::_Run()

// Register the specified test.  Typically not used directly, but
// invoked via the macro expansion of TEST.
extern bool RegisterTest(const char* base, const char* name, void (*func)());
}  // namespace test
}  // namespace sentencepiece
#endif  // TESTHARNESS_H_
