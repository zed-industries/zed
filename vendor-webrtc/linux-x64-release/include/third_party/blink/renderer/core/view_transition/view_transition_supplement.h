// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_SUPPLEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_SUPPLEMENT_H_

#include "components/viz/common/view_transition_element_resource_id.h"
#include "third_party/blink/public/mojom/frame/frame.mojom-blink.h"
#include "third_party/blink/public/mojom/navigation/navigation_params.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_view_transition_callback.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_view_transition_options.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/view_transition/view_transition.h"
#include "third_party/blink/renderer/core/view_transition/view_transition_request.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {
class DOMViewTransition;

class CORE_EXPORT ViewTransitionSupplement
    : public GarbageCollected<ViewTransitionSupplement>,
      public Supplement<Document>,
      public ViewTransition::Delegate {
 public:
  static const char kSupplementName[];

  // Supplement functionality.
  static ViewTransitionSupplement* From(Document&);
  static ViewTransitionSupplement* FromIfExists(const Document&);

  // Creates and starts a same-document ViewTransition initiated using the
  // script API.
  // With callback:
  static DOMViewTransition* startViewTransition(
      ScriptState*,
      Document&,
      V8ViewTransitionCallback* callback,
      ExceptionState&);
  // With options
  static DOMViewTransition* startViewTransition(ScriptState*,
                                                Document&,
                                                ViewTransitionOptions* options,
                                                ExceptionState&);
  // Without callback or options:
  static DOMViewTransition* startViewTransition(ScriptState*,
                                                Document&,
                                                ExceptionState&);

  static DOMViewTransition* StartViewTransitionForElement(
      ScriptState*,
      Element*,
      V8ViewTransitionCallback* callback,
      const std::optional<Vector<String>>& types,
      ExceptionState&);

  // Creates a ViewTransition to cache the state of a Document before a
  // navigation. The cached state is provided to the caller using the
  // |ViewTransitionStateCallback|.
  static void SnapshotDocumentForNavigation(
      Document&,
      const blink::ViewTransitionToken& transition_token,
      mojom::blink::PageSwapEventParamsPtr,
      ViewTransition::ViewTransitionStateCallback);

  // Creates a ViewTransition using cached state from the previous Document
  // of a navigation. This ViewTransition is responsible for running
  // animations on the new Document using the cached state.
  static void CreateFromSnapshotForNavigation(
      Document&,
      ViewTransitionState transition_state);

  // Abort any ongoing transitions in the document.
  static void AbortTransition(Document&);

  ViewTransition* GetTransition();
  ViewTransition* GetTransition(const Element&);
  void ForEachTransition(base::FunctionRef<void(ViewTransition&)>);

  explicit ViewTransitionSupplement(Document&);
  ~ViewTransitionSupplement() override;

  // GC functionality.
  void Trace(Visitor* visitor) const override;

  // ViewTransition::Delegate implementation.
  void AddPendingRequest(std::unique_ptr<ViewTransitionRequest>) override;
  VectorOf<std::unique_ptr<ViewTransitionRequest>> TakePendingRequests();
  void OnTransitionFinished(ViewTransition* transition) override;

  // TODO(https://crbug.com/1422251): Expand this to receive a the full set of
  // @view-transition options.
  void OnViewTransitionsStyleUpdated(bool cross_document_enabled,
                                     const Vector<String>& types);

  // Notifies that the `body` element has been parsed and will be added to the
  // Document.
  void WillInsertBody();

  // Since outbound transitions are only enabled once the document has been
  // revealed, we recompute the opt-in status here.
  void DidChangeRevealState() { SendOptInStatusToHost(); }
  void DidChangeVisibilityState();

  // In the new page of a cross-document transition, this resolves the
  // @view-transition rule to use, sets types, and returns the ViewTransition.
  // It is the 'resolve cross-document view-transition` steps in the spec:
  // https://drafts.csswg.org/css-view-transitions-2/#document-resolve-cross-document-view-transition
  DOMViewTransition* ResolveCrossDocumentViewTransition();

  // Generates a new ID usable from viz to refer to a snapshot resource.
  viz::ViewTransitionElementResourceId GenerateResourceId(
      const blink::ViewTransitionToken& transition_token,
      bool for_subframe_snapshot);

  // Initializes the sequence such that the next call to GenerateResourceId()
  // will return `next_local_id`. Used to ensure a unique and continuous
  // sequence of ids across documents in a cross-document transition.
  void InitializeResourceIdSequence(uint32_t next_local_id);

 private:
  DOMViewTransition* StartTransition(Element& element,
                                     V8ViewTransitionCallback* callback,
                                     const std::optional<Vector<String>>& types,
                                     ExceptionState& exception_state);
  void StartTransition(Document& document,
                       const blink::ViewTransitionToken& transition_token,
                       mojom::blink::PageSwapEventParamsPtr,
                       ViewTransition::ViewTransitionStateCallback callback);
  void StartTransition(Document& document,
                       ViewTransitionState transition_state);

  void SetCrossDocumentOptIn(mojom::blink::ViewTransitionSameOriginOptIn);

  void SendOptInStatusToHost();

  // Document-level view transition.
  // TODO(crbug.com/394052227): Change document transitions to be stored in
  // element_transitions_ (keyed on the documentElement), and remove
  // document_transition_.
  Member<ViewTransition> document_transition_;

  // Element-scoped view transitions.
  HeapHashMap<WeakMember<const Element>, Member<ViewTransition>>
      element_transitions_;

  VectorOf<std::unique_ptr<ViewTransitionRequest>> pending_requests_;

  mojom::blink::ViewTransitionSameOriginOptIn cross_document_opt_in_ =
      mojom::blink::ViewTransitionSameOriginOptIn::kDisabled;

  uint32_t resource_local_id_sequence_ =
      viz::ViewTransitionElementResourceId::kInvalidLocalId;

  Vector<String> cross_document_types_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_SUPPLEMENT_H_
