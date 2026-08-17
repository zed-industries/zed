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

#ifndef SRC_TRACED_PROBES_COMMON_CPU_FREQ_INFO_FOR_TESTING_H_
#define SRC_TRACED_PROBES_COMMON_CPU_FREQ_INFO_FOR_TESTING_H_

#include "src/traced/probes/common/cpu_freq_info.h"

#include <memory>

#include "src/base/test/tmp_dir_tree.h"

namespace perfetto {

class CpuFreqInfoForTesting {
 public:
  CpuFreqInfoForTesting();

  std::unique_ptr<CpuFreqInfo> GetInstance();

 private:
  base::TmpDirTree tmpdir_;
};

}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_COMMON_CPU_FREQ_INFO_FOR_TESTING_H_
