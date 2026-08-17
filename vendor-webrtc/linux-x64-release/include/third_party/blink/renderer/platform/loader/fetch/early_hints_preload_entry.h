// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_EARLY_HINTS_PRELOAD_ENTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_EARLY_HINTS_PRELOAD_ENTRY_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Represents a resource preloaded by an Early Hints ressponse.
// TODO(https://crbug.com/1317936): Add more fields such as request destination.
struct PLATFORM_EXPORT EarlyHintsPreloadEntry {
  enum class State {
    // The resource is not used by the document yet.
    kUnused,
    // The resource was not used within a few seconds from the window's load
    // event and a warning message was shown.
    kWarnedUnused,
  };

  DISALLOW_NEW();

  EarlyHintsPreloadEntry() = default;

  State state = State::kUnused;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_EARLY_HINTS_PRELOAD_ENTRY_H_
