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

#ifndef GRPC_TEST_CORE_CALL_YODEL_YODEL_TEST_H
#define GRPC_TEST_CORE_CALL_YODEL_YODEL_TEST_H

#include <google/protobuf/text_format.h>
#include <grpc/event_engine/event_engine.h>

#include "absl/functional/any_invocable.h"
#include "absl/log/log.h"
#include "absl/random/bit_gen_ref.h"
#include "absl/strings/string_view.h"
#include "fuzztest/fuzztest.h"
#include "gtest/gtest.h"
#include "src/core/call/call_arena_allocator.h"
#include "src/core/call/call_spine.h"
#include "src/core/call/metadata.h"
#include "src/core/lib/event_engine/event_engine_context.h"
#include "src/core/lib/promise/cancel_callback.h"
#include "src/core/lib/promise/detail/promise_factory.h"
#include "src/core/lib/promise/promise.h"
#include "src/core/util/debug_location.h"
#include "test/core/call/yodel/fuzzer.pb.h"
#include "test/core/event_engine/fuzzing_event_engine/fuzzing_event_engine.h"
#include "test/core/test_util/fuzz_config_vars.h"
#include "test/core/test_util/fuzz_config_vars_helpers.h"
#include "test/core/test_util/proto_bit_gen.h"
#include "test/core/test_util/test_config.h"

namespace grpc_core {

class YodelTest;

namespace yodel_detail {

// Capture the name and location of a test step.
class NameAndLocation {
 public:
  // Allow implicit construction from a string, to capture the start location
  // from the variadic StartTestSeq name argument.
  // NOLINTNEXTLINE
  NameAndLocation(const char* name, SourceLocation location = {})
      : location_(location), name_(name) {}

  SourceLocation location() const { return location_; }
  absl::string_view name() const { return name_; }

 private:
  SourceLocation location_;
  absl::string_view name_;
};

// Capture the state of a test step.
class ActionState {
 public:
  enum State : uint8_t {
    // Initial state: construction of this step in the sequence has not been
    // performed.
    kNotCreated,
    // The step has been created, but not yet started (the initial poll of the
    // created promise has not occurred).
    kNotStarted,
    // The step has been polled, but it's not yet been completed.
    kStarted,
    // The step has been completed.
    kDone,
    // The step has been cancelled.
    kCancelled,
  };

  // Generate a nice little prefix for log messages.
  static absl::string_view StateString(State state);

  ActionState(NameAndLocation name_and_location, int step);

  State Get() const { return state_; }
  void Set(State state, SourceLocation whence = {});
  const NameAndLocation& name_and_location() const {
    return name_and_location_;
  }
  SourceLocation location() const { return name_and_location().location(); }
  const char* file() const { return location().file(); }
  int line() const { return location().line(); }
  absl::string_view name() const { return name_and_location().name(); }
  int step() const { return step_; }
  bool IsDone();

 private:
  const NameAndLocation name_and_location_;
  const int step_;
  std::atomic<State> state_;
};

class SequenceSpawner {
 public:
  SequenceSpawner(
      NameAndLocation name_and_location,
      absl::AnyInvocable<void(absl::string_view, Promise<Empty>)>
          promise_spawner,
      absl::FunctionRef<std::shared_ptr<ActionState>(NameAndLocation, int)>
          action_state_factory)
      : name_and_location_(name_and_location),
        promise_spawner_(
            std::make_shared<
                absl::AnyInvocable<void(absl::string_view, Promise<Empty>)>>(
                std::move(promise_spawner))),
        action_state_factory_(action_state_factory) {}

  template <typename First, typename... FollowUps>
  void Start(First first, FollowUps... followups) {
    using Factory = promise_detail::OncePromiseFactory<void, First>;
    using FactoryPromise = typename Factory::Promise;
    using Result = typename FactoryPromise::Result;
    auto action_state = action_state_factory_(name_and_location_, step_);
    ++step_;
    auto next = MakeNext<Result>(std::move(followups)...);
    (*promise_spawner_)(
        name_and_location_.name(),
        [spawner = promise_spawner_, first = Factory(std::move(first)),
         next = std::move(next), action_state = std::move(action_state),
         name_and_location = name_and_location_]() mutable {
          action_state->Set(ActionState::kNotStarted);
          auto promise = first.Make();
          (*spawner)(name_and_location.name(),
                     WrapPromiseAndNext(std::move(action_state),
                                        std::move(promise), std::move(next)));
          return Empty{};
        });
  }

