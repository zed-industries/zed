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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_INTERCEPTOR_LIST_H
#define GRPC_SRC_CORE_LIB_PROMISE_INTERCEPTOR_LIST_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <algorithm>
#include <new>
#include <optional>
#include <string>
#include <utility>

#include "absl/log/check.h"
#include "absl/log/log.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "src/core/lib/promise/context.h"
#include "src/core/lib/promise/detail/promise_factory.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/util/construct_destruct.h"
#include "src/core/util/debug_location.h"

namespace grpc_core {

// Tracks a list of maps of T -> optional<T> via promises.
// When Run, runs each transformation in order, and resolves to the ulimate
// result.
// If a map resolves to nullopt, the chain is terminated and the result is
// nullopt.
// Maps can also have synchronous cleanup functions, which are guaranteed to be
// called at the termination of each run through the chain.
template <typename T>
class InterceptorList {
 private:
  // A map of T -> T via promises.
  class Map {
   public:
    explicit Map(DebugLocation from) : from_(from) {}
    // Construct a promise to transform x into some other value at memory.
    virtual void MakePromise(T x, void* memory) = 0;
    // Destroy a promise constructed at memory.
    virtual void Destroy(void* memory) = 0;
    // Poll a promise constructed at memory.
    // Resolves to an optional<T> -- if nullopt it means terminate the chain and
    // resolve.
    virtual Poll<std::optional<T>> PollOnce(void* memory) = 0;
    virtual ~Map() = default;

    // Update the next pointer stored with this map.
    // This is only valid to call once, and only before the map is used.
    void SetNext(Map* next) {
      DCHECK_EQ(next_, nullptr);
      next_ = next;
    }

    // Access the creation location for this map (for debug tracing).
    DebugLocation from() const { return from_; }

    // Access the next map in the chain (or nullptr if this is the last map).
    Map* next() const { return next_; }

   private:
    GPR_NO_UNIQUE_ADDRESS const DebugLocation from_;
    Map* next_ = nullptr;
  };

 public:
  // The result of Run: a promise that will execute the entire chain.
  class RunPromise {
   public:
    RunPromise(size_t memory_required, Map** factory, std::optional<T> value) {
      if (!value.has_value() || *factory == nullptr) {
        GRPC_TRACE_VLOG(promise_primitives, 2)
            << "InterceptorList::RunPromise[" << this << "]: create immediate";
        is_immediately_resolved_ = true;
        Construct(&result_, std::move(value));
      } else {
        is_immediately_resolved_ = false;
        Construct(&async_resolution_, memory_required);
        (*factory)->MakePromise(std::move(*value),
                                async_resolution_.space.get());
        async_resolution_.current_factory = *factory;
        async_resolution_.first_factory = factory;
        GRPC_TRACE_VLOG(promise_primitives, 2)
            << "InterceptorList::RunPromise[" << this
            << "]: create async; mem=" << async_resolution_.space.get();
      }
    }

    ~RunPromise() {
      GRPC_TRACE_VLOG(promise_primitives, 2)
          << "InterceptorList::RunPromise[" << this << "]: destroy";
      if (is_immediately_resolved_) {
        Destruct(&result_);
      } else {
        if (async_resolution_.current_factory != nullptr) {
          async_resolution_.current_factory->Destroy(
              async_resolution_.space.get());
        }
        Destruct(&async_resolution_);
      }
    }

    RunPromise(const RunPromise&) = delete;
    RunPromise& operator=(const RunPromise&) = delete;

    RunPromise(RunPromise&& other) noexcept
        : is_immediately_resolved_(other.is_immediately_resolved_) {
      GRPC_TRACE_VLOG(promise_primitives, 2)
          << "InterceptorList::RunPromise[" << this << "]: move from "
          << &other;
      if (is_immediately_resolved_) {
        Construct(&result_, std::move(other.result_));
      } else {
        Construct(&async_resolution_, std::move(other.async_resolution_));
      }
    }

    RunPromise& operator=(RunPromise&& other) noexcept = delete;

    Poll<std::optional<T>> operator()() {
      GRPC_TRACE_VLOG(promise_primitives, 2)
          << "InterceptorList::RunPromise[" << this << "]: " << DebugString();
      if (is_immediately_resolved_) return std::move(result_);
      while (true) {
        if (*async_resolution_.first_factory == nullptr) {
          // Cancelled whilst polling
          return std::nullopt;
        }
        auto r = async_resolution_.current_factory->PollOnce(
            async_resolution_.space.get());
        if (auto* p = r.value_if_ready()) {
          async_resolution_.current_factory->Destroy(
              async_resolution_.space.get());
          async_resolution_.current_factory =
              async_resolution_.current_factory->next();
          if (!p->has_value()) async_resolution_.current_factory = nullptr;
          GRPC_TRACE_VLOG(promise_primitives, 2)
              << "InterceptorList::RunPromise[" << this
              << "]: " << DebugString();
          if (async_resolution_.current_factory == nullptr) {
            return std::move(*p);
          }
          async_resolution_.current_factory->MakePromise(
              std::move(**p), async_resolution_.space.get());
          continue;
        }
        return Pending{};
      }
    }

