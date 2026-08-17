/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

// A red-black tree, which is a form of a balanced binary tree. It
// supports efficient insertion, deletion and queries of comparable
// elements. The same element may be inserted multiple times. The
// algorithmic complexity of common operations is:
//
//   Insertion: O(lg(n))
//   Deletion:  O(lg(n))
//   Querying:  O(lg(n))
//
// The data type T that is stored in this red-black tree must be only
// Plain Old Data (POD), or bottom out into POD. It must _not_ rely on
// having its destructor called. This implementation internally
// allocates storage in large chunks and does not call the destructor
// on each object.
//
// Type T must supply a default constructor, a copy constructor, and
// the "<" and "==" operators.
//
// In debug mode, printing of the data contained in the tree is
// enabled. This requires the template specialization to be available:
//
//   template<> struct ValueToString<T> {
//       static String toString(const T& t);
//   };
//
// Note that when complex types are stored in this red/black tree, it
// is possible that single invocations of the "<" and "==" operators
// will be insufficient to describe the ordering of elements in the
// tree during queries. As a concrete example, consider the case where
// intervals are stored in the tree sorted by low endpoint. The "<"
// operator on the Interval class only compares the low endpoint, but
// the "==" operator takes into account the high endpoint as well.
// This makes the necessary logic for querying and deletion somewhat
// more complex. In order to properly handle such situations, the
// property "needsFullOrderingComparisons" must be set to true on
// the tree.
//
// This red-black tree is designed to be _augmented_; subclasses can
// add additional and summary information to each node to efficiently
// store and index more complex data structures. A concrete example is
// the IntervalTree, which extends each node with a summary statistic
// to efficiently store one-dimensional intervals.
//
// The design of this red-black tree comes from Cormen, Leiserson,
// and Rivest, _Introduction to Algorithms_, MIT Press, 1990.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_RED_BLACK_TREE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_RED_BLACK_TREE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/pod_free_list_arena.h"
#ifndef NDEBUG
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#endif

namespace WTF {

#ifndef NDEBUG
template <class T>
struct ValueToString;
#endif

enum UninitializedTreeEnum { kUninitializedTree };

template <class T>
class PODRedBlackTree {
  DISALLOW_NEW();

 public:
  class Node;

  // Visitor interface for walking all of the tree's elements.
  class Visitor {
   public:
    virtual void Visit(const T& data) = 0;

   protected:
    virtual ~Visitor() = default;
  };

  // Constructs a new red-black tree without allocating an arena.
  // isInitialized will return false in this case. initIfNeeded can be used
  // to init the structure. This constructor is usefull for creating
  // lazy initialized tree.
  explicit PODRedBlackTree(UninitializedTreeEnum)
      : root_(nullptr),
        needs_full_ordering_comparisons_(false)
#ifndef NDEBUG
        ,
        verbose_debugging_(false)
#endif
  {
  }

  // Constructs a new red-black tree, allocating temporary objects
  // from a newly constructed PODFreeListArena.
  PODRedBlackTree()
      : arena_(PODFreeListArena<Node>::Create()),
        root_(nullptr),
        needs_full_ordering_comparisons_(false)
#ifndef NDEBUG
        ,
        verbose_debugging_(false)
#endif
  {
  }

  // Constructs a new red-black tree, allocating temporary objects
  // from the given PODArena.
  explicit PODRedBlackTree(scoped_refptr<PODFreeListArena<Node>> arena)
      : arena_(std::move(arena)),
        root_(nullptr),
        needs_full_ordering_comparisons_(false)
#ifndef NDEBUG
        ,
        verbose_debugging_(false)
#endif
  {
  }

  virtual ~PODRedBlackTree() = default;

  // Clearing will delete the contents of the tree. After this call
  // isInitialized will return false.
  void Clear() {
    MarkFree(root_);
    arena_ = nullptr;
    root_ = nullptr;
  }

  bool IsInitialized() const { return arena_.get(); }

  void InitIfNeeded() {
    if (!arena_)
      arena_ = PODFreeListArena<Node>::Create();
  }

  void InitIfNeeded(PODFreeListArena<Node>* arena) {
    if (!arena_)
      arena_ = arena;
  }

  void Add(const T& data) {
    DCHECK(IsInitialized());
    Node* node = arena_->template AllocateObject<T>(data);
    InsertNode(node);
  }

  // Returns true if the datum was found in the tree.
  bool Remove(const T& data) {
    DCHECK(IsInitialized());
    Node* node = TreeSearch(data);
    if (node) {
      DeleteNode(node);
      return true;
    }
    return false;
  }

