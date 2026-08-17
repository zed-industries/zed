// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_DRAG_AND_DROP_COMMAND_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_DRAG_AND_DROP_COMMAND_H_

#include "third_party/blink/renderer/core/editing/commands/composite_edit_command.h"

namespace blink {

// |DragAndDropCommand| is a dummy command. It doesn't do anything by itself,
// but will act as a catcher for the following |DeleteByDrag| and
// |InsertFromDrop| commands, and combine them into a single undo entry.
// In the future when necessary, this mechanism can be generalized into a common
// command wrapper to achieve undo group.
class DragAndDropCommand final : public CompositeEditCommand {
 public:
  explicit DragAndDropCommand(Document&);

  bool IsCommandGroupWrapper() const override;
  bool IsDragAndDropCommand() const override;

 private:
  void DoApply(EditingState*) override;
  InputEvent::InputType GetInputType() const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_DRAG_AND_DROP_COMMAND_H_
