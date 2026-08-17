//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_TEST_CORE_TEST_UTIL_SOCKET_USE_AFTER_CLOSE_DETECTOR_H
#define GRPC_TEST_CORE_TEST_UTIL_SOCKET_USE_AFTER_CLOSE_DETECTOR_H

#include <grpc/support/port_platform.h>
#include <grpc/support/sync_generic.h>

#include <memory>
#include <thread>

namespace grpc_core {
namespace testing {

// This class is meant to detect file descriptor use-after-close
// bugs occurring somewhere in the program while the object is in live.
// The implementation currently uses a background thread to open
// and close sockets in a loop, catching socket use-after-close bugs
// by watching them manifest as unexpected socket operation failures.
//
// Note: this will not give false positives but may give false negatives.
// That said this seems to be fairly reliable at finding use-after-close
// bugs, at least on linux, because of fd handles being quickly reused.
// For example this was able to catch the use-after-close bug from
// https://github.com/grpc/grpc/pull/33871 "almost every time".
class SocketUseAfterCloseDetector {
 public:
  SocketUseAfterCloseDetector();
  ~SocketUseAfterCloseDetector();

 private:
  std::unique_ptr<std::thread> thread_;
  gpr_event done_ev_;
};

}  // namespace testing
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TEST_UTIL_SOCKET_USE_AFTER_CLOSE_DETECTOR_H
