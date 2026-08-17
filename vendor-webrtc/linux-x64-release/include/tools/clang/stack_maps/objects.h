// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_STACK_MAPS_OBJECTS_H_
#define TOOLS_CLANG_STACK_MAPS_OBJECTS_H_

#include <stdint.h>

#define GC_AS __attribute__((address_space(1)))

// This should be used only when finer control is needed to prevent statepoint
// insertion. It must not be used on functions which will have a pointer on the
// stack across a GC. It should be used very carefully as it overrides the
// default statepointing mechanism.
#define NO_STATEPOINT \
  __attribute__((noinline)) __attribute__((annotate("no-statepoint")))

using Address = void GC_AS*;

// A HeapObject is just a heap allocated long integer. This is all that is
// necessary to show precise stack scanning in practise and greatly simplifies
// the implementation.
class HeapObject {
 public:
  NO_STATEPOINT HeapObject(long data) : data(data) {}
  long data;
};

template <typename T>
class Handle {
 public:
  static NO_STATEPOINT Handle<T> New(T* obj_ptr) {
    // We have to break the style guide here and do a C style cast because it
    // guarantees an address space cast takes place in the IR. reinterpret_cast
    // will fail to compile when address space qualifiers do not match.
    auto gcptr = (Address GC_AS*)obj_ptr;
    return Handle<T>(gcptr);
  }

  T operator*() const {
    long data = *(long GC_AS*)address;
    return HeapObject(data);
  }

 private:
  Address address;
  Handle<T>(Address address) : address(address) {}
};

#endif  // TOOLS_CLANG_STACK_MAPS_OBJECTS_H_
