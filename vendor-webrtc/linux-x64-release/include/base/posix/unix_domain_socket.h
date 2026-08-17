// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POSIX_UNIX_DOMAIN_SOCKET_H_
#define BASE_POSIX_UNIX_DOMAIN_SOCKET_H_

#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#include <vector>

#include "base/base_export.h"
#include "base/files/scoped_file.h"
#include "base/process/process_handle.h"
#include "build/build_config.h"

namespace base {

class Pickle;

// Creates a connected pair of UNIX-domain SOCK_SEQPACKET sockets, and passes
// ownership of the newly allocated file descriptors to |one| and |two|.
// Returns true on success.
bool BASE_EXPORT CreateSocketPair(ScopedFD* one, ScopedFD* two);

class BASE_EXPORT UnixDomainSocket {
 public:
  // Maximum number of file descriptors that can be read by RecvMsg().
  static const size_t kMaxFileDescriptors;

  // Use to enable receiving process IDs in RecvMsgWithPid.  Should be called on
  // the receiving socket (i.e., the socket passed to RecvMsgWithPid). Returns
  // true if successful.
  static bool EnableReceiveProcessId(int fd);

  // Use sendmsg to write the given msg and include a vector of file
  // descriptors. Returns true if successful.
  static bool SendMsg(int fd,
                      const void* msg,
                      size_t length,
                      const std::vector<int>& fds);

  // Use recvmsg to read a message and an array of file descriptors. Returns
  // -1 on failure. Note: will read, at most, |kMaxFileDescriptors| descriptors.
  static ssize_t RecvMsg(int fd,
                         void* msg,
                         size_t length,
                         std::vector<ScopedFD>* fds);

  // Same as RecvMsg above, but also returns the sender's process ID (as seen
  // from the caller's namespace).  However, before using this function to
  // receive process IDs, EnableReceiveProcessId() should be called on the
  // receiving socket.
  static ssize_t RecvMsgWithPid(int fd,
                                void* msg,
                                size_t length,
                                std::vector<ScopedFD>* fds,
                                ProcessId* pid);

  // Perform a sendmsg/recvmsg pair.
  //   1. This process creates a UNIX SEQPACKET socketpair. Using
  //      connection-oriented sockets (SEQPACKET or STREAM) is critical here,
  //      because if one of the ends closes the other one must be notified.
  //   2. This process writes a request to |fd| with an SCM_RIGHTS control
  //      message containing on end of the fresh socket pair.
  //   3. This process blocks reading from the other end of the fresh
  //      socketpair.
  //   4. The target process receives the request, processes it and writes the
  //      reply to the end of the socketpair contained in the request.
  //   5. This process wakes up and continues.
  //
  //   fd: descriptor to send the request on
  //   reply: buffer for the reply
  //   reply_len: size of |reply|
  //   result_fd: (may be NULL) the file descriptor returned in the reply
  //              (if any)
  //   request: the bytes to send in the request
  static ssize_t SendRecvMsg(int fd,
                             uint8_t* reply,
                             unsigned reply_len,
                             int* result_fd,
                             const Pickle& request);

  // Similar to SendRecvMsg(), but |recvmsg_flags| allows to control the flags
  // of the recvmsg(2) call.
  static ssize_t SendRecvMsgWithFlags(int fd,
                                      uint8_t* reply,
                                      unsigned reply_len,
                                      int recvmsg_flags,
                                      int* result_fd,
                                      const Pickle& request);

 private:
  // Similar to RecvMsg, but allows to specify |flags| for recvmsg(2).
  static ssize_t RecvMsgWithFlags(int fd,
                                  void* msg,
                                  size_t length,
                                  int flags,
                                  std::vector<ScopedFD>* fds,
                                  ProcessId* pid);
};

}  // namespace base

#endif  // BASE_POSIX_UNIX_DOMAIN_SOCKET_H_
