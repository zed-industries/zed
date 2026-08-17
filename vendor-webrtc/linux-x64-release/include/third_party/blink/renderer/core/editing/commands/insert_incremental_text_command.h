// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_INSERT_INCREMENTAL_TEXT_COMMAND_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_INSERT_INCREMENTAL_TEXT_COMMAND_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/commands/insert_text_command.h"

namespace blink {

class CORE_EXPORT InsertIncrementalTextCommand final
    : public InsertTextCommand {
 public:
  InsertIncrementalTextCommand(
      Document&,
      const String& text,
      RebalanceType = kRebalanceLeadingAndTrailingWhitespaces);

 private:
  void DoApply(EditingState*) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_INSERT_INCREMENTAL_TEXT_COMMAND_H_
