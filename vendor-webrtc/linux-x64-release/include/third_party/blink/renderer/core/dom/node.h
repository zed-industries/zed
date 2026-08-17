/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004-2011, 2014 Apple Inc. All rights reserved.
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_H_

#include <climits>

#include "base/dcheck_is_on.h"
#include "base/notreached.h"
#include "third_party/blink/public/mojom/devtools/console_message.mojom-blink.h"
#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/counters_attachment_context.h"
#include "third_party/blink/renderer/core/css/invalidation/invalidation_tracing_flag.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/core/dom/mutation_observer_options.h"
#include "third_party/blink/renderer/core/dom/node_rare_data.h"
#include "third_party/blink/renderer/core/dom/tree_scope.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/custom_spaces.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/wtf.h"

// Exposes |DumpStatistics()| for dumping information about nodes. To use, call
// |DumpStatistics()| from the Node constructor or GDB.
// This needs to be here because Element.h also depends on it.
#define DUMP_NODE_STATISTICS 0

namespace gfx {
class Rect;
}

namespace blink {

class ContainerNode;
class Document;
class Element;
class Event;
class EventDispatchHandlingState;
class ExceptionState;
class FlatTreeNodeData;
class GetRootNodeOptions;
class HTMLQualifiedName;
class HTMLSlotElement;
class KURL;
class LayoutBox;
class LayoutBoxModelObject;
class LayoutObject;
class MathMLQualifiedName;
class MutationObserver;
class MutationObserverRegistration;
class NodeCloningData;
class NodeList;
class NodeListsNodeData;
class NodeRareData;
class Part;
class QualifiedName;
class RegisteredEventListener;
class ScrollTimeline;
class SVGQualifiedName;
class ShadowRoot;
template <typename NodeType>
class StaticNodeTypeList;
using StaticNodeList = StaticNodeTypeList<Node>;
class StyleChangeReasonForTracing;
class TextVisitor;
class V8UnionNodeOrStringOrTrustedScript;
class V8UnionStringOrTrustedScript;
class WebPluginContainerImpl;

struct PhysicalRect;

const int kElementNamespaceTypeShift = 5;
const int kNodeStyleChangeShift = 16;
const int kNodeCustomElementShift = 18;

// Values for kChildNeedsStyleRecalcFlag, controlling whether a node gets its
// style recalculated.
enum StyleChangeType : uint32_t {
  // This node does not need style recalculation.
  kNoStyleChange = 0,
  // This node needs style recalculation, but the changes are of
  // a very limited set:
  //
  //  1. They only touch the node's inline style (style="" attribute).
  //  2. They don't add or remove any properties.
  //  3. They only touch independent properties.
  //
  // If all changes are of this type, we can do incremental style
  // recalculation by reusing the previous style and just applying
  // any modified inline style, which is cheaper than a full recalc.
  // See CanApplyInlineStyleIncrementally() and comments on
  // StyleResolver::ApplyBaseStyle() for more details.
  kInlineIndependentStyleChange = 1 << kNodeStyleChangeShift,
  // This node needs (full) style recalculation.
  kLocalStyleChange = 2 << kNodeStyleChangeShift,
  // This node and all of its flat-tree descendeants need style recalculation.
  kSubtreeStyleChange = 3 << kNodeStyleChangeShift,
};

enum class CustomElementState : uint32_t {
  // https://dom.spec.whatwg.org/#concept-element-custom-element-state
  kUncustomized = 0,
  kCustom = 1 << kNodeCustomElementShift,
  kPreCustomized = 2 << kNodeCustomElementShift,
  kUndefined = 3 << kNodeCustomElementShift,
  kFailed = 4 << kNodeCustomElementShift,
};

enum class SlotChangeType {
  kSignalSlotChangeEvent,
  kSuppressSlotChangeEvent,
};

// LinkHighlight determines the largest enclosing node with hand cursor set.
enum class LinkHighlightCandidate {
  // This node is with hand cursor set.
  kYes,
  // This node is not with hand cursor set.
  kNo,
  // |kYes| if its ancestor is |kYes|.
  kMayBe
};

// A Node is a base class for all objects in the DOM tree.
// The spec governing this interface can be found here:
// https://dom.spec.whatwg.org/#interface-node
class CORE_EXPORT Node : public EventTarget {
  DEFINE_WRAPPERTYPEINFO();
  friend class TreeScope;
  friend class TreeScopeAdopter;

 public:
  enum NodeType {
    kElementNode = 1,
    kAttributeNode = 2,
    kTextNode = 3,
    kCdataSectionNode = 4,
    kProcessingInstructionNode = 7,
    kCommentNode = 8,
    kDocumentNode = 9,
    kDocumentTypeNode = 10,
    kDocumentFragmentNode = 11,
  };

  // Entity, EntityReference, and Notation nodes are impossible to create in
  // Blink.  But for compatibility reasons we want these enum values exist in
  // JS, and this enum makes the bindings generation not complain about
  // kEntityReferenceNode being missing from the implementation while not
  // requiring all switch(NodeType) blocks to include this deprecated constant.
  enum DeprecatedNodeType {
    kEntityReferenceNode = 5,
    kEntityNode = 6,
    kNotationNode = 12,
  };

  enum DocumentPosition {
    kDocumentPositionEquivalent = 0x00,
    kDocumentPositionDisconnected = 0x01,
    kDocumentPositionPreceding = 0x02,
    kDocumentPositionFollowing = 0x04,
    kDocumentPositionContains = 0x08,
    kDocumentPositionContainedBy = 0x10,
    kDocumentPositionImplementationSpecific = 0x20,
  };

#if DUMP_NODE_STATISTICS
  static void DumpStatistics();
#endif

  static Node* FromDomNodeId(DOMNodeId dom_node_id);

  ~Node() override;

  // Returns the existing DOMNodeID for the node if it has already been
  // assigned, otherwise, assigns a new DOMNodeID and return that.
  DOMNodeId GetDomNodeId();

  // DOM methods & attributes for Node

  bool HasTagName(const HTMLQualifiedName&) const;
  bool HasTagName(const MathMLQualifiedName&) const;
  bool HasTagName(const SVGQualifiedName&) const;
  virtual String nodeName() const = 0;
  virtual String nodeValue() const;
  virtual void setNodeValue(const String&,
                            ExceptionState& = ASSERT_NO_EXCEPTION);
  ContainerNode* parentNode() const {
    return reinterpret_cast<ContainerNode*>(
        parent_or_shadow_host_node_.TryGetAs<ParentNodeTag>());
  }

