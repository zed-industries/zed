// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_DISPATCHER_TLS_H_
#define BASE_ALLOCATOR_DISPATCHER_TLS_H_

#include <string_view>

#include "build/build_config.h"

#if BUILDFLAG(IS_POSIX)  // the current allocation mechanism (mmap) and TLS
                         // support (pthread) are both defined by POSIX
#define USE_LOCAL_TLS_EMULATION() true
#else
#define USE_LOCAL_TLS_EMULATION() false
#endif

#if USE_LOCAL_TLS_EMULATION()
#include <pthread.h>

#include <algorithm>
#include <atomic>
#include <functional>
#include <memory>
#include <mutex>

#include "base/base_export.h"
#include "base/check.h"
#include "base/compiler_specific.h"

#if PA_BUILDFLAG(USE_PARTITION_ALLOC)
#include "partition_alloc/partition_alloc_constants.h"  // nogncheck
#endif

#if HAS_FEATURE(thread_sanitizer)
#define DISABLE_TSAN_INSTRUMENTATION __attribute__((no_sanitize("thread")))
#else
#define DISABLE_TSAN_INSTRUMENTATION
#endif

#define STR_HELPER(x) #x
#define STR(x) STR_HELPER(x)

// Verify that a condition holds and cancel the process in case it doesn't. The
// functionality is similar to RAW_CHECK but includes more information in the
// logged messages. It is non allocating to prevent recursions.
#define TLS_RAW_CHECK(error_message, condition) \
  TLS_RAW_CHECK_IMPL(error_message, condition, __FILE__, __LINE__)

#define TLS_RAW_CHECK_IMPL(error_message, condition, file, line)        \
  do {                                                                  \
    if (!(condition)) {                                                 \
      constexpr const char* message =                                   \
          "TLS System: " error_message " Failed condition '" #condition \
          "' in (" file "@" STR(line) ").\n";                           \
      ::logging::RawCheckFailure(message);                              \
    }                                                                   \
  } while (0)

namespace base::debug {
struct CrashKeyString;
}

namespace base::allocator::dispatcher {
namespace internal {

// Allocate memory using POSIX' mmap and unmap functionality. The allocator
// implements the allocator interface required by ThreadLocalStorage.
struct BASE_EXPORT MMapAllocator {
// The minimum size of a memory chunk when allocating. Even for chunks with
// fewer bytes, at least AllocationChunkSize bytes are allocated. For mmap, this
// is usually the page size of the system.
// For various OS-CPU combinations, partition_alloc::PartitionPageSize() is not
// constexpr. Hence, we can not use this value but define it locally.
#if defined(PAGE_ALLOCATOR_CONSTANTS_ARE_CONSTEXPR) && \
    PAGE_ALLOCATOR_CONSTANTS_ARE_CONSTEXPR
  constexpr static size_t AllocationChunkSize =
      partition_alloc::PartitionPageSize();
#elif BUILDFLAG(IS_APPLE)
  constexpr static size_t AllocationChunkSize = 16384;
#elif BUILDFLAG(IS_ANDROID) && defined(ARCH_CPU_64_BITS)
  constexpr static size_t AllocationChunkSize = 16384;
#elif BUILDFLAG(IS_LINUX) && defined(ARCH_CPU_ARM64)
  constexpr static size_t AllocationChunkSize = 16384;
#else
  constexpr static size_t AllocationChunkSize = 4096;
#endif

  // Allocate size_in_bytes bytes of raw memory. Return nullptr if allocation
  // fails.
  void* AllocateMemory(size_t size_in_bytes);
  // Free the raw memory pointed to by pointer_to_allocated. Returns a boolean
  // value indicating if the free was successful.
  bool FreeMemoryForTesting(void* pointer_to_allocated, size_t size_in_bytes);
};

// The allocator used by default for the thread local storage.
using DefaultAllocator = MMapAllocator;

using OnThreadTerminationFunction = void (*)(void*);

// The TLS system used by default for the thread local storage. It stores and
// retrieves thread specific data pointers.
class BASE_EXPORT PThreadTLSSystem {
 public:
  PThreadTLSSystem();

