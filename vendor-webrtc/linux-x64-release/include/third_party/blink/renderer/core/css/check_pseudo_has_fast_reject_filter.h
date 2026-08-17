// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CHECK_PSEUDO_HAS_FAST_REJECT_FILTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CHECK_PSEUDO_HAS_FAST_REJECT_FILTER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/platform/wtf/bloom_filter.h"

namespace blink {

// CheckPseudoHasFastRejectFilter uses a bloom filter for quickly rejecting
// :has() argument selector checking.
//
// We can create the bloom filter by adding identifier hashes (tag hash, id hash
// and class hashes) of all elements in the :has() argument checking traversal.
//
// Once the filter have been created, we can cheaply check whether a :has()
// argument selector possibly matches one of the elements in the :has() argument
// checking traversal by checking whether the filter contains all the identifier
// hashes from the :has() argument selector.
//
// For example, assume this tree:
//
// <div id="has_anchor">
//   <div id="child">
//     <span class="a">
//
// When we check ':has(.a .b)' on '#has_anchor', the bloom filter will contain
// hashes corresponding to 'div', 'span', '#child' and '.a'. From the :has()
// argument selector '.a .b', we will collect identifier hashes corresponding to
// '.a' and '.b'. Then, we will look up the hashes from argument selector in the
// bloom filter and get negative result proving that the argument selector
// '.a .b' doesn't match any descendants of '#has_anchor' since the bloom filter
// doesn't contain the hash for '.b'.
class CORE_EXPORT CheckPseudoHasFastRejectFilter {
 public:
  using FastRejectFilter = WTF::BloomFilter<12>;

  CheckPseudoHasFastRejectFilter() = default;
  CheckPseudoHasFastRejectFilter(CheckPseudoHasFastRejectFilter&) = delete;

  static void CollectPseudoHasArgumentHashes(
      Vector<unsigned>& pseudo_has_argument_hashes,
      const CSSSelector* simple_selector);

  void AddElementIdentifierHashes(const Element& element);

  bool FastReject(const Vector<unsigned>& pseudo_has_argument_hashes) const;

  void AllocateBloomFilter();
  bool BloomFilterAllocated() const { return filter_.get(); }

 private:
  std::unique_ptr<FastRejectFilter> filter_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CHECK_PSEUDO_HAS_FAST_REJECT_FILTER_H_
