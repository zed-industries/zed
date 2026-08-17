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

#ifndef GRPC_TEST_CPP_QPS_USAGE_TIMER_H
#define GRPC_TEST_CPP_QPS_USAGE_TIMER_H

class UsageTimer {
 public:
  UsageTimer();

  struct Result {
    double wall;
    double user;
    double system;
    unsigned long long total_cpu_time;
    unsigned long long idle_cpu_time;
  };

  Result Mark() const;

  static double Now();

 private:
  static Result Sample();

  const Result start_;
};

#endif  // GRPC_TEST_CPP_QPS_USAGE_TIMER_H
