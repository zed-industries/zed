// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_SEQUENCED_QUEUE_H_
#define IPCZ_SRC_IPCZ_SEQUENCED_QUEUE_H_

#include <cstddef>
#include <cstdint>
#include <optional>
#include <vector>

#include "ipcz/sequence_number.h"
#include "third_party/abseil-cpp/absl/base/macros.h"

namespace ipcz {

template <typename T>
struct DefaultSequencedQueueTraits {
  static size_t GetElementSize(const T& element) { return 0; }
};

// SequencedQueue retains a queue of objects strictly ordered by SequenceNumber.
// This class is not thread-safe.
//
// This is useful in situations where queued elements may accumulate slightly
// out-of-order and need to be reordered efficiently for consumption. The
// implementation relies on an assumption that sequence gaps are common but tend
// to be small and short-lived. As such, a SequencedQueue retains at least
// enough linear storage to hold every object between the last popped
// SequenceNumber (exclusive) and the highest queued (or anticipated)
// SequenceNumber so far (inclusive).
//
// Storage may be sparsely populated at times, but as elements are consumed from
// the queue, storage is compacted to reduce waste.
//
// ElementTraits may be overridden to attribute a measurable size to each stored
// element. SequencedQueue performs additional accounting to efficiently track
// the sum of this size across the set of all currently available elements in
// the queue.
template <typename T, typename ElementTraits = DefaultSequencedQueueTraits<T>>
class SequencedQueue {
 public:
  SequencedQueue() = default;

  // Constructs a new SequencedQueue starting at `initial_sequence_number`. The
  // queue will not accept elements with a SequenceNumber below this value, and
  // this will be the SequenceNumber of the first element to be popped.
  explicit SequencedQueue(SequenceNumber initial_sequence_number)
      : base_sequence_number_(initial_sequence_number) {}
  SequencedQueue(SequencedQueue&& other) = default;
  SequencedQueue& operator=(SequencedQueue&& other) = default;
  ~SequencedQueue() = default;

  // As a basic practical constraint, SequencedQueue won't tolerate sequence
  // gaps larger than this value.
  static constexpr uint64_t GetMaxSequenceGap() { return 1000000; }

  // The SequenceNumber of the next element that is or will be available from
  // the queue. This starts at the constructor's `initial_sequence_number` and
  // increments any time an element is successfully popped from the queue or a
  // a SequenceNumber is explicitly skipped via SkipNextSequenceNumber().
  SequenceNumber current_sequence_number() const {
    return base_sequence_number_;
  }

  // The final length of the sequence that can be popped from this queue. Null
  // if a final length has not yet been set. If the final length is N, then the
  // last ordered element that can be pushed to or popped from the queue has a
  // SequenceNumber of N-1.
  std::optional<SequenceNumber> final_sequence_length() const {
    if (!is_final_length_known_) {
      return std::nullopt;
    }
    return SequenceNumber{base_sequence_number_.value() +
                          (entries_.size() - front_index_)};
  }

  // Returns the number of elements currently ready for popping at the front of
  // the queue. This is the number of contiguously sequenced elements
  // available starting from `current_sequence_number()`. If the current
  // SequenceNumber is 5 and this queue holds elements 5, 6, and 8, then this
  // method returns 2: only elements 5 and 6 are available, because element 8
  // cannot be made available until element 7 is also available.
  size_t GetNumAvailableElements() const {
    if (entries_.empty() || !entries_[front_index_]) {
      return 0;
    }

    return entries_[front_index_]->num_entries_in_span;
  }

  // Returns the total size of elements currently ready for popping at the
  // front of the queue. This is the sum of ElementTraits::GetElementSize() for
  // each element counted by `GetNumAvailableElements()`, and it is always
  // returned in constant time.
  size_t GetTotalAvailableElementSize() const {
    if (entries_.empty() || !entries_[front_index_]) {
      return 0;
    }

    return entries_[front_index_]->total_span_size;
  }

