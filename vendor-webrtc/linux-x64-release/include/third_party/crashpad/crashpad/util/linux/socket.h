// Copyright 2019 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_UTIL_LINUX_SOCKET_H_
#define CRASHPAD_UTIL_LINUX_SOCKET_H_

#include <sys/socket.h>
#include <sys/types.h>

#include <vector>

#include "util/file/file_io.h"

namespace crashpad {

//! \brief Utilities for communicating over `SO_PASSCRED` enabled `AF_UNIX`
//!     sockets.
class UnixCredentialSocket {
 public:
  UnixCredentialSocket() = delete;
  UnixCredentialSocket(const UnixCredentialSocket&) = delete;
  UnixCredentialSocket& operator=(const UnixCredentialSocket&) = delete;

  //! \brief Creates an `AF_UNIX` family socket pair with `SO_PASSCRED` set on
  //!     each socket.
  //!
  //! \param[out] s1 One end of the connected pair.
  //! \param[out] s2 The other end of the connected pair.
  //! \return `true` on success. Otherwise, `false` with a message logged.
  static bool CreateCredentialSocketpair(ScopedFileHandle* s1,
                                         ScopedFileHandle* s2);

  //! \brief The maximum number of file descriptors that may be sent/received
  //!     with `SendMsg()` or `RecvMsg()`.
  static constexpr size_t kMaxSendRecvMsgFDs = 4;

  //! \brief Wraps `sendmsg()` to send a message with file descriptors.
  //!
  //! This function is intended for use with `AF_UNIX` family sockets and
  //! passes file descriptors with `SCM_RIGHTS`.
  //!
  //! This function may be used in a compromised context.
  //!
  //! \param[in] fd The file descriptor to write the message to.
  //! \param[in] buf The buffer containing the message.
  //! \param[in] buf_size The size of the message.
  //! \param[in] fds An array of at most `kMaxSendRecvMsgFDs` file descriptors.
  //!     Optional.
  //! \param[in] fd_count The number of file descriptors in \a fds. Required
  //!     only if \a fds was set.
  //! \return 0 on success or an error code on failure.
  static int SendMsg(int fd,
                     const void* buf,
                     size_t buf_size,
                     const int* fds = nullptr,
                     size_t fd_count = 0);

  //! \brief Wraps `recvmsg()` to receive a message with file descriptors and
  //!     credentials.
  //!
  //! This function is intended to be used with `AF_UNIX` family sockets. Up to
  //! `kMaxSendRecvMsgFDs` file descriptors may be received (via `SCM_RIGHTS`).
  //! The socket must have `SO_PASSCRED` set.
  //!
  //! \param[in] fd The file descriptor to receive the message on.
  //! \param[out] buf The buffer to fill with the message.
  //! \param[in] buf_size The size of the message.
  //! \param[out] creds The credentials of the sender.
  //! \param[out] fds The recieved file descriptors. Optional. If `nullptr`, all
  //!     received file descriptors will be closed.
  //! \return `true` on success. Otherwise, `false`, with a message logged. No
  //!     message will be logged if the message was detected to be an EOF
  //!     condition triggered by all clients disconnecting. This case is
  //!     indistinguishable from misuses of this interface that haven't set
  //!     `SO_PASSCRED` on \a fd.
  static bool RecvMsg(int fd,
                      void* buf,
                      size_t buf_size,
                      ucred* creds,
                      std::vector<ScopedFileHandle>* fds = nullptr);
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_LINUX_SOCKET_H_
