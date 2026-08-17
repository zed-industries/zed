/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef SRC_TRACED_PROBES_PROBES_PRODUCER_H_
#define SRC_TRACED_PROBES_PROBES_PRODUCER_H_

#include <functional>
#include <memory>
#include <unordered_map>
#include <utility>

#include "perfetto/base/task_runner.h"
#include "perfetto/ext/base/watchdog.h"
#include "perfetto/ext/base/weak_ptr.h"
#include "perfetto/ext/tracing/core/producer.h"
#include "perfetto/ext/tracing/core/tracing_service.h"
#include "src/traced/probes/filesystem/lru_inode_cache.h"
#include "src/traced/probes/ftrace/ftrace_controller.h"
#include "src/traced/probes/ftrace/ftrace_metadata.h"
#include "src/traced/probes/probes_data_source.h"

namespace perfetto {

class ProbesDataSource;

const uint64_t kLRUInodeCacheSize = 1000;

class ProbesProducer : public Producer, public FtraceController::Observer {
 public:
  ProbesProducer();
  ~ProbesProducer() override;

  ProbesProducer(const ProbesProducer&) = delete;
  ProbesProducer& operator=(const ProbesProducer&) = delete;

  static ProbesProducer* GetInstance();

  // Producer Impl:
  void OnConnect() override;
  void OnDisconnect() override;
  void SetupDataSource(DataSourceInstanceID, const DataSourceConfig&) override;
  void StartDataSource(DataSourceInstanceID, const DataSourceConfig&) override;
  void StopDataSource(DataSourceInstanceID) override;
  void OnTracingSetup() override;
  void Flush(FlushRequestID,
             const DataSourceInstanceID* data_source_ids,
             size_t num_data_sources,
             FlushFlags) override;
  void ClearIncrementalState(const DataSourceInstanceID* data_source_ids,
                             size_t num_data_sources) override;

  // FtraceController::Observer implementation.
  void OnFtraceDataWrittenIntoDataSourceBuffers() override;

  // Our Impl
  void ConnectWithRetries(const char* socket_name,
                          base::TaskRunner* task_runner);

  // Constructs an instance of a data source of type T.
  template <typename T>
  std::unique_ptr<ProbesDataSource> CreateDSInstance(
      TracingSessionID session_id,
      const DataSourceConfig& config);

  void ActivateTrigger(std::string trigger);

  // Calls `cb` when all data sources have been registered.
  void SetAllDataSourcesRegisteredCb(std::function<void()> cb) {
    all_data_sources_registered_cb_ = std::move(cb);
  }

 private:
  static ProbesProducer* instance_;

  enum State {
    kNotStarted = 0,
    kNotConnected,
    kConnecting,
    kConnected,
  };

  void Connect();
  void Restart();
  void ResetConnectionBackoff();
  void IncreaseConnectionBackoff();
  void OnDataSourceFlushComplete(FlushRequestID, DataSourceInstanceID);
  void OnFlushTimeout(FlushRequestID);

  State state_ = kNotStarted;
  base::TaskRunner* task_runner_ = nullptr;
  std::unique_ptr<TracingService::ProducerEndpoint> endpoint_;
  std::unique_ptr<FtraceController> ftrace_;
  bool ftrace_creation_failed_ = false;
  uint32_t connection_backoff_ms_ = 0;
  const char* socket_name_ = nullptr;

  // Owning map for all active data sources.
  std::unordered_map<DataSourceInstanceID, std::unique_ptr<ProbesDataSource>>
      data_sources_;

  // Keeps (pointers to) data sources grouped by session id and data source
  // type. The pointers do not own the data sources (they're owned by
  // data_sources_).
  //
  // const ProbesDataSource::Descriptor* identifies the type.
  //
  // Used by OnFtraceDataWrittenIntoDataSourceBuffers().
  std::unordered_map<
      TracingSessionID,
      std::unordered_multimap<const ProbesDataSource::Descriptor*,
                              ProbesDataSource*>>
      session_data_sources_;

  std::unordered_multimap<FlushRequestID, DataSourceInstanceID>
      pending_flushes_;

  std::function<void()> all_data_sources_registered_cb_;

  std::unordered_map<DataSourceInstanceID, base::Watchdog::Timer> watchdogs_;
  LRUInodeCache cache_{kLRUInodeCacheSize};
  std::map<BlockDeviceID, std::unordered_map<Inode, InodeMapValue>>
      system_inodes_;

  base::WeakPtrFactory<ProbesProducer> weak_factory_;  // Keep last.
};

}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_PROBES_PRODUCER_H_
