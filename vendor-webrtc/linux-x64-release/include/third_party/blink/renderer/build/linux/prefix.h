// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-pch-file
// no-std-usage-because-pch-file

// This file is used as a precompiled header for both C and C++ files. So
// any C++ headers must go in the __cplusplus block below.

#if defined(THIRD_PARTY_BLINK_RENDERER_BUILD_LINUX_PREFIX_H_)
#error You shouldn't include the precompiled header file more than once.
#endif

#define THIRD_PARTY_BLINK_RENDERER_BUILD_LINUX_PREFIX_H_

#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <math.h>
#include <memory.h>
#include <signal.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#if defined(__cplusplus)

#include <algorithm>
#include <bitset>
#include <cmath>
#include <cstddef>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <functional>
#include <iomanip>
#include <iosfwd>
#include <iterator>
#include <limits>
#include <list>
#include <map>
#include <numeric>
#include <ostream>
#include <queue>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#endif  // __cplusplus
