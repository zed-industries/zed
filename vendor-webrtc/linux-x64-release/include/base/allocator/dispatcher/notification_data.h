// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_DISPATCHER_NOTIFICATION_DATA_H_
#define BASE_ALLOCATOR_DISPATCHER_NOTIFICATION_DATA_H_

#include <cstdint>

#include "base/allocator/dispatcher/memory_tagging.h"
#include "base/allocator/dispatcher/subsystem.h"
#include "base/base_export.h"
#include "partition_alloc/buildflags.h"

namespace base::allocator::dispatcher {

// Definitions of the parameter structures passed to the observer hooks. They
// are similar to the structures defined by PartitionAllocator but provide
// further information.

// The notification data for the allocation path.
class BASE_EXPORT AllocationNotificationData {
 public:
  constexpr AllocationNotificationData(void* address,
                                       size_t size,
                                       const char* type_name,
                                       AllocationSubsystem allocation_subsystem)
      : address_(address),
        size_(size),
        type_name_(type_name),
        allocation_subsystem_(allocation_subsystem) {}

  constexpr void* address() const { return address_; }

  constexpr size_t size() const { return size_; }

  constexpr const char* type_name() const { return type_name_; }

  constexpr AllocationSubsystem allocation_subsystem() const {
    return allocation_subsystem_;
  }

  // In the allocation observer path, it's interesting which reporting mode is
  // enabled.
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  constexpr AllocationNotificationData& SetMteReportingMode(MTEMode mode) {
    mte_reporting_mode_ = mode;
    return *this;
  }
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)

  constexpr MTEMode mte_reporting_mode() const {
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
    return mte_reporting_mode_;
#else
    return MTEMode::kUndefined;
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  }

 private:
  void* address_ = nullptr;
  size_t size_ = 0;
  const char* type_name_ = nullptr;
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  MTEMode mte_reporting_mode_ = MTEMode::kUndefined;
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  AllocationSubsystem allocation_subsystem_;
};

// The notification data for the free path.
class BASE_EXPORT FreeNotificationData {
 public:
  constexpr explicit FreeNotificationData(
      void* address,
      AllocationSubsystem allocation_subsystem)
      : address_(address), allocation_subsystem_(allocation_subsystem) {}

  constexpr void* address() const { return address_; }

  constexpr AllocationSubsystem allocation_subsystem() const {
    return allocation_subsystem_;
  }

  // In the free observer path, it's interesting which reporting mode is
  // enabled.
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  constexpr FreeNotificationData& SetMteReportingMode(MTEMode mode) {
    mte_reporting_mode_ = mode;
    return *this;
  }
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)

  constexpr MTEMode mte_reporting_mode() const {
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
    return mte_reporting_mode_;
#else
    return MTEMode::kUndefined;
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  }

 private:
  void* address_ = nullptr;
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  MTEMode mte_reporting_mode_ = MTEMode::kUndefined;
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  AllocationSubsystem allocation_subsystem_;
};

}  // namespace base::allocator::dispatcher
#endif  // BASE_ALLOCATOR_DISPATCHER_NOTIFICATION_DATA_H_
