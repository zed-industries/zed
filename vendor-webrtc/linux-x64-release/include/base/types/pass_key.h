// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_PASS_KEY_H_
#define BASE_TYPES_PASS_KEY_H_

namespace base {

// base::PassKey can be used to restrict access to functions to an authorized
// caller. The primary use case is restricting the construction of an object in
// situations where the constructor needs to be public, which may be the case
// if the object must be constructed through a helper function, such as
// blink::MakeGarbageCollected.
//
// For example, to limit the creation of 'Foo' to the 'Manager' class:
//
//  class Foo {
//   public:
//    Foo(base::PassKey<Manager>);
//  };
//
//  class Manager {
//   public:
//    using PassKey = base::PassKey<Manager>;
//    Manager() : foo_(blink::MakeGarbageCollected<Foo>(PassKey())) {}
//    void Trace(blink::Visitor* visitor) const { visitor->Trace(foo_); }
//    Foo* GetFooSingleton() { foo_; }
//
//   private:
//    blink::Member<Foo> foo_;
//  };
//
// In the above example, the 'Foo' constructor requires an instance of
// base::PassKey<Manager>. Only Manager is allowed to create such instances,
// making the constructor unusable elsewhere.
template <typename T>
class PassKey {
  friend T;
  PassKey() = default;
};

// NonCopyablePassKey is a version of PassKey that also disallows copy/move
// construction/assignment. This way functions called with a passkey cannot use
// that key to invoke other passkey-protected functions.
template <typename T>
class NonCopyablePassKey {
  friend T;
  NonCopyablePassKey() = default;
  NonCopyablePassKey(const NonCopyablePassKey&) = delete;
  NonCopyablePassKey& operator=(const NonCopyablePassKey&) = delete;
};

}  // namespace base

#endif  // BASE_TYPES_PASS_KEY_H_
