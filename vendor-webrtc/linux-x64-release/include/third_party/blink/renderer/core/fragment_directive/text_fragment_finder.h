// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_FINDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_FINDER_H_

#include "base/functional/callback.h"
#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/ephemeral_range.h"
#include "third_party/blink/renderer/core/editing/finder/find_buffer_runner.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/editing/position.h"
#include "third_party/blink/renderer/core/editing/position_iterator.h"
#include "third_party/blink/renderer/core/editing/relocatable_position.h"
#include "third_party/blink/renderer/core/fragment_directive/text_fragment_anchor_metrics.h"
#include "third_party/blink/renderer/core/fragment_directive/text_fragment_selector.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Document;

// This is a helper class to TextFragmentAnchor. It's responsible for actually
// performing the text search for the anchor and returning the results to the
// anchor. Has derived class |MockTextFragmentFinder|.
class CORE_EXPORT TextFragmentFinder
    : public GarbageCollected<TextFragmentFinder> {
 public:
  class Client {
   public:
    virtual void DidFindMatch(const RangeInFlatTree& range, bool is_unique) = 0;
    virtual void NoMatchFound() = 0;
  };

  // Used for tracking what should be the next match stage.
  enum FindBufferRunnerType { kAsynchronous, kSynchronous };

  // Returns next position that is not space or equivalent.
  static PositionInFlatTree NextTextPosition(PositionInFlatTree position,
                                             PositionInFlatTree end_position);

  // Returns previous position that is not space or equivalent.
  static PositionInFlatTree PreviousTextPosition(
      PositionInFlatTree position,
      PositionInFlatTree max_position);

  // Returns true if start and end positions are in the same block and there are
  // no other blocks between them. Otherwise, returns false.
  static bool IsInSameUninterruptedBlock(const PositionInFlatTree& start,
                                         const PositionInFlatTree& end);

  // Client must outlive the finder.
  TextFragmentFinder(Client& client,
                     const TextFragmentSelector& selector,
                     Document* document,
                     FindBufferRunnerType runner_type);
  TextFragmentFinder(Client& client,
                     const TextFragmentSelector& selector,
                     Range* range,
                     FindBufferRunnerType runner_type);
  virtual ~TextFragmentFinder() = default;

  // Begins searching in the given top-level document.
  void FindMatch();

  void Cancel();

  void Trace(Visitor*) const;

  const RangeInFlatTree* FirstMatch() const { return first_match_.Get(); }

  const TextFragmentSelector& GetSelector() const { return selector_; }

 protected:
  void FindPrefix();
  void FindTextStart();
  void FindTextEnd();
  void FindSuffix();

  // Used for tracking what should be the next match stage.
  enum SelectorMatchStep {
    kMatchPrefix,
    kMatchTextStart,
    kMatchTextEnd,
    kMatchSuffix
  };

  SelectorMatchStep step_ = kMatchPrefix;

 private:
  friend class MockTextFragmentFinder;
  friend class TextFragmentFinderTest;
  FRIEND_TEST_ALL_PREFIXES(TextFragmentFinderTest, DOMMutation);

  void FindMatchFromPosition(PositionInFlatTree search_start);

  void OnFindMatchInRangeComplete(String search_text,
                                  RangeInFlatTree* range,
                                  bool word_start_bounded,
                                  bool word_end_bounded,
                                  const EphemeralRangeInFlatTree& match);

  void FindMatchInRange(String search_text,
                        RangeInFlatTree* range,
                        bool word_start_bounded,
                        bool word_end_bounded);

  void OnPrefixMatchComplete(EphemeralRangeInFlatTree match);
  void OnTextStartMatchComplete(EphemeralRangeInFlatTree match);
  void OnTextEndMatchComplete(EphemeralRangeInFlatTree match);
  void OnSuffixMatchComplete(EphemeralRangeInFlatTree match);

  virtual void GoToStep(SelectorMatchStep step);

  void OnMatchComplete();

  void SetPotentialMatch(EphemeralRangeInFlatTree range);
  void SetPrefixMatch(EphemeralRangeInFlatTree range);

  bool HasValidRanges();

  Client& client_;
  const TextFragmentSelector selector_;
  Member<Range> range_;

  // Start positions for the next text end |FindTask|, this is separate as the
  // search for end might move the position, which should be discarded.
  Member<RelocatablePosition> range_end_search_start_;

  // For all the following ranges use |RangeInFlatTree| relocatable range to be
  // safe for DOM mutations during the Find tasks.

  // Successful match after |FindMatchFromPosition| first run.
  Member<RangeInFlatTree> first_match_;
  // Current match after |FindMatchFromPosition| run. See
  // https://wicg.github.io/scroll-to-text-fragment/#ref-for-range-collapsed:~:text=Let-,potentialMatch
  Member<RangeInFlatTree> potential_match_;
  // Used for text start match, the presence will change how the text start is
  // matched. See
  // https://wicg.github.io/scroll-to-text-fragment/#ref-for-range-collapsed:~:text=Let-,prefixMatch
  Member<RangeInFlatTree> prefix_match_;
  // Range for the current match task, including |prefix_match_|,
  // |potential_match_| and |suffix_match|. See
  // https://wicg.github.io/scroll-to-text-fragment/#ref-for-range-collapsed:~:text=Let-,searchRange
  Member<RangeInFlatTree> search_range_;
  // Start and end positions used for search for |potential_match_|.
  // https://wicg.github.io/scroll-to-text-fragment/#ref-for-range-collapsed:~:text=Let-,matchRange
  Member<RangeInFlatTree> match_range_;
  // Used for running FindBuffer tasks.
  Member<FindBufferRunner> find_buffer_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_FINDER_H_
