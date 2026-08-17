/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Find the top N elements of an incrementally provided stream of elements.
//
// BatchTopN is optimized for the use case that the top N elements are requested
// once at the end of the stream or no more frequently than every N log2(N)
// elements.
//

//
// This implementation maintains a buffer of 2N elements and
// uses fast selection (std::nth_element) to discard N smaller elements at a
// time, which has constant average complexity per element. The buffered
// elements are sorted when the top N elements are requested. This outperforms a
// heap-based algorithm if the top N elements are requested infrequently.
//
// The top N elements are obtained by the GetTopElements() method. It returns a
// "Result" object which can be iterated to obtain the top elements. To save
// from a typically unnecessary copy, the Result object does *not* own its
// elements, rather it views buffered elements in the BatchTopN instance.
//
// Example use:
//   BatchTopN<int> topn(3);
//   topn.Push(0);
//   topn.Push(99);
//   topn.Push(1);
//   topn.Push(2);
//   topn.Push(3);
//   topn.Push(4);
//
//   for (int value : topn.GetTopElements()) {
//     // Successive iterates are value = 99, 4, 3.
//   }
//
// If you just want to copy the top elements into a vector, do
//   vector<int> top_elements;
//   topn.GetTopElements().CopyTo(&top_elements);

#ifndef AUDIO_DSP_BATCH_TOP_N_H_
#define AUDIO_DSP_BATCH_TOP_N_H_

#include <algorithm>
#include <functional>
#include <utility>
#include <vector>

#include "glog/logging.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// ElementType must be DefaultConstructible, MoveConstructible, and
// MoveAssignable. Comparator is an STL binary predicate (a function object).
// Note that Comparator is the "greater" predicate. If you use a "less"
// predicate, the results are the bottom N elements.
template <typename ElementType, class Comparator = std::greater<ElementType>>
class BatchTopN {
 public:
  // Constructor, where limit is the maximum number of top results to return.
  explicit BatchTopN(int limit): BatchTopN(limit, Comparator()) {}
  BatchTopN(int limit, const Comparator& comparator)
      : limit_(limit),
        max_size_(2 * limit),
        elements_(new ElementType[max_size_]),
        indices_(new int[max_size_]),
        index_comparator_(elements_, comparator) {
    ABSL_CHECK_GE(limit, 1);
    Reset();
  }

  ~BatchTopN() {
    delete [] elements_;
    delete [] indices_;
  }

  int limit() const { return limit_; }

  // Reset BatchTopN to initial state. Invalidates existing Result objects.
  void Reset() {
    current_size_ = 0;
    threshold_index_ = -1;
    for (int i = 0; i < max_size_; ++i) {  // std::iota is unavailable on iOS.
      indices_[i] = i;
    }
  }

  // Push an element. Invalidates existing Result objects.
  void Push(const ElementType& v) { PushImpl(v); }
  // Move overload of Push.
  void Push(ElementType&& v) { PushImpl(std::move(v)); }

  // Container class representing the result from GetTopElements(). Elements are
  // *not* owned by the Result object, but by the BatchTopN.
  //
  // Result has vector-like iterface. To iterate over elements do
  //   BatchTopN<T>::Result result = topn.GetTopElements();
  //   for (int i = 0; i < result.size(); ++i) {
  //     const T& element = result[i];
  //     ...
  //   }
  // or using a ranged for loop
  //   BatchTopN<T>::Result result = topn.GetTopElements();
  //   for (const T& element : result) {
  //     ...
  //   }
  // or just
  //   for (const T& element : topn.GetTopElements()) {
  //     ...
  //   }
  class Result {
   public:
    class Iterator;
    class ConstIterator;
    typedef ElementType value_type;
    typedef Iterator iterator;
    typedef ConstIterator const_iterator;

    Result(ElementType* elements, const int* indices, int size)
        : elements_(elements), indices_(indices), size_(size) {}

    int size() const { return size_; }
    bool empty() const { return size_ == 0; }

    const ElementType& operator[](int i) const {
      ABSL_DCHECK_GE(i, 0);
      ABSL_DCHECK_LT(i, size_);
      return elements_[indices_[i]];
    }
    ElementType& operator[](int i) {
      ABSL_DCHECK_GE(i, 0);
      ABSL_DCHECK_LT(i, size_);
      return elements_[indices_[i]];
    }

