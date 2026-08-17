// Copyright 2009 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TRACELINE_LOGGING_H_
#define TRACELINE_LOGGING_H_

#include <windows.h>

#include <stdio.h>

#define CHECK(exp, ...) \
  if (!(exp)) { \
    printf("FAILED CHECK: %s\n  %s:%d\n", #exp, __FILE__, __LINE__); \
    printf("\naborted.\n"); \
    if (::IsDebuggerPresent()) __debugbreak(); \
    exit(1); \
  }

#define NOTREACHED(...) \
  if (1) { \
    printf("NOTREACHED:\n  %s:%d\n", __FILE__, __LINE__); \
    printf(__VA_ARGS__); \
    printf("\naborted.\n"); \
    if (::IsDebuggerPresent()) __debugbreak(); \
    exit(1); \
  }

#endif  // TRACELINE_LOGGING_H_