  PThreadTLSSystem(const PThreadTLSSystem&) = delete;
  PThreadTLSSystem(PThreadTLSSystem&&);
  PThreadTLSSystem& operator=(const PThreadTLSSystem&) = delete;
  PThreadTLSSystem& operator=(PThreadTLSSystem&&);

  // Initialize the TLS system to store a data set for different threads.
  // @param thread_termination_function An optional function which will be
  // invoked upon termination of a thread.
  bool Setup(OnThreadTerminationFunction thread_termination_function,
             std::string_view instance_id);
  // Tear down the TLS system. After completing tear down, the thread
  // termination function passed to Setup will not be invoked anymore.
  bool TearDownForTesting();

  // Get the pointer to the data associated to the current thread. Returns
  // nullptr if the TLS system is not initialized or no data was set before.
  void* GetThreadSpecificData();
  // Set the pointer to the data associated to the current thread. Return true
  // if stored successfully, false otherwise.
  bool SetThreadSpecificData(void* data);

 private:
  base::debug::CrashKeyString* crash_key_ = nullptr;
  pthread_key_t data_access_key_ = 0;
#if DCHECK_IS_ON()
  // From POSIX standard at https://www.open-std.org/jtc1/sc22/open/n4217.pdf:
  // The effect of calling pthread_getspecific() or pthread_setspecific() with a
  // key value not obtained from pthread_key_create() or after key has been
  // deleted with pthread_key_delete() is undefined.
  //
  // Unfortunately, POSIX doesn't define a special value of pthread_key_t
  // indicating an invalid key which would allow us to detect accesses outside
  // of initialized state. Hence, to prevent us from drifting into the evil
  // realm of undefined behaviour we store whether we're somewhere between Setup
  // and Teardown.
  std::atomic_bool initialized_{false};
#endif
};

using DefaultTLSSystem = PThreadTLSSystem;

// In some scenarios, most notably when testing, the allocator and TLS system
// passed to |ThreadLocalStorage| are not copyable and have to be wrapped, i.e.
// using std::reference_wrapper. |dereference| is a small helper to retrieve the
// underlying value.
template <typename T>
T& dereference(T& ref) {
  return ref;
}

template <typename T>
T& dereference(std::reference_wrapper<T>& ref) {
  // std::reference_wrapper requires a valid reference for construction,
  // therefore, no need in checking here.
  return ref.get();
}

// Store thread local data. The data is organized in chunks, where each chunk
// holds |ItemsPerChunk|. Each item may be free or used.
//
// When a thread requests data, the chunks are searched for a free data item,
// which is registered for this thread and marked as |used|. Further requests by
// this thread will then always return the same item. When a thread terminates,
// the item will be reset and return to the pool of free items.
//
// Upon construction, the first chunk is created. If a thread requests data and
// there is no free item available, another chunk is created. Upon destruction,
// all memory is freed. Pointers to data items become invalid!
//
// Constructor and destructor are not thread safe.
//
// @tparam PayloadType The item type to be stored.
// @tparam AllocatorType The allocator being used. An allocator must provide
// the following interface:
//  void* AllocateMemory(size_t size_in_bytes); // Allocate size_in_bytes bytes
//  of raw memory.
//  void FreeMemory(void* pointer_to_allocated, size_t size_in_bytes); // Free
//  the raw memory pointed to by pointer_to_allocated.
// Any failure in allocation or free must terminate the process.
// @tparam TLSSystemType The TLS system being used. A TLS system must provide
// the following interface:
//  bool Setup(OnThreadTerminationFunction thread_termination_function);
//  bool Destroy();
//  void* GetThreadSpecificData();
//  bool SetThreadSpecificData(void* data);
// @tparam AllocationChunkSize The minimum size of a memory chunk that the
// allocator can handle. We try to size the chunks so that each chunk uses this
// size to the maximum.
// @tparam IsDestructibleForTesting For testing purposes we allow the destructor
// to perform clean up upon destruction. Otherwise, using the destructor will
// result in a compilation failure.
template <typename PayloadType,
          typename AllocatorType,
          typename TLSSystemType,
          size_t AllocationChunkSize,
          bool IsDestructibleForTesting>
struct ThreadLocalStorage {
  explicit ThreadLocalStorage(std::string_view instance_id)
      : root_(AllocateAndInitializeChunk()) {
    Initialize(instance_id);
  }