  Element* parentElement() const;
  ContainerNode* ParentElementOrShadowRoot() const;
  ContainerNode* ParentElementOrDocumentFragment() const;
  Node* previousSibling() const { return previous_.Get(); }
  bool HasPreviousSibling() const { return static_cast<bool>(previous_); }
  Node* nextSibling() const { return next_.Get(); }
  bool HasNextSibling() const { return static_cast<bool>(next_); }
  NodeList* childNodes();
  Node* firstChild() const;
  Node* lastChild() const;
  Node* getRootNode(const GetRootNodeOptions*) const;
  Node& TreeRoot() const;
  Node& ShadowIncludingRoot() const;
  // closed-shadow-hidden is defined at
  // https://dom.spec.whatwg.org/#concept-closed-shadow-hidden
  bool IsClosedShadowHiddenFrom(const Node&) const;

  // ParentNode interface. These functions are only actually web-exposed on
  // interfaces that include ParentNode in their idl.
  void prepend(
      const HeapVector<Member<V8UnionNodeOrStringOrTrustedScript>>& nodes,
      ExceptionState& exception_state);
  void append(
      const HeapVector<Member<V8UnionNodeOrStringOrTrustedScript>>& nodes,
      ExceptionState& exception_state);
  void replaceChildren(
      const HeapVector<Member<V8UnionNodeOrStringOrTrustedScript>>& nodes,
      ExceptionState& exception_state);

  // ChildNode interface. These functions are only actually web-exposed on
  // interfaces that include ChildNode in their idl.
  void before(
      const HeapVector<Member<V8UnionNodeOrStringOrTrustedScript>>& nodes,
      ExceptionState& exception_state);
  void after(
      const HeapVector<Member<V8UnionNodeOrStringOrTrustedScript>>& nodes,
      ExceptionState& exception_state);
  void replaceWith(
      const HeapVector<Member<V8UnionNodeOrStringOrTrustedScript>>& nodes,
      ExceptionState& exception_state);
  void remove(ExceptionState&);
  void remove();

  // NonDocumentTypeChildNode interface. These functions are only actually
  // web-exposed on  interfaces that include NonDocumentTypeChildNode in their
  // idl.
  Element* previousElementSibling();
  Element* nextElementSibling();

  Node* PseudoAwareNextSibling() const;
  Node* PseudoAwarePreviousSibling() const;
  Node* PseudoAwareFirstChild() const;
  Node* PseudoAwareLastChild() const;

  const KURL& baseURI() const;

  Node* insertBefore(Node* new_child, Node* ref_child, ExceptionState&);
  Node* insertBefore(Node* new_child, Node* ref_child);
  void moveBefore(Node* new_child, Node* ref_child, ExceptionState&);
  Node* replaceChild(Node* new_child, Node* old_child, ExceptionState&);
  Node* replaceChild(Node* new_child, Node* old_child);
  Node* removeChild(Node* child, ExceptionState&);
  Node* removeChild(Node* child);
  Node* appendChild(Node* new_child, ExceptionState&);
  Node* appendChild(Node* new_child);

  bool hasChildren() const { return firstChild(); }
  Node* cloneNode(bool deep, ExceptionState&) const;

  // https://dom.spec.whatwg.org/#concept-node-clone
  // The implementation differs a bit from the spec algorithm, notably in the
  // order that nodes are appended to their eventual destination. The spec
  // requires each Element's children to be cloned before they are appended to
  // the Element, whereas the Chromium implementation first attaches a new
  // clone to its parent, and then clones children. This avoids an O(log-n^2)
  // set of calls to Node::InsertedInto().
  virtual Node* Clone(
      Document& factory,
      NodeCloningData& data,
      ContainerNode* append_to,
      ExceptionState& append_exception_state = ASSERT_NO_EXCEPTION) const = 0;

  // This is not web-exposed. We should rename it or remove it.
  Node* cloneNode(bool deep) const;
  void normalize();

  bool isEqualNode(Node*) const;
  bool isSameNode(const Node* other) const { return this == other; }
  bool isDefaultNamespace(const AtomicString& namespace_uri) const;
  const AtomicString& lookupPrefix(const AtomicString& namespace_uri) const;
  const AtomicString& lookupNamespaceURI(const String& prefix) const;

  String textContent(bool convert_brs_to_newlines = false,
                     TextVisitor* visitor = nullptr,
                     unsigned int max_length = UINT_MAX) const;
  virtual void setTextContent(const String&);
  V8UnionStringOrTrustedScript* textContentForBinding() const;
  virtual void setTextContentForBinding(
      const V8UnionStringOrTrustedScript* value,
      ExceptionState& exception_state);

  bool SupportsAltText();

  // Other methods (not part of DOM)
  ALWAYS_INLINE NodeType getNodeType() const {
    return static_cast<NodeType>(node_flags_ & kNodeTypeMask);
  }
  ALWAYS_INLINE bool IsTextNode() const {
    return getNodeType() == kTextNode || getNodeType() == kCdataSectionNode;
  }
  ALWAYS_INLINE bool IsContainerNode() const {
    return GetFlag(kIsContainerFlag);
  }
  ALWAYS_INLINE bool IsElementNode() const {
    return getNodeType() == kElementNode;
  }
  ALWAYS_INLINE bool IsDocumentFragment() const {
    return getNodeType() == kDocumentFragmentNode;
  }
  ALWAYS_INLINE bool IsHTMLElement() const {
    return GetElementNamespaceType() == ElementNamespaceType::kHTML;
  }
  ALWAYS_INLINE bool IsMathMLElement() const {
    return GetElementNamespaceType() == ElementNamespaceType::kMathML;
  }
  ALWAYS_INLINE bool IsSVGElement() const {
    return GetElementNamespaceType() == ElementNamespaceType::kSVG;
  }