 private:
  template <typename Arg, typename FirstFollowUp, typename... FollowUps>
  absl::AnyInvocable<void(Arg)> MakeNext(FirstFollowUp first,
                                         FollowUps... followups) {
    using Factory = promise_detail::OncePromiseFactory<Arg, FirstFollowUp>;
    using FactoryPromise = typename Factory::Promise;
    using Result = typename FactoryPromise::Result;
    auto action_state = action_state_factory_(name_and_location_, step_);
    ++step_;
    auto next = MakeNext<Result>(std::move(followups)...);
    return [spawner = promise_spawner_, factory = Factory(std::move(first)),
            next = std::move(next), action_state = std::move(action_state),
            name_and_location = name_and_location_](Arg arg) mutable {
      action_state->Set(ActionState::kNotStarted);
      (*spawner)(
          name_and_location.name(),
          WrapPromiseAndNext(std::move(action_state),
                             factory.Make(std::move(arg)), std::move(next)));
    };
  }

  template <typename R, typename P>
  static Promise<Empty> WrapPromiseAndNext(
      std::shared_ptr<ActionState> action_state, P promise,
      absl::AnyInvocable<void(R)> next) {
    return Promise<Empty>(OnCancel(
        [action_state, promise = std::move(promise),
         next = std::move(next)]() mutable -> Poll<Empty> {
          action_state->Set(ActionState::kStarted);
          auto r = promise();
          if (auto* p = r.value_if_ready()) {
            action_state->Set(ActionState::kDone);
            next(std::move(*p));
            return Empty{};
          } else {
            return Pending{};
          }
        },
        [action_state]() { action_state->Set(ActionState::kCancelled); }));
  }

  template <typename Arg>
  absl::AnyInvocable<void(Arg)> MakeNext() {
    // Enforce last-arg is Empty so we don't drop things
    return [](Empty) {};
  }

  NameAndLocation name_and_location_;
  std::shared_ptr<absl::AnyInvocable<void(absl::string_view, Promise<Empty>)>>
      promise_spawner_;
  absl::FunctionRef<std::shared_ptr<ActionState>(NameAndLocation, int)>
      action_state_factory_;
  int step_ = 1;
};

template <typename Context>
auto SpawnerForContext(
    Context context,
    grpc_event_engine::experimental::EventEngine* event_engine) {
  return [context = std::move(context), event_engine](
             absl::string_view name, Promise<Empty> promise) mutable {
    // Pass new promises via event engine to allow fuzzers to explore
    // reorderings of possibly interleaved spawns.
    event_engine->Run([name, context, promise = std::move(promise)]() mutable {
      context.SpawnInfallible(name, std::move(promise));
    });
  };
}

}  // namespace yodel_detail

class YodelTest {
 public:
  YodelTest(const fuzzing_event_engine::Actions& actions, absl::BitGenRef rng);
  void RunTest();
  virtual ~YodelTest() = default;

 protected:
  // Helpers to generate various random values.
  // When we're fuzzing, delegates to the fuzzer input to generate this data.
  std::string RandomString(int min_length, int max_length,
                           absl::string_view character_set);
  std::string RandomStringFrom(
      std::initializer_list<absl::string_view> choices);
  std::string RandomMetadataKey();
  std::string RandomMetadataValue(absl::string_view key);
  std::string RandomMetadataBinaryKey();
  std::string RandomMetadataBinaryValue();
  std::vector<std::pair<std::string, std::string>> RandomMetadata();
  std::string RandomMessage();
  absl::BitGenRef rng() { return rng_; }

  // Alternative for Seq for test driver code.
  // Registers each step so that WaitForAllPendingWork() can report progress,
  // and wait for completion... AND generate good failure messages when a
  // sequence doesn't complete in a timely manner.
  // Uses the `SpawnInfallible` method on `context` to provide an execution
  // environment for each step.
  // Initiates each step in a different event engine closure to maximize
  // opportunities for fuzzers to reorder the steps, or thready-tsan to expose
  // potential threading issues.
  template <typename Context, typename... Actions>
  void SpawnTestSeq(Context context,
                    yodel_detail::NameAndLocation name_and_location,
                    Actions... actions) {
    yodel_detail::SequenceSpawner(
        name_and_location,
        yodel_detail::SpawnerForContext(std::move(context),
                                        state_->event_engine.get()),
        [this](yodel_detail::NameAndLocation name_and_location, int step) {
          auto action = std::make_shared<yodel_detail::ActionState>(
              name_and_location, step);
          pending_actions_.push(action);
          return action;
        })
        .Start(std::move(actions)...);
  }

