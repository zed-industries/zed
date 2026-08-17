// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_TEST_OBJECTS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_TEST_OBJECTS_H_

#include "base/functional/callback_forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

template <typename T>
class ObjectWithCallbackBeforeInitializer
    : public GarbageCollected<ObjectWithCallbackBeforeInitializer<T>> {
 public:
  ObjectWithCallbackBeforeInitializer(
      base::OnceCallback<void(ObjectWithCallbackBeforeInitializer<T>*)>&& cb,
      T* value)
      : bool_(ExecuteCallbackReturnTrue(this, std::move(cb))), value_(value) {}

  ObjectWithCallbackBeforeInitializer(  // NOLINT
      base::OnceCallback<void(ObjectWithCallbackBeforeInitializer<T>*)>&& cb)
      : bool_(ExecuteCallbackReturnTrue(this, std::move(cb))) {}

  virtual void Trace(Visitor* visitor) const { visitor->Trace(value_); }

  T* value() const { return value_.Get(); }

 private:
  static bool ExecuteCallbackReturnTrue(
      ObjectWithCallbackBeforeInitializer* thiz,
      base::OnceCallback<void(ObjectWithCallbackBeforeInitializer<T>*)>&& cb) {
    std::move(cb).Run(thiz);
    return true;
  }

  bool bool_;
  Member<T> value_;
};

template <typename T>
class MixinWithCallbackBeforeInitializer : public GarbageCollectedMixin {
 public:
  MixinWithCallbackBeforeInitializer(
      base::OnceCallback<void(MixinWithCallbackBeforeInitializer<T>*)>&& cb,
      T* value)
      : bool_(ExecuteCallbackReturnTrue(this, std::move(cb))), value_(value) {}

  MixinWithCallbackBeforeInitializer(  // NOLINT
      base::OnceCallback<void(MixinWithCallbackBeforeInitializer<T>*)>&& cb)
      : bool_(ExecuteCallbackReturnTrue(this, std::move(cb))) {}

  void Trace(Visitor* visitor) const override { visitor->Trace(value_); }

  T* value() const { return value_.Get(); }

 private:
  static bool ExecuteCallbackReturnTrue(
      MixinWithCallbackBeforeInitializer* thiz,
      base::OnceCallback<void(MixinWithCallbackBeforeInitializer<T>*)>&& cb) {
    std::move(cb).Run(thiz);
    return true;
  }

  bool bool_;
  Member<T> value_;
};

class BoolMixin {
 protected:
  bool bool_ = false;
};

template <typename T>
class ObjectWithMixinWithCallbackBeforeInitializer
    : public GarbageCollected<ObjectWithMixinWithCallbackBeforeInitializer<T>>,
      public BoolMixin,
      public MixinWithCallbackBeforeInitializer<T> {
 public:
  using Mixin = MixinWithCallbackBeforeInitializer<T>;

  ObjectWithMixinWithCallbackBeforeInitializer(
      base::OnceCallback<void(Mixin*)>&& cb,
      T* value)
      : Mixin(std::move(cb), value) {}

  ObjectWithMixinWithCallbackBeforeInitializer(  // NOLINT
      base::OnceCallback<void(Mixin*)>&& cb)
      : Mixin(std::move(cb)) {}

  void Trace(Visitor* visitor) const override { Mixin::Trace(visitor); }
};

// Simple linked object to be used in tests.
class LinkedObject : public GarbageCollected<LinkedObject> {
 public:
  LinkedObject() = default;
  explicit LinkedObject(LinkedObject* next) : next_(next) {}

  void set_next(LinkedObject* next) { next_ = next; }
  LinkedObject* next() const { return next_.Get(); }
  Member<LinkedObject>& next_ref() { return next_; }

  virtual void Trace(Visitor* visitor) const { visitor->Trace(next_); }

 private:
  Member<LinkedObject> next_;
};

class IntegerObject : public GarbageCollected<IntegerObject> {
 public:
  static std::atomic_int destructor_calls;

  explicit IntegerObject(int x) : x_(x) {}

  virtual ~IntegerObject() {
    destructor_calls.fetch_add(1, std::memory_order_relaxed);
  }

  virtual void Trace(Visitor* visitor) const {}

  int Value() const { return x_; }

  bool operator==(const IntegerObject& other) const {
    return other.Value() == Value();
  }

  unsigned GetHash() { return WTF::GetHash(x_); }

 private:
  int x_;
};

struct IntegerObjectHash {
  static unsigned GetHash(const IntegerObject& key) {
    return WTF::HashInt(static_cast<uint32_t>(key.Value()));
  }

  static bool Equal(const IntegerObject& a, const IntegerObject& b) {
    return a == b;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_TEST_OBJECTS_H_
