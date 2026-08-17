//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_FILTERS_LOAD_REPORTING_REGISTERED_OPENCENSUS_OBJECTS_H
#define GRPC_SRC_CORE_EXT_FILTERS_LOAD_REPORTING_REGISTERED_OPENCENSUS_OBJECTS_H

#include <grpc/support/port_platform.h>

#include "opencensus/stats/stats.h"
#include "opencensus/tags/tag_key.h"
#include "src/cpp/server/load_reporter/constants.h"

namespace grpc {
namespace load_reporter {

// Note that the functions here are specified as inline to share the static
// objects across all the translation units including this header. See more
// details on https://en.cppreference.com/w/cpp/language/inline.

// Measures.

inline ::opencensus::stats::MeasureInt64 MeasureStartCount() {
  static const ::opencensus::stats::MeasureInt64 measure =
      ::opencensus::stats::MeasureInt64::Register(
          kMeasureStartCount, kMeasureStartCount, kMeasureStartCount);
  return measure;
}

inline ::opencensus::stats::MeasureInt64 MeasureEndCount() {
  static const ::opencensus::stats::MeasureInt64 measure =
      ::opencensus::stats::MeasureInt64::Register(
          kMeasureEndCount, kMeasureEndCount, kMeasureEndCount);
  return measure;
}

inline ::opencensus::stats::MeasureInt64 MeasureEndBytesSent() {
  static const ::opencensus::stats::MeasureInt64 measure =
      ::opencensus::stats::MeasureInt64::Register(
          kMeasureEndBytesSent, kMeasureEndBytesSent, kMeasureEndBytesSent);
  return measure;
}

inline ::opencensus::stats::MeasureInt64 MeasureEndBytesReceived() {
  static const ::opencensus::stats::MeasureInt64 measure =
      ::opencensus::stats::MeasureInt64::Register(kMeasureEndBytesReceived,
                                                  kMeasureEndBytesReceived,
                                                  kMeasureEndBytesReceived);
  return measure;
}

inline ::opencensus::stats::MeasureInt64 MeasureEndLatencyMs() {
  static const ::opencensus::stats::MeasureInt64 measure =
      ::opencensus::stats::MeasureInt64::Register(
          kMeasureEndLatencyMs, kMeasureEndLatencyMs, kMeasureEndLatencyMs);
  return measure;
}

inline ::opencensus::stats::MeasureDouble MeasureOtherCallMetric() {
  static const ::opencensus::stats::MeasureDouble measure =
      ::opencensus::stats::MeasureDouble::Register(kMeasureOtherCallMetric,
                                                   kMeasureOtherCallMetric,
                                                   kMeasureOtherCallMetric);
  return measure;
}

// Tags.

inline ::opencensus::tags::TagKey TagKeyToken() {
  static const ::opencensus::tags::TagKey token =
      opencensus::tags::TagKey::Register(kTagKeyToken);
  return token;
}

inline ::opencensus::tags::TagKey TagKeyHost() {
  static const ::opencensus::tags::TagKey token =
      opencensus::tags::TagKey::Register(kTagKeyHost);
  return token;
}

inline ::opencensus::tags::TagKey TagKeyUserId() {
  static const ::opencensus::tags::TagKey token =
      opencensus::tags::TagKey::Register(kTagKeyUserId);
  return token;
}

inline ::opencensus::tags::TagKey TagKeyStatus() {
  static const ::opencensus::tags::TagKey token =
      opencensus::tags::TagKey::Register(kTagKeyStatus);
  return token;
}

inline ::opencensus::tags::TagKey TagKeyMetricName() {
  static const ::opencensus::tags::TagKey token =
      opencensus::tags::TagKey::Register(kTagKeyMetricName);
  return token;
}

}  // namespace load_reporter
}  // namespace grpc

#endif  // GRPC_SRC_CORE_EXT_FILTERS_LOAD_REPORTING_REGISTERED_OPENCENSUS_OBJECTS_H
