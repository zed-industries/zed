// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_SCOPED_AMOUNT_OF_PHYSICAL_MEMORY_OVERRIDE_H_
#define BASE_TEST_SCOPED_AMOUNT_OF_PHYSICAL_MEMORY_OVERRIDE_H_

#include <stdint.h>

#include <optional>

namespace base::test {

// Sets the amount of physical memory in MB override on construction, and
// removes it when the object goes out of scope. This class is intended to be
// used by tests that need to override the amount of physical memory on the
// system to validate different system conditions.
class ScopedAmountOfPhysicalMemoryOverride {
 public:
  // Constructor that initializes the amount of memory override. Memory is
  // specified in MB.
  explicit ScopedAmountOfPhysicalMemoryOverride(uint64_t amount_of_memory_mb);

  ScopedAmountOfPhysicalMemoryOverride(
      const ScopedAmountOfPhysicalMemoryOverride&) = delete;
  ScopedAmountOfPhysicalMemoryOverride& operator=(
      const ScopedAmountOfPhysicalMemoryOverride&) = delete;

  ~ScopedAmountOfPhysicalMemoryOverride();

 private:
  std::optional<uint64_t> old_amount_of_physical_memory_mb_;
};

}  // namespace base::test

#endif  // BASE_TEST_SCOPED_AMOUNT_OF_PHYSICAL_MEMORY_OVERRIDE_H_
