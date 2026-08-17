// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_STRINGS_CSTRING_BUILDER_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_STRINGS_CSTRING_BUILDER_H_

#include <cstddef>

#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc::internal::base::strings {

// Similar to std::ostringstream, but creates a C string, i.e. nul-terminated
// char-type string, instead of std::string. To use inside memory allocation,
// this method must not allocate any memory with malloc, aligned_malloc,
// calloc, and so on.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) CStringBuilder {
 public:
  // If kBufferSize is too large, PA_LOG() and PA_BASE_*CHECK() will spend
  // much more stack. This causes out-of-stack.
  // ThreadTest.StartWithOptions_StackSize checks if threads can run with
  // some specified stack size. If kBufferSize==1024u, the test will fail
  // on 32bit bots.
  static constexpr size_t kBufferSize = 256u;

  CStringBuilder() : ptr_(buffer_) {}

  CStringBuilder& operator<<(char ch);
  CStringBuilder& operator<<(const char* text);
  CStringBuilder& operator<<(float value);
  CStringBuilder& operator<<(double value);
  CStringBuilder& operator<<(int value);
  CStringBuilder& operator<<(unsigned int value);
  CStringBuilder& operator<<(long value);
  CStringBuilder& operator<<(unsigned long value);
  CStringBuilder& operator<<(long long value);
  CStringBuilder& operator<<(unsigned long long value);
  CStringBuilder& operator<<(const void* value);
  CStringBuilder& operator<<(std::nullptr_t);
  const char* c_str();

 private:
  template <typename T>
  void PutInteger(T value);
  void PutFloatingPoint(double value, unsigned num_digits10);
  void PutNormalFloatingPoint(double value, unsigned num_digits10);
  void PutText(const char* text);
  void PutText(const char* text, size_t length);

  char buffer_[kBufferSize];
  char* ptr_;
};

}  // namespace partition_alloc::internal::base::strings

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_STRINGS_CSTRING_BUILDER_H_
