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

#ifndef GRPC_SRC_CORE_UTIL_SUBPROCESS_H
#define GRPC_SRC_CORE_UTIL_SUBPROCESS_H

#include <grpc/support/port_platform.h>

#include <string>

typedef struct gpr_subprocess gpr_subprocess;

/// .exe on windows, empty on unices
const char* gpr_subprocess_binary_extension();

gpr_subprocess* gpr_subprocess_create(int argc, const char** argv);

gpr_subprocess* gpr_subprocess_create_with_envp(int argc, const char** argv,
                                                int envc, const char** envp);

// communicate to the subprocess via stdin, stdout and stderr
bool gpr_subprocess_communicate(gpr_subprocess* p, std::string& input_data,
                                std::string* output_data, std::string* error);
/// if subprocess has not been joined, kill it
void gpr_subprocess_destroy(gpr_subprocess* p);
/// returns exit status; can be called at most once
int gpr_subprocess_join(gpr_subprocess* p);
void gpr_subprocess_interrupt(gpr_subprocess* p);
int gpr_subprocess_get_process_id(gpr_subprocess* p);

#endif  // GRPC_SRC_CORE_UTIL_SUBPROCESS_H
