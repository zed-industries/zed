// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Consider this file an implementation detail. None of this is part of the
// public API.

#ifndef MEDIAPIPE_GPU_REUSABLE_POOL_H_
#define MEDIAPIPE_GPU_REUSABLE_POOL_H_

#include <algorithm>
#include <cstddef>
#include <memory>
#include <utility>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/functional/any_invocable.h"
#include "absl/log/absl_log.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_format.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/framework/port/status_macros.h"
#include "mediapipe/gpu/multi_pool.h"

namespace mediapipe {

template <class Item>
class ReusablePool : public std::enable_shared_from_this<ReusablePool<Item>> {
 public:
  using ItemFactory =
      absl::AnyInvocable<absl::StatusOr<std::unique_ptr<Item>>() const>;

  // Creates a pool. This pool will manage buffers of the specified dimensions,
  // and will keep keep_count buffers around for reuse.
  // We enforce creation as a shared_ptr so that we can use a weak reference in
  // the buffers' deleters.
  static std::shared_ptr<ReusablePool<Item>> Create(
      ItemFactory item_factory, const MultiPoolOptions& options) {
    return std::shared_ptr<ReusablePool<Item>>(
        new ReusablePool<Item>(std::move(item_factory), options));
  }

  // Obtains a buffer. May either be reused or created anew.
  // A GlContext must be current when this is called.
  absl::StatusOr<std::shared_ptr<Item>> GetBuffer();

  // This method is meant for testing.
  std::pair<int, int> GetInUseAndAvailableCounts();

 protected:
  ReusablePool(ItemFactory item_factory, const MultiPoolOptions& options)
      : item_factory_(std::move(item_factory)),
        keep_count_(options.keep_count) {}

 private:
  // Return a buffer to the pool.
  void Return(std::unique_ptr<Item> buf);

  // If the total number of buffers is greater than keep_count, destroys any
  // surplus buffers that are no longer in use.
  void TrimAvailable(std::vector<std::unique_ptr<Item>>* trimmed)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  const ItemFactory item_factory_;
  const int keep_count_;

  absl::Mutex mutex_;
  int in_use_count_ ABSL_GUARDED_BY(mutex_) = 0;
  std::vector<std::unique_ptr<Item>> available_ ABSL_GUARDED_BY(mutex_);
};

template <class Item>
inline absl::StatusOr<std::shared_ptr<Item>> ReusablePool<Item>::GetBuffer() {
  std::unique_ptr<Item> buffer;
  bool reuse = false;

  {
    absl::MutexLock lock(&mutex_);
    if (available_.empty()) {
      MP_ASSIGN_OR_RETURN(buffer, item_factory_());
      RET_CHECK_NE(buffer, nullptr) << "Failed to create buffer";
    } else {
      buffer = std::move(available_.back());
      available_.pop_back();
      reuse = true;
    }

    ++in_use_count_;
  }

  // This needs to wait on consumer sync points, therefore it should not be
  // done while holding the mutex.
  if (reuse) {
    buffer->Reuse();
  }

  // Return a shared_ptr with a custom deleter that adds the buffer back
  // to our available list.
  std::weak_ptr<ReusablePool<Item>> weak_pool(this->shared_from_this());
  return std::shared_ptr<Item>(buffer.release(), [weak_pool](Item* buf) {
    auto pool = weak_pool.lock();
    if (pool) {
      pool->Return(absl::WrapUnique(buf));
    } else {
      delete buf;
    }
  });
}

template <class Item>
inline std::pair<int, int> ReusablePool<Item>::GetInUseAndAvailableCounts() {
  absl::MutexLock lock(&mutex_);
  return {in_use_count_, available_.size()};
}

template <class Item>
void ReusablePool<Item>::Return(std::unique_ptr<Item> buf) {
  std::vector<std::unique_ptr<Item>> trimmed;
  {
    absl::MutexLock lock(&mutex_);
    --in_use_count_;
    available_.emplace_back(std::move(buf));
    TrimAvailable(&trimmed);
  }
  // The trimmed buffers will be released without holding the lock.
}

template <class Item>
void ReusablePool<Item>::TrimAvailable(
    std::vector<std::unique_ptr<Item>>* trimmed) {
  size_t keep = std::max(keep_count_ - in_use_count_, 0);
  if (available_.size() > keep) {
    ABSL_LOG_FIRST_N(WARNING, 1) << absl::StrFormat(
        "Maximum number of pooled buffers reached (set to keep at most %d "
        "buffers of the same type and size). MP buffer allocation patterns can "
        "be observed with Perfetto by inspecting the "
        "PerfettoScopedMemoryObjectCounters-based GpuBuffer and AhwbBuffer "
        "counters. Then consider adjusting the MultiPoolOptions.keep_count",
        keep_count_);
    auto trim_it = std::next(available_.begin(), keep);
    if (trimmed) {
      std::move(trim_it, available_.end(), std::back_inserter(*trimmed));
    }
    available_.erase(trim_it, available_.end());
  }
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_REUSABLE_POOL_H_
