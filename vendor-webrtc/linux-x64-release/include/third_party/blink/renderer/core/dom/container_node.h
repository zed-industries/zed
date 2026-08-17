/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2009, 2010, 2011, 2013 Apple Inc. All
 * rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CONTAINER_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CONTAINER_NODE_H_

#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/style_recalc_change.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/static_node_list.h"
#include "third_party/blink/renderer/core/html/collection_type.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Element;
class ExceptionState;
class GetHTMLOptions;
class HTMLCollection;
class RadioNodeList;
class StyleRecalcContext;
class WhitespaceAttacher;

using StaticElementList = StaticNodeTypeList<Element>;

enum class DynamicRestyleFlags {
  kChildrenOrSiblingsAffectedByFocus = 1 << 0,
  kChildrenOrSiblingsAffectedByHover = 1 << 1,
  kChildrenOrSiblingsAffectedByActive = 1 << 2,
  kChildrenOrSiblingsAffectedByDrag = 1 << 3,
  kChildrenAffectedByFirstChildRules = 1 << 4,
  kChildrenAffectedByLastChildRules = 1 << 5,
  kChildrenAffectedByDirectAdjacentRules = 1 << 6,
  kChildrenAffectedByIndirectAdjacentRules = 1 << 7,
  kChildrenAffectedByForwardPositionalRules = 1 << 8,
  kChildrenAffectedByBackwardPositionalRules = 1 << 9,
  kAffectedByFirstChildRules = 1 << 10,
  kAffectedByLastChildRules = 1 << 11,
  kChildrenOrSiblingsAffectedByFocusWithin = 1 << 12,
  kChildrenOrSiblingsAffectedByFocusVisible = 1 << 13,

  kNumberOfDynamicRestyleFlags = 14,

  kChildrenAffectedByStructuralRules =
      kChildrenAffectedByFirstChildRules | kChildrenAffectedByLastChildRules |
      kChildrenAffectedByDirectAdjacentRules |
      kChildrenAffectedByIndirectAdjacentRules |
      kChildrenAffectedByForwardPositionalRules |
      kChildrenAffectedByBackwardPositionalRules
};

enum SubtreeModificationAction {
  kDispatchSubtreeModifiedEvent,
  kOmitSubtreeModifiedEvent
};

// This constant controls how much buffer is initially allocated
// for a Node Vector that is used to store child Nodes of a given Node.
// FIXME: Optimize the value.
const int kInitialNodeVectorSize = 11;
using NodeVector = HeapVector<Member<Node>, kInitialNodeVectorSize>;

// ContainerNode itself isn't web-exposed exactly, but it maps closely to the
// ParentNode mixin interface. A number of methods it implements (such as
// firstChild, lastChild) use web-style naming to shadow the corresponding
// methods on Node. This is a performance optimization, as it avoids a virtual
// dispatch if the type is statically known to be ContainerNode.
class CORE_EXPORT ContainerNode : public Node {
 public:
  ~ContainerNode() override;

  // ParentNode web-exposed:
  // Note that some of the ParentNode interface is implemented in Node.
  HTMLCollection* children();
  Element* firstElementChild();
  Element* lastElementChild();
  unsigned childElementCount();
  Element* querySelector(const AtomicString& selectors, ExceptionState&);
  StaticElementList* querySelectorAll(const AtomicString& selectors,
                                      ExceptionState&);

  Node* firstChild() const { return first_child_.Get(); }
  Node* lastChild() const { return last_child_.Get(); }
  bool hasChildren() const { return static_cast<bool>(first_child_); }
  bool HasChildren() const { return static_cast<bool>(first_child_); }

  bool HasOneChild() const {
    return first_child_ && !first_child_->HasNextSibling();
  }

  bool HasChildCount(unsigned) const;
  unsigned CountChildren() const;

  bool HasOneTextChild() const {
    return HasOneChild() && first_child_->IsTextNode();
  }

  Element* QuerySelector(const AtomicString& selectors, ExceptionState&);
  Element* QuerySelector(const AtomicString& selectors);
  StaticElementList* QuerySelectorAll(const AtomicString& selectors,
                                      ExceptionState&);
  StaticElementList* QuerySelectorAll(const AtomicString& selectors);

