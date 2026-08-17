// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_PROMISE_TESTER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_PROMISE_TESTER_H_

#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/platform/heap/disallow_new_wrapper.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ScriptState;

// Utility for writing unit tests involving promises.
// Typical usage:
//   ScriptPromiseTester tester(script_state, script_promise);
//   tester.WaitUntilSettled();  // Runs a nested event loop.
//   EXPECT_TRUE(tester.IsFulfilled());
//   EXPECT_TRUE(tester.Value().IsUndefined());
class ScriptPromiseTester final {
  STACK_ALLOCATED();

 public:
  template <typename IDLType>
  ScriptPromiseTester(ScriptState* script_state,
                      ScriptPromise<IDLType> script_promise)
      : script_state_(script_state),
        value_object_(MakeGarbageCollected<ScriptValueObject>()) {
    CHECK(script_state);
    if (script_promise.IsEmpty()) {
      return;
    }
    script_promise.Then(script_state,
                        MakeGarbageCollected<ThenFunction<IDLType>>(
                            weak_factory_.GetWeakPtr(), State::kFulfilled),
                        MakeGarbageCollected<ThenFunction<IDLAny>>(
                            weak_factory_.GetWeakPtr(), State::kRejected));
  }

  ScriptPromiseTester(const ScriptPromiseTester&) = delete;
  ScriptPromiseTester& operator=(const ScriptPromiseTester&) = delete;

  // Run microtasks and tasks until the promise is either fulfilled or rejected.
  // If the promise never settles this will busy loop until the test times out.
  void WaitUntilSettled();

  // Did the promise fulfill?
  bool IsFulfilled() const { return state_ == State::kFulfilled; }

  // Did the promise reject?
  bool IsRejected() const { return state_ == State::kRejected; }

  // The value the promise fulfilled or rejected with.
  ScriptValue Value() const;

  String ValueAsString() const;

 private:
  enum class State { kNotSettled, kFulfilled, kRejected };

  template <typename IDLType>
  class ThenFunction : public ThenCallable<IDLType, ThenFunction<IDLType>> {
   public:
    ThenFunction(base::WeakPtr<ScriptPromiseTester> owner, State target_state)
        : owner_(std::move(owner)), target_state_(target_state) {}

    using BlinkType =
        std::conditional_t<WTF::IsGarbageCollectedType<IDLType>::value,
                           std::add_pointer_t<IDLType>,
                           typename IDLTypeToBlinkImplType<IDLType>::type>;

    template <typename T = IDLType>
      requires(!std::is_same_v<T, IDLUndefined>)
    void React(ScriptState* script_state, BlinkType value) {
      if (!owner_) {
        return;
      }

      CHECK_EQ(owner_->state_, State::kNotSettled);
      owner_->state_ = target_state_;

      CHECK(owner_->value_object_->Value().IsEmpty());
      owner_->value_object_->Value() =
          ScriptValue(script_state->GetIsolate(),
                      ToV8Traits<IDLType>::ToV8(script_state, value));
    }

    template <typename T = IDLType>
      requires(std::is_same_v<T, IDLUndefined>)
    void React(ScriptState* script_state) {
      if (!owner_) {
        return;
      }
      CHECK_EQ(owner_->state_, State::kNotSettled);
      owner_->state_ = target_state_;
      owner_->value_object_->Value() =
          ScriptValue(script_state->GetIsolate(),
                      v8::Undefined(script_state->GetIsolate()));
    }

   private:
    GC_PLUGIN_IGNORE("Pointer to on-stack class is valid here.")
    base::WeakPtr<ScriptPromiseTester> owner_;
    State target_state_;
  };

  using ScriptValueObject = DisallowNewWrapper<ScriptValue>;

  ScriptState* script_state_;
  State state_ = State::kNotSettled;
  // Keep ScriptValue explicitly alive via stack-bound root. This allows running
  // tests with `ScriptPromiseTester` that pump the message loop and invoke GCs
  // without stack.
  Persistent<ScriptValueObject> value_object_;

  base::WeakPtrFactory<ScriptPromiseTester> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_PROMISE_TESTER_H_
