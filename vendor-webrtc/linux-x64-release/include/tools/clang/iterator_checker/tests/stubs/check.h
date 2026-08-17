// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef TOOLS_CLANG_ITERATOR_CHECKER_TESTS_STUBS_CHECK_H_
#define TOOLS_CLANG_ITERATOR_CHECKER_TESTS_STUBS_CHECK_H_

namespace std {
class ostream;
}  // namespace std

namespace logging {
class VoidifyStream {
 public:
  VoidifyStream() = default;
  void operator&(std::ostream&) {}
};

class CheckError {
 public:
  static CheckError Check(const char* file, int line, const char* condition);
  static CheckError DCheck(const char* file, int line, const char* condition);
  static CheckError PCheck(const char* file, int line, const char* condition);
  static CheckError PCheck(const char* file, int line);
  static CheckError DPCheck(const char* file, int line, const char* condition);

  std::ostream& stream();

  ~CheckError();

  CheckError(const CheckError& other) = delete;
  CheckError& operator=(const CheckError& other) = delete;
  CheckError(CheckError&& other) = default;
  CheckError& operator=(CheckError&& other) = default;
};

}  // namespace logging

#define LAZY_CHECK_STREAM(stream, condition) \
  !(condition) ? (void)0 : ::logging::VoidifyStream() & (stream)

#define CHECK(condition)                                                     \
  LAZY_CHECK_STREAM(                                                         \
      ::logging::CheckError::Check(__FILE__, __LINE__, #condition).stream(), \
      !(condition))

#define PCHECK(condition)                                                     \
  LAZY_CHECK_STREAM(                                                          \
      ::logging::CheckError::PCheck(__FILE__, __LINE__, #condition).stream(), \
      !(condition))

#define DCHECK(condition)                                                     \
  LAZY_CHECK_STREAM(                                                          \
      ::logging::CheckError::DCheck(__FILE__, __LINE__, #condition).stream(), \
      !(condition))

#define DPCHECK(condition)                                                     \
  LAZY_CHECK_STREAM(                                                           \
      ::logging::CheckError::DPCheck(__FILE__, __LINE__, #condition).stream(), \
      !(condition))

#endif  // TOOLS_CLANG_ITERATOR_CHECKER_TESTS_STUBS_CHECK_H_
