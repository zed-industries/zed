// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_TRAVERSAL_ROOT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_TRAVERSAL_ROOT_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/container_node.h"

namespace blink {

// Class used to represent a common ancestor for all dirty nodes in a DOM tree.
// Subclasses implement the various types of dirtiness for style recalc, style
// invalidation, and layout tree rebuild. The common ancestor is used as a
// starting point for traversal to avoid unnecessary DOM tree traversal.
//
// The first dirty node is stored as a single root. When a second node is
// added with a common child-dirty ancestor which is not dirty, we store that
// as a common root. Any subsequent dirty nodes added whose closest child-dirty
// ancestor is not itself dirty, or is the current root, will cause us to fall
// back to use the document as the root node. In order to find a lowest common
// ancestor we would have had to traverse up the ancestor chain to see if we are
// below the current common root or not.
//
// Note that when the common ancestor candidate passed into Update is itself
// dirty, we know that we are currently below the current root node and don't
// have to modify it.

class CORE_EXPORT StyleTraversalRoot {
  DISALLOW_NEW();

 public:
  // Update the common ancestor root when dirty_node is marked dirty. The
  // common_ancestor is the closest ancestor of dirty_node which was already
  // marked as having dirty children.
  void Update(ContainerNode* common_ancestor, Node* dirty_node);

  // Update the root node if the current has been removed from the tree.
  // The 'tree' here may refer to the flat tree if marking ancestors happen in
  // the flat for the given subclass.
  virtual void SubtreeModified(ContainerNode& parent) = 0;

  Node* GetRootNode() const { return root_node_.Get(); }
  void Clear() {
    root_node_ = nullptr;
    root_type_ = RootType::kSingleRoot;
  }

  void Trace(Visitor* visitor) const { visitor->Trace(root_node_); }

 protected:
  virtual ~StyleTraversalRoot() = default;

#if DCHECK_IS_ON()
  // Return the parent node for type of traversal for which the implementation
  // is a root.
  virtual ContainerNode* Parent(const Node&) const = 0;

  // Return true if the given node is marked dirty or child-dirty.
  virtual bool IsChildDirty(const Node&) const = 0;
#endif  // DCHECK_IS_ON()

  // Return true if the given node is dirty.
  virtual bool IsDirty(const Node&) const = 0;

  bool IsSingleRoot() const { return root_type_ == RootType::kSingleRoot; }

 private:
  friend class StyleTraversalRootTestImpl;

#if DCHECK_IS_ON()
  bool IsModifyingFlatTree() const;
#endif

  void AssertRootNodeInvariants() {
#if DCHECK_IS_ON()
    DCHECK(!root_node_ || root_node_->IsDocumentNode() ||
           IsDirty(*root_node_) || IsChildDirty(*root_node_) ||
           IsModifyingFlatTree());
#endif
  }

  // The current root for dirty nodes.
  Member<Node> root_node_;

  // Is the current root a common ancestor or a single dirty node.
  enum class RootType { kSingleRoot, kCommonRoot };
  RootType root_type_ = RootType::kSingleRoot;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_TRAVERSAL_ROOT_H_
