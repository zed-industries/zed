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

#ifndef MEDIAPIPE_DEPS_THREAD_OPTIONS_H_
#define MEDIAPIPE_DEPS_THREAD_OPTIONS_H_

#include <stddef.h>

#include <set>
#include <string>

namespace mediapipe {

// Options to configure a thread.  Default values are listed in
// the field descriptions.
class ThreadOptions {
 public:
  ThreadOptions() : stack_size_(0), nice_priority_level_(0) {}

  // Set the thread stack size (in bytes).  Passing stack_size==0 resets
  // the stack size to the default value for the system. The system default
  // is also the default for this class.
  ThreadOptions& set_stack_size(size_t stack_size) {
    stack_size_ = stack_size;
    return *this;
  }

  ThreadOptions& set_nice_priority_level(int nice_priority_level) {
    nice_priority_level_ = nice_priority_level;
    return *this;
  }

  ThreadOptions& set_cpu_set(const std::set<int>& cpu_set) {
    cpu_set_ = cpu_set;
    return *this;
  }

  ThreadOptions& set_name_prefix(const std::string& name_prefix) {
    name_prefix_ = name_prefix;
    return *this;
  }

  size_t stack_size() const { return stack_size_; }

  int nice_priority_level() const { return nice_priority_level_; }

  const std::set<int>& cpu_set() const { return cpu_set_; }

  std::string name_prefix() const { return name_prefix_; }

 private:
  size_t stack_size_;        // Size of thread stack
  int nice_priority_level_;  // Nice priority level of the workers
  std::set<int> cpu_set_;    // CPU set for affinity setting
  std::string name_prefix_;  // Name of the thread
};

}  // namespace mediapipe
#endif  // MEDIAPIPE_DEPS_THREAD_OPTIONS_H_
