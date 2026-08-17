// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_DETAIL_PROMISE_VARIANT_H
#define GRPC_SRC_CORE_LIB_PROMISE_DETAIL_PROMISE_VARIANT_H

#include <variant>

namespace grpc_core {

namespace promise_detail {

// Visitor function for PromiseVariant - calls the poll operator on the inner
// type
class PollVisitor {
 public:
  template <typename T>
  auto operator()(T& x) {
    return x();
  }
};

// Helper type - given a variant V, provides the poll operator (which simply
// visits the inner type on the variant with PollVisitor)
template <typename V>
class PromiseVariant {
 public:
  explicit PromiseVariant(V variant) : variant_(std::move(variant)) {}
  auto operator()() { return std::visit(PollVisitor(), variant_); }

 private:
  V variant_;
};

}  // namespace promise_detail

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_DETAIL_PROMISE_VARIANT_H
