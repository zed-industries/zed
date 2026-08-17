// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)

#ifndef GOOGLE_PROTOBUF_COMPILER_SUBPROCESS_H__
#define GOOGLE_PROTOBUF_COMPILER_SUBPROCESS_H__

#ifdef _WIN32
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN  // right...
#endif
#include <windows.h>
#else  // _WIN32
#include <sys/types.h>
#include <unistd.h>
#endif  // !_WIN32
#include <string>

#include "google/protobuf/port.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {

class Message;

namespace compiler {

// Utility class for launching sub-processes.
class PROTOC_EXPORT Subprocess {
 public:
  Subprocess();
  ~Subprocess();

  enum SearchMode {
    SEARCH_PATH,  // Use PATH environment variable.
    EXACT_NAME    // Program is an exact file name; don't use the PATH.
  };

  // Start the subprocess.  Currently we don't provide a way to specify
  // arguments as protoc plugins don't have any.
  void Start(const std::string& program, SearchMode search_mode);

  // Serialize the input message and pipe it to the subprocess's stdin, then
  // close the pipe.  Meanwhile, read from the subprocess's stdout and parse
  // the data into *output.  All this is done carefully to avoid deadlocks.
  // Returns true if successful.  On any sort of error, returns false and sets
  // *error to a description of the problem.
  bool Communicate(const Message& input, Message* output, std::string* error);

#ifdef _WIN32
  // Given an error code, returns a human-readable error message.  This is
  // defined here so that CommandLineInterface can share it.
  static std::string Win32ErrorMessage(DWORD error_code);
#endif

 private:
#ifdef _WIN32
  DWORD process_start_error_;
  HANDLE child_handle_;

  // The file handles for our end of the child's pipes.  We close each and
  // set it to NULL when no longer needed.
  HANDLE child_stdin_;
  HANDLE child_stdout_;

#else  // _WIN32
  pid_t child_pid_;

  // The file descriptors for our end of the child's pipes.  We close each and
  // set it to -1 when no longer needed.
  int child_stdin_;
  int child_stdout_;

#endif  // !_WIN32
};

}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_SUBPROCESS_H__
