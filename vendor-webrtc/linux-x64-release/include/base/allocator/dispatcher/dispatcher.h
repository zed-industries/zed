// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_DISPATCHER_DISPATCHER_H_
#define BASE_ALLOCATOR_DISPATCHER_DISPATCHER_H_

#include <memory>

#include "base/allocator/dispatcher/internal/dispatcher_internal.h"
#include "base/base_export.h"

namespace base::allocator::dispatcher {

namespace internal {
struct DispatchData;
}

// Dispatcher serves as the top level instance for managing the dispatch
// mechanism. The class instance manages connections to the various memory
// subsystems such as PartitionAlloc. To keep the public interface as lean as
// possible it uses a pimpl pattern.
class BASE_EXPORT Dispatcher {
 public:
  static Dispatcher& GetInstance();

  Dispatcher();

  // Initialize the dispatch mechanism with the given tuple of observers. The
  // observers must be valid (it is only DCHECKed internally at initialization,
  // but not verified further)
  // If Initialize is called multiple times, the first one wins. All later
  // invocations are silently ignored. Initialization is protected from
  // concurrent invocations. In case of concurrent accesses, the first one to
  // get the lock wins.
  // The dispatcher invokes following functions on the observers:
  // void OnAllocation(void* address,
  //                   size_t size,
  //                   AllocationSubsystem sub_system,
  //                   const char* type_name);
  // void OnFree(void* address);
  //
  // Note: The dispatcher mechanism does NOT bring systematic protection against
  // recursive invocations. That is, observers which allocate memory on the
  // heap, i.e. through dynamically allocated containers or by using the
  // CHECK-macro, are responsible to break these recursions!
  template <typename... ObserverTypes>
  void Initialize(const std::tuple<ObserverTypes...>& observers) {
    // Get the hooks for running these observers and pass them to further
    // initialization
    Initialize(internal::GetNotificationHooks(observers));
  }

  // The following functions provide an interface to setup and tear down the
  // dispatcher when testing. This must NOT be used from production code since
  // the hooks cannot be removed reliably under all circumstances.
  template <typename ObserverType>
  void InitializeForTesting(ObserverType* observer) {
    Initialize(std::make_tuple(observer));
  }

  void ResetForTesting();

 private:
  // structure and pointer to the private implementation.
  struct Impl;
  std::unique_ptr<Impl> const impl_;

  ~Dispatcher();

  void Initialize(const internal::DispatchData& dispatch_data);
};
}  // namespace base::allocator::dispatcher

#endif  // BASE_ALLOCATOR_DISPATCHER_DISPATCHER_H_
