// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_TRAP_EVENT_DISPATCHER_H_
#define IPCZ_SRC_IPCZ_TRAP_EVENT_DISPATCHER_H_

#include <cstdint>

#include "ipcz/ipcz.h"
#include "third_party/abseil-cpp/absl/container/inlined_vector.h"
#include "util/ref_counted.h"

namespace ipcz {

// Accumulates IpczTrapEvent dispatches to specific handlers. Handler invocation
// is deferred until DispatchAll() is called or the TrapEventDispatcher is
// destroyed. This allows event dispatches to be accumulated while e.g. Node and
// Router locks are held, and dispatched later, once such locks are released.
//
// This object is not thread-safe but is generally constructed on the stack and
// passed into whatever might want to accumulate events for dispatch.
class TrapEventDispatcher {
 public:
  TrapEventDispatcher();
  TrapEventDispatcher(const TrapEventDispatcher&) = delete;
  TrapEventDispatcher& operator=(const TrapEventDispatcher&) = delete;
  ~TrapEventDispatcher();

  // Schedules a new event for dispatch by this object as soon as DispatchAll()
  // is explicitly called or the TrapEventDispatcher is destroyed.
  void DeferEvent(IpczTrapEventHandler handler,
                  uintptr_t context,
                  IpczTrapConditionFlags flags,
                  const IpczPortalStatus& status);

  // Dispatches any events deferred by DeferEvent() above.
  void DispatchAll();

 private:
  // Details of an event to be dipsatched.
  struct Event {
    Event();
    Event(IpczTrapEventHandler handler,
          uintptr_t context,
          IpczTrapConditionFlags flags,
          IpczPortalStatus status);
    Event(const Event&);
    Event& operator=(const Event&);
    ~Event();

    IpczTrapEventHandler handler;
    uintptr_t context;
    IpczTrapConditionFlags flags;
    IpczPortalStatus status;
  };

  // Space for four events should avoid heap allocations in the vast majority of
  // cases where we accumulate events for imminent dispatch.
  using DeferredEventQueue = absl::InlinedVector<Event, 4>;
  DeferredEventQueue events_;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_TRAP_EVENT_DISPATCHER_H_