    // Copy elements to a vector [or other container having an assign() method].
    template <typename ContainerType>
    void CopyTo(ContainerType* output) const {
      output->assign(begin(), end());
    }

    // Define an iterator yielding ElementType. It is a mutable
    // "ForwardIterator" category iterator; it supports reading, writing, and
    // incrementing [http://en.cppreference.com/w/cpp/iterator]. This iterator
    // is sufficient for use in a ranged for loop.
    class Iterator
        : public std::iterator<std::forward_iterator_tag, ElementType> {
     public:
      Iterator(ElementType* elements, const int* index)
          : elements_(elements), index_(index) {}

      ElementType& operator*() const { return elements_[*index_]; }
      ElementType* operator->() const { return &(operator*()); }

      Iterator& operator++() {
        ++index_;
        return *this;
      }

      Iterator operator++(int /* postincrement */) {
        Iterator old(*this);
        ++(*this);
        return old;
      }

      bool operator==(const Iterator& rhs) const {
        return index_ == rhs.index_;
      }

      bool operator!=(const Iterator& rhs) const {
        return index_ != rhs.index_;
      }

     private:
      ElementType* elements_;  // Not owned.
      const int* index_;  // Represents iterator position in the indices_ array.
    };

    // Define iterator again, but with const-ness.
    class ConstIterator
        : public std::iterator<std::forward_iterator_tag, ElementType> {
     public:
      ConstIterator(const ElementType* elements, const int* index)
          : elements_(elements), index_(index) {}

      const ElementType& operator*() const { return elements_[*index_]; }
      const ElementType* operator->() const { return &(operator*()); }

      ConstIterator& operator++() {
        ++index_;
        return *this;
      }

      ConstIterator operator++(int /* postincrement */) {
        ConstIterator old(*this);
        ++(*this);
        return old;
      }

      bool operator==(const ConstIterator& rhs) const {
        return index_ == rhs.index_;
      }

      bool operator!=(const ConstIterator& rhs) const {
        return index_ != rhs.index_;
      }

     private:
      const ElementType* elements_;  // Not owned.
      const int* index_;  // Represents iterator position in the indices_ array.
    };

    Iterator begin() { return Iterator(elements_, indices_); }
    Iterator end() { return Iterator(elements_, indices_ + size_); }
    ConstIterator begin() const { return ConstIterator(elements_, indices_); }
    ConstIterator end() const {
      return ConstIterator(elements_, indices_ + size_);
    }

   private:
    ElementType* elements_;  // Not owned.
    const int* indices_;  // Not owned.
    int size_;
  };

  // Get the top elements sorted in descending order. The Result return value is
  // a container of the sorted top elements.
  // NOTE: Result does *not* own its elements. Result remains valid until
  // Reset() or Push() are called or when the BatchTopN is destructed.
  Result GetTopElements() {
    // Since current_size_ may be larger than limit, we could consider calling
    // Prune() first or using std::partial_sort instead of performing a full
    // sort. However, since current_size_ is no more than 2 * limit, a full sort
    // is usually fastest. Benchmarks support this.
    std::sort(indices_, indices_ + current_size_, index_comparator_);
    if (current_size_ >= limit_) {
      current_size_ = limit_;
      threshold_index_ = indices_[limit_ - 1];
    }
    return Result(elements_, indices_, current_size_);
  }

 private:
  template <typename PushType>
  void PushImpl(PushType&& v) {
    // Only consider a new element if it is greater than
    // elements_[threshold_index], or if no threshold has been set yet.
    if (threshold_index_ < 0 || index_comparator_.value_comparator(
        v, elements_[threshold_index_])) {
      ABSL_DCHECK_LT(current_size_, max_size_);
      elements_[indices_[current_size_++]] = std::forward<PushType>(v);

      if (current_size_ >= max_size_) {
        // Prune to the larger limit elements.
        std::nth_element(indices_,
                         indices_ + limit_ - 1,
                         indices_ + current_size_,
                         index_comparator_);
        current_size_ = limit_;
        threshold_index_ = indices_[limit_ - 1];
      } else if (current_size_ == limit_) {
        // This happens once after the first limit elements of the stream. Find
        // the index of the smallest element seen so far.
        threshold_index_ = 0;
        for (int i = 1; i < limit_; ++i) {
          if (index_comparator_.value_comparator(
              elements_[threshold_index_], elements_[i])) {
            threshold_index_ = i;
          }
        }
      }
    }
  }