  void InsertBefore(const VectorOf<Node>& new_children,
                    Node* ref_child,
                    ExceptionState&);
  Node* InsertBefore(Node* new_child, Node* ref_child, ExceptionState&);
  Node* InsertBefore(Node* new_child, Node* ref_child);
  void ReplaceChild(const VectorOf<Node>& new_children,
                    Node* old_child,
                    ExceptionState&);
  Node* ReplaceChild(Node* new_child, Node* old_child, ExceptionState&);
  Node* ReplaceChild(Node* new_child, Node* old_child);
  Node* RemoveChild(Node* child, ExceptionState&);
  Node* RemoveChild(Node* child);
  void AppendChildren(const VectorOf<Node>& new_children, ExceptionState&);
  Node* AppendChild(Node* new_child, ExceptionState&);
  Node* AppendChild(Node* new_child);
  bool EnsurePreInsertionValidity(const Node* new_child,
                                  const VectorOf<Node>* new_children,
                                  const Node* next,
                                  const Node* old_child,
                                  ExceptionState&) const;

  Element* getElementById(const AtomicString& id) const;
  HTMLCollection* getElementsByTagName(const AtomicString&);
  HTMLCollection* getElementsByTagNameNS(const AtomicString& namespace_uri,
                                         const AtomicString& local_name);
  NodeList* getElementsByName(const AtomicString& element_name);
  HTMLCollection* getElementsByClassName(const AtomicString& class_names);
  RadioNodeList* GetRadioNodeList(const AtomicString&,
                                  bool only_match_img_elements = false);

  // Returns all Text nodes where `regex` would match for the text inside of
  // the node, case-insensitive. This function does not normalize adjacent Text
  // nodes and search them together. It only matches within individual Text
  // nodes. It is therefore possible that some text is displayed to the user as
  // a single run of text, but will not match the regex, because the nodes
  // aren't normalized. This function searches within both the DOM and Shadow
  // DOM.
  StaticNodeList* FindAllTextNodesMatchingRegex(const String& regex) const;

  // These methods are only used during parsing.
  // They don't send DOM mutation events or accept DocumentFragments.
  void ParserAppendChild(Node*);

  // Called when the parser adds a child to a DocumentFragment as the result
  // of parsing inner/outer html.
  void ParserAppendChildInDocumentFragment(Node* new_child);
  // Called when the parser has finished building a DocumentFragment. This is
  // not called if the parser fails parsing (if parsing fails, the
  // DocumentFragment is orphaned and will eventually be gc'd).
  //
  // ShouldNotifyInsertedNodes controls whether to skip notifications that are
  // redone if the contents of the DocumentFragment are moved to a new parent.
  enum class ShouldNotifyInsertedNodes { kNotify, kSkip };
  void ParserFinishedBuildingDocumentFragment(ShouldNotifyInsertedNodes);
  void ParserRemoveChild(Node&);
  void ParserInsertBefore(Node* new_child, Node& ref_child);
  void ParserTakeAllChildrenFrom(ContainerNode&);

  void RemoveChildren(
      SubtreeModificationAction = kDispatchSubtreeModifiedEvent);

  void CloneChildNodesFrom(const ContainerNode&, NodeCloningData&);

  using Node::DetachLayoutTree;
  void AttachLayoutTree(AttachContext&) override;
  void DetachLayoutTree(bool performing_reattach) override;
  PhysicalRect BoundingBox() const final;

  void RemovedFrom(ContainerNode& insertion_point) override;

  bool ChildrenOrSiblingsAffectedByFocus() const {
    return HasRestyleFlag(
        DynamicRestyleFlags::kChildrenOrSiblingsAffectedByFocus);
  }
  void SetChildrenOrSiblingsAffectedByFocus() {
    SetRestyleFlag(DynamicRestyleFlags::kChildrenOrSiblingsAffectedByFocus);
  }

  bool ChildrenOrSiblingsAffectedByFocusVisible() const {
    return HasRestyleFlag(
        DynamicRestyleFlags::kChildrenOrSiblingsAffectedByFocusVisible);
  }
  void SetChildrenOrSiblingsAffectedByFocusVisible() {
    SetRestyleFlag(
        DynamicRestyleFlags::kChildrenOrSiblingsAffectedByFocusVisible);
  }

  bool ChildrenOrSiblingsAffectedByFocusWithin() const {
    return HasRestyleFlag(
        DynamicRestyleFlags::kChildrenOrSiblingsAffectedByFocusWithin);
  }
  void SetChildrenOrSiblingsAffectedByFocusWithin() {
    SetRestyleFlag(
        DynamicRestyleFlags::kChildrenOrSiblingsAffectedByFocusWithin);
  }

