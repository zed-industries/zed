// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_PARTY_H
#define GRPC_SRC_CORE_LIB_PROMISE_PARTY_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include <atomic>
#include <limits>
#include <string>
#include <utility>

#include "absl/base/attributes.h"
#include "absl/log/check.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/event_engine/event_engine_context.h"
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/context.h"
#include "src/core/lib/promise/detail/promise_factory.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/util/check_class_size.h"
#include "src/core/util/construct_destruct.h"
#include "src/core/util/crash.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

// Terms Used:
//
// Concurrent promises: Promises that may be in progress at a given instant
// of time, but they may or may not execute simultaneously.
// Parallel promises: Two or more Promises that are getting executed
// simultaneously at the same instant of time.
//
// Promise Party
// A Promise Party forms the execution environment for Promises. It runs up
// to sixteen concurrent party participants and ensures none of the participants
// run in parallel with each other.
//
// Party Participant
// One Promise Party has 16 Participant slots. Each Participant slot can hold
// one of the three
// 1. A Promise
// 2. Or a Promise Factory
// 3. or any type of Participant object, such as SpawnSerializer.
//
// Using a Party
// A party should be used when
// 1. You need many promises to be run in a way that is concurrent but not
// parallel.
// 2. These promises have their own complex sleep and wake mechanisms.
// 3. You need a way to run the pending promises to completion by repolling them
// as needed.
//
// Creating a new Party
// A Party must only be created using Party::Make function.
//
// Spawning a promise on a Party
// 1. A promise can be spawned on a party using either Spawn or SpawnWaitable
// method.
// 2. If you want to bulk spawn promises on a party before any thread
// starts executing them, use Party::WakeupHold.
//
// Execution of Spawned Promises
// A Participant spawned on a party will get executed by that party.
// Whenever the party wakes up, it will execute all unresolved Party
// Participants at least once. After any Party Participant is resolved, its slot
// is freed up to make place for new participants.
//
// When these Party Participants are executed (polled), they can either
// 1. Resolve by returning a value
// 2. Return Pending{}
// 3. Wait for a certain event to happen using either a Notification or a Latch.
//
// When a promise factory is passed as a Party Participant, the promise factory
// is used to generate the promise once, and then the promise is executed once
// each time the party is polled till it resolves.
//
// Sleep mechanism of a Party
// A party will Sleep/Quiece if all Participants Spawned on the party are in any
// of the following states
// 1. Return Pending{}
// 2. Resolve
// 3. Are waiting because of a Latch.
// If a party is currently running a Participant, it is said to be active/awake.
// Otherwise it is said to be Sleeping or Quieced.
//
// Wake mechanism of a Party
// To wake up a sleeping party you can use the Waker object. Once the Party is
// woken, it will be executed as mentioned above.
//
// Party Cancellation
// A Party can be cancelled using party_.reset() method.
//
// Guarantees of a Party
// 1. All Participant spawned on one party are guaranteed to be run serially.
// Their execution will not happen in parallel.
// 2. If a promise is executed, its on_complete is guaranteed to be executed as
// long as the party is not cancelled.
// 3. Once a party is cancelled, Participants that were Spawned onto the party,
// but not yet executed, will not get executed.
// 4. Promise spawned on a party will never be repolled after it is resolved.
// 5. A promise spawned on a party, can in turn spawn another promise either on
// the same party or on another party. We allow nesting of Spawn function.
// 6. A promise or promise factory that is passed to a Spawn function could
// either be a single simple promise, or it could be a promise combinator such
// as TrySeq, TryJoin, Loop or any such promise combinator. Nesting of these
// promise combinators is allowed.
// 7. You can re-use the same party to spawn new Participants as long as the
// older Participants have been resolved.
// 8. We guarantee safe working of up to 16 un-resolved participants
// on a party at a time.
//
// Non-Guarantees of a Party
// 1. Promises spawned on one party are not guaranteed to execute in the same
// order. They can execute in any order. If you need the promises to be executed
// in a specific order, use a SpawnSerializer. If that is not feasible for some
// reason, then either consider the use of a promise combinator, or order the
// execution of promises using Notifications or Latches.
// 2. A party cannot guarantee which thread a party Participant will execute on.
// It could either execute on the current thread, or an event engine thread or
// any other thread.
// 3. Say, we spawned promises P1_1, P1_2, P1_3 on party1. Then promise P1_1 in
// turn spawns promise P2_1, P2_2, P2_3 on party2. In such cases, party1 and
// party2 may either execute on the same thread, or they may both execute on
// different threads. party1 and party2 may or may not run in parallel.

