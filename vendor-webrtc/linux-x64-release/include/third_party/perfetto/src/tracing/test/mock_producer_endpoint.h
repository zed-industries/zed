/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACING_TEST_MOCK_PRODUCER_ENDPOINT_H_
#define SRC_TRACING_TEST_MOCK_PRODUCER_ENDPOINT_H_

#include "perfetto/ext/tracing/core/tracing_service.h"
#include "protos/perfetto/common/data_source_descriptor.gen.h"
#include "test/gtest_and_gmock.h"

namespace perfetto {

class MockProducerEndpoint : public TracingService::ProducerEndpoint {
 public:
  MOCK_METHOD(void, Disconnect, (), (override));
  MOCK_METHOD(void,
              RegisterDataSource,
              (const DataSourceDescriptor&),
              (override));
  MOCK_METHOD(void,
              UpdateDataSource,
              (const DataSourceDescriptor&),
              (override));
  MOCK_METHOD(void, UnregisterDataSource, (const std::string&), (override));
  MOCK_METHOD(void, RegisterTraceWriter, (uint32_t, uint32_t), (override));
  MOCK_METHOD(void, UnregisterTraceWriter, (uint32_t), (override));
  MOCK_METHOD(void,
              CommitData,
              (const CommitDataRequest&, CommitDataCallback),
              (override));
  MOCK_METHOD(SharedMemory*, shared_memory, (), (const, override));
  MOCK_METHOD(size_t, shared_buffer_page_size_kb, (), (const, override));
  MOCK_METHOD(std::unique_ptr<TraceWriter>,
              CreateTraceWriter,
              (BufferID, BufferExhaustedPolicy),
              (override));
  MOCK_METHOD(SharedMemoryArbiter*, MaybeSharedMemoryArbiter, (), (override));
  MOCK_METHOD(bool, IsShmemProvidedByProducer, (), (const, override));
  MOCK_METHOD(void, NotifyFlushComplete, (FlushRequestID), (override));
  MOCK_METHOD(void,
              NotifyDataSourceStarted,
              (DataSourceInstanceID),
              (override));
  MOCK_METHOD(void,
              NotifyDataSourceStopped,
              (DataSourceInstanceID),
              (override));
  MOCK_METHOD(void,
              ActivateTriggers,
              (const std::vector<std::string>&),
              (override));
  MOCK_METHOD(void, Sync, (std::function<void()>), (override));
};

}  // namespace perfetto

#endif  // SRC_TRACING_TEST_MOCK_PRODUCER_ENDPOINT_H_