  bool ChildrenOrSiblingsAffectedByHover() const {
    return HasRestyleFlag(
        DynamicRestyleFlags::kChildrenOrSiblingsAffectedByHover);
  }
  void SetChildrenOrSiblingsAffectedByHover() {
    SetRestyleFlag(DynamicRestyleFlags::kChildrenOrSiblingsAffectedByHover);
  }

  bool ChildrenOrSiblingsAffectedByActive() const {
    return HasRestyleFlag(
        DynamicRestyleFlags::kChildrenOrSiblingsAffectedByActive);
  }
  void SetChildrenOrSiblingsAffectedByActive() {
    SetRestyleFlag(DynamicRestyleFlags::kChildrenOrSiblingsAffectedByActive);
  }

  bool ChildrenOrSiblingsAffectedByDrag() const {
    return HasRestyleFlag(
        DynamicRestyleFlags::kChildrenOrSiblingsAffectedByDrag);
  }
  void SetChildrenOrSiblingsAffectedByDrag() {
    SetRestyleFlag(DynamicRestyleFlags::kChildrenOrSiblingsAffectedByDrag);
  }

  bool ChildrenAffectedByFirstChildRules() const {
    return HasRestyleFlag(
        DynamicRestyleFlags::kChildrenAffectedByFirstChildRules);
  }
  void SetChildrenAffectedByFirstChildRules() {
    SetRestyleFlag(DynamicRestyleFlags::kChildrenAffectedByFirstChildRules);
  }

  bool ChildrenAffectedByLastChildRules() const {
    return HasRestyleFlag(
        DynamicRestyleFlags::kChildrenAffectedByLastChildRules);
  }
  void SetChildrenAffectedByLastChildRules() {
    SetRestyleFlag(DynamicRestyleFlags::kChildrenAffectedByLastChildRules);
  }

  bool ChildrenAffectedByDirectAdjacentRules() const {
    return HasRestyleFlag(
        DynamicRestyleFlags::kChildrenAffectedByDirectAdjacentRules);
  }
  void SetChildrenAffectedByDirectAdjacentRules() {
    SetRestyleFlag(DynamicRestyleFlags::kChildrenAffectedByDirectAdjacentRules);
  }

  bool ChildrenAffectedByIndirectAdjacentRules() const {
    return HasRestyleFlag(
        DynamicRestyleFlags::kChildrenAffectedByIndirectAdjacentRules);
  }
  void SetChildrenAffectedByIndirectAdjacentRules() {
    SetRestyleFlag(
        DynamicRestyleFlags::kChildrenAffectedByIndirectAdjacentRules);
  }

  bool ChildrenAffectedByForwardPositionalRules() const {
    return HasRestyleFlag(
        DynamicRestyleFlags::kChildrenAffectedByForwardPositionalRules);
  }
  void SetChildrenAffectedByForwardPositionalRules() {
    SetRestyleFlag(
        DynamicRestyleFlags::kChildrenAffectedByForwardPositionalRules);
  }

  bool ChildrenAffectedByBackwardPositionalRules() const {
    return HasRestyleFlag(
        DynamicRestyleFlags::kChildrenAffectedByBackwardPositionalRules);
  }
  void SetChildrenAffectedByBackwardPositionalRules() {
    SetRestyleFlag(
        DynamicRestyleFlags::kChildrenAffectedByBackwardPositionalRules);
  }

  bool AffectedByFirstChildRules() const {
    return HasRestyleFlag(DynamicRestyleFlags::kAffectedByFirstChildRules);
  }
  void SetAffectedByFirstChildRules() {
    SetRestyleFlag(DynamicRestyleFlags::kAffectedByFirstChildRules);
  }

  bool AffectedByLastChildRules() const {
    return HasRestyleFlag(DynamicRestyleFlags::kAffectedByLastChildRules);
  }
  void SetAffectedByLastChildRules() {
    SetRestyleFlag(DynamicRestyleFlags::kAffectedByLastChildRules);
  }

  bool NeedsAdjacentStyleRecalc() const;

