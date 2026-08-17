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

#ifndef INCLUDE_PERFETTO_EXT_TRACING_CORE_NULL_CONSUMER_ENDPOINT_FOR_TESTING_H_
#define INCLUDE_PERFETTO_EXT_TRACING_CORE_NULL_CONSUMER_ENDPOINT_FOR_TESTING_H_

#include "perfetto/ext/tracing/core/tracing_service.h"

namespace perfetto {

// An empty implemetation of ConsumerEndpoint. This is used only to handle
// code in other repos like arctraceservice/PerfettoClient_test.cpp which
// ended up depending on internal perfetto interfaces against our plans.
// This allows to make changes to the ConsumerEndpoint without requiring 3way
// patches when touching methods that are not overridden by other projects.
class NullConsumerEndpointForTesting : public ConsumerEndpoint {
 public:
  ~NullConsumerEndpointForTesting() override {}

  void EnableTracing(const TraceConfig&, base::ScopedFile) override {}
  void ChangeTraceConfig(const perfetto::TraceConfig&) override {}
  void StartTracing() override {}
  void DisableTracing() override {}
  void CloneSession(CloneSessionArgs) override {}
  void Flush(uint32_t, FlushCallback, FlushFlags) override {}
  void ReadBuffers() override {}
  void FreeBuffers() override {}
  void Detach(const std::string&) override {}
  void Attach(const std::string&) override {}
  void GetTraceStats() override {}
  void ObserveEvents(uint32_t) override {}
  void QueryServiceState(ConsumerEndpoint::QueryServiceStateArgs,
                         ConsumerEndpoint::QueryServiceStateCallback) override {
  }
  void QueryCapabilities(ConsumerEndpoint::QueryCapabilitiesCallback) override {
  }
  void SaveTraceForBugreport(
      ConsumerEndpoint::SaveTraceForBugreportCallback) override {}
};

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_TRACING_CORE_NULL_CONSUMER_ENDPOINT_FOR_TESTING_H_
