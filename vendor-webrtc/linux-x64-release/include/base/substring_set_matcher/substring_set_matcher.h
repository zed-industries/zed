// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/40284755): Remove this and spanify to fix the errors.
#pragma allow_unsafe_buffers
#endif

#ifndef BASE_SUBSTRING_SET_MATCHER_SUBSTRING_SET_MATCHER_H_
#define BASE_SUBSTRING_SET_MATCHER_SUBSTRING_SET_MATCHER_H_

#include <stdint.h>

#include <limits>
#include <set>
#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/check_op.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/substring_set_matcher/matcher_string_pattern.h"

namespace base {

// Class that store a set of string patterns and can find for a string S,
// which string patterns occur in S.
class BASE_EXPORT SubstringSetMatcher {
 public:
  SubstringSetMatcher();
  SubstringSetMatcher(const SubstringSetMatcher&) = delete;
  SubstringSetMatcher& operator=(const SubstringSetMatcher&) = delete;

  ~SubstringSetMatcher();

  // Registers all |patterns|. Each pattern needs to have a unique ID and all
  // pattern strings must be unique. Build() should be called exactly once
  // (before it is called, the tree is empty).
  //
  // Complexity:
  //    Let n = number of patterns.
  //    Let S = sum of pattern lengths.
  //    Let k = range of char. Generally 256.
  // Complexity = O(nlogn + S * logk)
  // nlogn comes from sorting the patterns.
  // log(k) comes from our usage of std::map to store edges.
  //
  // Returns true on success (may fail if e.g. if the tree gets too many nodes).
  bool Build(const std::vector<MatcherStringPattern>& patterns);
  bool Build(std::vector<const MatcherStringPattern*> patterns);

  // Matches |text| against all registered MatcherStringPatterns. Stores the IDs
  // of matching patterns in |matches|. |matches| is not cleared before adding
  // to it.
  // Complexity:
  //    Let t = length of |text|.
  //    Let k = range of char. Generally 256.
  //    Let z = number of matches returned.
  // Complexity = O(t * logk + zlogz)
  bool Match(const std::string& text,
             std::set<MatcherStringPattern::ID>* matches) const;

  // As Match(), except it returns immediately on the first match.
  // This allows true/false matching to be done without any dynamic
  // memory allocation.
  // Complexity = O(t * logk)
  bool AnyMatch(const std::string& text) const;

  // Returns true if this object retains no allocated data.
  bool IsEmpty() const { return is_empty_; }

  // Returns the dynamically allocated memory usage in bytes. See
  // base/trace_event/memory_usage_estimator.h for details.
  size_t EstimateMemoryUsage() const;

 private:
  // Represents the index of the node within |tree_|. It is specifically
  // uint32_t so that we can be sure it takes up 4 bytes when stored together
  // with the 9-bit label (so 23 bits are allocated to the NodeID, even though
  // it is exposed as uint32_t). If the computed size of |tree_| is
  // larger than what can be stored within 23 bits, Build() will fail.
  using NodeID = uint32_t;

  // This is the maximum possible size of |tree_| and hence can't be a valid ID.
  static constexpr NodeID kInvalidNodeID = (1u << 23) - 1;

  static constexpr NodeID kRootID = 0;

  // A node of an Aho Corasick Tree. See
  // http://web.stanford.edu/class/archive/cs/cs166/cs166.1166/lectures/02/Small02.pdf
  // to understand the algorithm.
  //
  // The algorithm is based on the idea of building a trie of all registered
  // patterns. Each node of the tree is annotated with a set of pattern
  // IDs that are used to report matches.
  //
  // The root of the trie represents an empty match. If we were looking whether
  // any registered pattern matches a text at the beginning of the text (i.e.
  // whether any pattern is a prefix of the text), we could just follow
  // nodes in the trie according to the matching characters in the text.
  // E.g., if text == "foobar", we would follow the trie from the root node
  // to its child labeled 'f', from there to child 'o', etc. In this process we
  // would report all pattern IDs associated with the trie nodes as matches.
  //
  // As we are not looking for all prefix matches but all substring matches,
  // this algorithm would need to compare text.substr(0), text.substr(1), ...
  // against the trie, which is in O(|text|^2).
  //
  // The Aho Corasick algorithm improves this runtime by using failure edges.
  // In case we have found a partial match of length k in the text
  // (text[i, ..., i + k - 1]) in the trie starting at the root and ending at
  // a node at depth k, but cannot find a match in the trie for character
  // text[i + k] at depth k + 1, we follow a failure edge. This edge
  // corresponds to the longest proper suffix of text[i, ..., i + k - 1] that
  // is a prefix of any registered pattern.
  //
  // If your brain thinks "Forget it, let's go shopping.", don't worry.
  // Take a nap and read an introductory text on the Aho Corasick algorithm.
  // It will make sense. Eventually.

