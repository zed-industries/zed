// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-pch-file
// no-std-usage-because-pch-file

#ifdef THIRD_PARTY_BLINK_RENDERER_BUILD_WIN_PRECOMPILE_H_
#error You shouldn't include the precompiled header file more than once.
#endif

#define THIRD_PARTY_BLINK_RENDERER_BUILD_WIN_PRECOMPILE_H_

// Precompiled header for Blink when built on Windows using
// GYP-generated project files.  Not used by other build
// configurations.
//
// Using precompiled headers speeds the build up significantly.  On a
// fast machine (HP Z600, 12 GB of RAM), an ~18% decrease in full
// build time was measured.

#define _USE_MATH_DEFINES  // Make math.h behave like other platforms.

#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <math.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <algorithm>
#include <ciso646>
#include <cmath>
#include <cstddef>
#include <limits>
#include <string>
#include <utility>
