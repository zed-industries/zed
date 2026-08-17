// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_MEMORY_DANGLING_PTR_INSTRUMENTATION_H_
#define BASE_TEST_MEMORY_DANGLING_PTR_INSTRUMENTATION_H_

#include <cstdint>
#include <string_view>

#include "base/memory/raw_ptr.h"
#include "base/types/expected.h"
#include "partition_alloc/dangling_raw_ptr_checks.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace base::test {

// It is difficult to configure malloc as partition_alloc in death test and
// enable BackupRefPtr. This can be used as an alternative. This replaces a
// crash by incrementing a set of counters.
//
// Usage:
//
// ```cpp
// TEST(DanglingTest, Basic) {
//   auto instrumentation = test::DanglingPtrInstrumentation::Create();
//   if (!instrumentation.has_value()) {
//     GTEST_SKIP() << instrumentation.error();
//   }
//
//   [...]
//   EXPECT_EQ(instrumentation->dangling_ptr_detected(), 0u);
//   EXPECT_EQ(instrumentation->dangling_ptr_released(), 0u);
// }
// ```
class DanglingPtrInstrumentation {
 public:
  // Returns the DanglingPtrInstrumentation or a reason why it can't be used,
  // in which case the test should be skipped.
  //
  // This function should typically be called from the `testing::Test::SetUp()`
  // override so that it can skip the test with `GTEST_SKIP()` on failure.
  static base::expected<DanglingPtrInstrumentation, std::string_view> Create();

  ~DanglingPtrInstrumentation();
  DanglingPtrInstrumentation(const DanglingPtrInstrumentation&) = delete;
  DanglingPtrInstrumentation(DanglingPtrInstrumentation&&);
  DanglingPtrInstrumentation& operator=(const DanglingPtrInstrumentation&) =
      delete;
  DanglingPtrInstrumentation& operator=(DanglingPtrInstrumentation&&);

  size_t dangling_ptr_detected() { return dangling_ptr_detected_; }
  size_t dangling_ptr_released() { return dangling_ptr_released_; }

 private:
  static void IncreaseCountDetected(std::uintptr_t);
  static void IncreaseCountReleased(std::uintptr_t);
  static raw_ptr<DanglingPtrInstrumentation> g_observer;

  DanglingPtrInstrumentation();

  void Register();
  void Unregister();

  size_t dangling_ptr_detected_ = 0;
  size_t dangling_ptr_released_ = 0;
  partition_alloc::DanglingRawPtrDetectedFn* old_detected_fn_ = nullptr;
  partition_alloc::DanglingRawPtrReleasedFn* old_dereferenced_fn_ = nullptr;
};

}  // namespace base::test

#endif  // BASE_TEST_MEMORY_DANGLING_PTR_INSTRUMENTATION_H_
