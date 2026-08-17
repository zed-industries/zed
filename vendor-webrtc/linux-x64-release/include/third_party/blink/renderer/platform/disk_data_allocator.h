// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_DISK_DATA_ALLOCATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_DISK_DATA_ALLOCATOR_H_

#include <map>
#include <memory>

#include "base/containers/span.h"
#include "base/dcheck_is_on.h"
#include "base/files/file.h"
#include "base/gtest_prod_util.h"
#include "base/synchronization/lock.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "third_party/blink/public/mojom/disk_allocator.mojom-blink.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/threading.h"

namespace blink {

class DiskDataMetadata;
class ReservedChunk;

// Stores data onto a single file.
//
// The file is provided after construction. As a consequence, the allocator
// initially does not accept writes, that is |Write()| returns nullptr. It may
// also become not usable later, for instance if disk space is no longer
// available.
//
// Threading:
// - Reads and writes can be done from any thread.
// - public methods are thread-safe, and unless otherwise noted, can be called
//   from any thread.
class PLATFORM_EXPORT DiskDataAllocator : public mojom::blink::DiskAllocator {
 public:

  // Must be called on the main thread.
  void ProvideTemporaryFile(::base::File file) override;

  // Whether writes may succeed. This is not a guarantee. However, when this
  // returns false, writes will fail.
  bool may_write() LOCKS_EXCLUDED(lock_);

  // Return valid |ReservedChunk| if success, otherwise nullptr.
  // It may fail to reserve if remaining free space is not enough for |size|.
  std::unique_ptr<ReservedChunk> TryReserveChunk(size_t size);

  // Returns |nullptr| in case of error.
  // Note that this performs a blocking disk write.
  std::unique_ptr<DiskDataMetadata> Write(std::unique_ptr<ReservedChunk> chunk,
                                          base::span<const uint8_t> data);

  // Reads data. A read failure is fatal.
  // Caller must make sure that this is not called at the same time as
  // |Discard()|.
  // Can be called at any time before |Discard()| destroys |metadata|.
  //
  // |data| must point to an area large enough to fit a |metadata.size|-ed
  // array. Note that this performs a blocking disk read.
  void Read(const DiskDataMetadata& metadata, base::span<uint8_t> data);

  // Discards existing data pointed at by |metadata|. Caller must make sure this
  // is not called while the same file is being read.
  void Discard(std::unique_ptr<DiskDataMetadata> metadata);

  ~DiskDataAllocator() override;
  static DiskDataAllocator& Instance();
  static void Bind(mojo::PendingReceiver<mojom::blink::DiskAllocator> receiver);

  int64_t disk_footprint() {
    base::AutoLock locker(lock_);
    return file_tail_;
  }

  size_t free_chunks_size() {
    base::AutoLock locker(lock_);
    return free_chunks_size_;
  }

  void set_may_write_for_testing(bool may_write) LOCKS_EXCLUDED(lock_);

 protected:
  // Protected methods for testing.
  DiskDataAllocator();

 private:
  DiskDataMetadata FindFreeChunk(size_t size) EXCLUSIVE_LOCKS_REQUIRED(lock_);
  void ReleaseChunk(const DiskDataMetadata& metadata)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Virtual for testing.
  virtual std::optional<size_t> DoWrite(int64_t offset,
                                        base::span<const uint8_t> data)
      LOCKS_EXCLUDED(lock_);
  // CHECK()s that the read is successful.
  virtual void DoRead(int64_t offset, base::span<uint8_t> data);

  mojo::Receiver<mojom::blink::DiskAllocator> receiver_{this};
  base::File file_;  // May be invalid.

  bool has_capacity_limit_ = false;
  size_t max_capacity_ = 0;

 protected:  // For testing.
  base::Lock lock_;
  std::map<int64_t, size_t> free_chunks_ GUARDED_BY(lock_)
      ALLOW_DISCOURAGED_TYPE("We rely on |{lower,upper}_bound()|");
  size_t free_chunks_size_ GUARDED_BY(lock_) = 0;

 private:
  int64_t file_tail_ GUARDED_BY(lock_) = 0;
  // Whether writing is possible now. This can be true if:
  // - |set_may_write_for_testing()| was called, or
  // - |file_.IsValid()| and no write error occurred (which would set
  //   |may_write_| to false).
  bool may_write_ GUARDED_BY(lock_) = false;
#if DCHECK_IS_ON()
  std::map<int64_t, size_t> allocated_chunks_ GUARDED_BY(lock_)
      ALLOW_DISCOURAGED_TYPE("We rely on |{lower,upper}_bound()|");
#endif

  FRIEND_TEST_ALL_PREFIXES(DiskDataAllocatorTest, ProvideInvalidFile);
  FRIEND_TEST_ALL_PREFIXES(DiskDataAllocatorTest, ProvideValidFile);
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_DISK_DATA_ALLOCATOR_H_
