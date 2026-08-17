// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_PROPERTY_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_PROPERTY_NODE_H_

#include <algorithm>
#include <iosfwd>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/memory/stack_allocated.h"
#include "cc/trees/property_ids.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/json/json_values.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

#if DCHECK_IS_ON()
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#endif

namespace blink {

// Used to report whether and how paint properties have changed. The order is
// important - it must go from no change to the most significant change.
enum class PaintPropertyChangeType : unsigned char {
  // Nothing has changed.
  kUnchanged,
  // We only changed values that are either mutated by compositor animations
  // which are updated automatically during the compositor-side animation tick,
  // or have been updated directly on the associated compositor node during the
  // PrePaint lifecycle phase.
  kChangedOnlyCompositedValues,
  // We only changed values that don't require re-raster (e.g. compositor
  // element id changed).
  kChangedOnlyNonRerasterValues,
  // We only changed values and not the hierarchy of the tree, and we know that
  // the value changes are 'simple' in that they don't cause cascading changes.
  // For example, they do not cause a new render surface to be created, which
  // may otherwise cause tree changes elsewhere. An example of this is opacity
  // changing in the [0, 1) range. PaintPropertyTreeBuilder may try to directly
  // update the associated compositor node through PaintArtifactCompositor::
  // DirectlyUpdate*(), and if that's successful, the change will be downgraded
  // to kChangeOnlyCompositedValues.
  kChangedOnlySimpleValues,
  // We only changed values and not the hierarchy of the tree, but nothing is
  // known about the kind of value change. The difference between
  // kChangedOnlySimpleValues and kChangedOnlyValues is only meaningful in
  // PaintPropertyTreeBuilder for eligibility of direct update of compositor
  // node. Otherwise we should never distinguish between them.
  kChangedOnlyValues,
  // We have directly modified the tree topology by adding or removing a node.
  kNodeAddedOrRemoved,
};

PLATFORM_EXPORT const char* PaintPropertyChangeTypeToString(
    PaintPropertyChangeType);

class PLATFORM_EXPORT PaintPropertyNode
    : public GarbageCollected<PaintPropertyNode> {
 public:
  PaintPropertyNode(const PaintPropertyNode&) = delete;
  PaintPropertyNode& operator=(const PaintPropertyNode&) = delete;

  virtual ~PaintPropertyNode() = default;

  virtual void Trace(Visitor* visitor) const { visitor->Trace(parent_); }

  bool IsRoot() const { return !parent_; }

  // Returns true if this node is an alias for its parent. A parent alias is a
  // node which on its own does not contribute to the rendering output, and only
  // exists to enforce a particular structure of the paint property tree. Its
  // value is ignored during display item list generation, instead the parent
  // value is used. See Unalias().
  bool IsParentAlias() const { return is_parent_alias_; }

  void CompositorSimpleValuesUpdated() const {
    if (changed_ == PaintPropertyChangeType::kChangedOnlySimpleValues)
      changed_ = PaintPropertyChangeType::kChangedOnlyCompositedValues;
  }

  String ToString() const;

  virtual std::unique_ptr<JSONObject> ToJSON() const;

  int CcNodeId(int sequence_number) const {
    return cc_sequence_number_ == sequence_number ? cc_node_id_
                                                  : cc::kInvalidPropertyNodeId;
  }
  void SetCcNodeId(int sequence_number, int id) const {
    cc_sequence_number_ = sequence_number;
    cc_node_id_ = id;
  }

  PaintPropertyChangeType NodeChanged() const { return changed_; }
  bool NodeChangeAffectsRaster() const {
    return changed_ != PaintPropertyChangeType::kUnchanged &&
           changed_ != PaintPropertyChangeType::kChangedOnlyNonRerasterValues;
  }

#if DCHECK_IS_ON()
  String ToTreeString() const;

  String DebugName() const { return debug_name_; }
  void SetDebugName(const String& name) { debug_name_ = name; }
#endif

 protected:
  // The tags have the following purposes:
  // 1. Distinguish the constructors in parent alias subclasses from the copy
  //    constructors (deleted in this class);
  // 2. Prevent the subclass constructors (which are public required by
  //    MakeGarbageCollected) from being called from outside (which is required
  //    by 1 for parent alias subclasses, and for consistency for
  //    non-parent-alias subclasses).
  enum RootTag { kRoot };
  enum ParentAliasTag { kParentAlias };
  enum NonParentAliasTag { kNonParentAlias };

  explicit PaintPropertyNode(RootTag)
      : is_parent_alias_(false),
        changed_(PaintPropertyChangeType::kUnchanged),
        parent_(nullptr) {}
  PaintPropertyNode(ParentAliasTag, const PaintPropertyNode& parent)
      : is_parent_alias_(true),
        changed_(PaintPropertyChangeType::kNodeAddedOrRemoved),
        parent_(parent) {}
  PaintPropertyNode(NonParentAliasTag, const PaintPropertyNode& parent)
      : is_parent_alias_(false),
        changed_(PaintPropertyChangeType::kNodeAddedOrRemoved),
        parent_(parent) {}

  PaintPropertyChangeType SetParent(const PaintPropertyNode& parent) {
    DCHECK(!IsRoot());
    DCHECK_NE(&parent, this);
    if (&parent == parent_) {
      return PaintPropertyChangeType::kUnchanged;
    }
    parent_ = &parent;
    AddChanged(PaintPropertyChangeType::kChangedOnlyValues);
    return PaintPropertyChangeType::kChangedOnlyValues;
  }

  // Parent property node, or nullptr if this is the root node.
  const PaintPropertyNode* Parent() const { return parent_.Get(); }

  // Returns the first node up the parent chain that is not an alias; return the
  // root node if every node is an alias.
  const PaintPropertyNode& Unalias() const {
    const auto* node = this;
    while (node->Parent() && node->IsParentAlias()) {
      node = node->Parent();
    }
    return *node;
  }

  const PaintPropertyNode* UnaliasedParent() const {
    return Parent() ? &Parent()->Unalias() : nullptr;
  }

  bool IsAncestorOf(const PaintPropertyNode& other) const {
    for (const auto* node = &other; node != this; node = node->Parent()) {
      if (!node) {
        return false;
      }
    }
    return true;
  }

  const PaintPropertyNode& LowestCommonAncestor(
      const PaintPropertyNode& other) const {
    // Fast path of common cases.
    if (this == &other || !Parent() || other.Parent() == this) {
      DCHECK(IsAncestorOf(other));
      return *this;
    }
    if (!other.Parent() || Parent() == &other) {
      DCHECK(other.IsAncestorOf(*this));
      return other;
    }
    return LowestCommonAncestorInternal(other);
  }

  virtual void AddChanged(PaintPropertyChangeType changed) {
    DCHECK(!IsRoot());
    changed_ = std::max(changed_, changed);
  }

  // The following two functions are for subclasses to implement
  // ClearChangedToRoot() which should clear changed flags along the path to
  // the root, and set sequence number.
  // For information about |sequence_number|, see: |changed_sequence_number_|.
  void ClearChanged(int sequence_number) const {
    DCHECK_NE(changed_sequence_number_, sequence_number);
    changed_ = PaintPropertyChangeType::kUnchanged;
    changed_sequence_number_ = sequence_number;
  }
  int ChangedSequenceNumber() const { return changed_sequence_number_; }

 private:
  friend class PaintPropertyNodeTest;
  friend class PropertyTreePrinter;

  // Returns -1 if `maybe_ancestor` is found in the ancestor chain, or returns
  // the depth of the node from the root.
  int NodeDepthOrFoundAncestor(const PaintPropertyNode& maybe_ancestor) const;
  const PaintPropertyNode& LowestCommonAncestorInternal(
      const PaintPropertyNode& other) const;

  // Indicates whether this node is an alias for its parent. Parent aliases are
  // nodes that do not affect rendering and are ignored for the purposes of
  // display item list generation.
  bool is_parent_alias_;

  // Indicates that the paint property value changed in the last update in the
  // prepaint lifecycle step. This is used for raster invalidation and damage
  // in the compositor. This value is cleared through ClearChangedToRoot().
  mutable PaintPropertyChangeType changed_;
  // The changed sequence number is an optimization to avoid an O(n^2) to O(n^4)
  // treewalk when clearing the changed bits for the entire tree. When starting
  // to clear the changed bits, a new (unique) number is selected for the entire
  // tree, and |changed_sequence_number_| is set to this number if the node (and
  // ancestors) have already been visited for clearing.
  mutable int changed_sequence_number_ = 0;

  // Caches the id of the associated cc property node. It's valid only when
  // cc_sequence_number_ matches the sequence number of the cc property tree.
  mutable int cc_node_id_ = cc::kInvalidPropertyNodeId;
  mutable int cc_sequence_number_ = 0;

  Member<const PaintPropertyNode> parent_;

#if DCHECK_IS_ON()
  String debug_name_;
#endif
};

template <typename NodeTypeOrAlias, typename NodeType>
class PaintPropertyNodeBase : public PaintPropertyNode {
 public:
  PaintPropertyChangeType SetParent(const NodeTypeOrAlias& parent) {
    return PaintPropertyNode::SetParent(parent);
  }

