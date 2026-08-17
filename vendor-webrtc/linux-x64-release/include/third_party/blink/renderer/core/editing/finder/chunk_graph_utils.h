// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_CHUNK_GRAPH_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_CHUNK_GRAPH_UTILS_H_

#include <tuple>

#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Node;
class Text;

struct TextOrChar {
  DISALLOW_NEW();

  Member<const Text> text;
  // This field is valid if `text` is nullptr.
  // The type is UChar, not UChar32 because it stores a return value of
  // CharConstantForNode(), and is appended to Vector<UChar>.
  UChar code_point = 0;

  TextOrChar(const Text* arg_text, UChar arg_code_point)
      : text(arg_text), code_point(arg_code_point) {}
  void Trace(Visitor* visitor) const;
};

// CorpusChunk class represents a sequence of text split by <ruby> and <rt>.
// It's an intermediate data to build corpus strings in FindBuffer.
//
// e.g.  <p>foo <span>bar</span> <ruby>base<rt>anno</ruby> baz</p>
//       This IFC contains four CorpusChunks:
//         1. "foo bar ", level=*, linking to 2 and 3
//         2. "base", level=0, linking to 4
//         3. "anno", level=1, linking to 4
//         4. " baz", level=*
class CorpusChunk : public GarbageCollected<CorpusChunk> {
 public:
  CorpusChunk();
  CorpusChunk(const HeapVector<TextOrChar>& text_list, const String& level);
  void Trace(Visitor* visitor) const;

  void Link(CorpusChunk* next_chunk);

  const HeapVector<TextOrChar>& TextList() const { return text_list_; }
  const CorpusChunk* FindNext(const String& level) const;

 private:
  HeapVector<TextOrChar> text_list_;
  // Annotation level for this chunk. If this chunk is for the inner-most
  // annotation of another inner-most annotation, this field is "1,1".
  const String level_;
  HeapVector<Member<CorpusChunk>, 1> next_list_;
};

std::tuple<HeapVector<Member<CorpusChunk>>, Vector<String>, const Node*>
BuildChunkGraph(const Node& first_visible_text_node,
                const Node* end_node,
                const Node& block_ancestor,
                const Node* just_after_block);

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::TextOrChar)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_CHUNK_GRAPH_UTILS_H_
