// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_PARKABLE_STRING_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_PARKABLE_STRING_MANAGER_H_

#include <memory>
#include <utility>

#include "base/feature_list.h"
#include "base/gtest_prod_util.h"
#include "base/memory/post_delayed_memory_reduction_task.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "base/trace_event/memory_dump_provider.h"
#include "third_party/blink/renderer/platform/bindings/parkable_string.h"
#include "third_party/blink/renderer/platform/disk_data_allocator.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/rail_mode_observer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class DiskDataAllocator;
class ParkableString;

class PLATFORM_EXPORT ParkableStringManagerDumpProvider
    : public base::trace_event::MemoryDumpProvider {
  USING_FAST_MALLOC(ParkableStringManagerDumpProvider);

 public:
  static ParkableStringManagerDumpProvider* Instance();
  ParkableStringManagerDumpProvider(const ParkableStringManagerDumpProvider&) =
      delete;
  ParkableStringManagerDumpProvider& operator=(
      const ParkableStringManagerDumpProvider&) = delete;
  ~ParkableStringManagerDumpProvider() override;

  bool OnMemoryDump(const base::trace_event::MemoryDumpArgs&,
                    base::trace_event::ProcessMemoryDump*) override;

 private:
  ParkableStringManagerDumpProvider();
};

// Manages all the ParkableStrings, and parks eligible strings after the
// renderer has been backgrounded.
// Main Thread only.
// When a `ParkableString` is unparked on a background thread, a task is posted
// to the main thread to update the entries in the manager. Hence, it is
// possible to temporarily have an unparked `ParkableString` inaccessible
// through `unparked_strings_`. This can cause aging of the string to be
// delayed or a variation on the sizes recorded in 'ComputeStatistics()`.
class PLATFORM_EXPORT ParkableStringManager : public RAILModeObserver {
  USING_FAST_MALLOC(ParkableStringManager);

 public:
  struct Statistics;

  static ParkableStringManager& Instance();
  ParkableStringManager(const ParkableStringManager&) = delete;
  ParkableStringManager& operator=(const ParkableStringManager&) = delete;
  ~ParkableStringManager() override;

  void SetRendererBackgrounded(bool backgrounded);
  void OnRAILModeChanged(RAILMode rail_mode) override;

  void PurgeMemory();
  // Number of parked and unparked strings. Public for testing.
  size_t Size() const;

  bool OnMemoryDump(base::trace_event::ProcessMemoryDump* pmd);

  // Whether a string is parkable or not. Can be called from any thread.
  static bool ShouldPark(const StringImpl& string);

  static base::TimeDelta AgingInterval();

  // According to UMA data (as of 2021-11-09) ~70% of renderers exist for less
  // than 60 seconds. Using this as a delay of the first parking attempts
  // to lower the number of parking operations on strings that are discarded
  // soon after when the renderer dies. Subsequent additions will schedule
  // according to the default delay. This applies to all renderers, young or
  // old. This is considered a necessary trade-off to cover the spare renderer
  // use-case. The spare renderer might have a long lifetime on first addition
  // of a parkable string but it does not indicate a higher chance of the
  // renderer staying alive for a long time.
  constexpr static base::TimeDelta kFirstParkingDelay{base::Seconds(60)};

  static const char* kAllocatorDumpName;

  // Compares not the pointers, but the arrays. Uses pointers to save space.
  struct SecureDigestHashTraits
      : GenericHashTraits<const ParkableStringImpl::SecureDigest*> {
    static unsigned GetHash(const ParkableStringImpl::SecureDigest* digest) {
      // The first bytes of the hash are as good as anything else.
      return *reinterpret_cast<const unsigned*>(digest->data());
    }

    static bool Equal(const ParkableStringImpl::SecureDigest* const a,
                      const ParkableStringImpl::SecureDigest* const b) {
      return a == b || *a == *b;
    }

    static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
  };

  // Relies on secure hash equality for deduplication. If one day SHA256 becomes
  // insecure, then this would need to be updated to a more robust hash.
  using StringMap = WTF::HashMap<const ParkableStringImpl::SecureDigest*,
                                 ParkableStringImpl*,
                                 SecureDigestHashTraits>;

