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

#ifndef GRPC_TEST_CORE_TEST_UTIL_CMDLINE_H
#define GRPC_TEST_CORE_TEST_UTIL_CMDLINE_H

#include <grpc/support/port_platform.h>

#include <string>

#include "absl/log/log.h"

/// Simple command line parser.

/// Supports flags that can be specified as -foo, --foo, --no-foo, -no-foo, etc
/// And integers, strings that can be specified as -foo=4, -foo blah, etc

/// No support for short command line options (but we may get that in the
/// future.)

/// Usage (for a program with a single flag argument 'foo'):

/// int main(int argc, char **argv) {
///   gpr_cmdline *cl;
///   int verbose = 0;

///  cl = gpr_cmdline_create("My cool tool");
///  gpr_cmdline_add_int(cl, "verbose", "Produce verbose output?", &verbose);
///  gpr_cmdline_parse(cl, argc, argv);
///  gpr_cmdline_destroy(cl);

///  if (verbose) {
///    LOG(INFO) << "Goodbye cruel world!";
///  }

///  return 0;
///}

typedef struct gpr_cmdline gpr_cmdline;

/// Construct a command line parser: takes a short description of the tool
/// doing the parsing
gpr_cmdline* gpr_cmdline_create(const char* description);
/// Add an integer parameter, with a name (used on the command line) and some
/// helpful text (used in the command usage)
void gpr_cmdline_add_int(gpr_cmdline* cl, const char* name, const char* help,
                         int* value);
/// The same, for a boolean flag
void gpr_cmdline_add_flag(gpr_cmdline* cl, const char* name, const char* help,
                          int* value);
/// And for a string
void gpr_cmdline_add_string(gpr_cmdline* cl, const char* name, const char* help,
                            const char** value);
/// Set a callback for non-named arguments
void gpr_cmdline_on_extra_arg(
    gpr_cmdline* cl, const char* name, const char* help,
    void (*on_extra_arg)(void* user_data, const char* arg), void* user_data);
/// Enable surviving failure: default behavior is to exit the process
void gpr_cmdline_set_survive_failure(gpr_cmdline* cl);
/// Parse the command line; returns 1 on success, on failure either dies
///(by default) or returns 0 if gpr_cmdline_set_survive_failure() has been
/// called
int gpr_cmdline_parse(gpr_cmdline* cl, int argc, char** argv);
/// Destroy the parser
void gpr_cmdline_destroy(gpr_cmdline* cl);
/// Get a string describing usage
std::string gpr_cmdline_usage_string(gpr_cmdline* cl, const char* argv0);

#endif  // GRPC_TEST_CORE_TEST_UTIL_CMDLINE_H
