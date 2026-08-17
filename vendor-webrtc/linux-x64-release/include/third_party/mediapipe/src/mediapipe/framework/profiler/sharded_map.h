// Copyright 2018 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_FRAMEWORK_PROFILER_SHARDED_MAP_H_
#define MEDIAPIPE_FRAMEWORK_PROFILER_SHARDED_MAP_H_

#include <stddef.h>

#include <unordered_map>
#include <vector>

#include "absl/synchronization/mutex.h"

// A thread-safe unordered map with locking at the key level.
template <typename Key, typename T, class Hash = std::hash<Key>>
class ShardedMap {
 public:
  using Map = std::unordered_map<Key, T, Hash>;
  using hasher = typename Map::hasher;
  using value_type = typename Map::value_type;
  template <typename ShardedMapPtr, class map_iterator>
  class Iterator;
  using iterator = Iterator<ShardedMap*, typename Map::iterator>;
  using const_iterator =
      Iterator<const ShardedMap*, typename Map::const_iterator>;

  // Creates a ShardedMap to hold |size| elements in |num_shards| partitions.
  ShardedMap(size_t capacity, size_t num_shards)
      : maps_(num_shards, Map(capacity / num_shards)),
        mutexes_(num_shards),
        size_(0) {}

  // Creates a ShardedMap to hold approximately |size| elements.
  // Default capacity is 100, which avoids most lock contention.
  explicit ShardedMap(size_t capacity = 100)
      : ShardedMap(capacity, capacity / 10 + 1) {}

  // Returns the iterator to the entry for a key.
  inline iterator find(const Key& key) ABSL_NO_THREAD_SAFETY_ANALYSIS {
    size_t shard = Index(key);
    mutexes_[shard].Lock();
    typename Map::iterator iter = maps_[shard].find(key);
    if (iter == maps_[shard].end()) {
      mutexes_[shard].Unlock();
      return end();
    }
    return {shard, iter, this};
  }

  // Returns 1 if the container includes a certain key.
  inline size_t count(const Key& key) const {
    size_t shard = Index(key);
    absl::MutexLock lock(&mutexes_[shard]);
    return maps_[shard].count(key);
  }

  // Adds an entry to the map and returns the iterator to it.
  inline std::pair<iterator, bool> insert(const value_type& val)
      ABSL_NO_THREAD_SAFETY_ANALYSIS {
    size_t shard = Index(val.first);
    mutexes_[shard].Lock();
    std::pair<typename Map::iterator, bool> p = maps_[shard].insert(val);
    size_ += p.second ? 1 : 0;
    return {std::move(iterator{shard, p.first, this}), p.second};
  }

  // Removes the entry for an iterator.
  inline void erase(iterator& pos) {
    if (pos != end()) {
      auto next_iter = pos.iter_;
      next_iter++;
      maps_[pos.shard_].erase(pos.iter_);
      pos.iter_ = next_iter;
      pos.NextEntryShard();
      --size_;
    }
  }

  // The total count of entries.
  inline size_t size() const { return size_; }

  // Returns the iterator to the first element.
  inline iterator begin() ABSL_NO_THREAD_SAFETY_ANALYSIS {
    mutexes_[0].Lock();
    iterator result{0, maps_[0].begin(), this};
    result.NextEntryShard();
    return result;
  }

  // Returns the iterator after the last element.
  // The end() iterator doesn't belong to any shard.
  inline iterator end() {
    return iterator{maps_.size() - 1, maps_.back().end(), this};
  }

  inline const_iterator begin() const {
    mutexes_[0].Lock();
    const_iterator result{0, maps_[0].begin(), this};
    result.NextEntryShard();
    return result;
  }
  inline const_iterator end() const {
    return const_iterator{maps_.size() - 1, maps_.back().end(), this};
  }

  // The iterator across map entries.
  // The iterator keeps its shard locked until it is destroyed.
  template <typename ShardedMapPtr, class map_iterator>
  class Iterator {
   public:
    Iterator(Iterator&& other)
        : shard_(other.shard_), iter_(other.iter_), map_(other.map_) {
      other.map_ = nullptr;
    }
    ~Iterator() { Clear(); }
    Iterator& operator=(Iterator&& other) {
      Clear();
      shard_ = other.shard_, iter_ = other.iter_, map_ = other.map_;
      other.map_ = nullptr;
      return *this;
    }
    inline bool operator==(const Iterator& other) const {
      return shard_ == other.shard_ && iter_ == other.iter_;
    }
    inline bool operator!=(const Iterator& other) const {
      return !operator==(other);
    }
    inline typename std::iterator_traits<map_iterator>::reference operator*()
        const {
      return *iter_;
    }
    inline typename std::iterator_traits<map_iterator>::pointer operator->()
        const {
      return &(operator*());
    }
    inline void operator++() {
      iter_++;
      NextEntryShard();
    }

   private:
    Iterator(size_t shard, map_iterator iter, ShardedMapPtr map)
        : shard_(shard), iter_(iter), map_(map) {}
    // Releases all resources.
    inline void Clear() ABSL_NO_THREAD_SAFETY_ANALYSIS {
      if (!map_) return;
      bool is_end = (shard_ == map_->maps_.size() - 1 &&
                     iter_ == map_->maps_[shard_].end());
      if (!is_end) {
        map_->mutexes_[shard_].Unlock();
      }
      map_ = nullptr;
    }
    // Moves to the shard of the next entry.
    void NextEntryShard() ABSL_NO_THREAD_SAFETY_ANALYSIS {
      size_t last = map_->maps_.size() - 1;
      while (iter_ == map_->maps_[shard_].end() && shard_ < last) {
        map_->mutexes_[shard_].Unlock();
        shard_++;
        map_->mutexes_[shard_].Lock();
        iter_ = map_->maps_[shard_].begin();
      }
      if (iter_ == map_->maps_.back().end()) {
        map_->mutexes_[shard_].Unlock();
      }
    }
    size_t shard_;
    map_iterator iter_;
    ShardedMapPtr map_;
    friend ShardedMap;
  };

 private:
  // Returns the shard index for a key.
  inline size_t Index(const Key& key) const {
    return hasher{}(key) % maps_.size();
  }

  // One unordered map for each key shard.
  std::vector<Map> maps_;

  // One mutex for each key shard.
  mutable std::vector<absl::Mutex> mutexes_;

  // The total count of entries.
  std::atomic<int> size_;
};

#endif  // MEDIAPIPE_FRAMEWORK_PROFILER_SHARDED_MAP_H_
