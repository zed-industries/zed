// Copyright 2017 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_LINUX_EXCEPTION_HANDLER_PROTOCOL_H_
#define CRASHPAD_UTIL_LINUX_EXCEPTION_HANDLER_PROTOCOL_H_

#include <errno.h>
#include <signal.h>
#include <stdint.h>
#include <sys/types.h>

#include "build/build_config.h"
#include "util/file/file_io.h"
#include "util/misc/address_types.h"

namespace crashpad {

class ExceptionHandlerProtocol {
 public:
#pragma pack(push, 1)

  //! \brief The type used for error reporting.
  using Errno = int32_t;
  static_assert(sizeof(Errno) >= sizeof(errno), "Errno type is too small");

  //! \brief A boolean status suitable for communication between processes.
  enum Bool : char { kBoolFalse, kBoolTrue };

  //! \brief Information about a client registered with an
  //!     ExceptionHandlerServer.
  struct ClientInformation {
    //! \brief Constructs this object.
    ClientInformation();

    //! \brief The address in the client's address space of an
    //!     ExceptionInformation struct.
    VMAddress exception_information_address;

    //! \brief The address in the client's address space of a
    //!     SanitizationInformation struct, or 0 if there is no such struct.
    VMAddress sanitization_information_address;

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
    //! \brief Indicates that the client is likely in a crash loop if a crash
    //!     occurs before this timestamp. This value is only used by ChromeOS's
    //!     `/sbin/crash_reporter`.
    uint64_t crash_loop_before_time;
#endif
  };

  //! \brief The signal used to indicate a crash dump is complete.
  //!
  //! When multiple clients share a single socket connection with the handler,
  //! the handler sends this signal to the dump requestor to indicate when the
  //! the dump is either done or has failed and the client may continue.
  static constexpr int kDumpDoneSignal = SIGCONT;

  //! \brief The message passed from client to server.
  struct ClientToServerMessage {
    static constexpr int32_t kVersion = 1;

    //! \brief Constructs this object.
    ClientToServerMessage();

    //! \brief Indicates what message version is being used.
    int32_t version;

    enum Type : uint32_t {
      //! \brief Request that the server respond with its credentials.
      kTypeCheckCredentials,

      //! \brief Used to request a crash dump for the sending client.
      kTypeCrashDumpRequest
    };

    Type type;

    //! \brief A stack address of the thread sending the message.
    VMAddress requesting_thread_stack_address;

    union {
      //! \brief Valid for type == kCrashDumpRequest
      ClientInformation client_info;
    };
  };

  //! \brief The message passed from server to client.
  struct ServerToClientMessage {
    enum Type : uint32_t {
      //! \brief Used to pass credentials with `SCM_CREDENTIALS`.
      kTypeCredentials,

      //! \brief Indicates that the client should fork a PtraceBroker process.
      kTypeForkBroker,

      //! \brief Inidicates that the client should set allow the handler to
      //!     trace it using PR_SET_PTRACER.
      kTypeSetPtracer,

      //! \brief Indicates that the handler has completed a requested crash
      //!     dump.
      kTypeCrashDumpComplete,

      //! \brief Indicicates that the handler was unable to produce a crash
      //!     dump.
      kTypeCrashDumpFailed
    };

    Type type;

    //! \brief The handler's process ID. Valid for kTypeSetPtracer.
    pid_t pid;
  };

  ExceptionHandlerProtocol() = delete;
  ExceptionHandlerProtocol(const ExceptionHandlerProtocol&) = delete;
  ExceptionHandlerProtocol& operator=(const ExceptionHandlerProtocol&) = delete;

#pragma pack(pop)
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_LINUX_EXCEPTION_HANDLER_PROTOCOL_H_
