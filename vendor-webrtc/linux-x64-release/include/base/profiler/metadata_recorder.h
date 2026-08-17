// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_METADATA_RECORDER_H_
#define BASE_PROFILER_METADATA_RECORDER_H_

#include <array>
#include <atomic>
#include <optional>
#include <utility>

#include "base/base_export.h"
#include "base/memory/stack_allocated.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/threading/platform_thread.h"

namespace base {

// MetadataRecorder provides a data structure to store metadata key/value pairs
// to be associated with samples taken by the sampling profiler. Whatever
// metadata is present in this map when a sample is recorded is then associated
// with the sample.
//
// Methods on this class are safe to call unsynchronized from arbitrary threads.
//
// This class was designed to read metadata from a single sampling thread and
// write metadata from many Chrome threads within the same process. These other
// threads might be suspended by the sampling thread at any time in order to
// collect a sample.
//
// This class has a few notable constraints:
//
// A) If a lock that's required to read the metadata might be held while writing
//    the metadata, that lock must be acquirable *before* the thread is
//    suspended. Otherwise, the sampling thread might suspend the target thread
//    while it is holding the required lock, causing deadlock.
//
//      Ramifications:
//
//      - When retrieving items, lock acquisition (through
//        CreateMetadataProvider()) and actual item retrieval (through
//        MetadataProvider::GetItems()) are separate.
//
// B) We can't allocate data on the heap while reading the metadata items. This
//    is because, on many operating systems, there's a process-wide heap lock
//    that is held while allocating on the heap. If a thread is suspended while
//    holding this lock and the sampling thread then tries to allocate on the
//    heap to read the metadata, it will deadlock trying to acquire the heap
//    lock.
//
//      Ramifications:
//
//      - We hold and retrieve the metadata using a fixed-size array, which
//        allows readers to preallocate the data structure that we pass back
//        the metadata in.
//
// C) We shouldn't guard writes with a lock that also guards reads, since the
//    read lock is held from the time that the sampling thread requests that a
//    thread be suspended up to the time that the thread is resumed. If all
//    metadata writes block their thread during that time, we're very likely to
//    block all Chrome threads.
//
//      Ramifications:
//
//      - We use two locks to guard the metadata: a read lock and a write
//        lock. Only the write lock is required to write into the metadata, and
//        only the read lock is required to read the metadata.
//
//      - Because we can't guard reads and writes with the same lock, we have to
//        face the possibility of writes occurring during a read. This is
//        especially problematic because there's no way to read both the key and
//        value for an item atomically without using mutexes, which violates
//        constraint A). If the sampling thread were to see the following
//        interleaving of reads and writes:
//
//          * Reader thread reads key for slot 0
//          * Writer thread removes item at slot 0
//          * Writer thread creates new item with different key in slot 0
//          * Reader thread reads value for slot 0
//
//        then the reader would see an invalid value for the given key. Because
//        of this possibility, we keep slots reserved for a specific key even
//        after that item has been removed. We reclaim these slots on a
//        best-effort basis during writes when the metadata recorder has become
//        sufficiently full and we can acquire the read lock.
//
//      - We use state stored in atomic data types to ensure that readers and
//        writers are synchronized about where data should be written to and
//        read from. We must use atomic data types to guarantee that there's no
//        instruction during a write after which the recorder is in an
//        inconsistent state that might yield garbage data for a reader.
//
// Here are a few of the many states the recorder can be in:
//
// - No thread is using the recorder.
//
// - A single writer is writing into the recorder without a simultaneous read.
//   The write will succeed.
//
// - A reader is reading from the recorder without a simultaneous write. The
//   read will succeed.
//
// - Multiple writers attempt to write into the recorder simultaneously. All
//   writers but one will block because only one can hold the write lock.
//
// - A writer is writing into the recorder, which hasn't reached the threshold
//   at which it will try to reclaim inactive slots. The writer won't try to
//   acquire the read lock to reclaim inactive slots. The reader will therefore
//   be able to immediately acquire the read lock, suspend the target thread,
//   and read the metadata.
//
// - A writer is writing into the recorder, the recorder has reached the
//   threshold at which it needs to reclaim inactive slots, and the writer
//   thread is now in the middle of reclaiming those slots when a reader
//   arrives. The reader will try to acquire the read lock before suspending the
//   thread but will block until the writer thread finishes reclamation and
//   releases the read lock. The reader will then be able to acquire the read
//   lock and suspend the target thread.
//
// - A reader is reading the recorder when a writer attempts to write. The write
//   will be successful. However, if the writer deems it necessary to reclaim
//   inactive slots, it will skip doing so because it won't be able to acquire
//   the read lock.
class BASE_EXPORT MetadataRecorder {
 public:
  MetadataRecorder();
  virtual ~MetadataRecorder();
  MetadataRecorder(const MetadataRecorder&) = delete;
  MetadataRecorder& operator=(const MetadataRecorder&) = delete;

