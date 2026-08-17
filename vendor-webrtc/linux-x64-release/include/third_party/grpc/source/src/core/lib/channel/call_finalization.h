// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_CHANNEL_CALL_FINALIZATION_H
#define GRPC_SRC_CORE_LIB_CHANNEL_CALL_FINALIZATION_H

#include <grpc/support/port_platform.h>

#include <utility>

#include "src/core/lib/promise/context.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/transport/call_final_info.h"

namespace grpc_core {

// Call finalization context.
// Sometimes a filter needs to perform some operation after the last byte of
// data is flushed to the wire. This context is used to perform that
// finalization.
// Filters can register a finalizer by calling Add().
// The finalizer will be called before the call is destroyed but after
// the top level promise is completed.
class CallFinalization {
 public:
  // Add a step to the finalization context.
  // Takes a callable with a signature compatible with:
  // (const grpc_call_final_info*) -> void.
  // Finalizers are run in the reverse order they are added.
  template <typename F>
  void Add(F&& t) {
    first_ =
        GetContext<Arena>()->New<FuncFinalizer<F>>(std::forward<F>(t), first_);
  }

  void Run(const grpc_call_final_info* final_info) {
    if (Finalizer* f = std::exchange(first_, nullptr)) f->Run(final_info);
  }

 private:
  // Base class for finalizer implementations.
  class Finalizer {
   public:
    // Run the finalizer and call the destructor of this Finalizer.
    virtual void Run(const grpc_call_final_info* final_info) = 0;

   protected:
    ~Finalizer() {}
  };
  // Specialization for callable objects.
  template <typename F>
  class FuncFinalizer final : public Finalizer {
   public:
    FuncFinalizer(F&& f, Finalizer* next)
        : next_(next), f_(std::forward<F>(f)) {}

    void Run(const grpc_call_final_info* final_info) override {
      f_(final_info);
      Finalizer* next = next_;
      this->~FuncFinalizer();
      if (next != nullptr) next->Run(final_info);
    }

   private:
    Finalizer* next_;
    F f_;
  };
  // The first finalizer in the chain.
  Finalizer* first_ = nullptr;
};

template <>
struct ContextType<CallFinalization> {};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_CHANNEL_CALL_FINALIZATION_H
