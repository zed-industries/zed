// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SYNCHRONIZATION_CANCELABLE_EVENT_H_
#define BASE_SYNCHRONIZATION_CANCELABLE_EVENT_H_

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/time/time.h"

#if BUILDFLAG(IS_WIN)
#include <windows.h>
#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_CHROMEOS)
#include <semaphore.h>
#else
#include "base/synchronization/waitable_event.h"
#endif

namespace base {

// A CancelableEvent functions as a 0-1 semaphore. It does not start signaled.
// It must not be signaled twice.
//
// Cancel() can only succeed on Windows, Linux, ChromeOS, and Android.
class BASE_EXPORT CancelableEvent {
 public:
  CancelableEvent();
  ~CancelableEvent();

  // Puts the event in the signaled state. Causes the thread blocked on
  // Wait() (if there is one) to be woken up.
  void Signal();

  // Cancels a signal, if possible. Returns whether canceling a signal was
  // successful or not. On success, no thread will wake up. On failure, either
  // no signal was sent in the first place, or a waiting thread already consumed
  // the signal.
  [[nodiscard]] bool Cancel();

  // Waits for this event to be Signal()ed until `wait_delta` has elapsed
  // (real-time; ignores time overrides). Returns true if Signal() occurs or
  // false if `wait_delta` elapses without a Signal().
  //
  // TimedWait() can synchronise its own destruction.
  NOT_TAIL_CALLED bool TimedWait(TimeDelta wait_delta);

  void Wait() { TimedWait(TimeDelta::Max()); }

#if BUILDFLAG(IS_WIN)
  using NativeHandle = HANDLE;
#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID)
  using NativeHandle = sem_t;
#else
  using NativeHandle = WaitableEvent;
#endif

  // Declares that this CancelableEvent will only ever be used by a thread that
  // is idle at the bottom of its stack and waiting for work (in particular, it
  // is not synchronously waiting on this event before resuming ongoing
  // work). This is useful to avoid telling base-internals that this thread is
  // "blocked" when it's merely idle and ready to do work. As such, this is only
  // expected to be used by thread and thread pool impls. In such cases
  // wakeup.flow events aren't emitted on `Signal`/`Wait`, because threading
  // implementations are responsible for emitting the cause of their wakeup from
  // idle.
  void declare_only_used_while_idle() { only_used_while_idle_ = true; }

 private:
  void SignalImpl();
  bool CancelImpl();
  bool TimedWaitImpl(TimeDelta wait_delta);

  bool only_used_while_idle_ = false;

  NativeHandle native_handle_;
};

}  // namespace base

#endif  // BASE_SYNCHRONIZATION_CANCELABLE_EVENT_H_
