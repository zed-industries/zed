//===--- rtsan_context.h - Realtime Sanitizer -------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
//===----------------------------------------------------------------------===//

#pragma once

namespace __rtsan {

class Context {
public:
  Context();

  void RealtimePush();
  void RealtimePop();

  void BypassPush();
  void BypassPop();

  bool InRealtimeContext() const;
  bool IsBypassed() const;

  Context(const Context &) = delete;
  Context(Context &&) = delete;
  Context &operator=(const Context &) = delete;
  Context &operator=(Context &&) = delete;

private:
  int realtime_depth_{0};
  int bypass_depth_{0};
};

class ScopedBypass {
public:
  [[nodiscard]] explicit ScopedBypass(Context &context) : context_(context) {
    context_.BypassPush();
  }

  ~ScopedBypass() { context_.BypassPop(); }

  ScopedBypass(const ScopedBypass &) = delete;
  ScopedBypass &operator=(const ScopedBypass &) = delete;
  ScopedBypass(ScopedBypass &&) = delete;
  ScopedBypass &operator=(ScopedBypass &&) = delete;

private:
  Context &context_;
};

Context &GetContextForThisThread();
} // namespace __rtsan
