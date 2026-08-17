//
// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_OUTLIER_DETECTION_OUTLIER_DETECTION_H
#define GRPC_SRC_CORE_LOAD_BALANCING_OUTLIER_DETECTION_OUTLIER_DETECTION_H

#include <grpc/support/port_platform.h>
#include <stdint.h>  // for uint32_t

#include <optional>

#include "src/core/util/json/json.h"
#include "src/core/util/json/json_args.h"
#include "src/core/util/json/json_object_loader.h"
#include "src/core/util/time.h"
#include "src/core/util/validation_errors.h"

namespace grpc_core {

struct OutlierDetectionConfig {
  Duration interval = Duration::Seconds(10);
  Duration base_ejection_time = Duration::Milliseconds(30000);
  Duration max_ejection_time = Duration::Milliseconds(30000);
  uint32_t max_ejection_percent = 10;
  struct SuccessRateEjection {
    uint32_t stdev_factor = 1900;
    uint32_t enforcement_percentage = 100;
    uint32_t minimum_hosts = 5;
    uint32_t request_volume = 100;

    SuccessRateEjection() {}

    bool operator==(const SuccessRateEjection& other) const {
      return stdev_factor == other.stdev_factor &&
             enforcement_percentage == other.enforcement_percentage &&
             minimum_hosts == other.minimum_hosts &&
             request_volume == other.request_volume;
    }

    static const JsonLoaderInterface* JsonLoader(const JsonArgs&);
    void JsonPostLoad(const Json&, const JsonArgs&, ValidationErrors* errors);
  };
  struct FailurePercentageEjection {
    uint32_t threshold = 85;
    uint32_t enforcement_percentage = 100;
    uint32_t minimum_hosts = 5;
    uint32_t request_volume = 50;

    FailurePercentageEjection() {}

    bool operator==(const FailurePercentageEjection& other) const {
      return threshold == other.threshold &&
             enforcement_percentage == other.enforcement_percentage &&
             minimum_hosts == other.minimum_hosts &&
             request_volume == other.request_volume;
    }

    static const JsonLoaderInterface* JsonLoader(const JsonArgs&);
    void JsonPostLoad(const Json&, const JsonArgs&, ValidationErrors* errors);
  };
  std::optional<SuccessRateEjection> success_rate_ejection;
  std::optional<FailurePercentageEjection> failure_percentage_ejection;

  bool operator==(const OutlierDetectionConfig& other) const {
    return interval == other.interval &&
           base_ejection_time == other.base_ejection_time &&
           max_ejection_time == other.max_ejection_time &&
           max_ejection_percent == other.max_ejection_percent &&
           success_rate_ejection == other.success_rate_ejection &&
           failure_percentage_ejection == other.failure_percentage_ejection;
  }

  static const JsonLoaderInterface* JsonLoader(const JsonArgs&);
  void JsonPostLoad(const Json& json, const JsonArgs&,
                    ValidationErrors* errors);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_OUTLIER_DETECTION_OUTLIER_DETECTION_H