  DISABLE_CFI_PERF bool IsCheckPseudoElement() const {
    return GetPseudoId() == kPseudoIdCheckMark;
  }
  DISABLE_CFI_PERF bool IsBeforePseudoElement() const {
    return GetPseudoId() == kPseudoIdBefore;
  }
  DISABLE_CFI_PERF bool IsAfterPseudoElement() const {
    return GetPseudoId() == kPseudoIdAfter;
  }
  DISABLE_CFI_PERF bool IsPickerIconPseudoElement() const {
    return GetPseudoId() == kPseudoIdPickerIcon;
  }
  DISABLE_CFI_PERF bool IsScrollMarkerGroupBeforePseudoElement() const {
    return GetPseudoId() == kPseudoIdScrollMarkerGroupBefore;
  }
  DISABLE_CFI_PERF bool IsScrollMarkerGroupAfterPseudoElement() const {
    return GetPseudoId() == kPseudoIdScrollMarkerGroupAfter;
  }
  DISABLE_CFI_PERF bool IsScrollButtonBlockStartPseudoElement() const {
    return GetPseudoId() == kPseudoIdScrollButtonBlockStart;
  }
  DISABLE_CFI_PERF bool IsScrollButtonInlineStartPseudoElement() const {
    return GetPseudoId() == kPseudoIdScrollButtonInlineStart;
  }
  DISABLE_CFI_PERF bool IsScrollButtonInlineEndPseudoElement() const {
    return GetPseudoId() == kPseudoIdScrollButtonInlineEnd;
  }
  DISABLE_CFI_PERF bool IsScrollButtonBlockEndPseudoElement() const {
    return GetPseudoId() == kPseudoIdScrollButtonBlockEnd;
  }
  DISABLE_CFI_PERF bool IsMarkerPseudoElement() const {
    return GetPseudoId() == kPseudoIdMarker;
  }
  DISABLE_CFI_PERF bool IsFirstLetterPseudoElement() const {
    return GetPseudoId() == kPseudoIdFirstLetter;
  }
  DISABLE_CFI_PERF bool IsBackdropPseudoElement() const {
    return GetPseudoId() == kPseudoIdBackdrop;
  }
  DISABLE_CFI_PERF bool IsViewTransitionPseudoElement() const {
    return IsTransitionPseudoElement(GetPseudoId());
  }
  DISABLE_CFI_PERF bool IsViewTransitionGroupPseudoElement() const {
    return GetPseudoId() == kPseudoIdViewTransitionGroup;
  }
  virtual PseudoId GetPseudoId() const { return kPseudoIdNone; }
  virtual PseudoId GetPseudoIdForStyling() const { return kPseudoIdNone; }
  virtual const AtomicString& GetPseudoArgument() const { return g_null_atom; }

  CustomElementState GetCustomElementState() const {
    return static_cast<CustomElementState>(node_flags_ &
                                           kCustomElementStateMask);
  }
  bool IsCustomElement() const {
    return GetCustomElementState() != CustomElementState::kUncustomized;
  }
  void SetCustomElementState(CustomElementState);

  virtual bool IsPseudoElement() const { return false; }
  virtual bool IsColumnPseudoElement() const { return false; }
  virtual bool IsScrollMarkerPseudoElement() const { return false; }
  virtual bool IsScrollMarkerGroupPseudoElement() const { return false; }
  virtual bool IsScrollButtonPseudoElement() const { return false; }
  virtual bool IsMediaControlElement() const { return false; }
  virtual bool IsMediaControls() const { return false; }
  virtual bool IsMediaElement() const { return false; }
  virtual bool IsTextTrackContainer() const { return false; }
  virtual bool IsVTTElement() const { return false; }
  virtual bool IsAttributeNode() const { return false; }
  virtual bool IsCharacterDataNode() const { return false; }
  virtual bool IsFrameOwnerElement() const { return false; }
  virtual bool IsMediaRemotingInterstitial() const { return false; }
  virtual bool IsPictureInPictureInterstitial() const { return false; }

  // Either ::scroll-marker or ::scroll-*-button pseudo element.
  bool IsScrollControlPseudoElement() const {
    return IsScrollMarkerPseudoElement() || IsScrollButtonPseudoElement();
  }

  // Traverses the ancestors of this node and returns true if any of them are
  // either a MediaControlElement or MediaControls.
  bool HasMediaControlAncestor() const;

  // StyledElements allow inline style (style="border: 1px"), presentational
  // attributes (ex. color), class names (ex. class="foo bar") and other
  // non-basic styling features. They also control if this element can
  // participate in style sharing.
  //
  // TODO(crbug.com/1305488): The only things that ever go through StyleResolver
  // that aren't StyledElements are PseudoElements and VTTElements. It's
  // possible we can just remove this function entirely, and replace it at the
  // callsites with a DCHECK, since those elements will never have class
  // names, inline style, or other things that this apparently guards
  // against.
  bool IsStyledElement() const {
    return IsHTMLElement() || IsSVGElement() || IsMathMLElement();
  }

  bool IsDocumentNode() const;
  bool IsTreeScope() const;
  bool IsShadowRoot() const {
    const bool result = parent_or_shadow_host_node_.Is<ShadowHostTag>();
#if DCHECK_IS_ON()
    DCHECK(!result || (IsDocumentFragment() && IsTreeScope()));
#endif
    return result;
  }

  bool IsActiveSlot() const;
  bool IsSlotable() const { return IsTextNode() || IsElementNode(); }
  AtomicString SlotName() const;

  bool HasCustomStyleCallbacks() const {
    return GetFlag(kHasCustomStyleCallbacksFlag);
  }

  // If this node is in a shadow tree, returns its shadow host. Otherwise,
  // returns nullptr.
  Element* OwnerShadowHost() const;
  // crbug.com/569532: containingShadowRoot() can return nullptr even if
  // isInShadowTree() returns true.
  // This can happen when handling queued events (e.g. during execCommand())
  ShadowRoot* ContainingShadowRoot() const;

  // Inline-defined in shadow_root.h
  inline ShadowRoot* GetShadowRoot() const;
  inline bool IsInUserAgentShadowRoot() const;

  // Returns nullptr, a child of ShadowRoot, or a legacy shadow root.
  Node* NonBoundaryShadowTreeRootNode();

  // Node's parent, shadow tree host.
  ContainerNode* ParentOrShadowHostNode() const;
  Element* ParentOrShadowHostElement() const;
  void SetParentNode(ContainerNode*);
  void SetShadowHostNode(ContainerNode*);

  // Knows about all kinds of hosts.
  ContainerNode* ParentOrShadowHostOrTemplateHostNode() const;

  // Returns the parent node, but nullptr if the parent node is a ShadowRoot.
  ContainerNode* NonShadowBoundaryParentNode() const;

  // Returns the enclosing event parent Element (or self) that, when clicked,
  // would trigger a navigation.
  Element* EnclosingLinkEventParentOrSelf() const;

  // These low-level calls give the caller responsibility for maintaining the
  // integrity of the tree.
  void SetPreviousSibling(Node* previous) { previous_ = previous; }
  void SetNextSibling(Node* next) { next_ = next; }

  virtual bool CanContainRangeEndPoint() const { return false; }

  // For <link> and <style> elements.
  virtual bool SheetLoaded() { return true; }
  enum LoadedSheetErrorStatus {
    kNoErrorLoadingSubresource,
    kErrorOccurredLoadingSubresource
  };
  virtual void NotifyLoadedSheetAndAllCriticalSubresources(
      LoadedSheetErrorStatus) {}
  virtual void SetToPendingState() { NOTREACHED(); }

  bool HasName() const {
    DCHECK(!IsTextNode());
    return GetFlag(kHasNameOrIsEditingTextFlag);
  }

  bool IsUserActionElement() const { return GetFlag(kIsUserActionElementFlag); }
  void SetUserActionElement(bool flag) {
    SetFlag(flag, kIsUserActionElementFlag);
  }