  // An edge internal to the tree. We pack the label (character we are
  // matching on) and the destination node ID into 32 bits, to save memory.
  // We also use these edges as a sort of generic key/value store for
  // some special values that not all nodes will have; this also saves on
  // memory over the otherwise obvious choice of having them as struct fields,
  // as it means we do not to store them when they are not present.
  struct AhoCorasickEdge {
    // char (unsigned, so [0..255]), or a special label below.
    uint32_t label : 9;
    NodeID node_id : 23;
  };

  // Node index that failure edge leads to. The failure node corresponds to
  // the node which represents the longest proper suffix (include empty
  // string) of the string represented by this node. Not stored if it is
  // equal to kRootID (since that is the most common value).
  //
  // NOTE: Assigning |root| as the failure edge for itself doesn't strictly
  // abide by the definition of "proper" suffix. The proper suffix of an empty
  // string should probably be defined as null, but we assign it to the |root|
  // to simplify the code and have the invariant that the failure edge is always
  // defined.
  static constexpr uint32_t kFailureNodeLabel = 0x100;
  static constexpr uint32_t kFirstSpecialLabel = kFailureNodeLabel;

  // Node index that corresponds to the longest proper suffix (including empty
  // suffix) of this node and which also represents the end of a pattern.
  // Does not have to exist.
  static constexpr uint32_t kOutputLinkLabel = 0x101;

  // If present, this node represents the end of a pattern. It stores the ID of
  // the corresponding pattern (ie., it is not really a NodeID, but a
  // MatcherStringPattern::ID).
  static constexpr uint32_t kMatchIDLabel = 0x102;

  // Used for uninitialized label slots; used so that we do not have to test for
  // them in other ways, since we know the data will be initialized and never
  // match any other labels.
  static constexpr uint32_t kEmptyLabel = 0x103;

  // A node in the trie, packed tightly together so that it occupies 12 bytes
  // (both on 32- and 64-bit platforms), but aligned to at least 4 (see the
  // comment on edges_).
  class alignas(AhoCorasickEdge) AhoCorasickNode {
   public:
    AhoCorasickNode();
    ~AhoCorasickNode();
    AhoCorasickNode(AhoCorasickNode&& other);
    AhoCorasickNode& operator=(AhoCorasickNode&& other);

    NodeID GetEdge(uint32_t label) const {
      if (edges_capacity_ != 0) {
        return GetEdgeNoInline(label);
      }
      static_assert(kNumInlineEdges == 2, "Code below needs updating");
      if (edges_.inline_edges[0].label == label) {
        return edges_.inline_edges[0].node_id;
      }
      if (edges_.inline_edges[1].label == label) {
        return edges_.inline_edges[1].node_id;
      }
      return kInvalidNodeID;
    }
    NodeID GetEdgeNoInline(uint32_t label) const;
    void SetEdge(uint32_t label, NodeID node);
    const AhoCorasickEdge* edges() const {
      // NOTE: Returning edges_.inline_edges here is fine, because it's
      // the first thing in the struct (see the comment on edges_).
      DCHECK_EQ(0u, reinterpret_cast<uintptr_t>(edges_.inline_edges) %
                        alignof(AhoCorasickEdge));
      return edges_capacity_ == 0 ? edges_.inline_edges : edges_.edges;
    }

    NodeID failure() const {
      // NOTE: Even if num_edges_ == 0, we are not doing anything
      // undefined, as we will have room for at least two edges
      // and empty edges are set to kEmptyLabel.
      const AhoCorasickEdge& first_edge = *edges();
      if (first_edge.label == kFailureNodeLabel) {
        return first_edge.node_id;
      } else {
        return kRootID;
      }
    }
    void SetFailure(NodeID failure);

    void SetMatchID(MatcherStringPattern::ID id) {
      DCHECK(!IsEndOfPattern());
      DCHECK(id < kInvalidNodeID);  // This is enforced by Build().
      SetEdge(kMatchIDLabel, static_cast<NodeID>(id));
      has_outputs_ = true;
    }

    // Returns true if this node corresponds to a pattern.
    bool IsEndOfPattern() const {
      if (!has_outputs_) {
        // Fast reject.
        return false;
      }
      return GetEdge(kMatchIDLabel) != kInvalidNodeID;
    }

    // Must only be called if |IsEndOfPattern| returns true for this node.
    MatcherStringPattern::ID GetMatchID() const {
      DCHECK(IsEndOfPattern());
      return GetEdge(kMatchIDLabel);
    }

    void SetOutputLink(NodeID node) {
      if (node != kInvalidNodeID) {
        SetEdge(kOutputLinkLabel, node);
        has_outputs_ = true;
      }
    }
    NodeID output_link() const { return GetEdge(kOutputLinkLabel); }