  // Returns the total length of the contiguous sequence already pushed and/or
  // popped from this queue so far. This is essentially
  // `current_sequence_number()` plus `GetNumAvailableElements()`. If
  // `current_sequence_number()` is 5 and `GetNumAvailableElements()` is 3, then
  // elements 5, 6, and 7 are available for retrieval and the current sequence
  // length is 8; so this method would return 8.
  SequenceNumber GetCurrentSequenceLength() const {
    return SequenceNumber{current_sequence_number().value() +
                          GetNumAvailableElements()};
  }

  // Sets the final length of this queue's sequence. This is the SequenceNumber
  // of the last element that can be pushed, plus 1. If this is set to zero, no
  // elements can ever be pushed onto this queue.
  //
  // May fail and return false if the queue already has pushed and/or popped
  // elements with a SequenceNumber greater than or equal to `length`, or if a
  // the final sequence length had already been set prior to this call.
  bool SetFinalSequenceLength(SequenceNumber length) {
    if (is_final_length_known_) {
      return false;
    }

    // We've already pushed some entries beyond the current sequence number, and
    // the final sequence length must be at least long enough to contain them.
    ABSL_ASSERT((entries_.empty() && front_index_ == 0) ||
                front_index_ < entries_.size());
    const size_t min_gap = entries_.size() - front_index_;

    const size_t gap = length.value() - base_sequence_number_.value();
    if (gap < min_gap || gap > GetMaxSequenceGap()) {
      return false;
    }

    is_final_length_known_ = true;

    // Resize storage to exactly fit whatever remaining entries are expected
    // after `front_index_`.
    entries_.resize(front_index_ + gap);
    if (entries_.empty()) {
      // No longer any use for our storage capacity.
      ResetAndReleaseStorage();
    }
    return true;
  }

  // Forcibly sets the final length of this queue's sequence to its currently
  // available length. This means that if there were already non-contiguous
  // elements pushed beyond that point in the queue, they are destroyed. If the
  // final sequence length had already been set beyond the current length, this
  // overrides that.
  //
  // This method should be used to whenever an unrecoverable failure makes it
  // impossible for any more entries to be pushed into the queue, to ensure that
  // the queue still behaves consistently up to the point of forced termination.
  //
  // Returns the collection of all non-empty elements which were purged from
  // beyond the last contiguous sequence element.
  [[nodiscard]] std::vector<T> ForceTerminateSequence() {
    is_final_length_known_ = true;
    const SequenceNumber length = GetCurrentSequenceLength();
    const size_t required_storage_size =
        length.value() - base_sequence_number_.value();
    if (required_storage_size == 0) {
      // We're not going to be pushing any more entries into this queue.
      ResetAndReleaseStorage();
      return {};
    }

    // Drop entries pushed anywhere beyond the forced termination point.
    const size_t final_storage_size = front_index_ + required_storage_size;
    std::vector<T> removed_elements;
    for (size_t i = final_storage_size; i < entries_.size(); ++i) {
      if (entries_[i]) {
        removed_elements.push_back(std::move(entries_[i]->element));
      }
    }
    entries_.resize(final_storage_size);
    return removed_elements;
  }

  // Indicates whether this queue is still expecting to have more elements
  // pushed. This is always true if the final sequence length has not been set
  // yet.
  //
  // Once the final sequence length is set, this remains true only until all
  // elements between the initial sequence number (inclusive) and the final
  // sequence length (exclusive) have been pushed into the queue.
  bool ExpectsMoreElements() const {
    const std::optional<SequenceNumber> length = final_sequence_length();
    return !length || GetCurrentSequenceLength() < *length;
  }

  // Indicates whether the next element (in sequence order) is available to pop.
  bool HasNextElement() const {
    return !entries_.empty() && entries_[front_index_];
  }

  // Indicates whether this queue's sequence has been fully consumed. This means
  // the final sequence length has been set AND all elements up to that length
  // have been pushed into and popped from the queue.
  bool IsSequenceFullyConsumed() const {
    return !HasNextElement() && !ExpectsMoreElements();
  }

