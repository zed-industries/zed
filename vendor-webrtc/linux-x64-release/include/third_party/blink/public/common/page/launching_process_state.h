// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_LAUNCHING_PROCESS_STATE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_LAUNCHING_PROCESS_STATE_H_

#include "build/build_config.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

// This file is used to maintain a consistent initial set of state between the
// RendererProcessHostImpl and the RendererSchedulerImpl.
#if BUILDFLAG(IS_ANDROID)
// This matches Android's ChildProcessConnection state before OnProcessLaunched.
constexpr bool kLaunchingProcessIsBackgrounded = true;
#else
constexpr bool kLaunchingProcessIsBackgrounded = false;
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_LAUNCHING_PROCESS_STATE_H_