  // Create a new instance of |ThreadLocalStorage| using the passed allocator
  // and TLS system. This initializes the underlying TLS system and creates the
  // first chunk of data.
  ThreadLocalStorage(std::string_view instance_id,
                     AllocatorType allocator,
                     TLSSystemType tls_system)
      : allocator_(std::move(allocator)),
        tls_system_(std::move(tls_system)),
        root_(AllocateAndInitializeChunk()) {
    Initialize(instance_id);
  }

  // Deletes an instance of |ThreadLocalStorage| and delete all the data chunks
  // created.
  ~ThreadLocalStorage() {
    if constexpr (IsDestructibleForTesting) {
      TearDownForTesting();
    } else if constexpr (!IsDestructibleForTesting) {
      static_assert(
          IsDestructibleForTesting,
          "ThreadLocalStorage cannot be destructed outside of test code.");
    }
  }

  // Explicitly prevent all forms of Copy/Move construction/assignment. For an
  // exact copy of ThreadLocalStorage we would need to copy the mapping of
  // thread to item, which we can't do at the moment. On the other side, our
  // atomic members do not support moving out of the box.
  ThreadLocalStorage(const ThreadLocalStorage&) = delete;
  ThreadLocalStorage(ThreadLocalStorage&& other) = delete;
  ThreadLocalStorage& operator=(const ThreadLocalStorage&) = delete;
  ThreadLocalStorage& operator=(ThreadLocalStorage&&) = delete;

  // Get the data item for the current thread. If no data is registered so far,
  // find a free item in the chunks and register it for the current thread.
  PayloadType* GetThreadLocalData() {
    auto& tls_system = dereference(tls_system_);

    auto* slot = static_cast<SingleSlot*>(tls_system.GetThreadSpecificData());

    if (slot == nullptr) [[unlikely]] {
      slot = FindAndAllocateFreeSlot(root_.load(std::memory_order_relaxed));

      // We might be called in the course of handling a memory allocation. We do
      // not use CHECK since they might allocate and cause a recursion.
      TLS_RAW_CHECK("Failed to set thread specific data.",
                    tls_system.SetThreadSpecificData(slot));

      // Reset the content to wipe out any previous data.
      Reset(slot->item);
    }

    return &(slot->item);
  }

 private:
  // Encapsulate the payload item and some administrative data.
  struct SingleSlot {
    PayloadType item;
#if !defined(__cpp_lib_atomic_value_initialization) || \
    __cpp_lib_atomic_value_initialization < 201911L
    std::atomic_flag is_used = ATOMIC_FLAG_INIT;
#else
    std::atomic_flag is_used;
#endif
  };

  template <size_t NumberOfItems>
  struct ChunkT {
    SingleSlot slots[NumberOfItems];
    // Pointer to the next chunk.
    std::atomic<ChunkT*> next_chunk = nullptr;
    // Helper flag to ensure we create the next chunk only once in a multi
    // threaded environment.
    std::once_flag create_next_chunk_flag;
  };

