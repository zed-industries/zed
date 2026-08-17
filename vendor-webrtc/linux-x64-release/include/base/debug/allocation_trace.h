// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_DEBUG_ALLOCATION_TRACE_H_
#define BASE_DEBUG_ALLOCATION_TRACE_H_

#include <algorithm>
#include <array>
#include <atomic>
#include <bit>
#include <cstdint>

#include "base/allocator/dispatcher/notification_data.h"
#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/debug/debugging_buildflags.h"
#include "base/debug/stack_trace.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "build/build_config.h"

namespace base::debug::tracer {

// Number of traces that can be stored. This number must be a power of two to
// allow for fast computation of modulo.
constexpr size_t kMaximumNumberOfMemoryOperationTraces = (1 << 15);
// Number of frames stored for each operation. Probably the lower frames
// represent the memory allocation system. Hence, we store more frames to
// increase chances of having a meaningful trace of the path that caused the
// allocation or free.
constexpr size_t kStackTraceSize = 16;

// The type of an operation stored in the recorder.
enum class OperationType {
  // The state of an operation record before calling any of the initialization
  // functions.
  kNone = 0,
  // The record represents an allocation operation.
  kAllocation,
  // The record represents a free operation.
  kFree,
};

using StackTraceContainer = std::array<const void*, kStackTraceSize>;

// The record for a single operation. A record can represent any type of
// operation, allocation or free, but not at the same time.
//
// A record protects itself from concurrent initializations. If a thread B calls
// any of the Initialize*-functions while another thread A is currently
// initializing, B's invocations shall immediately return |false| without
// interfering with thread A.
class BASE_EXPORT OperationRecord {
 public:
  constexpr OperationRecord() = default;

  OperationRecord(const OperationRecord&) = delete;
  OperationRecord& operator=(const OperationRecord&) = delete;

  // Is the record currently being taken?
  bool IsRecording() const;

  OperationType GetOperationType() const;
  // The address allocated or freed.
  const void* GetAddress() const;
  // Number of allocated bytes. Returns 0 for free operations.
  size_t GetSize() const;
  // The stacktrace as taken by the Initialize*-functions.
  const StackTraceContainer& GetStackTrace() const LIFETIME_BOUND;

  // Initialize the record with data for another operation. Data from any
  // previous operation will be silently overwritten. These functions are
  // declared ALWAYS_INLINE to minimize pollution of the recorded stack trace.
  //
  // Both functions return false in case no record was taken, i.e. if another
  // thread is capturing.
  ALWAYS_INLINE bool InitializeFree(const void* freed_address) {
    return InitializeOperationRecord(freed_address, 0, OperationType::kFree);
  }

  ALWAYS_INLINE bool InitializeAllocation(const void* allocated_address,
                                          size_t allocated_size) {
    return InitializeOperationRecord(allocated_address, allocated_size,
                                     OperationType::kAllocation);
  }

 private:
  // Initialize a record with the given data. Return true if the record was
  // initialized successfully, false if no record was taken, i.e. if another
  // thread is capturing.
  ALWAYS_INLINE bool InitializeOperationRecord(const void* address,
                                               size_t size,
                                               OperationType operation_type);
  ALWAYS_INLINE void StoreStackTrace();