  // FIXME: These methods should all be renamed to something better than
  // "check", since it's not clear that they alter the style bits of siblings
  // and children.
  enum SiblingCheckType {
    kFinishedParsingChildren,
    kSiblingElementInserted,
    kSiblingElementRemoved
  };
  void CheckForSiblingStyleChanges(SiblingCheckType,
                                   Element* changed_element,
                                   Node* node_before_change,
                                   Node* node_after_change);
  void RecalcDescendantStyles(const StyleRecalcChange,
                              const StyleRecalcContext&,
                              Element& host_or_element);
  void RebuildChildrenLayoutTrees(WhitespaceAttacher&);
  void RebuildLayoutTreeForChild(Node* child, WhitespaceAttacher&);

  // -----------------------------------------------------------------------------
  // Notification of document structure changes (see core/dom/node.h for more
  // notification methods)

  enum class ChildrenChangeType : uint8_t {
    kElementInserted,
    kNonElementInserted,
    kElementRemoved,
    kNonElementRemoved,
    kAllChildrenRemoved,
    kTextChanged,
    // When the parser builds nodes (because of inner/outer-html or
    // parseFromString) a single ChildrenChange event is sent at the end.
    kFinishedBuildingDocumentFragmentTree,
  };
  enum class ChildrenChangeSource : uint8_t { kAPI, kParser };
  enum class ChildrenChangeAffectsElements : uint8_t { kNo, kYes };
  struct ChildrenChange {
    STACK_ALLOCATED();

   public:
    static ChildrenChange ForFinishingBuildingDocumentFragmentTree() {
      return ChildrenChange{
          .type = ChildrenChangeType::kFinishedBuildingDocumentFragmentTree,
          .by_parser = ChildrenChangeSource::kParser,
          .affects_elements = ChildrenChangeAffectsElements::kYes,
      };
    }
    static ChildrenChange ForInsertion(Node& node,
                                       Node* unchanged_previous,
                                       Node* unchanged_next,
                                       ChildrenChangeSource by_parser) {
      ChildrenChange change = {
          .type = node.IsElementNode()
                      ? ChildrenChangeType::kElementInserted
                      : ChildrenChangeType::kNonElementInserted,
          .by_parser = by_parser,
          .affects_elements = node.IsElementNode()
                                  ? ChildrenChangeAffectsElements::kYes
                                  : ChildrenChangeAffectsElements::kNo,
          .sibling_changed = &node,
          .sibling_before_change = unchanged_previous,
          .sibling_after_change = unchanged_next,
      };
      return change;
    }

    static ChildrenChange ForRemoval(Node& node,
                                     Node* previous_sibling,
                                     Node* next_sibling,
                                     ChildrenChangeSource by_parser) {
      ChildrenChange change = {
          .type = node.IsElementNode() ? ChildrenChangeType::kElementRemoved
                                       : ChildrenChangeType::kNonElementRemoved,
          .by_parser = by_parser,
          .affects_elements = node.IsElementNode()
                                  ? ChildrenChangeAffectsElements::kYes
                                  : ChildrenChangeAffectsElements::kNo,
          .sibling_changed = &node,
          .sibling_before_change = previous_sibling,
          .sibling_after_change = next_sibling,
      };
      return change;
    }

    bool IsChildInsertion() const {
      return type == ChildrenChangeType::kElementInserted ||
             type == ChildrenChangeType::kNonElementInserted ||
             type == ChildrenChangeType::kFinishedBuildingDocumentFragmentTree;
    }
    bool IsChildRemoval() const {
      return type == ChildrenChangeType::kElementRemoved ||
             type == ChildrenChangeType::kNonElementRemoved;
    }
    bool IsChildElementChange() const {
      return type == ChildrenChangeType::kElementInserted ||
             type == ChildrenChangeType::kElementRemoved ||
             type == ChildrenChangeType::kFinishedBuildingDocumentFragmentTree;
    }

    bool ByParser() const { return by_parser == ChildrenChangeSource::kParser; }

