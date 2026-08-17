// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Lightweight Quarantine (LQ) provides a low-cost quarantine mechanism with
// following characteristics.
//
// - Built on PartitionAlloc: only supports allocations in a known root
// - As fast as PA: LQ just defers `Free()` handling and may benefit from thread
//   cache etc.
// - Thread-safe
// - No allocation time information: triggered on `Free()`
// - Don't use quarantined objects' payload - available for zapping
// - Don't allocate heap memory.
// - Flexible to support several applications
//
// `LightweightQuarantineRoot` represents one quarantine system
// (e.g. scheduler loop quarantine).
// `LightweightQuarantineBranch` provides a quarantine request interface.
// It belongs to a `LightweightQuarantineRoot` and there can be multiple
// instances (e.g. one per thread). By having one branch per thread, it requires
// no lock for faster quarantine.
// ┌────────────────────────────┐
// │PartitionRoot               │
// └┬──────────────────────────┬┘
// ┌▽────────────────────────┐┌▽────────────────────┐
// │LQRoot 1                 ││LQRoot 2             │
// └┬───────────┬───────────┬┘└──────────────┬──┬──┬┘
// ┌▽─────────┐┌▽─────────┐┌▽─────────┐      ▽  ▽  ▽
// │LQBranch 1││LQBranch 2││LQBranch 3│
// └──────────┘└──────────┘└──────────┘

#ifndef PARTITION_ALLOC_LIGHTWEIGHT_QUARANTINE_H_
#define PARTITION_ALLOC_LIGHTWEIGHT_QUARANTINE_H_

#include <array>
#include <atomic>
#include <cstdint>
#include <limits>
#include <memory>
#include <optional>
#include <type_traits>
#include <vector>

#include "partition_alloc/internal_allocator_forward.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/rand_util.h"
#include "partition_alloc/partition_alloc_base/thread_annotations.h"
#include "partition_alloc/partition_alloc_forward.h"
#include "partition_alloc/partition_lock.h"
#include "partition_alloc/partition_stats.h"

namespace partition_alloc {

struct PartitionRoot;
struct LightweightQuarantineStats;

namespace internal {

struct LightweightQuarantineBranchConfig {
  // When set to `false`, the branch is for single-thread use (faster).
  bool lock_required = true;
  // Capacity for a branch in bytes.
  size_t branch_capacity_in_bytes = 0;
  // Leak quarantined allocations at exit.
  bool leak_on_destruction = false;
};

class LightweightQuarantineBranch;

class PA_COMPONENT_EXPORT(PARTITION_ALLOC) LightweightQuarantineRoot {
 public:
  explicit LightweightQuarantineRoot(PartitionRoot& allocator_root)
      : allocator_root_(allocator_root) {}

  LightweightQuarantineBranch CreateBranch(
      const LightweightQuarantineBranchConfig& config);

  PartitionRoot& GetAllocatorRoot() { return allocator_root_; }

  void AccumulateStats(LightweightQuarantineStats& stats) const {
    stats.count += count_.load(std::memory_order_relaxed);
    stats.size_in_bytes += size_in_bytes_.load(std::memory_order_relaxed);
    stats.cumulative_count += cumulative_count_.load(std::memory_order_relaxed);
    stats.cumulative_size_in_bytes +=
        cumulative_size_in_bytes_.load(std::memory_order_relaxed);
    stats.quarantine_miss_count +=
        quarantine_miss_count_.load(std::memory_order_relaxed);
  }

 private:
  PartitionRoot& allocator_root_;

  // Stats.
  std::atomic_size_t size_in_bytes_ = 0;
  std::atomic_size_t count_ = 0;  // Number of quarantined entries
  std::atomic_size_t cumulative_count_ = 0;
  std::atomic_size_t cumulative_size_in_bytes_ = 0;
  std::atomic_size_t quarantine_miss_count_ = 0;

  friend class LightweightQuarantineBranch;
};

class PA_COMPONENT_EXPORT(PARTITION_ALLOC) LightweightQuarantineBranch {
 public:
  using Root = LightweightQuarantineRoot;

  LightweightQuarantineBranch(Root& root,
                              const LightweightQuarantineBranchConfig& config);
  LightweightQuarantineBranch(const LightweightQuarantineBranch&) = delete;
  LightweightQuarantineBranch(LightweightQuarantineBranch&& b);
  ~LightweightQuarantineBranch();

