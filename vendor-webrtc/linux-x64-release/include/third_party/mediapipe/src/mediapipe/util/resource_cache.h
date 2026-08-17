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

#ifndef MEDIAPIPE_UTIL_RESOURCE_CACHE_H_
#define MEDIAPIPE_UTIL_RESOURCE_CACHE_H_

#include <cstddef>
#include <unordered_map>

#include "absl/container/flat_hash_map.h"
#include "absl/functional/function_ref.h"
#include "absl/log/absl_check.h"
#include "mediapipe/framework/port/logging.h"

namespace mediapipe {

// Maintains a cache for resources of type `Value`, where the type of the
// resource (e.g., image dimension for an image pool) is described bye the `Key`
// type. The `Value` type must include an unset value, with implicit conversion
// to bool reflecting set/unset state.
template <typename Key, typename Value,
          typename KeyHash = typename absl::flat_hash_map<Key, int>::hasher>
class ResourceCache {
 public:
  Value Lookup(
      const Key& key,
      absl::FunctionRef<Value(const Key& key, int request_count)> create) {
    auto map_it = map_.find(key);
    Entry* entry;
    if (map_it == map_.end()) {
      std::tie(map_it, std::ignore) =
          map_.try_emplace(key, std::make_unique<Entry>(key));
      entry = map_it->second.get();
      ABSL_CHECK_EQ(entry->request_count, 0);
      entry->request_count = 1;
      entry_list_.Append(entry);
      if (entry->prev != nullptr) ABSL_CHECK_GE(entry->prev->request_count, 1);
    } else {
      entry = map_it->second.get();
      ++entry->request_count;
      Entry* larger = entry->prev;
      while (larger != nullptr &&
             larger->request_count < entry->request_count) {
        larger = larger->prev;
      }
      if (larger != entry->prev) {
        entry_list_.Remove(entry);
        entry_list_.InsertAfter(entry, larger);
      }
    }
    if (!entry->value) {
      entry->value = create(entry->key, entry->request_count);
    }
    ++total_request_count_;
    return entry->value;
  }

  std::vector<Value> Evict(int max_count, int request_count_scrub_interval) {
    std::vector<Value> evicted;

    // Remove excess entries.
    ABSL_CHECK_GE(max_count, 0);
    while (entry_list_.size() > static_cast<size_t>(max_count)) {
      Entry* victim = entry_list_.tail();
      evicted.emplace_back(std::move(victim->value));
      entry_list_.Remove(victim);
      map_.erase(victim->key);
    }
    // Every request_count_scrub_interval, halve the request counts, and
    // remove entries which have fallen to 0.
    // This keeps sporadic requests from accumulating and eventually exceeding
    // the minimum request threshold for allocating a pool. Also, it means that
    // if the request regimen changes (e.g. a graph was always requesting a
    // large size, but then switches to a small size to save memory or CPU), the
    // pool can quickly adapt to it.
    bool scrub = total_request_count_ >= request_count_scrub_interval;
    if (scrub) {
      total_request_count_ = 0;
      for (Entry* entry = entry_list_.head(); entry != nullptr;) {
        entry->request_count /= 2;
        Entry* next = entry->next;
        if (entry->request_count == 0) {
          evicted.emplace_back(std::move(entry->value));
          entry_list_.Remove(entry);
          map_.erase(entry->key);
        }
        entry = next;
      }
    }
    return evicted;
  }

 private:
  struct Entry {
    Entry(const Key& key) : key(key) {}
    Entry* prev = nullptr;
    Entry* next = nullptr;
    int request_count = 0;
    Key key;
    Value value;
  };

  // Unlike std::list, this is an intrusive list, meaning that the prev and next
  // pointers live inside the element. Apart from not requiring an extra
  // allocation, this means that once we look up an entry by key in the pools_
  // map we do not need to look it up separately in the list.
  //
  class EntryList {
   public:
    void Prepend(Entry* entry) {
      if (head_ == nullptr) {
        head_ = tail_ = entry;
      } else {
        entry->next = head_;
        head_->prev = entry;
        head_ = entry;
      }
      ++size_;
    }
    void Append(Entry* entry) {
      if (tail_ == nullptr) {
        head_ = tail_ = entry;
      } else {
        tail_->next = entry;
        entry->prev = tail_;
        tail_ = entry;
      }
      ++size_;
    }
    void Remove(Entry* entry) {
      if (entry == head_) {
        head_ = entry->next;
      } else {
        entry->prev->next = entry->next;
      }
      if (entry == tail_) {
        tail_ = entry->prev;
      } else {
        entry->next->prev = entry->prev;
      }
      entry->prev = nullptr;
      entry->next = nullptr;
      --size_;
    }
    void InsertAfter(Entry* entry, Entry* after) {
      if (after != nullptr) {
        entry->next = after->next;
        if (entry->next) entry->next->prev = entry;
        entry->prev = after;
        after->next = entry;
        ++size_;
      } else {
        Prepend(entry);
      }
    }

    Entry* head() { return head_; }
    Entry* tail() { return tail_; }
    size_t size() { return size_; }

   private:
    Entry* head_ = nullptr;
    Entry* tail_ = nullptr;
    size_t size_ = 0;
  };

  absl::flat_hash_map<Key, std::unique_ptr<Entry>, KeyHash> map_;
  EntryList entry_list_;
  int total_request_count_ = 0;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_RESOURCE_CACHE_H_