    const ChildrenChangeType type;
    const ChildrenChangeSource by_parser;
    const ChildrenChangeAffectsElements affects_elements;
    Node* const sibling_changed = nullptr;
    // |siblingBeforeChange| is
    //  - siblingChanged.previousSibling before node removal
    //  - siblingChanged.previousSibling after single node insertion
    //  - previousSibling of the first inserted node after multiple node
    //    insertion
    //  - null for kFinishedBuildingDocumentFragmentTree.
    Node* const sibling_before_change = nullptr;
    // |siblingAfterChange| is
    //  - siblingChanged.nextSibling before node removal
    //  - siblingChanged.nextSibling after single node insertion
    //  - nextSibling of the last inserted node after multiple node insertion.
    //  - null for kFinishedBuildingDocumentFragmentTree.
    Node* const sibling_after_change = nullptr;
    // List of removed nodes for ChildrenChangeType::kAllChildrenRemoved.
    // Only populated if ChildrenChangedAllChildrenRemovedNeedsList() returns
    // true.
    const HeapVector<Member<Node>> removed_nodes;
    // Non-null if and only if |type| is ChildrenChangeType::kTextChanged.
    const String* const old_text = nullptr;
  };

  // Notifies the node that it's list of children have changed (either by adding
  // or removing child nodes), or a child node that is of the type
  // kCdataSectionNode, kTextNode or kCommentNode has changed its value.
  //
  // ChildrenChanged() implementations may modify the DOM tree, and may dispatch
  // synchronous events.
  virtual void ChildrenChanged(const ChildrenChange&);

  // Provides ChildrenChange::removed_nodes for kAllChildrenRemoved.
  virtual bool ChildrenChangedAllChildrenRemovedNeedsList() const;

  virtual bool ChildrenCanHaveStyle() const { return true; }

  // This is similar to GetLayoutBox(), but returns nullptr if it's not
  // scrollable. Some elements override this to delegate scroll operations to
  // a descendant LayoutBox.
  virtual LayoutBox* GetLayoutBoxForScrolling() const;

  Element* GetAutofocusDelegate() const;

  bool IsReadingFlowContainer() const;

  HTMLCollection* PopoverInvokers() {
    DCHECK(IsTreeScope());
    return EnsureCachedCollection<HTMLCollection>(kPopoverInvokers);
  }

  HTMLCollection* CommandInvokers() {
    DCHECK(IsTreeScope());
    return EnsureCachedCollection<HTMLCollection>(kCommandInvokers);
  }

  void ReplaceChildren(const VectorOf<Node>& nodes,
                       ExceptionState& exception_state);

  // IDL implementation of getHTML. This is exposed on Element and ShadowRoot
  // only.
  String getHTML(const GetHTMLOptions*, ExceptionState&) const;

  // DocumentOrElementEventHandlers:
  // These event listeners are only actually web-exposed on interfaces that
  // include the DocumentOrElementEventHandlers mixin in their idl.
  DEFINE_ATTRIBUTE_EVENT_LISTENER(copy, kCopy)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(cut, kCut)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(paste, kPaste)

  void Trace(Visitor*) const override;

 protected:
  ContainerNode(TreeScope*, ConstructionType);

  // |attr_name| and |owner_element| are only used for element attribute
  // modifications. |ChildrenChange| is either nullptr or points to a
  // ChildNode::ChildrenChange structure that describes the changes in the tree.
  // If non-null, blink may preserve caches that aren't affected by the change.
  void InvalidateNodeListCachesInAncestors(const QualifiedName* attr_name,
                                           Element* attribute_owner_element,
                                           const ChildrenChange*);

  void SetFirstChild(Node* child) { first_child_ = child; }
  void SetLastChild(Node* child) { last_child_ = child; }

  // Utility functions for NodeListsNodeData API.
  template <typename Collection>
  Collection* EnsureCachedCollection(CollectionType);
  template <typename Collection>
  Collection* EnsureCachedCollection(CollectionType, const AtomicString& name);
  template <typename Collection>
  Collection* EnsureCachedCollection(CollectionType,
                                     const AtomicString& namespace_uri,
                                     const AtomicString& local_name);
  template <typename Collection>
  Collection* CachedCollection(CollectionType);
  template <typename Collection>
  const Collection* CachedCollection(CollectionType) const;

 private:
  bool IsContainerNode() const =
      delete;  // This will catch anyone doing an unnecessary check.
  bool IsTextNode() const =
      delete;  // This will catch anyone doing an unnecessary check.

  // Called from ParserFinishedBuildingDocumentFragment() to notify `node` that
  // it was inserted.
  void NotifyNodeAtEndOfBuildingFragmentTree(Node& node,
                                             const ChildrenChange& change,
                                             bool may_contain_shadow_roots,
                                             ShouldNotifyInsertedNodes);

