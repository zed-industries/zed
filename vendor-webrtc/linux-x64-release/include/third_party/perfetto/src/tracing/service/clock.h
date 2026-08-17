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

#ifndef SRC_TRACING_SERVICE_CLOCK_H_
#define SRC_TRACING_SERVICE_CLOCK_H_

#include "perfetto/base/time.h"

namespace perfetto::tracing_service {

class Clock {
 public:
  virtual ~Clock();
  virtual base::TimeNanos GetBootTimeNs() = 0;
  virtual base::TimeNanos GetWallTimeNs() = 0;

  base::TimeMillis GetBootTimeMs() {
    return std::chrono::duration_cast<base::TimeMillis>(GetBootTimeNs());
  }
  base::TimeMillis GetWallTimeMs() {
    return std::chrono::duration_cast<base::TimeMillis>(GetWallTimeNs());
  }

  base::TimeSeconds GetBootTimeS() {
    return std::chrono::duration_cast<base::TimeSeconds>(GetBootTimeNs());
  }
  base::TimeSeconds GetWallTimeS() {
    return std::chrono::duration_cast<base::TimeSeconds>(GetWallTimeNs());
  }
};

class ClockImpl : public Clock {
 public:
  ~ClockImpl() override;
  base::TimeNanos GetBootTimeNs() override;
  base::TimeNanos GetWallTimeNs() override;
};

}  // namespace perfetto::tracing_service

#endif  // SRC_TRACING_SERVICE_CLOCK_H_
