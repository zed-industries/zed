// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_ACCESSIBILITY_INSPECT_AX_UTILS_H_
#define TOOLS_ACCESSIBILITY_INSPECT_AX_UTILS_H_

#include <optional>

#include "base/command_line.h"
#include "ui/accessibility/platform/inspect/ax_api_type.h"
#include "ui/accessibility/platform/inspect/ax_inspect.h"
#include "ui/accessibility/platform/inspect/ax_inspect_scenario.h"

namespace tools {

// Prints help for options and the help footer.
void PrintHelpShared();

// Prints help for tree selectors like --pattern, --chromium etc.
void PrintHelpTreeSelectors();

// Prints help for filters.
void PrintHelpFilters();

// Prints the help footer portion.
void PrintHelpFooter();

// Returns tree selector from the command line arguments. Returns nullopt in
// case of error.
std::optional<ui::AXTreeSelector> TreeSelectorFromCommandLine(
    const base::CommandLine& command_line);

// Returns inspect scenario from the command line arguments. Returns nullopt in
// case of error.
std::optional<ui::AXInspectScenario> ScenarioFromCommandLine(
    const base::CommandLine& command_line,
    ui::AXApiType::Type api = ui::AXApiType::kNone);

}  // namespace tools

#endif  // TOOLS_ACCESSIBILITY_INSPECT_AX_UTILS_H_
