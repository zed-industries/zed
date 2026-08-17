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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_COMPOSITE_EDIT_COMMAND_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_COMPOSITE_EDIT_COMMAND_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/editing/commands/edit_command.h"
#include "third_party/blink/renderer/core/editing/commands/editing_state.h"
#include "third_party/blink/renderer/core/editing/commands/undo_step.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class DeleteSelectionOptions;
class EditingStyle;
class Element;
class HTMLBRElement;
class HTMLElement;
class HTMLSpanElement;
class Text;

class CORE_EXPORT CompositeEditCommand : public EditCommand {
 public:
  enum ShouldPreserveSelection { kPreserveSelection, kDoNotPreserveSelection };
  enum ShouldPreserveStyle { kPreserveStyle, kDoNotPreserveStyle };

  ~CompositeEditCommand() override;

  const SelectionForUndoStep& StartingSelection() const {
    return starting_selection_;
  }
  const SelectionForUndoStep& EndingSelection() const {
    return ending_selection_;
  }

  void SetStartingSelection(const SelectionForUndoStep&);
  void SetEndingSelection(const SelectionForUndoStep&);

  void SetParent(CompositeEditCommand*) override;

  // Returns |false| if the command failed.  e.g. It's aborted.
  bool Apply();
  bool IsFirstCommand(EditCommand* command) {
    return !commands_.empty() && commands_.front() == command;
  }
  UndoStep* GetUndoStep() { return undo_step_.Get(); }
  UndoStep* EnsureUndoStep();
  // Append undo step from an already applied command.
  void AppendCommandToUndoStep(CompositeEditCommand*);

  virtual bool IsReplaceSelectionCommand() const;
  virtual bool IsTypingCommand() const;
  virtual bool IsCommandGroupWrapper() const;
  virtual bool IsDragAndDropCommand() const;
  virtual bool PreservesTypingStyle() const;

  virtual void AppliedEditing();

  void Trace(Visitor*) const override;

 protected:
  explicit CompositeEditCommand(Document&);

  VisibleSelection EndingVisibleSelection() const;
  //
  // sugary-sweet convenience functions to help create and apply edit commands
  // in composite commands
  //
  void AppendNode(Node*, ContainerNode* parent, EditingState*);
  void ApplyCommandToComposite(EditCommand*, EditingState*);
  void ApplyStyle(const EditingStyle*, EditingState*);
  void ApplyStyle(const EditingStyle*,
                  const Position& start,
                  const Position& end,
                  EditingState*);
  void ApplyStyledElement(Element*, EditingState*);
  void RemoveStyledElement(Element*, EditingState*);
  // Returns |false| if the EditingState has been aborted.
  bool DeleteSelection(EditingState*, const DeleteSelectionOptions&);
  virtual void DeleteTextFromNode(Text*, unsigned offset, unsigned count);
  bool IsRemovableBlock(const Node*);
  void InsertNodeAfter(Node*, Node* ref_child, EditingState*);
  void InsertNodeAt(Node*, const Position&, EditingState*);
  void InsertNodeAtTabSpanPosition(Node*, const Position&, EditingState*);
  void InsertNodeBefore(Node*,
                        Node* ref_child,
                        EditingState*,
                        ShouldAssumeContentIsAlwaysEditable =
                            kDoNotAssumeContentIsAlwaysEditable);
  void InsertParagraphSeparator(
      EditingState*,
      bool use_default_paragraph_element = false,
      bool paste_blockqutoe_into_unquoted_area = false);
  void InsertTextIntoNode(Text*, unsigned offset, const String& text);
  void MergeIdenticalElements(Element*, Element*, EditingState*);
  void RebalanceWhitespace();
  void RebalanceWhitespaceAt(const Position&);
  void RebalanceWhitespaceOnTextSubstring(Text*,
                                          int start_offset,
                                          int end_offset);
  void PrepareWhitespaceAtPositionForSplit(Position&);
  void ReplaceCollapsibleWhitespaceWithNonBreakingSpaceIfNeeded(
      const VisiblePosition&);
  bool CanRebalance(const Position&) const;
  void RemoveCSSProperty(Element*, CSSPropertyID);
  void RemoveElementAttribute(Element*, const QualifiedName& attribute);
  // Remove all children if possible
  void RemoveAllChildrenIfPossible(ContainerNode*,
                                   EditingState*,
                                   ShouldAssumeContentIsAlwaysEditable =
                                       kDoNotAssumeContentIsAlwaysEditable);
  void RemoveChildrenInRange(Node*, unsigned from, unsigned to, EditingState*);
  virtual void RemoveNode(Node*,
                          EditingState*,
                          ShouldAssumeContentIsAlwaysEditable =
                              kDoNotAssumeContentIsAlwaysEditable);
  HTMLSpanElement* ReplaceElementWithSpanPreservingChildrenAndAttributes(
      HTMLElement*);
  void RemoveNodePreservingChildren(Node*,
                                    EditingState*,
                                    ShouldAssumeContentIsAlwaysEditable =
                                        kDoNotAssumeContentIsAlwaysEditable);
  void RemoveNodeAndPruneAncestors(Node*,
                                   EditingState*,
                                   Node* exclude_node = nullptr);
  void MoveRemainingSiblingsToNewParent(Node*,
                                        Node* past_last_node_to_move,
                                        Element* new_parent,
                                        EditingState*);
  void UpdatePositionForNodeRemovalPreservingChildren(Position&, Node&);
  void Prune(Node*, EditingState*, Node* exclude_node = nullptr);
  void ReplaceTextInNode(Text*,
                         unsigned offset,
                         unsigned count,
                         const String& replacement_text);
  Position ReplaceSelectedTextInNode(const String&);
  Position PositionOutsideTabSpan(const Position&);
  void SetNodeAttribute(Element*,
                        const QualifiedName& attribute,
                        const AtomicString& value);
  void SplitElement(Element*, Node* at_child);
  void SplitTextNode(Text*, unsigned offset);
  void SplitTextNodeContainingElement(Text*, unsigned offset);
  void WrapContentsInDummySpan(Element*);