  bool IsActive() const {
    return IsUserActionElement() && IsUserActionElementActive();
  }
  bool InActiveChain() const {
    return IsUserActionElement() && IsUserActionElementInActiveChain();
  }
  bool IsDragged() const {
    return IsUserActionElement() && IsUserActionElementDragged();
  }
  bool IsHovered() const {
    return IsUserActionElement() && IsUserActionElementHovered();
  }
  // Note: As a shadow host whose root with delegatesFocus=false may become
  // focused state when an inner element gets focused, in that case more than
  // one elements in a document can return true for |isFocused()|.  Use
  // Element::isFocusedElementInDocument() or Document::focusedElement() to
  // check which element is exactly focused.
  bool IsFocused() const {
    return IsUserActionElement() && IsUserActionElementFocused();
  }
  bool HasFocusWithin() const {
    return IsUserActionElement() && IsUserActionElementHasFocusWithin();
  }

  // True if the style recalc process should recalculate style for this node.
  bool NeedsStyleRecalc() const {
    return GetStyleChangeType() != kNoStyleChange;
  }
  StyleChangeType GetStyleChangeType() const {
    return static_cast<StyleChangeType>(node_flags_ & kStyleChangeMask);
  }
  // True if the style recalculation process should traverse this node's
  // children when looking for nodes that need recalculation.
  bool ChildNeedsStyleRecalc() const {
    return GetFlag(kChildNeedsStyleRecalcFlag);
  }
  bool IsLink() const { return GetFlag(kIsLinkFlag); }
  bool IsEditingText() const {
    DCHECK(IsTextNode());
    return GetFlag(kHasNameOrIsEditingTextFlag);
  }

  void SetHasName(bool f) {
    DCHECK(!IsTextNode());
    SetFlag(f, kHasNameOrIsEditingTextFlag);
  }
  void SetChildNeedsStyleRecalc() { SetFlag(kChildNeedsStyleRecalcFlag); }
  void ClearChildNeedsStyleRecalc() { ClearFlag(kChildNeedsStyleRecalcFlag); }

  // Sets the flag for the current node and also calls
  // MarkAncestorsWithChildNeedsStyleRecalc
  void SetNeedsStyleRecalc(StyleChangeType, const StyleChangeReasonForTracing&);
  void ClearNeedsStyleRecalc();

  // Propagates a dirty bit breadcrumb for this element up the ancestor chain.
  void MarkAncestorsWithChildNeedsStyleRecalc();

  // Traverses subtree (include pseudo elements and shadow trees) and
  // invalidates nodes whose styles depend on font metrics (e.g., 'ex' unit).
  void MarkSubtreeNeedsStyleRecalcForFontUpdates();

  // Nodes which are not connected are style clean. Mark them for style recalc
  // when inserting them into a document. This method was added as a light-
  // weight alternative to SetNeedsStyleRecalc because using that method caused
  // a micro-benchmark regression (https://crbug.com/926343).
  void SetStyleChangeOnInsertion() {
    DCHECK(isConnected());
    if (ShouldSkipMarkingStyleDirty()) {
      return;
    }
    if (InvalidationTracingFlag::IsEnabled()) [[unlikely]] {
      MaybeAddNodeInsertedTraceEvent();
    }
    if (!NeedsStyleRecalc()) {
      SetStyleChange(kLocalStyleChange);
    }
    MarkAncestorsWithChildNeedsStyleRecalc();
  }

  // Mark non-slotted shadow host child dirty for style recalc to enforce new
  // ComputedStyles for elements outside the flat tree for getComputedStyle().
  // To be called when descendant recalcs are skipped for such subtrees.
  void SetStyleChangeForNonSlotted() {
    DCHECK(IsElementNode());
    DCHECK(isConnected());
    DCHECK(parentElement() && !GetStyleRecalcParent());
    if (!NeedsStyleRecalc()) {
      SetStyleChange(kLocalStyleChange);
    }
  }

  bool NeedsReattachLayoutTree() const {
    return GetFlag(kNeedsReattachLayoutTree);
  }
  bool ChildNeedsReattachLayoutTree() const {
    return GetFlag(kChildNeedsReattachLayoutTree);
  }

  void SetNeedsReattachLayoutTree();
  void SetChildNeedsReattachLayoutTree() {
    SetFlag(kChildNeedsReattachLayoutTree);
  }

  void ClearNeedsReattachLayoutTree() { ClearFlag(kNeedsReattachLayoutTree); }
  void ClearChildNeedsReattachLayoutTree() {
    ClearFlag(kChildNeedsReattachLayoutTree);
  }

  void MarkAncestorsWithChildNeedsReattachLayoutTree();

  // Mark node for forced layout tree re-attach during next lifecycle update.
  // This is to trigger layout tree re-attachment when we cannot detect that we
  // need to re-attach based on the computed style changes. This can happen when
  // re-slotting shadow host children, for instance.
  void SetForceReattachLayoutTree();
  bool GetForceReattachLayoutTree() const {
    return GetFlag(kForceReattachLayoutTree);
  }

  bool NeedsLayoutSubtreeUpdate() const;
  bool NeedsWhitespaceChildrenUpdate() const;
  bool IsDirtyForStyleRecalc() const {
    return NeedsStyleRecalc() || GetForceReattachLayoutTree() ||
           NeedsLayoutSubtreeUpdate();
  }
  bool IsDirtyForRebuildLayoutTree() const {
    return NeedsReattachLayoutTree() || NeedsLayoutSubtreeUpdate();
  }

  // True if the style invalidation process should traverse this node's children
  // when looking for pending invalidations.
  bool ChildNeedsStyleInvalidation() const {
    return GetFlag(kChildNeedsStyleInvalidationFlag);
  }
  void SetChildNeedsStyleInvalidation() {
    SetFlag(kChildNeedsStyleInvalidationFlag);
  }
  void ClearChildNeedsStyleInvalidation() {
    ClearFlag(kChildNeedsStyleInvalidationFlag);
  }
  void MarkAncestorsWithChildNeedsStyleInvalidation();

  // True if there are pending invalidations against this node.
  bool NeedsStyleInvalidation() const {
    return GetFlag(kNeedsStyleInvalidationFlag);
  }
  void ClearNeedsStyleInvalidation() { ClearFlag(kNeedsStyleInvalidationFlag); }
  // Sets the flag for the current node and also calls
  // MarkAncestorsWithChildNeedsStyleInvalidation
  void SetNeedsStyleInvalidation();

  void SetIsLink(bool f);

  void SetHasFocusWithin(bool flag);
  virtual void SetDragged(bool flag);

  // This is called only when the node is focused.
  virtual bool ShouldHaveFocusAppearance() const;

  void FocusabilityLost();

  // Returns how |this| participates to the nodes with hand cursor set.
  LinkHighlightCandidate IsLinkHighlightCandidate() const;

  virtual PhysicalRect BoundingBox() const;
  gfx::Rect PixelSnappedBoundingBox() const;

