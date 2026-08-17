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

// The data types used for communication between heapprofd and the client
// embedded in processes that are being profiled.

#ifndef SRC_PROFILING_MEMORY_WIRE_PROTOCOL_H_
#define SRC_PROFILING_MEMORY_WIRE_PROTOCOL_H_

#include <algorithm>
#include <cinttypes>

#include <unwindstack/Elf.h>
#include <unwindstack/MachineArm.h>
#include <unwindstack/MachineArm64.h>
#include <unwindstack/MachineRiscv64.h>
#include <unwindstack/MachineX86.h>
#include <unwindstack/MachineX86_64.h>

#include "perfetto/heap_profile.h"
#include "src/profiling/memory/shared_ring_buffer.h"
#include "src/profiling/memory/util.h"

namespace perfetto {

namespace base {
class UnixSocketRaw;
}

namespace profiling {

constexpr size_t kMaxRegisterDataSize =
    std::max({sizeof(uint32_t) * unwindstack::ARM_REG_LAST,
              sizeof(uint64_t) * unwindstack::ARM64_REG_LAST,
              sizeof(uint32_t) * unwindstack::X86_REG_LAST,
              sizeof(uint64_t) * unwindstack::X86_64_REG_LAST,
              sizeof(uint64_t) * unwindstack::RISCV64_REG_COUNT});

// Types needed for the wire format used for communication between the client
// and heapprofd. The basic format of a record sent by the client is
// record size (uint64_t) | record type (RecordType = uint64_t) | record
// If record type is Malloc, the record format is AllocMetadata | raw stack.
// If the record type is Free, the record is a FreeEntry.
// If record type is HeapName, the record is a HeapName.
// On connect, heapprofd sends one ClientConfiguration struct over the control
// socket.

// Use uint64_t to make sure the following data is aligned as 64bit is the
// strongest alignment requirement.

struct ClientConfigurationHeap {
  char name[HEAPPROFD_HEAP_NAME_SZ];
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) interval;
};

struct ClientConfiguration {
  // On average, sample one allocation every interval bytes,
  // If interval == 1, sample every allocation.
  // Must be >= 1.
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) default_interval;
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) block_client_timeout_us;
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) num_heaps;
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) adaptive_sampling_shmem_threshold;
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t)
  adaptive_sampling_max_sampling_interval_bytes;
  alignas(8) ClientConfigurationHeap heaps[64];
  PERFETTO_CROSS_ABI_ALIGNED(bool) block_client;
  PERFETTO_CROSS_ABI_ALIGNED(bool) disable_fork_teardown;
  PERFETTO_CROSS_ABI_ALIGNED(bool) disable_vfork_detection;
  PERFETTO_CROSS_ABI_ALIGNED(bool) all_heaps;
  // Just double check that the array sizes are in correct order.
};

enum class RecordType : uint64_t {
  Free = 0,
  Malloc = 1,
  HeapName = 2,
};

// Make the whole struct 8-aligned. This is to make sizeof(AllocMetadata)
// the same on 32 and 64-bit.
struct alignas(8) AllocMetadata {
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) sequence_number;
  // Size of the allocation that was made.
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) alloc_size;
  // Total number of bytes attributed to this allocation.
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) sample_size;
  // Pointer returned by malloc(2) for this allocation.
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) alloc_address;
  // Current value of the stack pointer.
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) stack_pointer;
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) clock_monotonic_coarse_timestamp;
  // unwindstack::AsmGetRegs assumes this is aligned.
  alignas(8) char register_data[kMaxRegisterDataSize];
  PERFETTO_CROSS_ABI_ALIGNED(uint32_t) heap_id;
  // CPU architecture of the client.
  PERFETTO_CROSS_ABI_ALIGNED(unwindstack::ArchEnum) arch;
};

struct FreeEntry {
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) sequence_number;
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) addr;
  PERFETTO_CROSS_ABI_ALIGNED(uint32_t) heap_id;
};

struct HeapName {
  PERFETTO_CROSS_ABI_ALIGNED(uint64_t) sample_interval;
  PERFETTO_CROSS_ABI_ALIGNED(uint32_t) heap_id;
  PERFETTO_CROSS_ABI_ALIGNED(char) heap_name[HEAPPROFD_HEAP_NAME_SZ];
};

// Make sure the sizes do not change on different architectures.
static_assert(sizeof(AllocMetadata) == 328,
              "AllocMetadata needs to be the same size across ABIs.");
static_assert(sizeof(FreeEntry) == 24,
              "FreeEntry needs to be the same size across ABIs.");
static_assert(sizeof(HeapName) == 80,
              "HeapName needs to be the same size across ABIs.");
static_assert(sizeof(ClientConfiguration) == 4656,
              "ClientConfiguration needs to be the same size across ABIs.");

enum HandshakeFDs : size_t {
  kHandshakeMaps = 0,
  kHandshakeMem,
  kHandshakeSize,
};

struct WireMessage {
  RecordType record_type;

  AllocMetadata* alloc_header;
  FreeEntry* free_header;
  HeapName* heap_name_header;

  char* payload;
  size_t payload_size;
};

int64_t SendWireMessage(SharedRingBuffer* buf, const WireMessage& msg);

// Parse message received over the wire.
// |buf| has to outlive |out|.
// If buf is not a valid message, return false.
bool ReceiveWireMessage(char* buf, size_t size, WireMessage* out);

uint64_t GetHeapSamplingInterval(const ClientConfiguration& cli_config,
                                 const char* heap_name);

constexpr const char* kHeapprofdSocketEnvVar = "ANDROID_SOCKET_heapprofd";
constexpr const char* kHeapprofdSocketFile = "/dev/socket/heapprofd";

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_MEMORY_WIRE_PROTOCOL_H_
