/*
 * Copyright (C) 2025 The Android Open Source Project
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

#ifndef SRC_TRACED_PROBES_ANDROID_KERNEL_WAKELOCKS_ANDROID_KERNEL_WAKELOCKS_DATA_SOURCE_H_
#define SRC_TRACED_PROBES_ANDROID_KERNEL_WAKELOCKS_ANDROID_KERNEL_WAKELOCKS_DATA_SOURCE_H_

#include <memory>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/scoped_file.h"
#include "perfetto/ext/base/weak_ptr.h"
#include "perfetto/tracing/core/data_source_config.h"
#include "src/traced/probes/probes_data_source.h"

namespace perfetto {

struct KernelWakelockInfo;

class TraceWriter;
namespace base {
class TaskRunner;
}

class AndroidKernelWakelocksDataSource : public ProbesDataSource {
 public:
  static const ProbesDataSource::Descriptor descriptor;

  AndroidKernelWakelocksDataSource(const DataSourceConfig&,
                                   base::TaskRunner*,
                                   TracingSessionID,
                                   std::unique_ptr<TraceWriter> writer);

  ~AndroidKernelWakelocksDataSource() override;

  // ProbesDataSource implementation.
  void Start() override;
  void Flush(FlushRequestID, std::function<void()> callback) override;
  void ClearIncrementalState() override;

 private:
  struct DynamicLibLoader;

  void Tick();
  void WriteKernelWakelocks();
  base::WeakPtr<AndroidKernelWakelocksDataSource> GetWeakPtr() const;

  uint32_t poll_interval_ms_ = 0;
  base::FlatHashMap<std::string, KernelWakelockInfo> wakelocks_;
  uint32_t next_id_ = 0;

  base::TaskRunner* const task_runner_;
  std::unique_ptr<TraceWriter> writer_;
  std::unique_ptr<DynamicLibLoader> lib_;
  base::WeakPtrFactory<AndroidKernelWakelocksDataSource>
      weak_factory_;  // Keep last.
};

}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_ANDROID_KERNEL_WAKELOCKS_ANDROID_KERNEL_WAKELOCKS_DATA_SOURCE_H_
