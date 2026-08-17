// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PARKABLE_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PARKABLE_IMAGE_H_

#include "base/containers/span.h"
#include "base/debug/stack_trace.h"
#include "base/feature_list.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "third_party/blink/public/platform/web_data.h"
#include "third_party/blink/renderer/platform/disk_data_metadata.h"
#include "third_party/blink/renderer/platform/image-decoders/rw_buffer.h"
#include "third_party/blink/renderer/platform/image-decoders/segment_reader.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"

namespace blink {

class SegmentReader;
class ParkableImageManager;
class ParkableImage;
class ParkableImageSegmentReader;

PLATFORM_EXPORT BASE_DECLARE_FEATURE(kDelayParkingImages);

// Implementation of ParkableImage. See ParkableImage below.
// We split ParkableImage like this because we want to avoid destroying the
// content of the ParkableImage on anything besides the main thread.
// See |ParkableImageManager::MaybeParkImages| for details on this.
class PLATFORM_EXPORT ParkableImageImpl final
    : public ThreadSafeRefCounted<ParkableImageImpl> {
 public:
  ParkableImageImpl& operator=(const ParkableImage&) = delete;
  ParkableImageImpl(const ParkableImage&) = delete;

  // Smallest encoded size that will actually be parked.
  static constexpr size_t kMinSizeToPark = 1024;  // 1 KiB
  // How long to wait before parking an image.
  //
  // Chosen arbitrarily, did not regress metrics in field trials in 2022. From
  // local experiments, images are typically only decoded once, to raster the
  // tile(s) they are a part of, then never used as long as the image decode
  // cache is not emptied and the tiles are not re-rasterized. This is set to
  // something longer than e.g. 1s in case there is a looping GIF for instance,
  // and/or the decoded image cache is too small.
  static constexpr base::TimeDelta kParkingDelay = base::Seconds(30);

 private:
  friend class ThreadSafeRefCounted<ParkableImageImpl>;
  template <typename T, typename... Args>
  friend scoped_refptr<T> base::MakeRefCounted(Args&&... args);
  friend class ParkableImageManager;
  friend class ParkableImage;
  friend class ParkableImageBaseTest;
  friend class ParkableImageSegmentReader;

  // |initial_capacity| reserves space in the internal buffer, if you know how
  // much data you'll be appending in advance.
  explicit ParkableImageImpl(size_t initial_capacity = 0);

  ~ParkableImageImpl();

  // Factory method to construct a ParkableImageImpl.
  static scoped_refptr<ParkableImageImpl> Create(size_t initial_capacity = 0);

  // Implementations of the methods of the same name from ParkableImage.
  void Freeze() LOCKS_EXCLUDED(lock_);
  void Append(WTF::SharedBuffer* buffer, size_t offset = 0)
      LOCKS_EXCLUDED(lock_);
  scoped_refptr<SharedBuffer> Data() LOCKS_EXCLUDED(lock_);
  void LockData() EXCLUSIVE_LOCKS_REQUIRED(lock_);
  void UnlockData() EXCLUSIVE_LOCKS_REQUIRED(lock_);
  size_t size() const;

  bool is_frozen() const { return !frozen_time_.is_null(); }

  bool ShouldReschedule() const LOCKS_EXCLUDED(lock_) {
    base::AutoLock lock(lock_);
    return TransientlyUnableToPark();
  }

  // Attempt to park to disk. Returns false if it cannot be parked right now for
  // whatever reason, true if we will _attempt_ to park it to disk.
  bool MaybePark(scoped_refptr<base::SingleThreadTaskRunner> task_runner)
      LOCKS_EXCLUDED(lock_);

  // Unpark the data from disk. This is blocking, on the same thread (since we
  // cannot expect to continue with anything that needs the data until we have
  // unparked it).
  void Unpark() EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Tries to write the data from |rw_buffer_| to disk. Then, if the data is
  // successfully written to disk, posts a task to discard |rw_buffer_|.
  static void WriteToDiskInBackground(
      scoped_refptr<ParkableImageImpl>,
      scoped_refptr<base::SingleThreadTaskRunner> callback_task_runner)
      LOCKS_EXCLUDED(lock_);

  // Writes the data referred to by |on_disk_metadata| from disk into the
  // provided |buffer|.
  static size_t ReadFromDiskIntoBuffer(DiskDataMetadata* on_disk_metadata,
                                       base::span<uint8_t> buffer);

  // Attempt to discard the data. This should only be called after we've written
  // the data to disk. Fails if the image can not be parked at the time this is
  // called for whatever reason.
  void MaybeDiscardData() LOCKS_EXCLUDED(lock_);

  // Discards the data in |rw_buffer_|. Caller is responsible for making sure
  // this is only called when the image can be parked.
  void DiscardData() EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Only larger images are parked, see kMinSizeToPark for the threshold used.
  bool is_below_min_parking_size() const;

  // Returns whether the ParkableImageImpl is locked or not. See |Lock| and
  // |Unlock| for details.
  bool is_locked() const EXCLUSIVE_LOCKS_REQUIRED(lock_);
  bool is_on_disk() const EXCLUSIVE_LOCKS_REQUIRED(lock_) {
    return !rw_buffer_ && on_disk_metadata_;
  }
  // Whether or not a failure of trying to park the image now would be
  // transient (e.g. due to not being frozen) or not.
  bool TransientlyUnableToPark() const EXCLUSIVE_LOCKS_REQUIRED(lock_);

  bool CanParkNow() const EXCLUSIVE_LOCKS_REQUIRED(lock_);

  mutable base::Lock lock_;

  std::unique_ptr<RWBuffer> rw_buffer_ GUARDED_BY(lock_);

  std::unique_ptr<ReservedChunk> reserved_chunk_ GUARDED_BY(lock_);
  // Non-null iff we have the data from |rw_buffer_| saved to disk.
  std::unique_ptr<DiskDataMetadata> on_disk_metadata_ GUARDED_BY(lock_);
  // |size_| is only modified on the main thread.
  size_t size_ = 0;
  // |frozen_time_| is only modified on the main thread. |frozen_time_| is the
  // time we've frozen the ParkableImage, or a null value if it's not yet
  // frozen.
  base::TimeTicks frozen_time_;
  // Counts the number of Lock/Unlock calls. Incremented by Lock, decremented by
  // Unlock. The ParkableImageImpl is unlocked iff |lock_depth_| is 0, i.e.
  // we've called Lock and Unlock the same number of times.
  size_t lock_depth_ GUARDED_BY(lock_) = 0;
  bool background_task_in_progress_ GUARDED_BY(lock_) = false;
  bool used_ GUARDED_BY(lock_) = false;

  THREAD_CHECKER(thread_checker_);
};

// Wraps a RWBuffer containing encoded image data. This buffer can be written
// to/read from disk when not needed, to improve memory usage.
class PLATFORM_EXPORT ParkableImage final
    : public ThreadSafeRefCounted<ParkableImage> {
 public:
  // Factory method to construct a ParkableImage.
  static scoped_refptr<ParkableImage> Create(size_t initial_capacity = 0);

  // Creates a read-only snapshot of the ParkableImage. This can be used
  // from other threads.
  scoped_refptr<SegmentReader> MakeROSnapshot();

  // Freezes the ParkableImage. This changes the following:
  // (1) We are no longer allowed to mutate the internal buffer (e.g. via
  // Append);
  // (2) The image may now be parked to disk.
  void Freeze() LOCKS_EXCLUDED(impl_->lock_);

  // Appends data to the ParkableImage. Cannot be called after the ParkableImage
  // has been frozen. (see: Freeze())
  void Append(WTF::SharedBuffer* buffer, size_t offset = 0)
      LOCKS_EXCLUDED(impl_->lock_);

  // Returns a copy of the data stored in ParkableImage. Calling this will
  // unpark the image from disk if needed.
  scoped_refptr<SharedBuffer> Data() LOCKS_EXCLUDED(impl_->lock_);

  // Returns the size of the encoded image data stored in the ParkableImage. Can
  // be called even if the image is currently parked, and will not unpark it.
  size_t size() const;

  scoped_refptr<SegmentReader> CreateSegmentReader();

 private:
  friend class ThreadSafeRefCounted<ParkableImage>;
  template <typename T, typename... Args>
  friend scoped_refptr<T> base::MakeRefCounted(Args&&... args);
  friend class ParkableImageManager;
  friend class ParkableImageBaseTest;
  friend class ParkableImageSegmentReader;
  friend class ThreadSafeRefCounted<ParkableImageImpl>;

  explicit ParkableImage(size_t initial_capacity = 0);
  ~ParkableImage();

  // Locks and Unlocks the ParkableImageImpl. A locked ParkableImage cannot be
  // parked. Every call to Lock must have a corresponding call to Unlock.
  void LockData() EXCLUSIVE_LOCKS_REQUIRED(impl_->lock_);
  void UnlockData() EXCLUSIVE_LOCKS_REQUIRED(impl_->lock_);

  bool is_on_disk() const EXCLUSIVE_LOCKS_REQUIRED(impl_->lock_);

  scoped_refptr<ParkableImageImpl> impl_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PARKABLE_IMAGE_H_
