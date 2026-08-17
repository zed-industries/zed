/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_UNDO_STEP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_UNDO_STEP_H_

#include "third_party/blink/renderer/core/editing/commands/selection_for_undo_step.h"
#include "third_party/blink/renderer/core/events/input_event.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SimpleEditCommand;

class UndoStep final : public GarbageCollected<UndoStep> {
 public:
  UndoStep(Document*,
           const SelectionForUndoStep& starting_selection,
           const SelectionForUndoStep& ending_selection);

  // Returns true if is owned by |element|
  bool IsOwnedBy(const Element& element) const;
  void Unapply();
  void Reapply();
  void Append(SimpleEditCommand*);
  void Append(UndoStep*);

  Document& GetDocument() const { return *document_; }
  const SelectionForUndoStep& StartingSelection() const {
    return starting_selection_;
  }
  const SelectionForUndoStep& EndingSelection() const {
    return ending_selection_;
  }
  bool SelectionIsDirectional() const { return selection_is_directional_; }
  void SetStartingSelection(const SelectionForUndoStep&);
  void SetEndingSelection(const SelectionForUndoStep&);
  void SetSelectionIsDirectional(bool is_directional) {
    selection_is_directional_ = is_directional;
  }
  Element* StartingRootEditableElement() const {
    return starting_selection_.RootEditableElement();
  }
  Element* EndingRootEditableElement() const {
    return ending_selection_.RootEditableElement();
  }

  uint64_t SequenceNumber() const { return sequence_number_; }

  void Trace(Visitor*) const;

 private:
  Member<Document> document_;
  SelectionForUndoStep starting_selection_;
  SelectionForUndoStep ending_selection_;
  HeapVector<Member<SimpleEditCommand>> commands_;
  const uint64_t sequence_number_;
  bool selection_is_directional_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_UNDO_STEP_H_
