/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef WEBRTC_SYSTEM_WRAPPERS_SOURCE_CPU_LINUX_H_
#define WEBRTC_SYSTEM_WRAPPERS_SOURCE_CPU_LINUX_H_

#include "cpu_wrapper.h"

namespace webrtc {
class CpuLinux : public CpuWrapper {
 public:
  CpuLinux();
  virtual ~CpuLinux();

  int32_t CpuUsage() override;
  int32_t CpuUsage(int8_t* pProcessName, uint32_t length) override { return 0; }
  int32_t CpuUsage(uint32_t dwProcessID) override { return 0; }

  int32_t CpuUsageMultiCore(uint32_t& numCores, uint32_t*& array) override;

  void Reset() override { return; }
  void Stop() override { return; }

  int GetNumCores() override;

 private:
  int GetData(long long& busy,
              long long& idle,
              long long*& busyArray,
              long long*& idleArray);
  

  long long m_oldBusyTime;
  long long m_oldIdleTime;

  long long* m_oldBusyTimeMulti;
  long long* m_oldIdleTimeMulti;

  long long* m_idleArray;
  long long* m_busyArray;
  uint32_t* m_resultArray;
  uint32_t m_numCores;
};
}  // namespace webrtc

#endif  // WEBRTC_SYSTEM_WRAPPERS_SOURCE_CPU_LINUX_H_
