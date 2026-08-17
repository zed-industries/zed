// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_ANDROID_FORWARDER2_HOST_CONTROLLER_H_
#define TOOLS_ANDROID_FORWARDER2_HOST_CONTROLLER_H_

#include <memory>
#include <string>

#include "base/compiler_specific.h"
#include "base/functional/callback.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread.h"
#include "tools/android/forwarder2/forwarders_manager.h"
#include "tools/android/forwarder2/pipe_notifier.h"
#include "tools/android/forwarder2/self_deleter_helper.h"
#include "tools/android/forwarder2/socket.h"

namespace forwarder2 {

// This class partners with DeviceController and has the same lifetime and
// threading characteristics as DeviceListener. In a nutshell, this class
// operates on its own thread and is destroyed on the thread it was constructed
// on. The class' deletion can happen in two different ways:
// - Its destructor was called by its owner (HostControllersManager).
// - Its internal thread requested self-deletion after an error happened. In
//   this case the owner (HostControllersManager) is notified on the
//   construction thread through the provided ErrorCallback invoked with the
//   HostController instance. When this callback is invoked, it's up to the
//   owner to delete the instance.
class HostController {
 public:
  // Callback used for self-deletion when an error happens so that the client
  // can perform some cleanup work before deleting the HostController instance.
  using ErrorCallback =
      base::OnceCallback<void(std::unique_ptr<HostController>)>;

  // If |device_port| is zero then a dynamic port is allocated (and retrievable
  // through device_port() below).
  static std::unique_ptr<HostController> Create(
      const std::string& device_serial,
      int device_port,
      int host_port,
      int adb_port,
      int exit_notifier_fd,
      ErrorCallback error_callback);

  HostController(const HostController&) = delete;
  HostController& operator=(const HostController&) = delete;

  ~HostController();

  // Starts the internal controller thread.
  void Start();

  int adb_port() const { return adb_port_; }

  int device_port() const { return device_port_; }

 private:
  HostController(const std::string& device_serial,
                 int device_port,
                 int host_port,
                 int adb_port,
                 ErrorCallback error_callback,
                 std::unique_ptr<Socket> adb_control_socket,
                 std::unique_ptr<PipeNotifier> delete_controller_notifier);

  void ReadNextCommandSoon();
  void ReadCommandOnInternalThread();

  bool StartForwarder(std::unique_ptr<Socket> host_server_data_socket);

  // Note that this gets also called when ~HostController() is invoked.
  void OnInternalThreadError();

  void UnmapPortOnDevice();

  SelfDeleterHelper<HostController> self_deleter_helper_;
  const std::string device_serial_;
  const int device_port_;
  const int host_port_;
  const int adb_port_;
  std::unique_ptr<Socket> adb_control_socket_;
  // Used to cancel the pending blocking IO operations when the host controller
  // instance is deleted.
  std::unique_ptr<PipeNotifier> delete_controller_notifier_;
  // Task runner used for deletion set at deletion time (i.e. the object is
  // deleted on the same thread it is created on).
  const scoped_refptr<base::SingleThreadTaskRunner> deletion_task_runner_;
  base::Thread thread_;
  ForwardersManager forwarders_manager_;
};

}  // namespace forwarder2

#endif  // TOOLS_ANDROID_FORWARDER2_HOST_CONTROLLER_H_
