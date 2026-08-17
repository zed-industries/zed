// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_STACK_MAPS_GC_GC_API_H_
#define TOOLS_CLANG_STACK_MAPS_GC_GC_API_H_

#include <assert.h>
#include <map>
#include <vector>

#include "objects.h"

using ReturnAddress = uint64_t;
using FramePtr = uintptr_t*;
using RBPOffset = uint32_t;
using DWARF = uint16_t;

using HeapAddress = long*;

// The place where HeapObjects live. For simplicity, the underlying data in a
// HeapObject is always a single uintptr_t. The heap layout mocks a simple
// semi-space collector where objects can be moved between two heap fragments.
//
// Note that this is a no-op collector: unreachable objects are not reclaimed
// and allocation will keep filling the heap until its limited memory is
// exhausted.
class Heap {
 public:
  // Allocates a HeapObject's underlying data field on the heap and returns a
  // pointer to it. This allocation will use the heap fragment returned from a
  // fromspace() call.
  HeapAddress AllocRaw(long value);

  // Moves all values from fromspace to tospace. fromspace becomes tospace and
  // vice versa (i.e. future allocations take place on the opposite heap
  // fragment). Note no objects are dropped in the process.
  void MoveObjects();

  // For an arbitrary pointer into the heap, this will return a new pointer with
  // a corresponding offset into the opposite heap fragment. E.g. a pointer to
  // an address at offset +4 into heap fragment A would return an address at
  // offset +4 into heap fragment B.
  //
  // This is used for relocating root pointer values across a collection during
  // stack walking.
  HeapAddress UpdatePointer(HeapAddress ptr);

 private:
  static constexpr int kHeapSize = 24;

  HeapAddress fromspace() {
    if (alloc_on_a_) {
      return a_frag_;
    } else {
      return b_frag_;
    }
  }

  HeapAddress tospace() {
    if (alloc_on_a_) {
      return b_frag_;
    } else {
      return a_frag_;
    }
  }

  int heap_ptr = 0;
  long a_frag_[kHeapSize];
  long b_frag_[kHeapSize];
  bool alloc_on_a_ = true;
};

// A FrameRoots object contains all the information needed to precisely identify
// live roots for a given safepoint. It contains a list of registers which are
// known to contain roots, and a list of offsets from the stack pointer to known
// on-stack-roots.
//
// Each stackmap entry in .llvm_stackmaps has two parts: a base pointer (not to
// be confused with EBP), which simply points to an object header; and a derived
// pointer which specifies an offset (if any) into the object's interior. In the
// case where only a base object pointer is desired, the derived pointer will be
// 0.
//
// DWARF Register number mapping can be found here:
// Pg.63
// https://software.intel.com/sites/default/files/article/402129/mpx-linux64-abi.pdf
class FrameRoots {
 public:
  FrameRoots(std::vector<DWARF> reg_roots, std::vector<RBPOffset> stack_roots)
      : reg_roots_(reg_roots), stack_roots_(stack_roots) {}

  const std::vector<DWARF>* reg_roots() { return &reg_roots_; }
  const std::vector<RBPOffset>* stack_roots() { return &stack_roots_; }

  bool empty() { return reg_roots_.empty() && stack_roots_.empty(); }

  void Print() const;

 private:
  std::vector<DWARF> reg_roots_;
  std::vector<RBPOffset> stack_roots_;
};

// A SafepointTable provides a runtime mapping of function return addresses to
// on-stack and in-register gc root locations. Return addresses are used as a
// function call site is the only place where safepoints can exist. This map is
// a convenient format for the collector to use while walking a call stack
// looking for the rootset.
class SafepointTable {
 public:
  SafepointTable(std::map<ReturnAddress, FrameRoots> roots)
      : roots_(std::move(roots)) {}

  const std::map<ReturnAddress, FrameRoots>* roots() { return &roots_; }

  void Print() const;

 private:
  const std::map<ReturnAddress, FrameRoots> roots_;
};

SafepointTable GenSafepointTable();

extern SafepointTable spt;
extern Heap* heap;

// During stack scanning, the GC must know when it has reached the top of the
// stack so that it can hand execution back over to the mutator. This global
// variable serves that purpose - it is initialised in main to be equal to
// main's RBP value, and checked against each time the gc steps up into the next
// stack frame. For non-main threads this could be pthread top.
extern "C" void InitTopOfStack();
extern uintptr_t TopOfStack;

void PrintSafepointTable();

// Walks the execution stack looking for live gc roots. This function should
// never be called directly. Instead, the void |GC| function should be
// called. |GC| is an assembly shim which jumps to this function after
// placing the value of RBP in RDI (First arg slot mandated by Sys V ABI).
//
// Stack walking starts from the address in `fp` (assumed to be RBP's
// address). The stack is traversed from bottom to top until the frame pointer
// hits a terminal value (usually main's RBP value).
//
// This works by assuming the calling convention for each frame adheres to the
// Sys V ABI, where the frame pointer is known to point to the address of last
// saved frame pointer (and so on), creating a linked list of frames on the
// stack (shown below).
//
//        +--------------------+
//        |  ...               |
//        +--------------------+
//        |  Saved RBP         |<--+
//        +--------------------+   |
//        |                    |   |
//        | ...                |   |
//        |                    |   |
//        +--------------------+   |
//        |  Return Address    |   |
//        +--------------------+   |
// RBP--> |  Saved RBP         |---+
//        +--------------------+
//        |                    |
//        |  Args              |
//        |                    |
//        +--------------------+
//
// This therefore requires that the optimisation -fomit-frame-pointer is
// disabled in order to guarantee that RBP will not be used as a
// general-purpose register.
extern "C" void StackWalkAndMoveObjects(FramePtr fp);

// A very simple allocator for a HeapObject. For the purposes of this
// experiment, a HeapObject's contents is simply a 64 bit integer. The data
// itself is not important, what is, however, is that it can be accessed through
// the rootset after the collector moves it.
Handle<HeapObject> AllocateHeapObject(HeapAddress data);

#endif  // TOOLS_CLANG_STACK_MAPS_GC_GC_API_H_
