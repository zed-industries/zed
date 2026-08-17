/*
 * Copyright (C) 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_TYPING_COMMAND_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_TYPING_COMMAND_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/commands/composite_edit_command.h"
#include "third_party/blink/renderer/core/editing/text_granularity.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CORE_EXPORT TypingCommand final : public CompositeEditCommand {
 public:
  enum CommandType {
    kDeleteSelection,
    kDeleteKey,
    kForwardDeleteKey,
    kInsertText,
    kInsertLineBreak,
    kInsertParagraphSeparator,
    kInsertParagraphSeparatorInQuotedContent
  };

  enum TextCompositionType {
    kTextCompositionNone,
    kTextCompositionUpdate,
    kTextCompositionConfirm,
    kTextCompositionCancel
  };

  enum Option {
    kSelectInsertedText = 1 << 0,
    kKillRing = 1 << 1,
    kSmartDelete = 1 << 2
  };
  typedef unsigned Options;

  static void DeleteSelection(Document&, Options = 0);
  static void DeleteKeyPressed(Document&,
                               Options,
                               TextGranularity = TextGranularity::kCharacter);
  static void ForwardDeleteKeyPressed(
      Document&,
      EditingState*,
      Options = 0,
      TextGranularity = TextGranularity::kCharacter);
  static void InsertText(Document&,
                         const String&,
                         Options,
                         TextCompositionType = kTextCompositionNone,
                         const bool is_incremental_insertion = false);
  static void InsertText(
      Document&,
      const String&,
      const SelectionInDOMTree&,
      Options,
      EditingState*,
      TextCompositionType = kTextCompositionNone,
      const bool is_incremental_insertion = false,
      InputEvent::InputType = InputEvent::InputType::kInsertText);
  static bool InsertLineBreak(Document&);
  static bool InsertParagraphSeparator(Document&);
  static bool InsertParagraphSeparatorInQuotedContent(Document&);
  static void CloseTyping(LocalFrame*);
  static void CloseTypingIfNeeded(LocalFrame*);

  static TypingCommand* LastTypingCommandIfStillOpenForTyping(LocalFrame*);
  static void UpdateSelectionIfDifferentFromCurrentSelection(TypingCommand*,
                                                             LocalFrame*);

  TypingCommand(Document&,
                CommandType,
                const String& text = g_empty_string,
                Options options = 0,
                TextGranularity granularity = TextGranularity::kCharacter,
                TextCompositionType = kTextCompositionNone);

  void InsertTextRunWithoutNewlines(const String& text,
                                    EditingState*);
  void InsertLineBreak(EditingState*);
  void InsertParagraphSeparatorInQuotedContent(EditingState*);
  void InsertParagraphSeparator(EditingState*);
  void DeleteKeyPressed(TextGranularity, bool kill_ring, EditingState*);
  void ForwardDeleteKeyPressed(TextGranularity, bool kill_ring, EditingState*);
  void DeleteSelection(bool smart_delete, EditingState*);
  void SetCompositionType(TextCompositionType type) {
    composition_type_ = type;
  }
  void AdjustSelectionAfterIncrementalInsertion(
      LocalFrame*,
      const wtf_size_t selection_start,
      const wtf_size_t text_length,
      EditingState*);

  CommandType CommandTypeOfOpenCommand() const { return command_type_; }
  TextCompositionType CompositionType() const { return composition_type_; }
  // |TypingCommand| may contain multiple |InsertTextCommand|, should return
  // |textDataForInputEvent()| of the last one.
  String TextDataForInputEvent() const final;

 private:
  void SetSmartDelete(bool smart_delete) { smart_delete_ = smart_delete; }
  bool IsOpenForMoreTyping() const { return open_for_more_typing_; }
  void CloseTyping() { open_for_more_typing_ = false; }

  void InsertTextInternal(const String& text,
                          bool select_inserted_text,
                          EditingState*);

  void DoApply(EditingState*) override;
  InputEvent::InputType GetInputType() const override;
  bool IsTypingCommand() const override;
  bool PreservesTypingStyle() const override { return preserves_typing_style_; }

  void UpdatePreservesTypingStyle(CommandType);
  void TypingAddedToOpenCommand(CommandType);
  bool MakeEditableRootEmpty(EditingState*);

  void UpdateCommandTypeOfOpenCommand(CommandType command_type) {
    command_type_ = command_type;
  }

  bool IsIncrementalInsertion() const { return is_incremental_insertion_; }

  void DeleteKeyPressedInternal(
      const SelectionForUndoStep& selection_to_delete,
      const SelectionForUndoStep& selection_after_undo,
      bool kill_ring,
      EditingState*);

  void DeleteSelectionIfRange(const SelectionForUndoStep&, EditingState*);

  void ForwardDeleteKeyPressedInternal(
      const SelectionForUndoStep& selection_to_delete,
      const SelectionForUndoStep& selection_after_undo,
      bool kill_ring,
      EditingState*);

  CommandType command_type_;
  String text_to_insert_;
  bool open_for_more_typing_;
  const bool select_inserted_text_;
  bool smart_delete_;
  const TextGranularity granularity_;
  TextCompositionType composition_type_;
  const bool kill_ring_;
  bool preserves_typing_style_;

  // Undoing a series of backward deletes will restore a selection around all of
  // the characters that were deleted, but only if the typing command being
  // undone was opened with a backward delete.
  bool opened_by_backward_delete_;

  bool is_incremental_insertion_;
  wtf_size_t selection_start_;
  InputEvent::InputType input_type_;
};

template <>
struct DowncastTraits<TypingCommand> {
  static bool AllowFrom(const CompositeEditCommand& command) {
    return command.IsTypingCommand();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_TYPING_COMMAND_H_