  // BoundingBoxForScrollIntoView() is the node's scroll snap area.
  // It is expanded from the BoundingBox() by scroll-margin.
  // https://drafts.csswg.org/css-scroll-snap-1/#scroll-snap-area
  PhysicalRect BoundingBoxForScrollIntoView() const;

  unsigned NodeIndex() const;

  // Returns the DOM ownerDocument attribute. This method never returns null,
  // except in the case of a Document node.
  Document* ownerDocument() const;

  // Returns the document associated with this node. A Document node returns
  // itself.
  Document& GetDocument() const { return GetTreeScope().GetDocument(); }

  TreeScope& GetTreeScope() const {
    DCHECK(tree_scope_);
    return *tree_scope_;
  }

  // Returns the tree scope where this element originated.
  // Use this when resolving element references for (CSS url(...)s and #id).
  // This differs from GetTreeScope for shadow clones inside <svg:use/>.
  TreeScope& OriginatingTreeScope() const;

  HeapHashSet<Member<TreeScope>> GetAncestorTreeScopes() const;

  bool InActiveDocument() const;

  // Returns true if this node is connected to a document, false otherwise.
  // See https://dom.spec.whatwg.org/#connected for the definition.
  bool isConnected() const { return GetFlag(kIsConnectedFlag); }

  bool IsInDocumentTree() const { return isConnected() && !IsInShadowTree(); }
  bool IsInShadowTree() const { return GetFlag(kIsInShadowTreeFlag); }
  bool IsInTreeScope() const {
    return GetFlag(
        static_cast<NodeFlags>(kIsConnectedFlag | kIsInShadowTreeFlag));
  }

  ShadowRoot* ParentElementShadowRoot() const;
  bool IsChildOfShadowHost() const;
  ShadowRoot* ShadowRootOfParent() const;
  Element* FlatTreeParentForChildDirty() const;
  Element* GetStyleRecalcParent() const {
    return FlatTreeParentForChildDirty();
  }
  Element* GetReattachParent() const { return FlatTreeParentForChildDirty(); }

  bool IsDocumentTypeNode() const { return getNodeType() == kDocumentTypeNode; }
  virtual bool ChildTypeAllowed(NodeType) const { return false; }
  unsigned CountChildren() const;

  bool IsDescendantOf(const Node*) const;
  bool IsDescendantOrShadowDescendantOf(const Node*) const;
  bool contains(const Node*) const;
  // https://dom.spec.whatwg.org/#concept-shadow-including-inclusive-ancestor
  bool IsShadowIncludingInclusiveAncestorOf(const Node&) const;
  // https://dom.spec.whatwg.org/#concept-shadow-including-ancestor
  bool IsShadowIncludingAncestorOf(const Node&) const;
  bool ContainsIncludingHostElements(const Node&) const;
  Node* CommonAncestor(const Node&,
                       ContainerNode* (*parent)(const Node&)) const;

  // Whether or not a selection can be started in this object
  virtual bool CanStartSelection() const;

  // TODO(bebeaudr): This is a temporary solution only. Accessibility and Blink
  // shouldn't differ when it comes to determining if a node is editable, richly
  // editable or not editable at all.
  // See https://crbug.com/1331359 for more info.
  virtual bool IsRichlyEditableForAccessibility() const;

  void NotifyPriorityScrollAnchorStatusChanged();

  // ---------------------------------------------------------------------------
  // Integration with layout tree

  // As layoutObject() includes a branch you should avoid calling it repeatedly
  // in hot code paths.
  // Note that if a Node has a layoutObject, it's parentNode is guaranteed to
  // have one as well.
  LayoutObject* GetLayoutObject() const { return layout_object_.Get(); }
  void SetLayoutObject(LayoutObject* layout_object) {
    layout_object_ = layout_object;
  }
  // Use these two methods with caution.
  LayoutBox* GetLayoutBox() const;
  LayoutBoxModelObject* GetLayoutBoxModelObject() const;

  struct AttachContext {
    STACK_ALLOCATED();

   public:
    // Keep track of previously attached in-flow box during attachment so that
    // we don't need to backtrack past display:none/contents and out of flow
    // objects when we need to do whitespace re-attachment.
    LayoutObject* previous_in_flow = nullptr;
    // The parent LayoutObject to use when inserting a new child into the layout
    // tree in LayoutTreeBuilder::CreateLayoutObject.
    LayoutObject* parent = nullptr;
    // LayoutObject to be used as the next pointer when inserting a LayoutObject
    // into the tree.
    LayoutObject* next_sibling = nullptr;
    // Set to true if the AttachLayoutTree is done as part of the
    // RebuildLayoutTree pass.
    bool performing_reattach = false;
    // True if the previous_in_flow member is up-to-date, even if it is nullptr.
    bool use_previous_in_flow = false;
    // True if the next_sibling member is up-to-date, even if it is nullptr.
    bool next_sibling_valid = false;
    // Context for keeping track of counters values in the document.
    CountersAttachmentContext counters_context;

    AttachContext() = default;
    AttachContext(const AttachContext& other)
        : previous_in_flow(other.previous_in_flow),
          parent(other.parent),
          next_sibling(other.next_sibling),
          performing_reattach(other.performing_reattach),
          use_previous_in_flow(other.use_previous_in_flow),
          next_sibling_valid(other.next_sibling_valid),
          counters_context(other.counters_context.ShallowClone()) {}
  };

  // Attaches this node to the layout tree. This calculates the style to be
  // applied to the node and creates an appropriate LayoutObject which will be
  // inserted into the tree (except when the style has display: none). This
  // makes the node visible in the LocalFrameView.
  virtual void AttachLayoutTree(AttachContext&);

  // Detaches the node from the layout tree, making it invisible in the rendered
  // view. This method will remove the node's layout object from the layout tree
  // and delete it.
  void DetachLayoutTree() { DetachLayoutTree(/*performing_reattach=*/false); }
  virtual void DetachLayoutTree(bool performing_reattach);

  void ReattachLayoutTree(AttachContext&);

  bool ShouldSkipMarkingStyleDirty() const;

  // ---------------------------------------------------------------------------
  // Notification of document structure changes (see container_node.h for more
  // notification methods)
  //
  // At first, Blink notifies the node that it has been inserted into the
  // document. This is called during document parsing, and also when a node is
  // added through the DOM methods insertBefore(), appendChild() or
  // replaceChild(). The call happens _after_ the node has been added to the
  // tree.  This is similar to the DOMNodeInsertedIntoDocument DOM event, but
  // does not require the overhead of event dispatching.
  //
  // Blink notifies this callback regardless if the subtree of the node is a
  // document tree or a floating subtree.  Implementation can determine the type
  // of subtree by seeing insertion_point->isConnected().  For performance
  // reasons, notifications are delivered only to ContainerNode subclasses if
  // the insertion_point is not in a document tree.
  //
  // There is another callback, DidNotifySubtreeInsertionsToDocument(),
  // which is called after all the descendants are notified, if this node was
  // inserted into the document tree. Only a few subclasses actually need
  // this. To utilize this, the node should return
  // kInsertionShouldCallDidNotifySubtreeInsertions from InsertedInto().
  //
  // InsertedInto() implementations must not modify the DOM tree, and must not
  // dispatch synchronous events. On the other hand,
  // DidNotifySubtreeInsertionsToDocument() may modify the DOM tree, and may
  // dispatch synchronous events.
  enum InsertionNotificationRequest {
    kInsertionDone,
    kInsertionShouldCallDidNotifySubtreeInsertions
  };

