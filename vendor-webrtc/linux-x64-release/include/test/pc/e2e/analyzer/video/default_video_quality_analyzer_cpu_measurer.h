/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_DEFAULT_VIDEO_QUALITY_ANALYZER_CPU_MEASURER_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_DEFAULT_VIDEO_QUALITY_ANALYZER_CPU_MEASURER_H_

#include "rtc_base/synchronization/mutex.h"

namespace webrtc {

// This class is thread safe.
class DefaultVideoQualityAnalyzerCpuMeasurer {
 public:
  double GetCpuUsagePercent() const;

  void StartMeasuringCpuProcessTime();
  void StopMeasuringCpuProcessTime();
  void StartExcludingCpuThreadTime();
  void StopExcludingCpuThreadTime();

 private:
  mutable Mutex mutex_;
  int64_t cpu_time_ RTC_GUARDED_BY(mutex_) = 0;
  int64_t wallclock_time_ RTC_GUARDED_BY(mutex_) = 0;
};

}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_DEFAULT_VIDEO_QUALITY_ANALYZER_CPU_MEASURER_H_
