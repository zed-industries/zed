/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_APPLY_BLOCK_ELEMENT_COMMAND_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_APPLY_BLOCK_ELEMENT_COMMAND_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/core/editing/commands/composite_edit_command.h"

namespace blink {

class CORE_EXPORT ApplyBlockElementCommand : public CompositeEditCommand {
 protected:
  ApplyBlockElementCommand(Document&,
                           const QualifiedName& tag_name,
                           const AtomicString& inline_style);
  ApplyBlockElementCommand(Document&, const QualifiedName& tag_name);

  virtual void FormatSelection(const VisiblePosition& start_of_selection,
                               const VisiblePosition& end_of_selection,
                               EditingState*);
  HTMLElement* CreateBlockElement() const;
  const QualifiedName& TagName() const { return tag_name_; }

 private:
  void DoApply(EditingState*) final;
  virtual void FormatRange(
      const Position& start,
      const Position& end,
      const Position& end_of_selection,
      HTMLElement*& blockquote_for_next_indent,
      VisiblePosition& out_end_of_next_of_paragraph_to_move,
      EditingState*) = 0;
  void RangeForParagraphSplittingTextNodesIfNeeded(
      const VisiblePosition& end_of_current_paragraph,
      Position& end_of_last_paragraph,
      Position& start,
      Position& end);
  VisiblePosition EndOfNextParagrahSplittingTextNodesIfNeeded(
      VisiblePosition& end_of_current_paragraph,
      Position& end_of_last_paragraph,
      Position& start,
      Position& end);

  QualifiedName tag_name_;
  AtomicString inline_style_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_APPLY_BLOCK_ELEMENT_COMMAND_H_
