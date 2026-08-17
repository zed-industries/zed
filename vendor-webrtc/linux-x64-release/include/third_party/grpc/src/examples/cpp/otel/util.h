//
//
// Copyright 2024 gRPC authors.
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

#ifndef GRPCPP_EXAMPLES_CPP_OTEL_UTIL_H
#define GRPCPP_EXAMPLES_CPP_OTEL_UTIL_H

#include <string>

#include "opentelemetry/sdk/metrics/meter_provider.h"

// Helper function that adds view for gRPC latency instrument \a name with unit
// \a unit with bucket boundaries that are more useful for RPCs.
void AddLatencyView(opentelemetry::sdk::metrics::MeterProvider* provider,
                    const std::string& name, const std::string& unit);

void RunServer(uint16_t port);
void RunXdsEnabledServer(uint16_t port);
void RunClient(const std::string& target_str);
#endif  // GRPCPP_EXAMPLES_CPP_OTEL_UTIL_H
