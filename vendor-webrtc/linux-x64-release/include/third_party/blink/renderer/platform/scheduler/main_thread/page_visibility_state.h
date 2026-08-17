// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_PAGE_VISIBILITY_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_PAGE_VISIBILITY_STATE_H_

#include "third_party/perfetto/include/perfetto/tracing/string_helpers.h"

namespace blink::scheduler {

// TODO(altimin): Move to core/.
enum class PageVisibilityState { kVisible, kHidden };

perfetto::StaticString PageVisibilityStateToString(
    PageVisibilityState visibility);

constexpr PageVisibilityState kDefaultPageVisibility =
    PageVisibilityState::kVisible;

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_PAGE_VISIBILITY_STATE_H_