  template <size_t LowerNumberOfItems,
            size_t UpperNumberOfItems,
            size_t NumberOfBytes>
  static constexpr size_t CalculateEffectiveNumberOfItemsBinSearch() {
    if constexpr (LowerNumberOfItems == UpperNumberOfItems) {
      return LowerNumberOfItems;
    }

    constexpr size_t CurrentNumberOfItems =
        (UpperNumberOfItems - LowerNumberOfItems) / 2 + LowerNumberOfItems;

    if constexpr (sizeof(ChunkT<CurrentNumberOfItems>) > NumberOfBytes) {
      return CalculateEffectiveNumberOfItemsBinSearch<
          LowerNumberOfItems, CurrentNumberOfItems, NumberOfBytes>();
    }

    if constexpr (sizeof(ChunkT<CurrentNumberOfItems + 1>) < NumberOfBytes) {
      return CalculateEffectiveNumberOfItemsBinSearch<
          CurrentNumberOfItems + 1, UpperNumberOfItems, NumberOfBytes>();
    }

    return CurrentNumberOfItems;
  }

  // Calculate the maximum number of items we can store in one chunk without the
  // size of the chunk exceeding NumberOfBytes. To avoid things like alignment
  // and packing tampering with the calculation, instead of calculating the
  // correct number of items we use sizeof-operator against ChunkT to search for
  // the correct size. Unfortunately, the number of recursions is limited by the
  // compiler. Therefore, we use a binary search instead of a simple linear
  // search.
  template <size_t MinimumNumberOfItems, size_t NumberOfBytes>
  static constexpr size_t CalculateEffectiveNumberOfItems() {
    if constexpr (sizeof(ChunkT<MinimumNumberOfItems>) < NumberOfBytes) {
      constexpr size_t LowerNumberOfItems = MinimumNumberOfItems;
      constexpr size_t UpperNumberOfItems =
          NumberOfBytes / sizeof(PayloadType) + 1;
      return CalculateEffectiveNumberOfItemsBinSearch<
          LowerNumberOfItems, UpperNumberOfItems, NumberOfBytes>();
    }

    return MinimumNumberOfItems;
  }

 public:
  // The minimum number of items per chunk. It should be high enough to
  // accommodate most items in the root chunk whilst not wasting to much space
  // on unnecessary items.
  static constexpr size_t MinimumNumberOfItemsPerChunk = 75;
  // The effective number of items per chunk. We use the AllocationChunkSize as
  // a hint to calculate to effective number of items so we occupy one of these
  // memory chunks to the maximum extent possible.
  static constexpr size_t ItemsPerChunk =
      CalculateEffectiveNumberOfItems<MinimumNumberOfItemsPerChunk,
                                      AllocationChunkSize>();

 private:
  using Chunk = ChunkT<ItemsPerChunk>;

  static_assert(ItemsPerChunk >= MinimumNumberOfItemsPerChunk);

  // Mark an item's slot ready for reuse. This function is used as thread
  // termination function in the TLS system. We do not destroy anything at this
  // point but simply mark the slot as unused.
  static void MarkSlotAsFree(void* data) {
    // We always store SingleSlots in the TLS system. Therefore, we cast to
    // SingleSlot and reset the is_used flag.
    auto* const slot = static_cast<SingleSlot*>(data);

    // We might be called in the course of handling a memory allocation.
    // Therefore, do not use CHECK since it might allocate and cause a
    // recursion.
    TLS_RAW_CHECK("Received an invalid slot.",
                  slot && slot->is_used.test_and_set());

    slot->is_used.clear(std::memory_order_relaxed);
  }

  // Perform common initialization during construction of an instance.
  void Initialize(std::string_view instance_id) {
    // The constructor must be called outside of the allocation path. Therefore,
    // it is secure to verify with CHECK.

    // Passing MarkSlotAsFree as thread_termination_function we ensure the
    // slot/item assigned to the finished thread will be returned to the pool of
    // unused items.
    CHECK(dereference(tls_system_).Setup(&MarkSlotAsFree, instance_id));
  }

  Chunk* AllocateAndInitializeChunk() {
    void* const uninitialized_memory =
        dereference(allocator_).AllocateMemory(sizeof(Chunk));

    // We might be called in the course of handling a memory allocation. We do
    // not use CHECK since they might allocate and cause a recursion.
    TLS_RAW_CHECK("Failed to allocate memory for new chunk.",
                  uninitialized_memory != nullptr);

    return new (uninitialized_memory) Chunk{};
  }

