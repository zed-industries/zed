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

#ifndef GRPCPP_IMPL_CALL_HOOK_H
#define GRPCPP_IMPL_CALL_HOOK_H

namespace grpc {

namespace internal {
class CallOpSetInterface;
class Call;

/// This is an interface that Channel and Server implement to allow them to hook
/// performing ops.
class CallHook {
 public:
  virtual ~CallHook() {}
  virtual void PerformOpsOnCall(CallOpSetInterface* ops, Call* call) = 0;
};
}  // namespace internal

}  // namespace grpc

#endif  // GRPCPP_IMPL_CALL_HOOK_H