  bool IsOnParkedMapForTesting(ParkableStringImpl* string);
  bool IsOnDiskMapForTesting(ParkableStringImpl* string);

 private:
  friend class ParkableString;
  friend class ParkableStringImpl;

  scoped_refptr<ParkableStringImpl> Add(
      scoped_refptr<StringImpl>&&,
      std::unique_ptr<ParkableStringImpl::SecureDigest> digest);

  void RemoveOnMainThread(ParkableStringImpl* string);
  // If on a background thread, posts a `RemoveOnMainThread` task to the Main
  // thread. Calls `RemoveOnMainThread` otherwise.
  void Remove(ParkableStringImpl* string);

  void OnParked(ParkableStringImpl*);
  void OnWrittenToDisk(ParkableStringImpl*);
  void OnUnparked(ParkableStringImpl*, bool);

  // If on a background thread, posts a `CompleteUnparkOnMainThread` task to
  // the Main thread. Calls `CompleteUnparkOnMainThread` otherwise.
  void CompleteUnpark(ParkableStringImpl* string,
                      base::TimeDelta elapsed,
                      base::TimeDelta disk_elapsed);

  void CompleteUnparkOnMainThread(ParkableStringImpl* string,
                                  base::TimeDelta elapsed,
                                  base::TimeDelta disk_elapsed);

  void ParkAll(ParkableStringImpl::ParkingMode mode);
  void RecordStatisticsAfter5Minutes() const;
  void AgeStringsAndPark(base::MemoryReductionTaskContext context);
  void ScheduleAgingTaskIfNeeded();

  void RecordUnparkingTime(base::TimeDelta unparking_time) {
    total_unparking_time_ += unparking_time;
  }
  void RecordParkingThreadTime(base::TimeDelta parking_thread_time) {
    total_parking_thread_time_ += parking_thread_time;
  }
  void RecordDiskWriteTime(base::TimeDelta write_time) {
    total_disk_write_time_ += write_time;
  }
  void RecordDiskReadTime(base::TimeDelta read_time) {
    total_disk_read_time_ += read_time;
  }

  Statistics ComputeStatistics() const;

  DiskDataAllocator& data_allocator() const {
    if (allocator_for_testing_)
      return *allocator_for_testing_;

    return DiskDataAllocator::Instance();
  }

  void SetDataAllocatorForTesting(
      std::unique_ptr<DiskDataAllocator> allocator) {
    allocator_for_testing_ = std::move(allocator);
  }

  void SetTaskRunnerForTesting(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    task_runner_ = std::move(task_runner);
  }

  scoped_refptr<base::SingleThreadTaskRunner> task_runner() {
    return task_runner_;
  }

  void AssertRemoved(ParkableStringImpl* string);
  void ResetForTesting();
  bool IsPaused() const;
  bool HasPendingWork() const;
  ParkableStringManager();

  // Arbitrarily chosen, was shown to not regress metrics in a field experiment
  // in 2019 on desktop and Android. From local testing, strings are either
  // requested in a very rapid succession (during compilation), or almost
  // never. We want to allow strings to be dropped quickly, to reduce peak
  // memory usage, particularly as reading and decompressing strings is
  // typically very cheap.
  constexpr static base::TimeDelta kAgingInterval = base::Seconds(2);
  constexpr static base::TimeDelta kLessAggressiveAgingInterval =
      base::Seconds(10);

  bool backgrounded_ = false;
  RAILMode rail_mode_ = RAILMode::kDefault;
  bool has_pending_aging_task_ = false;
  bool has_posted_unparking_time_accounting_task_ = false;
  bool did_register_memory_pressure_listener_ = false;
  base::TimeDelta total_unparking_time_;
  base::TimeDelta total_parking_thread_time_;
  base::TimeDelta total_disk_read_time_;
  base::TimeDelta total_disk_write_time_;

  StringMap unparked_strings_;
  StringMap parked_strings_;
  StringMap on_disk_strings_;

  bool first_string_aging_was_delayed_ = false;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  std::unique_ptr<DiskDataAllocator> allocator_for_testing_;

  friend class ParkableStringTest;
  FRIEND_TEST_ALL_PREFIXES(ParkableStringTest, SynchronousCompression);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_PARKABLE_STRING_MANAGER_H_
