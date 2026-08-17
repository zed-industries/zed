// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_TEST_TEST_CHILD_LAUNCHER_H_
#define IPCZ_SRC_TEST_TEST_CHILD_LAUNCHER_H_

#include <sys/types.h>

#include "reference_drivers/file_descriptor.h"

#include <string_view>

namespace ipcz::test {

// This class helps multinode tests launch child processes to run test nodes in
// isolation. This is only supported on Linux.
class TestChildLauncher {
 public:
  TestChildLauncher();
  ~TestChildLauncher();

  // Must be called early in process startup with inputs from main().
  static void Initialize(int argc, char** argv);

  // Called by standalone ipcz_tests main() to run a TestNode body within a
  // child process. Must be called after Initialize(). Returns true if and only
  // if the calling process was launched as a test child process and has
  // appropriate command line arguments to initialize a TestNode.
  static bool RunTestChild(int& exit_code);

  // Extracts a FileDescriptor passed on the command line to this (child)
  // process.
  static reference_drivers::FileDescriptor TakeChildSocketDescriptor();

  // Waits for a child process to terminate, and returns true if and only if
  // the process terminated normally with an exit code of 0.
  static bool WaitForSuccessfulProcessTermination(pid_t pid);

  // Launches a new child process to run the TestNode identified by `node_name`,
  // using `socket` as the basis for a SocketTransport which will connect to the
  // test's main broker node. Returns the PID of the new child process. This
  // call either succeeds or crashes, so the return PID will always be valid.
  pid_t Launch(std::string_view node_name,
               std::string_view feature_set,
               reference_drivers::FileDescriptor socket);
};

}  // namespace ipcz::test

#endif  // IPCZ_SRC_TEST_TEST_CHILD_LAUNCHER_H_
