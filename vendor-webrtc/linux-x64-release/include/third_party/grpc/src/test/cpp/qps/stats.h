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

#ifndef GRPC_TEST_CPP_QPS_STATS_H
#define GRPC_TEST_CPP_QPS_STATS_H

#include <string>

#include "test/cpp/qps/histogram.h"

namespace grpc {
namespace testing {

template <class T, class F>
double sum(const T& container, F functor) {
  double r = 0;
  for (auto v = container.begin(); v != container.end(); v++) {
    r += functor(*v);
  }
  return r;
}

template <class T, class F>
double average(const T& container, F functor) {
  return sum(container, functor) / container.size();
}

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_QPS_STATS_H
