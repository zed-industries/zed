// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_FUNCTIONS_H_
#error This header is meant to be included only once by allocator_shim*.cc except allocator_shim_win_component.cc
#endif

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_FUNCTIONS_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_FUNCTIONS_H_

#include <atomic>
#include <cstddef>
#include <new>

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_check.h"
#include "partition_alloc/shim/allocator_dispatch.h"
#include "partition_alloc/shim/allocator_shim.h"
#include "partition_alloc/shim/allocator_shim_internals.h"

#if PA_BUILDFLAG(IS_WIN)
#include "partition_alloc/shim/winheap_stubs_win.h"
#endif

namespace allocator_shim {
namespace internal {

std::atomic<const allocator_shim::AllocatorDispatch*> g_chain_head{
    &allocator_shim::AllocatorDispatch::default_dispatch};

bool g_call_new_handler_on_malloc_failure = false;

// Calls the std::new handler thread-safely. Returns true if a new_handler was
// set and called, false if no new_handler was set.
bool CallNewHandler(size_t size) {
#if PA_BUILDFLAG(IS_WIN)
  return allocator_shim::WinCallNewHandler(size);
#else
  std::new_handler nh = std::get_new_handler();
  if (!nh) {
    return false;
  }
  (*nh)();
  // Assume the new_handler will abort if it fails. Exception are disabled and
  // we don't support the case of a new_handler throwing std::bad_balloc.
  return true;
#endif
}

#if !(PA_BUILDFLAG(IS_WIN) && defined(COMPONENT_BUILD))
PA_ALWAYS_INLINE
#endif
const allocator_shim::AllocatorDispatch* GetChainHead() {
  return internal::g_chain_head.load(std::memory_order_relaxed);
}

}  // namespace internal

void SetCallNewHandlerOnMallocFailure(bool value) {
  internal::g_call_new_handler_on_malloc_failure = value;
}

void* UncheckedAlloc(size_t size) {
  const AllocatorDispatch* const chain_head = internal::GetChainHead();
  return chain_head->alloc_unchecked_function(size, nullptr);
}

void* UncheckedRealloc(void* ptr, size_t size) {
  const AllocatorDispatch* const chain_head = internal::GetChainHead();
  return chain_head->realloc_unchecked_function(ptr, size, nullptr);
}

void UncheckedFree(void* ptr) {
  const AllocatorDispatch* const chain_head = internal::GetChainHead();
  return chain_head->free_function(ptr, nullptr);
}

void* UncheckedAlignedAlloc(size_t size, size_t align) {
  const AllocatorDispatch* const chain_head = internal::GetChainHead();
  return chain_head->aligned_malloc_unchecked_function(size, align, nullptr);
}

void* UncheckedAlignedRealloc(void* ptr, size_t size, size_t align) {
  const AllocatorDispatch* const chain_head = internal::GetChainHead();
  return chain_head->aligned_realloc_unchecked_function(ptr, size, align,
                                                        nullptr);
}

void UncheckedAlignedFree(void* ptr) {
  const AllocatorDispatch* const chain_head = internal::GetChainHead();
  return chain_head->aligned_free_function(ptr, nullptr);
}

void InsertAllocatorDispatch(AllocatorDispatch* dispatch) {
  // Loop in case of (an unlikely) race on setting the list head.
  constexpr size_t kMaxRetries = 7;
  const AllocatorDispatch original_dispatch = *dispatch;
  for (size_t i = 0; i < kMaxRetries; ++i) {
    const AllocatorDispatch* chain_head = internal::GetChainHead();

    dispatch->OptimizeAllocatorDispatchTable(&original_dispatch, chain_head);
    dispatch->next = chain_head;

    // This function guarantees to be thread-safe w.r.t. concurrent
    // insertions. It also has to guarantee that all the threads always
    // see a consistent chain, hence the atomic_thread_fence() below.
    // InsertAllocatorDispatch() is NOT a fastpath, as opposite to malloc(), so
    // we don't really want this to be a release-store with a corresponding
    // acquire-load during malloc().
    std::atomic_thread_fence(std::memory_order_seq_cst);
    // Set the chain head to the new dispatch atomically. If we lose the race,
    // retry.
    if (internal::g_chain_head.compare_exchange_strong(
            chain_head, dispatch, std::memory_order_relaxed,
            std::memory_order_relaxed)) {
      // Success.
      return;
    }
  }

  PA_CHECK(false);  // Too many retries, this shouldn't happen.
}

void RemoveAllocatorDispatchForTesting(AllocatorDispatch* dispatch) {
  // See `AllocatorDispatch::OptimizeAllocatorDispatchTable`. Only the chain
  // head can be removed. Otherwise, the optimization gets broken.
  PA_DCHECK(internal::GetChainHead() == dispatch);
  internal::g_chain_head.store(dispatch->next, std::memory_order_relaxed);
}

const AllocatorDispatch* GetAllocatorDispatchChainHeadForTesting() {
  return internal::GetChainHead();
}

AutoResetAllocatorDispatchChainForTesting::
    AutoResetAllocatorDispatchChainForTesting() {
  original_dispatch_ = internal::g_chain_head.exchange(
      &allocator_shim::AllocatorDispatch::default_dispatch);
}

AutoResetAllocatorDispatchChainForTesting::
    ~AutoResetAllocatorDispatchChainForTesting() {
  internal::g_chain_head = original_dispatch_;
}

}  // namespace allocator_shim

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_FUNCTIONS_H_
