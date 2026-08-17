// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_USER_LEVEL_MEMORY_PRESSURE_SIGNAL_GENERATOR_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_USER_LEVEL_MEMORY_PRESSURE_SIGNAL_GENERATOR_H_

#include "build/build_config.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

#if BUILDFLAG(IS_ANDROID)
// Request |blink::UserLevelMemoryPressureSignalGenerator| to generate a memory
// pressure siganl.
BLINK_EXPORT void RequestUserLevelMemoryPressureSignal();
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_USER_LEVEL_MEMORY_PRESSURE_SIGNAL_GENERATOR_H_