  virtual InsertionNotificationRequest InsertedInto(
      ContainerNode& insertion_point);
  virtual void DidNotifySubtreeInsertionsToDocument() {}

  // Notifies the node that it is no longer part of the tree.
  //
  // This is a dual of InsertedInto(), and is similar to the
  // DOMNodeRemovedFromDocument DOM event, but does not require the overhead of
  // event dispatching, and is called _after_ the node is removed from the tree.
  //
  // RemovedFrom() implementations must not modify the DOM tree, and must not
  // dispatch synchronous events.
  virtual void RemovedFrom(ContainerNode& insertion_point);

  // Notifies the node that it has moved from a different parent.
  // This corresponds to "moving steps"
  // (https://dom.spec.whatwg.org/#concept-node-move-ext)
  virtual void MovedFrom(ContainerNode& old_parent);

  // FIXME(dominicc): This method is not debug-only--it is used by
  // Tracing--rename it to something indicative.
  String DebugName() const;

  String ToString() const;

#if DCHECK_IS_ON()
  String ToTreeStringForThis() const;
  String ToFlatTreeStringForThis() const;
  void PrintNodePathTo(std::ostream&) const;
  String ToMarkedTreeString(const Node* marked_node1,
                            const char* marked_label1,
                            const Node* marked_node2 = nullptr,
                            const char* marked_label2 = nullptr) const;
  String ToMarkedFlatTreeString(const Node* marked_node1,
                                const char* marked_label1,
                                const Node* marked_node2 = nullptr,
                                const char* marked_label2 = nullptr) const;
  void ShowTreeForThisAcrossFrame() const;
#endif

  NodeListsNodeData* NodeLists();
  const NodeListsNodeData* NodeLists() const;
  void ClearNodeLists();

  FlatTreeNodeData* GetFlatTreeNodeData() const;
  FlatTreeNodeData& EnsureFlatTreeNodeData();
  void ClearFlatTreeNodeData();
  void ClearFlatTreeNodeDataIfHostChanged(const ContainerNode& parent);

  virtual bool WillRespondToMouseMoveEvents() const;
  virtual bool WillRespondToMouseClickEvents();

  enum ShadowTreesTreatment {
    kTreatShadowTreesAsDisconnected,
    kTreatShadowTreesAsComposed
  };

  uint16_t compareDocumentPosition(
      const Node*,
      ShadowTreesTreatment = kTreatShadowTreesAsDisconnected) const;

  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  void RemoveAllEventListeners() override;
  void RemoveAllEventListenersRecursively();

  // Handlers to do/undo actions on the target node before an event is
  // dispatched to it and after the event has been dispatched.  The data pointer
  // is handed back by the preDispatch and passed to postDispatch.
  virtual EventDispatchHandlingState* PreDispatchEventHandler(Event&) {
    return nullptr;
  }
  virtual void PostDispatchEventHandler(Event&, EventDispatchHandlingState*) {}

  void DispatchScopedEvent(Event&);

  virtual void HandleLocalEvents(Event&);

  void DispatchSubtreeModifiedEvent();
  DispatchEventResult DispatchDOMActivateEvent(int detail,
                                               Event& underlying_event);

  void DispatchSimulatedClick(const Event* underlying_event,
                              SimulatedClickCreationScope =
                                  SimulatedClickCreationScope::kFromUserAgent);

  // Perform the default action for an event.
  virtual void DefaultEventHandler(Event&);
  void UpdateHadKeyboardEvent(const Event&);
  // Should return true if this Node has activation behavior.
  // https://dom.spec.whatwg.org/#eventtarget-activation-behavior
  virtual bool HasActivationBehavior() const;

  void GetRegisteredMutationObserversOfType(
      HeapHashMap<Member<MutationObserver>, MutationRecordDeliveryOptions>&,
      MutationType,
      const QualifiedName* attribute_name);
  void RegisterMutationObserver(MutationObserver&,
                                MutationObserverOptions,
                                const HashSet<AtomicString>& attribute_filter);
  void UnregisterMutationObserver(MutationObserverRegistration*);
  void RegisterTransientMutationObserver(MutationObserverRegistration*);
  void UnregisterTransientMutationObserver(MutationObserverRegistration*);
  void NotifyMutationObserversNodeWillDetach();

  unsigned ConnectedSubframeCount() const;
  void IncrementConnectedSubframeCount();
  void DecrementConnectedSubframeCount();

  StaticNodeList* getDestinationInsertionPoints();
  HTMLSlotElement* AssignedSlot() const;
  HTMLSlotElement* assignedSlotForBinding();
  HTMLSlotElement* AssignedSlotWithoutRecalc() const;

  bool IsFinishedParsingChildren() const {
    return GetFlag(kIsFinishedParsingChildrenFlag);
  }

  void CheckSlotChange(SlotChangeType);
  void CheckSlotChangeAfterInserted() {
    CheckSlotChange(SlotChangeType::kSignalSlotChangeEvent);
  }
  void CheckSlotChangeBeforeRemoved() {
    CheckSlotChange(SlotChangeType::kSignalSlotChangeEvent);
  }

  // Called from slot re-assignment for host children which change which slot
  // they are assigned to.
  void ParentSlotChanged();

  // Called from slot re-assignment for:
  // 1. Host children that are no longer assigned to a slot.
  // 2. Light tree children of slots which are no longer rendered as fallback
  //    content.
  void RemovedFromFlatTree();

  void SetHasDuplicateAttributes() { SetFlag(kHasDuplicateAttributes); }
  bool HasDuplicateAttribute() const {
    return GetFlag(kHasDuplicateAttributes);
  }

  bool IsEffectiveRootScroller() const;

  virtual LayoutBox* AutoscrollBox();
  virtual void StopAutoscroll();

  // If the node is a plugin, then this returns its WebPluginContainer.
  WebPluginContainerImpl* GetWebPluginContainer() const;

  void RegisterScrollTimeline(ScrollTimeline*);
  void UnregisterScrollTimeline(ScrollTimeline*);

