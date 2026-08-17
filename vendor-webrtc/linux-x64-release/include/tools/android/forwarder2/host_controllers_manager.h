// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_ANDROID_FORWARDER2_HOST_CONTROLLERS_MANAGER_H_
#define TOOLS_ANDROID_FORWARDER2_HOST_CONTROLLERS_MANAGER_H_

#include <memory>
#include <string>
#include <unordered_map>

#include "base/at_exit.h"
#include "base/gtest_prod_util.h"
#include "base/memory/weak_ptr.h"
#include "tools/android/forwarder2/host_controller.h"
#include "tools/android/forwarder2/socket.h"

namespace forwarder2 {

enum : int {
  MAP = 0,
  UNMAP = 1,
  UNMAP_ALL = 2,
};

// Manages HostController instances. There is one HostController instance for
// each connection being forwarded. Note that forwarding can happen with many
// devices (identified with a serial id).
class HostControllersManager {
 public:
  explicit HostControllersManager(
      base::RepeatingCallback<int()> exit_notifier_fd_callback);
  ~HostControllersManager();
  void HandleRequest(const std::string& adb_path,
                     const std::string& device_serial,
                     int command,
                     int device_port,
                     int host_port,
                     std::unique_ptr<Socket> client_socket);
  bool has_failed() const { return has_failed_; }

 private:
  FRIEND_TEST_ALL_PREFIXES(HostControllersManagerTest, AdbNoExtraFds);
  FRIEND_TEST_ALL_PREFIXES(HostControllersManagerTest, AdbArgumentSequence);

  using HostControllerMap =
      std::unordered_map<std::string, std::unique_ptr<HostController>>;

  static std::string MakeHostControllerMapKey(int adb_port, int device_port);

  void InitOnce();

  // Invoked when a HostController instance reports an error (e.g. due to a
  // device connectivity issue). Note that this could be called after the
  // controller manager was destroyed which is why a weak pointer is used.
  static void DeleteHostController(
      const base::WeakPtr<HostControllersManager>& manager_ptr,
      std::unique_ptr<HostController> host_controller);

  void Map(const std::string& adb_path,
           const std::string& device_serial,
           int adb_port,
           int device_port,
           int host_port,
           Socket* client_socket);

  void Unmap(const std::string& adb_path,
             const std::string& device_serial,
             int adb_port,
             int device_port,
             Socket* client_socket);

  void UnmapAll(const std::string& adb_path,
                const std::string& device_serial,
                int adb_port,
                Socket* client_socket);

  bool Adb(const std::string& adb_path,
           const std::string& device_serial,
           const std::string& command,
           std::string* output_and_error);

  void HandleRequestOnInternalThread(const std::string& adb_path,
                                     const std::string& device_serial,
                                     int command,
                                     int device_port,
                                     int host_port,
                                     std::unique_ptr<Socket> client_socket);

  void LogExistingControllers(Socket* client_socket);

  void RemoveAdbPortForDeviceIfNeeded(const std::string& adb_path,
                                      const std::string& device_serial);

  int GetAdbPortForDevice(const std::string adb_path,
                          const std::string& device_serial);

  bool SendMessage(const std::string& msg, Socket* client_socket);

  // This is a separate virtual method solely for easy mocking. The default
  // implementation is a wrapper around base::GetAppOutputAndError.
  virtual bool GetAppOutputAndError(const std::vector<std::string>& argv,
                                    std::string* output);

  std::unordered_map<std::string, int> device_serial_to_adb_port_map_;
  std::unique_ptr<HostControllerMap> controllers_;
  std::unique_ptr<base::AtExitManager>
      at_exit_manager_;  // Needed by base::Thread.
  std::unique_ptr<base::Thread> thread_;
  base::RepeatingCallback<int()> exit_notifier_fd_callback_;
  bool has_failed_;
  base::WeakPtrFactory<HostControllersManager> weak_ptr_factory_;
};

}  // namespace forwarder2

#endif  // TOOLS_ANDROID_FORWARDER2_HOST_CONTROLLERS_MANAGER_H_
