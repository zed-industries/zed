/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACING_TEST_PROXY_PRODUCER_ENDPOINT_H_
#define SRC_TRACING_TEST_PROXY_PRODUCER_ENDPOINT_H_

#include "perfetto/ext/tracing/core/tracing_service.h"

namespace perfetto {

// A "proxy" ProducerEndpoint that forwards all the requests to a real
// (`backend_`) ProducerEndpoint endpoint or drops them if (`backend_`) is
// nullptr.
class ProxyProducerEndpoint : public ProducerEndpoint {
 public:
  ~ProxyProducerEndpoint() override;

  // `backend` is not owned.
  void set_backend(ProducerEndpoint* backend) { backend_ = backend; }

  ProducerEndpoint* backend() const { return backend_; }

  // Begin ProducerEndpoint implementation
  void Disconnect() override;
  void RegisterDataSource(const DataSourceDescriptor&) override;
  void UpdateDataSource(const DataSourceDescriptor&) override;
  void UnregisterDataSource(const std::string& name) override;
  void RegisterTraceWriter(uint32_t writer_id, uint32_t target_buffer) override;
  void UnregisterTraceWriter(uint32_t writer_id) override;
  void CommitData(const CommitDataRequest&,
                  CommitDataCallback callback = {}) override;
  SharedMemory* shared_memory() const override;
  size_t shared_buffer_page_size_kb() const override;
  std::unique_ptr<TraceWriter> CreateTraceWriter(
      BufferID target_buffer,
      BufferExhaustedPolicy buffer_exhausted_policy) override;
  SharedMemoryArbiter* MaybeSharedMemoryArbiter() override;
  bool IsShmemProvidedByProducer() const override;
  void NotifyFlushComplete(FlushRequestID) override;
  void NotifyDataSourceStarted(DataSourceInstanceID) override;
  void NotifyDataSourceStopped(DataSourceInstanceID) override;
  void ActivateTriggers(const std::vector<std::string>&) override;
  void Sync(std::function<void()> callback) override;
  // End ProducerEndpoint implementation

 private:
  ProducerEndpoint* backend_ = nullptr;
};

}  // namespace perfetto

#endif  // SRC_TRACING_TEST_PROXY_PRODUCER_ENDPOINT_H_