  // The stack trace taken in one of the Initialize* functions.
  StackTraceContainer stack_trace_ = {};
  // The number of allocated bytes.
  size_t size_ = 0;
  // The address that was allocated or freed.
  // We use a raw C++ pointer instead of base::raw_ptr for performance
  // reasons.
  // - In the recorder we only store pointers, we never allocate or free on
  //   our own.
  // - Storing is the hot path. base::raw_ptr::operator== may perform sanity
  //   checks which do not make sense in our case (otherwise the allocated
  //   address would have been quirky)
  RAW_PTR_EXCLUSION const void* address_ = nullptr;
  // The type of the operation that was performed. In the course of making a
  // record, this value is reset to |OperationType::kNone| and later set to
  // the operation type specific value, so if the process crashes whilst writing
  // the record, it's marked as empty. To prevent the compiler from optimizing
  // away the initial reset, this value is marked as volatile.
  volatile OperationType operation_type_ = OperationType::kNone;
  // Is the record currently being taken from another thread? Used to prevent
  // concurrent writes to the same record.
  //
  // The value is mutable since pre C++20 there is no const getter in
  // atomic_flag. All ways to get the value involve setting it.
  // TODO(crbug.com/42050406): Remove mutable and make IsRecording() use
  // atomic_flag::test();
  mutable std::atomic_flag is_recording_ = ATOMIC_FLAG_INIT;
};

ALWAYS_INLINE bool OperationRecord::InitializeOperationRecord(
    const void* address,
    size_t size,
    OperationType operation_type) {
  if (is_recording_.test_and_set(std::memory_order_acquire)) {
    return false;
  }

  operation_type_ = operation_type;
  StoreStackTrace();
  address_ = address;
  size_ = size;

  is_recording_.clear(std::memory_order_release);

  return true;
}

ALWAYS_INLINE void OperationRecord::StoreStackTrace() {
  stack_trace_.fill(nullptr);

#if BUILDFLAG(CAN_UNWIND_WITH_FRAME_POINTERS)
  // Currently we limit ourselves to use TraceStackFramePointers. We know that
  // TraceStackFramePointers has an acceptable performance impact on Android.
  base::debug::TraceStackFramePointers(stack_trace_, 0);
#elif BUILDFLAG(IS_LINUX)
  // Use base::debug::CollectStackTrace as an alternative for tests on Linux. We
  // still have a check in /base/debug/debug.gni to prevent that
  // AllocationStackTraceRecorder is enabled accidentally on Linux.
  base::debug::CollectStackTrace(stack_trace_);
#else
#error "No supported stack tracer found."
#endif
}

struct BASE_EXPORT AllocationTraceRecorderStatistics {
#if BUILDFLAG(ENABLE_ALLOCATION_TRACE_RECORDER_FULL_REPORTING)
  AllocationTraceRecorderStatistics(size_t total_number_of_allocations,
                                    size_t total_number_of_collisions);
#else
  AllocationTraceRecorderStatistics(size_t total_number_of_allocations);
#endif

  // The total number of allocations that have been recorded.
  size_t total_number_of_allocations;
#if BUILDFLAG(ENABLE_ALLOCATION_TRACE_RECORDER_FULL_REPORTING)
  // The total number of collisions that have been encountered. A collision
  // happens when two threads concurrently try to record using the same slot.
  size_t total_number_of_collisions;
#endif
};

// The recorder which holds entries for past memory operations.
//
// The memory image of the recorder will be copied into the crash-handler.
// Therefore, it must not hold any references to external data which are vital
// for proper functioning.
//
// It is important that the recorder itself does not allocate to prevent
// recursive calls and save as much runtime overhead as possible.
//
// Therefore, records are stored in a preallocated buffer with a compile time
// constant maximum size, see |kMaximumNumberOfMemoryOperationTraces|. Once all
// records have been used, old records will be overwritten (fifo-style).
//
// The recorder works in an multithreaded environment without external locking.
// Concurrent writes are prevented by two means:
//  1 - We atomically increment and calculate the effective index of the record
//  to be written.
//  2 - If this entry is still being used (the recording thread didn't finish
//  yet), we go back to step 1
// Currently we do not enforce separate cache lines for each entry, which means
// false sharing can occur. On the other hand, with 64 byte cachelines a clean
// separation would introduce some 3*64 - sizeof(OperationRecord) = 40 bytes of
// padding per entry.
//
// Note: As a process might be terminated for whatever reason while stack
// traces are being written, the recorded data may contain some garbage.
//
// TODO(crbug.com/40258550): Evaluate the impact of the shared cache
// lines between entries.
class BASE_EXPORT AllocationTraceRecorder {
 public:
  constexpr AllocationTraceRecorder() = default;

