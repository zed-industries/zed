// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BREAK_APPEAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BREAK_APPEAL_H_

namespace blink {

// The "appeal" of a breakpoint. Higher is better. The perfect appeal is when
// we're not violating any rules. As we violate rule after rule, appeal will
// decrease. When figuring out where to break, a layout algorithm will use the
// breakpoint with the highest appeal (first priority) that has progressed the
// furthest through the content (second priority). The list here is sorted by
// rule violation severity, i.e. reverse appeal.
enum BreakAppeal {
  // We're attempting to break at a really undesirable place. This is not a
  // valid class A, B or C breakpoint [1]. The only requirement we're satisfying
  // is to not slice monolithic content.
  //
  // [1] https://www.w3.org/TR/css-break-3/#possible-breaks
  kBreakAppealLastResort,

  // The worst thing we're violating is an avoid* value of break-before,
  // break-after, or break-inside.
  kBreakAppealViolatingBreakAvoid,

  // The only thing we're violating is orphans and/or widows requirements.
  kBreakAppealViolatingOrphansAndWidows,

  // We're not violating anything. This is a perfect break location. Note that
  // forced breaks are always perfect, since they trump everything else.
  kBreakAppealPerfect,
};

// Keep this one in sync with the above enum.
const int kBreakAppealBitsNeeded = 2;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BREAK_APPEAL_H_