  // Resets this queue to a state which behaves as if a sequence of parcels of
  // length `n` has already been pushed and popped from the queue. Must be
  // called only on an empty queue and only when the caller can be sure they
  // won't want to push any elements with a SequenceNumber below `n`.
  void ResetSequence(SequenceNumber n) {
    ABSL_ASSERT(entries_.empty());
    base_sequence_number_ = n;
    is_final_length_known_ = false;
    ResetAndReleaseStorage();
  }

  // Attempts to skip SequenceNumber `n` in the sequence by advancing the
  // current SequenceNumber by one. Returns true on success and false on
  // failure.
  //
  // This can only succeed when `current_sequence_number()` is equal to `n`, no
  // entry for SequenceNumber `n` is already in the queue, and `n` is less than
  // the final sequence length if applicable. Success is equivalent to pushing
  // and immediately popping element `n`.
  bool SkipElement(SequenceNumber n) {
    if (base_sequence_number_ != n || HasNextElement()) {
      return false;
    }

    std::optional<SequenceNumber> final_length = final_sequence_length();
    if (final_length && n >= *final_length) {
      return false;
    }

    base_sequence_number_ = NextSequenceNumber(n);
    if (entries_.empty()) {
      // Nothing else needs to change if storage is unoccupied.
      ABSL_ASSERT(front_index_ == 0);
      return true;
    }

    ++front_index_;
    if (front_index_ == entries_.size()) {
      // We've hit the end of storage, so all elements are null.
      ResetStorage();
    }
    return true;
  }

  // Pushes an element into the queue with the given SequenceNumber. This may
  // fail if `n` falls below the minimum or above the maximum (when applicable)
  // expected sequence number for elements in this queue.
  bool Push(SequenceNumber n, T element) {
    if (n < base_sequence_number_) {
      return false;
    }

    std::optional<SequenceNumber> final_length = final_sequence_length();
    if (final_length && n >= *final_length) {
      return false;
    }

    const size_t gap = n.value() - base_sequence_number_.value();
    if (gap > GetMaxSequenceGap()) {
      return false;
    }

    const size_t index = front_index_ + gap;
    if (index >= entries_.size()) {
      entries_.resize(index + 1);
    } else if (entries_[index]) {
      return false;
    }

    PlaceNewEntry(index, n, element);
    return true;
  }

  // Pops the next (in sequence order) element off the queue if available,
  // populating `element` with its contents and returning true on success. On
  // failure `element` is untouched and this returns false.
  bool Pop(T& element) {
    if (!HasNextElement()) {
      return false;
    }

    Entry& head = *entries_[front_index_];
    element = std::move(head.element);

    const SequenceNumber sequence_number = base_sequence_number_;
    base_sequence_number_ = NextSequenceNumber(sequence_number);

    // Make sure the next queued entry has up-to-date accounting, if present.
    const size_t element_size = ElementTraits::GetElementSize(element);
    const size_t next_index = front_index_ + 1;
    if (next_index < entries_.size() && entries_[next_index]) {
      Entry& next = *entries_[next_index];
      next.span_start = head.span_start;
      next.span_end = head.span_end;
      next.num_entries_in_span = head.num_entries_in_span - 1;
      next.total_span_size = head.total_span_size - element_size;

      // Find the tail entry for this span, derived from its stored
      // SequenceNumber. We compute the offset in `entries_` relative to
      // `front_index_`. Note that if the offset is 1, it's the same entry as
      // the new head which we already updated above.
      size_t tail_offset = next.span_end.value() - sequence_number.value();
      if (tail_offset > 1) {
        Entry& tail = *entries_[front_index_ + tail_offset];
        tail.num_entries_in_span = next.num_entries_in_span;
        tail.total_span_size = next.total_span_size;
      }
    }

    entries_[front_index_].reset();
    if (front_index_ < entries_.size() - 1) {
      ++front_index_;
    } else {
      // This was the last element in storage. Now we can reuse all capacity.
      ResetStorage();
    }
    return true;
  }

