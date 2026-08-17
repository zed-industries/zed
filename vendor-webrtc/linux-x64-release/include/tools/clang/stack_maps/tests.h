// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_STACK_MAPS_TESTS_H_
#define TOOLS_CLANG_STACK_MAPS_TESTS_H_

// Initialises the GC by setting up the heap and marking top of stack so the
// gc knows where to stop during walking.
extern void InitGC();

// Calls the collector, which will move the underlying heap objects and update
// pointer values on the stack.
extern "C" void GC();

// Frees all heap memory
extern void TeardownGC();

#endif  // TOOLS_CLANG_STACK_MAPS_TESTS_H_
