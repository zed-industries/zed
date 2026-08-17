// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_TEXT_ANNOTATION_SELECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_TEXT_ANNOTATION_SELECTOR_H_

#include <optional>

#include "third_party/blink/renderer/core/annotation/annotation_selector.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/fragment_directive/text_fragment_finder.h"
#include "third_party/blink/renderer/core/fragment_directive/text_fragment_selector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// AnnotationSelector based on TextFragmentFinder. This selector allows
// attaching to DOM based on exact or a range of text with provided prefix or
// suffix.
class CORE_EXPORT TextAnnotationSelector : public AnnotationSelector,
                                           public TextFragmentFinder::Client {
 public:
  explicit TextAnnotationSelector(const TextFragmentSelector& params);
  ~TextAnnotationSelector() override = default;

  void Trace(Visitor* visitor) const override;

  // AnnotationSelector Interface
  String Serialize() const override;
  void FindRange(Range& search_range,
                 SearchType type,
                 FinishedCallback finished_cb) override;
  bool IsTextSelector() const override { return true; }

  // TextFragmentFinder::Client Interface
  void DidFindMatch(const RangeInFlatTree& range, bool is_unique) override;
  void NoMatchFound() override;

  // This is specific to a metric for TextFragmentAnchor so it isn't part of
  // the selector API. If there's other use cases for metrics gathering it may
  // make sense to make FindRange return a more general Metrics object into
  // which this bit could be added.
  bool WasMatchUnique() const;

 private:
  TextFragmentSelector params_;

  std::optional<bool> was_unique_;

  FinishedCallback finished_callback_;
  Member<TextFragmentFinder> finder_;
};

template <>
struct DowncastTraits<TextAnnotationSelector> {
  static bool AllowFrom(const AnnotationSelector& selector) {
    return selector.IsTextSelector();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_TEXT_ANNOTATION_SELECTOR_H_
