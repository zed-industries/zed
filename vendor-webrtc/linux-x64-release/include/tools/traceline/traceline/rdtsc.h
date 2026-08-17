// Copyright 2009 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TRACELINE_RDTSC_H_
#define TRACELINE_RDTSC_H_

#include <windows.h>

#include <powrprof.h>

#include "logging.h"

class RDTSCNormalizer {
 public:
  RDTSCNormalizer() { }
  ~RDTSCNormalizer() { }

  void Start() {
    LARGE_INTEGER freq, now;
    if (QueryPerformanceFrequency(&freq) == 0) {
      NOTREACHED("");
    }
    freq_ = freq.QuadPart;

    if (QueryPerformanceCounter(&now) == 0) {
      NOTREACHED("");
    }
    start_ = now.QuadPart;
  }

  // Calculate the time from start for a given processor.
  double MsFromStart(void* procid, __int64 stamp) {
    return (stamp - start_) / (freq_ / 1000.0);
  }

 private:
  __int64 freq_;
  __int64 start_;
};

#endif  // TRACELINE_RDTSC_H_