  const NodeTypeOrAlias* Parent() const {
    return static_cast<const NodeTypeOrAlias*>(PaintPropertyNode::Parent());
  }

  const NodeType& Unalias() const {
    return static_cast<const NodeType&>(PaintPropertyNode::Unalias());
  }

  const NodeType* UnaliasedParent() const {
    return static_cast<const NodeType*>(PaintPropertyNode::UnaliasedParent());
  }

  bool IsAncestorOf(const NodeTypeOrAlias& other) const {
    return PaintPropertyNode::IsAncestorOf(other);
  }

  const NodeTypeOrAlias& LowestCommonAncestor(
      const NodeTypeOrAlias& other) const {
    return static_cast<const NodeTypeOrAlias&>(
        PaintPropertyNode::LowestCommonAncestor(other));
  }

 protected:
  using PaintPropertyNode::PaintPropertyNode;
};

#if DCHECK_IS_ON()

class PLATFORM_EXPORT PropertyTreePrinter {
  STACK_ALLOCATED();

 public:
  void AddNode(const PaintPropertyNode* node);
  String NodesAsTreeString();
  String PathAsString(const PaintPropertyNode& last_node);

 private:
  void BuildTreeString(StringBuilder& string_builder,
                       const PaintPropertyNode& node,
                       unsigned indent);
  const PaintPropertyNode& RootNode();

  HeapLinkedHashSet<Member<const PaintPropertyNode>> nodes_;
};

#endif  // DCHECK_IS_ON()

inline std::ostream& operator<<(std::ostream& os,
                                const PaintPropertyNode& node) {
  return os << node.ToString().Utf8();
}

inline std::ostream& operator<<(std::ostream& os,
                                PaintPropertyChangeType change) {
  return os << PaintPropertyChangeTypeToString(change);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_PROPERTY_NODE_H_