  void AddDOMPart(Part& part) {
    DCHECK(!RuntimeEnabledFeatures::DOMPartsAPIMinimalEnabled());
    EnsureRareData().AddDOMPart(part);
  }
  void RemoveDOMPart(Part& part) {
    DCHECK(!RuntimeEnabledFeatures::DOMPartsAPIMinimalEnabled());
    EnsureRareData().RemoveDOMPart(part);
  }
  PartsList* GetDOMParts() const {
    return data_ ? data_->GetDOMParts() : nullptr;
  }

  DOMNodeId NodeID() const {
    return data_ ? data_->NodeId() : kInvalidDOMNodeId;
  }
  DOMNodeId& EnsureNodeID() { return EnsureRareData().NodeId(); }

  // For the imperative slot distribution API.
  void SetManuallyAssignedSlot(HTMLSlotElement* slot);
  HTMLSlotElement* ManuallyAssignedSlot();

  // For Element.
  void SetHasDisplayLockContext() { SetFlag(kHasDisplayLockContext); }
  bool HasDisplayLockContext() const { return GetFlag(kHasDisplayLockContext); }

  // Converts |node_unions| from bindings into actual Nodes by converting
  // strings and script into text nodes, and if more than one node resulted,
  // removes them from their old parent (as though they had been inserted into
  // a DocumentFragment).
  static HeapVector<Member<Node>> ConvertNodeUnionsIntoNodes(
      const ContainerNode* parent,
      const HeapVector<Member<V8UnionNodeOrStringOrTrustedScript>>& node_unions,
      Document& document,
      const char* property_name,
      ExceptionState& exception_state);

  bool SelfOrAncestorHasDirAutoAttribute() const {
    return GetFlag(kSelfOrAncestorHasDirAutoAttribute);
  }
  void SetSelfOrAncestorHasDirAutoAttribute() {
    SetFlag(kSelfOrAncestorHasDirAutoAttribute);
  }
  void ClearSelfOrAncestorHasDirAutoAttribute() {
    ClearFlag(kSelfOrAncestorHasDirAutoAttribute);
  }
  TextDirection CachedDirectionality() const {
    return (node_flags_ & kCachedDirectionalityIsRtl) ? TextDirection::kRtl
                                                      : TextDirection::kLtr;
  }
  void SetCachedDirectionality(TextDirection direction);

  bool SelfOrAncestorHasContainerTiming() const {
    return GetFlag(kSelfOrAncestorHasContainerTiming);
  }
  void SetSelfOrAncestorHasContainerTiming() {
    SetFlag(kSelfOrAncestorHasContainerTiming);
  }
  void ClearSelfOrAncestorHasContainerTiming() {
    ClearFlag(kSelfOrAncestorHasContainerTiming);
  }
  bool HasContainerTiming() const;

  void Trace(Visitor*) const override;

  bool IsModifiedBySoftNavigation() const {
    return GetFlag(kModifiedBySoftNavigation);
  }
  void SetIsModifiedBySoftNavigation() { SetFlag(kModifiedBySoftNavigation); }

  bool HasNodePart() const { return GetFlag(kHasNodePart); }
  void SetHasNodePart() { SetFlag(kHasNodePart); }
  void ClearHasNodePart() { ClearFlag(kHasNodePart); }

  // This method calls Document::AddConsoleMessage but also attaches this
  // node to the console message so developers can see the relevant element
  // in DevTools.
  void AddConsoleMessage(mojom::blink::ConsoleMessageSource source,
                         mojom::blink::ConsoleMessageLevel level,
                         const String& message);

  // Called when a node changes its flat tree parent, either because slot
  // assignments changed, or the node got reparented by a moveBefore().
  void FlatTreeParentChanged();

 private:
  enum NodeFlags : uint32_t {
    // getNodeType() is called extensively. As it's called quite a bit its
    // value is first so that a bit-shift is not needed to extract the value.
    // Also note the node-type never changes once created.
    kNodeTypeMask = 0xf,
    kIsContainerFlag = 1u << 4,
    kElementNamespaceTypeMask = 0x3u << kElementNamespaceTypeShift,

    // Changes based on if the element should be treated like a link,
    // ex. When setting the href attribute on an <a>.
    kIsLinkFlag = 1u << 7,

    // Changes based on :hover, :active and :focus state.
    kIsUserActionElementFlag = 1u << 8,

    // Tree state flags. These change when the element is added/removed
    // from a DOM tree.
    kIsConnectedFlag = 1u << 9,
    kIsInShadowTreeFlag = 1u << 10,

    // Set by the parser when the children are done parsing.
    kIsFinishedParsingChildrenFlag = 1u << 11,

    // Flags related to recalcStyle.
    kHasCustomStyleCallbacksFlag = 1u << 12,
    kChildNeedsStyleInvalidationFlag = 1u << 13,
    kNeedsStyleInvalidationFlag = 1u << 14,
    kChildNeedsStyleRecalcFlag = 1u << 15,
    kStyleChangeMask = 0x3u << kNodeStyleChangeShift,

    kCustomElementStateMask = 0x7u << kNodeCustomElementShift,

    kHasNameOrIsEditingTextFlag = 1u << 21,

    kNeedsReattachLayoutTree = 1u << 22,
    kChildNeedsReattachLayoutTree = 1u << 23,

    kHasDuplicateAttributes = 1u << 24,

    kForceReattachLayoutTree = 1u << 25,

    kHasDisplayLockContext = 1u << 26,

    kSelfOrAncestorHasDirAutoAttribute = 1u << 27,
    kCachedDirectionalityIsRtl = 1u << 28,

    // Indicates that the node was added in a task descendant of a potential
    // soft navigation.
    kModifiedBySoftNavigation = 1u << 29,

    // Bits indicating this Node is a NodePart or a ChildNodePart endpoint.
    kHasNodePart = 1u << 30,

    // Indicate the node is in a hierarchy that needs to be considered for
    // ContainerTiming events.
    kSelfOrAncestorHasContainerTiming = 1u << 31,

    kDefaultNodeFlags = kIsFinishedParsingChildrenFlag,

    // 1 bit(s) remaining.
  };

  ALWAYS_INLINE bool GetFlag(NodeFlags mask) const {
    return node_flags_ & mask;
  }
  void SetFlag(bool f, NodeFlags mask) {
    node_flags_ = (node_flags_ & ~mask) | (-(int32_t)f & mask);
  }
  void SetFlag(NodeFlags mask) { node_flags_ |= mask; }
  void ClearFlag(NodeFlags mask) { node_flags_ &= ~mask; }

  enum class ElementNamespaceType : uint32_t {
    kHTML = 0,
    kMathML = 1 << kElementNamespaceTypeShift,
    kSVG = 2 << kElementNamespaceTypeShift,
    kOther = 3 << kElementNamespaceTypeShift,
  };
  ALWAYS_INLINE ElementNamespaceType GetElementNamespaceType() const {
    return static_cast<ElementNamespaceType>(node_flags_ &
                                             kElementNamespaceTypeMask);
  }