namespace party_detail {

// Number of bits reserved for wakeups gives us the maximum number of
// participants. This can change in the future and we don't guarantee this
// number to be 16 always.
static constexpr size_t kMaxParticipants = 16;

}  // namespace party_detail

class Party : public Activity, private Wakeable {
 private:
  // Non-owning wakeup handle.
  class Handle;

  // One promise participant in the party.
  class Participant {
   public:
    // Poll the participant promise. Return true if complete.
    // Participant should take care of its own deallocation in this case.
    virtual bool PollParticipantPromise() = 0;

    // Destroy the participant before finishing.
    virtual void Destroy() = 0;

    // Return a Handle instance for this participant.
    Wakeable* MakeNonOwningWakeable(Party* party);

   protected:
    ~Participant();

   private:
    Handle* handle_ = nullptr;
  };

 public:
  Party(const Party&) = delete;
  Party& operator=(const Party&) = delete;

  // A Party object must be created only by using this method.
  static RefCountedPtr<Party> Make(RefCountedPtr<Arena> arena) {
    auto* arena_ptr = arena.get();
    return RefCountedPtr<Party>(arena_ptr->New<Party>(std::move(arena)));
  }

  // When calling into a Party from outside the promises system we often would
  // like to perform more than one action.
  // This class tries to acquire the party lock just once - if it succeeds then
  // it runs the party in its destructor, effectively holding all wakeups of the
  // party until it goes out of scope.
  // If it fails, presumably some other thread holds the lock - and in this case
  // we don't attempt to do any buffering.
  class WakeupHold {
   public:
    WakeupHold() = default;
    explicit WakeupHold(Party* party)
        : prev_state_(party->state_.load(std::memory_order_relaxed)) {
      // Try to lock
      if ((prev_state_ & kLocked) == 0 &&
          party->state_.compare_exchange_weak(prev_state_,
                                              (prev_state_ | kLocked) + kOneRef,
                                              std::memory_order_relaxed)) {
        DCHECK_EQ(prev_state_ & ~(kRefMask | kAllocatedMask), 0u)
            << "Party should have contained no wakeups on lock";
        // If we win, record that fact for the destructor
        party->LogStateChange("WakeupHold", prev_state_,
                              (prev_state_ | kLocked) + kOneRef);
        party_ = party;
      }
    }
    WakeupHold(const WakeupHold&) = delete;
    WakeupHold& operator=(const WakeupHold&) = delete;
    WakeupHold(WakeupHold&& other) noexcept
        : party_(std::exchange(other.party_, nullptr)),
          prev_state_(other.prev_state_) {}
    WakeupHold& operator=(WakeupHold&& other) noexcept {
      std::swap(party_, other.party_);
      std::swap(prev_state_, other.prev_state_);
      return *this;
    }

    ~WakeupHold() {
      if (party_ == nullptr) return;
      party_->RunLockedAndUnref(party_, prev_state_);
    }

   private:
    Party* party_ = nullptr;
    uint64_t prev_state_;
  };

