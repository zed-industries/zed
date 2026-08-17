/*
 * Copyright (C) 2006, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_INDENT_OUTDENT_COMMAND_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_INDENT_OUTDENT_COMMAND_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/commands/apply_block_element_command.h"

namespace blink {

class CORE_EXPORT IndentOutdentCommand final : public ApplyBlockElementCommand {
 public:
  enum IndentType { kIndent, kOutdent };

  IndentOutdentCommand(Document&, IndentType);

  bool PreservesTypingStyle() const override { return true; }

 private:
  InputEvent::InputType GetInputType() const override;

  void OutdentRegion(const VisiblePosition&,
                     const VisiblePosition&,
                     EditingState*);
  void OutdentParagraph(EditingState*);
  bool TryIndentingAsListItem(
      const Position& start,
      const Position& end,
      VisiblePosition& out_end_of_next_of_paragraph_to_move,
      EditingState*);
  void IndentIntoBlockquote(const Position&,
                            const Position&,
                            HTMLElement*&,
                            EditingState*);
  void SetEndingSelectionToListChildIfListItem();

  void FormatSelection(const VisiblePosition& start_of_selection,
                       const VisiblePosition& end_of_selection,
                       EditingState*) override;
  void FormatRange(const Position& start,
                   const Position& end,
                   const Position& end_of_selection,
                   HTMLElement*& blockquote_for_next_indent,
                   VisiblePosition& out_end_of_next_of_paragraph_to_move,
                   EditingState*) override;

  IndentType type_of_action_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_INDENT_OUTDENT_COMMAND_H_
