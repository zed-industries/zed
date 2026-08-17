/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SWAP_QUEUE_H_
#define RTC_BASE_SWAP_QUEUE_H_

#include <stddef.h>

#include <atomic>
#include <utility>
#include <vector>

#include "absl/base/attributes.h"
#include "rtc_base/checks.h"

namespace webrtc {

namespace internal {

// (Internal; please don't use outside this file.)
template <typename T>
bool NoopSwapQueueItemVerifierFunction(const T&) {
  return true;
}

}  // namespace internal

// Functor to use when supplying a verifier function for the queue.
template <typename T,
          bool (*QueueItemVerifierFunction)(const T&) =
              internal::NoopSwapQueueItemVerifierFunction>
class SwapQueueItemVerifier {
 public:
  bool operator()(const T& t) const { return QueueItemVerifierFunction(t); }
};

// This class is a fixed-size queue. A single producer calls Insert() to insert
// an element of type T at the back of the queue, and a single consumer calls
// Remove() to remove an element from the front of the queue. It's safe for the
// producer and the consumer to access the queue concurrently, from different
// threads.
//
// To avoid the construction, copying, and destruction of Ts that a naive
// queue implementation would require, for each "full" T passed from
// producer to consumer, SwapQueue<T> passes an "empty" T in the other
// direction (an "empty" T is one that contains nothing of value for the
// consumer). This bidirectional movement is implemented with swap().
//
// // Create queue:
// Bottle proto(568);  // Prepare an empty Bottle. Heap allocates space for
//                     // 568 ml.
// SwapQueue<Bottle> q(N, proto);  // Init queue with N copies of proto.
//                                 // Each copy allocates on the heap.
// // Producer pseudo-code:
// Bottle b(568); // Prepare an empty Bottle. Heap allocates space for 568 ml.
// loop {
//   b.Fill(amount);  // Where amount <= 568 ml.
//   q.Insert(&b);    // Swap our full Bottle for an empty one from q.
// }
//
// // Consumer pseudo-code:
// Bottle b(568);  // Prepare an empty Bottle. Heap allocates space for 568 ml.
// loop {
//   q.Remove(&b); // Swap our empty Bottle for the next-in-line full Bottle.
//   Drink(&b);
// }
//
// For a well-behaved Bottle class, there are no allocations in the
// producer, since it just fills an empty Bottle that's already large
// enough; no deallocations in the consumer, since it returns each empty
// Bottle to the queue after having drunk it; and no copies along the
// way, since the queue uses swap() everywhere to move full Bottles in
// one direction and empty ones in the other.
template <typename T, typename QueueItemVerifier = SwapQueueItemVerifier<T>>
class SwapQueue {
 public:
  // Creates a queue of size size and fills it with default constructed Ts.
  explicit SwapQueue(size_t size) : queue_(size) {
    RTC_DCHECK(VerifyQueueSlots());
  }

  // Same as above and accepts an item verification functor.
  SwapQueue(size_t size, const QueueItemVerifier& queue_item_verifier)
      : queue_item_verifier_(queue_item_verifier), queue_(size) {
    RTC_DCHECK(VerifyQueueSlots());
  }

  // Creates a queue of size size and fills it with copies of prototype.
  SwapQueue(size_t size, const T& prototype) : queue_(size, prototype) {
    RTC_DCHECK(VerifyQueueSlots());
  }

  // Same as above and accepts an item verification functor.
  SwapQueue(size_t size,
            const T& prototype,
            const QueueItemVerifier& queue_item_verifier)
      : queue_item_verifier_(queue_item_verifier), queue_(size, prototype) {
    RTC_DCHECK(VerifyQueueSlots());
  }

  // Resets the queue to have zero content while maintaining the queue size.
  // Just like Remove(), this can only be called (safely) from the
  // consumer.
  void Clear() {
    // Drop all non-empty elements by resetting num_elements_ and incrementing
    // next_read_index_ by the previous value of num_elements_. Relaxed memory
    // ordering is sufficient since the dropped elements are not accessed.
    next_read_index_ += std::atomic_exchange_explicit(
        &num_elements_, size_t{0}, std::memory_order_relaxed);
    if (next_read_index_ >= queue_.size()) {
      next_read_index_ -= queue_.size();
    }

    RTC_DCHECK_LT(next_read_index_, queue_.size());
  }

