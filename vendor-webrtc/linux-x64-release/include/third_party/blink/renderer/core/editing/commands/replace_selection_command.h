/*
 * Copyright (C) 2005, 2006, 2008 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_REPLACE_SELECTION_COMMAND_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_REPLACE_SELECTION_COMMAND_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node_traversal.h"
#include "third_party/blink/renderer/core/editing/commands/composite_edit_command.h"

namespace blink {

class DocumentFragment;
class ReplacementFragment;

class CORE_EXPORT ReplaceSelectionCommand final : public CompositeEditCommand {
 public:
  enum CommandOption {
    kSelectReplacement = 1 << 0,
    kSmartReplace = 1 << 1,
    kMatchStyle = 1 << 2,
    kPreventNesting = 1 << 3,
    kMovingParagraph = 1 << 4,
    kSanitizeFragment = 1 << 5
  };

  typedef unsigned CommandOptions;

  ReplaceSelectionCommand(Document&,
                          DocumentFragment*,
                          CommandOptions,
                          InputEvent::InputType = InputEvent::InputType::kNone);

  EphemeralRange InsertedRange() const;

  void Trace(Visitor*) const override;

  String TextDataForInputEvent() const final;

 private:
  void DoApply(EditingState*) override;
  InputEvent::InputType GetInputType() const override;
  bool IsReplaceSelectionCommand() const override;
  void HandleStyleSpansBeforeInsertion(ReplacementFragment& fragment,
                                       const Position& insertion_pos);

  class InsertedNodes {
    STACK_ALLOCATED();

   public:
    void RespondToNodeInsertion(Node&);
    void WillRemoveNodePreservingChildren(Node&);
    void WillRemoveNode(Node&);
    void DidReplaceNode(Node&, Node& new_node);

    Node* FirstNodeInserted() const { return first_node_inserted_; }
    Node* LastLeafInserted() const {
      return last_node_inserted_
                 ? &NodeTraversal::LastWithinOrSelf(*last_node_inserted_)
                 : nullptr;
    }
    Node* PastLastLeaf() const {
      return last_node_inserted_
                 ? NodeTraversal::Next(
                       NodeTraversal::LastWithinOrSelf(*last_node_inserted_))
                 : nullptr;
    }
    Node* RefNode() const { return ref_node_; }
    void SetRefNode(Node* node) { ref_node_ = node; }

   private:
    Node* first_node_inserted_ = nullptr;
    Node* last_node_inserted_ = nullptr;
    Node* ref_node_ = nullptr;
  };

  Node* InsertAsListItems(HTMLElement* list_element,
                          Element* insertion_block,
                          const Position&,
                          InsertedNodes&,
                          EditingState*);

  void UpdateNodesInserted(Node*);
  bool ShouldRemoveEndBR(HTMLBRElement*, const VisiblePosition&);

  bool ShouldMergeStart(bool, bool, bool);
  bool ShouldMergeEnd(bool selection_end_was_end_of_paragraph);
  bool ShouldMerge(const VisiblePosition&, const VisiblePosition&);

  void MergeEndIfNeeded(EditingState*);

  void RemoveUnrenderedTextNodesAtEnds(InsertedNodes&);

  void RemoveRedundantStylesAndKeepStyleSpanInline(InsertedNodes&,
                                                   EditingState*);
  void MakeInsertedContentRoundTrippableWithHTMLTreeBuilder(
      const InsertedNodes&,
      EditingState*);
  void MoveElementOutOfAncestor(Element*, Element* ancestor, EditingState*);
  void HandleStyleSpans(InsertedNodes&, EditingState*);

  VisiblePosition PositionAtStartOfInsertedContent() const;
  VisiblePosition PositionAtEndOfInsertedContent() const;

  bool ShouldPerformSmartReplace() const;
  void AddSpacesForSmartReplace(EditingState*);
  void CompleteHTMLReplacement(const Position& last_position_to_select,
                               EditingState*);
  void MergeTextNodesAroundPosition(Position&,
                                    Position& position_only_to_be_updated,
                                    EditingState*);

  bool PerformTrivialReplace(const ReplacementFragment&, EditingState*);
  void SetUpStyle(const VisibleSelection&);
  void InsertParagraphSeparatorIfNeeds(const VisibleSelection&,
                                       const ReplacementFragment&,
                                       EditingState*);

  Position start_of_inserted_content_;
  Position end_of_inserted_content_;
  Member<EditingStyle> insertion_style_;
  const bool select_replacement_;
  const bool smart_replace_;
  bool match_style_;
  Member<DocumentFragment> document_fragment_;
  bool prevent_nesting_;
  const bool moving_paragraph_;
  InputEvent::InputType input_type_;
  const bool sanitize_fragment_;
  bool should_merge_end_;
  String input_event_data_;

  Position start_of_inserted_range_;
  Position end_of_inserted_range_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_REPLACE_SELECTION_COMMAND_H_
