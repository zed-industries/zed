//===- nsan_thread.h --------------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef NSAN_THREAD_H
#define NSAN_THREAD_H

#include "nsan_allocator.h"
#include "sanitizer_common/sanitizer_common.h"
#include "sanitizer_common/sanitizer_posix.h"

namespace __nsan {

class NsanThread {
public:
  static NsanThread *Create(thread_callback_t start_routine, void *arg);
  static void TSDDtor(void *tsd);
  void Destroy();

  void Init(); // Should be called from the thread itself.
  thread_return_t ThreadStart();

  uptr stack_top();
  uptr stack_bottom();
  uptr tls_begin() { return tls_begin_; }
  uptr tls_end() { return tls_end_; }
  bool IsMainThread() { return start_routine_ == nullptr; }

  bool AddrIsInStack(uptr addr);

  void StartSwitchFiber(uptr bottom, uptr size);
  void FinishSwitchFiber(uptr *bottom_old, uptr *size_old);

  NsanThreadLocalMallocStorage &malloc_storage() { return malloc_storage_; }

  int destructor_iterations_;
  __sanitizer_sigset_t starting_sigset_;

private:
  void SetThreadStackAndTls();
  void ClearShadowForThreadStackAndTLS();
  struct StackBounds {
    uptr bottom;
    uptr top;
  };
  StackBounds GetStackBounds() const;

  thread_callback_t start_routine_;
  void *arg_;

  bool stack_switching_;

  StackBounds stack_;
  StackBounds next_stack_;

  uptr tls_begin_;
  uptr tls_end_;

  NsanThreadLocalMallocStorage malloc_storage_;
};

NsanThread *GetCurrentThread();
void SetCurrentThread(NsanThread *t);
void NsanTSDInit(void (*destructor)(void *tsd));
void NsanTSDDtor(void *tsd);

} // namespace __nsan

#endif // NSAN_THREAD_H