  // Gets a reference to the next element. This reference is NOT stable across
  // any non-const methods here.
  T& NextElement() {
    ABSL_ASSERT(HasNextElement());
    return entries_[front_index_]->element;
  }

 private:
  // See detailed comments on Entry below for an explanation of this logic.
  void PlaceNewEntry(size_t index, SequenceNumber n, T& element) {
    ABSL_ASSERT(index < entries_.size());
    ABSL_ASSERT(!entries_[index].has_value());

    Entry& entry = entries_[index].emplace();
    entry.num_entries_in_span = 1;
    entry.total_span_size = ElementTraits::GetElementSize(element);
    entry.element = std::move(element);

    if (index == 0 || !entries_[index - 1]) {
      entry.span_start = n;
    } else {
      Entry& left = *entries_[index - 1];
      entry.span_start = left.span_start;
      entry.num_entries_in_span += left.num_entries_in_span;
      entry.total_span_size += left.total_span_size;
    }

    if (index == entries_.size() - 1 || !entries_[index + 1]) {
      entry.span_end = n;
    } else {
      Entry& right = *entries_[index + 1];
      entry.span_end = right.span_end;
      entry.num_entries_in_span += right.num_entries_in_span;
      entry.total_span_size += right.total_span_size;
    }

    Entry* start;
    if (entry.span_start <= base_sequence_number_) {
      start = &entries_[front_index_].value();
    } else {
      const size_t start_index = front_index_ + (entry.span_start.value() -
                                                 base_sequence_number_.value());
      start = &entries_[start_index].value();
    }

    ABSL_ASSERT(entry.span_end >= base_sequence_number_);
    const size_t end_index =
        front_index_ + (entry.span_end.value() - base_sequence_number_.value());
    ABSL_ASSERT(end_index < entries_.size());
    Entry* end = &entries_[end_index].value();

    start->span_end = entry.span_end;
    start->num_entries_in_span = entry.num_entries_in_span;
    start->total_span_size = entry.total_span_size;

    end->span_start = entry.span_start;
    end->num_entries_in_span = entry.num_entries_in_span;
    end->total_span_size = entry.total_span_size;
  }

  // Wipes out any logical storage and resets `front_index_`. This does NOT
  // shrink underlying storage capacity, in anticipation of the capacity being
  // reused by subsequent pushes.
  void ResetStorage() {
    entries_.clear();
    front_index_ = 0;
  }

  // Wipes out any logical storage and resets `front_index_`, also releasing any
  // underlying storage capacity. This is used to eagerly free resources when
  // the queue will no longer accept new elements.
  void ResetAndReleaseStorage() {
    ResetStorage();
    entries_.shrink_to_fit();
  }

  struct Entry {
    Entry() = default;
    Entry(Entry&& other) = default;
    Entry& operator=(Entry&&) = default;
    ~Entry() = default;

    T element;

