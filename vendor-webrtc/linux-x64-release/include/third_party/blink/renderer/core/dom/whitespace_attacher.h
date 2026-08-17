// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_WHITESPACE_ATTACHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_WHITESPACE_ATTACHER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class Element;
class LayoutObject;
class Node;
class Text;

// The WhitespaceAttacher is used during the layout tree rebuild to lazily re-
// attach whitespace LayoutObjects when necessary. For more details about white-
// space LayoutObjects, see the WhitespaceLayoutObjects.md file in this
// directory.
//
// As RebuildLayoutTree walks from last to first child, we track the last text
// node, or the last skipped display:contents element we have seen. These are
// reset to null as soon as we encounter an in-flow element.
//
// If the tracked text node needed a (re-)attach, we call
// ReattachWhitespaceSiblings once we visit or re-attach the first preceding
// in-flow.
//
// If we re-attach a preceding in-flow, we also call ReattachWhitespaceSiblings
// since the need for a succeeding whitespace LayoutObject may change.

class CORE_EXPORT WhitespaceAttacher {
  STACK_ALLOCATED();

 public:
  WhitespaceAttacher() = default;
  ~WhitespaceAttacher();

  void DidVisitText(Text*);
  void DidReattachText(Text*);
  void DidVisitElement(Element*);
  void DidReattachElement(Element*, LayoutObject*);
  bool LastTextNodeNeedsReattach() const {
    return last_text_node_needs_reattach_;
  }
  void SetReattachAllWhitespaceNodes() {
    reattach_all_whitespace_nodes_ = true;
  }
  bool TraverseIntoDisplayContents() const {
    return last_text_node_needs_reattach_ || reattach_all_whitespace_nodes_;
  }

 private:
  void DidReattach(Node*, LayoutObject*);
  void ReattachWhitespaceSiblings(LayoutObject* previous_in_flow);
  void ForceLastTextNodeNeedsReattach();
  void UpdateLastTextNodeFromDisplayContents();

  void SetLastTextNode(Text* text) {
    last_display_contents_ = nullptr;
    last_text_node_ = text;
    if (!text)
      last_text_node_needs_reattach_ = false;
  }

  // If we encounter a display:contents, without traversing its flat tree
  // children during layout tree rebuild, we store that element and start
  // traversing it for text nodes as needed if we re-attach a preceding node
  // without encountering a text node or an in-flow element first.
  //
  // If there is already a text node which needs re-attachment, we traverse into
  // the display:contents element instead as we need to find the last in-flow
  // descendant of that subtree which is used to check if the re-attached text
  // node needs a LayoutText or not.
  //
  // Invariants:
  // DCHECK(!last_display_contents_ || !last_text_node_needs_reattach_)
  // DCHECK(last_text_node_ || !last_text_node_needs_reattach_)
  Element* last_display_contents_ = nullptr;

  // The last text node we've visited during rebuild for this attacher.
  Text* last_text_node_ = nullptr;

  // Set to true if we need to re-attach last_text_node_ when:
  // 1. We visiting a previous in-flow sibling, or
  // 2. We get to the start of the sibling list during the rebuild.
  bool last_text_node_needs_reattach_ = false;

  // Removing a node from the DOM may cause the need for a whitespace
  // LayoutObject to be attached or detached. When the display type changes on
  // an element, the WhitespaceAttacher keeps track of the last text node seen
  // and re-attaches whitespaces as necessary during the tree walk. When
  // removing an element from the tree, we can not selectively track which
  // whitespace nodes which needs to be checked for re-attachment. Thus, we need
  // to check all LayoutText children of the layout tree parent of the removed
  // node for re-attachment. Set to true when all whitespace children needs to
  // be checked for re-attachement.
  bool reattach_all_whitespace_nodes_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_WHITESPACE_ATTACHER_H_
