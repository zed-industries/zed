//
// Copyright 2023 gRPC authors.
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
//

#ifndef GRPC_TEST_CORE_TEST_UTIL_AUDIT_LOGGING_UTILS_H
#define GRPC_TEST_CORE_TEST_UTIL_AUDIT_LOGGING_UTILS_H

#include <grpc/grpc_audit_logging.h>
#include <grpc/support/json.h>
#include <grpc/support/port_platform.h>

#include <memory>
#include <string>
#include <vector>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"

namespace grpc_core {
namespace testing {

class TestAuditLogger : public experimental::AuditLogger {
 public:
  explicit TestAuditLogger(std::vector<std::string>* audit_logs)
      : audit_logs_(audit_logs) {}

  absl::string_view name() const override;
  void Log(const experimental::AuditContext& context) override;

 private:
  std::vector<std::string>* audit_logs_;
};

class TestAuditLoggerFactory : public experimental::AuditLoggerFactory {
 public:
  class Config : public AuditLoggerFactory::Config {
    absl::string_view name() const override;
    std::string ToString() const override { return "{}"; }
  };

  explicit TestAuditLoggerFactory(std::vector<std::string>* audit_logs)
      : audit_logs_(audit_logs) {}

  absl::string_view name() const override;
  absl::StatusOr<std::unique_ptr<AuditLoggerFactory::Config>>
  ParseAuditLoggerConfig(const experimental::Json&) override;
  std::unique_ptr<experimental::AuditLogger> CreateAuditLogger(
      std::unique_ptr<AuditLoggerFactory::Config>) override;

 private:
  std::vector<std::string>* audit_logs_;
};

}  // namespace testing
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TEST_UTIL_AUDIT_LOGGING_UTILS_H
