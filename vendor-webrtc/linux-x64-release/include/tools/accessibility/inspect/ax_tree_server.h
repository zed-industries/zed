// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef AX_TREE_SERVER_H_
#define AX_TREE_SERVER_H_

#include "base/files/file_path.h"
#include "base/functional/callback.h"
#include "build/build_config.h"
#include "ui/accessibility/platform/inspect/ax_api_type.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/scoped_com_initializer.h"
#endif

namespace ui {
struct AXTreeSelector;
class AXInspectScenario;
}  // namespace ui

namespace content {

class AXTreeServer final {
 public:
  AXTreeServer(const ui::AXTreeSelector& selector,
               const ui::AXInspectScenario& scenario,
               ui::AXApiType::Type type);

  AXTreeServer(const AXTreeServer&) = delete;
  AXTreeServer& operator=(const AXTreeServer&) = delete;

  // If an error occurs during initialization, set bit here.
  bool error;

 private:
#if BUILDFLAG(IS_WIN)
  // Only one COM initializer per thread is permitted.
  base::win::ScopedCOMInitializer com_initializer_;
#endif
};

}  // namespace content

#endif  // AX_TREE_SERVER_H_
