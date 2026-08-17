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

#ifndef GRPC_TEST_CPP_QPS_INTERARRIVAL_H
#define GRPC_TEST_CPP_QPS_INTERARRIVAL_H

#include <grpcpp/support/config.h>

#include <chrono>
#include <cmath>
#include <random>
#include <vector>

namespace grpc {
namespace testing {

// First create classes that define a random distribution
// Note that this code does not include C++-specific random distribution
// features supported in std::random. Although this would make this code easier,
// this code is required to serve as the template code for other language
// stacks. Thus, this code only uses a uniform distribution of doubles [0,1)
// and then provides the distribution functions itself.

class RandomDistInterface {
 public:
  RandomDistInterface() {}
  virtual ~RandomDistInterface() = 0;
  // Argument to transform is a uniform double in the range [0,1)
  virtual double transform(double uni) const = 0;
};

inline RandomDistInterface::~RandomDistInterface() {}

// ExpDist implements an exponential distribution, which is the
// interarrival distribution for a Poisson process. The parameter
// lambda is the mean rate of arrivals. This is the
// most useful distribution since it is actually additive and
// memoryless. It is a good representation of activity coming in from
// independent identical stationary sources. For more information,
// see http://en.wikipedia.org/wiki/Exponential_distribution

class ExpDist final : public RandomDistInterface {
 public:
  explicit ExpDist(double lambda) : lambda_recip_(1.0 / lambda) {}
  ~ExpDist() override {}
  double transform(double uni) const override {
    // Note: Use 1.0-uni above to avoid NaN if uni is 0
    return lambda_recip_ * (-log(1.0 - uni));
  }

 private:
  double lambda_recip_;
};

// A class library for generating pseudo-random interarrival times
// in an efficient re-entrant way. The random table is built at construction
// time, and each call must include the thread id of the invoker

class InterarrivalTimer {
 public:
  InterarrivalTimer() {}
  void init(const RandomDistInterface& r, int threads, int entries = 1000000) {
    std::random_device devrand;
    std::mt19937_64 generator(devrand());
    std::uniform_real_distribution<double> rando(0, 1);
    for (int i = 0; i < entries; i++) {
      random_table_.push_back(
          static_cast<int64_t>(1e9 * r.transform(rando(generator))));
    }
    // Now set up the thread positions
    for (int i = 0; i < threads; i++) {
      thread_posns_.push_back(random_table_.begin() + (entries * i) / threads);
    }
  }
  virtual ~InterarrivalTimer() {};

  int64_t next(int thread_num) {
    auto ret = *(thread_posns_[thread_num]++);
    if (thread_posns_[thread_num] == random_table_.end()) {
      thread_posns_[thread_num] = random_table_.begin();
    }
    return ret;
  }

 private:
  typedef std::vector<int64_t> time_table;
  std::vector<time_table::const_iterator> thread_posns_;
  time_table random_table_;
};
}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_QPS_INTERARRIVAL_H
