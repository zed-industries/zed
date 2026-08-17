/*===- CtxInstrProfiling.h- Contextual instrumentation-based PGO  ---------===*\
|*
|* Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
|* See https://llvm.org/LICENSE.txt for license information.
|* SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
|*
\*===----------------------------------------------------------------------===*/

#ifndef CTX_PROFILE_CTXINSTRPROFILING_H_
#define CTX_PROFILE_CTXINSTRPROFILING_H_

#include "CtxInstrContextNode.h"
#include "sanitizer_common/sanitizer_dense_map.h"
#include "sanitizer_common/sanitizer_mutex.h"
#include <sanitizer/common_interface_defs.h>

using namespace llvm::ctx_profile;

// Forward-declare for the one unittest checking Arena construction zeroes out
// its allocatable space.
class ArenaTest_ZeroInit_Test;
namespace __ctx_profile {

static constexpr size_t ExpectedAlignment = 8;
// We really depend on this, see further below. We currently support x86_64.
// When we want to support other archs, we need to trace the places Alignment is
// used and adjust accordingly.
static_assert(sizeof(void *) == ExpectedAlignment);

/// Arena (bump allocator) forming a linked list. Intentionally not thread safe.
/// Allocation and de-allocation happen using sanitizer APIs. We make that
/// explicit.
class Arena final {
public:
  // When allocating a new Arena, optionally specify an existing one to append
  // to, assumed to be the last in the Arena list. We only need to support
  // appending to the arena list.
  static Arena *allocateNewArena(size_t Size, Arena *Prev = nullptr);
  static void freeArenaList(Arena *&A);

  uint64_t size() const { return Size; }

  // Allocate S bytes or return nullptr if we don't have that many available.
  char *tryBumpAllocate(size_t S) {
    if (Pos + S > Size)
      return nullptr;
    Pos += S;
    return start() + (Pos - S);
  }

  Arena *next() const { return Next; }

  // the beginning of allocatable memory.
  const char *start() const { return const_cast<Arena *>(this)->start(); }
  const char *pos() const { return start() + Pos; }

private:
  friend class ::ArenaTest_ZeroInit_Test;
  explicit Arena(uint32_t Size);
  ~Arena() = delete;

  char *start() { return reinterpret_cast<char *>(&this[1]); }

  Arena *Next = nullptr;
  uint64_t Pos = 0;
  const uint64_t Size;
};

// The memory available for allocation follows the Arena header, and we expect
// it to be thus aligned.
static_assert(alignof(Arena) == ExpectedAlignment);

// Verify maintenance to ContextNode doesn't change this invariant, which makes
// sure the inlined vectors are appropriately aligned.
static_assert(alignof(ContextNode) == ExpectedAlignment);

/// ContextRoots hold memory and the start of the contextual profile tree for a
/// root function.
struct ContextRoot {
  ContextNode *FirstNode = nullptr;
  Arena *FirstMemBlock = nullptr;
  Arena *CurrentMem = nullptr;

  // Count the number of entries - regardless if we could take the `Taken` mutex
  ::__sanitizer::atomic_uint64_t TotalEntries = {};

  // Profiles for functions we encounter when collecting a contexutal profile,
  // that are not associated with a callsite. This is expected to happen for
  // signal handlers, but it also - problematically - currently happens for
  // call sites generated after profile instrumentation, primarily
  // mem{memset|copy|move|set}.
  // `Unhandled` serves 2 purposes:
  //   1. identifying such cases (like the memops)
  //   2. collecting a profile for them, which can be at least used as a flat
  //   profile
  ::__sanitizer::DenseMap<GUID, ContextNode *> Unhandled;
  // Keep the unhandled contexts in a list, as we allocate them, as it makes it
  // simpler to send to the writer when the profile is fetched.
  ContextNode *FirstUnhandledCalleeNode = nullptr;