  // SpawnSerializer is a helper class to serialize the execution of multiple
  // promises on a party.
  //
  // Provides the guarantee that given:
  //   SpawnSerializer* serializer = party->MakeSpawnSerializer();
  //   serializer->Spawn([] { /* promise 1 */; });
  //   serializer->Spawn([] { /* promise 2 */; });
  // 1. promise 1 will be resolved before promise 2 is started.
  // 2. promise 1 and promise 2 will execute on `party`.
  //
  // It's possible to have multiple SpawnSerializer instances on a party.
  // Once created, the SpawnSerializer is only valid until the party is
  // released - note that SpawnSerializer itself does not hold a ref to the
  // party.
  //
  // Each SpawnSerializer consumes one slot in the party's participant array for
  // the lifetime of the party - that is that a SpawnSerializer counts as one of
  // the sixteen promises executing on a Party.
  //
  // The promises spawned by SpawnSerializer *DO NOT* count towards the sixteen
  // promise limit.
  //
  // The size of this type matters, and so we leverage private inheritance to
  // minimize the number of pointers needed to be kept per instance.
  class SpawnSerializer final : private Participant {
   public:
    // Spawn a promise into the party.
    //
    // The promise will be polled until it is resolved, or until the party is
    // shut down.
    //
    // Later promises spawned by this serializer will not be polled until this
    // promise is resolved, and then one at a time in the order they were
    // spawned.
    //
    // Spawn itself is not thread safe: at most one thread can call Spawn at a
    // time. Just as the execution of promises is serialized by this type, the
    // spawning of promises is expected to be serialized by some external entity
    // (usually this is a Seq running on a different party).
    template <class Factory>
    void Spawn(Factory factory) {
      auto empty_completion = [](Empty) {};
      next_.Push(new ParticipantImpl<Factory, decltype(empty_completion)>(
          "SpawnSerializer", std::move(factory), empty_completion));
      party_->WakeupFromState<false>(
          party_->state_.load(std::memory_order_relaxed), wakeup_mask_);
    }

   private:
    friend class Party;
    friend class Arena;

    bool PollParticipantPromise() override;
    void Destroy() override;

    explicit SpawnSerializer(Party* party)
        : next_(party->arena()), party_(party) {}

    // Queue of promises to be executed after the active promise resolves.
    ArenaSpsc<Participant*, false> next_;
    // The promise currently being executed.
    Participant* active_ = nullptr;
    // The wakeup mask for this serializers participant.
    // Allows us to wake up the party when a new promise is added to the queue.
    WakeupMask wakeup_mask_;
    // The party this serializer is running on.
    Party* const party_;
  };

  // Spawn one promise into the party.
  // The party can poll the promise until it is resolved, or until the party is
  // shut down.
  // The on_complete callback will be called with the result of the
  // promise if it completes. Even if the promise returns a failed status,
  // on_complete will be called.
  // promise_factory called to create the promise with the party lock taken;
  // after the promise is created the factory is destroyed. This means that
  // pointers or references to factory members will be invalidated after the
  // promise is created - so the promise should not retain any of these.
  // This function is thread safe. We can Spawn different promises onto the
  // same party from different threads.
  // A party can hold upto 16 unresolved promises at a time. However, this
  // number might change in the future.
  template <typename Factory, typename OnComplete>
  void Spawn(absl::string_view name, Factory promise_factory,
             OnComplete on_complete);

  template <typename Factory>
  auto SpawnWaitable(absl::string_view name, Factory factory);

  void Orphan() final { Crash("unused"); }

