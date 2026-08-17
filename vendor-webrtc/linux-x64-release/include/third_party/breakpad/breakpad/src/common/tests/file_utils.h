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

// file_utils.h: Define utility functions for file manipulation, which
// are used for testing.

#ifndef COMMON_TESTS_FILE_UTILS_H_
#define COMMON_TESTS_FILE_UTILS_H_

#include <string>

namespace google_breakpad {

// Copies a file from |from_path| to |to_path|. Returns true on success.
bool CopyFile(const std::string& from_path, const std::string& to_path);
bool CopyFile(const char* from_path, const char* to_path);

// Reads the content of a file at |path| into |buffer|. |buffer_size| specifies
// the size of |buffer| in bytes and returns the number of bytes read from the
// file on success. Returns true on success.
bool ReadFile(const char* path, void* buffer, ssize_t* buffer_size);

// Writes |buffer_size| bytes of the content in |buffer| to a file at |path|.
// Returns true on success.
bool WriteFile(const char* path, const void* buffer, size_t buffer_size);

}  // namespace google_breakpad

#endif  // COMMON_TESTS_FILE_UTILS_H_
