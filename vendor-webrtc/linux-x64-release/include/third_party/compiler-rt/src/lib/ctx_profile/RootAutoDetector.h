/*===- RootAutodetector.h- auto-detect roots for ctxprof  -----------------===*\
|*
|* Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
|* See https://llvm.org/LICENSE.txt for license information.
|* SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
|*
\*===----------------------------------------------------------------------===*/

#ifndef CTX_PROFILE_ROOTAUTODETECTOR_H_
#define CTX_PROFILE_ROOTAUTODETECTOR_H_

#include "sanitizer_common/sanitizer_dense_map.h"
#include "sanitizer_common/sanitizer_internal_defs.h"
#include "sanitizer_common/sanitizer_stacktrace.h"
#include "sanitizer_common/sanitizer_vector.h"
#include <pthread.h>
#include <sanitizer/common_interface_defs.h>

using namespace __asan;
using namespace __sanitizer;

namespace __ctx_profile {

/// Capture all the stack traces observed for a specific thread. The "for a
/// specific thread" part is not enforced, but assumed in determineRoots.
class PerThreadCallsiteTrie {
protected:
  /// A trie. A node is the address of a callsite in a function activation. A
  /// child is a callsite in the activation made from the callsite
  /// corresponding to the parent.
  struct Trie final {
    const uptr CallsiteAddress;
    uint64_t Count = 0;
    DenseMap<uptr, Trie> Children;

    Trie(uptr CallsiteAddress = 0) : CallsiteAddress(CallsiteAddress) {}
  };
  Trie TheTrie;

  /// Return the runtime start address of the function that contains the call at
  /// the runtime address CallsiteAddress. May be overriden for easy testing.
  virtual uptr getFctStartAddr(uptr CallsiteAddress) const;

public:
  PerThreadCallsiteTrie(const PerThreadCallsiteTrie &) = delete;
  PerThreadCallsiteTrie(PerThreadCallsiteTrie &&) = default;
  PerThreadCallsiteTrie() = default;

  virtual ~PerThreadCallsiteTrie() = default;

  void insertStack(const StackTrace &ST);

  /// Return the runtime address of root functions, as determined for this
  /// thread, together with the number of samples that included them.
  DenseMap<uptr, uint64_t> determineRoots() const;
};

class RootAutoDetector final {
  // A prime number. We may want to make this configurable at collection start.
  static const uint64_t SampleRate = 6113;
  const unsigned WaitSeconds;
  pthread_t WorkerThread;

  struct PerThreadSamples {
    PerThreadSamples(RootAutoDetector &Parent);

    PerThreadCallsiteTrie TrieRoot;
    SpinMutex M;
  };
  SpinMutex AllSamplesMutex;
  SANITIZER_GUARDED_BY(AllSamplesMutex)
  Vector<PerThreadSamples *> AllSamples;
  atomic_uintptr_t &FunctionDataListHead;
  atomic_uintptr_t &Self;
  void collectStack();

public:
  RootAutoDetector(atomic_uintptr_t &FunctionDataListHead,
                   atomic_uintptr_t &Self, unsigned WaitSeconds)
      : WaitSeconds(WaitSeconds), FunctionDataListHead(FunctionDataListHead),
        Self(Self) {}

  // Samples the stack at `SampleRate` (rate observed independently on each
  // thread) in thread local `PerThreadCallsiteTrie`s.
  void sample();

  // Start a thread waiting `WaitSeconds`, after which it uses the
  // `PerThreadCallsiteTrie` data observed so far over all threads to determine
  // roots. Marks those roots by traversing the linked list of FunctionData that
  // starts at `FunctionDataListHead`, and assigning their `CtxRoot`. Finally,
  // resets the `Self` atomic, so that other threads don't continue calling
  // `sample`.
  void start();

  // join the waiting thread.
  void join();
};

} // namespace __ctx_profile
#endif
