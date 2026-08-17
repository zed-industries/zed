// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MEMORY_SIMULATOR_UTILS_H_
#define TOOLS_MEMORY_SIMULATOR_UTILS_H_

#include <stdint.h>

#include "base/time/time.h"

namespace memory_simulator {

// Convert between bytes, GB, MB and pages.
double BytesToGB(uint64_t bytes);
double BytesToMB(uint64_t bytes);
double PagesToGB(uint64_t pages);
double PagesToMB(uint64_t pages);
int64_t MBToPages(uint64_t mb);

// Computes the amount of Megabytes per Second from a number of page at the
// beginning and end of an interval and the duration of the interval.
double PagesToMBPerSec(int64_t pages_begin,
                       int64_t pages_end,
                       base::TimeDelta duration);

}  // namespace memory_simulator

#endif  // TOOLS_MEMORY_SIMULATOR_UTILS_H_
