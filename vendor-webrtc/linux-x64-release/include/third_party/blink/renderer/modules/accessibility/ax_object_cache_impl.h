/*
 * Copyright (C) 2014, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_OBJECT_CACHE_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_OBJECT_CACHE_IMPL_H_

#include <memory>
#include <utility>

#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/mojom/render_accessibility.mojom-blink.h"
#include "third_party/blink/public/web/web_ax_enums.h"
#include "third_party/blink/renderer/core/accessibility/ax_object_cache_base.h"
#include "third_party/blink/renderer/core/accessibility/axid.h"
#include "third_party/blink/renderer/core/accessibility/blink_ax_event_intent.h"
#include "third_party/blink/renderer/core/editing/commands/selection_for_undo_step.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/frame/local_frame_view.h"
#include "third_party/blink/renderer/core/html/forms/html_select_element.h"
#include "third_party/blink/renderer/core/layout/hit_test_result.h"
#include "third_party/blink/renderer/core/layout/layout_block_flow.h"
#include "third_party/blink/renderer/modules/accessibility/aria_notification.h"
#include "third_party/blink/renderer/modules/accessibility/ax_block_flow_iterator.h"
#include "third_party/blink/renderer/modules/accessibility/ax_object.h"
#include "third_party/blink/renderer/modules/accessibility/ax_object_cache_lifecycle.h"
#include "third_party/blink/renderer/modules/accessibility/blink_ax_tree_source.h"
#include "third_party/blink/renderer/modules/accessibility/inspector_accessibility_agent.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/weak_cell.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/accessibility/ax_enums.mojom-blink-forward.h"
#include "ui/accessibility/ax_error_types.h"
#include "ui/accessibility/ax_location_and_scroll_updates.h"
#include "ui/accessibility/ax_mode.h"
#include "ui/accessibility/ax_tree_id.h"
#include "ui/accessibility/ax_tree_serializer.h"

namespace blink {

class AXBlockFlowData;
class AXRelationCache;
class AbstractInlineTextBox;
class HTMLAreaElement;
class WebLocalFrameClient;

// Describes a decision on whether to create an AXNodeObject with or without a
// LayoutObject, or to prune the AX subtree at that point. Only pseudo element
// descendants are missing DOM nodes.
enum AXObjectType { kPruneSubtree = 0, kCreateFromNode, kCreateFromLayout };

struct TextChangedOperation {
  TextChangedOperation()
      : start(0),
        end(0),
        start_anchor_id(0),
        end_anchor_id(0),
        op(ax::mojom::blink::Command::kNone) {}
  TextChangedOperation(int start_in,
                       int end_in,
                       AXID start_id_in,
                       AXID end_id_in,
                       ax::mojom::blink::Command op_in)
      : start(start_in),
        end(end_in),
        start_anchor_id(start_id_in),
        end_anchor_id(end_id_in),
        op(op_in) {}
  int start;
  int end;
  AXID start_anchor_id;
  AXID end_anchor_id;
  ax::mojom::blink::Command op;
};

// This class should only be used from inside the accessibility directory.
class MODULES_EXPORT AXObjectCacheImpl : public AXObjectCacheBase {
 public:
  static AXObjectCache* Create(Document&, const ui::AXMode&);

  AXObjectCacheImpl(Document&, const ui::AXMode&);

  AXObjectCacheImpl(const AXObjectCacheImpl&) = delete;
  AXObjectCacheImpl& operator=(const AXObjectCacheImpl&) = delete;

  ~AXObjectCacheImpl() override;
  void Trace(Visitor*) const override;

  // The main document.
  Document& GetDocument() const { return *document_; }
  // The popup document, if showing, otherwise null.
  Document* GetPopupDocumentIfShowing() const { return popup_document_.Get(); }
  // Get the focused object or an ancestor if that is not possible.
  // Use this if side effects are not ok, otherwise use EnsureFocusedObject().
  // * In rare cases, the focused element might not have an associated a11y
  // object if it's the descendant of a pruned subtree. In this case, return the
  // root as the focus. This is more likely more likely in a fuzzer than in real
  // content (e.g. focus is inside of an unused image map's <area>).
  // * If the focus is on an AXObject that's not part of the included subtree
  // that will be serialized, return the included ancestor. This is rare because
  // focusable objects are generally part of the included subtree.
  AXObject* FocusedObject() const;
  // Always return a focused object, even if it's in an aria-hidden subtree.
  // Use this method if side effects are ok, otherwise use FocusedObject().
  // * If the focus is inside an aria-hidden subtree, discard the illegal
  // aria-hidden and restore the subtree, then return the new AXObject.
  AXObject* EnsureFocusedObject();

  const ui::AXMode& GetAXMode() override;
  void SetAXMode(const ui::AXMode&) override;

  const AXObjectCacheLifecycle& lifecycle() const { return lifecycle_; }

  // When the accessibility tree view is open in DevTools, we listen for changes
  // to the tree by registering an InspectorAccessibilityAgent here and notify
  // the agent when AXEvents are fired or nodes are marked dirty.
  void AddInspectorAgent(InspectorAccessibilityAgent*);
  void RemoveInspectorAgent(InspectorAccessibilityAgent*);

  // Ensure that a full document lifecycle will occur, which in turn ensures
  // that a call to CommitAXUpdates() will occur soon.
  void ScheduleAXUpdate() const override;
  // Same as `ScheduleAXUpdate()` but will call `callback` once
  // `CompleteAXUpdate` is done.
  void ScheduleAXUpdateWithCallback(base::OnceClosure callback);

  void Dispose() override;

  // Freeze that AXObject tree and do not allow changes until Thaw() is called.
  // Prefer ScopedFreezeAXCache where possible.
  void Freeze() override {
    if (frozen_count_++) {
      // Already frozen.
      return;
    }
    ax_tree_source_->Freeze();

    // Force a cache reset for mutable cached object properties. Any property
    // values cached while the tree is frozen is valid until the next thaw.
    IncrementGenerationalCacheId();

    CHECK(FocusedObject());
    DUMP_WILL_BE_CHECK(!IsDirty());
  }
  void Thaw() override {
    CHECK_GE(frozen_count_, 1);
    if (--frozen_count_ == 0) {
      ax_tree_source_->Thaw();
      ClearCachedNodesOnLine();
    }
  }
  bool IsFrozen() const override { return frozen_count_; }

  void SelectionChanged(Node*) override;

  // Uses the relation cache to check whether the current element is pointed to
  // by aria-labelledby or aria-describedby.
  bool IsLabelOrDescription(Element&);

  // Effects a ChildrenChanged() on the passed-in object, if unignored,
  // otherwise, uses the first unignored ancestor. Returns the object that the
  // children changed occurs on.
  AXObject* ChildrenChanged(AXObject*);
  void ChildrenChangedWithCleanLayout(AXObject*);
  void ChildrenChanged(Node*) override;
  void ChildrenChanged(const LayoutObject*) override;
  void SlotAssignmentWillChange(Node*) override;
  void CheckedStateChanged(Node*) override;
  void ListboxOptionStateChanged(HTMLOptionElement*) override;
  void ListboxSelectedChildrenChanged(HTMLSelectElement*) override;
  void ListboxActiveIndexChanged(HTMLSelectElement*) override;
  void SetMenuListOptionsBounds(HTMLSelectElement*,
                                const WTF::Vector<gfx::Rect>&) override;
  // Return the bounds for <option>s in an open <select>, or nullptr if they
  // are not available.
  const WTF::Vector<gfx::Rect>* GetOptionsBounds(
      const AXObject& ax_menu_list) const;

  // Return true if the node has previously had aria-hidden="true" that was used
  // illegally, e.g. focus went inside of it.
  bool HasBadAriaHidden(const AXObject&) const;
  // Mark any aria-hidden ancestors of this object as "bad", ignoring their
  // aria-hidden markup from this point forward, and rebuild the top aria-hidden
  // element's subtree without the aria-hidden markup.
  void DiscardBadAriaHiddenBecauseOfFocus(AXObject& focus);
  // Mark an aria-hidden usage as bad/discarded when used on <body>/<html>/etc.
  void DiscardBadAriaHiddenBecauseOfElement(const AXObject& obj);

  // Implicit selection aka "selection follows focus" is not allowed on
  // containers with subwidgets that have had checked or selected, or expanded
  // in the case of tabs.
  bool IsImplicitSelectionAllowed(const AXObject* container);

  void ImageLoaded(const LayoutObject*) override;

  // Removes AXObject backed by passed-in object, if there is one.
  // It will also notify the parent that its children have changed, so that the
  // parent will recompute its children and be reserialized.
  void Remove(Node*) override;
  void RemovePopup(Document*) override;
  void Remove(AbstractInlineTextBox*) override;
  // Remove an AXObject or its subtree, and if |notify_parent| is true,
  // recompute the parent's children and reserialize the parent.
  void Remove(AXObject*, bool notify_parent);
  void Remove(Node*, bool notify_parent);

  // This will remove all AXObjects in the subtree, whether they or not they are
  // marked as included for serialization. This can only be called while flat
  // tree traversal is safe and there are no slot assignments pending.
  // To remove only included nodes, use RemoveIncludedSubtree(), which can be
  // called at any time.
  // If |remove_root|, remove the root of the subtree, otherwise only
  // descendants are removed.
  // If |notify_parent|, call ChildrenChanged() on the parent.
  // If |only_layout_objects|, will only remove nodes in the subtree that
  // corresponded with an AXLayoutObject (useful for subtrees that lose layout).
  void RemoveSubtree(const Node*) override;
  void RemoveSubtree(const Node*, bool remove_root) override;
  void RemoveSubtree(const Node*, bool remove_root, bool notify_parent);

  // Remove the cached subtree of included AXObjects. If |remove_root| is false,
  // then only descendants will be removed. To remove unincluded AXObjects as
  // well, call RemoveSubtree().
  // If |remove_root|, remove the root of the subtree, otherwise only
  // descendants are removed.
  void RemoveIncludedSubtree(AXObject* object, bool remove_root);
  // Remove all AXObjects in the layout subtree of node, and notify the parent.
  void RemoveAXObjectsInLayoutSubtree(LayoutObject* layout_object) override;

  // For any ancestor that could contain the passed-in AXObject* in their cached
  // children, clear their children and set needs to update children on them.
  // In addition, ChildrenChanged() on an included ancestor that might contain
  // this child, if one exists.
  void ChildrenChangedOnAncestorOf(AXObject*);

  const Element* RootAXEditableElement(const Node*) override;

  // Called when aspects of the style (e.g. color, alignment) change.
  void StyleChanged(const LayoutObject*,
                    bool visibility_or_inertness_changed) override;

  // Called when the anchor(s) of |positioned_obj| change.
  void CSSAnchorChanged(const LayoutObject* positioned_obj) override;
  void CSSAnchorChangedWithCleanLayout(Node* positioned_node);

  // Called by a node when text or a text equivalent (e.g. alt) attribute is
  // changed.
  void TextChanged(const LayoutObject*) override;
  void TextChangedWithCleanLayout(Node* optional_node, AXObject*);

  // Called when fragments in the LayoutBlockFlow associated with
  // `object`changed.
  void ClearBlockFlowCachedData(const LayoutObject* object) override;

  void DocumentTitleChanged() override;

  // Returns true if we can immediately process tree updates for this node.
  // The main reason we cannot is lacking enough context to determine the
  // relevance of a whitespace node.
  bool IsReadyToProcessTreeUpdatesForNode(const Node*);
  // Called when a node is connected to the document.
  void NodeIsConnected(Node*) override;
  // Called when a node is attached to the layout tree.
  void NodeIsAttached(Node*) override;
  // Called when a subtree is attached to the layout tree because of
  // content-visibility or previously display:none content gaining layout.
  void SubtreeIsAttached(Node*) override;

  void HandleAttributeChanged(const QualifiedName& attr_name,
                              Element*) override;
  void HandleValidationMessageVisibilityChanged(Node* form_control) override;
  void HandleEventListenerAdded(Node& node,
                                const AtomicString& event_type) override;
  void HandleEventListenerRemoved(Node& node,
                                  const AtomicString& event_type) override;
  void HandleReferenceTargetChanged(Element&) override;
  void HandleFocusedUIElementChanged(Element* old_focused_element,
                                     Element* new_focused_element) override;
  void HandleInitialFocus() override;
  void HandleTextFormControlChanged(Node*) override;
  void HandleEditableTextContentChanged(Node*) override;
  void HandleDeletionOrInsertionInTextField(
      const SelectionInDOMTree& changed_selection,
      bool is_deletion) override;
  void HandleTextMarkerDataAdded(Node* start, Node* end) override;
  void HandleValueChanged(Node*) override;
  void HandleUpdateActiveMenuOption(Node*) override;
  void DidShowMenuListPopup(Node*) override;
  void DidHideMenuListPopup(Node*) override;
  void HandleLoadStart(Document*) override;
  void HandleLoadComplete(Document*) override;
  void HandleClicked(Node*) override;

  void HandleAriaNotification(const Node*,
                              const String&,
                              const AriaNotificationOptions*) override;

  // Returns the ARIA notifications associated to a given `AXObject` and
  // releases them from `aria_notifications_`. If there are no notifications
  // stored for the given object, returns an empty `AriaNotifications`.
  AriaNotifications RetrieveAriaNotifications(const AXObject*) override;

  void SetCanvasObjectBounds(HTMLCanvasElement*,
                             Element*,
                             const PhysicalRect&) override;

  // If the object referenced by `ax_id` is part of a canvas and had explicit
  // bounds set, this returns the bounds for the object as well as the id of the
  // canvas owner of this object.
  std::optional<std::pair<PhysicalRect, AXID>> GetCanvasElementBounds(
      AXID ax_id);

  std::optional<ui::AXTreeID> GetAXObjectChildAXTreeID(AXID ax_id);
  void SetAXObjectChildTreeID(AXID ax_id, const ui::AXTreeID& tree_id) {
    ax_id_to_child_tree_id_.Set(ax_id, tree_id);
  }

  void InlineTextBoxesUpdated(LayoutObject*) override;

  // Get the amount of time, in ms, that event processing should be deferred
  // in order to more efficiently batch changes.
  int GetDeferredEventsDelay() const;

  // Get the amount of time, in ms, that location serialization should be
  // deferred in order to more efficiently batch changes.
  int GetLocationSerializationDelay();

  // Called during the accessibility lifecycle to refresh the AX tree.
  void CommitAXUpdates(Document&, bool force) override;

  // Called when a HTMLFrameOwnerElement (such as an iframe element) changes the
  // embedding token of its child frame.
  void EmbeddingTokenChanged(HTMLFrameOwnerElement*) override;

  // Called when the scroll offset changes.
  void HandleScrollPositionChanged(LayoutObject*) override;

  void HandleScrolledToAnchor(const Node* anchor_node) override;

  // Invalidates the bounding box, which can be later retrieved by
  // SerializeLocationChanges.
  void InvalidateBoundingBox(const LayoutObject*) override;
  void InvalidateBoundingBox(const AXID&);

  void SetCachedBoundingBox(AXID id,
                            const ui::AXRelativeBounds& bounds,
                            const int scroll_x,
                            const int scroll_y);

  const AtomicString& ComputedRoleForNode(Node*) override;
  String ComputedNameForNode(Node*) override;

  void OnTouchAccessibilityHover(const gfx::Point&) override;

  AXObject* ObjectFromAXID(AXID id) const override;
  AXObject* FirstObjectWithRole(ax::mojom::blink::Role role) override;
  AXObject* Root() override;

  // Create an AXObject, and do not check if a previous one exists.
  // Also, initialize the object and add it to maps for later retrieval.
  AXObject* CreateAndInit(Node*, LayoutObject*, AXObject* parent);

  // Note that these functions do NOT guarantee that an AXObject will
  // be created. For instance, not all HTMLElements can have an AXObject,
  // such as <head> or <script> tags.
  AXObject* GetOrCreate(LayoutObject*, AXObject* parent);
  AXObject* GetOrCreate(LayoutObject* layout_object);
  AXObject* GetOrCreate(const Node*, AXObject* parent) override;
  AXObject* GetOrCreate(Node*, AXObject* parent);
  AXObject* GetOrCreate(AbstractInlineTextBox*, AXObject* parent);
  AXObject* GetOrCreate(AXBlockFlowIterator::FragmentIndex index,
                        AXObject* parent);

  AXObject* Get(AbstractInlineTextBox*) const;
  AXObject* Get(const LayoutObject* object,
                AXBlockFlowIterator::FragmentIndex index) const;

  // Get an AXObject* backed by the passed-in DOM node.
  AXObject* Get(const Node*) const override;

  // Get an AXObject* backed by the passed-in LayoutObject, or the
  // LayoutObject's DOM node, if that is available.
  // If |parent_for_repair| is provided, and the object had been detached from
  // its parent, it will be set as the new parent.
  AXObject* Get(const LayoutObject*,
                AXObject* parent_for_repair = nullptr) const;

  // Return the object that has been anchored (with css anchor positioning)
  // to the input object.
  AXObject* GetPositionedObjectForAnchor(const AXObject*);

  // Return the input object's anchor.
  AXObject* GetAnchorForPositionedObject(const AXObject*);

  // Return true if the object is still part of the tree, meaning that ancestors
  // exist or can be repaired all the way to the root.
  bool IsStillInTree(AXObject*);

  void ChildrenChangedWithCleanLayout(Node* optional_node_for_relation_update,
                                      AXObject*);

  // Mark an object or subtree dirty, aka its properties have changed and it
  // needs to be reserialized. Use the |*WithCleanLayout| versions when layout
  // is already known to be clean.
  void MarkAXObjectDirty(AXObject*);

  void MarkAXObjectDirtyWithCleanLayout(AXObject*);

  void MarkAXSubtreeDirtyWithCleanLayout(AXObject*);
  void MarkSubtreeDirty(Node*);
  void NotifySubtreeDirty(AXObject* obj);

  // Set the parent of the AXObject associated with |child|. If no parent is
  // possible, this means the child can no longer be in the AXTree, so remove
  // any AXObject subtree associated with the child.
  void RestoreParentOrPrune(Node* child_node);
  void RestoreParentOrPruneWithCleanLayout(Node* child_node);

  // When an object is created or its id changes, this must be called so that
  // the relation cache is updated.
  void MaybeNewRelationTarget(Node& node, AXObject* obj);

  void HandleActiveDescendantChangedWithCleanLayout(Node*);
  void SectionOrRegionRoleMaybeChangedWithCleanLayout(Node*);
  void TableCellRoleMaybeChanged(Node* node);
  void HandleRoleMaybeChangedWithCleanLayout(Node*);
  void HandleRoleChangeWithCleanLayout(Node*);
  void HandleAriaExpandedChangeWithCleanLayout(Node*);
  void HandleAriaSelectedChangedWithCleanLayout(Node*);
  void HandleAriaPressedChangedWithCleanLayout(Node*);
  void HandleNodeLostFocusWithCleanLayout(Node*);
  void HandleNodeGainedFocusWithCleanLayout(Node*);
  void NodeIsAttachedWithCleanLayout(Node*);
  void HandleValidationMessageVisibilityChangedWithCleanLayout(const Node*);
  void HandleEditableTextContentChangedWithCleanLayout(Node*);
  void UpdateAriaOwnsWithCleanLayout(Node*);
  void UpdateTableRoleWithCleanLayout(Node*);
  void HandleUpdateMenuListPopupWithCleanLayout(Node*, bool did_show = false);

  AXID GenerateAXID() const override;

  void PostNotification(const LayoutObject*, ax::mojom::blink::Event);
  void PostNotification(Node*, ax::mojom::blink::Event);
  void PostNotification(AXObject*, ax::mojom::blink::Event);

  //
  // Aria-owns support.
  //

  // Returns true if the given object's position in the tree was due to
  // aria-owns.
  bool IsAriaOwned(const AXObject*, bool checks = true) const;

  // Returns the parent of the given object due to aria-owns, if valid.
  AXObject* ValidatedAriaOwner(const AXObject*) const;

  // Given an object that has an aria-owns attribute, return the validated
  // set of aria-owned children.
  void ValidatedAriaOwnedChildren(const AXObject* owner,
                                  HeapVector<Member<AXObject>>& owned_children);

  // Given a <map> element, get the image currently associated with it, if any.
  AXObject* GetAXImageForMap(const HTMLMapElement& map);

  // Adds |object| to |fixed_or_sticky_node_ids_| if it has a fixed or sticky
  // position.
  void AddToFixedOrStickyNodeList(const AXObject* object);

  bool MayHaveHTMLLabel(const HTMLElement& elem);

  // For built-in HTML form validation messages.
  AXObject* ValidationMessageObjectIfInvalid();

  WebAXAutofillSuggestionAvailability GetAutofillSuggestionAvailability(
      AXID id) const;
  void SetAutofillSuggestionAvailability(
      AXID id,
      WebAXAutofillSuggestionAvailability suggestion_availability);

  // Plugin support. These could in (along with the tree source/serializer
  // fields) move to their own subclass of AXObject.
  void AddPluginTreeToUpdate(ui::AXTreeUpdate* update);
  ui::AXTreeSource<const ui::AXNode*, ui::AXTreeData*, ui::AXNodeData>*
  GetPluginTreeSource();
  void SetPluginTreeSource(
      ui::AXTreeSource<const ui::AXNode*, ui::AXTreeData*, ui::AXNodeData>*
          source);
  ui::AXTreeSerializer<const ui::AXNode*,
                       std::vector<const ui::AXNode*>,
                       ui::AXTreeUpdate*,
                       ui::AXTreeData*,
                       ui::AXNodeData>*
  GetPluginTreeSerializer();
  void ResetPluginTreeSerializer();
  void MarkPluginDescendantDirty(ui::AXNodeID node_id);

  std::pair<ax::mojom::blink::EventFrom, ax::mojom::blink::Action>
  active_event_from_data() const {
    return std::make_pair(active_event_from_, active_event_from_action_);
  }

  void set_active_event_from_data(
      const ax::mojom::blink::EventFrom event_from,
      const ax::mojom::blink::Action event_from_action) {
    active_event_from_ = event_from;
    active_event_from_action_ = event_from_action;
  }

  Element* GetActiveAriaModalDialog() const;

  static bool IsRelevantPseudoElement(const Node& node);
  static bool IsRelevantPseudoElementDescendant(
      const LayoutObject& layout_object);
  static bool IsRelevantSlotElement(const HTMLSlotElement& slot);
  static Node* GetClosestNodeForLayoutObject(const LayoutObject* layout_object);

  // Token to return this token in the next IPC, so that RenderFrameHostImpl
  // can discard stale data, when the token does not match the expected token.
  std::optional<uint32_t> reset_token_;
  void SetSerializationResetToken(uint32_t token) override {
    reset_token_ = token;
  }

  // Retrieves a vector of all AXObjects whose bounding boxes may have changed
  // since the last query. Note that this function is destructive and clears the
  // vector so that the next time it's called, it will only retrieve objects
  // that have changed since now.
  ui::AXLocationAndScrollUpdates TakeLocationChangsForSerialization();

  // Sends the location changes over mojo to the browser process.
  void SerializeLocationChanges();

  // This method is used to fulfill AXTreeSnapshotter requests.
  bool SerializeEntireTree(
      size_t max_node_count,
      base::TimeDelta timeout,
      ui::AXTreeUpdate*,
      std::set<ui::AXSerializationErrorFlag>* out_error = nullptr) override;

  // Marks an object as dirty to be serialized in the next serialization.
  // If |subtree| is true, the entire subtree is dirty.
  // |event_from| and |event_from_action| annotate this node change with info
  // about the event which caused the change. For example, an event from a user
  // or an event from a focus action.
  void AddDirtyObjectToSerializationQueue(
      const AXObject* obj,
      ax::mojom::blink::EventFrom event_from =
          ax::mojom::blink::EventFrom::kNone,
      ax::mojom::blink::Action event_from_action =
          ax::mojom::blink::Action::kNone,
      const std::vector<ui::AXEventIntent>& event_intents = {});

  void GetUpdatesAndEventsForSerialization(
      std::vector<ui::AXTreeUpdate>& updates,
      std::vector<ui::AXEvent>& events,
      bool& had_end_of_test_event,
      bool& had_load_complete_messages);

  void GetImagesToAnnotate(ui::AXTreeUpdate& updates,
                           std::vector<ui::AXNodeData*>& nodes) override;

  // The difference between this and IsDirty():
  // - IsDirty() means there are updates to be processed when layout becomes
  // clean, in order to have a complete representation in the tree structure.
  // - HasObjectsPendingSerialization() means there are updates ready to be sent
  // to the serializer.
  bool HasObjectsPendingSerialization() const override {
    return !pending_objects_to_serialize_.empty();
  }
  bool IsDirty() override;

  // The generation ID is used in conjunction with a temporary cache to store
  // results for repeated calculations during the serialization process that
  // are immutable during the update process.
  uint64_t GenerationalCacheId() { return generational_cache_id_; }

  // Set the id of the node to fetch image data for. Normally the content
  // of images is not part of the accessibility tree, but one node at a
  // time can be designated as the image data node, which will send the
  // contents of the image with each accessibility update until another
  // node is designated.
  void SetImageAsDataNodeId(AXID id, const gfx::Size& max_size) {
    image_data_node_id_ = id;
    max_image_data_size_ = max_size;
  }

  AXID image_data_node_id() { return image_data_node_id_; }
  const gfx::Size& max_image_data_size() { return max_image_data_size_; }

  static constexpr int kDataTableHeuristicMinRows = 20;

  void UpdateAXForAllDocuments() override;
  void MarkDocumentDirty() override;
  void ResetSerializer() override;
  void MarkElementDirty(const Node*) override;
  void MarkElementDirtyWithCleanLayout(const Node*);

  // Returns true if FinalizeTree() has been called and has not finished.
  bool IsUpdatingTree() {
    return lifecycle_.GetState() == AXObjectCacheLifecycle::kFinalizingTree;
  }
  // The cache is in the tear-down phase.
  bool IsDisposing() const {
    return lifecycle_.GetState() == AXObjectCacheLifecycle::kDisposing;
  }
  // The cache has released all data and is no longer processing updates.
  bool HasBeenDisposed() const {
    return lifecycle_.GetState() == AXObjectCacheLifecycle::kDisposed;
  }

  bool EntireDocumentIsDirty() const { return mark_all_dirty_; }
  // Assert that tree is completely up-to-date.
  void CheckTreeIsFinalized();
  void CheckStyleIsComplete(Document& document) const;

  bool SerializeUpdatesAndEvents();

  const AXBlockFlowData* GetBlockFlowData(const AXObject* ax_object);

  // Returns the `TextChangedOperation` associated with the `id` from the
  // `text_operation_in_node_ids_` map, if `id` is in the map.
  WTF::Vector<TextChangedOperation>* GetFromTextOperationInNodeIdMap(AXID id);

  // Clears the map after each call, should be called after each serialization.
  void ClearTextOperationInNodeIdMap();

  // Adds an event to the list of pending_events_ and mark the object as dirty
  // via AXObjectCache::AddDirtyObjectToSerializationQueue. If
  // immediate_serialization is set, it schedules a serialization to be done at
  // the next available time without delays.
  void AddEventToSerializationQueue(const ui::AXEvent& event,
                                    bool immediate_serialization) override;

  // Called from browser to RAI and then to AXCache to notify that a
  // serialization has arrived to Browser.
  void OnSerializationReceived() override;

  // Used by outside classes to determine if a serialization is in the process
  // or not.
  bool IsSerializationInFlight() const override;

  // Used by outside classes, mainly RenderAccessibilityImpl, to inform
  // AXObjectCacheImpl that a serialization was cancelled.
  void OnSerializationCancelled() override;

  // Used by outside classes, mainly RenderAccessibilityImpl, to inform
  // AXObjectCacheImpl that a serialization was sent.
  void OnSerializationStartSend() override;

  Node* GetAccessibilityFocus() const override;

#if AX_FAIL_FAST_BUILD()
  // This is called after a node's included status changes, to update the
  // included_node_count_ which is used to debug tree mismatches between the the
  // AXObjectCache and AXTreeSerializer.
  void UpdateIncludedNodeCount(const AXObject* obj);
  size_t GetIncludedNodeCount() const { return included_node_count_; }
  void UpdatePluginIncludedNodeCount();
  size_t GetPluginIncludedNodeCount() const {
    return plugin_included_node_count_;
  }
  HeapHashMap<AXID, Member<AXObject>>& GetObjects() { return objects_; }

  // Used to turn on accessibility checks for internal Web UI, e.g. history,
  // preferences, etc. Will trigger DCHECKS so that WebUI with basic a11y errors
  // fail tests.
  // TODO(accessibility) Use for more things that have 0% false positives, such
  // as focusable objects requiring a name.
  bool IsInternalUICheckerOn(const AXObject& obj) const;
#endif  // AX_FAIL_FAST_BUILD()

  // Used to turn on accessibility checks for internal Web UI, e.g. history,
  // preferences, etc. Will trigger DCHECKS so that WebUI with basic a11y errors
  // fail tests.
  // TODO(accessibility) Use for more things that have 0% false positives, such
  // as focusable objects requiring a name.
  bool IsInternalUICheckerOn() const { return internal_ui_checker_on_; }

  struct TreeUpdateParams final : public GarbageCollected<TreeUpdateParams> {
    TreeUpdateParams(
        Node* node_arg,
        AXID axid_arg,
        ax::mojom::blink::EventFrom event_from_arg,
        ax::mojom::blink::Action event_from_action_arg,
        const BlinkAXEventIntentsSet& intents_arg,
        TreeUpdateReason update_reason_arg,
        ax::mojom::blink::Event event_arg = ax::mojom::blink::Event::kNone)
        : node(node_arg),
          axid(axid_arg),
          event(event_arg),
          event_from(event_from_arg),
          update_reason(update_reason_arg),
          event_from_action(event_from_action_arg) {
      for (const auto& intent : intents_arg) {
        DCHECK(node || axid) << "Either a DOM Node or AXID is required.";
        DCHECK(!node || !axid) << "Provide a DOM Node *or* AXID, not both.";
        event_intents.insert(intent.key, intent.value);
      }
    }

    // Only either node or AXID will be filled at a time. Some events use Node
    // while others use AXObject.
    WeakMember<Node> node;
    AXID axid;

    ax::mojom::blink::Event event;
    ax::mojom::blink::EventFrom event_from;
    TreeUpdateReason update_reason;
    ax::mojom::blink::Action event_from_action;
    BlinkAXEventIntentsSet event_intents;

    ~TreeUpdateParams() = default;
    void Trace(Visitor* visitor) const { visitor->Trace(node); }
    std::string ToString();
  };

  // Computes and links all nodes backed by a LayoutObject, and stores this
  // information in next|previous_on_line_map_. If a node is already part of the
  // map, skips the computation. Also see Next|PreviousOnLine() for where this
  // information is used.
  void ComputeNodesOnLine(const LayoutObject* layout_object);

  bool HasCachedDataForNodesOnLine() const {
    return !processed_blocks_.empty();
  }

  // Helper method to clear the cached data to compute next/previous on line.
  void ClearCachedNodesOnLine();

  // Returns the next LayoutObject that is in the same line as `layout_object`,
  // nullptr if it is the last object or if it is not part of a line.
  const LayoutObject* CachedNextOnLine(const LayoutObject* layout_object);

  // Returns the previous LayoutObject that is in the same line as
  // `layout_object`, nullptr if it is the first object or if it is not part of
  // a line.
  const LayoutObject* CachedPreviousOnLine(const LayoutObject* layout_object);

  // Updates the node on which the browser last requested accessibility focus.
  void UpdateAccessibilityFocus(AXID id) { accessibility_focus_ = id; }

#if AX_FAIL_FAST_BUILD()
  void AddNodeRequiringCacheUpdate(AXID ax_id, TreeUpdateReason reason);
  void RemoveNodeRequiringCacheUpdate(AXID ax_id);
#endif

 protected:
  void ScheduleImmediateSerialization() override;

  void PostPlatformNotification(
      AXObject* obj,
      ax::mojom::blink::Event event_type,
      ax::mojom::blink::EventFrom event_from =
          ax::mojom::blink::EventFrom::kNone,
      ax::mojom::blink::Action event_from_action =
          ax::mojom::blink::Action::kNone,
      const BlinkAXEventIntentsSet& event_intents = BlinkAXEventIntentsSet());

  // Returns a reference to the set of currently active event intents.
  BlinkAXEventIntentsSet& ActiveEventIntents() override {
    return active_event_intents_;
  }

#if AX_FAIL_FAST_BUILD()
  const HashMap<AXID, TreeUpdateReason>& GetNodesRequiringCacheUpdate() const {
    return nodes_requiring_cache_update_;
  }
#endif

 private:
  struct AXDirtyObject : public GarbageCollected<AXDirtyObject> {
    AXDirtyObject(const AXObject* obj_arg,
                  ax::mojom::blink::EventFrom event_from_arg,
                  ax::mojom::blink::Action event_from_action_arg,
                  std::vector<ui::AXEventIntent> event_intents_arg)
        : obj(obj_arg),
          event_from(event_from_arg),
          event_from_action(event_from_action_arg),
          event_intents(event_intents_arg) {}

    static AXDirtyObject* Create(const AXObject* obj,
                                 ax::mojom::blink::EventFrom event_from,
                                 ax::mojom::blink::Action event_from_action,
                                 std::vector<ui::AXEventIntent> event_intents) {
      return MakeGarbageCollected<AXDirtyObject>(
          obj, event_from, event_from_action, event_intents);
    }

    void Trace(Visitor* visitor) const { visitor->Trace(obj); }

    Member<const AXObject> obj;
    ax::mojom::blink::EventFrom event_from;
    ax::mojom::blink::Action event_from_action;
    std::vector<ui::AXEventIntent> event_intents ALLOW_DISCOURAGED_TYPE(
        "Avoids conversion when passed from/to ui::AXTreeUpdate or "
        "blink::WebAXObject");
  };

  // Updates the AX tree by walking from the root, calling AXObject::
  // UpdateChildrenIfNecessary on each AXObject for which NeedsUpdate is true.
  // This method is part of a11y-during-render, and in particular transitioning
  // to an eager (as opposed to lazy) AX tree update pattern. See
  // https://bugs.chromium.org/p/chromium/issues/detail?id=1342801#c12 for more
  // details.
  void FinalizeTree();

  // Make sure a relation cache exists and is initialized. Must be called with
  // clean layout.
  void EnsureRelationCacheAndInitialTree();

  // Make sure the AXTreeSerializer has been created.
  void EnsureSerializer();

  // Helpers for CreateAndInit().
  AXObject* CreateFromRenderer(LayoutObject*);
  AXObject* CreateFromNode(Node*);
  AXObject* CreateFromInlineTextBox(AbstractInlineTextBox*);
  AXObject* CreateFromBlockFlowIterator(
      AXBlockFlowIterator::FragmentIndex index);

  // Removes AXObject backed by passed-in object, if there is one.
  // It will also notify the parent that its children have changed, so that the
  // parent will recompute its children and be reserialized, unless
  // |notify_parent| is passed in as false.
  void Remove(LayoutObject*, bool notify_parent);
  void Remove(AbstractInlineTextBox*, bool notify_parent);
  void Remove(const LayoutObject* object,
              AXBlockFlowIterator::FragmentIndex index,
              bool notify_parent);

  // Helper to remove the object from the cache.
  // Most callers should be using Remove(AXObject) instead.
  void Remove(AXID, bool notify_parent);
  // Helper to clean up any references to the AXObject's AXID.
  void RemoveReferencesToAXID(AXID);

  HeapMojoRemote<mojom::blink::RenderAccessibilityHost>&
  GetOrCreateRemoteRenderAccessibilityHost();
  WebLocalFrameClient* GetWebLocalFrameClient() const;
  void UpdateLifecycleIfNeeded(Document& document);

  // Is the main document currently parsing content, as opposed to being blocked
  // by script execution or being load complete state.
  bool IsParsingMainDocument() const;

  bool IsMainDocumentDirty() const;
  bool IsPopupDocumentDirty() const;

  // Returns true if the AXID is for a DOM node.
  // All other AXIDs are generated.
  bool IsDOMNodeID(AXID axid) { return axid > 0; }

  // When the AXMode kOnScreenOnly is on, this is the last step performed before
  // FinalizeTree() is called. It will recursively traversse the tree and mark
  // nodes as on-screen or off-screen. This information is later used to
  // determine which nodes will be serialized.
  bool MarkOnScreenNodes(AXObject* obj,
                         const HitTestResult::NodeSet* on_screen_nodes);

  HeapHashSet<WeakMember<InspectorAccessibilityAgent>> agents_;

  struct AXEventParams final : public GarbageCollected<AXEventParams> {
    AXEventParams(AXObject* target,
                  ax::mojom::blink::Event event_type,
                  ax::mojom::blink::EventFrom event_from,
                  ax::mojom::blink::Action event_from_action,
                  const BlinkAXEventIntentsSet& intents)
        : target(target),
          event_type(event_type),
          event_from(event_from),
          event_from_action(event_from_action) {
      for (const auto& intent : intents) {
        event_intents.insert(intent.key, intent.value);
      }
    }
    Member<AXObject> target;
    ax::mojom::blink::Event event_type;
    ax::mojom::blink::EventFrom event_from;
    ax::mojom::blink::Action event_from_action;
    BlinkAXEventIntentsSet event_intents;

    void Trace(Visitor* visitor) const { visitor->Trace(target); }
  };

  typedef HeapVector<Member<TreeUpdateParams>> TreeUpdateCallbackQueue;

  bool IsImmediateProcessingRequired(TreeUpdateParams* tree_update) const;
  bool IsImmediateProcessingRequiredForEvent(
      ax::mojom::blink::EventFrom& event_from,
      AXObject* target,
      ax::mojom::blink::Event& event_type) const;

  ax::mojom::blink::EventFrom ComputeEventFrom();

  void MarkAXObjectDirtyWithCleanLayoutHelper(
      AXObject* obj,
      ax::mojom::blink::EventFrom event_from,
      ax::mojom::blink::Action event_from_action);
  void MarkAXSubtreeDirty(AXObject*);
  void MarkDocumentDirtyWithCleanLayout();

  // Given an object to mark dirty or fire an event on, return an object
  // included in the tree that can be used with the serializer, or null if there
  // is no relevant object to use. Objects that are not included in the tree,
  // and have no ancestor object included in the tree, are pruned from the tree,
  // in which case there is nothing to be serialized.
  AXObject* GetSerializationTarget(AXObject* obj);

  // Helper that clears children up to the first included ancestor and returns
  // the ancestor if a children changed notification should be fired on it.
  AXObject* InvalidateChildren(AXObject* obj);

  // Implicit selection aka "selection follows focus" is not allowed on
  // containers with subwidgets that have had checked or selected, or expanded
  // in the case of tabs.
  void MaybeDisallowImplicitSelectionWithCleanLayout(AXObject* subwidget);

  // Helper method for `ComputeNodesOnLine()`. Given a `line_object` which is
  // the last LayoutObject of a line and that is a child of `block_flow`,
  // connects the previous LayoutObject to a LayoutObject that represents a
  // trailing white space on this line, if one exists.
  void ConnectToTrailingWhitespaceOnLine(const LayoutObject& line_object,
                                         const LayoutBlockFlow& block_flow);

  const Member<Document> document_;

  // Any popup document except for the popup for <select size=1>.
  Member<Document> popup_document_;

  const bool internal_ui_checker_on_;

  ui::AXMode ax_mode_;

  // AXIDs for AXNodeObjects reuse the int ids in dom_node_id, all other AXIDs
  // are negative in order to avoid a conflict.
  HeapHashMap<AXID, Member<AXObject>> objects_;
  // When the AXObject is backed by layout, its AXID can be looked up in
  // layout_object_mapping_. When the AXObject is backed by a node, its
  // AXID can be looked up via node->GetDomNodeId().
  HeapHashMap<Member<const LayoutObject>, AXID> layout_object_mapping_;
  HeapHashMap<Member<AbstractInlineTextBox>, AXID>
      inline_text_box_object_mapping_;

  // A LayoutObject may be connected to one or more AXInlineTextBoxes.
  struct AXInlineTextBoxFragmentMapping {
    // The index of the first AXInlineTextBox associated with a LayoutObject.
    AXBlockFlowIterator::FragmentIndex starting_index;
    // A compact representation of the AXIds of the AXInlineTextBoxes of a
    // LayoutObject. Because fragment indexes are sequential,
    // normally one per object, this Vector stores them as follows:
    // ids[fragment_index - starting_index] = <the AXId>.
    //
    // Example: If starting_index is 10, and fragment indexes 10, 11, and 12
    // have AXIds -100, -101 and -102 respectively, then:
    //   ids[0] (10 - 10) = -100
    //   ids[1] (11 - 10) = -101
    //   ids[2] (12 - 10) = -102
    //
    // Significant gaps in fragment indexes would reduce the efficiency of this
    // approach, however gaps are expected to be rare since the fragments are
    // associated with the same text layout object.
    Vector<AXID> ids;
    // Number of AXInlineTextBoxes that have AXIds set. Note that this can be
    // different from `ids.size()` as the vector may contain gaps if the
    // fragment indices are not consecutive.  Gaps may be introduced during
    // the course of layout updates, particularly as AXInlineTextBoxes are
    // removed. When size is reduced to zero, the entry can be removed from the
    // map for inline text boxes.
    wtf_size_t size;
  };
  HeapHashMap<Member<const LayoutObject>, AXInlineTextBoxFragmentMapping>
      layout_object_to_inline_text_boxes_;

  // When the AXMode filter flag kOnScreenOnly is set, this set holds the IDs of
  // nodes that are not on-screen, but are still serialized.
  WTF::HashSet<AXID> extra_off_screen_nodes_to_serialize_;
#if AX_FAIL_FAST_BUILD()
  size_t included_node_count_ = 0;
  size_t plugin_included_node_count_ = 0;
#endif

  // Used for a mock AXObject representing the message displayed in the
  // validation message bubble.
  // There can be only one of these per document with invalid form controls,
  // and it will always be related to the currently focused control.
  AXID validation_message_axid_;

  // The currently active aria-modal dialog element, if one has been computed,
  // null if otherwise. This is only ever computed on platforms that have the
  // AriaModalPrunesAXTree setting enabled, such as Mac.
  WeakMember<Element> active_aria_modal_dialog_;

  // If non-null, this is the node that the current aria-activedescendant caused
  // to have the selected state.
  WeakMember<Node> last_selected_from_active_descendant_;
  // If non-zero, this is the DOMNodeID for the last <option> element selected
  // in a select with size > 1.
  DOMNodeId last_selected_list_option_ = 0;

  std::unique_ptr<AXRelationCache> relation_cache_;

  // Stages of cache/tree.
  AXObjectCacheLifecycle lifecycle_;
  // If > 0, tree is frozen.
  int frozen_count_ = 0;  // Used with Freeze(), Thaw() and IsFrozen() above.

#if AX_FAIL_FAST_BUILD()
  bool updating_layout_and_ax_ = false;

  // The number of tree checks performed during warm-up. A tree check is
  // performed on each of the first five commits. After this period, a check is
  // performed at most once every five seconds.
  int tree_check_warmup_counter_ = 0;
  base::TimeTicks last_tree_check_time_stamp_;

  // AXIDs of nodes that need their cached attribute values updated mapped to
  // the reason for the update. This is used to validate whether there are any
  // such nodes after the tree is finalized. Otherwise, there may be missed
  // cache updates.
  HashMap<AXID, TreeUpdateReason> nodes_requiring_cache_update_;
#endif

  // If non-zero, do not do work to process a11y or build the a11y tree in
  // CommitAXUpdates(). Will be set to 0 when more content
  // is loaded or the load is completed.
  size_t allowed_tree_update_pauses_remaining_ = 0;
  // If null, then any new connected node will unpause tree updates.
  // Otherwise, tree updates will unpause once the node is fully parsed.
  WeakMember<Node> node_to_parse_before_more_tree_updates_;

  // Call the queued callback methods that do processing which must occur when
  // layout is clean. These callbacks are stored in tree_update_callback_queue_,
  // and have names like FooBarredWithCleanLayout().
  void ProcessCleanLayoutCallbacks(Document&);

  // Get the currently focused Node (an element or a document).
  Node* FocusedNode() const;

  AXObject* FocusedImageMapUIElement(HTMLAreaElement*);

  // Associate an AXObject with an AXID. Generate one if none is supplied.
  AXID AssociateAXID(AXObject*, AXID use_axid = 0);

  void TextChanged(Node*);
  bool NodeIsTextControl(const Node*);
  AXObject* NearestExistingAncestor(Node*);

  Settings* GetSettings();

  // When a <tr> or <td> is inserted or removed, the containing table may have
  // gained or lost rows or columns.
  void ContainingTableRowsOrColsMaybeChanged(Node*);

  // Object for HTML validation alerts. Created at most once per object cache.
  AXObject* GetOrCreateValidationMessageObject();
  void RemoveValidationMessageObjectWithCleanLayout(Node* document);

  // To be called inside DeferTreeUpdate to check the queue status before
  // adding.
  bool CanDeferTreeUpdate(Document* tree_update_document);

  // Checks the update queue, then pauses and rebuilds it if full. Returns true
  // of the queue was paused.
  bool PauseTreeUpdatesIfQueueFull();

  // Enqueue a callback to the given method to be run after layout is
  // complete.
  void DeferTreeUpdate(
      TreeUpdateReason update_reason,
      Node* node,
      ax::mojom::blink::Event event = ax::mojom::blink::Event::kNone);

  // Provide either a DOM node or AXObject. If both are provided, then they must
  // match, meaning that the AXObject's DOM node must equal the provided node.
  void DeferTreeUpdate(
      TreeUpdateReason update_reason,
      AXObject* obj,
      ax::mojom::blink::Event event = ax::mojom::blink::Event::kNone,
      bool invalidate_cached_values = true);

  void TextChangedWithCleanLayout(Node* node);
  void ChildrenChangedWithCleanLayout(Node* node);

  // If the presence of document markers changed for the given text node, then
  // call children changed.
  void HandleTextMarkerDataAddedWithCleanLayout(Node*);
  void HandleUseMapAttributeChangedWithCleanLayout(Node*);
  void HandleNameAttributeChanged(Node*);

  bool DoesEventListenerImpactIgnoredState(const AtomicString& event_type,
                                           const Node& node) const;
  void HandleEventSubscriptionChanged(Node& node,
                                      const AtomicString& event_type);

  //
  // aria-modal support
  //

  // This function is only ever called on platforms where the
  // AriaModalPrunesAXTree setting is enabled, and the accessibility tree must
  // be manually pruned to remove background content.
  void UpdateActiveAriaModalDialog(Node* element);

  // This will return null on platforms without the AriaModalPrunesAXTree
  // setting enabled, or where there is no active ancestral aria-modal dialog.
  Element* AncestorAriaModalDialog(Node* node);

  void FireTreeUpdatedEventForAXID(TreeUpdateParams* update,
                                   Document& document);
  void FireTreeUpdatedEventForNode(TreeUpdateParams* update);

  void SetMaxPendingUpdatesForTesting(wtf_size_t max_pending_updates) {
    max_pending_updates_ = max_pending_updates;
  }

  void UpdateNumTreeUpdatesQueuedBeforeLayoutHistogram();

  // Invalidates the bounding boxes of fixed or sticky positioned objects which
  // should be updated when the scroll offset is changed. Like
  // InvalidateBoundingBox, it can be later retrieved by
  // SerializeLocationChanges.
  void InvalidateBoundingBoxForFixedOrStickyPosition();

  // Return true if this is the popup document. There can only be one popup
  // document at a time. If it is not the popup document, it's the main
  // document stored in |document_|.
  bool IsPopup(Document& document) const;

  // Get the queued tree update callbacks for the passed-in document
  TreeUpdateCallbackQueue& GetTreeUpdateCallbackQueue(Document& document);

  // Helper method to notify a parent node that its children have changed.
  // The notify method depends on the phase we are in. Please see
  // `processing_deferred_events_` for more details.
  void NotifyParentChildrenChanged(AXObject* parent);

  void MaybeSendCanvasHasNonTrivialFallbackUKM(const AXObject* canvas);

  void IncrementGenerationalCacheId() { ++generational_cache_id_; }

  // Queued callbacks.
  TreeUpdateCallbackQueue tree_update_callback_queue_main_;
  TreeUpdateCallbackQueue tree_update_callback_queue_popup_;

  // Help de-dupe processing of repetitive events.
  HashSet<AXID> nodes_with_pending_children_changed_;

  // Nodes with document markers that have received accessibility updates.
  HashSet<AXID> nodes_with_spelling_or_grammar_markers_;

  // Container nodes with an bad aria-hidden="true" usage, where the aria-hidden
  // will be ignored so that the user can navigate the page.
  // Example, aria-hidden="true" on an element, where focus has gone inside
  // of the element.
  HashSet<AXID> nodes_with_bad_aria_hidden_;

  AXID accessibility_focus_ = ui::AXNodeData::kInvalidAXID;
  AXID last_value_change_node_ = ui::AXNodeData::kInvalidAXID;

  // If tree_update_callback_queue_ gets improbably large, stop
  // enqueueing updates and fire a single ChildrenChanged event on the
  // document once layout occurs.
  wtf_size_t max_pending_updates_ = 1UL << 16;
  bool tree_updates_paused_ = false;

  // This will flip to true when we initiate the process of sending AX data to
  // the browser, and will flip back to false once we receive back an ACK.
  bool serialization_in_flight_ = false;

  // This stores the last time a serialization was ACK'ed after being sent to
  // the browser, so that serializations can be skipped if the time since the
  // last serialization is less than GetDeferredEventsDelay(). Setting to zero
  // causes the upcoming serialization to occur at the next available
  // opportunity.  Batching is used to reduce the number of serializations, in
  // order to provide overall faster content updates while using less CPU,
  // because nodes that change multiple times in a short time period only need
  // to be serialized once, e.g. during page loads or animations.
  base::TimeTicks last_serialization_timestamp_;

  // The last time dirty_objects_from_location_change_ were serialized and sent.
  base::TimeTicks last_location_serialization_time_;

  // If true, will not attempt to batch and will serialize at the next
  // opportunity.
  bool serialize_immediately_ = false;

  // This flips to true if a request for an immediate update was not honored
  // because serialization_in_flight_ was true. It flips back to false once
  // serialization_in_flight_ has flipped to false and an immediate update has
  // been requested.
  bool serialize_immediately_after_current_serialization_ = false;

  // Maps ids to their object's autofill suggestion availability.
  HashMap<AXID, WebAXAutofillSuggestionAvailability>
      autofill_suggestion_availability_map_;

  // The set of node IDs whose bounds has changed since the last time
  // SerializeLocationChanges was called.
  HashSet<AXID> changed_bounds_ids_;

  // If the currently focused element is an expanded <select>, this tracks the
  // bounding boxes for the options, which are rendered in a special popup
  // document that is not in the AX tree that duplicates the option elements
  // from the main document.
  WTF::Vector<gfx::Rect> options_bounds_;
  // AXID for the <select> containing tracked options bounds.
  AXID current_menu_list_axid_ = 0;

  // Known locations and sizes of bounding boxes that are known to have been
  // serialized as well as their scroll offsets.
  struct CachedLocationChange {
    ui::AXRelativeBounds bounds;
    int scroll_x;
    int scroll_y;
  };
  HashMap<AXID, CachedLocationChange> cached_bounding_boxes_;

  // The list of node IDs whose position is fixed or sticky.
  HashSet<AXID> fixed_or_sticky_node_ids_;

  // Map of node IDs where there was an operation done, could be deletion or
  // insertion. The items in the vector are in the order that the operations
  // were made in.
  HashMap<AXID, WTF::Vector<TextChangedOperation>> text_operation_in_node_ids_;

  // A set of ARIA notifications that have yet to be added to `ax_tree_data`.
  HashMap<AXID, AriaNotifications> aria_notifications_;

  // The source of the event that is currently being handled.
  ax::mojom::blink::EventFrom active_event_from_ =
      ax::mojom::blink::EventFrom::kNone;

  // The accessibility action that caused the event. Will only be valid if
  // active_event_from_ is set to kAction.
  ax::mojom::blink::Action active_event_from_action_ =
      ax::mojom::blink::Action::kNone;

  // A set of currently active event intents.
  BlinkAXEventIntentsSet active_event_intents_;

  HeapMojoRemote<mojom::blink::RenderAccessibilityHost>
      render_accessibility_host_;

  Member<BlinkAXTreeSource> ax_tree_source_;
  std::unique_ptr<ui::AXTreeSerializer<const AXObject*,
                                       HeapVector<Member<const AXObject>>,
                                       ui::AXTreeUpdate*,
                                       ui::AXTreeData*,
                                       ui::AXNodeData>>
      ax_tree_serializer_;

  HeapVector<Member<AXDirtyObject>> pending_objects_to_serialize_;

  Vector<ui::AXEvent> pending_events_to_serialize_;

  // Any tree, tab or listbox that disallows implicit "selection from focus".
  HashSet<AXID> containers_disallowing_implicit_selection_;

  // Make sure the next serialization sends everything.
  bool mark_all_dirty_ = false;

  mutable bool has_axid_generator_looped_ = false;

  // These maps get cleared when the tree is thawed. Contains the data used to
  // compute Next|PreviousOnLineId attributes.
  HeapHashMap<Member<const LayoutObject>, Member<const LayoutObject>>
      next_on_line_map_;
  HeapHashMap<Member<const LayoutObject>, Member<const LayoutObject>>
      previous_on_line_map_;
  HeapHashSet<Member<const LayoutBlockFlow>> processed_blocks_;

  HeapHashMap<Member<const LayoutBlockFlow>, Member<AXBlockFlowData>>
      block_flow_data_cache_;

  FRIEND_TEST_ALL_PREFIXES(AccessibilityTest, PauseUpdatesAfterMaxNumberQueued);
  FRIEND_TEST_ALL_PREFIXES(AccessibilityTest,
                           UpdateAXForAllDocumentsAfterPausedUpdates);
  FRIEND_TEST_ALL_PREFIXES(AccessibilityTest, RemoveReferencesToAXID);
  FRIEND_TEST_ALL_PREFIXES(AccessibilityTest, NodesRequiringCacheUpdate);

  // The ID of the object to fetch image data for.
  AXID image_data_node_id_ = ui::AXNodeData::kInvalidAXID;

  gfx::Size max_image_data_size_;

  using PluginAXTreeSerializer =
      ui::AXTreeSerializer<const ui::AXNode*,
                           std::vector<const ui::AXNode*>,
                           ui::AXTreeUpdate*,
                           ui::AXTreeData*,
                           ui::AXNodeData>;
  // AXTreeSerializer's AXSourceNodeVectorType is not a vector<raw_ptr> due to
  // performance regressions detected in blink_perf.accessibility tests.
  RAW_PTR_EXCLUSION std::unique_ptr<PluginAXTreeSerializer> plugin_serializer_;
  raw_ptr<ui::AXTreeSource<const ui::AXNode*, ui::AXTreeData*, ui::AXNodeData>>
      plugin_tree_source_;

  // So we can ensure the serialization pipeline never stalls with dirty objects
  // remaining to be serialized.
  blink::WeakCellFactory<AXObjectCacheImpl>
      weak_factory_for_serialization_pipeline_{this};

  // So we can ensure the location changes pipeline never stalls with location
  // changes remaining to be serialized.
  blink::WeakCellFactory<AXObjectCacheImpl>
      weak_factory_for_loc_updates_pipeline_{this};

  // Whether or not the load event was sent in a previous serialization.
  bool load_sent_ = false;

  bool has_emitted_canvas_fallback_ukm_ = false;

  // Used to determine if a previously computed attribute is from the same
  // serialization update.
  uint64_t generational_cache_id_ = 0;

  // All the callbacks passed to `ScheduleAXUpdate` that `CompleteAXUpdate` has
  // to call once it's done.
  Vector<base::OnceClosure> ready_callbacks_;

  // Holds the bounds as well as the canvas owner id of objects which had values
  // explicitly set.
  HashMap<AXID, std::pair<PhysicalRect, AXID>> ax_id_to_explicit_bounds_;

  // Map that holds per AXID the ID of another tree that should be attached to
  // the object as a child tree. This should not be used for iframes since the
  // child tree for an iframe can be retrieved from the child frame's embedding
  // token. It should only be used whenever the
  // `ax::mojom::Action::kStitchChildTree` is sent to the renderer requesting
  // that another tree is joined with the existing tree. This might be needed
  // when another tree with some generated content should be stitched into the
  // current tree.
  HashMap<AXID, ui::AXTreeID> ax_id_to_child_tree_id_;
};

// This is the only subclass of AXObjectCache.
template <>
struct DowncastTraits<AXObjectCacheImpl> {
  static bool AllowFrom(const AXObjectCache& cache) { return true; }
};

MODULES_EXPORT std::ostream& operator<<(std::ostream&,
                                        const AXObjectCacheImpl&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_OBJECT_CACHE_IMPL_H_