  void FreeAndDeallocateChunkForTesting(Chunk* chunk_to_erase) {
    chunk_to_erase->~Chunk();

    // FreeAndDeallocateChunkForTesting must be called outside of the allocation
    // path. Therefore, it is secure to verify with CHECK.
    CHECK(dereference(allocator_)
              .FreeMemoryForTesting(chunk_to_erase, sizeof(Chunk)));
  }

  // Find a free slot in the passed chunk, reserve it and return it to the
  // caller. If no free slot can be found, head on to the next chunk. If the
  // next chunk doesn't exist, create it.
  SingleSlot* FindAndAllocateFreeSlot(Chunk* const chunk) {
    SingleSlot* const slot = std::find_if_not(
        std::begin(chunk->slots), std::end(chunk->slots),
        [](SingleSlot& candidate_slot) {
          return candidate_slot.is_used.test_and_set(std::memory_order_relaxed);
        });

    // So we found a slot. Happily return it to the caller.
    if (slot != std::end(chunk->slots)) {
      return slot;
    }

    // Ok, there are no more free slots in this chunk. First, ensure the next
    // chunk is valid and create one if necessary.
    std::call_once(chunk->create_next_chunk_flag, [&] {
      // From https://eel.is/c++draft/thread.once.callonce#3
      //
      // Synchronization: For any given once_­flag: all active executions occur
      // in a total order; completion of an active execution synchronizes with
      // the start of the next one in this total order; and the returning
      // execution synchronizes with the return from all passive executions.
      //
      // Therefore, we do only a relaxed store here, call_once synchronizes with
      // other threads.
      chunk->next_chunk.store(AllocateAndInitializeChunk(),
                              std::memory_order_relaxed);
    });

    return FindAndAllocateFreeSlot(chunk->next_chunk);
  }

  template <bool IsDestructibleForTestingP = IsDestructibleForTesting>
  typename std::enable_if<IsDestructibleForTestingP>::type
  TearDownForTesting() {
    // The destructor must be called outside of the allocation path. Therefore,
    // it is secure to verify with CHECK.

    // All accessing threads must be terminated by now. For additional security
    // we tear down the TLS system first. This way we ensure that
    // MarkSlotAsFree is not called anymore and we have no accesses from the
    // TLS system's side.
    CHECK(dereference(tls_system_).TearDownForTesting());

    // Delete all data chunks.
    for (auto* chunk = root_.load(); chunk != nullptr;) {
      auto* next_chunk = chunk->next_chunk.load();
      FreeAndDeallocateChunkForTesting(chunk);
      chunk = next_chunk;
    }
  }

  // Reset a single item to its default value.
  // Since items are re-used, they may be accessed from different threads,
  // causing TSan to trigger. Therefore, the reset is exempt from TSan
  // instrumentation.
  DISABLE_TSAN_INSTRUMENTATION void Reset(PayloadType& item) { item = {}; }

  AllocatorType allocator_;
  TLSSystemType tls_system_;
  std::atomic<Chunk*> const root_;
};

}  // namespace internal

// The ThreadLocalStorage visible to the user. This uses the internal default
// allocator and TLS system.
template <typename StorageType,
          typename AllocatorType = internal::DefaultAllocator,
          typename TLSSystemType = internal::DefaultTLSSystem,
          size_t AllocationChunkSize = AllocatorType::AllocationChunkSize,
          bool IsDestructibleForTesting = false>
using ThreadLocalStorage =
    internal::ThreadLocalStorage<StorageType,
                                 AllocatorType,
                                 TLSSystemType,
                                 AllocationChunkSize,
                                 IsDestructibleForTesting>;

}  // namespace base::allocator::dispatcher

#undef TLS_RAW_CHECK_IMPL
#undef TLS_RAW_CHECK
#undef STR
#undef STR_HELPER

#endif  // USE_LOCAL_TLS_EMULATION()
#endif  // BASE_ALLOCATOR_DISPATCHER_TLS_H_
