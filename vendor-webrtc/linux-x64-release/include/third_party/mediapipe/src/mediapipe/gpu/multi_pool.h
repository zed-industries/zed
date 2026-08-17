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

#ifndef MEDIAPIPE_GPU_MULTI_POOL_H_
#define MEDIAPIPE_GPU_MULTI_POOL_H_

#include <functional>
#include <memory>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/status/statusor.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/port/status_macros.h"
#include "mediapipe/util/resource_cache.h"

namespace mediapipe {

struct MultiPoolOptions {
  // Keep this many buffers allocated for a given frame size.
  int keep_count = 2;
  // The maximum size of a concrete MultiPool. When the limit is reached, the
  // oldest BufferSpec will be dropped.
  int max_pool_count = 10;
  // Time in seconds after which an inactive buffer can be dropped from the
  // pool. Currently only used with CVPixelBufferPool.
  float max_inactive_buffer_age = 0.25;
  // Skip allocating a buffer pool until at least this many requests have been
  // made for a given BufferSpec.
  int min_requests_before_pool = 2;
  // Do a deeper flush every this many requests.
  int request_count_scrub_interval = 50;
};

static constexpr MultiPoolOptions kDefaultMultiPoolOptions;

// MultiPool is a generic class for vending reusable resources of type Item,
// which are assumed to be relatively expensive to create, so that reusing them
// is beneficial.
// Items are classified by Spec; when an item with a given Spec is requested,
// an old Item with the same Spec can be reused, if available; otherwise a new
// Item will be created. When user code is done with an Item, it is returned
// to the pool for reuse.
// In order to manage this, a MultiPool contains a map of Specs to SimplePool;
// each SimplePool manages Items with the same Spec, which are thus considered
// interchangeable.
// Item retention and eviction policies are controlled by options.
// A concrete example would be a pool of GlTextureBuffer, grouped by dimensions
// and format.
template <class SimplePool, class Spec, class Item>
class MultiPool {
 public:
  using SimplePoolFactory = std::function<std::shared_ptr<SimplePool>(
      const Spec& spec, const MultiPoolOptions& options)>;

  explicit MultiPool(SimplePoolFactory factory = DefaultMakeSimplePool,
                     MultiPoolOptions options = kDefaultMultiPoolOptions)
      : create_simple_pool_(factory), options_(options) {}
  explicit MultiPool(MultiPoolOptions options)
      : MultiPool(DefaultMakeSimplePool, options) {}

  // Obtains an item. May either be reused or created anew.
  absl::StatusOr<Item> Get(const Spec& spec);

 private:
  static std::shared_ptr<SimplePool> DefaultMakeSimplePool(
      const Spec& spec, const MultiPoolOptions& options) {
    return SimplePool::Create(spec, options);
  }

  // Requests a simple buffer pool for the given spec. This may return nullptr
  // if we have not yet reached a sufficient number of requests to allocate a
  // pool, in which case the caller should invoke CreateBufferWithoutPool.
  std::shared_ptr<SimplePool> RequestPool(const Spec& spec);

  absl::Mutex mutex_;
  mediapipe::ResourceCache<Spec, std::shared_ptr<SimplePool>> cache_
      ABSL_GUARDED_BY(mutex_);
  SimplePoolFactory create_simple_pool_ = DefaultMakeSimplePool;
  MultiPoolOptions options_;
};

template <class SimplePool, class Spec, class Item>
std::shared_ptr<SimplePool> MultiPool<SimplePool, Spec, Item>::RequestPool(
    const Spec& spec) {
  std::shared_ptr<SimplePool> pool;
  std::vector<std::shared_ptr<SimplePool>> evicted;
  {
    absl::MutexLock lock(&mutex_);
    pool = cache_.Lookup(spec, [this](const Spec& spec, int request_count) {
      return (request_count >= options_.min_requests_before_pool)
                 ? create_simple_pool_(spec, options_)
                 : nullptr;
    });
    evicted = cache_.Evict(options_.max_pool_count,
                           options_.request_count_scrub_interval);
  }
  // Evicted pools, and their buffers, will be released without holding the
  // lock.
  return pool;
}

template <class SimplePool, class Spec, class Item>
absl::StatusOr<Item> MultiPool<SimplePool, Spec, Item>::Get(const Spec& spec) {
  std::shared_ptr<SimplePool> pool = RequestPool(spec);
  if (pool) {
    // Note: we release our multipool lock before accessing the simple pool.
    MP_ASSIGN_OR_RETURN(auto item, pool->GetBuffer());
    return Item(std::move(item));
  }
  MP_ASSIGN_OR_RETURN(auto item, SimplePool::CreateBufferWithoutPool(spec));
  return Item(std::move(item));
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_MULTI_POOL_H_
