// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_ANCHOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_ANCHOR_H_

#include "third_party/blink/public/mojom/loader/same_document_navigation_type.mojom-blink.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/renderer/core/annotation/annotation_agent_container_impl.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/fragment_directive/selector_fragment_anchor.h"
#include "third_party/blink/renderer/core/fragment_directive/text_fragment_anchor_metrics.h"
#include "third_party/blink/renderer/core/fragment_directive/text_fragment_finder.h"
#include "third_party/blink/renderer/core/page/scrolling/element_fragment_anchor.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class AnnotationAgentImpl;
class DocumentLoader;
class LocalFrame;
class KURL;
class TextDirective;

// TextFragmentAnchor is the coordinator class for applying text directives
// from the URL (also known as "scroll-to-text") to a document. This class'
// purpose is to integrate with Blink's loading and lifecycle states and
// execute behavior specific to scroll-to-text. The actual logic of performing
// the text search and applying highlights is delegated out to the core
// annotation API. The Annotations API performs the text matching while this
// class tracks the state of its created annotations and performs necessary
// actions based on that state.
//
// This class has three distinct responsibilities:
//
// 1) Vet security restrictions and create an annotation for each text
//    directive in the URL.
// 2) Track the state of the first matching directive, apply "effects" to it
//    (highlight, scroll to it, apply focus, etc.).
// 3) Mark non-matching directives as needing to retry their search. The
//    initial search occurs after parsing completes; if the load event takes
//    longer this class retries unmatched directives again at the load event.
//
// A frame will try to create a TextFragmentAnchor when parsing in a document
// completes. If the URL has a valid text directive an instance of
// TextFragmentAnchor will be created and stored on the LocalFrameView.
// TextFragmentAnchor then turns the directive into AnnotationAgents to perform
// the text search.
//
// Each directive is initially searched automatically after creating its
// corresponding AnnotationAgentImpl. However, as pages often load content
// dynamically, this class may schedule up to two additional search attempts.
// One at document load completion and another a short delay after load.
//
// The anchor performs its operations via the InvokeSelector method which is
// invoked repeatedly, each time layout finishes in the document. Thus, the
// anchor is guaranteed that layout is clean in InvokeSelector; however,
// end-of-layout is a script-forbidden section so no actions that can result in
// script being run can be invoked from there. Script-running actions are
// performed in finalization py posting an animation frame task.
//
// TextFragmentAnchor is a state machine that transitions state via
// InvokeSelector (and some external events). The state represents the status
// of the first matching directive, which is the directive to which effects
// will be applied. Here are the state transitions (see the enum definition for
// details of each state):
//
//           ┌──────┐
//           │   ┌──┴────────────────────────┐
//           └───►       kSearching          ├───────┐
//               └─────────────┬─────────────┘       │
//           ┌──────┐          │                     │
//           │   ┌──┴──────────▼─────────────┐       │
//           └───►  kWaitingForDOMMutations  │       │
//               └─────────────┬─────────────┘       │
//                             │                     │
//               ┌─────────────▼─────────────┐       │
//               │        kApplyEffects      │◄──────┤
//               └─────────────┬─────────────┘       │
//           ┌──────┐          │                     │
//           │   ┌──┴──────────▼─────────────┐       │
//           └───►        kKeepInView        │       │
//               └─────────────┬─────────────┘       │
//                             │                     │
//               ┌─────────────▼─────────────┐       │
//               │        kFinalized         │◄──────┘
//               └───────────────────────────┘
class CORE_EXPORT TextFragmentAnchor final
    : public SelectorFragmentAnchor,
      public AnnotationAgentContainerImpl::Observer {
 public:
  static base::TimeDelta PostLoadTaskDelay();
  static base::TimeDelta PostLoadTaskTimeout();

  // When a document is loaded, this method will be called to see if it meets
  // the criteria to generate a new permission token (at a high level it means:
  // did the user initiate the navigation?). This token can then be used to
  // invoke the text fragment anchor later during loading or propagated across
  // a redirect so that the real destination can invoke a text fragment.
  static bool GenerateNewToken(const DocumentLoader&);

  // Same as above but for same-document navigations. These require a bit of
  // care since DocumentLoader's state is based on the initial document load.
  // In this case, we also avoid generating the token unless the new URL has a
  // text fragment in it (and thus it'll be consumed immediately).
  static bool GenerateNewTokenForSameDocument(
      const DocumentLoader&,
      WebFrameLoadType load_type,
      mojom::blink::SameDocumentNavigationType same_document_navigation_type);

  static TextFragmentAnchor* TryCreate(const KURL& url,
                                       LocalFrame& frame,
                                       bool should_scroll);

  TextFragmentAnchor(HeapVector<Member<TextDirective>>& text_directives,
                     LocalFrame& frame,
                     bool should_scroll);
  TextFragmentAnchor(const TextFragmentAnchor&) = delete;
  TextFragmentAnchor& operator=(const TextFragmentAnchor&) = delete;
  ~TextFragmentAnchor() override = default;

  // SelectorFragmentAnchor implementation
  bool InvokeSelector() override;

  // FragmentAnchor implementation
  void Installed() override;
  bool IsTextFragmentAnchor() override { return true; }
  void NewContentMayBeAvailable() override;

  void SetTickClockForTesting(const base::TickClock* tick_clock);

  void Trace(Visitor*) const override;

  // AnnotationAgentContainerImpl::Observer implementation
  void WillPerformAttach() override;

 private:
  friend class TextFragmentAnchorTestBase;

  // Called after annotation agents attempt attachment to check each
  // agent's state and update this class' state machine.
  void UpdateCurrentState();

  // Called once matched text is ready (any necessary DOM mutations have been
  // been processed). CSS style and scroll into view can now be performed.
  void ApplyEffectsToFirstMatch();

  // Performs ScrollIntoView so that the first match is visible in the viewport.
  // Returns true if scrolling was performed, false otherwise.
  bool EnsureFirstMatchInViewIfNeeded();

  // Called when the search is finished. Reports metrics and activates the
  // element fragment anchor if we didn't find a match. Schedules a
  // finalization task to perform any needed script-running actions and move to
  // kDone.
  void DidFinishSearch();

  // Called in an animation frame, performs any nfinal actions that need a
  // script-safe section and moves the anchor into the kDone state.
  void FinalizeAnchor();

  void ApplyTargetToCommonAncestor(const EphemeralRangeInFlatTree& range);

  bool HasSearchEngineSource();

  // Calls SetNeedsAttachment on any annotations that haven't previously
  // matched. Returns true if any such annotations were found.
  bool MarkFailedAttachmentsForRetry();

  // The post load task scheduled to cause a final search for unmatched
  // directives. Called from a timer.
  void PostLoadTask(TimerBase*);

  // This keeps track of each TextDirective and its associated
  // AnnotationAgentImpl. The directive is the DOM object exposed to JS that's
  // parsed from the URL while the AnnotationAgent is the object responsible
  // for performing the search for the specified text in the Document.
  using DirectiveAnnotationPair =
      std::pair<Member<TextDirective>, Member<AnnotationAgentImpl>>;
  HeapVector<DirectiveAnnotationPair> directive_annotation_pairs_;

  // Used to track which annotations have already recorded a match in
  // UpdateCurrentState so we can track metrics across multiple attachment
  // attempts.
  HeapHashSet<Member<AnnotationAgentImpl>> matched_annotations_;

  // Once the first directive (in specified order) has been successfully
  // attached to DOM its annotation will be stored in `first_match_`.
  Member<const AnnotationAgentImpl> first_match_;

  // If the text fragment anchor is defined as a fragment directive and we don't
  // find a match, we fall back to the element anchor if it is present.
  Member<ElementFragmentAnchor> element_fragment_anchor_;

  // Timers to call the post load task. `post_load_timer_` is scheduled shortly
  // after load but can be rescheduled each time a DOM mutation occurs.
  // However, if DOM mutations keep pushing the post load task out, eventually
  // the timeout timer will run and clear both timers. These timers are mutually
  // exclusive, if either is run it will stop the other.
  HeapTaskRunnerTimer<TextFragmentAnchor> post_load_timer_;
  HeapTaskRunnerTimer<TextFragmentAnchor> post_load_timeout_timer_;

  // Tracks which search attempt the anchor is currently on.
  enum SearchIteration {
    // The initial search occurs when parsing completes.
    kParsing,
    // If parsing completed before the document was loaded, a second search is
    // is being performed at load completion time.
    kLoad,
    // A final "post-load" search has been scheduled.
    kPostLoad,
    // All directives have been matched or all the above search iterations have
    // been exhausted. No further searches will occur.
    kDone
  } iteration_ = kParsing;

  bool finalize_pending_ = false;

  enum AnchorState {
    // We're still waiting to find any matches. Either a search hasn't yet been
    // performed or no matches were found. If no matches were found in the
    // initial attachment, a second try will occur only after the load event
    // has fired; InvokeSelector calls before then will be a no-op.
    kSearching,
    // At least one match has been found but requires some kind of DOM mutation
    // (e.g. expanding a <details> element). Wait in this state until those
    // complete. InvokeSelector will be a no-op in this state other than
    // checking
    // whether this has completed.
    kWaitingForDOMMutations,
    // The matched text is found and fully ready so InvokeSelector will now
    // apply effects like CSS :target, scrollIntoView, etc.
    kApplyEffects,
    // The first match has been completely processed. The anchor is kept alive
    // in this state until the load event fires and all remaining directives
    // are matched. All that's done during this state is to keep the match
    // centered in the viewport in spite of layout shifts.
    kKeepInView,
    // Script-running actions have completed - nothing left to do.
    kFinalized
  } state_ = kSearching;

  Member<TextFragmentAnchorMetrics> metrics_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_ANCHOR_H_