   private:
    std::string DebugString() const {
      if (is_immediately_resolved_) {
        return absl::StrFormat("Result:has_value:%d", result_.has_value());
      } else {
        return absl::StrCat(
            "Running:",
            async_resolution_.current_factory == nullptr
                ? "END"
                : ([p = async_resolution_.current_factory->from()]() {
                    return absl::StrCat(p.file(), ":", p.line());
                  })()
                      .c_str());
      }
    }
    struct AsyncResolution {
      explicit AsyncResolution(size_t max_size)
          : space(GetContext<Arena>()->MakePooledArray<char>(max_size)) {}
      AsyncResolution(const AsyncResolution&) = delete;
      AsyncResolution& operator=(const AsyncResolution&) = delete;
      AsyncResolution(AsyncResolution&& other) noexcept
          : current_factory(std::exchange(other.current_factory, nullptr)),
            first_factory(std::exchange(other.first_factory, nullptr)),
            space(std::move(other.space)) {}
      Map* current_factory;
      Map** first_factory;
      Arena::PoolPtr<char[]> space;
    };
    union {
      AsyncResolution async_resolution_;
      std::optional<T> result_;
    };
    // If true, the result_ union is valid, otherwise async_resolution_ is.
    // Indicates whether the promise resolved immediately at construction or if
    // additional steps were needed.
    bool is_immediately_resolved_;
  };

  InterceptorList() = default;
  InterceptorList(const InterceptorList&) = delete;
  InterceptorList& operator=(const InterceptorList&) = delete;
  ~InterceptorList() { DeleteFactories(); }

  RunPromise Run(std::optional<T> initial_value) {
    return RunPromise(promise_memory_required_, &first_map_,
                      std::move(initial_value));
  }

  // Append a new map to the end of the chain.
  template <typename Fn>
  void AppendMap(Fn fn, DebugLocation from) {
    Append(MakeMapToAdd(std::move(fn), [] {}, from));
  }

  // Prepend a new map to the beginning of the chain.
  template <typename Fn>
  void PrependMap(Fn fn, DebugLocation from) {
    Prepend(MakeMapToAdd(std::move(fn), [] {}, from));
  }

  // Append a new map to the end of the chain, with a cleanup function to be
  // called at the end of run promise execution.
  template <typename Fn, typename CleanupFn>
  void AppendMapWithCleanup(Fn fn, CleanupFn cleanup_fn, DebugLocation from) {
    Append(MakeMapToAdd(std::move(fn), std::move(cleanup_fn), from));
  }

  // Prepend a new map to the beginning of the chain, with a cleanup function to
  // be called at the end of run promise execution.
  template <typename Fn, typename CleanupFn>
  void PrependMapWithCleanup(Fn fn, CleanupFn cleanup_fn, DebugLocation from) {
    Prepend(MakeMapToAdd(std::move(fn), std::move(cleanup_fn), from));
  }

 protected:
  // Clear the interceptor list
  void ResetInterceptorList() {
    DeleteFactories();
    first_map_ = nullptr;
    last_map_ = nullptr;
    promise_memory_required_ = 0;
  }

 private:
  template <typename Fn, typename CleanupFn>
  class MapImpl final : public Map {
   public:
    using PromiseFactory = promise_detail::RepeatedPromiseFactory<T, Fn>;
    using Promise = typename PromiseFactory::Promise;

    explicit MapImpl(Fn fn, CleanupFn cleanup_fn, DebugLocation from)
        : Map(from), fn_(std::move(fn)), cleanup_fn_(std::move(cleanup_fn)) {}
    ~MapImpl() override { cleanup_fn_(); }
    void MakePromise(T x, void* memory) override {
      new (memory) Promise(fn_.Make(std::move(x)));
    }
    void Destroy(void* memory) override {
      static_cast<Promise*>(memory)->~Promise();
    }
    Poll<std::optional<T>> PollOnce(void* memory) override {
      return poll_cast<std::optional<T>>((*static_cast<Promise*>(memory))());
    }

   private:
    GPR_NO_UNIQUE_ADDRESS PromiseFactory fn_;
    GPR_NO_UNIQUE_ADDRESS CleanupFn cleanup_fn_;
  };

  template <typename Fn, typename CleanupFn>
  Map* MakeMapToAdd(Fn fn, CleanupFn cleanup_fn, DebugLocation from) {
    using FactoryType = MapImpl<Fn, CleanupFn>;
    promise_memory_required_ = std::max(promise_memory_required_,
                                        sizeof(typename FactoryType::Promise));
    return GetContext<Arena>()->New<FactoryType>(std::move(fn),
                                                 std::move(cleanup_fn), from);
  }

  void Append(Map* f) {
    if (first_map_ == nullptr) {
      first_map_ = f;
      last_map_ = f;
    } else {
      last_map_->SetNext(f);
      last_map_ = f;
    }
  }

  void Prepend(Map* f) {
    if (first_map_ == nullptr) {
      first_map_ = f;
      last_map_ = f;
    } else {
      f->SetNext(first_map_);
      first_map_ = f;
    }
  }

  void DeleteFactories() {
    for (auto* f = first_map_; f != nullptr;) {
      auto* next = f->next();
      f->~Map();
      f = next;
    }
  }

  // The first map in the chain.
  Map* first_map_ = nullptr;
  // The last map in the chain.
  Map* last_map_ = nullptr;
  // The amount of memory required to store the largest promise in the chain.
  size_t promise_memory_required_ = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_INTERCEPTOR_LIST_H
