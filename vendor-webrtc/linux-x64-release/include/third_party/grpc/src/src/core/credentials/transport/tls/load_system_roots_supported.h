//
//
// Copyright 2018 gRPC authors.
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
//
//

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_LOAD_SYSTEM_ROOTS_SUPPORTED_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_LOAD_SYSTEM_ROOTS_SUPPORTED_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>

#if defined(GPR_LINUX) || defined(GPR_FREEBSD) || defined(GPR_APPLE)

namespace grpc_core {

// Creates a bundle slice containing the contents of all certificate files in
// a directory.
// Returns such slice.
// Exposed for testing purposes only.
grpc_slice CreateRootCertsBundle(const char* certs_directory);

// Gets the absolute file path needed to load a certificate file.
// Populates path_buffer, which must be of size MAXPATHLEN.
// Exposed for testing purposes only.
void GetAbsoluteFilePath(const char* valid_file_dir,
                         const char* file_entry_name, char* path_buffer);

}  // namespace grpc_core

#endif  // GPR_LINUX || GPR_FREEBSD || GPR_APPLE
#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_LOAD_SYSTEM_ROOTS_SUPPORTED_H
