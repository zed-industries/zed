// Copyright 2015 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_WIN_REGISTRATION_PROTOCOL_WIN_STRUCTS_H_
#define CRASHPAD_UTIL_WIN_REGISTRATION_PROTOCOL_WIN_STRUCTS_H_

#include <windows.h>
#include <stdint.h>

#include "util/win/address_types.h"

namespace crashpad {

#pragma pack(push, 1)

//! \brief Structure read out of the client process by the crash handler when an
//!     exception occurs.
struct ExceptionInformation {
  //! \brief The address of an EXCEPTION_POINTERS structure in the client
  //!     process that describes the exception.
  WinVMAddress exception_pointers;

  //! \brief The thread on which the exception happened.
  DWORD thread_id;
};

//! \brief Context to be passed to WerRegisterRuntimeExceptionModule().
//!
//! Used by the crashpad client, and the WER exception DLL.
struct WerRegistration {
  //! \brief The expected value of `version`. This should be changed whenever
  //!     this struct is modified incompatibly.
  enum { kWerRegistrationVersion = 1 };
  //! \brief Version field to detect skew between target process and helper.
  //!     Should be set to kWerRegistrationVersion.
  int version;
  //! \brief Used by DumpWithoutCrashing and the WER module to initiate a dump.
  //!     These handles are leaked in the client process.
  HANDLE dump_without_crashing;
  //! \brief Used by DumpWithoutCrashing to signal that a dump has been taken.
  //!     These handles are leaked in the client process.
  HANDLE dump_completed;
  //! \brief Set just before and cleared just after the events above are
  //!     triggered or signalled in a normal DumpWithoutCrashing call.
  //! When `true` the WER handler should not set the exception structures until
  //! after dump_completed has been signalled.
  bool in_dump_without_crashing;
  //! \brief Address of g_non_crash_exception_information.
  //!
  //! Provided by the target process. Just before dumping we will point
  //! (*crashpad_exception_info).exception_pointers at `pointers`. As WerFault
  //! loads the helper with the same bitness as the client this can be void*.
  void* crashpad_exception_info;
  //! \brief These will point into the `exception` and `context` members in this
  //!     structure.
  //!
  //! Filled in by the helper DLL.
  EXCEPTION_POINTERS pointers;
  //! \brief The exception provided by WerFault.
  //!
  //! Filled in by the helper DLL.
  EXCEPTION_RECORD exception;
  //! \brief The context provided by WerFault.
  //!
  //! Filled in by the helper DLL.
  CONTEXT context;
};

//! \brief A client registration request.
struct RegistrationRequest {
  //! \brief The expected value of `version`. This should be changed whenever
  //!     the messages or ExceptionInformation are modified incompatibly.
  enum { kMessageVersion = 1 };

  //! \brief Version field to detect skew between client and server. Should be
  //!     set to kMessageVersion.
  int version;

  //! \brief The PID of the client process.
  DWORD client_process_id;

  //! \brief The address, in the client process's address space, of an
  //!     ExceptionInformation structure, used when handling a crash dump
  //!     request.
  WinVMAddress crash_exception_information;

  //! \brief The address, in the client process's address space, of an
  //!     ExceptionInformation structure, used when handling a non-crashing dump
  //!     request.
  WinVMAddress non_crash_exception_information;

  //! \brief The address, in the client process's address space, of a
  //!     `CRITICAL_SECTION` allocated with a valid .DebugInfo field. This can
  //!     be accomplished by using
  //!     InitializeCriticalSectionWithDebugInfoIfPossible() or equivalent. This
  //!     value can be `0`, however then limited lock data will be available in
  //!     minidumps.
  WinVMAddress critical_section_address;
};

//! \brief A message only sent to the server by itself to trigger shutdown.
struct ShutdownRequest {
  //! \brief A randomly generated token used to validate the the shutdown
  //!     request was not sent from another process.
  uint64_t token;
};

//! \brief The message passed from client to server by
//!     SendToCrashHandlerServer().
struct ClientToServerMessage {
  //! \brief Indicates which field of the union is in use.
  enum Type : uint32_t {
    //! \brief For RegistrationRequest.
    kRegister,

    //! \brief For ShutdownRequest.
    kShutdown,

    //! \brief An empty message sent by the initial client in asynchronous mode.
    //!     No data is required, this just confirms that the server is ready to
    //!     accept client registrations.
    kPing,
  } type;

  union {
    RegistrationRequest registration;
    ShutdownRequest shutdown;
  };
};

//! \brief A client registration response.
struct RegistrationResponse {
  //! \brief An event `HANDLE`, valid in the client process, that should be
  //!     signaled to request a crash report. Clients should convert the value
  //!     to a `HANDLE` by calling IntToHandle().
  int request_crash_dump_event;

  //! \brief An event `HANDLE`, valid in the client process, that should be
  //!     signaled to request a non-crashing dump be taken. Clients should
  //!     convert the value to a `HANDLE` by calling IntToHandle().
  int request_non_crash_dump_event;

  //! \brief An event `HANDLE`, valid in the client process, that will be
  //!     signaled by the server when the non-crashing dump is complete. Clients
  //!     should convert the value to a `HANDLE` by calling IntToHandle().
  int non_crash_dump_completed_event;
};

//! \brief The response sent back to the client via SendToCrashHandlerServer().
union ServerToClientMessage {
  RegistrationResponse registration;
};

#pragma pack(pop)

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_WIN_REGISTRATION_PROTOCOL_WIN_STRUCTS_H_