  // Inserts a "full" T at the back of the queue by swapping *input with an
  // "empty" T from the queue.
  // Returns true if the item was inserted or false if not (the queue was full).
  // When specified, the T given in *input must pass the ItemVerifier() test.
  // The contents of *input after the call are then also guaranteed to pass the
  // ItemVerifier() test.
  ABSL_MUST_USE_RESULT bool Insert(T* input) {
    RTC_DCHECK(input);

    RTC_DCHECK(queue_item_verifier_(*input));

    // Load the value of num_elements_. Acquire memory ordering prevents reads
    // and writes to queue_[next_write_index_] to be reordered to before the
    // load. (That element might be accessed by a concurrent call to Remove()
    // until the load finishes.)
    if (std::atomic_load_explicit(&num_elements_, std::memory_order_acquire) ==
        queue_.size()) {
      return false;
    }

    using std::swap;
    swap(*input, queue_[next_write_index_]);

    // Increment the value of num_elements_ to account for the inserted element.
    // Release memory ordering prevents the reads and writes to
    // queue_[next_write_index_] to be reordered to after the increment. (Once
    // the increment has finished, Remove() might start accessing that element.)
    const size_t old_num_elements = std::atomic_fetch_add_explicit(
        &num_elements_, size_t{1}, std::memory_order_release);

    ++next_write_index_;
    if (next_write_index_ == queue_.size()) {
      next_write_index_ = 0;
    }

    RTC_DCHECK_LT(next_write_index_, queue_.size());
    RTC_DCHECK_LT(old_num_elements, queue_.size());

    return true;
  }

  // Removes the frontmost "full" T from the queue by swapping it with
  // the "empty" T in *output.
  // Returns true if an item could be removed or false if not (the queue was
  // empty). When specified, The T given in *output must pass the ItemVerifier()
  // test and the contents of *output after the call are then also guaranteed to
  // pass the ItemVerifier() test.
  ABSL_MUST_USE_RESULT bool Remove(T* output) {
    RTC_DCHECK(output);

    RTC_DCHECK(queue_item_verifier_(*output));

    // Load the value of num_elements_. Acquire memory ordering prevents reads
    // and writes to queue_[next_read_index_] to be reordered to before the
    // load. (That element might be accessed by a concurrent call to Insert()
    // until the load finishes.)
    if (std::atomic_load_explicit(&num_elements_, std::memory_order_acquire) ==
        0) {
      return false;
    }

    using std::swap;
    swap(*output, queue_[next_read_index_]);

    // Decrement the value of num_elements_ to account for the removed element.
    // Release memory ordering prevents the reads and writes to
    // queue_[next_write_index_] to be reordered to after the decrement. (Once
    // the decrement has finished, Insert() might start accessing that element.)
    std::atomic_fetch_sub_explicit(&num_elements_, size_t{1},
                                   std::memory_order_release);

    ++next_read_index_;
    if (next_read_index_ == queue_.size()) {
      next_read_index_ = 0;
    }

    RTC_DCHECK_LT(next_read_index_, queue_.size());

    return true;
  }

  // Returns the current number of elements in the queue. Since elements may be
  // concurrently added to the queue, the caller must treat this as a lower
  // bound, not an exact count.
  // May only be called by the consumer.
  size_t SizeAtLeast() const {
    // Acquire memory ordering ensures that we wait for the producer to finish
    // inserting any element in progress.
    return std::atomic_load_explicit(&num_elements_, std::memory_order_acquire);
  }

 private:
  // Verify that the queue slots complies with the ItemVerifier test. This
  // function is not thread-safe and can only be used in the constructors.
  bool VerifyQueueSlots() {
    for (const auto& v : queue_) {
      RTC_DCHECK(queue_item_verifier_(v));
    }
    return true;
  }

  // TODO(peah): Change this to use std::function() once we can use C++11 std
  // lib.
  QueueItemVerifier queue_item_verifier_;

  // Only accessed by the single producer.
  size_t next_write_index_ = 0;

  // Only accessed by the single consumer.
  size_t next_read_index_ = 0;

  // Accessed by both the producer and the consumer and used for synchronization
  // between them.
  std::atomic<size_t> num_elements_{0};

  // The elements of the queue are acced by both the producer and the consumer,
  // mediated by num_elements_. queue_.size() is constant.
  std::vector<T> queue_;

  SwapQueue(const SwapQueue&) = delete;
  SwapQueue& operator=(const SwapQueue&) = delete;
};

}  // namespace webrtc

#endif  // RTC_BASE_SWAP_QUEUE_H_
