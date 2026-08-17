// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-pch-file

#ifdef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PRECOMPILE_PLATFORM_H_
#error You shouldn't include the precompiled header file more than once.
#endif

#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PRECOMPILE_PLATFORM_H_

#if defined(_MSC_VER)
#include "third_party/blink/renderer/build/win/precompile.h"
#elif defined(__APPLE__)
#include "third_party/blink/renderer/build/mac/prefix.h"
#else
#include "third_party/blink/renderer/build/linux/prefix.h"
#endif