  class NoContext {
   public:
    explicit NoContext(grpc_event_engine::experimental::EventEngine* ee) {
      auto arena = SimpleArenaAllocator()->MakeArena();
      arena->SetContext(ee);
      party_ = Party::Make(std::move(arena));
    }

    template <typename PromiseFactory>
    void SpawnInfallible(absl::string_view name,
                         PromiseFactory promise_factory) {
      party_->Spawn(
          name,
          [party = party_,
           promise_factory = std::move(promise_factory)]() mutable {
            promise_detail::OncePromiseFactory<void, PromiseFactory> factory(
                std::move(promise_factory));
            return [party, underlying = factory.Make()]() mutable {
              return underlying();
            };
          },
          [](Empty) {});
    }

   private:
    RefCountedPtr<Party> party_;
  };

  template <typename... Actions>
  void SpawnTestSeqWithoutContext(
      yodel_detail::NameAndLocation name_and_location, Actions... actions) {
    SpawnTestSeq(NoContext{event_engine().get()}, name_and_location,
                 std::move(actions)...);
  }

  auto MakeCall(ClientMetadataHandle client_initial_metadata) {
    auto arena = state_->call_arena_allocator->MakeArena();
    arena->SetContext<grpc_event_engine::experimental::EventEngine>(
        state_->event_engine.get());
    return MakeCallPair(std::move(client_initial_metadata), std::move(arena));
  }

  void WaitForAllPendingWork();

  template <typename T>
  T TickUntil(absl::FunctionRef<Poll<T>()> poll) {
    std::optional<T> result;
    TickUntilTrue([poll, &result]() {
      auto r = poll();
      if (auto* p = r.value_if_ready()) {
        result = std::move(*p);
        return true;
      }
      return false;
    });
    return std::move(*result);
  }

  void TickUntilTrue(absl::FunctionRef<bool()> poll);

  const std::shared_ptr<grpc_event_engine::experimental::FuzzingEventEngine>&
  event_engine() {
    return state_->event_engine;
  }

  void SetMaxRandomMessageSize(size_t max_random_message_size) {
    max_random_message_size_ = max_random_message_size;
  }

 private:
  class WatchDog;
  struct State {
    std::shared_ptr<grpc_event_engine::experimental::FuzzingEventEngine>
        event_engine;
    RefCountedPtr<CallArenaAllocator> call_arena_allocator;
  };

  virtual void TestImpl() = 0;

  void Timeout();

  // Called before the test runs, after core configuration has been reset
  // and before the event engine is started.
  // This is a good time to register any custom core configuration builders.
  virtual void InitCoreConfiguration() {}
  // Called after the event engine has been started, but before the test runs.
  virtual void InitTest() {}
  // Called after the test has run, but before the event engine is shut down.
  virtual void Shutdown() {}

  absl::BitGenRef rng_;
  fuzzing_event_engine::Actions actions_;
  std::unique_ptr<State> state_;
  std::queue<std::shared_ptr<yodel_detail::ActionState>> pending_actions_;
  size_t max_random_message_size_ = 1024 * 1024;
};

inline yodel::Msg ParseTestProto(const std::string& proto) {
  yodel::Msg msg;
  CHECK(google::protobuf::TextFormat::ParseFromString(proto, &msg));
  return msg;
}

}  // namespace grpc_core

#define YODEL_TEST(test_type, name)                                          \
  class YodelTest_##test_type##_##name final : public grpc_core::test_type { \
   public:                                                                   \
    using test_type::test_type;                                              \
                                                                             \
   private:                                                                  \
    void TestImpl() override;                                                \
  };                                                                         \
  void name(const yodel::Msg& msg) {                                         \
    if (!grpc_core::IsEventEngineClientEnabled()) return;                    \
    grpc_core::ApplyFuzzConfigVars(msg.config_vars());                       \
    grpc_core::ProtoBitGen bitgen(msg.rng());                                \
    YodelTest_##test_type##_##name test(msg.event_engine_actions(), bitgen); \
    test.RunTest();                                                          \
  }                                                                          \
  FUZZ_TEST(test_type, name)                                                 \
      .WithDomains(::fuzztest::Arbitrary<yodel::Msg>().WithProtobufField(    \
          "config_vars", AnyConfigVars()));                                  \
  void YodelTest_##test_type##_##name::TestImpl()

#endif  // GRPC_TEST_CORE_CALL_YODEL_YODEL_TEST_H