  // Quarantines an object. This list holds information you put into `entry`
  // as much as possible.  If the object is too large, this may return
  // `false`, meaning that quarantine request has failed (and freed
  // immediately). Otherwise, returns `true`.
  PA_ALWAYS_INLINE bool Quarantine(
      void* object,
      SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span,
      uintptr_t slot_start,
      size_t usable_size) {
    if (lock_required_) {
      PA_MUSTTAIL return QuarantineWithAcquiringLock(object, slot_span,
                                                     slot_start, usable_size);
    } else {
      PA_MUSTTAIL return QuarantineWithoutAcquiringLock(
          object, slot_span, slot_start, usable_size);
    }
  }

  // Despite that LightweightQuarantineBranchConfig::lock_required_ is already
  // specified, we provide two versions `With/WithoutAcquiringLock` so that we
  // can avoid the overhead of runtime conditional branches.
  bool QuarantineWithAcquiringLock(
      void* object,
      SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span,
      uintptr_t slot_start,
      size_t usable_size);
  bool QuarantineWithoutAcquiringLock(
      void* object,
      SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span,
      uintptr_t slot_start,
      size_t usable_size);

  // Dequarantine all entries **held by this branch**.
  // It is possible that another branch with entries and it remains untouched.
  void Purge();

  // Determines this list contains an object.
  bool IsQuarantinedForTesting(void* object);

  Root& GetRoot() { return root_; }

  size_t GetCapacityInBytes() {
    return branch_capacity_in_bytes_.load(std::memory_order_relaxed);
  }
  // After shrinking the capacity, this branch may need to `Purge()` to meet the
  // requirement.
  void SetCapacityInBytes(size_t capacity_in_bytes);

 private:
  class PA_SCOPED_LOCKABLE FakeScopedGuard;
  class PA_SCOPED_LOCKABLE RuntimeConditionalScopedGuard;

  // `ToBeFreedArray` is used in `PurgeInternalInTwoPhases1of2` and
  // `PurgeInternalInTwoPhases2of2`. See the function comment about the purpose.
  // In order to avoid reentrancy issues, we must not deallocate any object in
  // `Quarantine`. So, std::vector is not an option. std::array doesn't
  // deallocate, plus, std::array has perf advantages.
  static constexpr size_t kMaxFreeTimesPerPurge = 1024;
  using ToBeFreedArray = std::array<uintptr_t, kMaxFreeTimesPerPurge>;

  // Try to dequarantine entries to satisfy below:
  //   root_.size_in_bytes_ <=  target_size_in_bytes
  // It is possible that this branch cannot satisfy the
  // request as it has control over only what it has. If you need to ensure the
  // constraint, call `Purge()` for each branch in sequence, synchronously.
  PA_ALWAYS_INLINE void PurgeInternal(size_t target_size_in_bytes)
      PA_EXCLUSIVE_LOCKS_REQUIRED(lock_);
  // In order to reduce thread contention, dequarantines entries in two phases:
  //   Phase 1) With the lock acquired, saves `slot_start`s of the quarantined
  //     objects in an array, and shrinks `slots_`. Then, releases the lock so
  //     that another thread can quarantine an object.
  //   Phase 2) Without the lock acquired, deallocates objects saved in the
  //     array in Phase 1. This may take some time, but doesn't block other
  //     threads.
  PA_ALWAYS_INLINE void PurgeInternalWithDefferedFree(
      size_t target_size_in_bytes,
      ToBeFreedArray& to_be_freed,
      size_t& num_of_slots) PA_EXCLUSIVE_LOCKS_REQUIRED(lock_);
  PA_ALWAYS_INLINE void BatchFree(const ToBeFreedArray& to_be_freed,
                                  size_t num_of_slots);

  Root& root_;

  const bool lock_required_;
  Lock lock_;

  // Non-cryptographic random number generator.
  // Thread-unsafe so guarded by `lock_`.
  base::InsecureRandomGenerator random_ PA_GUARDED_BY(lock_);

