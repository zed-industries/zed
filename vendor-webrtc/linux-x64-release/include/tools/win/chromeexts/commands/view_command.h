// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_WIN_CHROMEEXTS_COMMANDS_VIEW_COMMAND_H_
#define TOOLS_WIN_CHROMEEXTS_COMMANDS_VIEW_COMMAND_H_

#include "tools/win/chromeexts/chrome_exts_command.h"

namespace tools {
namespace win {
namespace chromeexts {

class ViewCommand : public ChromeExtsCommand {
 public:
  ViewCommand();
  ViewCommand(const ViewCommand&) = delete;
  ViewCommand& operator=(const ViewCommand&) = delete;
  ~ViewCommand() override;

 protected:
  HRESULT Execute() override;
};

}  // namespace chromeexts
}  // namespace win
}  // namespace tools

#endif  // TOOLS_WIN_CHROMEEXTS_COMMANDS_VIEW_COMMAND_H_
