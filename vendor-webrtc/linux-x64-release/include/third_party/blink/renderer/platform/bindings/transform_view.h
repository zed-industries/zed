// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_TRANSFORM_VIEW_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_TRANSFORM_VIEW_H_

#include <stddef.h>

#include <concepts>
#include <iterator>
#include <utility>

#include "base/compiler_specific.h"

namespace blink::bindings {

// This is somewhat similar in spirit to std::range::views, but really lean,
// mean and specifically tailored to exposing ranges of blink data to the
// generated bindings where those expect an IDLSequence<>.
// Some limitations include:
// - TransformedView only offers a forward iterator, even if
//     unedrlying collection can offer better iterators;
// - No sentinel support
// - Transform instance is not accepted as an argument or preserved
//     across calls.
// All of these may be re-evaluated if new usages arise.

template <typename Range, typename Transform>
  requires(std::forward_iterator<decltype(std::begin(
               std::declval<const Range&>()))> &&
           std::regular_invocable<Transform,
                                  decltype(*std::begin(
                                      std::declval<const Range&>()))>)
class TransformedView {
 public:
  class TransformingIterator {
   public:
    using inner_iterator = decltype(std::begin(std::declval<const Range&>()));
    using inner_value_t = std::iter_value_t<inner_iterator>;

    // std::iterator interface
    // TODO(caseq): It may be nice to follow the category of the
    // underlying iterator instead of just offering a forward one.
    // We don't want to implement the entire bunch of operations required
    // by random_access_iterator so far, but we need to be able to compute
    // size for current use cases, so we offer size if underlying iterator
    // is a random access one.
    using iterator_category = std::forward_iterator_tag;
    using value_type = decltype(std::declval<Transform>()(
        std::declval<const inner_value_t&>()));

    TransformingIterator() = default;
    explicit TransformingIterator(inner_iterator it) : it_(it) {}
    TransformingIterator(const TransformingIterator& r) = default;
    TransformingIterator& operator=(const TransformingIterator& r) = default;

    bool operator==(const TransformingIterator& r) const {
      return it_ == r.it_;
    }
    bool operator!=(const TransformingIterator& r) const {
      return it_ != r.it_;
    }
    TransformingIterator& operator++() {
      UNSAFE_TODO(++it_);
      return *this;
    }
    TransformingIterator operator++(int) {
      return UNSAFE_TODO(TransformingIterator(it_++));
    }

    value_type operator*() const { return Transform()(*it_); }

    size_t operator-(const TransformingIterator& r) const
      requires(std::random_access_iterator<inner_iterator>)
    {
      return it_ - r.it_;
    }

   private:
    inner_iterator it_;
  };

  using inner_iterator = decltype(std::begin(std::declval<const Range&>()));
  using iterator_t = TransformingIterator;
  using value_type = iterator_t::value_type;

  TransformedView(inner_iterator begin, inner_iterator end)
      : begin_(begin), end_(end) {}
  explicit TransformedView(const Range& r)
      : begin_(std::begin(r)), end_(std::end(r)) {}

  iterator_t begin() const { return begin_; }
  iterator_t end() const { return end_; }

  size_t size() const
    requires(std::random_access_iterator<inner_iterator>)
  {
    return end_ - begin_;
  }
  bool empty() const { return begin_ == end_; }

  iterator_t begin_;
  iterator_t end_;
};

template <typename TransformFunc, typename Range>
auto Transform(const Range& range) {
  return TransformedView<Range, TransformFunc>(range);
}

}  // namespace blink::bindings

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_TRANSFORM_VIEW_H_
