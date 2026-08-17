// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_ANDROID_FORWARDER2_DEVICE_CONTROLLER_H_
#define TOOLS_ANDROID_FORWARDER2_DEVICE_CONTROLLER_H_

#include <memory>
#include <string>
#include <unordered_map>

#include "base/memory/ref_counted.h"
#include "base/memory/weak_ptr.h"
#include "tools/android/forwarder2/socket.h"

namespace base {
class SingleThreadTaskRunner;
}  // namespace base

namespace forwarder2 {

class DeviceListener;

// There is a single DeviceController per device_forwarder process, and it is in
// charge of managing all active redirections on the device side (one
// DeviceListener each).
class DeviceController {
 public:
  static std::unique_ptr<DeviceController> Create(
      const std::string& adb_unix_socket,
      int exit_notifier_fd);

  DeviceController(const DeviceController&) = delete;
  DeviceController& operator=(const DeviceController&) = delete;

  ~DeviceController();

  void Start();

 private:
  DeviceController(std::unique_ptr<Socket> host_socket, int exit_notifier_fd);

  void AcceptHostCommandSoon();
  void AcceptHostCommandInternal();

  // Note that this can end up being called after the DeviceController is
  // destroyed which is why a weak pointer is used.
  static void DeleteListenerOnError(
      const base::WeakPtr<DeviceController>& device_controller_ptr,
      std::unique_ptr<DeviceListener> device_listener);

  const std::unique_ptr<Socket> host_socket_;
  // Used to notify the controller to exit.
  const int exit_notifier_fd_;
  // Lets ensure DeviceListener instances are deleted on the thread they were
  // created on.
  const scoped_refptr<base::SingleThreadTaskRunner> construction_task_runner_;
  std::unordered_map<int /* port */, std::unique_ptr<DeviceListener>>
      listeners_;

  //WeakPtrFactory's documentation says:
  // Member variables should appear before the WeakPtrFactory, to ensure
  // that any WeakPtrs to Controller are invalidated before its members
  // variable's destructors are executed, rendering them invalid.
  base::WeakPtrFactory<DeviceController> weak_ptr_factory_{this};
};

}  // namespace forwarder

#endif  // TOOLS_ANDROID_FORWARDER2_DEVICE_CONTROLLER_H_