  struct BASE_EXPORT Item {
    Item(uint64_t name_hash,
         std::optional<int64_t> key,
         std::optional<PlatformThreadId> thread_id,
         int64_t value);
    Item();

    Item(const Item& other);
    Item& operator=(const Item& other);

    // The hash of the metadata name, as produced by HashMetricName().
    uint64_t name_hash;
    // The key if specified when setting the item.
    std::optional<int64_t> key;
    // The thread_id is captured from the current thread for a thread-scoped
    // item.
    std::optional<PlatformThreadId> thread_id;
    // The value of the metadata item.
    int64_t value;
  };
  static constexpr size_t MAX_METADATA_COUNT = 50;
  typedef std::array<Item, MAX_METADATA_COUNT> ItemArray;

  // Sets a value for a (|name_hash|, |key|, |thread_id|) tuple, overwriting any
  // value previously set for the tuple. Nullopt keys are treated as just
  // another key state for the purpose of associating values.
  void Set(uint64_t name_hash,
           std::optional<int64_t> key,
           std::optional<PlatformThreadId> thread_id,
           int64_t value);

  // Removes the item with the specified name hash and optional key. Has no
  // effect if such an item does not exist.
  void Remove(uint64_t name_hash,
              std::optional<int64_t> key,
              std::optional<PlatformThreadId> thread_id);

  // An object that provides access to a MetadataRecorder's items and holds the
  // necessary exclusive read lock until the object is destroyed. Reclaiming of
  // inactive slots in the recorder can't occur while this object lives, so it
  // should be created as soon before it's needed as possible and released as
  // soon as possible.
  //
  // This object should be created *before* suspending the target thread and
  // destroyed after resuming the target thread. Otherwise, that thread might be
  // suspended while reclaiming inactive slots and holding the read lock, which
  // would cause the sampling thread to deadlock.
  //
  // Example usage:
  //
  //   MetadataRecorder r;
  //   base::MetadataRecorder::ItemArray arr;
  //   size_t item_count;
  //   ...
  //   {
  //     MetadtaRecorder::MetadataProvider provider;
  //     item_count = provider.GetItems(arr);
  //   }
  class SCOPED_LOCKABLE BASE_EXPORT MetadataProvider {
    STACK_ALLOCATED();

   public:
    // Acquires an exclusive read lock on the metadata recorder which is held
    // until the object is destroyed.
    explicit MetadataProvider(MetadataRecorder* metadata_recorder,
                              PlatformThreadId thread_id)
        EXCLUSIVE_LOCK_FUNCTION(metadata_recorder_->read_lock_);
    ~MetadataProvider() UNLOCK_FUNCTION();
    MetadataProvider(const MetadataProvider&) = delete;
    MetadataProvider& operator=(const MetadataProvider&) = delete;

    // Retrieves the first |available_slots| items in the metadata recorder for
    // |thread_id| and copies them into |items|, returning the number of
    // metadata items that were copied. To ensure that all items can be copied,
    // |available slots| should be greater than or equal to
    // |MAX_METADATA_COUNT|. Requires NO_THREAD_SAFETY_ANALYSIS because clang's
    // analyzer doesn't understand the cross-class locking used in this class'
    // implementation.
    size_t GetItems(ItemArray* const items) const NO_THREAD_SAFETY_ANALYSIS;

