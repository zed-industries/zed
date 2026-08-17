// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_PREFINALIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_PREFINALIZER_H_

#include "v8/include/cppgc/prefinalizer.h"

// Allows registering a method called `PreFinalizer` (name is adjustable) in an
// object of type `GarbageCollected<T>` or `GarbageCollectedMixin`. A
// pre-finalizer must have the function signature `void()`.
//
// A pre-finalizer is a user-defined member function of a garbage-collected
// class that is called when the object is going to be reclaimed. It is invoked
// before the sweeping phase starts to allow a pre-finalizer to touch any other
// on-heap objects which is forbidden from destructors. It is useful for doing
// cleanups that cannot be done with a destructor.
//
// Example:
//
// class YourClass : public GarbageCollected<YourClass> {
//   USING_PRE_FINALIZER(YourClass, Dispose);
//
//  public:
//   void Dispose() {
//     // OK: Other on-heap objects can be touched in a pre-finalizer.
//     other_->Dispose();
//   }
//
//   ~YourClass() {
//     // BAD: Not allowed.
//     // other_->Dispose();
//   }
//
//  private:
//   Member<OtherClass> other_;
// };
//
#define USING_PRE_FINALIZER(Class, PreFinalizer) \
  CPPGC_USING_PRE_FINALIZER(Class, PreFinalizer)

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_PREFINALIZER_H_
