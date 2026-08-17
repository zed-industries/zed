// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITING_COMMAND_FILTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITING_COMMAND_FILTER_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {
bool IsCommandFilteredOut(const String& command_name);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITING_COMMAND_FILTER_H_
