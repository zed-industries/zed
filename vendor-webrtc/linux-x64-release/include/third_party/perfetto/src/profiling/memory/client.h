/*
 * Copyright (C) 2018 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_PROFILING_MEMORY_CLIENT_H_
#define SRC_PROFILING_MEMORY_CLIENT_H_

#include <stddef.h>
#include <sys/types.h>

#include <atomic>
#include <condition_variable>
#include <mutex>
#include <vector>

#include <unwindstack/Arch.h>

#include "perfetto/base/build_config.h"
#include "perfetto/base/compiler.h"
#include "perfetto/ext/base/unix_socket.h"
#include "src/profiling/memory/sampler.h"
#include "src/profiling/memory/shared_ring_buffer.h"
#include "src/profiling/memory/unhooked_allocator.h"
#include "src/profiling/memory/wire_protocol.h"

namespace perfetto {
namespace profiling {

struct StackRange {
  const char* begin;
  // One past the highest address part of the stack.
  const char* end;
};

StackRange GetThreadStackRange();
StackRange GetSigAltStackRange();
StackRange GetMainThreadStackRange();

constexpr uint64_t kInfiniteTries = 0;
constexpr uint32_t kClientSockTimeoutMs = 1000;

uint64_t GetMaxTries(const ClientConfiguration& client_config);

// Profiling client, used to sample and record the malloc/free family of calls,
// and communicate the necessary state to a separate profiling daemon process.
//
// Created and owned by the malloc hooks.
//
// Methods of this class are thread-safe unless otherwise stated, in which case
// the caller needs to synchronize calls behind a mutex or similar.
//
// Implementation warning: this class should not use any heap, as otherwise its
// destruction would enter the possibly-hooked |free|, which can reference the
// Client itself. If avoiding the heap is not possible, then look at using
// UnhookedAllocator.
class Client {
 public:
  // Returns a client that is ready for sampling allocations, using the given
  // socket (which should already be connected to heapprofd).
  //
  // Returns a shared_ptr since that is how the client will ultimately be used,
  // and to take advantage of std::allocate_shared putting the object & the
  // control block in one block of memory.
  static std::shared_ptr<Client> CreateAndHandshake(
      base::UnixSocketRaw sock,
      UnhookedAllocator<Client> unhooked_allocator);

  static std::optional<base::UnixSocketRaw> ConnectToHeapprofd(
      const std::string& sock_name);

  bool RecordMalloc(uint32_t heap_id,
                    uint64_t sample_size,
                    uint64_t alloc_size,
                    uint64_t alloc_address) PERFETTO_WARN_UNUSED_RESULT;

  // Add address to buffer of deallocations. Flushes the buffer if necessary.
  bool RecordFree(uint32_t heap_id,
                  uint64_t alloc_address) PERFETTO_WARN_UNUSED_RESULT;
  bool RecordHeapInfo(uint32_t heap_id,
                      const char* heap_name,
                      uint64_t interval);

  void AddClientSpinlockBlockedUs(size_t n) {
    shmem_.AddClientSpinlockBlockedUs(n);
  }

  // Public for std::allocate_shared. Use CreateAndHandshake() to create
  // instances instead.
  Client(base::UnixSocketRaw sock,
         ClientConfiguration client_config,
         SharedRingBuffer shmem,
         pid_t pid_at_creation,
         StackRange main_thread_stack_range);

  ~Client();

  const ClientConfiguration& client_config() { return client_config_; }
  uint64_t adaptive_sampling_shmem_threshold() {
    return client_config_.adaptive_sampling_shmem_threshold;
  }
  uint64_t adaptive_sampling_max_sampling_interval_bytes() {
    return client_config_.adaptive_sampling_max_sampling_interval_bytes;
  }
  uint64_t write_avail() { return shmem_.write_avail(); }

  bool IsConnected();

 private:
#if PERFETTO_BUILDFLAG(PERFETTO_ARCH_CPU_RISCV) && \
    !PERFETTO_HAS_BUILTIN_STACK_ADDRESS()
  // For specific architectures, such as riscv, different calling conventions
  // make a difference in the meaning of the frame pointer. (see comments in
  // client.cc) So, we want to use other method to get the stack address for
  // specific architectures such as riscv.
  ssize_t GetStackRegister(unwindstack::ArchEnum arch);
  uintptr_t GetStackAddress(char* reg_data, unwindstack::ArchEnum arch);
#endif
  const char* GetStackEnd(const char* stacktop);
  bool SendControlSocketByte() PERFETTO_WARN_UNUSED_RESULT;
  int64_t SendWireMessageWithRetriesIfBlocking(const WireMessage&)
      PERFETTO_WARN_UNUSED_RESULT;

  bool IsPostFork();

  ClientConfiguration client_config_;
  uint64_t max_shmem_tries_;
  base::UnixSocketRaw sock_;

  StackRange main_thread_stack_range_{nullptr, nullptr};
  std::atomic<uint64_t>
      sequence_number_[base::ArraySize(ClientConfiguration{}.heaps)] = {};
  SharedRingBuffer shmem_;

  // Used to detect (during the slow path) the situation where the process has
  // forked during profiling, and is performing malloc operations in the child.
  // In this scenario, we want to stop profiling in the child, as otherwise
  // it'll proceed to write to the same shared buffer & control socket (with
  // duplicate sequence ids).
  const pid_t pid_at_creation_;
  bool detected_fork_ = false;
  bool postfork_return_value_ = false;
};

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_MEMORY_CLIENT_H_
