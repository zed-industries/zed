// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_ANCHOR_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_ANCHOR_METRICS_H_

#include "base/time/tick_clock.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/fragment_directive/text_fragment_selector.h"

namespace blink {

// Helper class for TextFragmentAnchor that provides hooks for tracking and
// reporting usage and performance metrics to UMA.
class CORE_EXPORT TextFragmentAnchorMetrics final
    : public GarbageCollected<TextFragmentAnchorMetrics> {
 public:
  // An enum to indicate which parameters were specified in the text fragment.
  enum class TextFragmentAnchorParameters {
    kUnknown = 0,
    kExactText = 1,
    kExactTextWithPrefix = 2,
    kExactTextWithSuffix = 3,
    kExactTextWithContext = 4,
    kTextRange = 5,
    kTextRangeWithPrefix = 6,
    kTextRangeWithSuffix = 7,
    kTextRangeWithContext = 8,
    kMaxValue = kTextRangeWithContext,
  };

  // Update corresponding |TextFragmentLinkOpenSource| in enums.xml.
  enum class TextFragmentLinkOpenSource {
    kUnknown,
    kSearchEngine,

    kMaxValue = kSearchEngine,
  };

  explicit TextFragmentAnchorMetrics(Document* document);

  static TextFragmentAnchorParameters GetParametersForSelector(
      const TextFragmentSelector& selector);

  void DidCreateAnchor(int selector_count);

  void DidFindMatch();

  void DidFindAmbiguousMatch();

  void DidInvokeScrollIntoView();

  void ReportMetrics();

  void SetTickClockForTesting(const base::TickClock* tick_clock);

  void SetSearchEngineSource(bool has_search_engine_source);

  void Trace(Visitor*) const;

 private:
  std::string GetPrefixForHistograms() const;

  Member<Document> document_;

#ifndef NDEBUG
  bool metrics_reported_ = false;
#endif

  int selector_count_ = 0;
  int matches_count_ = 0;
  bool ambiguous_match_ = false;
  base::TimeTicks search_start_time_;
  base::TimeTicks first_scroll_into_view_time_;
  bool has_search_engine_source_ = false;

  const base::TickClock* tick_clock_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_ANCHOR_METRICS_H_