  void DeleteInsignificantText(Text*, unsigned start, unsigned end);
  void DeleteInsignificantText(const Position& start, const Position& end);
  void DeleteInsignificantTextDownstream(const Position&);

  HTMLBRElement* AppendBlockPlaceholder(Element*, EditingState*);
  HTMLBRElement* InsertBlockPlaceholder(const Position&, EditingState*);
  HTMLBRElement* AddBlockPlaceholderIfNeeded(Element*, EditingState*);
  void RemovePlaceholderAt(const Position&);

  HTMLElement* InsertNewDefaultParagraphElementAt(const Position&,
                                                  EditingState*);

  HTMLElement* MoveParagraphContentsToNewBlockIfNecessary(const Position&,
                                                          EditingState*);

  void PushAnchorElementDown(Element*, EditingState*);

  void MoveParagraph(const VisiblePosition&,
                     const VisiblePosition&,
                     const VisiblePosition&,
                     EditingState*,
                     ShouldPreserveSelection = kDoNotPreserveSelection,
                     ShouldPreserveStyle = kPreserveStyle,
                     Node* constraining_ancestor = nullptr);
  void MoveParagraphs(const VisiblePosition&,
                      const VisiblePosition&,
                      const VisiblePosition&,
                      EditingState*,
                      ShouldPreserveSelection = kDoNotPreserveSelection,
                      ShouldPreserveStyle = kPreserveStyle,
                      Node* constraining_ancestor = nullptr);
  void MoveParagraphWithClones(
      const VisiblePosition& start_of_paragraph_to_move,
      const VisiblePosition& end_of_paragraph_to_move,
      HTMLElement* block_element,
      Node* outer_node,
      EditingState*);
  void CloneParagraphUnderNewElement(const Position& start,
                                     const Position& end,
                                     Node* outer_node,
                                     Element* block_element,
                                     EditingState*);
  void CleanupAfterDeletion(EditingState*, VisiblePosition destination);
  void CleanupAfterDeletion(EditingState*);

  bool BreakOutOfEmptyListItem(EditingState*);
  bool BreakOutOfEmptyMailBlockquotedParagraph(EditingState*);

  Position PositionAvoidingSpecialElementBoundary(const Position&,
                                                  EditingState*);

  Node* SplitTreeToNode(Node*, Node*, bool split_ancestor = false);

  static bool IsNodeVisiblyContainedWithin(Node&, const EphemeralRange&);

  HeapVector<Member<EditCommand>> commands_;

 private:
  bool IsCompositeEditCommand() const final { return true; }

  SelectionForUndoStep starting_selection_;
  SelectionForUndoStep ending_selection_;
  Member<UndoStep> undo_step_;
};

template <>
struct DowncastTraits<CompositeEditCommand> {
  static bool AllowFrom(const EditCommand& command) {
    return command.IsCompositeEditCommand();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_COMPOSITE_EDIT_COMMAND_H_
