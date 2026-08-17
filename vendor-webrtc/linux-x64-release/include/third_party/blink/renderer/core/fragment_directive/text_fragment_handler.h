// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_HANDLER_H_

#include "base/gtest_prod_util.h"
#include "components/shared_highlighting/core/common/shared_highlighting_metrics.h"
#include "third_party/blink/public/mojom/link_to_text/link_to_text.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/fragment_directive/text_fragment_selector_generator.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"

namespace blink {

class AnnotationAgentImpl;
class Document;
class LocalFrame;
class TextFragmentAnchor;

// TextFragmentHandler is responsible for handling requests from the
// browser-side link-to-text/shared-highlighting feature. It is responsible for
// generating a text fragment URL based on the current selection as well as
// collecting information about and modifying text fragments on the current
// page. This class is registered on and owned by the frame that interacts with
// the link-to-text/shared-highlighting feature.
class CORE_EXPORT TextFragmentHandler final
    : public GarbageCollected<TextFragmentHandler>,
      public blink::mojom::blink::TextFragmentReceiver {
 public:
  explicit TextFragmentHandler(LocalFrame* frame);
  TextFragmentHandler(const TextFragmentHandler&) = delete;
  TextFragmentHandler& operator=(const TextFragmentHandler&) = delete;

  // Determine if |result| represents a click on an existing highlight.
  static bool IsOverTextFragment(const HitTestResult& result);

  // Called to notify the frame's TextFragmentHandler on context menu open over
  // a selection. Will trigger preemptive generation if needed.
  static void OpenedContextMenuOverSelection(LocalFrame* frame);

  // TODO(crbug.com/1303887): This temporarily takes a Document since
  // TextFragmentHandler is currently 1:1 with Document. This will change when
  // we get a SharedHighlightingManager and this method can avoid storing the
  // agent, instead bind the AnnotationAgent to a host in the browser.
  static void DidCreateTextFragment(AnnotationAgentImpl& agent,
                                    Document& owning_document);

  // mojom::blink::TextFragmentReceiver interface
  void Cancel() override;
  void RequestSelector(RequestSelectorCallback callback) override;
  void GetExistingSelectors(GetExistingSelectorsCallback callback) override;
  void RemoveFragments() override;
  void ExtractTextFragmentsMatches(
      ExtractTextFragmentsMatchesCallback callback) override;
  void ExtractFirstFragmentRect(
      ExtractFirstFragmentRectCallback callback) override;

  void BindTextFragmentReceiver(
      mojo::PendingReceiver<mojom::blink::TextFragmentReceiver> producer);

  void Trace(Visitor*) const;

  TextFragmentSelectorGenerator* GetTextFragmentSelectorGenerator() {
    return text_fragment_selector_generator_.Get();
  }

  void DidDetachDocumentOrFrame();

 private:
  FRIEND_TEST_ALL_PREFIXES(TextFragmentHandlerTest,
                           IfGeneratorResetShouldRecordCorrectError);
  FRIEND_TEST_ALL_PREFIXES(TextFragmentHandlerTest, NotGenerated);
  // Returns whether preemptive generation should run for the given frame.
  static bool ShouldPreemptivelyGenerateFor(LocalFrame* frame);

  // The callback passed to TextFragmentSelectorGenerator that will receive the
  // result.
  void DidFinishSelectorGeneration(
      const TextFragmentSelector& selector,
      shared_highlighting::LinkGenerationError error);

  // This starts running the generator over the current selection.
  // The result will be returned by invoking DidFinishSelectorGeneration().
  void StartGeneratingForCurrentSelection();

  // Called to reply to the client's RequestSelector call with the result.
  void InvokeReplyCallback(const TextFragmentSelector& selector,
                           shared_highlighting::LinkGenerationError error);

  TextFragmentAnchor* GetTextFragmentAnchor();

  LocalFrame* GetFrame() { return frame_.Get(); }

  HeapVector<Member<AnnotationAgentImpl>> annotation_agents_;

  // Class responsible for generating text fragment selectors for the current
  // selection.
  Member<TextFragmentSelectorGenerator> text_fragment_selector_generator_;

  // The result of preemptively generating on selection changes will be stored
  // in this member when completed. Used only in preemptive link generation
  // mode.
  std::optional<TextFragmentSelector> preemptive_generation_result_;

  // If generation failed, contains the reason that generation failed. Default
  // value is kNone.
  shared_highlighting::LinkGenerationError error_;

  // Reports whether |RequestSelector| was called before or after selector was
  // ready. Used only in preemptive link generation mode.
  std::optional<shared_highlighting::LinkGenerationReadyStatus>
      selector_ready_status_;

  // This will hold the reply callback to the RequestSelector mojo call. This
  // will be invoked in InvokeReplyCallback to send the reply back to the
  // browser.
  RequestSelectorCallback response_callback_;

  // Used for communication between |TextFragmentHandler| in renderer
  // and |TextFragmentSelectorClientImpl| in browser.
  HeapMojoReceiver<blink::mojom::blink::TextFragmentReceiver,
                   TextFragmentHandler>
      selector_producer_{this, nullptr};

  Member<LocalFrame> frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_HANDLER_H_