  bool Contains(const T& data) const {
    DCHECK(IsInitialized());
    return TreeSearch(data);
  }

  void VisitInorder(Visitor* visitor) const {
    DCHECK(IsInitialized());
    if (!root_)
      return;
    VisitInorderImpl(root_, visitor);
  }

  int size() const {
    DCHECK(IsInitialized());
    Counter counter;
    VisitInorder(&counter);
    return counter.Count();
  }

  // See the class documentation for an explanation of this property.
  void SetNeedsFullOrderingComparisons(bool needs_full_ordering_comparisons) {
    needs_full_ordering_comparisons_ = needs_full_ordering_comparisons;
  }

  virtual bool CheckInvariants() const {
    DCHECK(IsInitialized());
    int black_count;
    return CheckInvariantsFromNode(root_, &black_count);
  }

#ifndef NDEBUG
  // Dumps the tree's contents to the logging info stream for
  // debugging purposes.
  void Dump() const {
    if (arena_)
      DumpFromNode(root_, 0);
  }

  // Turns on or off verbose debugging of the tree, causing many
  // messages to be logged during insertion and other operations in
  // debug mode.
  void SetVerboseDebugging(bool verbose_debugging) {
    verbose_debugging_ = verbose_debugging;
  }
#endif

  enum NodeColor { kRed = 1, kBlack };

  // The base Node class which is stored in the tree. Nodes are only
  // an internal concept; users of the tree deal only with the data
  // they store in it.
  class Node {
    DISALLOW_NEW();

   public:
    // Constructor. Newly-created nodes are colored red.
    explicit Node(const T& data)
        : left_(nullptr),
          right_(nullptr),
          parent_(nullptr),
          color_(kRed),
          data_(data) {}

    Node(const Node&) = delete;
    Node& operator=(const Node&) = delete;

    virtual ~Node() = default;

    NodeColor GetColor() const { return color_; }
    void SetColor(NodeColor color) { color_ = color; }

    // Fetches the user data.
    T& Data() { return data_; }
    T const& Data() const { return data_; }

    // Copies all user-level fields from the source node, but not
    // internal fields. For example, the base implementation of this
    // method copies the "m_data" field, but not the child or parent
    // fields. Any augmentation information also does not need to be
    // copied, as it will be recomputed. Subclasses must call the
    // superclass implementation.
    virtual void CopyFrom(Node* src) { data_ = src->Data(); }

    Node* Left() { return left_; }
    Node const* Left() const { return left_; }
    void SetLeft(Node* node) { left_ = node; }

    Node const* Right() const { return right_; }
    Node* Right() { return right_; }
    void SetRight(Node* node) { right_ = node; }

    Node const* Parent() const { return parent_; }
    Node* Parent() { return parent_; }
    void SetParent(Node* node) { parent_ = node; }

   private:
    Node* left_;
    Node* right_;
    Node* parent_;
    NodeColor color_;
    T data_;
  };

 protected:
  // Returns the root of the tree, which is needed by some subclasses.
  Node* Root() const { return root_; }

 private:
  // This virtual method is the hook that subclasses should use when
  // augmenting the red-black tree with additional per-node summary
  // information. For example, in the case of an interval tree, this
  // is used to compute the maximum endpoint of the subtree below the
  // given node based on the values in the left and right children. It
  // is guaranteed that this will be called in the correct order to
  // properly update such summary information based only on the values
  // in the left and right children. This method should return true if
  // the node's summary information changed.
  virtual bool UpdateNode(Node*) { return false; }

  //----------------------------------------------------------------------
  // Generic binary search tree operations
  //

  // Searches the tree for the given datum.
  Node* TreeSearch(const T& data) const {
    if (needs_full_ordering_comparisons_)
      return TreeSearchFullComparisons(root_, data);

    return TreeSearchNormal(root_, data);
  }

  // Searches the tree using the normal comparison operations,
  // suitable for simple data types such as numbers.
  Node* TreeSearchNormal(Node* current, const T& data) const {
    while (current) {
      if (current->Data() == data)
        return current;
      if (data < current->Data())
        current = current->Left();
      else
        current = current->Right();
    }
    return nullptr;
  }

