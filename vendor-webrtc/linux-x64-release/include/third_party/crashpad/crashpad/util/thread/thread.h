// Copyright 2015 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_UTIL_THREAD_THREAD_H_
#define CRASHPAD_UTIL_THREAD_THREAD_H_

#include "build/build_config.h"

#if BUILDFLAG(IS_POSIX)
#include <pthread.h>
#include <stdint.h>
#elif BUILDFLAG(IS_WIN)
#include <windows.h>
#endif  // BUILDFLAG(IS_POSIX)

#include "build/build_config.h"

namespace crashpad {

//! \brief Basic thread abstraction. Users should derive from this
//!     class and implement ThreadMain().
class Thread {
 public:
  Thread();

  Thread(const Thread&) = delete;
  Thread& operator=(const Thread&) = delete;

  virtual ~Thread();

  //! \brief Create a platform thread, and run ThreadMain() on that thread. Must
  //!     be paired with a call to Join().
  void Start();

  //! \brief Block until ThreadMain() exits. This may be called from any thread.
  //!     Must paired with a call to Start().
  void Join();

#if BUILDFLAG(IS_APPLE)
  //! \brief Returns the thread id of the Thread pthread_t.
  static uint64_t GetThreadIdForTesting();
#endif  // BUILDFLAG(IS_APPLE)

 private:
  //! \brief The thread entry point to be implemented by the subclass.
  virtual void ThreadMain() = 0;

  static
#if BUILDFLAG(IS_POSIX)
      void*
#elif BUILDFLAG(IS_WIN)
      DWORD WINAPI
#endif  // BUILDFLAG(IS_POSIX)
      ThreadEntryThunk(void* argument);

#if BUILDFLAG(IS_POSIX)
  pthread_t platform_thread_;
#elif BUILDFLAG(IS_WIN)
  HANDLE platform_thread_;
#endif
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_THREAD_THREAD_H_
