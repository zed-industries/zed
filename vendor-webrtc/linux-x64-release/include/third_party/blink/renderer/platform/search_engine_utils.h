// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SEARCH_ENGINE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SEARCH_ENGINE_UTILS_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// Returns whether provided string is a URL of a known search engine.
PLATFORM_EXPORT bool IsKnownSearchEngine(const String&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SEARCH_ENGINE_UTILS_H_
