// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_ALLOCATION_DATA_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_ALLOCATION_DATA_H_

#include <cstdint>

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/tagging.h"

namespace partition_alloc {

// Definitions of various parameters of override and observer hooks. Allocation
// and free path differ from each other in that the allocation override provides
// data to the caller (we have an out parameter there), whereas the free
// override just consumes the data.

// AllocationNotificationData is the in-parameter of an allocation observer
// hook.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) AllocationNotificationData {
 public:
  AllocationNotificationData(void* address, size_t size, const char* type_name)
      : address_(address), size_(size), type_name_(type_name) {}

  void* address() const { return address_; }
  size_t size() const { return size_; }
  const char* type_name() const { return type_name_; }

  // In the allocation observer path, it's interesting which reporting mode is
  // enabled.
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  AllocationNotificationData& SetMteReportingMode(
      TagViolationReportingMode mode) {
    mte_reporting_mode_ = mode;
    return *this;
  }

  TagViolationReportingMode mte_reporting_mode() const {
    return mte_reporting_mode_;
  }
#else
  constexpr TagViolationReportingMode mte_reporting_mode() const {
    return TagViolationReportingMode::kUndefined;
  }
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)

 private:
  void* address_ = nullptr;
  size_t size_ = 0;
  const char* type_name_ = nullptr;
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  TagViolationReportingMode mte_reporting_mode_ =
      TagViolationReportingMode::kUndefined;
#endif
};

// FreeNotificationData is the in-parameter of a free observer hook.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) FreeNotificationData {
 public:
  constexpr explicit FreeNotificationData(void* address) : address_(address) {}

  void* address() const { return address_; }

  // In the free observer path, it's interesting which reporting mode is
  // enabled.
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  FreeNotificationData& SetMteReportingMode(TagViolationReportingMode mode) {
    mte_reporting_mode_ = mode;
    return *this;
  }

  TagViolationReportingMode mte_reporting_mode() const {
    return mte_reporting_mode_;
  }
#else
  constexpr TagViolationReportingMode mte_reporting_mode() const {
    return TagViolationReportingMode::kUndefined;
  }
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)

 private:
  void* address_ = nullptr;
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  TagViolationReportingMode mte_reporting_mode_ =
      TagViolationReportingMode::kUndefined;
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)
};

}  // namespace partition_alloc
#endif  // PARTITION_ALLOC_PARTITION_ALLOC_ALLOCATION_DATA_H_
