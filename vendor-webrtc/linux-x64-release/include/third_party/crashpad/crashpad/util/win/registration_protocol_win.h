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

#ifndef CRASHPAD_UTIL_WIN_REGISTRATION_PROTOCOL_WIN_H_
#define CRASHPAD_UTIL_WIN_REGISTRATION_PROTOCOL_WIN_H_

#include <windows.h>
#include <stdint.h>

#include <string>

#include "util/win/address_types.h"
#include "util/win/registration_protocol_win_structs.h"

namespace crashpad {

//! \brief Connect over the given \a pipe_name, passing \a message to the
//!     server, storing the server's reply into \a response.
//!
//! Typically clients will not use this directly, instead using
//! CrashpadClient::SetHandler().
//!
//! \sa CrashpadClient::SetHandler()
bool SendToCrashHandlerServer(const std::wstring& pipe_name,
                              const ClientToServerMessage& message,
                              ServerToClientMessage* response);

//! \brief Wraps CreateNamedPipe() to create a single named pipe instance.
//!
//! \param[in] pipe_name The name to use for the pipe.
//! \param[in] first_instance If `true`, the named pipe instance will be
//!     created with `FILE_FLAG_FIRST_PIPE_INSTANCE`. This ensures that the the
//!     pipe name is not already in use when created. The first instance will be
//!     created with an untrusted integrity SACL so instances of this pipe can
//!     be connected to by processes of any integrity level.
HANDLE CreateNamedPipeInstance(const std::wstring& pipe_name,
                               bool first_instance);

//! \brief Returns the `SECURITY_DESCRIPTOR` blob that will be used for creating
//!     the connection pipe in CreateNamedPipeInstance().
//!
//! This function is only exposed for testing.
//!
//! \param[out] size The size of the returned blob. May be `nullptr` if not
//!     required.
//!
//! \return A pointer to a self-relative `SECURITY_DESCRIPTOR`. Ownership is not
//!     transferred to the caller.
const void* GetSecurityDescriptorForNamedPipeInstance(size_t* size);

//! \brief Returns the `SECURITY_DESCRIPTOR` blob that will be used for creating
//!     the connection pipe in CreateNamedPipeInstance() if the full descriptor
//!     can't be created.
//!
//! This function is only exposed for testing.
//!
//! \param[out] size The size of the returned blob. May be `nullptr` if not
//!     required.
//!
//! \return A pointer to a self-relative `SECURITY_DESCRIPTOR`. Ownership is not
//!     transferred to the caller.
const void* GetFallbackSecurityDescriptorForNamedPipeInstance(size_t* size);

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_WIN_REGISTRATION_PROTOCOL_WIN_H_