    size_t EstimateMemoryUsage() const;
    size_t num_edges() const {
      if (edges_capacity_ == 0) {
        return kNumInlineEdges - num_free_edges_;
      } else {
        return edges_capacity_ - num_free_edges_;
      }
    }

    bool has_outputs() const { return has_outputs_; }

   private:
    // Outgoing edges of current node, including failure edge and output links.
    // Most nodes have only one or two (or even zero) edges, not the last
    // because many of them are leaves. Thus, we make an optimization for this
    // common case; instead of a pointer to an edge array on the heap, we can
    // pack two edges inline where the pointer would otherwise be. This reduces
    // memory usage dramatically, as well as saving us a cache-line fetch.
    //
    // Note that even though most nodes have fewer outgoing edges, most nodes
    // that we actually traverse will have any of them. This apparent
    // contradiction is because we tend to spend more of our time near the root
    // of the trie, where it is wide. This means that another layout would be
    // possible: If we wanted to, non-inline nodes could simply store an array
    // of 259 (256 possible characters plus the three special label types)
    // edges, indexed directly by label type. This would use 20–50% more RAM,
    // but also increases the speed of lookups due to removing the search loop.
    //
    // The nodes are generally unordered; since we typically index text, even
    // the root will rarely be more than 20–30 wide, and at that point, it's
    // better to just do a linear search than a binary one (which fares poorly
    // on branch predictors). However, a special case, we put kFailureNodeLabel
    // in the first slot if it exists (ie., is not equal to kRootID), since we
    // need to access that label during every single node we look at during
    // traversal.
    //
    // NOTE: Keep this the first member in the struct, so that inline_edges gets
    // 4-aligned (since the class is marked as such, despite being packed.
    // Otherwise, edges() can return an unaligned pointer marked as aligned
    // (the unalignedness gets lost).
    static constexpr int kNumInlineEdges = 2;
    union {
      // Out-of-line edge storage, having room for edges_capacity_ elements.
      // Note that due to __attribute__((packed)) below, this pointer may be
      // unaligned on 64-bit platforms, causing slightly less efficient
      // access to it in some cases.
      // This field is not a raw_ptr<> because it was filtered by the rewriter
      // for: #union
      RAW_PTR_EXCLUSION AhoCorasickEdge* edges;

      // Inline edge storage, used if edges_capacity_ == 0.
      AhoCorasickEdge inline_edges[kNumInlineEdges];
    } edges_;

    // Whether we have an edge for kMatchIDLabel or kOutputLinkLabel,
    // ie., hitting this node during traversal will create one or more
    // matches. This is redundant, but since every single lookup during
    // traversal needs this, it saves a few searches for us.
    bool has_outputs_ = false;

    // Number of unused left in edges_. Edges are always allocated from the
    // beginning and never deleted; those after num_edges_ will be marked with
    // kEmptyLabel (and have an undefined node_id). We store the number of
    // free edges instead of the more common number of _used_ edges, to be
    // sure that we are able to fit it in an uint8_t. num_edges() provides
    // a useful abstraction over this.
    uint8_t num_free_edges_ = kNumInlineEdges;

    // How many edges we have allocated room for (can never be more than
    // kEmptyLabel + 1). If equal to zero, we are not using heap storage,
    // but instead are using inline_edges.
    //
    // If not equal to zero, will be a multiple of 4, so that we can use
    // SIMD to accelerate looking for edges.
    uint16_t edges_capacity_ = 0;
  } __attribute__((packed));

  using SubstringPatternVector = std::vector<const MatcherStringPattern*>;

  // Given the set of patterns, compute how many nodes will the corresponding
  // Aho-Corasick tree have. Note that |patterns| need to be sorted.
  NodeID GetTreeSize(
      const std::vector<const MatcherStringPattern*>& patterns) const;

  void BuildAhoCorasickTree(const SubstringPatternVector& patterns);

  // Inserts a path for |pattern->pattern()| into the tree and adds
  // |pattern->id()| to the set of matches.
  void InsertPatternIntoAhoCorasickTree(const MatcherStringPattern* pattern);

  void CreateFailureAndOutputEdges();

  // Adds all pattern IDs to |matches| which are a suffix of the string
  // represented by |node|.
  void AccumulateMatchesForNode(
      const AhoCorasickNode* node,
      std::set<MatcherStringPattern::ID>* matches) const;

  // The nodes of a Aho-Corasick tree.
  std::vector<AhoCorasickNode> tree_;

  bool is_empty_ = true;
};

}  // namespace base

#endif  // BASE_SUBSTRING_SET_MATCHER_SUBSTRING_SET_MATCHER_H_