  // Activity implementation: not allowed to be overridden by derived types.
  void ForceImmediateRepoll(WakeupMask mask) final;
  WakeupMask CurrentParticipant() const final {
    DCHECK(currently_polling_ != kNotPolling);
    return 1u << currently_polling_;
  }
  Waker MakeOwningWaker() final;
  Waker MakeNonOwningWaker() final;
  std::string ActivityDebugTag(WakeupMask wakeup_mask) const final;

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION void IncrementRefCount() {
    const uint64_t prev_state =
        state_.fetch_add(kOneRef, std::memory_order_relaxed);
    LogStateChange("IncrementRefCount", prev_state, prev_state + kOneRef);
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION void Unref() {
    uint64_t prev_state = state_.fetch_sub(kOneRef, std::memory_order_acq_rel);
    LogStateChange("Unref", prev_state, prev_state - kOneRef);
    if ((prev_state & kRefMask) == kOneRef) PartyIsOver();
  }

  RefCountedPtr<Party> Ref() {
    IncrementRefCount();
    return RefCountedPtr<Party>(this);
  }

  template <typename T>
  RefCountedPtr<T> RefAsSubclass() {
    IncrementRefCount();
    return RefCountedPtr<T>(DownCast<T*>(this));
  }

  Arena* arena() { return arena_.get(); }

  SpawnSerializer* MakeSpawnSerializer() {
    auto* const serializer = arena_->New<SpawnSerializer>(this);
    const size_t slot = AddParticipant(serializer);
    DCHECK_NE(slot, std::numeric_limits<size_t>::max());
    serializer->wakeup_mask_ = 1ull << slot;
    return serializer;
  }

 protected:
  friend class Arena;

  // Derived types should be constructed upon `arena`.
  explicit Party(RefCountedPtr<Arena> arena) : arena_(std::move(arena)) {
    CHECK(arena_->GetContext<grpc_event_engine::experimental::EventEngine>() !=
          nullptr);
  }
  ~Party() override;

  // Main run loop. Must be locked.
  // Polls participants and drains the add queue until there is no work left to
  // be done.
  void RunPartyAndUnref(uint64_t prev_state);

  bool RefIfNonZero();

 private:
  // Concrete implementation of a participant for some promise & oncomplete
  // type.
  template <typename SuppliedFactory, typename OnComplete>
  class ParticipantImpl final : public Participant {
    using Factory = promise_detail::OncePromiseFactory<void, SuppliedFactory>;
    using Promise = typename Factory::Promise;

   public:
    ParticipantImpl(absl::string_view, SuppliedFactory promise_factory,
                    OnComplete on_complete)
        : on_complete_(std::move(on_complete)) {
      Construct(&factory_, std::move(promise_factory));
    }
    ~ParticipantImpl() {
      if (!started_) {
        Destruct(&factory_);
      } else {
        Destruct(&promise_);
      }
    }

    bool PollParticipantPromise() override {
      if (!started_) {
        auto p = factory_.Make();
        Destruct(&factory_);
        Construct(&promise_, std::move(p));
        started_ = true;
      }
      auto p = promise_();
      if (auto* r = p.value_if_ready()) {
        on_complete_(std::move(*r));
        delete this;
        return true;
      }
      return false;
    }

    void Destroy() override { delete this; }

   private:
    union {
      GPR_NO_UNIQUE_ADDRESS Factory factory_;
      GPR_NO_UNIQUE_ADDRESS Promise promise_;
    };
    GPR_NO_UNIQUE_ADDRESS OnComplete on_complete_;
    bool started_ = false;
  };

  template <typename SuppliedFactory>
  class PromiseParticipantImpl final
      : public RefCounted<PromiseParticipantImpl<SuppliedFactory>,
                          NonPolymorphicRefCount>,
        public Participant {
    using Factory = promise_detail::OncePromiseFactory<void, SuppliedFactory>;
    using Promise = typename Factory::Promise;
    using Result = typename Promise::Result;

   public:
    PromiseParticipantImpl(absl::string_view, SuppliedFactory promise_factory) {
      Construct(&factory_, std::move(promise_factory));
    }

    ~PromiseParticipantImpl() {
      switch (state_.load(std::memory_order_acquire)) {
        case State::kFactory:
          Destruct(&factory_);
          break;
        case State::kPromise:
          Destruct(&promise_);
          break;
        case State::kResult:
          Destruct(&result_);
          break;
      }
    }

    // Inside party poll: drive from factory -> promise -> result
    bool PollParticipantPromise() override {
      switch (state_.load(std::memory_order_relaxed)) {
        case State::kFactory: {
          auto p = factory_.Make();
          Destruct(&factory_);
          Construct(&promise_, std::move(p));
          state_.store(State::kPromise, std::memory_order_relaxed);
        }
          [[fallthrough]];
        case State::kPromise: {
          auto p = promise_();
          if (auto* r = p.value_if_ready()) {
            Destruct(&promise_);
            Construct(&result_, std::move(*r));
            state_.store(State::kResult, std::memory_order_release);
            waiter_.Wakeup();
            this->Unref();
            return true;
          }
          return false;
        }
        case State::kResult:
          Crash(
              "unreachable: promises should not be repolled after completion");
      }
    }

    // Outside party poll: check whether the spawning party has completed this
    // promise.
    Poll<Result> PollCompletion() {
      switch (state_.load(std::memory_order_acquire)) {
        case State::kFactory:
        case State::kPromise:
          return Pending{};
        case State::kResult:
          return std::move(result_);
      }
    }

    void Destroy() override { this->Unref(); }

   private:
    enum class State : uint8_t { kFactory, kPromise, kResult };
    union {
      GPR_NO_UNIQUE_ADDRESS Factory factory_;
      GPR_NO_UNIQUE_ADDRESS Promise promise_;
      GPR_NO_UNIQUE_ADDRESS Result result_;
    };
    Waker waiter_{GetContext<Activity>()->MakeOwningWaker()};
    std::atomic<State> state_{State::kFactory};
  };

  // State bits:
  // The atomic state_ field is composed of the following:
  //   - 24 bits for ref counts
  //     1 is owned by the party prior to Orphan()
  //     All others are owned by owning wakers
  //   - 1 bit to indicate whether the party is locked
  //     The first thread to set this owns the party until it is unlocked
  //     That thread will run the main loop until no further work needs to
  //     be done.
  //   - 1 bit to indicate whether there are participants waiting to be
  //   added
  //   - 16 bits, one per participant, indicating which participants have
  //   been
  //     woken up and should be polled next time the main loop runs.

  // clang-format off
  // Bits used to store 16 bits of wakeups
  static constexpr uint64_t kWakeupMask    = 0x0000'0000'0000'ffff;
  // Bits used to store 16 bits of allocated participant slots.
  static constexpr uint64_t kAllocatedMask = 0x0000'0000'ffff'0000;
  // Bit indicating locked or not
  static constexpr uint64_t kLocked        = 0x0000'0008'0000'0000;
  // Bits used to store 24 bits of ref counts
  static constexpr uint64_t kRefMask       = 0xffff'ff00'0000'0000;
  // clang-format on

  // Shift to get from a participant mask to an allocated mask.
  static constexpr size_t kAllocatedShift = 16;
  // How far to shift to get the refcount
  static constexpr size_t kRefShift = 40;
  // One ref count
  static constexpr uint64_t kOneRef = 1ull << kRefShift;

  // Destroy any remaining participants.
  // Needs to have normal context setup before calling.
  void CancelRemainingParticipants();

  // Run the locked part of the party until it is unlocked.
  static void RunLockedAndUnref(Party* party, uint64_t prev_state);
  // Called in response to Unref() hitting zero - ultimately calls PartyOver,
  // but needs to set some stuff up.
  // Here so it gets compiled out of line.
  void PartyIsOver();

  // Wakeable implementation
  void Wakeup(WakeupMask wakeup_mask) final {
    GRPC_LATENT_SEE_INNER_SCOPE("Party::Wakeup");
    if (Activity::current() == this) {
      wakeup_mask_ |= wakeup_mask;
      Unref();
      return;
    }
    WakeupFromState<true>(state_.load(std::memory_order_relaxed), wakeup_mask);
  }

  template <bool kReffed>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION void WakeupFromState(
      uint64_t cur_state, WakeupMask wakeup_mask) {
    GRPC_LATENT_SEE_INNER_SCOPE("Party::WakeupFromState");
    DCHECK_NE(wakeup_mask & kWakeupMask, 0u)
        << "Wakeup mask must be non-zero: " << wakeup_mask;
    while (true) {
      if (cur_state & kLocked) {
        // If the party is locked, we need to set the wakeup bits, and then
        // we'll immediately unref. Since something is running this should never
        // bring the refcount to zero.
        if (kReffed) {
          DCHECK_GT(cur_state & kRefMask, kOneRef);
        } else {
          DCHECK_GE(cur_state & kRefMask, kOneRef);
        }
        const uint64_t new_state =
            (cur_state | wakeup_mask) - (kReffed ? kOneRef : 0);
        if (state_.compare_exchange_weak(cur_state, new_state,
                                         std::memory_order_release)) {
          LogStateChange("Wakeup", cur_state, cur_state | wakeup_mask);
          return;
        }
      } else {
        // If the party is not locked, we need to lock it and run.
        DCHECK_EQ(cur_state & kWakeupMask, 0u);
        const uint64_t new_state =
            (cur_state | kLocked) + (kReffed ? 0 : kOneRef);
        if (state_.compare_exchange_weak(cur_state, new_state,
                                         std::memory_order_acq_rel)) {
          LogStateChange("WakeupAndRun", cur_state, new_state);
          wakeup_mask_ |= wakeup_mask;
          RunLockedAndUnref(this, cur_state);
          return;
        }
      }
    }
  }

  void WakeupAsync(WakeupMask wakeup_mask) final;
  void Drop(WakeupMask wakeup_mask) final;

  // Add a participant (backs Spawn, after type erasure to ParticipantFactory).
  size_t AddParticipant(Participant* participant);
  void MaybeAsyncAddParticipant(Participant* participant);

  static uint64_t NextAllocationMask(uint64_t current_allocation_mask);

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION void LogStateChange(
      const char* op, uint64_t prev_state, uint64_t new_state,
      DebugLocation loc = {}) {
    GRPC_TRACE_LOG(party_state, INFO).AtLocation(loc.file(), loc.line())
        << this << " " << op << " "
        << absl::StrFormat("%016" PRIx64 " -> %016" PRIx64, prev_state,
                           new_state);
  }

  // Sentinel value for currently_polling_ when no participant is being polled.
  static constexpr uint8_t kNotPolling = 255;

  std::atomic<uint64_t> state_{kOneRef};
  uint8_t currently_polling_ = kNotPolling;
  WakeupMask wakeup_mask_ = 0;
  // All current participants, using a tagged format.
  // If the lower bit is unset, then this is a Participant*.
  // If the lower bit is set, then this is a ParticipantFactory*.
  std::atomic<Participant*> participants_[party_detail::kMaxParticipants] = {};
  RefCountedPtr<Arena> arena_;
};

GRPC_CHECK_CLASS_SIZE(Party, 180);

template <>
struct ContextSubclass<Party> {
  using Base = Activity;
};

template <typename Factory, typename OnComplete>
void Party::Spawn(absl::string_view name, Factory promise_factory,
                  OnComplete on_complete) {
  GRPC_TRACE_LOG(party_state, INFO) << "PARTY[" << this << "]: spawn " << name;
  MaybeAsyncAddParticipant(new ParticipantImpl<Factory, OnComplete>(
      name, std::move(promise_factory), std::move(on_complete)));
}

template <typename Factory>
auto Party::SpawnWaitable(absl::string_view name, Factory promise_factory) {
  GRPC_TRACE_LOG(party_state, INFO) << "PARTY[" << this << "]: spawn " << name;
  auto participant = MakeRefCounted<PromiseParticipantImpl<Factory>>(
      name, std::move(promise_factory));
  Participant* p = participant->Ref().release();
  MaybeAsyncAddParticipant(p);
  return [participant = std::move(participant)]() mutable {
    return participant->PollCompletion();
  };
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_PARTY_H
