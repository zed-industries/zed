// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PARKABLE_IMAGE_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PARKABLE_IMAGE_MANAGER_H_

#include "base/feature_list.h"
#include "base/no_destructor.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "base/trace_event/memory_dump_provider.h"
#include "third_party/blink/public/common/features.h"
#include "third_party/blink/renderer/platform/disk_data_allocator.h"
#include "third_party/blink/renderer/platform/graphics/parkable_image.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

class ParkableImageImpl;
class ParkableImage;

// Manages parkable images, which are used in blink::BitmapImage. Currently,
// only records metrics for this. In the future we will park eligible images
// to disk.
// Main Thread only.
class PLATFORM_EXPORT ParkableImageManager
    : public base::trace_event::MemoryDumpProvider {
 public:
  static ParkableImageManager& Instance();
  ~ParkableImageManager() override = default;

  bool OnMemoryDump(const base::trace_event::MemoryDumpArgs&,
                    base::trace_event::ProcessMemoryDump*) override;

  // Number of parked and unparked images.
  size_t Size() const LOCKS_EXCLUDED(lock_);

  static bool IsParkableImagesToDiskEnabled() {
    return base::FeatureList::IsEnabled(features::kParkableImagesToDisk);
  }

  void MaybeParkImagesForTesting() { MaybeParkImages(); }

 private:
  struct Statistics;

  friend class ParkableImage;
  friend class ParkableImageImpl;
  friend class base::NoDestructor<ParkableImageManager>;
  friend class ParkableImageBaseTest;

  ParkableImageManager();

  DiskDataAllocator& data_allocator() const;

  // Register and unregister a ParkableImage with the manager. ParkableImage
  // should call these when created/destructed.
  void Add(ParkableImageImpl* image) LOCKS_EXCLUDED(lock_);
  void Remove(ParkableImageImpl* image) LOCKS_EXCLUDED(lock_)
      EXCLUSIVE_LOCKS_REQUIRED(image->lock_);

  scoped_refptr<ParkableImageImpl> CreateParkableImage(size_t offset);
  void DestroyParkableImageOnMainThread(scoped_refptr<ParkableImageImpl> image)
      LOCKS_EXCLUDED(lock_);
  void DestroyParkableImage(scoped_refptr<ParkableImageImpl> image)
      LOCKS_EXCLUDED(lock_);

  bool IsRegistered(ParkableImageImpl* image) LOCKS_EXCLUDED(lock_)
      EXCLUSIVE_LOCKS_REQUIRED(image->lock_);
  // bool IsRegistered(ParkableImage* image) LOCKS_EXCLUDED(lock_)
  //     EXCLUSIVE_LOCKS_REQUIRED(image->impl_->lock_);

  void ScheduleDelayedParkingTaskIfNeeded() EXCLUSIVE_LOCKS_REQUIRED(lock_);
  void MaybeParkImages() LOCKS_EXCLUDED(lock_);

  Statistics ComputeStatistics() const EXCLUSIVE_LOCKS_REQUIRED(lock_);

  void RecordStatisticsAfter5Minutes() const LOCKS_EXCLUDED(lock_);

  void MoveImage(ParkableImageImpl* image,
                 WTF::HashSet<ParkableImageImpl*>* from,
                 WTF::HashSet<ParkableImageImpl*>* to)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  void RecordDiskWriteTime(base::TimeDelta write_time) LOCKS_EXCLUDED(lock_) {
    base::AutoLock lock(lock_);
    total_disk_write_time_ += write_time;
  }

  void RecordDiskReadTime(base::TimeDelta read_time) LOCKS_EXCLUDED(lock_) {
    base::AutoLock lock(lock_);
    total_disk_read_time_ += read_time;
  }

  // Keeps track of whether the image is unparked or on disk. ParkableImage
  // should call these when written to or read from disk.
  void OnWrittenToDisk(ParkableImageImpl* image) LOCKS_EXCLUDED(lock_);
  void OnReadFromDisk(ParkableImageImpl* image) LOCKS_EXCLUDED(lock_);

  void SetDataAllocatorForTesting(
      std::unique_ptr<DiskDataAllocator> allocator) {
    allocator_for_testing_ = std::move(allocator);
  }

  void SetTaskRunnerForTesting(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  void ResetForTesting();
  constexpr static auto kDelayedParkingInterval = base::Seconds(2);
  constexpr static const char* kAllocatorDumpName = "parkable_images";

  mutable base::Lock lock_;

  // The following two sets are used to keep track of all ParkableImages that
  // have been created. ParkableImages are added to |unparked_images_| upon
  // creation, and removed from whichever set they are in at the time of their
  // destruction.
  //
  // Parking or Unparking a ParkableImage moves the image to the appropriate
  // set, using |OnReadFromDisk| and |OnWrittenToDisk|.
  //
  // |unparked_images_| keeps track of all images that have a in-memory
  // representation.
  //
  // |on_disk_images_| keeps track of all images that do not have an in-memory
  // representation. Accessing the data for any image in |on_disk_images_|
  // involves a read from disk.
  WTF::HashSet<ParkableImageImpl*> unparked_images_ GUARDED_BY(lock_);
  WTF::HashSet<ParkableImageImpl*> on_disk_images_ GUARDED_BY(lock_);

  bool has_pending_parking_task_ GUARDED_BY(lock_) = false;
  bool has_posted_accounting_task_ GUARDED_BY(lock_) = false;

  base::TimeDelta total_disk_read_time_ GUARDED_BY(lock_) = base::TimeDelta();
  base::TimeDelta total_disk_write_time_ GUARDED_BY(lock_) = base::TimeDelta();

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  std::unique_ptr<DiskDataAllocator> allocator_for_testing_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PARKABLE_IMAGE_MANAGER_H_
