// Copyright 2006 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// windows_frame_info.h: Holds debugging information about a stack frame.
//
// This structure is specific to Windows debugging information obtained
// from pdb files using the DIA API.
//
// Author: Mark Mentovai


#ifndef PROCESSOR_WINDOWS_FRAME_INFO_H__
#define PROCESSOR_WINDOWS_FRAME_INFO_H__

#include <string.h>
#include <stdlib.h>

#include <string>
#include <vector>

#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"
#include "processor/logging.h"
#include "processor/tokenize.h"

namespace google_breakpad {

#ifdef _WIN32
#define strtoull _strtoui64
#endif

struct WindowsFrameInfo {
 public:
  enum Validity {
    VALID_NONE           = 0,
    VALID_PARAMETER_SIZE = 1,
    VALID_ALL            = -1
  };

  // The types for stack_info_.  This is equivalent to MS DIA's
  // StackFrameTypeEnum.  Each identifies a different type of frame
  // information, although all are represented in the symbol file in the
  // same format.  These are used as indices to the stack_info_ array.
  enum StackInfoTypes {
    STACK_INFO_FPO = 0,
    STACK_INFO_TRAP,  // not used here
    STACK_INFO_TSS,   // not used here
    STACK_INFO_STANDARD,
    STACK_INFO_FRAME_DATA,
    STACK_INFO_LAST,  // must be the last sequentially-numbered item
    STACK_INFO_UNKNOWN = -1
  };

  WindowsFrameInfo() : type_(STACK_INFO_UNKNOWN),
                     valid(VALID_NONE),
                     prolog_size(0),
                     epilog_size(0),
                     parameter_size(0),
                     saved_register_size(0),
                     local_size(0),
                     max_stack_size(0),
                     allocates_base_pointer(0),
                     program_string() {}

  WindowsFrameInfo(StackInfoTypes type,
                 uint32_t set_prolog_size,
                 uint32_t set_epilog_size,
                 uint32_t set_parameter_size,
                 uint32_t set_saved_register_size,
                 uint32_t set_local_size,
                 uint32_t set_max_stack_size,
                 int set_allocates_base_pointer,
                 const string& set_program_string)
      : type_(type),
        valid(VALID_ALL),
        prolog_size(set_prolog_size),
        epilog_size(set_epilog_size),
        parameter_size(set_parameter_size),
        saved_register_size(set_saved_register_size),
        local_size(set_local_size),
        max_stack_size(set_max_stack_size),
        allocates_base_pointer(set_allocates_base_pointer),
        program_string(set_program_string) {}

  // Parse a textual serialization of a WindowsFrameInfo object from
  // a string. Returns NULL if parsing fails, or a new object
  // otherwise. type, rva and code_size are present in the STACK line,
  // but not the StackFrameInfo structure, so return them as outparams.
  static WindowsFrameInfo *ParseFromString(const string& string,
                                           int& type,
                                           uint64_t& rva,
                                           uint64_t& code_size) {
    // The format of a STACK WIN record is documented at:
    //
    // https://chromium.googlesource.com/breakpad/breakpad/+/master/docs/symbol_files.md

    std::vector<char>  buffer;
    StringToVector(string, buffer);
    std::vector<char*> tokens;
    if (!Tokenize(&buffer[0], " \r\n", 11, &tokens))
      return nullptr;

    type = strtol(tokens[0], nullptr, 16);
    if (type < 0 || type > STACK_INFO_LAST - 1)
      return nullptr;

    rva                           = strtoull(tokens[1], nullptr, 16);
    code_size                     = strtoull(tokens[2], nullptr, 16);
    uint32_t prolog_size          =  strtoul(tokens[3], nullptr, 16);
    uint32_t epilog_size          =  strtoul(tokens[4], nullptr, 16);
    uint32_t parameter_size       =  strtoul(tokens[5], nullptr, 16);
    uint32_t saved_register_size  =  strtoul(tokens[6], nullptr, 16);
    uint32_t local_size           =  strtoul(tokens[7], nullptr, 16);
    uint32_t max_stack_size       =  strtoul(tokens[8], nullptr, 16);
    int has_program_string        =  strtoul(tokens[9], nullptr, 16);

    const char *program_string = "";
    int allocates_base_pointer = 0;
    if (has_program_string) {
      program_string = tokens[10];
    } else {
      allocates_base_pointer = strtoul(tokens[10], nullptr, 16);
    }

    return new WindowsFrameInfo(static_cast<StackInfoTypes>(type),
                                prolog_size,
                                epilog_size,
                                parameter_size,
                                saved_register_size,
                                local_size,
                                max_stack_size,
                                allocates_base_pointer,
                                program_string);
  }

  // CopyFrom makes "this" WindowsFrameInfo object identical to "that".
  void CopyFrom(const WindowsFrameInfo& that) {
    type_ = that.type_;
    valid = that.valid;
    prolog_size = that.prolog_size;
    epilog_size = that.epilog_size;
    parameter_size = that.parameter_size;
    saved_register_size = that.saved_register_size;
    local_size = that.local_size;
    max_stack_size = that.max_stack_size;
    allocates_base_pointer = that.allocates_base_pointer;
    program_string = that.program_string;
  }

  // Clears the WindowsFrameInfo object so that users will see it as though
  // it contains no information.
  void Clear() {
    type_ = STACK_INFO_UNKNOWN;
    valid = VALID_NONE;
    program_string.erase();
  }

  StackInfoTypes type_;

  // Identifies which fields in the structure are valid.  This is of
  // type Validity, but it is defined as an int because it's not
  // possible to OR values into an enumerated type.  Users must check
  // this field before using any other.
  int valid;

  // These values come from IDiaFrameData.
  uint32_t prolog_size;
  uint32_t epilog_size;
  uint32_t parameter_size;
  uint32_t saved_register_size;
  uint32_t local_size;
  uint32_t max_stack_size;

  // Only one of allocates_base_pointer or program_string will be valid.
  // If program_string is empty, use allocates_base_pointer.
  bool allocates_base_pointer;
  string program_string;
};

}  // namespace google_breakpad


#endif  // PROCESSOR_WINDOWS_FRAME_INFO_H__
