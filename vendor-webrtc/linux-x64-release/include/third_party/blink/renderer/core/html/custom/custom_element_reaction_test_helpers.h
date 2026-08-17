// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_TEST_HELPERS_H_

#include "third_party/blink/renderer/core/html/custom/custom_element_reaction.h"

#include <initializer_list>
#include <memory>

#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/renderer/core/html/custom/custom_element_reaction_queue.h"
#include "third_party/blink/renderer/core/html/custom/custom_element_reaction_stack.h"
#include "third_party/blink/renderer/core/html/custom/custom_element_test_helpers.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

class Element;

class Command : public GarbageCollected<Command> {
 public:
  Command() = default;
  Command(const Command&) = delete;
  Command& operator=(const Command&) = delete;
  virtual ~Command() = default;
  virtual void Trace(Visitor* visitor) const {}
  virtual void Run(Element&) = 0;

};

class Call : public Command {
 public:
  using Callback = base::OnceCallback<void(Element&)>;
  Call(Callback callback) : callback_(std::move(callback)) {}
  Call(const Call&) = delete;
  Call& operator=(const Call&) = delete;
  ~Call() override = default;
  void Run(Element& element) override { std::move(callback_).Run(element); }

 private:
  Callback callback_;
};

class Unreached : public Command {
 public:
  Unreached(const char* message) : message_(message) {}
  Unreached(const Unreached&) = delete;
  Unreached& operator=(const Unreached&) = delete;
  ~Unreached() override = default;
  void Run(Element&) override { EXPECT_TRUE(false) << message_; }

 private:
  const char* message_;
};

class Log : public Command {
 public:
  Log(char what, Vector<char>& where) : what_(what), where_(where) {}
  Log(const Log&) = delete;
  Log& operator=(const Log&) = delete;
  ~Log() override = default;
  void Run(Element&) override { where_.push_back(what_); }

 private:
  char what_;
  Vector<char>& where_;
};

class Recurse : public Command {
 public:
  Recurse(CustomElementReactionQueue* queue) : queue_(queue) {}
  Recurse(const Recurse&) = delete;
  Recurse& operator=(const Recurse&) = delete;
  ~Recurse() override = default;
  void Trace(Visitor* visitor) const override {
    Command::Trace(visitor);
    visitor->Trace(queue_);
  }
  void Run(Element& element) override { queue_->InvokeReactions(element); }

 private:
  Member<CustomElementReactionQueue> queue_;
};

class Enqueue : public Command {
 public:
  Enqueue(CustomElementReactionQueue* queue, CustomElementReaction* reaction)
      : queue_(queue), reaction_(reaction) {}
  Enqueue(const Enqueue&) = delete;
  Enqueue& operator=(const Enqueue&) = delete;
  ~Enqueue() override = default;
  void Trace(Visitor* visitor) const override {
    Command::Trace(visitor);
    visitor->Trace(queue_);
    visitor->Trace(reaction_);
  }
  void Run(Element&) override { queue_->Add(*reaction_); }

 private:
  Member<CustomElementReactionQueue> queue_;
  Member<CustomElementReaction> reaction_;
};

class TestReaction : public CustomElementReaction {
 public:
  explicit TestReaction(HeapVector<Member<Command>>&& commands)
      : CustomElementReaction(
            *MakeGarbageCollected<TestCustomElementDefinition>(
                CustomElementDescriptor(AtomicString("mock-element"),
                                        AtomicString("mock-element")))),
        commands_(std::move(commands)) {}
  TestReaction(const TestReaction&) = delete;
  TestReaction& operator=(const TestReaction&) = delete;
  ~TestReaction() override = default;
  void Trace(Visitor* visitor) const override {
    CustomElementReaction::Trace(visitor);
    visitor->Trace(commands_);
  }
  void Invoke(Element& element) override {
    for (auto& command : commands_)
      command->Run(element);
  }

 private:
  HeapVector<Member<Command>> commands_;
};

class ResetCustomElementReactionStackForTest final {
  STACK_ALLOCATED();
 public:
  explicit ResetCustomElementReactionStackForTest(Agent& agent)
      : stack_(MakeGarbageCollected<CustomElementReactionStack>(agent)),
        old_stack_(CustomElementReactionStack::Swap(agent, stack_)),
        agent_(agent) {}
  ResetCustomElementReactionStackForTest(
      const ResetCustomElementReactionStackForTest&) = delete;
  ResetCustomElementReactionStackForTest& operator=(
      const ResetCustomElementReactionStackForTest&) = delete;

  ~ResetCustomElementReactionStackForTest() {
    CustomElementReactionStack::Swap(agent_, old_stack_);
  }

  CustomElementReactionStack& Stack() { return *stack_; }

 private:
  CustomElementReactionStack* stack_;
  CustomElementReactionStack* old_stack_;
  Agent& agent_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_TEST_HELPERS_H_
