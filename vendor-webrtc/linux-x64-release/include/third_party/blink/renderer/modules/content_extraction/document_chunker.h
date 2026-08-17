// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_DOCUMENT_CHUNKER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_DOCUMENT_CHUNKER_H_

#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class HTMLIFrameElement;

// Returns true if the content of `iframe_element` should be included for
// inner text or document passages.
bool ShouldContentExtractionIncludeIFrame(const HTMLIFrameElement& iframe_element);

// Chunks documents into text passages. Each passage contains either a single
// node of text, or the text of the node and its siblings and descendants if the
// total number of words is less than max_words_per_aggregate_passage. This is
// done by recursively walking the document tree, gathering the content of
// individual text nodes ("segments") and then aggregating these into longer
// strings ("passages"), each containing whitespace-joined segments from zero or
// more siblings and descendants.
class DocumentChunker {
 public:
  // Parameters:
  // max_words_per_aggregate_passage: Maximum number of words in a passage
  //  comprised of multiple nodes. A passage with text from only a single
  //  node may exceed this max.
  // greedily_aggregate_sibling_nodes: If true, sibling nodes are greedily
  //  aggregated into passages under max_words_per_aggregate_passage words. If
  //  false, each sibling node is output as a separate passage if they cannot
  //  all be combined into a single passage under
  //  max_words_per_aggregate_passage words.
  DocumentChunker(size_t max_words_per_aggregate_passage,
                  bool greedily_aggregate_sibling_nodes,
                  uint32_t max_passages,
                  uint32_t min_words_per_passage);

  // Chunks the node and its descendants into text passages.
  // Returns a vector of text passages.
  Vector<String> Chunk(const Node& tree);

 private:
  struct AggregateNode;

  // A list of finished aggregations of text segments, built from the leaves up.
  struct PassageList {
    // Creates and adds a text passage for the input node, if it is non-empty,
    // and contains more words than the given minimum.
    void AddPassageForNode(const AggregateNode& node,
                           size_t min_words_per_passage);

    // Extends this PassageList from another given |passage_list|.
    void Extend(const PassageList& passage_list);

    // Passages are completed aggregations of text segments. It is possible
    // for a single passage to exceed max_words_per_aggregate_passage but the
    // aggregation process tries to avoid it. This has an inline capacity of
    // 32 to avoid excessive per-node reallocations during the recursive walk.
    Vector<String, 32> passages;
  };

  // Contains aggregate information about a node and its descendants.
  struct AggregateNode {
    // Returns true if |node| can be added without exceeding |max_words|.
    bool Fits(const AggregateNode& node, size_t max_words);

    // Adds the input node to this AggregateNode.
    void AddNode(const AggregateNode& node);

    // Returns a text passage built from joined |segments|.
    String CreatePassage() const;

    // Segments of text that are part of this AggregateNode.
    // These are accumulated as work in progress toward creating full passages.
    Vector<String, 32> segments;

    // Total number of words in |segments|.
    size_t num_words = 0;

    // Completed passages for this node and its descendants.
    PassageList passage_list;
  };

  // Recursively processes a node and its descendants, returning early if
  // a maximum |depth| is reached.
  AggregateNode ProcessNode(const Node& node,
                            int depth,
                            uint32_t passage_count);

  size_t max_words_per_aggregate_passage_;
  bool greedily_aggregate_sibling_nodes_;
  uint32_t max_passages_;
  uint32_t min_words_per_passage_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_DOCUMENT_CHUNKER_H_
