/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACED_PROBES_COMMON_CPU_FREQ_INFO_H_
#define SRC_TRACED_PROBES_COMMON_CPU_FREQ_INFO_H_

#include <cstdint>
#include <string>
#include <vector>

namespace perfetto {

class CpuFreqInfo {
 public:
  explicit CpuFreqInfo(std::string cpu_dir_path = "/sys/devices/system/cpu");
  virtual ~CpuFreqInfo();

  using Range =
      std::pair</* begin */ const uint32_t*, /* end */ const uint32_t*>;
  Range GetFreqs(uint32_t cpu);
  uint32_t GetCpuFreqIndex(uint32_t cpu, uint32_t freq);

  const std::vector<uint32_t>& ReadCpuCurrFreq();

 private:
  // e.g. /sys/devices/system/cpu
  std::string sysfs_cpu_path_;
  // All frequencies of all CPUs, ordered by CPU and frequency. Includes a guard
  // at the end.
  std::vector<uint32_t> frequencies_;
  // frequencies_index_[cpu] points to first frequency in frequencies_. Includes
  // a guard at the end.
  std::vector<size_t> frequencies_index_;
  // Placeholder for CPU current frequency, refresh in ReadCpuCurrFreq()
  std::vector<uint32_t> cpu_curr_freq_;

  std::string ReadFile(std::string path);
};

}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_COMMON_CPU_FREQ_INFO_H_