  // `slots_` hold quarantined entries.
  struct QuarantineSlot {
    uintptr_t slot_start;
    size_t usable_size;
  };
  std::vector<QuarantineSlot, InternalAllocator<QuarantineSlot>> slots_
      PA_GUARDED_BY(lock_);
  size_t branch_size_in_bytes_ PA_GUARDED_BY(lock_) = 0;
  // Using `std::atomic` here so that other threads can update this value.
  std::atomic_size_t branch_capacity_in_bytes_;

  // This working memory is temporarily needed only while dequarantining
  // objects in slots_ when lock_required_ is true. However, allocating this
  // working memory on stack may cause stack overflow [1]. Plus, it's non-
  // negligible perf penalty to allocate and deallocate this working memory on
  // heap only while dequarantining. So, we reserve one chunk of working memory
  // on heap during the entire lifetime of this branch object and try to reuse
  // this working memory among threads. Only when thread contention occurs, we
  // allocate and deallocate another chunk of working memory.
  // [1] https://issues.chromium.org/issues/387508217
  std::atomic<ToBeFreedArray*> to_be_freed_working_memory_ = nullptr;

  bool leak_on_destruction_ = false;
};

// Scheduler-loop Quarantine is a quarantine pool behind PartitionAlloc with
// Advanced Checks and `ADVANCED_MEMORY_SAFETY_CHECKS()`.
// Both requests to prevent `free()`d allocation getting released to free-list,
// by passing `FreeFlags::kSchedulerLoopQuarantine` at time of `free()`.
// This will keep these allocations in Lightweight Qurantine for while.
// TODO(crbug.com/329027914): In addition to the threshold-based purging in
// Lightweight Quarantine, implement smarter purging strategy to detect "empty
// stack".

struct SchedulerLoopQuarantineConfig {
  LightweightQuarantineBranchConfig quarantine_config;
  bool enable_quarantine = false;
  bool enable_zapping = false;
};

// This is a wrapper of `LightweightQuarantineBranch` for Scheduler-loop
// Quarantine. All operations on the branch should be performed through this
// class.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) SchedulerLoopQuarantineBranch {
 public:
  explicit SchedulerLoopQuarantineBranch(PartitionRoot* allocator_root);
  SchedulerLoopQuarantineBranch(const SchedulerLoopQuarantineBranch&) = delete;
  SchedulerLoopQuarantineBranch(SchedulerLoopQuarantineBranch&& b) = delete;
  ~SchedulerLoopQuarantineBranch();

  void Configure(LightweightQuarantineRoot& root,
                 const SchedulerLoopQuarantineConfig& config);
  PA_ALWAYS_INLINE LightweightQuarantineRoot& GetRoot();

  void QuarantineWithAcquiringLock(
      void* object,
      SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span,
      uintptr_t slot_start,
      size_t usable_size);
  void QuarantineWithoutAcquiringLock(
      void* object,
      SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span,
      uintptr_t slot_start,
      size_t usable_size);

  const SchedulerLoopQuarantineConfig& GetConfigurationForTesting();
  LightweightQuarantineBranch& GetInternalBranchForTesting();

  class ScopedQuarantineExclusion {
    SchedulerLoopQuarantineBranch& branch_;

   public:
    PA_ALWAYS_INLINE explicit ScopedQuarantineExclusion(
        SchedulerLoopQuarantineBranch& branch)
        : branch_(branch) {
      ++branch_.pause_quarantine_;
    }
    ScopedQuarantineExclusion(const ScopedQuarantineExclusion&) = delete;
    PA_ALWAYS_INLINE ~ScopedQuarantineExclusion() {
      --branch_.pause_quarantine_;
    }
  };

 private:
  PA_ALWAYS_INLINE void QuarantineEpilogue(
      void* object,
      SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span,
      uintptr_t slot_start,
      size_t usable_size);

  PartitionRoot* const allocator_root_;
  std::optional<LightweightQuarantineBranch> branch_;

  bool enable_quarantine_ = false;
  bool enable_zapping_ = false;

  // When non-zero, this branch temporarily stops accepting incoming quarantine
  // requests.
  int pause_quarantine_ = 0;

  // Kept for testing purposes only.
  SchedulerLoopQuarantineConfig config_for_testing_;
};

}  // namespace internal

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_LIGHTWEIGHT_QUARANTINE_H_
