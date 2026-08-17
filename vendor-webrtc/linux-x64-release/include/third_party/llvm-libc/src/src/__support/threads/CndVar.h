//===-- A platform independent abstraction layer for cond vars --*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC___SUPPORT_SRC_THREADS_LINUX_CNDVAR_H
#define LLVM_LIBC___SUPPORT_SRC_THREADS_LINUX_CNDVAR_H

#include "src/__support/macros/config.h"
#include "src/__support/threads/linux/futex_utils.h" // Futex
#include "src/__support/threads/linux/raw_mutex.h"   // RawMutex
#include "src/__support/threads/mutex.h"             // Mutex

#include <stdint.h> // uint32_t

namespace LIBC_NAMESPACE_DECL {

class CndVar {
  enum CndWaiterStatus : uint32_t {
    WS_Waiting = 0xE,
    WS_Signalled = 0x5,
  };

  struct CndWaiter {
    Futex futex_word = WS_Waiting;
    CndWaiter *next = nullptr;
  };

  CndWaiter *waitq_front;
  CndWaiter *waitq_back;
  RawMutex qmtx;

public:
  LIBC_INLINE static int init(CndVar *cv) {
    cv->waitq_front = cv->waitq_back = nullptr;
    RawMutex::init(&cv->qmtx);
    return 0;
  }

  LIBC_INLINE static void destroy(CndVar *cv) {
    cv->waitq_front = cv->waitq_back = nullptr;
  }

  // Returns 0 on success, -1 on error.
  int wait(Mutex *m);
  void notify_one();
  void broadcast();
};

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC___SUPPORT_THREADS_LINUX_CNDVAR_H