  // Taken is used to ensure only one thread traverses the contextual graph -
  // either to read it or to write it. On server side, the same entrypoint will
  // be entered by numerous threads, but over time, the profile aggregated by
  // collecting sequentially on one thread at a time is expected to converge to
  // the aggregate profile that may have been observable on all the threads.
  // Note that this is node-by-node aggregation, i.e. summing counters of nodes
  // at the same position in the graph, not flattening.
  // Threads that cannot lock Taken (fail TryLock) are given a "scratch context"
  // - a buffer they can clobber, safely from a memory access perspective.
  //
  // Note about "scratch"-ness: we currently ignore the data written in them
  // (which is anyway clobbered). The design allows for that not be the case -
  // because "scratch"-ness is first and foremost about not trying to build
  // subcontexts, and is captured by tainting the pointer value (pointer to the
  // memory treated as context), but right now, we drop that info.
  //
  // We could consider relaxing the requirement of more than one thread
  // entering by holding a few context trees per entrypoint and then aggregating
  // them (as explained above) at the end of the profile collection - it's a
  // tradeoff between collection time and memory use: higher precision can be
  // obtained with either less concurrent collections but more collection time,
  // or with more concurrent collections (==more memory) and less collection
  // time. Note that concurrent collection does happen for different
  // entrypoints, regardless.
  ::__sanitizer::SpinMutex Taken;
};

// This is allocated and zero-initialized by the compiler, the in-place
// initialization serves mostly as self-documentation and for testing.
// The design is influenced by the observation that typically (at least for
// datacenter binaries, which is the motivating target of this profiler) less
// than 10% of functions in a binary even appear in a profile (of any kind).
//
// 1) We could pre-allocate the flat profile storage in the compiler, just like
// the flat instrumented profiling does. But that penalizes the static size of
// the binary for little reason
//
// 2) We could do the above but zero-initialize the buffers (which should place
// them in .bss), and dynamically populate them. This, though, would page-in
// more memory upfront for the binary's runtime
//
// The current design trades off a bit of overhead at the first time a function
// is encountered *for flat profiling* for avoiding size penalties.
struct FunctionData {
#define _PTRDECL(T, N) T *N = nullptr;
#define _VOLATILE_PTRDECL(T, N) T *volatile N = nullptr;
#define _MUTEXDECL(N) ::__sanitizer::SpinMutex N;
#define _CONTEXT_PTR ContextRoot *CtxRoot = nullptr;
  CTXPROF_FUNCTION_DATA(_PTRDECL, _CONTEXT_PTR, _VOLATILE_PTRDECL, _MUTEXDECL)
#undef _CONTEXT_PTR
#undef _PTRDECL
#undef _VOLATILE_PTRDECL
#undef _MUTEXDECL

  // Constructor for test only - since this is expected to be
  // initialized by the compiler.
  FunctionData() = default;
  ContextRoot *getOrAllocateContextRoot();

  // If (unlikely) StaticSpinMutex internals change, we need to modify the LLVM
  // instrumentation lowering side because it is responsible for allocating and
  // zero-initializing ContextRoots.
  static_assert(sizeof(Mutex) == 1);
};

/// This API is exposed for testing. See the APIs below about the contract with
/// LLVM.
inline bool isScratch(const void *Ctx) {
  return (reinterpret_cast<uint64_t>(Ctx) & 1);
}

// True if Ctx is either nullptr or not the 0x1 value.
inline bool canBeRoot(const ContextRoot *Ctx) {
  return reinterpret_cast<uintptr_t>(Ctx) != 1U;
}

} // namespace __ctx_profile

extern "C" {

// LLVM fills these in when lowering a llvm.instrprof.callsite intrinsic.
// position 0 is used when the current context isn't scratch, 1 when it is. They
// are volatile because of signal handlers - we mean to specifically control
// when the data is loaded.
//
/// TLS where LLVM stores the pointer of the called value, as part of lowering a
/// llvm.instrprof.callsite
extern __thread void *volatile __llvm_ctx_profile_expected_callee[2];
/// TLS where LLVM stores the pointer inside a caller's subcontexts vector that
/// corresponds to the callsite being lowered.
extern __thread ContextNode **volatile __llvm_ctx_profile_callsite[2];

// __llvm_ctx_profile_current_context_root is exposed for unit testing,
// othwerise it's only used internally by compiler-rt/ctx_profile.
extern __thread __ctx_profile::ContextRoot
    *volatile __llvm_ctx_profile_current_context_root;

/// called by LLVM in the entry BB of a "entry point" function. The returned
/// pointer may be "tainted" - its LSB set to 1 - to indicate it's scratch.
ContextNode *
__llvm_ctx_profile_start_context(__ctx_profile::FunctionData *FData, GUID Guid,
                                 uint32_t Counters, uint32_t Callsites);

/// paired with __llvm_ctx_profile_start_context, and called at the exit of the
/// entry point function.
void __llvm_ctx_profile_release_context(__ctx_profile::FunctionData *FData);

/// called for any other function than entry points, in the entry BB of such
/// function. Same consideration about LSB of returned value as .._start_context
ContextNode *__llvm_ctx_profile_get_context(__ctx_profile::FunctionData *FData,
                                            void *Callee, GUID Guid,
                                            uint32_t NumCounters,
                                            uint32_t NumCallsites);

/// Prepares for collection. Currently this resets counter values but preserves
/// internal context tree structure.
void __llvm_ctx_profile_start_collection(unsigned AutodetectDuration = 0);

/// Completely free allocated memory.
void __llvm_ctx_profile_free();

/// Used to obtain the profile. The Writer is called for each root ContextNode,
/// with the ContextRoot::Taken taken. The Writer is responsible for traversing
/// the structure underneath.
/// The Writer's first parameter plays the role of closure for Writer, and is
/// what the caller of __llvm_ctx_profile_fetch passes as the Data parameter.
/// The second parameter is the root of a context tree.
bool __llvm_ctx_profile_fetch(ProfileWriter &);
}
#endif // CTX_PROFILE_CTXINSTRPROFILING_H_
