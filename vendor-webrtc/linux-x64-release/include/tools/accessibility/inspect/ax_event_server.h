// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef AX_EVENT_SERVER_H_
#define AX_EVENT_SERVER_H_

#include "base/process/process_handle.h"
#include "build/build_config.h"
#include "ui/accessibility/platform/inspect/ax_event_recorder.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/scoped_com_initializer.h"
#endif

namespace ui {
struct AXTreeSelector;
class AXInspectScenario;
}  // namespace ui

namespace tools {

class AXEventServer final {
 public:
  // Dumps events into console for application identified either by process id
  // or tree selector.
  explicit AXEventServer(base::ProcessId pid,
                         const ui::AXTreeSelector& selector,
                         const ui::AXInspectScenario& scenario);

  AXEventServer(const AXEventServer&) = delete;
  AXEventServer& operator=(const AXEventServer&) = delete;

  ~AXEventServer();

 private:
  void OnEvent(const std::string& event) const;

#if BUILDFLAG(IS_WIN)
  // Only one COM initializer per thread is permitted.
  base::win::ScopedCOMInitializer com_initializer_;
#endif
  std::unique_ptr<ui::AXEventRecorder> recorder_;
};

}  // namespace tools

#endif  // AX_EVENT_SERVER_H_
