// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_WIN_CHROMEEXTS_CHROME_EXTS_COMMAND_H_
#define TOOLS_WIN_CHROMEEXTS_CHROME_EXTS_COMMAND_H_

#include <dbgeng.h>
#include <stdarg.h>
#include <wrl/client.h>

#include <memory>
#include <string>

#include "base/command_line.h"
#include "base/memory/ptr_util.h"

namespace tools {
namespace win {
namespace chromeexts {

namespace {
using Microsoft::WRL::ComPtr;
}  // namespace

// Superclass of all commands in the debugger extension.
// To implement your own command, just follow these steps:
// 1) Create a new class and subclass ChromeExtsCommand.
// 2) Implement Execute().
// 3) Add a function that calls Run<Your Subclass>() to chromeexts.cc.
// 4) Add your new function to the exports list in chromeexts.def.
// Done!
class ChromeExtsCommand {
 public:
  template <typename T>
  static HRESULT Run(IDebugClient* debug_client, const char* args) {
    std::unique_ptr<ChromeExtsCommand> command = std::make_unique<T>();
    HRESULT hr = command->Initialize(debug_client, args);
    if (SUCCEEDED(hr)) {
      hr = command->Execute();
    }
    return hr;
  }

  ChromeExtsCommand(const ChromeExtsCommand&) = delete;
  ChromeExtsCommand& operator=(const ChromeExtsCommand&) = delete;

  virtual ~ChromeExtsCommand();

 protected:
  ChromeExtsCommand();

  virtual HRESULT Initialize(IDebugClient* debug_client, const char* args);

  virtual HRESULT Execute() = 0;

  HRESULT Printf(const char* format, ...);
  HRESULT PrintfWithIndent(int indent_level, const char* format, ...);
  HRESULT PrintV(const char* format, va_list ap);

  HRESULT PrintErrorf(const char* format, ...);
  HRESULT PrintErrorV(const char* format, va_list ap);

  const base::CommandLine& command_line() const { return command_line_; }

  // Returns the Debug Client as T, null ComPtr<T> otherwise.
  template <typename T>
  ComPtr<T> GetDebugClientAs() {
    ComPtr<T> target_interface;
    debug_client_.As(&target_interface);
    return target_interface;
  }

 private:
  base::CommandLine command_line_{base::CommandLine::NO_PROGRAM};
  ComPtr<IDebugClient> debug_client_;
  ComPtr<IDebugControl> debug_control_;
};

}  // namespace chromeexts
}  // namespace win
}  // namespace tools

#endif  // TOOLS_WIN_CHROMEEXTS_CHROME_EXTS_COMMAND_H_
