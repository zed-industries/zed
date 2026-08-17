// Copyright 2025 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CHANNELZ_ZTRACE_COLLECTOR_H
#define GRPC_SRC_CORE_CHANNELZ_ZTRACE_COLLECTOR_H

#include <grpc/support/time.h>

#include <memory>
#include <tuple>
#include <vector>

#include "absl/container/flat_hash_set.h"
#include "src/core/channelz/channelz.h"
#include "src/core/util/single_set_ptr.h"
#include "src/core/util/string.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"

#ifdef GRPC_NO_ZTRACE
namespace grpc_core::channelz {
namespace ztrace_collector_detail {
class ZTraceImpl final : public ZTrace {
 public:
  explicit ZTraceImpl() {}

  void Run(Timestamp deadline, std::map<std::string, std::string> args,
           std::shared_ptr<grpc_event_engine::experimental::EventEngine>
               event_engine,
           absl::AnyInvocable<void(Json)> callback) override {
    event_engine->Run([callback = std::move(callback)]() mutable {
      callback(Json::FromBool(false));
    });
  }
};

class StubImpl {
 public:
  template <typename T>
  void Append(const T&) {}

  std::unique_ptr<ZTrace> MakeZTrace() {
    return std::make_unique<ZTraceImpl>();
  }
};
}  // namespace ztrace_collector_detail

template <typename...>
class ZTraceCollector : public ztrace_collector_detail::StubImpl {};
}  // namespace grpc_core::channelz
#else
namespace grpc_core::channelz {
namespace ztrace_collector_detail {

template <typename T>
using Collection = std::deque<std::pair<gpr_cycle_counter, T> >;

template <typename T>
void AppendResults(const Collection<T>& data, Json::Array& results) {
  for (const auto& value : data) {
    Json::Object object;
    object["timestamp"] =
        Json::FromString(gpr_format_timespec(gpr_convert_clock_type(
            gpr_cycle_counter_to_time(value.first), GPR_CLOCK_REALTIME)));
    value.second.RenderJson(object);
    results.emplace_back(Json::FromObject(std::move(object)));
  }
}

template <typename Needle, typename... Haystack>
constexpr bool kIsElement = false;

template <typename Needle, typename... Haystack>
constexpr bool kIsElement<Needle, Needle, Haystack...> = true;

template <typename Needle, typename H, typename... Haystack>
constexpr bool kIsElement<Needle, H, Haystack...> =
    kIsElement<Needle, Haystack...>;

}  // namespace ztrace_collector_detail

inline std::optional<int64_t> IntFromArgs(
    const std::map<std::string, std::string>& args, const std::string& name) {
  auto it = args.find(name);
  if (it == args.end()) return std::nullopt;
  int64_t out;
  if (!absl::SimpleAtoi(it->second, &out)) return std::nullopt;
  return out;
}

// Generic collector infrastructure for ztrace queries.
// Abstracts away most of the ztrace requirements in an efficient manner,
// allowing system authors to concentrate on emitting useful data.
// If no trace is performed, overhead is one pointer and one relaxed atomic read
// per trace event.
//
// Two kinds of objects are required:
// 1. A `Config`
//    - This type should be constructible with a std::map<std::string,
//    std::string>
//      and provides overall query configuration - the map can be used to pull
//      predicates from the calling system.
//    - Needs a `bool Finishes(T)` method for each Data type (see 2).
//      This allows the config to terminate a query in the event of reaching
//      some configured predicate.
// 2. N `Data` types
//    - One for each kind of data captured in the trace
//    - Allows avoiding e.g. variant<> data types; these are inefficient
//      in this context because they force every recorded entry to use the
//      same number of bytes whilst pending.
template <typename Config, typename... Data>
class ZTraceCollector {
 public:
  template <typename X>
  void Append(X producer_or_value) {
    if (!impl_.is_set()) return;
    if constexpr (ztrace_collector_detail::kIsElement<X, Data...>) {
      AppendValue(std::move(producer_or_value));
    } else {
      AppendValue(producer_or_value());
    }
  }

  std::unique_ptr<ZTrace> MakeZTrace() {
    return std::make_unique<ZTraceImpl>(impl_.GetOrCreate());
  }

 private:
  template <typename T>
  using Collection = ztrace_collector_detail::Collection<T>;

