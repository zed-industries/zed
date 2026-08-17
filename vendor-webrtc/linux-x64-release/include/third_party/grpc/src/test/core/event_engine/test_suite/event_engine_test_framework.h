// Copyright 2021 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_TEST_CORE_EVENT_ENGINE_TEST_SUITE_EVENT_ENGINE_TEST_FRAMEWORK_H
#define GRPC_TEST_CORE_EVENT_ENGINE_TEST_SUITE_EVENT_ENGINE_TEST_FRAMEWORK_H
#include <grpc/event_engine/event_engine.h>

#include <memory>
#include <utility>

#include "absl/functional/any_invocable.h"
#include "absl/log/check.h"
#include "gtest/gtest.h"

extern absl::AnyInvocable<
    std::shared_ptr<grpc_event_engine::experimental::EventEngine>()>*
    g_ee_factory;

extern absl::AnyInvocable<
    std::shared_ptr<grpc_event_engine::experimental::EventEngine>()>*
    g_oracle_ee_factory;

// Manages the lifetime of the global EventEngine factory.
class EventEngineTestEnvironment : public testing::Environment {
 public:
  EventEngineTestEnvironment(
      absl::AnyInvocable<
          std::shared_ptr<grpc_event_engine::experimental::EventEngine>()>
          factory,
      absl::AnyInvocable<
          std::shared_ptr<grpc_event_engine::experimental::EventEngine>()>
          oracle_factory)
      : factory_(std::move(factory)),
        oracle_factory_(std::move(oracle_factory)) {}

  void SetUp() override {
    g_ee_factory = &factory_;
    g_oracle_ee_factory = &oracle_factory_;
  }

  void TearDown() override {
    g_ee_factory = nullptr;
    g_oracle_ee_factory = nullptr;
  }

 private:
  absl::AnyInvocable<
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>()>
      factory_;
  absl::AnyInvocable<
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>()>
      oracle_factory_;
};

class EventEngineTest : public testing::Test {
 protected:
  std::shared_ptr<grpc_event_engine::experimental::EventEngine>
  NewEventEngine() {
    CHECK_NE(g_ee_factory, nullptr);
    return (*g_ee_factory)();
  }

  std::shared_ptr<grpc_event_engine::experimental::EventEngine>
  NewOracleEventEngine() {
    CHECK_NE(g_oracle_ee_factory, nullptr);
    return (*g_oracle_ee_factory)();
  }
};

// Set a custom factory for the EventEngine test suite. An optional oracle
// EventEngine can additionally be specified here.
void SetEventEngineFactories(
    absl::AnyInvocable<
        std::shared_ptr<grpc_event_engine::experimental::EventEngine>()>
        ee_factory,
    absl::AnyInvocable<
        std::shared_ptr<grpc_event_engine::experimental::EventEngine>()>
        oracle_ee_factory);

#endif  // GRPC_TEST_CORE_EVENT_ENGINE_TEST_SUITE_EVENT_ENGINE_TEST_FRAMEWORK_H
