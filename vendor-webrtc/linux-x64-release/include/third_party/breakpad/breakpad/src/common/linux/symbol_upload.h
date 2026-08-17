// -*- mode: c++ -*-

// Copyright 2011 Google LLC
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

// symbol_upload.h: helper functions for linux symbol upload tool.

#ifndef COMMON_LINUX_SYMBOL_UPLOAD_H_
#define COMMON_LINUX_SYMBOL_UPLOAD_H_

#include <string>

#include "common/using_std_string.h"

namespace google_breakpad {
namespace sym_upload {

enum class UploadProtocol {
  SYM_UPLOAD_V1,
  SYM_UPLOAD_V2,
};

constexpr char kBreakpadSymbolType[] = "BREAKPAD";

struct Options {
  Options() : upload_protocol(UploadProtocol::SYM_UPLOAD_V1), force(false) {}

  string symbolsPath;
  string uploadURLStr;
  string proxy;
  string proxy_user_pwd;
  string version;
  bool success;
  UploadProtocol upload_protocol;
  bool force;
  string api_key;

  // These only need to be set for native symbol uploads.
  string code_file;
  string debug_id;
  string type;
};

// Starts upload to symbol server with options.
void Start(Options* options);

}  // namespace sym_upload
}  // namespace google_breakpad

#endif  // COMMON_LINUX_SYMBOL_UPLOAD_H_