  AllocationTraceRecorder(const AllocationTraceRecorder&) = delete;
  AllocationTraceRecorder& operator=(const AllocationTraceRecorder&) = delete;

  // The allocation event observer interface. See the dispatcher for further
  // details. The functions are marked NO_INLINE. All other functions called but
  // the one taking the call stack are marked ALWAYS_INLINE. This way we ensure
  // the number of frames recorded from these functions is fixed.
  inline void OnAllocation(
      const base::allocator::dispatcher::AllocationNotificationData&
          allocation_data);

  // Handle all free events.
  inline void OnFree(
      const base::allocator::dispatcher::FreeNotificationData& free_data);

  // Access functions to retrieve the current content of the recorder.
  // Note: Since the recorder is usually updated upon each allocation or free,
  // it is important to prevent updates if you want to read the entries at any
  // point.

  // Get the current number of entries stored in the recorder. When the
  // recorder has reached its maximum capacity, it always returns
  // |GetMaximumNumberOfTraces()|.
  size_t size() const;

  // Access the record of an operation by index. Oldest operation is always
  // accessible at index 0, latest operation at |size()-1|.
  // Note: Since a process might have crashed while a trace is being written,
  // especially the last records might be corrupted.
  const OperationRecord& operator[](size_t idx) const;

  constexpr size_t GetMaximumNumberOfTraces() const {
    return kMaximumNumberOfMemoryOperationTraces;
  }

  AllocationTraceRecorderStatistics GetRecorderStatistics() const;

 private:
  // Handle all allocation events.
  NOINLINE void OnAllocation(const void* allocated_address,
                             size_t allocated_size);

  // Handle all free events.
  NOINLINE void OnFree(const void* freed_address);

  ALWAYS_INLINE size_t GetNextIndex();

  ALWAYS_INLINE static constexpr size_t WrapIdxIfNeeded(size_t idx);

  // The actual container.
  std::array<OperationRecord, kMaximumNumberOfMemoryOperationTraces>
      alloc_trace_buffer_ = {};
  // The total number of records that have been taken so far. Note that this
  // might be greater than |kMaximumNumberOfMemoryOperationTraces| since we
  // overwrite oldest items.
  std::atomic<size_t> total_number_of_records_ = 0;
#if BUILDFLAG(ENABLE_ALLOCATION_TRACE_RECORDER_FULL_REPORTING)
  std::atomic<size_t> total_number_of_collisions_ = 0;
#endif
};

inline void AllocationTraceRecorder::OnAllocation(
    const base::allocator::dispatcher::AllocationNotificationData&
        allocation_data) {
  OnAllocation(allocation_data.address(), allocation_data.size());
}

// Handle all free events.
inline void AllocationTraceRecorder::OnFree(
    const base::allocator::dispatcher::FreeNotificationData& free_data) {
  OnFree(free_data.address());
}

ALWAYS_INLINE constexpr size_t AllocationTraceRecorder::WrapIdxIfNeeded(
    size_t idx) {
  // Wrapping around counter, e.g. for BUFFER_SIZE = 256, the counter will
  // wrap around when reaching 256. To enable the compiler to emit more
  // optimized code we assert |kMaximumNumberOfMemoryOperationTraces| is a power
  // of two .
  static_assert(
      std::has_single_bit(kMaximumNumberOfMemoryOperationTraces),
      "kMaximumNumberOfMemoryOperationTraces should be a power of 2 to "
      "allow for fast modulo operation.");

  return idx % kMaximumNumberOfMemoryOperationTraces;
}

ALWAYS_INLINE size_t AllocationTraceRecorder::GetNextIndex() {
  const auto raw_idx =
      total_number_of_records_.fetch_add(1, std::memory_order_relaxed);
  return WrapIdxIfNeeded(raw_idx);
}

}  // namespace base::debug::tracer

#endif  // BASE_DEBUG_ALLOCATION_TRACE_H_