  NodeListsNodeData& EnsureNodeLists();
  void RemoveBetween(Node* previous_child, Node* next_child, Node& old_child);
  // Inserts the specified nodes before |next|.
  // |next| may be nullptr.
  template <typename Functor>
  void InsertNodeVector(const NodeVector&,
                        Node* next,
                        const Functor&,
                        NodeVector& post_insertion_notification_targets);
  void DidInsertNodeVector(
      const NodeVector&,
      Node* next,
      const NodeVector& post_insertion_notification_targets);
  class AdoptAndInsertBefore;
  class AdoptAndAppendChild;
  friend class AdoptAndInsertBefore;
  friend class AdoptAndAppendChild;
  void InsertBeforeCommon(Node& next_child, Node& new_child);
  void AppendChildCommon(Node& child);
  void WillRemoveChildren();
  void WillRemoveChild(Node& child);
  void RemoveDetachedChildrenInContainer(ContainerNode&);
  void AddChildNodesToDeletionQueue(Node*&, Node*&, ContainerNode&);

  void NotifyNodeInserted(Node&,
                          ChildrenChangeSource = ChildrenChangeSource::kAPI);
  void NotifyNodeInsertedInternal(
      Node&,
      NodeVector& post_insertion_notification_targets);
  void NotifyNodeRemoved(Node&);

  bool HasRestyleFlag(DynamicRestyleFlags mask) const {
    if (const NodeRareData* data = RareData()) {
      return data->HasRestyleFlag(mask);
    }
    return false;
  }
  bool HasRestyleFlags() const {
    if (const NodeRareData* data = RareData()) {
      return data->HasRestyleFlags();
    }
    return false;
  }
  void SetRestyleFlag(DynamicRestyleFlags);

  bool RecheckNodeInsertionStructuralPrereq(const NodeVector&,
                                            const Node* next,
                                            ExceptionState&);
  inline bool CheckParserAcceptChild(const Node& new_child) const;
  inline bool IsHostIncludingInclusiveAncestorOfThis(const Node&,
                                                     ExceptionState&) const;

  void CheckSoftNavigationHeuristicsTracking(const Document& document,
                                             Node& inserted_node);

  Member<Node> first_child_;
  Member<Node> last_child_;
};

template <>
struct DowncastTraits<ContainerNode> {
  static bool AllowFrom(const Node& node) { return node.IsContainerNode(); }
};

inline bool ContainerNode::HasChildCount(unsigned count) const {
  Node* child = first_child_.Get();
  while (count && child) {
    child = child->nextSibling();
    --count;
  }
  return !count && !child;
}

inline ContainerNode::ContainerNode(TreeScope* tree_scope,
                                    ConstructionType type)
    : Node(tree_scope, type), first_child_(nullptr), last_child_(nullptr) {}

inline bool ContainerNode::NeedsAdjacentStyleRecalc() const {
  if (!ChildrenAffectedByDirectAdjacentRules() &&
      !ChildrenAffectedByIndirectAdjacentRules())
    return false;
  return ChildNeedsStyleRecalc() || ChildNeedsStyleInvalidation();
}

inline unsigned Node::CountChildren() const {
  auto* this_node = DynamicTo<ContainerNode>(*this);
  if (!this_node)
    return 0;
  return this_node->CountChildren();
}

inline Node* Node::firstChild() const {
  auto* this_node = DynamicTo<ContainerNode>(*this);
  if (!this_node)
    return nullptr;
  return this_node->firstChild();
}

inline Node* Node::lastChild() const {
  auto* this_node = DynamicTo<ContainerNode>(*this);
  if (!this_node)
    return nullptr;
  return this_node->lastChild();
}

inline ContainerNode* Node::ParentElementOrShadowRoot() const {
  ContainerNode* parent = parentNode();
  return parent && (parent->IsElementNode() || parent->IsShadowRoot())
             ? parent
             : nullptr;
}

inline ContainerNode* Node::ParentElementOrDocumentFragment() const {
  ContainerNode* parent = parentNode();
  return parent && (parent->IsElementNode() || parent->IsDocumentFragment())
             ? parent
             : nullptr;
}

inline bool Node::IsTreeScope() const {
  return &GetTreeScope().RootNode() == this;
}

inline void GetChildNodes(ContainerNode& node, NodeVector& nodes) {
  DCHECK(!nodes.size());
  for (Node* child = node.firstChild(); child; child = child->nextSibling())
    nodes.push_back(child);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CONTAINER_NODE_H_
