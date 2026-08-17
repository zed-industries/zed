// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_ANDROID_FORWARDER2_FORWARDER_H_
#define TOOLS_ANDROID_FORWARDER2_FORWARDER_H_

#include <sys/select.h>

#include <memory>

#include "base/threading/thread_checker.h"

namespace forwarder2 {

class Socket;

// Internal class that forwards traffic between |socket1| and |socket2|. Note
// that this class is not thread-safe.
class Forwarder {
 public:
  Forwarder(std::unique_ptr<Socket> socket1, std::unique_ptr<Socket> socket2);

  ~Forwarder();

  void RegisterFDs(fd_set* read_fds, fd_set* write_fds, int* max_fd);

  void ProcessEvents(const fd_set& read_fds, const fd_set& write_fds);

  bool IsClosed() const;

  void Shutdown();

 private:
  class BufferedCopier;

  base::ThreadChecker thread_checker_;
  const std::unique_ptr<Socket> socket1_;
  const std::unique_ptr<Socket> socket2_;
  // Copies data from socket1 to socket2.
  const std::unique_ptr<BufferedCopier> buffer1_;
  // Copies data from socket2 to socket1.
  const std::unique_ptr<BufferedCopier> buffer2_;
};

}  // namespace forwarder2

#endif  // TOOLS_ANDROID_FORWARDER2_FORWARDER_H_
