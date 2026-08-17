// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_COUNTER_FACTORY_H_
#define MEDIAPIPE_FRAMEWORK_COUNTER_FACTORY_H_

#include <algorithm>
#include <cstdint>
#include <map>
#include <memory>
#include <string>

#include "absl/base/thread_annotations.h"
#include "absl/synchronization/mutex.h"
#include "absl/time/time.h"
#include "mediapipe/framework/counter.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/map_util.h"

namespace mediapipe {

// Holds a map of counter names to counter unique_ptrs.
// This class is thread safe.
class CounterSet {
 public:
  CounterSet();

  // In builds with streamz export enabled, this will synchronously export
  // the final counter values.
  ~CounterSet();
  // Prints the values of all the counters.
  // A call to PublishCounters will reset all counters.
  void PrintCounters();
  // Publishes the vales of all the counters for monitoring and resets
  // all internal counters.
  void PublishCounters();

  // Adds a counter of the given type by constructing the counter in place.
  // Returns a pointer to the new counter or if the counter already exists
  // to the existing pointer.
  template <typename CounterType, typename... Args>
  Counter* Emplace(const std::string& name, Args&&... args)
      ABSL_LOCKS_EXCLUDED(mu_) {
    absl::WriterMutexLock lock(&mu_);
    std::unique_ptr<Counter>* existing_counter = FindOrNull(counters_, name);
    if (existing_counter) {
      return existing_counter->get();
    }
    Counter* counter = new CounterType(std::forward<Args>(args)...);
    counters_[name].reset(counter);
    return counter;
  }
  // Retrieves the counter with the given name; return nullptr if it doesn't
  // exist.
  Counter* Get(const std::string& name);

  // Retrieves all counters names and current values from the internal map.
  std::map<std::string, int64_t> GetCountersValues() ABSL_LOCKS_EXCLUDED(mu_);

 private:
  absl::Mutex mu_;
  std::map<std::string, std::unique_ptr<Counter>> counters_
      ABSL_GUARDED_BY(mu_);
};

// Generic counter factory
class CounterFactory {
 public:
  virtual ~CounterFactory() {}
  virtual Counter* GetCounter(const std::string& name) = 0;
  CounterSet* GetCounterSet() { return &counter_set_; }

 protected:
  CounterSet counter_set_;
};

// Counter factory that makes the counters be our own basic counters.
class BasicCounterFactory : public CounterFactory {
 public:
  ~BasicCounterFactory() override {}
  Counter* GetCounter(const std::string& name) override;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_COUNTER_FACTORY_H_
