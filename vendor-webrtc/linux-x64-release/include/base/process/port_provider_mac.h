// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROCESS_PORT_PROVIDER_MAC_H_
#define BASE_PROCESS_PORT_PROVIDER_MAC_H_

#include <mach/mach.h>

#include "base/base_export.h"
#include "base/memory/scoped_refptr.h"
#include "base/observer_list_threadsafe.h"
#include "base/process/process_handle.h"

namespace base {

// Abstract base class that provides a mapping from ProcessHandle (pid_t) to the
// Mach task port. This replicates task_for_pid(), which requires root
// privileges.
class BASE_EXPORT PortProvider {
 public:
  PortProvider();

  PortProvider(const PortProvider&) = delete;
  PortProvider& operator=(const PortProvider&) = delete;

  virtual ~PortProvider();

  class Observer {
   public:
    virtual ~Observer() = default;
    // Called by the PortProvider to notify observers that the task port was
    // received for a given process.
    // This notification is guaranteed to be sent on the same task runner where
    // the observer was added.
    virtual void OnReceivedTaskPort(ProcessHandle process_handle) = 0;
  };

  // Returns the mach task port for `process_handle` if possible, or else
  // `MACH_PORT_NULL`.
  virtual mach_port_t TaskForHandle(ProcessHandle process_handle) const = 0;

  // Observer interface.
  void AddObserver(Observer* observer);
  void RemoveObserver(Observer* observer);

 protected:
  // Called by subclasses to send a notification to observers.
  void NotifyObservers(ProcessHandle process_handle);

 private:
  scoped_refptr<base::ObserverListThreadSafe<Observer>> observer_list_;
};

// Port provider that returns the calling process's task port, ignoring its
// argument.
class BASE_EXPORT SelfPortProvider : public base::PortProvider {
  mach_port_t TaskForHandle(base::ProcessHandle process_handle) const override;
};

}  // namespace base

#endif  // BASE_PROCESS_PORT_PROVIDER_MAC_H_