  struct IndexComparator {
    IndexComparator(const ElementType* elements_in,
                    const Comparator& value_comparator_in)
        : elements(elements_in), value_comparator(value_comparator_in) {}

    bool operator()(int i, int j) const {
      return value_comparator(elements[i], elements[j]);
    }

    const ElementType* elements;  // Not owned.
    Comparator value_comparator;
  };

  int limit_;  // Maximum number of top elements to return.
  int max_size_;  // Buffer size, the maximum number of elements to store.

  // Rather than performing reordering operations std::nth_element and std::sort
  // on ElementType values directly, we maintain a buffer of int indices and
  // reorder those instead. For non-POD ElementType, swap or move probably costs
  // at least a couple integer and pointer assignments. These buffers never
  // reallocate after construction, so we use raw pointers.
  ElementType* elements_;  // Dynamic array of size max_size_.
  int* indices_;  // Dynamic array of size max_size_.

  // Comparator for reordering indices_.
  IndexComparator index_comparator_;

  // Push() only considers a new element if it is greater than
  // elements_[threshold_index]. A value of -1 means no threshold is set.
  int threshold_index_;
  int current_size_;  // Current number of elements in the buffer.

  BatchTopN(const BatchTopN&) = delete;
  BatchTopN& operator=(const BatchTopN&) = delete;
};

namespace top_n_peaks_internal {

// We extract the largest N peaks from container. Vectorized operations can be
// used across the majority of container to find peak locations, but we
// must check the endpoints separately. Returns the indices of the maxima.
template <typename ContainerType, bool periodic>
std::vector<int> TopNPeaks(int N, const ContainerType& container) {
  const int size = static_cast<int>(container.size());
  if (container.size() < 2) {
    return {};  // Empty or single-element container has no peaks.
  }

  // Find the biggest peaks in container.
  auto comparator = [&container](int i, int j) {
    return container[i] > container[j];
  };
  BatchTopN<int, decltype(comparator)> top_n(N, comparator);

  for (int i = 1; i < size - 1; ++i) {
    if (container[i] > container[i - 1] && container[i] >= container[i + 1]) {
      // Compensate for the edges of the array that are not checked.
      top_n.Push(i);
    }
  }

  // Check the endpoints to see if they are a relative maxima.
  if (periodic) {
    if (container[0] > container[1] &&
        container[0] >= container[size - 1]) {
      top_n.Push(0);
    }
    if (container[size - 1] > container[size - 2] &&
        container[size - 1] >= container[0]) {
      top_n.Push(size - 1);
    }
  } else {
    if (container[0] > container[1]) {
      top_n.Push(0);
    }
    if (container[size - 1] > container[size - 2]) {
      top_n.Push(size - 1);
    }
  }
  std::vector<int> top_peaks;
  top_n.GetTopElements().CopyTo(&top_peaks);
  return top_peaks;  // Return by value; NRVO optimizes away the extra copy.
}
}  // namespace top_n_peaks_internal

// Find the N largest peaks in an array-like type. Peaks are defined as points
// that are strictly greater than their left point and greater than equal to
// their right point. This asymmetry is still a floating point compare and
// shouldn't make much difference in most applications. Fewer than N peaks may
// be returned if there are not N peaks in container.
// container must have .size() and operator[i] methods defined.

// In TopNPeaksNonperiodic, endpoints are considered peaks if they are greater
// than the only point adjacent to them.
template <typename ContainerType>
std::vector<int> TopNPeaksNonperiodic(
    int N, const ContainerType& container) {
  return top_n_peaks_internal::TopNPeaks<ContainerType, false>(N, container);
}

// Same as above but with periodic boundary conditions.
template <typename ContainerType>
std::vector<int> TopNPeaksPeriodic(
    int N, const ContainerType& container) {
  return top_n_peaks_internal::TopNPeaks<ContainerType, true>(N, container);
}
}  // namespace audio_dsp

#endif  // AUDIO_DSP_BATCH_TOP_N_H_
