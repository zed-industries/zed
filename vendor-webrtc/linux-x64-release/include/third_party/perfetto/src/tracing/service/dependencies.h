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

#ifndef SRC_TRACING_SERVICE_DEPENDENCIES_H_
#define SRC_TRACING_SERVICE_DEPENDENCIES_H_

#include <memory>

#include "src/tracing/service/clock.h"
#include "src/tracing/service/random.h"

namespace perfetto::tracing_service {

// Dependencies of TracingServiceImpl. Can point to real implementations or to
// mocks in tests.
struct Dependencies {
  std::unique_ptr<Clock> clock;
  std::unique_ptr<Random> random;
};

}  // namespace perfetto::tracing_service

#endif  // SRC_TRACING_SERVICE_DEPENDENCIES_H_