  // Searches the tree using multiple comparison operations, required
  // for data types with more complex behavior such as intervals.
  Node* TreeSearchFullComparisons(Node* current, const T& data) const {
    if (!current)
      return nullptr;
    if (data < current->Data())
      return TreeSearchFullComparisons(current->Left(), data);
    if (current->Data() < data)
      return TreeSearchFullComparisons(current->Right(), data);
    if (data == current->Data())
      return current;

    // We may need to traverse both the left and right subtrees.
    Node* result = TreeSearchFullComparisons(current->Left(), data);
    if (!result)
      result = TreeSearchFullComparisons(current->Right(), data);
    return result;
  }

  void TreeInsert(Node* z) {
    Node* y = nullptr;
    Node* x = root_;
    while (x) {
      y = x;
      if (z->Data() < x->Data())
        x = x->Left();
      else
        x = x->Right();
    }
    z->SetParent(y);
    if (!y) {
      root_ = z;
    } else {
      if (z->Data() < y->Data())
        y->SetLeft(z);
      else
        y->SetRight(z);
    }
  }

  // Finds the node following the given one in sequential ordering of
  // their data, or null if none exists.
  Node* TreeSuccessor(Node* x) {
    if (x->Right())
      return TreeMinimum(x->Right());
    Node* y = x->Parent();
    while (y && x == y->Right()) {
      x = y;
      y = y->Parent();
    }
    return y;
  }

  // Finds the minimum element in the sub-tree rooted at the given
  // node.
  Node* TreeMinimum(Node* x) {
    while (x->Left())
      x = x->Left();
    return x;
  }

  // Helper for maintaining the augmented red-black tree.
  void PropagateUpdates(Node* start) {
    bool should_continue = true;
    while (start && should_continue) {
      should_continue = UpdateNode(start);
      start = start->Parent();
    }
  }

  //----------------------------------------------------------------------
  // Red-Black tree operations
  //

  // Left-rotates the subtree rooted at x.
  // Returns the new root of the subtree (x's right child).
  Node* LeftRotate(Node* x) {
    // Set y.
    Node* y = x->Right();

    // Turn y's left subtree into x's right subtree.
    x->SetRight(y->Left());
    if (y->Left())
      y->Left()->SetParent(x);

    // Link x's parent to y.
    y->SetParent(x->Parent());
    if (!x->Parent()) {
      root_ = y;
    } else {
      if (x == x->Parent()->Left())
        x->Parent()->SetLeft(y);
      else
        x->Parent()->SetRight(y);
    }

    // Put x on y's left.
    y->SetLeft(x);
    x->SetParent(y);

    // Update nodes lowest to highest.
    UpdateNode(x);
    UpdateNode(y);
    return y;
  }

  // Right-rotates the subtree rooted at y.
  // Returns the new root of the subtree (y's left child).
  Node* RightRotate(Node* y) {
    // Set x.
    Node* x = y->Left();

    // Turn x's right subtree into y's left subtree.
    y->SetLeft(x->Right());
    if (x->Right())
      x->Right()->SetParent(y);

    // Link y's parent to x.
    x->SetParent(y->Parent());
    if (!y->Parent()) {
      root_ = x;
    } else {
      if (y == y->Parent()->Left())
        y->Parent()->SetLeft(x);
      else
        y->Parent()->SetRight(x);
    }

    // Put y on x's right.
    x->SetRight(y);
    y->SetParent(x);

    // Update nodes lowest to highest.
    UpdateNode(y);
    UpdateNode(x);
    return x;
  }

