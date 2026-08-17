// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_SELECTOR_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_SELECTOR_GENERATOR_H_

#include <optional>

#include "base/gtest_prod_util.h"
#include "base/time/time.h"
#include "components/shared_highlighting/core/common/shared_highlighting_metrics.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/fragment_directive/same_block_word_iterator.h"
#include "third_party/blink/renderer/core/fragment_directive/text_fragment_finder.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"

namespace blink {

class LocalFrame;
class RangeInFlatTree;
class TextFragmentSelector;

// TextFragmentSelectorGenerator is used to generate a TextFragmentSelector,
// given a range of DOM in a document. The TextFragmentSelector provides the
// necessary portions of a text fragment URL such that it scrolls to the given
// range when navigated. For more details, see:
// https://github.com/WICG/scroll-to-text-fragment#proposed-solution.
//
// TextFragmentSelectorGenerator works by starting with a candidate selector
// and repeatedly trying it against the page content to ensure the correct and
// unique match. While the match isn't unique, repeatedly add context/range to
// the selector until the correct match is uniquely identified or no new
// context/range can be added.
class CORE_EXPORT TextFragmentSelectorGenerator final
    : public GarbageCollected<TextFragmentSelectorGenerator>,
      public TextFragmentFinder::Client {
  using GenerateCallback =
      base::OnceCallback<void(const TextFragmentSelector&,
                              shared_highlighting::LinkGenerationError)>;

 public:
  explicit TextFragmentSelectorGenerator(LocalFrame* main_frame);
  TextFragmentSelectorGenerator(const TextFragmentSelectorGenerator&) = delete;
  TextFragmentSelectorGenerator& operator=(
      const TextFragmentSelectorGenerator&) = delete;

  // Requests a TextFragmentSelector be generated for the selection of DOM
  // specified by |range|. Will be generated asynchronously and returned by
  // invoking |callback|.
  void Generate(const RangeInFlatTree& range, GenerateCallback callback);

  // Resets generator state to initial values and cancels any existing async
  // tasks.
  void Reset();

  void SetCallbackForTesting(GenerateCallback callback) {
    pending_generate_selector_callback_ = std::move(callback);
  }

  void Trace(Visitor*) const;

  // Temporary diagnostic metric recorded to help explain discrepancies in
  // other metrics.
  void RecordSelectorStateUma() const;

  LocalFrame* GetFrame() { return frame_.Get(); }

  // Returns the text value of the range this generator is attempting to
  // generate a selector for. Returns empty string if the range is invalid or
  // if called before calling Generate().
  String GetSelectorTargetText() const;

 private:
  friend class TextFragmentSelectorGeneratorTest;

  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetPreviousTextEndPosition_PrevNode);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetPreviousTextEndPosition_PrevTextNode);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetPreviousTextEndPosition_ParentNode);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetPreviousTextEndPosition_SpacesBeforeSelection);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetPreviousTextEndPosition_InvisibleBeforeSelection);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetPreviousTextEndPosition_NoPrevious);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetNextTextStartPosition_NextNode);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetNextTextStartPosition_NextNode_WithComment);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetNextTextStartPosition_NextTextNode);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetNextTextStartPosition_ParentNode);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetNextTextStartPosition_SpacesAfterSelection);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetNextTextStartPosition_InvisibleAfterSelection);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           RangeSelector_RangeMultipleNonBlockNodes);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           GetNextTextStartPosition_NoNextNode);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           ExactTextSelector_Long);
  FRIEND_TEST_ALL_PREFIXES(
      TextFragmentSelectorGeneratorTest,
      GetPreviousTextEndPosition_ShouldSkipNodesWithNoLayoutObject);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentSelectorGeneratorTest,
                           RemoveLayoutObjectAsync);

  // Used for determining the next step of selector generation.
  enum GenerationStep { kExact, kRange, kContext };

  // Used for determining the current state of |selector_|.
  enum SelectorState {
    // Search for candidate selector didn't start.
    kNotStarted,

    // Candidate selector should be generated or extended.
    kNeedsNewCandidate,

    // Candidate selector generation was successful and selector is ready to be
    // tested for uniqueness and accuracy by running against the page's content.
    kTestCandidate,

    // Candidate selector generation was unsuccessful. No further attempts are
    // necessary.
    kFailure,

    // Selector is found. No further attempts are necessary.
    kSuccess,

    kMaxValue = kSuccess
  };

  // TextFragmentFinder::Client interface
  void DidFindMatch(const RangeInFlatTree& match, bool is_unique) override;
  void NoMatchFound() override;

  // Adjust the selection start/end to a valid position. That includes skipping
  // non text start/end nodes and extending selection from start and end to
  // contain full words.
  void AdjustSelection();

  // Generates selector for current selection.
  void StartGeneration();

  void GenerateSelectorCandidate();

  void ResolveSelectorState();
  void RunTextFinder();

  // Returns first position for the text following given position.
  PositionInFlatTree GetNextTextStartPosition(
      const PositionInFlatTree& position);

  // Returns last position for the text preceding given position.
  PositionInFlatTree GetPreviousTextEndPosition(
      const PositionInFlatTree& position);

  void GenerateExactSelector();
  void ExtendRangeSelector();
  void ExtendContext();

  void RecordAllMetrics(const TextFragmentSelector& selector);

  // Called when selector generation is complete.
  void OnSelectorReady(const TextFragmentSelector& selector);

  // Called by tests to change default parameters. A negative value will reset
  // the override.
  static void OverrideExactTextMaxCharsForTesting(int value);
  unsigned GetExactTextMaxChars();

  Member<LocalFrame> frame_;

  // This is the Range for which we're generating a selector.
  Member<RangeInFlatTree> range_;

  std::unique_ptr<TextFragmentSelector> selector_;

  GenerateCallback pending_generate_selector_callback_;

  // Callback invoked each time DidFindMatch is called; for testing only.
  // Allows inserting code to run between asynchronous generation steps.
  base::OnceCallback<void(bool is_unique)> did_find_match_callback_for_testing_;

  GenerationStep step_ = kExact;
  SelectorState state_ = kNeedsNewCandidate;

  shared_highlighting::LinkGenerationError error_;

  // Iterators for gradually forming prefix, suffix and range
  Member<ForwardSameBlockWordIterator> suffix_iterator_;
  Member<BackwardSameBlockWordIterator> prefix_iterator_;
  Member<ForwardSameBlockWordIterator> range_start_iterator_;
  Member<BackwardSameBlockWordIterator> range_end_iterator_;

  // Indicates a number of words used from |max_available_prefix_| and
  // |max_available_suffix_| for the current |selector_|.
  int num_context_words_ = 0;

  int num_range_words_ = 0;

  int iteration_ = 0;
  base::TimeTicks generation_start_time_;

  Member<TextFragmentFinder> finder_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_SELECTOR_GENERATOR_H_
