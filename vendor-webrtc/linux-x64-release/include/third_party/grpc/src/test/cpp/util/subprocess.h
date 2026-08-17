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

#ifndef GRPC_TEST_CPP_UTIL_SUBPROCESS_H
#define GRPC_TEST_CPP_UTIL_SUBPROCESS_H

#include <initializer_list>
#include <string>
#include <vector>

struct gpr_subprocess;

namespace grpc {

class SubProcess {
 public:
  explicit SubProcess(const std::vector<std::string>& args);
  ~SubProcess();

  int Join();
  void Interrupt();

 private:
  SubProcess(const SubProcess& other);
  SubProcess& operator=(const SubProcess& other);

  gpr_subprocess* const subprocess_;
};

}  // namespace grpc

#endif  // GRPC_TEST_CPP_UTIL_SUBPROCESS_H