   private:
    const MetadataRecorder* const metadata_recorder_;
    PlatformThreadId thread_id_;
    base::AutoLock auto_lock_;
  };

 private:
  // TODO(charliea): Support large quantities of metadata efficiently.
  struct ItemInternal {
    ItemInternal();
    ~ItemInternal();

    // Indicates whether the metadata item is still active (i.e. not removed).
    //
    // Requires atomic reads and writes to avoid word tearing when reading and
    // writing unsynchronized. Requires acquire/release semantics to ensure that
    // the other state in this struct is visible to the reading thread before it
    // is marked as active.
    std::atomic<bool> is_active{false};

    // Neither name_hash, key or thread_id require atomicity or memory order
    // constraints because no reader will attempt to read them mid-write.
    // Specifically, readers wait until |is_active| is true to read them.
    // Because |is_active| is always stored with a memory_order_release fence,
    // we're guaranteed that |name_hash|, |key| and |thread_id| will be finished
    // writing before |is_active| is set to true.
    uint64_t name_hash;
    std::optional<int64_t> key;
    std::optional<PlatformThreadId> thread_id;

    // Requires atomic reads and writes to avoid word tearing when updating an
    // existing item unsynchronized. Does not require acquire/release semantics
    // because we rely on the |is_active| acquire/release semantics to ensure
    // that an item is fully created before we attempt to read it.
    std::atomic<int64_t> value;
  };

  // Attempts to free slots in the metadata map that are currently allocated to
  // inactive items. May fail silently if the read lock is already held, in
  // which case no slots will be freed. Returns the number of item slots used
  // after the reclamation.
  size_t TryReclaimInactiveSlots(size_t item_slots_used)
      EXCLUSIVE_LOCKS_REQUIRED(write_lock_) LOCKS_EXCLUDED(read_lock_);
  // Updates item_slots_used_ to reflect the new item count and returns the
  // number of item slots used after the reclamation.
  size_t ReclaimInactiveSlots(size_t item_slots_used)
      EXCLUSIVE_LOCKS_REQUIRED(write_lock_)
          EXCLUSIVE_LOCKS_REQUIRED(read_lock_);

  // Retrieves items in the metadata recorder that are active for |thread_id|
  // and copies them into |items|, returning the number of metadata items that
  // were copied.
  size_t GetItems(ItemArray* const items, PlatformThreadId thread_id) const
      EXCLUSIVE_LOCKS_REQUIRED(read_lock_);

  // Metadata items that the recorder has seen. Rather than implementing the
  // metadata recorder as a dense array, we implement it as a sparse array where
  // removed metadata items keep their slot with their |is_active| bit set to
  // false. This avoids race conditions caused by reusing slots that might
  // otherwise cause mismatches between metadata name hashes and values.
  //
  // For the rationale behind this design (along with others considered), see
  // https://docs.google.com/document/d/18shLhVwuFbLl_jKZxCmOfRB98FmNHdKl0yZZZ3aEO4U/edit#.
  std::array<ItemInternal, MAX_METADATA_COUNT> items_;

  // The number of item slots used in the metadata map.
  //
  // Requires atomic reads and writes to avoid word tearing when reading and
  // writing unsynchronized. Requires acquire/release semantics to ensure that a
  // newly-allocated slot is fully initialized before the reader becomes aware
  // of its existence.
  std::atomic<size_t> item_slots_used_{0};

  // The number of item slots occupied by inactive items.
  size_t inactive_item_count_ GUARDED_BY(write_lock_) = 0;

  // A lock that guards against multiple threads trying to manipulate items_,
  // item_slots_used_, or inactive_item_count_ at the same time.
  base::Lock write_lock_;

  // A lock that guards against a reader trying to read items_ while inactive
  // slots are being reclaimed.
  base::Lock read_lock_;
};

}  // namespace base

#endif  // BASE_PROFILER_METADATA_RECORDER_H_
