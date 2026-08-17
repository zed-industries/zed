/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_MULTI_READER_QUEUE_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_MULTI_READER_QUEUE_H_

#include <deque>
#include <memory>
#include <optional>
#include <set>
#include <unordered_map>

#include "rtc_base/checks.h"

namespace webrtc {

// Represents the queue which can be read by multiple readers. Each reader reads
// from its own queue head. When an element is added it will become visible for
// all readers. When an element will be removed by all the readers, the element
// will be removed from the queue.
template <typename T>
class MultiReaderQueue {
 public:
  // Creates queue with exactly `readers_count` readers named from 0 to
  // `readers_count - 1`.
  explicit MultiReaderQueue(size_t readers_count) {
    for (size_t i = 0; i < readers_count; ++i) {
      heads_[i] = 0;
    }
  }
  // Creates queue with specified readers.
  explicit MultiReaderQueue(std::set<size_t> readers) {
    for (size_t reader : readers) {
      heads_[reader] = 0;
    }
  }

  // Adds a new `reader`, initializing its reading position (the reader's head)
  // equal to the one of `reader_to_copy`.
  // Complexity O(MultiReaderQueue::size(reader_to_copy)).
  void AddReader(size_t reader, size_t reader_to_copy) {
    size_t pos = GetHeadPositionOrDie(reader_to_copy);

    auto it = heads_.find(reader);
    RTC_CHECK(it == heads_.end())
        << "Reader " << reader << " is already in the queue";
    heads_[reader] = heads_[reader_to_copy];
    for (size_t i = pos; i < queue_.size(); ++i) {
      in_queues_[i]++;
    }
  }

  // Adds a new `reader`, initializing its reading position equal to first
  // element in the queue.
  // Complexity O(MultiReaderQueue::size()).
  void AddReader(size_t reader) {
    auto it = heads_.find(reader);
    RTC_CHECK(it == heads_.end())
        << "Reader " << reader << " is already in the queue";
    heads_[reader] = removed_elements_count_;
    for (size_t i = 0; i < queue_.size(); ++i) {
      in_queues_[i]++;
    }
  }

  // Removes specified `reader` from the queue.
  // Complexity O(MultiReaderQueue::size(reader)).
  void RemoveReader(size_t reader) {
    size_t pos = GetHeadPositionOrDie(reader);
    for (size_t i = pos; i < queue_.size(); ++i) {
      in_queues_[i]--;
    }
    while (!in_queues_.empty() && in_queues_[0] == 0) {
      PopFront();
    }
    heads_.erase(reader);
  }

  // Add value to the end of the queue. Complexity O(1).
  void PushBack(T value) {
    queue_.push_back(value);
    in_queues_.push_back(heads_.size());
  }

  // Extract element from specified head. Complexity O(1).
  std::optional<T> PopFront(size_t reader) {
    size_t pos = GetHeadPositionOrDie(reader);
    if (pos >= queue_.size()) {
      return std::nullopt;
    }

    T out = queue_[pos];

    in_queues_[pos]--;
    heads_[reader]++;

    if (in_queues_[pos] == 0) {
      RTC_DCHECK_EQ(pos, 0);
      PopFront();
    }
    return out;
  }

  // Returns element at specified head. Complexity O(1).
  std::optional<T> Front(size_t reader) const {
    size_t pos = GetHeadPositionOrDie(reader);
    if (pos >= queue_.size()) {
      return std::nullopt;
    }
    return queue_[pos];
  }

  // Returns true if for specified head there are no more elements in the queue
  // or false otherwise. Complexity O(1).
  bool IsEmpty(size_t reader) const {
    size_t pos = GetHeadPositionOrDie(reader);
    return pos >= queue_.size();
  }

  // Returns size of the longest queue between all readers.
  // Complexity O(1).
  size_t size() const { return queue_.size(); }

  // Returns size of the specified queue. Complexity O(1).
  size_t size(size_t reader) const {
    size_t pos = GetHeadPositionOrDie(reader);
    return queue_.size() - pos;
  }

  // Complexity O(1).
  size_t readers_count() const { return heads_.size(); }

 private:
  size_t GetHeadPositionOrDie(size_t reader) const {
    auto it = heads_.find(reader);
    RTC_CHECK(it != heads_.end()) << "No queue for reader " << reader;
    return it->second - removed_elements_count_;
  }

  void PopFront() {
    RTC_DCHECK(!queue_.empty());
    RTC_DCHECK_EQ(in_queues_[0], 0);
    queue_.pop_front();
    in_queues_.pop_front();
    removed_elements_count_++;
  }

  // Number of the elements that were removed from the queue. It is used to
  // subtract from each head to compute the right index inside `queue_`;
  size_t removed_elements_count_ = 0;
  std::deque<T> queue_;
  // In how may queues the element at index `i` is. An element can be removed
  // from the front if and only if it is in 0 queues.
  std::deque<size_t> in_queues_;
  // Map from the reader to the head position in the queue.
  std::unordered_map<size_t, size_t> heads_;
};

}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_MULTI_READER_QUEUE_H_