 protected:
  enum ConstructionType {
    kCreateBase = kDefaultNodeFlags |
                  static_cast<NodeFlags>(ElementNamespaceType::kOther),
    kCreateAttribute = kCreateBase | static_cast<NodeFlags>(kAttributeNode),
    kCreateComment = kCreateBase | static_cast<NodeFlags>(kCommentNode),
    kCreateDocumentType =
        kCreateBase | static_cast<NodeFlags>(kDocumentTypeNode),
    kCreateProcessingInstruction =
        kCreateBase | static_cast<NodeFlags>(kProcessingInstructionNode),
    kCreateCdataSection =
        kCreateBase | static_cast<NodeFlags>(kCdataSectionNode),
    kCreateText = kCreateBase | static_cast<NodeFlags>(kTextNode),
    kCreateElement =
        kCreateBase | kIsContainerFlag | static_cast<NodeFlags>(kElementNode),
    kCreateDocument = kCreateBase | kIsContainerFlag | kIsConnectedFlag |
                      static_cast<NodeFlags>(kDocumentNode),
    kCreateDocumentFragment = kCreateBase | kIsContainerFlag |
                              static_cast<NodeFlags>(kDocumentFragmentNode),
    kCreateShadowRoot = kCreateDocumentFragment | kIsInShadowTreeFlag,
    kCreateHTMLElement = kDefaultNodeFlags | kIsContainerFlag |
                         static_cast<NodeFlags>(kElementNode) |
                         static_cast<NodeFlags>(ElementNamespaceType::kHTML),
    kCreateMathMLElement =
        kDefaultNodeFlags | kIsContainerFlag |
        static_cast<NodeFlags>(kElementNode) |
        static_cast<NodeFlags>(ElementNamespaceType::kMathML),
    kCreateSVGElement = kDefaultNodeFlags | kIsContainerFlag |
                        static_cast<NodeFlags>(kElementNode) |
                        static_cast<NodeFlags>(ElementNamespaceType::kSVG),
    kCreateEditingText = kCreateText | kHasNameOrIsEditingTextFlag,
  };

  Node(TreeScope*, ConstructionType);

  void WillMoveToNewDocument(Document& new_document);
  virtual void DidMoveToNewDocument(Document& old_document);
  void MoveMutationObserversToNewDocument(Document& new_document);

  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;
  void RemovedEventListener(const AtomicString& event_type,
                            const RegisteredEventListener&) override;
  DispatchEventResult DispatchEventInternal(Event&) override;
  void MoveEventListenersToNewDocument(Document& old_document,
                                       Document& new_document);

  // |RareData| cannot be replaced or removed once assigned.
  NodeRareData* RareData() const { return data_.Get(); }
  NodeRareData& EnsureRareData() { return data_ ? *data_ : CreateRareData(); }

  void SetHasCustomStyleCallbacks() {
    SetFlag(true, kHasCustomStyleCallbacksFlag);
  }
  void UnsetHasCustomStyleCallbacks() {
    SetFlag(false, kHasCustomStyleCallbacksFlag);
  }

  void SetTreeScope(TreeScope* scope) { tree_scope_ = scope; }
  void SetIsFinishedParsingChildren(bool value) {
    SetFlag(value, kIsFinishedParsingChildrenFlag);
  }

  void InvalidateIfHasEffectiveAppearance() const;

 private:
  static constexpr struct ParentNodeTag {
  } kParentNodeTag;
  static constexpr struct ShadowHostTag {
  } kShadowHostTag;

  using TaggedParentOrShadowHostNode =
      subtle::TaggedUncompressedMember<Node, ParentNodeTag, ShadowHostTag>;

  Node* ToNode() final;

  bool IsUserActionElementActive() const;
  bool IsUserActionElementInActiveChain() const;
  bool IsUserActionElementDragged() const;
  bool IsUserActionElementHovered() const;
  bool IsUserActionElementFocused() const;
  bool IsUserActionElementHasFocusWithin() const;

  void SetStyleChange(StyleChangeType change_type) {
    node_flags_ = (node_flags_ & ~kStyleChangeMask) | change_type;
  }

  // Used exclusively by |EnsureRareData|.
  NodeRareData& CreateRareData();

  void MaybeAddNodeInsertedTraceEvent();

  const HeapVector<Member<MutationObserverRegistration>>*
  MutationObserverRegistry();
  const HeapHashSet<Member<MutationObserverRegistration>>*
  TransientMutationObserverRegistry();

  ShadowRoot* GetSlotAssignmentRoot() const;

  // EventTarget ends with a single 32-bit member, so put one 32-bit member
  // first to avoid padding on 64-bit.
  uint32_t node_flags_;
  // Both parent and tree_scope are hot accessed members. Keep them uncompressed
  // for performance reasons.
  TaggedParentOrShadowHostNode parent_or_shadow_host_node_;
  subtle::UncompressedMember<TreeScope> tree_scope_;
  // Compressed members and flags are after uncompressed members to minimize
  // padding.
  Member<Node> previous_;
  Member<Node> next_;
  Member<LayoutObject> layout_object_;
  Member<NodeRareData> data_;
};

inline void Node::SetParentNode(ContainerNode* parent) {
  DCHECK(IsMainThread());
  parent_or_shadow_host_node_.SetAs<ParentNodeTag>(
      reinterpret_cast<Node*>(parent));
}

inline void Node::SetShadowHostNode(ContainerNode* shadow_host) {
  DCHECK(IsMainThread());
  parent_or_shadow_host_node_.SetAs<ShadowHostTag>(
      reinterpret_cast<Node*>(shadow_host));
}

inline ContainerNode* Node::ParentOrShadowHostNode() const {
  DCHECK(IsMainThread());
  return reinterpret_cast<ContainerNode*>(
      parent_or_shadow_host_node_.GetUntagged());
}

// Allow equality comparisons of Nodes by reference or pointer, interchangeably.
DEFINE_COMPARISON_OPERATORS_WITH_REFERENCES(Node)

CORE_EXPORT std::ostream& operator<<(std::ostream&, const Node&);
CORE_EXPORT std::ostream& operator<<(std::ostream&, const Node*);

}  // namespace blink

#if DCHECK_IS_ON()
// Outside the blink namespace for ease of invocation from gdb.
void ShowNode(const blink::Node*);
void ShowTree(const blink::Node*);
void ShowNodePath(const blink::Node*);
#endif

namespace cppgc {
// Assign Node to be allocated on custom NodeSpace.
template <typename T>
struct SpaceTrait<T, std::enable_if_t<std::is_base_of<blink::Node, T>::value>> {
  using Space = blink::NodeSpace;
};
}  // namespace cppgc

namespace blink {
template <typename T>
struct ThreadingTrait<
    T,
    std::enable_if_t<std::is_base_of<blink::Node, T>::value>> {
  static constexpr ThreadAffinity kAffinity = kMainThreadOnly;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_H_
