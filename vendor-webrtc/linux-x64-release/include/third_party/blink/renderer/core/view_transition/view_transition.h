// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_H_

#include <memory>
#include <unordered_map>

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/types/pass_key.h"
#include "components/viz/common/view_transition_element_resource_id.h"
#include "third_party/blink/public/common/frame/view_transition_state.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_function.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_sync_iterator_view_transition_type_set.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_view_transition_callback.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/page/chrome_client.h"
#include "third_party/blink/renderer/core/view_transition/view_transition_request_forward.h"
#include "third_party/blink/renderer/core/view_transition/view_transition_style_tracker.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/graphics/paint/clip_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/paint/effect_paint_property_node.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

class Document;
class DOMViewTransition;
class Element;
class LayoutObject;
class PseudoElement;
class ViewTransitionPseudoElementBase;

class CORE_EXPORT ViewTransition : public GarbageCollected<ViewTransition>,
                                   public ExecutionContextLifecycleObserver,
                                   public ChromeClient::CommitObserver {
 public:
  using PassKey = base::PassKey<ViewTransition>;
  class Delegate {
   public:
    virtual ~Delegate() = default;

    virtual void AddPendingRequest(std::unique_ptr<ViewTransitionRequest>) = 0;
    virtual void OnTransitionFinished(ViewTransition*) = 0;
  };

  // Creates and starts a same-document ViewTransition initiated using the
  // script API.
  static ViewTransition* CreateFromScript(
      Element*,
      V8ViewTransitionCallback*,
      const std::optional<Vector<String>>& types,
      Delegate*,
      ViewTransition* previously_active);

  // Creates a skipped transition that still runs the specified callbacks.
  static ViewTransition* CreateSkipped(Element*, V8ViewTransitionCallback*);

  // Creates a ViewTransition to cache the state of a Document before a
  // navigation. The cached state is provided to the caller using the
  // |ViewTransitionStateCallback|.
  using ViewTransitionStateCallback =
      base::OnceCallback<void(const ViewTransitionState&)>;
  static ViewTransition* CreateForSnapshotForNavigation(
      Document*,
      const ViewTransitionToken& transition_token,
      ViewTransitionStateCallback,
      const Vector<String>& types,
      Delegate*);

  // Creates a ViewTransition using cached state from the previous Document
  // of a navigation. This ViewTransition is responsible for running
  // animations on the new Document using the cached state.
  static ViewTransition* CreateFromSnapshotForNavigation(Document*,
                                                         ViewTransitionState,
                                                         Delegate*);

  // Script-based constructor.
  ViewTransition(PassKey,
                 Element*,
                 V8ViewTransitionCallback*,
                 const std::optional<Vector<String>>& types,
                 Delegate*,
                 ViewTransition* previously_active);
  // Skipped transition constructor.
  ViewTransition(PassKey, Element*, V8ViewTransitionCallback*);
  // Navigation-initiated for-snapshot constructor.
  ViewTransition(PassKey,
                 Document*,
                 const ViewTransitionToken& transition_token,
                 ViewTransitionStateCallback,
                 const Vector<String>& types,
                 Delegate*);
  // Navigation-initiated from-snapshot constructor.
  ViewTransition(PassKey, Document*, ViewTransitionState, Delegate*);

  DOMViewTransition* GetScriptDelegate() { return script_delegate_.Get(); }

  // GC functionality.
  void Trace(Visitor* visitor) const override;

  // Returns true if the pseudo element corresponding to the given id and name
  // is the only child.
  bool MatchForOnlyChild(PseudoId pseudo_id,
                         const AtomicString& view_transition_name) const;

  // Returns true if the transition matches :active-view-transition
  bool MatchForActiveViewTransition();

  // Returns true if the transition matches :active-view-transition-type with
  // the given types.
  bool MatchForActiveViewTransitionType(
      const Vector<AtomicString>& pseudo_types);

  // ExecutionContextLifecycleObserver implementation.
  void ContextDestroyed() override;

  // Returns true if this object needs to create an EffectNode for its element
  // transition.
  bool NeedsViewTransitionEffectNode(const LayoutObject& object) const;

  // Returns true if this object needs a clip node to render a subset of its
  // painting in the snapshot.
  bool NeedsViewTransitionClipNode(const LayoutObject& object) const;

  // Returns true if this object is painted via pseudo elements. Note that this
  // is different from NeedsViewTransitionEffectNode() since the root may not
  // be a transitioning element, but require an effect node.
  bool IsRepresentedViaPseudoElements(const LayoutObject& object) const;

  // Returns true if `node` participates in the transition excluding the
  // document element. Since the root element's snapshot is hoisted up the
  // LayoutView, this API should be used for checks which are needed to set up
  // state for snapshotting an element. This state is set up on the LayoutView
  // instead of the root element's LayoutView.
  bool IsTransitionElementExcludingRoot(const Element& node) const;

  // Returns the resource id if `object` is producing a snapshot for this
  // transition.
  viz::ViewTransitionElementResourceId GetSnapshotId(
      const LayoutObject& object) const;

  // The layer used to paint the old Document rendered in a LocalFrame subframe
  // until the new Document can start rendering.
  const scoped_refptr<cc::ViewTransitionContentLayer>&
  GetSubframeSnapshotLayer() const;

  // Updates a clip node. The clip tracks the subset of the |object|'s ink
  // overflow rectangle which should be painted.The return value is a result of
  // updating the clip node.
  PaintPropertyChangeType UpdateCaptureClip(
      const LayoutObject& object,
      const ClipPaintPropertyNodeOrAlias* current_clip,
      const TransformPaintPropertyNodeOrAlias* current_transform);

  // Returns the clip. One needs to first call UpdateCaptureClip().
  const ClipPaintPropertyNode* GetCaptureClip(const LayoutObject& object) const;

  // Dispatched during a lifecycle update after prepaint has finished its work.
  // This is only done if the lifecycle update was triggered outside of a main
  // frame. For example by a script API like getComputedStyle.
  void RunViewTransitionStepsOutsideMainFrame();

  // Dispatched during a lifecycle update after prepaint has finished its work.
  // This is only done if we're in the main lifecycle update that will produce
  // painted output.
  void RunViewTransitionStepsDuringMainFrame();

  // This returns true if this transition object needs to gather tags as the
  // next step in the process. This is used to force activatable
  // content-visibility locks.
  bool NeedsUpToDateTags() const;

  // Creates a pseudo element for the given |pseudo_id|.
  PseudoElement* CreatePseudoElement(Element* parent,
                                     PseudoId pseudo_id,
                                     const AtomicString& view_transition_name);

  // Returns the UA style sheet for the pseudo element tree generated during a
  // transition.
  CSSStyleSheet* UAStyleSheet() const;

  // CommitObserver overrides.
  void WillCommitCompositorFrame() override;

  // Return non-root transitioning elements.
  VectorOf<Element> GetTransitioningElements() const {
    return style_tracker_ ? style_tracker_->GetTransitioningElements()
                          : VectorOf<Element>{};
  }

  bool IsRootTransitioning() const {
    return style_tracker_ && document_->documentElement() &&
           style_tracker_->IsTransitionElement(*document_->documentElement());
  }

  void RecalcTransitionPseudoTreeStyle() const;

  void RebuildTransitionPseudoLayoutTree() const;

  // In physical pixels. See comments on equivalent methods in
  // ViewTransitionStyleTracker for info.
  gfx::Size GetSnapshotRootSize() const;
  gfx::Vector2d GetFrameToSnapshotRootOffset() const;

  bool IsDone() const { return IsTerminalState(state_); }

  // Returns true if this object was created to cache a snapshot of the current
  // Document for a navigation.
  bool IsForNavigationSnapshot() const {
    return creation_type_ == CreationType::kForSnapshot;
  }

  // Returns true if this object was created for transitions in the same
  // Document via document.startViewTransition(...).
  bool IsCreatedViaScriptAPI() const {
    return creation_type_ == CreationType::kScript;
  }

  // Returns true if this object was created for a navigation initiated
  // transition on the new Document.
  bool IsForNavigationOnNewDocument() const {
    return creation_type_ == CreationType::kFromSnapshot;
  }

  // Notifies the transition that frames are being produced and that the
  // transition can start the animation phase (starting by capturing the
  // incoming elements). No-op unless the transition is created from a
  // snapshot.
  void ActivateFromSnapshot();

  // Ensure the LayoutViewTransitionRoot, representing the snapshot containing
  // block concept, has up to date style.
  void UpdateSnapshotContainingBlockStyle();

  // Indicates how the promise should be handled.
  enum class PromiseResponse {
    kResolve,
    kRejectAbort,
    kRejectInvalidState,
    kRejectTimeout
  };
  void SkipTransition(PromiseResponse response = PromiseResponse::kRejectAbort);

  // This can be called inside of the lifecycle. It will skip the transition
  // whenever view transition steps are run within the lifecycle.
  void SkipTransitionSoon();

  // Dispatched when the promise returned from the author's update callback has
  // resolved and start phase of the animation can be initiated. Note: this is
  // called only if a callback is provided.
  void NotifyDOMCallbackFinished(bool success);

  ViewTransitionTypeSet* Types();

  void InitTypes(const Vector<String>&);

  // Returns true if `pseudo_element` is generated for this transition.
  bool IsGeneratingPseudo(
      const ViewTransitionPseudoElementBase& pseudo_element) const;

  Element* Scope() const { return scope_.Get(); }

  // The start of a VT cancels the previous transition; however, first VT's
  // DOM callback must still run. To avoid capturing its DOM changes are part
  // of the new VT, we postpone advancement of the state until the fist VT's
  // has started the DOM callback. We do not wait for completion as the callback
  // may be asynchronous and might never complete.
  void NotifySkippedTransitionDOMCallbackScheduled();
  void NotifyInvokeDOMChangeCallback();
  bool PendingDomCallback();

 private:
  friend class ViewTransitionTest;
  friend class AXViewTransitionTest;

  // Tracks how the ViewTransition object was created.
  enum class CreationType {
    // Created via the document.startViewTransition() script API.
    kScript,
    // Created when a navigation is initiated from the Document associated with
    // this ViewTransition.
    kForSnapshot,
    // Created when a navigation is initiated to the Document associated with
    // this ViewTransition.
    kFromSnapshot,
  };

  // Note the states are possibly overly verbose, and several states can
  // transition in one function call, but it's useful to keep track of what is
  // happening. For the most part, the states transition in order, with the
  // exception of us being able to jump to some terminal states from other
  // states.
  enum class State {
    // Initial state.
    kInitial,

    // Capture states.
    kCaptureTagDiscovery,
    kCaptureRequestPending,
    kCapturing,
    kCaptured,

    // Navigation specific states.
    kTransitionStateCallbackDispatched,
    kWaitForRenderBlock,

    // Callback states.
    kDOMCallbackRunning,
    kDOMCallbackFinished,

    // Animate states.
    kAnimateTagDiscovery,
    kAnimateRequestPending,
    kAnimating,

    // Terminal states.
    kFinished,
    kAborted,
    kTimedOut
  };
  static const char* StateToString(State state);

  // Advance to the new state. This returns true if the state should be
  // processed immediately.
  bool AdvanceTo(State state);

  bool CanAdvanceTo(State state) const;
  static bool StateRunsInViewTransitionStepsDuringMainFrame(State state);

  // Returns true if we're in a state that doesn't require explicit flow (i.e.
  // we don't need to post task or schedule a frame). We're waiting for some
  // external notifications, like capture is finished, or callback finished
  // running.
  static bool WaitsForNotification(State state);

  static bool IsTerminalState(State state);

  void ProcessCurrentState();

  void NotifyCaptureFinished(
      const std::unordered_map<viz::ViewTransitionElementResourceId,
                               gfx::RectF>&);

  // Used to defer visual updates between transition prepare dispatching and
  // transition start to allow the page to set up the final scene
  // asynchronously.
  void PauseRendering();
  void OnRenderingPausedTimeout();
  void ResumeRendering();

  // Cross-document navigations may span across multiple CompositorFrameSinks if
  // the old/new Documents render to different WebWidgets. This returns false if
  // the navigation triggering the transition is guaranteed to not change the
  // WebWidget.
  //
  // Same-document transitions triggered via the `startViewTransition` script
  // API are never cross frame sink.
  bool MaybeCrossFrameSink() const;

  void LogIfDocumentElementChanged() const;

  State state_ = State::kInitial;
  const CreationType creation_type_;

  Member<Document> document_;

  // For a scoped transition, this is the element scope.
  // For a document transition, this is the document element at the time the
  // ViewTransition was created.
  // TODO(crbug.com/394052227): Consider skipping the transition if the identity
  // of the document element changes.
  Member<Element> scope_;
  bool has_document_scope_ = false;

  Delegate* const delegate_ = nullptr;

  // Each transition is assigned a unique ID. For cross-document navigations
  // this is also the `transition_token` provided to the browser/GPU process to
  // track the lifetime of generated resources.
  const ViewTransitionToken transition_token_;

  Member<ViewTransitionStyleTracker> style_tracker_ = nullptr;

  // Manages pausing rendering of the Document between capture and updateDOM
  // callback finishing.
  // If the Document is the local root frame then the backing CC instance is
  // paused which stops compositor driven animations, videos, offscreen canvas
  // etc. Otherwise only main thread lifecycle updates are paused. We'd like to
  // pause compositor/Viz driven effects for nested frames as well but
  // selectively pausing animations for a CC instance is difficult.
  class ScopedPauseRendering {
   public:
    explicit ScopedPauseRendering(const Element&);
    ~ScopedPauseRendering();

    bool ShouldThrottleRendering() const;

   private:
    std::unique_ptr<cc::ScopedPauseRendering> cc_paused_;
  };
  std::optional<ScopedPauseRendering> rendering_paused_scope_;

  ViewTransitionStateCallback transition_state_callback_;

  // This is the object that implements the IDL interface exposed to script. It
  // is cleared if the document is torn down.
  Member<DOMViewTransition> script_delegate_;

  Member<ViewTransitionTypeSet> types_;

  // Synchronization of view-transitions. When starting a view transition, we
  // cancel the previously active one. These members are used to ensure proper
  // synchronization of the old and new transition. The old VT's DOM callback
  // must run before the new VT can start.
  Member<ViewTransition> blocked_on_;
  Member<ViewTransition> blocking_;
  bool pending_dom_callback_ = false;

  bool in_main_lifecycle_update_ = false;
  bool dom_callback_succeeded_ = false;
  bool first_animating_frame_ = true;
  bool context_destroyed_ = false;
  bool pending_skip_view_transitions_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_H_
