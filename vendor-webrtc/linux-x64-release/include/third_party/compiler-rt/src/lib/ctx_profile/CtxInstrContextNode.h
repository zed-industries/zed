//===--- CtxInstrContextNode.h - Contextual Profile Node --------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//==============================================================================
//
// NOTE!
// llvm/include/llvm/ProfileData/CtxInstrContextNode.h and
//   compiler-rt/lib/ctx_profile/CtxInstrContextNode.h
// must be exact copies of each other.
//
// compiler-rt creates these objects as part of the instrumentation runtime for
// contextual profiling. LLVM only consumes them to convert a contextual tree
// to a bitstream.
//
//==============================================================================

/// The contextual profile is a directed tree where each node has one parent. A
/// node (ContextNode) corresponds to a function activation. The root of the
/// tree is at a function that was marked as entrypoint to the compiler. A node
/// stores counter values for edges and a vector of subcontexts. These are the
/// contexts of callees. The index in the subcontext vector corresponds to the
/// index of the callsite (as was instrumented via llvm.instrprof.callsite). At
/// that index we find a linked list, potentially empty, of ContextNodes. Direct
/// calls will have 0 or 1 values in the linked list, but indirect callsites may
/// have more.
///
/// The ContextNode has a fixed sized header describing it - the GUID of the
/// function, the size of the counter and callsite vectors. It is also an
/// (intrusive) linked list for the purposes of the indirect call case above.
///
/// Allocation is expected to happen on an Arena. The allocation lays out inline
/// the counter and subcontexts vectors. The class offers APIs to correctly
/// reference the latter.
///
/// The layout is as follows:
///
/// [[declared fields][counters vector][vector of ptrs to subcontexts]]
///
/// See also documentation on the counters and subContexts members below.
///
/// The structure of the ContextNode is known to LLVM, because LLVM needs to:
///   (1) increment counts, and
///   (2) form a GEP for the position in the subcontext list of a callsite
/// This means changes to LLVM contextual profile lowering and changes here
/// must be coupled.
/// Note: the header content isn't interesting to LLVM (other than its size)
///
/// Part of contextual collection is the notion of "scratch contexts". These are
/// buffers that are "large enough" to allow for memory-safe acceses during
/// counter increments - meaning the counter increment code in LLVM doesn't need
/// to be concerned with memory safety. Their subcontexts never get populated,
/// though. The runtime code here produces and recognizes them.

#ifndef LLVM_PROFILEDATA_CTXINSTRCONTEXTNODE_H
#define LLVM_PROFILEDATA_CTXINSTRCONTEXTNODE_H

#include <stdint.h>
#include <stdlib.h>

namespace llvm {
namespace ctx_profile {
using GUID = uint64_t;

class ContextNode final {
  const GUID Guid;
  ContextNode *const Next;
  const uint32_t NumCounters;
  const uint32_t NumCallsites;

public:
  ContextNode(GUID Guid, uint32_t NumCounters, uint32_t NumCallsites,
              ContextNode *Next = nullptr)
      : Guid(Guid), Next(Next), NumCounters(NumCounters),
        NumCallsites(NumCallsites) {}

  static inline size_t getAllocSize(uint32_t NumCounters,
                                    uint32_t NumCallsites) {
    return sizeof(ContextNode) + sizeof(uint64_t) * NumCounters +
           sizeof(ContextNode *) * NumCallsites;
  }

  // The counters vector starts right after the static header.
  uint64_t *counters() {
    ContextNode *addr_after = &(this[1]);
    return reinterpret_cast<uint64_t *>(addr_after);
  }

  uint32_t counters_size() const { return NumCounters; }
  uint32_t callsites_size() const { return NumCallsites; }

  const uint64_t *counters() const {
    return const_cast<ContextNode *>(this)->counters();
  }

  // The subcontexts vector starts right after the end of the counters vector.
  ContextNode **subContexts() {
    return reinterpret_cast<ContextNode **>(&(counters()[NumCounters]));
  }

  ContextNode *const *subContexts() const {
    return const_cast<ContextNode *>(this)->subContexts();
  }

  GUID guid() const { return Guid; }
  ContextNode *next() const { return Next; }

  size_t size() const { return getAllocSize(NumCounters, NumCallsites); }

  uint64_t entrycount() const { return counters()[0]; }
};

/// The internal structure of FunctionData. This makes sure that changes to
/// the fields of FunctionData either get automatically captured on the llvm
/// side, or force a manual corresponding update.
///
/// The macro arguments (see CtxInstrProfiling.h for example):
///
/// PTRDECL is a macro taking 2 parameters: a type and the name of the field.
/// The field is a pointer of that type;
///
/// VOLATILE_PTRDECL is the same as above, but for volatile pointers;
///
/// MUTEXDECL takes one parameter, the name of a field that is a mutex.
#define CTXPROF_FUNCTION_DATA(PTRDECL, CONTEXT_PTR, VOLATILE_PTRDECL,          \
                              MUTEXDECL)                                       \
  PTRDECL(FunctionData, Next)                                                  \
  VOLATILE_PTRDECL(void, EntryAddress)                                         \
  CONTEXT_PTR                                                                  \
  VOLATILE_PTRDECL(ContextNode, FlatCtx)                                       \
  MUTEXDECL(Mutex)

/// Abstraction for the parameter passed to `__llvm_ctx_profile_fetch`.
/// `startContextSection` is called before any context roots are sent for
/// writing. Then one or more `writeContextual` calls are made; finally,
/// `endContextSection` is called.
class ProfileWriter {
public:
  virtual void startContextSection() = 0;
  virtual void writeContextual(const ctx_profile::ContextNode &RootNode,
                               const ctx_profile::ContextNode *Unhandled,
                               uint64_t TotalRootEntryCount) = 0;
  virtual void endContextSection() = 0;

  virtual void startFlatSection() = 0;
  virtual void writeFlat(ctx_profile::GUID Guid, const uint64_t *Buffer,
                         size_t BufferSize) = 0;
  virtual void endFlatSection() = 0;

  virtual ~ProfileWriter() = default;
};
} // namespace ctx_profile
} // namespace llvm
#endif
