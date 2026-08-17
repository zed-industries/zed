// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_SET_CHARACTER_DATA_COMMAND_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_SET_CHARACTER_DATA_COMMAND_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/commands/edit_command.h"

namespace blink {

class CORE_EXPORT SetCharacterDataCommand final : public SimpleEditCommand {
 public:
  SetCharacterDataCommand(Text* node,
                          unsigned offset,
                          unsigned count,
                          const String& text);

  void Trace(Visitor*) const override;

 private:
  // EditCommand implementation
  void DoApply(EditingState*) final;
  void DoUnapply() final;

  const Member<Text> node_;
  const unsigned offset_;
  const unsigned count_;
  String previous_text_for_undo_;
  const String new_text_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_SET_CHARACTER_DATA_COMMAND_H_