  // Inserts the given node into the tree.
  void InsertNode(Node* x) {
    TreeInsert(x);
    x->SetColor(kRed);
    UpdateNode(x);

    LogIfVerbose("  PODRedBlackTree::InsertNode");

    // The node from which to start propagating updates upwards.
    Node* update_start = x->Parent();

    while (x != root_ && x->Parent()->GetColor() == kRed) {
      if (x->Parent() == x->Parent()->Parent()->Left()) {
        Node* y = x->Parent()->Parent()->Right();
        if (y && y->GetColor() == kRed) {
          // Case 1
          LogIfVerbose("  Case 1/1");
          x->Parent()->SetColor(kBlack);
          y->SetColor(kBlack);
          x->Parent()->Parent()->SetColor(kRed);
          UpdateNode(x->Parent());
          x = x->Parent()->Parent();
          UpdateNode(x);
          update_start = x->Parent();
        } else {
          if (x == x->Parent()->Right()) {
            LogIfVerbose("  Case 1/2");
            // Case 2
            x = x->Parent();
            LeftRotate(x);
          }
          // Case 3
          LogIfVerbose("  Case 1/3");
          x->Parent()->SetColor(kBlack);
          x->Parent()->Parent()->SetColor(kRed);
          Node* new_sub_tree_root = RightRotate(x->Parent()->Parent());
          update_start = new_sub_tree_root->Parent();
        }
      } else {
        // Same as "then" clause with "right" and "left" exchanged.
        Node* y = x->Parent()->Parent()->Left();
        if (y && y->GetColor() == kRed) {
          // Case 1
          LogIfVerbose("  Case 2/1");
          x->Parent()->SetColor(kBlack);
          y->SetColor(kBlack);
          x->Parent()->Parent()->SetColor(kRed);
          UpdateNode(x->Parent());
          x = x->Parent()->Parent();
          UpdateNode(x);
          update_start = x->Parent();
        } else {
          if (x == x->Parent()->Left()) {
            // Case 2
            LogIfVerbose("  Case 2/2");
            x = x->Parent();
            RightRotate(x);
          }
          // Case 3
          LogIfVerbose("  Case 2/3");
          x->Parent()->SetColor(kBlack);
          x->Parent()->Parent()->SetColor(kRed);
          Node* new_sub_tree_root = LeftRotate(x->Parent()->Parent());
          update_start = new_sub_tree_root->Parent();
        }
      }
    }

    PropagateUpdates(update_start);

    root_->SetColor(kBlack);
  }

  // Restores the red-black property to the tree after splicing out
  // a node. Note that x may be null, which is why xParent must be
  // supplied.
  void DeleteFixup(Node* x, Node* x_parent) {
    while (x != root_ && (!x || x->GetColor() == kBlack)) {
      if (x == x_parent->Left()) {
        // Note: the text points out that w can not be null.
        // The reason is not obvious from simply looking at
        // the code; it comes about from the properties of the
        // red-black tree.
        Node* w = x_parent->Right();
        DCHECK(w);  // x's sibling should not be null.
        if (w->GetColor() == kRed) {
          // Case 1
          w->SetColor(kBlack);
          x_parent->SetColor(kRed);
          LeftRotate(x_parent);
          w = x_parent->Right();
        }
        if ((!w->Left() || w->Left()->GetColor() == kBlack) &&
            (!w->Right() || w->Right()->GetColor() == kBlack)) {
          // Case 2
          w->SetColor(kRed);
          x = x_parent;
          x_parent = x->Parent();
        } else {
          if (!w->Right() || w->Right()->GetColor() == kBlack) {
            // Case 3
            w->Left()->SetColor(kBlack);
            w->SetColor(kRed);
            RightRotate(w);
            w = x_parent->Right();
          }
          // Case 4
          w->SetColor(x_parent->GetColor());
          x_parent->SetColor(kBlack);
          if (w->Right())
            w->Right()->SetColor(kBlack);
          LeftRotate(x_parent);
          x = root_;
          x_parent = x->Parent();
        }
      } else {
        // Same as "then" clause with "right" and "left"
        // exchanged.

        // Note: the text points out that w can not be null.
        // The reason is not obvious from simply looking at
        // the code; it comes about from the properties of the
        // red-black tree.
        Node* w = x_parent->Left();
        DCHECK(w);  // x's sibling should not be null.
        if (w->GetColor() == kRed) {
          // Case 1
          w->SetColor(kBlack);
          x_parent->SetColor(kRed);
          RightRotate(x_parent);
          w = x_parent->Left();
        }
        if ((!w->Right() || w->Right()->GetColor() == kBlack) &&
            (!w->Left() || w->Left()->GetColor() == kBlack)) {
          // Case 2
          w->SetColor(kRed);
          x = x_parent;
          x_parent = x->Parent();
        } else {
          if (!w->Left() || w->Left()->GetColor() == kBlack) {
            // Case 3
            w->Right()->SetColor(kBlack);
            w->SetColor(kRed);
            LeftRotate(w);
            w = x_parent->Left();
          }
          // Case 4
          w->SetColor(x_parent->GetColor());
          x_parent->SetColor(kBlack);
          if (w->Left())
            w->Left()->SetColor(kBlack);
          RightRotate(x_parent);
          x = root_;
          x_parent = x->Parent();
        }
      }
    }
    if (x)
      x->SetColor(kBlack);
  }

