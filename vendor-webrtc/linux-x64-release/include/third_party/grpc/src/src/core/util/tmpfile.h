//
//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_TMPFILE_H
#define GRPC_SRC_CORE_UTIL_TMPFILE_H

#include <grpc/support/port_platform.h>
#include <stdio.h>

// Creates a temporary file from a prefix.
// If tmp_filename is not NULL, *tmp_filename is assigned the name of the
// created file and it is the responsibility of the caller to gpr_free it
// unless an error occurs in which case it will be set to NULL.
FILE* gpr_tmpfile(const char* prefix, char** tmp_filename);

#endif  // GRPC_SRC_CORE_UTIL_TMPFILE_H
