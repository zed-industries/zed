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

#ifndef SRC_TRACING_SERVICE_RANDOM_H_
#define SRC_TRACING_SERVICE_RANDOM_H_

#include <stdint.h>

#include <random>

namespace perfetto::tracing_service {

class Random {
 public:
  virtual ~Random();
  virtual double GetValue() = 0;
};

class RandomImpl : public Random {
 public:
  explicit RandomImpl(uint32_t seed);
  ~RandomImpl() override;
  double GetValue() override;

 private:
  std::minstd_rand prng_;
  std::uniform_real_distribution<double> dist_;
};

}  // namespace perfetto::tracing_service

#endif  // SRC_TRACING_SERVICE_RANDOM_H_
