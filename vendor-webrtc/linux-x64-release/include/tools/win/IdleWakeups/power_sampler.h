// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_WIN_IDLEWAKEUPS_POWER_SAMPLER_H_
#define TOOLS_WIN_IDLEWAKEUPS_POWER_SAMPLER_H_

#include <windows.h>

#include <map>
#include <string>

// https://software.intel.com/en-us/blogs/2012/12/13/using-the-intel-power-gadget-api-on-mac-os-x
typedef int (*IntelEnergyLibInitialize_t)();
typedef int (*GetNumMsrs_t)(int* nMsr);
typedef int (*GetMsrName_t)(int iMsr, wchar_t* szName);
typedef int (*GetMsrFunc_t)(int iMsr, int* pFuncID);
typedef int (*GetPowerData_t)(int iNode,
                              int iMsr,
                              double* pResult,
                              int* nResult);
typedef int (*ReadSample_t)();

class PowerSampler {
 public:
  PowerSampler();
  ~PowerSampler();

  void SampleCPUPowerState();

  double get_power(std::wstring key) { return power_[key]; }

 private:
  // Power consumed in mWh since the last sample.
  std::map<std::wstring, double> power_;

  HMODULE energy_lib_ = nullptr;
  IntelEnergyLibInitialize_t IntelEnergyLibInitialize = nullptr;
  GetNumMsrs_t GetNumMsrs = nullptr;
  GetMsrName_t GetMsrName = nullptr;
  GetMsrFunc_t GetMsrFunc = nullptr;
  GetPowerData_t GetPowerData = nullptr;
  ReadSample_t ReadSample = nullptr;

  PowerSampler& operator=(const PowerSampler&) = delete;
  PowerSampler(const PowerSampler&) = delete;
};

#endif  // TOOLS_WIN_IDLEWAKEUPS_POWER_SAMPLER_H_