    // NOTE: The fields below are maintained during Push and Pop operations and
    // are used to support efficient implementation of GetNumAvailableElements()
    // and GetTotalAvailableElementSize(). This warrants some clarification.
    //
    // Conceptually we treat the active range of entries as a series of
    // contiguous spans:
    //
    //     `entries_`: [2][ ][4][5][6][ ][8][9]
    //
    // For example, above we can designate three contiguous spans: element 2
    // stands alone at the front of the queue, elements 4-6 form a second span,
    // and then elements 8-9 form the third. Elements 3 and 7 are absent.
    //
    // We're interested in knowing how many elements (and their total size, as
    // designated by ElementTraits) are available right now, which means we want
    // to answer the question: how long is the span starting at element 0? In
    // this case since element 2 stands alone at the front of the queue, the
    // answer is 1. There's 1 element available right now.
    //
    // If we pop element 2 off the queue, it then becomes:
    //
    //     `entries_`: [ ][4][5][6][ ][8][9]
    //
    // The head of the queue is pointing at the empty slot for element 3, and
    // because no span starts in element 0 there are now 0 elements available to
    // pop.
    //
    // Finally if we then push element 3, the queue looks like this:
    //
    //     `entries_`: [3][4][5][6][ ][8][9]
    //
    // and now there are 4 elements available to pop. Element 0 begins the span
    // of elements 3, 4, 5, and 6.
    //
    // To answer the question efficiently though, each entry records some
    // metadata about the span in which it resides. This information is not kept
    // up-to-date for all entries, but we maintain the invariant that the first
    // and last element of each distinct span has accurate metadata; and as a
    // consequence if any span starts at element 0, then we know element 0's
    // metadata accurately answers our general questions about the head of the
    // queue.
    //
    // When an element with sequence number N is inserted into the queue, it can
    // be classified in one of four ways:
    //
    //    (1) it stands alone with no element present at N-1 or N+1
    //    (2) it follows an element at N-1, but N+1 is empty
    //    (3) it precedes an element at N+1, but N-1 is empty
    //    (4) it falls between present elements at both N-1 and N+1.
    //
    // In case (1) we record in the entry that its span starts and ends at
    // element N; we also record the length of the span (1) and a traits-defined
    // accounting of the element's "size". This entry now has trivially correct
    // metadata about its containing span, which consists only of the entry
    // itself.
    //
    // In case (2), element N is now the tail of a pre-existing span. Because
    // tail elements are always up-to-date, we simply copy and augment the data
    // from the old tail (element N-1) into the new tail (element N). From this
    // data we also know where the head of the span is, and the head entry is
    // also updated to reflect the same new metadata.
    //
    // Case (3) is similar to case (2). Element N is now the head of a
    // pre-existing span, so we copy and augment the already up-to-date N+1
    // entry's metadata (the old head) into our new entry as well as the span's
    // tail entry.
    //
    // Case (4) is joining two pre-existing spans. In this case element N
    // fetches the span's start from element N-1 (the tail of the span to the
    // left), and the span's end from element N+1 (the head of the span to the
    // right); and it sums their element and byte counts with its own. This new
    // combined metadata is copied into both the head of the left span and the
    // tail of the right span, and with element N populated this now constitutes
    // a single combined span with accurate metadata in its head and tail
    // entries.
    //
    // Finally, the only other operation that matters for this accounting is
    // Pop(). All Pop() needs to do is derive new metadata for the new
    // head-of-queue's span (if present) after popping. This metadata is updated
    // in the new head entry as well as its span's tail.
    size_t num_entries_in_span = 0;
    size_t total_span_size = 0;
    SequenceNumber span_start{0};
    SequenceNumber span_end{0};
  };

  // Concrete, sparse storage for each entry. This is sparse because the queue
  // may push elements out of sequence order (e.g. elements 42 and 47 may be
  // pushed before elements 43-46).
  //
  // When the vector is non-empty, the element at `front_index_` always
  // corresponds to the element with `base_sequence_number_` as its
  // SequenceNumber. Elements below `front_index_` are always null.
  //
  // In general, this vector grows to accomodate new entries and is shrunk
  // only once all present entries have been consumed. This avoids the need to
  // remove elements from the front of the vector.
  std::vector<std::optional<Entry>> entries_;

  // The index into `entries_` which corresponds to the front of the queue. When
  // `entries_` is empty this is zero; otherwise it is always kept in bounds of
  // `entries_`. Elements below this index in `entries_` are always null.
  size_t front_index_ = 0;

  // If and only if this is true, the final length of this queue's sequence is
  // known and can be determined by the size of `entries_` relative to
  // `front_index_`.
  bool is_final_length_known_ = false;

  // The SequenceNumber corresponding to the front entry of this queue, which
  // may or may not yet be occupied. If `entries_` is non-empty, storage for
  // this entry is always at `entries_[front_index_]`.
  SequenceNumber base_sequence_number_{0};
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_SEQUENCED_QUEUE_H_