  struct Instance : public RefCounted<Instance> {
    Instance(std::map<std::string, std::string> args,
             std::shared_ptr<grpc_event_engine::experimental::EventEngine>
                 event_engine,
             absl::AnyInvocable<void(Json)> done)
        : memory_cap_(IntFromArgs(args, "memory_cap").value_or(1024 * 1024)),
          config(args),
          event_engine(std::move(event_engine)),
          done(std::move(done)) {}
    using Collections = std::tuple<Collection<Data>...>;
    struct RemoveMostRecentState {
      void (*enact)(Instance*) = nullptr;
      gpr_cycle_counter most_recent;
    };
    template <typename T>
    void Append(std::pair<gpr_cycle_counter, T> value) {
      memory_used_ += value.second.MemoryUsage();
      while (memory_used_ > memory_cap_) RemoveMostRecent();
      std::get<Collection<T> >(data).push_back(std::move(value));
    }
    void RemoveMostRecent() {
      RemoveMostRecentState state;
      (UpdateRemoveMostRecentState<Data>(&state), ...);
      CHECK(state.enact != nullptr);
      state.enact(this);
    }
    template <typename T>
    void UpdateRemoveMostRecentState(RemoveMostRecentState* state) {
      auto& collection = std::get<Collection<T> >(data);
      if (collection.empty()) return;
      if (state->enact == nullptr ||
          collection.front().first > state->most_recent) {
        state->enact = +[](Instance* instance) {
          auto& collection = std::get<Collection<T> >(instance->data);
          const size_t ent_usage = collection.front().second.MemoryUsage();
          CHECK_GE(instance->memory_used_, ent_usage);
          instance->memory_used_ -= ent_usage;
          collection.pop_front();
        };
        state->most_recent = collection.front().first;
      }
    }
    void Finish(absl::Status status) {
      event_engine->Run([data = std::move(data), done = std::move(done),
                         status = std::move(status),
                         memory_used = memory_used_]() mutable {
        Json::Array entries;
        (ztrace_collector_detail::AppendResults(
             std::get<Collection<Data> >(data), entries),
         ...);
        Json::Object result;
        result["entries"] = Json::FromArray(entries);
        result["status"] = Json::FromString(status.ToString());
        result["memory_used"] =
            Json::FromNumber(static_cast<uint64_t>(memory_used));
        done(Json::FromObject(std::move(result)));
      });
    }
    size_t memory_used_ = 0;
    size_t memory_cap_ = 0;
    Config config;
    const Timestamp start_time = Timestamp::Now();
    std::shared_ptr<grpc_event_engine::experimental::EventEngine> event_engine;
    grpc_event_engine::experimental::EventEngine::TaskHandle task_handle{
        grpc_event_engine::experimental::EventEngine::TaskHandle::kInvalid};
    Collections data;
    absl::AnyInvocable<void(Json)> done;
  };
  struct Impl : public RefCounted<Impl> {
    Mutex mu;
    absl::flat_hash_set<RefCountedPtr<Instance> > instances ABSL_GUARDED_BY(mu);
  };
  class ZTraceImpl final : public ZTrace {
   public:
    explicit ZTraceImpl(RefCountedPtr<Impl> impl) : impl_(std::move(impl)) {}

    void Run(Timestamp deadline, std::map<std::string, std::string> args,
             std::shared_ptr<grpc_event_engine::experimental::EventEngine>
                 event_engine,
             absl::AnyInvocable<void(Json)> callback) override {
      auto instance = MakeRefCounted<Instance>(std::move(args), event_engine,
                                               std::move(callback));
      auto impl = std::move(impl_);
      RefCountedPtr<Instance> oldest_instance;
      MutexLock lock(&impl->mu);
      if (impl->instances.size() > 20) {
        // Eject oldest running trace
        Timestamp oldest_time = Timestamp::InfFuture();
        for (auto& instance : impl->instances) {
          if (instance->start_time < oldest_time) {
            oldest_time = instance->start_time;
            oldest_instance = instance;
          }
        }
        CHECK(oldest_instance != nullptr);
        impl->instances.erase(oldest_instance);
        oldest_instance->Finish(
            absl::ResourceExhaustedError("Too many concurrent ztrace queries"));
      }
      instance->task_handle = event_engine->RunAfter(
          deadline - Timestamp::Now(), [instance, impl]() {
            bool finish;
            {
              MutexLock lock(&impl->mu);
              finish = impl->instances.erase(instance);
            }
            if (finish) instance->Finish(absl::DeadlineExceededError(""));
          });
      impl->instances.insert(instance);
    }

   private:
    RefCountedPtr<Impl> impl_;
  };

  template <typename T>
  void AppendValue(T&& data) {
    auto value = std::pair(gpr_get_cycle_counter(), std::forward<T>(data));
    auto* impl = impl_.Get();
    {
      MutexLock lock(&impl->mu);
      switch (impl->instances.size()) {
        case 0:
          return;
        case 1: {
          auto& instances = impl->instances;
          auto& instance = *instances.begin();
          const bool finishes = instance->config.Finishes(value.second);
          instance->Append(std::move(value));
          if (finishes) {
            instance->Finish(absl::OkStatus());
            instances.clear();
          }
        } break;
        default: {
          std::vector<RefCountedPtr<Instance> > finished;
          for (auto& instance : impl->instances) {
            const bool finishes = instance->config.Finishes(value.second);
            instance->Append(value);
            if (finishes) {
              finished.push_back(instance);
            }
          }
          for (const auto& instance : finished) {
            instance->Finish(absl::OkStatus());
            impl->instances.erase(instance);
          }
        }
      }
    }
  }

  SingleSetRefCountedPtr<Impl> impl_;
};
}  // namespace grpc_core::channelz
#endif  // GRPC_NO_ZTRACE

#endif  // GRPC_SRC_CORE_CHANNELZ_ZTRACE_COLLECTOR_H
