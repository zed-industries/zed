//
//
// Copyright 2015 gRPC authors.
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
//

#ifndef GRPC_SRC_CORE_LIB_TRANSPORT_TIMEOUT_ENCODING_H
#define GRPC_SRC_CORE_LIB_TRANSPORT_TIMEOUT_ENCODING_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <optional>

#include "src/core/lib/slice/slice.h"
#include "src/core/util/time.h"

namespace grpc_core {

class Timeout {
 public:
  static Timeout FromDuration(Duration duration);

  // Computes: 100 * ((this - other) / other)
  double RatioVersus(Timeout other) const;
  Slice Encode() const;
  Duration AsDuration() const;

 private:
  enum class Unit : uint8_t {
    kNanoseconds,
    kMilliseconds,
    kTenMilliseconds,
    kHundredMilliseconds,
    kSeconds,
    kTenSeconds,
    kHundredSeconds,
    kMinutes,
    kTenMinutes,
    kHundredMinutes,
    kHours,
  };

  Timeout(uint16_t value, Unit unit) : value_(value), unit_(unit) {}

  static Timeout FromMillis(int64_t millis);
  static Timeout FromSeconds(int64_t seconds);
  static Timeout FromMinutes(int64_t minutes);
  static Timeout FromHours(int64_t hours);

  uint16_t value_ = 0;
  Unit unit_ = Unit::kNanoseconds;
};

std::optional<Duration> ParseTimeout(const Slice& text);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_TRANSPORT_TIMEOUT_ENCODING_H
