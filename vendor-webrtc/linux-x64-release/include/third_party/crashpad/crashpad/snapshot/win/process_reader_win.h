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

#ifndef CRASHPAD_SNAPSHOT_WIN_PROCESS_READER_WIN_H_
#define CRASHPAD_SNAPSHOT_WIN_PROCESS_READER_WIN_H_

#include <windows.h>
#include <sys/time.h>

#include <string>
#include <vector>

#include "build/build_config.h"
#include "util/misc/initialization_state_dcheck.h"
#include "util/process/process_memory_win.h"
#include "util/win/address_types.h"
#include "util/win/process_info.h"

namespace crashpad {

//! \brief State of process being read by ProcessReaderWin.
enum class ProcessSuspensionState : bool {
  //! \brief The process has not been suspended.
  kRunning,

  //! \brief The process is suspended.
  kSuspended,
};

//! \brief Accesses information about another process, identified by a `HANDLE`.
class ProcessReaderWin {
 public:
  //! \brief Helper to make the context copyable and resizable.
  class ThreadContext {
   public:
    ThreadContext();
    ~ThreadContext() {}

    template <typename T>
    T* context() const {
      DCHECK(initialized_);
      return reinterpret_cast<T*>(
          const_cast<unsigned char*>(data_.data() + offset_));
    }
#if defined(ARCH_CPU_64_BITS)
    bool InitializeWow64(HANDLE thread_handle);
#endif  // ARCH_CPU_64_BITS
#if defined(ARCH_CPU_X86_64)
    // Initializes internal structures for extended compacted contexts.
    bool InitializeXState(HANDLE thread_handle, ULONG64 XStateCompactionMask);
#endif  // ARCH_CPU_X86_64
    void InitializeFromCurrentThread();
    bool InitializeNative(HANDLE thread_handle);

   private:
    // This is usually 0 but Windows might cause it to be positive when
    // fetching the extended context. This needs to be adjusted after
    // calls to InitializeContext2().
    size_t offset_;
    bool initialized_;
    std::vector<unsigned char> data_;
  };

  //! \brief Contains information about a thread that belongs to a process.
  struct Thread {
    Thread();
    ~Thread() {}

    ThreadContext context;
    std::string name;
    uint64_t id;
    WinVMAddress teb_address;
    WinVMSize teb_size;
    WinVMAddress stack_region_address;
    WinVMSize stack_region_size;
    uint32_t suspend_count;
    uint32_t priority_class;
    uint32_t priority;
  };

  ProcessReaderWin();

  ProcessReaderWin(const ProcessReaderWin&) = delete;
  ProcessReaderWin& operator=(const ProcessReaderWin&) = delete;

  ~ProcessReaderWin();

  //! \brief Initializes this object. This method must be called before any
  //!     other.
  //!
  //! \param[in] process Process handle, must have `PROCESS_QUERY_INFORMATION`,
  //!     `PROCESS_VM_READ`, and `PROCESS_DUP_HANDLE` access.
  //! \param[in] suspension_state Whether \a process has already been suspended
  //!     by the caller. Typically, this will be
  //!     ProcessSuspensionState::kSuspended, except for testing uses and where
  //!     the reader is reading itself.
  //!
  //! \return `true` on success, indicating that this object will respond
  //!     validly to further method calls. `false` on failure. On failure, no
  //!     further method calls should be made.
  //!
  //! \sa ScopedProcessSuspend
  bool Initialize(HANDLE process, ProcessSuspensionState suspension_state);

  //! \return `true` if the target task is a 64-bit process.
  bool Is64Bit() const { return process_info_.Is64Bit(); }

  //! \brief Return a memory reader for the target process.
  const ProcessMemoryWin* Memory() const { return &process_memory_; }

  //! \brief Determines the target process' start time.
  //!
  //! \param[out] start_time The time that the process started.
  //!
  //! \return `true` on success, `false` on failure, with a warning logged.
  bool StartTime(timeval* start_time) const;

  //! \brief Determines the target process' execution time.
  //!
  //! \param[out] user_time The amount of time the process has executed code in
  //!     user mode.
  //! \param[out] system_time The amount of time the process has executed code
  //!     in kernel mode.
  //!
  //! \return `true` on success, `false` on failure, with a warning logged.
  bool CPUTimes(timeval* user_time, timeval* system_time) const;

  //! \return The threads that are in the process. The first element (at index
  //!     `0`) corresponds to the main thread.
  const std::vector<Thread>& Threads();

  //! \return The modules loaded in the process. The first element (at index
  //!     `0`) corresponds to the main executable.
  const std::vector<ProcessInfo::Module>& Modules();

  //! \return A ProcessInfo object for the process being read.
  const ProcessInfo& GetProcessInfo() const;

  //! \brief Decrements the thread suspend counts for all thread ids other than
  //!     \a except_thread_id.
  //!
  //! Used to adjust the thread suspend count to correspond to the actual values
  //! for the process before Crashpad got involved.
  void DecrementThreadSuspendCounts(uint64_t except_thread_id);

 private:
  template <class Traits>
  void ReadThreadData(bool is_64_reading_32);

  HANDLE process_;
  ProcessInfo process_info_;
  ProcessMemoryWin process_memory_;
  std::vector<Thread> threads_;
  std::vector<ProcessInfo::Module> modules_;
  ProcessSuspensionState suspension_state_;
  bool initialized_threads_;
  InitializationStateDcheck initialized_;
};

}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_WIN_PROCESS_READER_WIN_H_
