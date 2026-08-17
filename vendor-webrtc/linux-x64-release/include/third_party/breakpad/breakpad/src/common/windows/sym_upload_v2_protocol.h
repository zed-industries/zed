// Copyright 2022 Google LLC
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

#ifndef COMMON_WINDOWS_SYM_UPLOAD_V2_PROTOCOL_H_
#define COMMON_WINDOWS_SYM_UPLOAD_V2_PROTOCOL_H_

#include <string>

namespace google_breakpad {

// Sends file at |symbol_filename| using the sym-upload-v2 protocol to
// |api_url| using key |api_key|, and using identifiers |debug_file| and
// |debug_id|. |timeout_ms| is the number of milliseconds to wait before
// terminating the upload attempt. |symbol_type| is the type of the symbol
// file, which is one of:
//   "BREAKPAD"
//   "ELF"
//   "PE"
//   "MACHO"
//   "DEBUG_ONLY"
//   "DWP"
//   "DSYM"
//   "PDB"
//   "SOURCE_MAP"
// If |product_name| is non-empty then it will be sent as part of the symbol
// metadata.
// If |force| is set then it will overwrite an existing file with the
// same |debug_file| and |debug_id| in the store.
bool SymUploadV2ProtocolSend(const wchar_t* api_url,
                             const wchar_t* api_key,
                             int* timeout_ms,
                             const std::wstring& debug_file,
                             const std::wstring& debug_id,
                             const std::wstring& symbol_filename,
                             const std::wstring& symbol_type,
                             const std::wstring& product_name,
                             bool force);

}  // namespace google_breakpad

#endif  // COMMON_WINDOWS_SYM_UPLOAD_V2_PROTOCOL_H_