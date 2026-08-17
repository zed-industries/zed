/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef SRC_TRACED_PROBES_STATSD_CLIENT_STATSD_BINDER_DATA_SOURCE_H_
#define SRC_TRACED_PROBES_STATSD_CLIENT_STATSD_BINDER_DATA_SOURCE_H_

#include <memory>

#include "perfetto/base/task_runner.h"
#include "perfetto/ext/base/utils.h"
#include "perfetto/ext/base/weak_ptr.h"
#include "perfetto/ext/protozero/proto_ring_buffer.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/ext/tracing/core/trace_writer.h"
#include "perfetto/tracing/core/forward_decls.h"
#include "src/traced/probes/probes_data_source.h"

namespace perfetto {

class StatsdBinderDataSource : public ProbesDataSource {
 public:
  static const ProbesDataSource::Descriptor descriptor;

  StatsdBinderDataSource(base::TaskRunner*,
                         TracingSessionID,
                         std::unique_ptr<TraceWriter> writer,
                         const DataSourceConfig&);
  ~StatsdBinderDataSource() override;

  // ProbesDataSource implementation.
  void Start() override;
  void Flush(FlushRequestID, std::function<void()> callback) override;
  void ClearIncrementalState() override;

  void OnData(uint32_t reason, const uint8_t* data, size_t sz);

 private:
  StatsdBinderDataSource(const StatsdBinderDataSource&) = delete;
  StatsdBinderDataSource& operator=(const StatsdBinderDataSource&) = delete;

  base::TaskRunner* const task_runner_;
  std::unique_ptr<TraceWriter> writer_;
  std::string shell_subscription_;
  int32_t subscription_id_ = -1;
  std::function<void()> pending_flush_callback_;

  base::WeakPtrFactory<StatsdBinderDataSource> weak_factory_;  // Keep last.
};

}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_STATSD_CLIENT_STATSD_BINDER_DATA_SOURCE_H_
