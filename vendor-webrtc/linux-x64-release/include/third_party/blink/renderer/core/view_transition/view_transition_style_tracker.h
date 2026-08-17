// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_STYLE_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_STYLE_TRACKER_H_

#include <unordered_map>

#include "base/containers/flat_map.h"
#include "base/unguessable_token.h"
#include "cc/layers/view_transition_content_layer.h"
#include "components/viz/common/view_transition_element_resource_id.h"
#include "third_party/blink/public/common/frame/view_transition_state.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/core/css/style_request.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/style/computed_style_base_constants.h"
#include "third_party/blink/renderer/core/style/style_view_transition_group.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/geometry/physical_size.h"
#include "third_party/blink/renderer/platform/graphics/paint/effect_paint_property_node.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/heap_traits.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "ui/gfx/geometry/transform.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {

class ClipPaintPropertyNode;
class PaintLayer;
class PseudoElement;

// This class manages the integration between ViewTransition and the style
// system which encompasses the following responsibilities :
//
// 1) Triggering style invalidation to change the DOM structure at different
//    stages during a transition. For example, pseudo elements for new-content
//    are generated after the new Document has loaded and the transition can be
//    started.
//
// 2) Tracking changes in the state of transition elements that are mirrored in
//    the style for their corresponding pseudo element. For example, if a
//    transition element's size or viewport space transform is updated. This
//    data is used to generate a dynamic UA stylesheet for these pseudo
//    elements.
//
// Note: The root element is special because its responsibilities are hoisted up
// to the LayoutView. For example, the root snapshot includes content from the
// root element and top layer elements.
// See
// https://drafts.csswg.org/css-view-transitions-1/#capture-the-image-algorithm.
// We avoid leaking this detail into this class by letting higher level code
// deal with this mapping.
//
// A new instance of this class is created for every transition.
class ViewTransitionStyleTracker
    : public GarbageCollected<ViewTransitionStyleTracker> {
 public:
  // Properties that transition on container elements.
  struct ContainerProperties {
    bool operator==(const ContainerProperties& other) const = default;

    // The rect used to compute the reference rect, which is what's eventually
    // used fo the projecting the content to the coordinate space of the
    // pseudo-element. It is in layer space, as the contents are captured in
    // layer space.
    PhysicalRect border_box_rect_in_enclosing_layer_css_space;

    // Transforms a point from local space into the snapshot viewport. For
    // details of the snapshot viewport, see README.md.
    gfx::Transform snapshot_matrix;

    PhysicalSize GroupSize() const {
      return border_box_rect_in_enclosing_layer_css_space.size;
    }
  };

  explicit ViewTransitionStyleTracker(
      Document& document,
      const blink::ViewTransitionToken& transition_token);
  ViewTransitionStyleTracker(Document& document, ViewTransitionState);
  ViewTransitionStyleTracker(
      Element& element,
      const blink::ViewTransitionToken& transition_token);
  ~ViewTransitionStyleTracker();

  void AddTransitionElementsFromCSS();

  // Returns true if the pseudo element corresponding to the given id and name
  // is the only child.
  bool MatchForOnlyChild(PseudoId pseudo_id,
                         const AtomicString& view_transition_name) const;

  // Indicate that capture was requested. This verifies that the combination of
  // set elements and names is valid. Returns true if capture phase started,
  // and false if the transition should be aborted. If `snap_browser_controls`
  // is set, browser controls will be forced to a fully shown state to ensure a
  // consistent state for cross-document transitiopns.
  bool Capture(bool snap_browser_controls);

  void SetCaptureRectsFromCompositor(
      const std::unordered_map<viz::ViewTransitionElementResourceId,
                               gfx::RectF>&);

  // Notifies when caching snapshots for elements in the old DOM finishes. This
  // is dispatched before script is notified to ensure this class releases any
  // references to elements in the old DOM before it is mutated by script.
  void CaptureResolved();

  // Indicate that start was requested. This verifies that the combination of
  // set elements and names is valid. Returns true if start phase started, and
  // false if the transition should be aborted.
  bool Start();

  // Notifies when the animation setup for the transition during Start have
  // finished executing.
  void StartFinished();

  // Dispatched if a transition is aborted. Must be called before "Start" stage
  // is initiated.
  void Abort();

  // Notifies when rendering is throttled for the local subframe associated with
  // this transition.
  void DidThrottleLocalSubframeRendering();

  // Returns the snapshot ID to identify the render pass based image produced by
  // this Element. Returns an invalid ID if this element is not participating in
  // the transition.
  viz::ViewTransitionElementResourceId GetSnapshotId(const Element&) const;

  // The layer used to paint the old Document rendered in a LocalFrame subframe
  // until the new Document can start rendering.
  const scoped_refptr<cc::ViewTransitionContentLayer>&
  GetSubframeSnapshotLayer() const;

  // Creates a PseudoElement for the corresponding |pseudo_id| and
  // |view_transition_name|. The |pseudo_id| must be a ::transition* element.
  PseudoElement* CreatePseudoElement(
      Element* parent,
      PseudoId pseudo_id,
      const AtomicString& view_transition_name) const;

  // Dispatched after the pre-paint lifecycle stage after each rendering
  // lifecycle update when a transition is in progress.
  // Returns false if the transition constraints were broken and the transition
  // should be skipped.
  bool RunPostPrePaintSteps();

  // Provides a UA stylesheet applied to ::transition* pseudo elements.
  CSSStyleSheet& UAStyleSheet();

  void Trace(Visitor* visitor) const;

  // Returns true if any of the pseudo elements are currently participating in
  // an animation.
  bool HasActiveAnimations() const;

  // Updates a clip node with the given state. The return value is a result of
  // updating the clip node.
  PaintPropertyChangeType UpdateCaptureClip(
      const Element& element,
      const ClipPaintPropertyNodeOrAlias* current_clip,
      const TransformPaintPropertyNodeOrAlias* current_transform);
  const ClipPaintPropertyNode* GetCaptureClip(const Element& element) const;

  int CapturedTagCount() const { return captured_name_count_; }

  // Returns true if `node` participates in this transition.
  bool IsTransitionElement(const Element& node) const;

  // Returns whether a clip node is required because `node`'s painting exceeds
  // max texture size.
  bool NeedsCaptureClipNode(const Element& node) const;

  std::vector<viz::ViewTransitionElementResourceId> TakeCaptureResourceIds() {
    return std::move(capture_resource_ids_);
  }

  // Returns whether styles applied to pseudo elements should be limited to UA
  // rules based on the current phase of the transition.
  StyleRequest::RulesToInclude StyleRulesToInclude() const;

  // Return non-root transitioning elements.
  VectorOf<Element> GetTransitioningElements() const;

  // In physical pixels. Returns the size of the snapshot root rect. This is
  // the fixed viewport size "as-if" all transient browser UI were hidden (e.g.
  // include the mobile URL bar, virutal-keyboard, layout scrollbars in the
  // rect size).
  gfx::Size GetSnapshotRootSize() const;

  // In physical pixels. Returns the offset from the fixed viewport's origin to
  // the snapshot root rect. These values will always be <= 0.
  gfx::Vector2d GetFixedToSnapshotRootOffset() const;

  // In physical pixels. Returns the offset from the frame origin to the the
  // snapshot root rect. The only time this currently differs from the above
  // offset is with a left-side vertical scrollbar (i.e. a vertical scrollbar
  // in an RTL document). In that case the frame origin is at x-coordinate 0
  // while the fixed viewport origin is inset by the scrollbar width.  Note: In
  // Chrome, only iframes can have a left-side vertical scrollbar.
  gfx::Vector2d GetFrameToSnapshotRootOffset() const;

  // Returns a serializable representation of the state cached by this class to
  // recreate the same pseudo-element tree in a new Document.
  ViewTransitionState GetViewTransitionState() const;

  // Returns if the current snapshot containing block size has changed since
  // the initial capture was taken.
  bool SnapshotRootDidChangeSize() const;

  // https://drafts.csswg.org/css-view-transitions-2/#captured-element-class-list
  // Returns the class list for a captured element by name.
  const Vector<AtomicString>& GetViewTransitionClassList(
      const AtomicString& name) const;

  const Vector<AtomicString>& GetViewTransitionNames() const {
    return view_transition_names_;
  }

  // This returns the resolved containing group name for a given view transition
  // name. Note that this only works once the transition starts.
  AtomicString GetContainingGroupName(const AtomicString& name) const;

 private:
  class ImageWrapperPseudoElement;

  // These state transitions are executed in a serial order unless the
  // transition is aborted.
  enum class State { kIdle, kCapturing, kCaptured, kStarted, kFinished };
  static const char* StateToString(State state);

  AtomicString ComputeContainingGroupName(
      const AtomicString& name,
      const StyleViewTransitionGroup& group) const;

  AtomicString GenerateAutoName(Element&, const TreeScope*, bool allow_from_id);

  struct ElementData : public GarbageCollected<ElementData> {
    void Trace(Visitor* visitor) const;

    // Returns the intrinsic size for the element's snapshot.
    gfx::RectF GetInkOverflowRect(bool use_cached_data) const;
    gfx::RectF GetCapturedSubrect(bool use_cached_data) const;

    // This is the geometry of the snapshot, in layer coordinate space, that is
    // going to be mapped to the old/new pseudo-element's content rect.
    gfx::RectF GetReferenceRect(bool use_cached_data,
                                float device_scale_factor) const;

    bool ShouldPropagateVisualOverflowRectAsMaxExtentsRect() const;

    // Caches the current state for the old snapshot.
    void CacheStateForOldSnapshot();

    // The element in the current DOM whose state is being tracked and mirrored
    // into the corresponding container pseudo element.
    Member<Element> target_element;

    // Computed info for each element participating in the transition for the
    // |target_element|. This information is mirrored into the UA stylesheet.
    // It is std::nullopt before it is populated for the first time.
    std::optional<ContainerProperties> container_properties;

    // Computed info cached before the DOM switches to the new state.
    ContainerProperties cached_container_properties;

    // Valid if there is an element in the old DOM generating a snapshot.
    viz::ViewTransitionElementResourceId old_snapshot_id;

    // Valid if there is an element in the new DOM generating a snapshot.
    viz::ViewTransitionElementResourceId new_snapshot_id;

    // A clip used to specify the subset of the `target_element`'s visual
    // overflow rect rendered into the element's snapshot.
    // TODO(khushalsagar): Move this to ObjectPaintProperties.
    Member<ClipPaintPropertyNode> clip_node;

    // Index to add to the view transition element id.
    int element_index;

    // The visual overflow rect for this element. This is used to compute
    // object-view-box if needed.
    // This rect is in layout space.
    PhysicalRect visual_overflow_rect_in_layout_space;
    PhysicalRect cached_visual_overflow_rect_in_layout_space;

    // A subset of the element's visual overflow rect which is painted into its
    // snapshot. Only populated if the element's painting needs to be clipped.
    // This rect is in layout space.
    std::optional<gfx::RectF> captured_rect_in_layout_space;
    std::optional<gfx::RectF> cached_captured_rect_in_layout_space;

    // For the following properties, they are initially set to the outgoing
    // element's value, and then switch to the incoming element's value, if one
    // exists.
    base::flat_map<CSSPropertyID, String> captured_css_properties;

    // This only contains properties that need to be animated, which is a
    // subset of `captured_css_properties`.
    base::flat_map<CSSPropertyID, String> cached_animated_css_properties;

    // https://drafts.csswg.org/css-view-transitions-2/#captured-element-class-list
    Vector<AtomicString> class_list;

    AtomicString containing_group_name;

    // Whether this name was auto-generated via auto/match-element.
    // Auto-generated names do not appear in reflection methods such as
    // getAnimations.
    bool is_generated_name;
  };

  // In physical pixels. Returns the snapshot root rect, relative to the
  // fixed viewport origin. See README.md for a detailed description of the
  // snapshot root rect.
  gfx::Rect GetSnapshotRootInFixedViewport() const;

  void InvalidateStyle();
  bool HasLiveNewContent() const;
  void EndTransition();

  void AddConsoleError(String message, Vector<DOMNodeId> related_nodes = {});
  void AddTransitionElement(Element*,
                            const AtomicString& name,
                            const AtomicString& nearest_containing_group,
                            const AtomicString& current_containing_group_name);
  bool FlattenAndVerifyElements(VectorOf<Element>&, VectorOf<AtomicString>&);

  void AddTransitionElementsFromCSSRecursive(
      PaintLayer*,
      const TreeScope*,
      Vector<AtomicString>& containing_group_stack,
      const AtomicString& nearest_group_with_contain);

  void InvalidateHitTestingCache();

  // Computes the visual overflow rect for the given box. If the ancestor is
  // specified, then the result is mapped to that ancestor space.
  PhysicalRect ComputeVisualOverflowRect(
      LayoutBoxModelObject& box,
      const LayoutBoxModelObject* ancestor = nullptr) const;

  // This corresponds to the state computed for keeping pseudo-elements in sync
  // with the state of live DOM elements described in
  // https://drafts.csswg.org/css-view-transitions-1/#style-transition-pseudo-elements-algorithm.
  void ComputeLiveElementGeometry(
      int max_capture_size,
      LayoutObject& layout_object,
      ContainerProperties&,
      PhysicalRect& visual_overflow_rect_in_layout_space,
      std::optional<gfx::RectF>& captured_rect_in_layout_space) const;

  // Computes a transform for the participant's border box relative to the
  // viewport in the case of a document transition, or the padding box rect of
  // the scope element in the case of a scoped transition.
  gfx::Transform ComputeTransformForParticipant(const LayoutObject&) const;

  viz::ViewTransitionElementResourceId GenerateResourceId(
      bool for_subframe_snapshot = false) const;

  void SnapBrowserControlsToFullyShown();

  // This computes the containing group name, validating that the given name is
  // found in one of the ancestors of the given element.
  AtomicString ComputeContainingGroupName(Element*) const;

  // This is the scope element in the case of a scoped transition, or the
  // root (html) element in the case of a document transition. It can be null
  // in the rare case that the root element has been removed from the DOM.
  Element* OriginatingElement() const;

  Member<Document> document_;

  Member<Element> element_;

  // Indicates which step during the transition we're currently at.
  State state_ = State::kIdle;

  const blink::ViewTransitionToken transition_token_;

  // Set if this style tracker was created by deserializing captured state
  // instead of running through the capture phase. This is done for transitions
  // initiated by navigations where capture and animation could run in different
  // Documents which are cross-process.
  const bool deserialized_ = false;

  // Tracks the number of names discovered during the capture phase of the
  // transition.
  int captured_name_count_ = 0;

  // Tracks the size of the snapshot root rect that was used to generate
  // snapshots. If the snapshot root changes size at any point in the
  // transition the transition will be aborted. For SPA transitions, this will
  // be empty until the kCapturing phase. For a cross-document transition, this
  // will be initialized from the cached state at creation but is currently
  // unset.
  std::optional<gfx::Size> snapshot_root_layout_size_at_capture_;

  // Map of the CSS |view-transition-name| property to state for that tag.
  HeapHashMap<AtomicString, Member<ElementData>> element_data_map_;

  // The device scale factor used for layout of the Document. This is kept in
  // sync with the Document during RunPostPrePaintSteps().
  float device_pixel_ratio_ = 0.f;

  // The dynamically generated UA stylesheet for default styles on
  // pseudo-elements.
  Member<CSSStyleSheet> ua_style_sheet_;

  // The following state is buffered until the capture phase and populated again
  // by script for the start phase.
  int set_element_sequence_id_ = 0;
  HeapHashMap<Member<Element>, HashSet<std::pair<AtomicString, int>>>
      pending_transition_element_names_;

  // This vector is passed as constructed to cc's view transition request,
  // so this uses the std::vector for that reason, instead of WTF::Vector.
  std::vector<viz::ViewTransitionElementResourceId> capture_resource_ids_
      ALLOW_DISCOURAGED_TYPE("cc API uses STL types");

  // Caches whether the root element is currently participating in the
  // transition. This is a purely performance optimization since this check is
  // used in hot code-paths.
  bool is_root_transitioning_ = false;

  // Set if this transition is in a LocalFrame sub-frame, when the capture is
  // initiated until the start phase of the animation.
  scoped_refptr<cc::ViewTransitionContentLayer> subframe_snapshot_layer_;

  // Returns true if GetViewTransitionState() has already been called. This is
  // used only to enforce additional captures don't happen after that.
  mutable bool state_extracted_ = false;

  // Used to save hierarchical information about the nearest
  // view-transition-group as well as the nearest one with "contain".
  struct AncestorGroupNames {
    AtomicString nearest;

    // This is the nearest ancestor view-transition-name that is set to contain
    // all of its descendants.
    AtomicString contain;
  };
  HashMap<AtomicString, AncestorGroupNames> group_state_map_;

  base::Token token_;

  HashMap<AtomicString, AtomicString> id_to_auto_name_map_;

  Vector<AtomicString> view_transition_names_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_STYLE_TRACKER_H_