  // Deletes the given node from the tree. Note that this
  // particular node may not actually be removed from the tree;
  // instead, another node might be removed and its contents
  // copied into z.
  void DeleteNode(Node* z) {
    // Y is the node to be unlinked from the tree.
    Node* y;
    if (!z->Left() || !z->Right())
      y = z;
    else
      y = TreeSuccessor(z);

    // Y is guaranteed to be non-null at this point.
    Node* x;
    if (y->Left())
      x = y->Left();
    else
      x = y->Right();

    // X is the child of y which might potentially replace y in
    // the tree. X might be null at this point.
    Node* x_parent;
    if (x) {
      x->SetParent(y->Parent());
      x_parent = x->Parent();
    } else {
      x_parent = y->Parent();
    }
    if (!y->Parent()) {
      root_ = x;
    } else {
      if (y == y->Parent()->Left())
        y->Parent()->SetLeft(x);
      else
        y->Parent()->SetRight(x);
    }
    if (y != z) {
      z->CopyFrom(y);
      // This node has changed location in the tree and must be updated.
      UpdateNode(z);
      // The parent and its parents may now be out of date.
      PropagateUpdates(z->Parent());
    }

    // If we haven't already updated starting from xParent, do so now.
    if (x_parent && x_parent != y && x_parent != z)
      PropagateUpdates(x_parent);
    if (y->GetColor() == kBlack)
      DeleteFixup(x, x_parent);

    arena_->FreeObject(y);
  }

  // Visits the subtree rooted at the given node in order.
  void VisitInorderImpl(Node* node, Visitor* visitor) const {
    if (node->Left())
      VisitInorderImpl(node->Left(), visitor);
    visitor->Visit(node->Data());
    if (node->Right())
      VisitInorderImpl(node->Right(), visitor);
  }

  void MarkFree(Node* node) {
    if (!node)
      return;

    if (node->Left())
      MarkFree(node->Left());
    if (node->Right())
      MarkFree(node->Right());
    arena_->FreeObject(node);
  }

  //----------------------------------------------------------------------
  // Helper class for size()

  // A Visitor which simply counts the number of visited elements.
  class Counter final : public Visitor {
    DISALLOW_NEW();

   public:
    Counter() : count_(0) {}
    Counter(const Counter&) = delete;
    Counter& operator=(const Counter&) = delete;

    void Visit(const T&) override { ++count_; }
    int Count() const { return count_; }

   private:
    int count_;
  };

  //----------------------------------------------------------------------
  // Verification and debugging routines
  //

  // Returns in the "blackCount" parameter the number of black
  // children along all paths to all leaves of the given node.
  bool CheckInvariantsFromNode(Node* node, int* black_count) const {
    // Base case is a leaf node.
    if (!node) {
      *black_count = 1;
      return true;
    }

    // Each node is either red or black.
    if (!(node->GetColor() == kRed || node->GetColor() == kBlack))
      return false;

    // Every leaf (or null) is black.

    if (node->GetColor() == kRed) {
      // Both of its children are black.
      if (!((!node->Left() || node->Left()->GetColor() == kBlack)))
        return false;
      if (!((!node->Right() || node->Right()->GetColor() == kBlack)))
        return false;
    }

    // Every simple path to a leaf node contains the same number of
    // black nodes.
    int left_count = 0, right_count = 0;
    bool left_valid = CheckInvariantsFromNode(node->Left(), &left_count);
    bool right_valid = CheckInvariantsFromNode(node->Right(), &right_count);
    if (!left_valid || !right_valid)
      return false;
    *black_count = left_count + (node->GetColor() == kBlack ? 1 : 0);
    return left_count == right_count;
  }

#ifdef NDEBUG
  void LogIfVerbose(const char*) const {}
#else
  void LogIfVerbose(const char* output) const {
    if (verbose_debugging_)
      DLOG(ERROR) << output;
  }
#endif

#ifndef NDEBUG
  // Dumps the subtree rooted at the given node.
  void DumpFromNode(Node* node, int indentation) const {
    StringBuilder builder;
    for (int i = 0; i < indentation; i++)
      builder.Append(' ');
    builder.Append('-');
    if (node) {
      builder.Append(' ');
      builder.Append(ValueToString<T>::GetString(node->Data()));
      builder.Append((node->GetColor() == kBlack) ? " (black)" : " (red)");
    }
    DLOG(ERROR) << builder.ToString();
    if (node) {
      DumpFromNode(node->Left(), indentation + 2);
      DumpFromNode(node->Right(), indentation + 2);
    }
  }
#endif

  //----------------------------------------------------------------------
  // Data members

  scoped_refptr<PODFreeListArena<Node>> arena_;
  Node* root_;
  bool needs_full_ordering_comparisons_;
#ifndef NDEBUG
  bool verbose_debugging_;
#endif
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_RED_BLACK_TREE_H_
