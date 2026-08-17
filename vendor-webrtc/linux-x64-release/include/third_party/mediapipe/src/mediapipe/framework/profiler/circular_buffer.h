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

#ifndef MEDIAPIPE_FRAMEWORK_PROFILER_CIRCULAR_BUFFER_H_
#define MEDIAPIPE_FRAMEWORK_PROFILER_CIRCULAR_BUFFER_H_

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <iterator>
#include <vector>

namespace mediapipe {

// A circular buffer for lock-free event logging.
// This class is thread-safe and writing using "push_back" is lock-free.
// Multiple writers and readers are supported.  All writes and reads
// will succeed as long as the buffer does not grow by more than
// "buffer_margin_" during a read.
template <typename T>
class CircularBuffer {
 public:
  class iterator;

  // Create a circular buffer to hold up to |capacity| events.
  // Buffer writers are separated from readers by |buffer_margin|.
  CircularBuffer(size_t capacity, double buffer_margin = 0.25);

  // Appends one event to the buffer.
  // Returns true if the buffer is free and writing succeeds.
  inline bool push_back(const T& event);

  // Returns the i-th event in the buffer from the current beginning location.
  // Reading blocks until buffer is free.
  inline T Get(size_t i) const;

  // Returns the i-th event in the absolute buffer coordinates. Wrapping from
  // the beginning must be implemented separately.
  // Reading blocks until buffer is free.
  inline T GetAbsolute(size_t i) const;

  // Returns the first available index in the buffer.
  inline iterator begin() const {
    return iterator(this, current_ < capacity_ ? 0 : current_ - capacity_);
  }

  // Returns one past the last available index in the buffer.
  inline iterator end() const { return iterator(this, current_); }

 private:
  // Marks an atom busy and returns its previous value.
  static inline char AcquireForWrite(std::atomic_char& atom);

  // After an atom reaches |lap|, marks it busy and returns its previous value.
  static inline char AcquireForRead(std::atomic_char& atom, char lap);

  // Marks an atom as not busy at |lap|.
  static inline void Release(std::atomic_char& atom, char lap);

  // Returns the modulo lap for a buffer index.
  static inline char GetLap(size_t i, size_t buffer_size);

  // Returns the greater of two modulo laps.
  static inline char MaxLap(char u, char v);

 private:
  double buffer_margin_;
  size_t capacity_;
  size_t buffer_size_;
  std::vector<T> buffer_;
  mutable std::vector<std::atomic_char> lap_;
  std::atomic<size_t> current_;
  static constexpr char kBusy = 0xFF;
  static constexpr char kMask = 0x7F;
};

template <typename T>
CircularBuffer<T>::CircularBuffer(size_t capacity, double buffer_margin)
    : capacity_(capacity),
      buffer_size_((size_t)capacity * (1 + buffer_margin)),
      buffer_(buffer_size_),
      lap_(buffer_size_),
      current_(0) {}

template <typename T>
bool CircularBuffer<T>::push_back(const T& event) {
  size_t i = current_++;
  char lap = GetLap(i, buffer_size_);
  size_t index = i % buffer_size_;
  char prev = AcquireForWrite(lap_[index]);
  buffer_[index] = event;
  Release(lap_[index], MaxLap(prev, lap));
  return true;
}

template <typename T>
T CircularBuffer<T>::GetAbsolute(size_t i) const {
  char lap = GetLap(i, buffer_size_);
  size_t index = i % buffer_size_;
  char prev = AcquireForRead(lap_[index], lap);
  T result = buffer_[index];
  Release(lap_[index], prev);
  return result;
}

template <typename T>
T CircularBuffer<T>::Get(size_t i) const {
  if (current_ > capacity_) {
    return GetAbsolute(i + current_ - capacity_);
  } else {
    return GetAbsolute(i);
  }
}

template <typename T>
char CircularBuffer<T>::AcquireForWrite(std::atomic_char& atom) {
  char prev;
  for (bool done = false; !done;) {
    prev = atom;
    if (prev != kBusy) {
      done =
          atom.compare_exchange_strong(prev, kBusy, std::memory_order_acquire);
    }
  }
  return prev;
}

template <typename T>
char CircularBuffer<T>::AcquireForRead(std::atomic_char& atom, char lap) {
  char prev;
  for (bool done = false; !done;) {
    prev = atom;
    if (prev != kBusy && prev == MaxLap(prev, lap)) {
      done =
          atom.compare_exchange_strong(prev, kBusy, std::memory_order_acquire);
    }
  }
  return prev;
}

template <typename T>
void CircularBuffer<T>::Release(std::atomic_char& atom, char lap) {
  atom.store(lap, std::memory_order_release);
}

template <typename T>
char CircularBuffer<T>::GetLap(size_t index, size_t buffer_size) {
  return (index / buffer_size + 1) & kMask;
}

template <typename T>
char CircularBuffer<T>::MaxLap(char u, char v) {
  return ((u - v) & kMask) <= (kMask / 2) ? u : v;
}

template <typename T>
class CircularBuffer<T>::iterator
    : public std::iterator<std::random_access_iterator_tag, T, int64_t> {
 public:
  explicit iterator(const CircularBuffer* buffer, size_t index)
      : buffer_(buffer), index_(index) {}
  bool operator==(iterator other) const { return index_ == other.index_; }
  bool operator!=(iterator other) const { return !(*this == other); }
  bool operator<(iterator other) const { return index_ < other.index_; }
  T operator*() const { return buffer_->GetAbsolute(index_); }
  T* operator->() const { &buffer_->GetAbsolute(index_); }
  iterator& operator++() { return (*this) += 1; }
  iterator& operator+=(const int64_t& num) { return index_ += num, *this; }
  int64_t operator-(const iterator& it) const { return index_ - it.index_; }
  iterator& operator+(const int64_t& num) { return iterator(*this) += num; }
  iterator& operator-(const int64_t& num) { return iterator(*this) += -num; }

 private:
  const CircularBuffer* buffer_;
  size_t index_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_PROFILER_CIRCULAR_BUFFER_H_
