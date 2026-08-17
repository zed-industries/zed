// Copyright 2019 Google LLC
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
//
// Base64 escaping methods to encode/decode strings.

#ifndef TOOLS_WINDOWS_CONVERTER_EXE_ESCAPING_H_
#define TOOLS_WINDOWS_CONVERTER_EXE_ESCAPING_H_

#include <string>

namespace strings {

using std::string;

// ----------------------------------------------------------------------
// Base64Escape()
// WebSafeBase64Escape()
//    Encode "src" to "dest" using base64 encoding.
//    src is not null terminated, instead specify len.
//    'dest' should have at least CalculateBase64EscapedLen() length.
//    RETURNS the length of dest.
//    The WebSafe variation use '-' instead of '+' and '_' instead of '/'
//    so that we can place the out in the URL or cookies without having
//    to escape them.  It also has an extra parameter "do_padding",
//    which when set to false will prevent padding with "=".
// ----------------------------------------------------------------------
void Base64Escape(const string& src, string* dest);
int Base64Escape(const unsigned char* src, int slen, char* dest, int szdest);
// Encode src into dest with padding.
void Base64Escape(const unsigned char* src, int szsrc,
                  string* dest, bool do_padding);

int WebSafeBase64Escape(const unsigned char* src, int slen, char* dest,
                        int szdest, bool do_padding);
// Encode src into dest web-safely without padding.
void WebSafeBase64Escape(const string& src, string* dest);
// Encode src into dest web-safely with padding.
void WebSafeBase64EscapeWithPadding(const string& src, string* dest);
void WebSafeBase64Escape(const unsigned char* src, int szsrc,
                         string* dest, bool do_padding);

// ----------------------------------------------------------------------
// Base64Unescape()
// WebSafeBase64Unescape()
//    Copies "src" to "dest", where src is in base64 and is written to its
//    ASCII equivalents. src is not null terminated, instead specify len.
//    I recommend that slen<szdest, but we honor szdest anyway.
//    RETURNS the length of dest, or -1 if src contains invalid chars.
//    The WebSafe variation use '-' instead of '+' and '_' instead of '/'.
//    The variations that store into a string clear the string first, and
//    return false (with dest empty) if src contains invalid chars; for
//    these versions src and dest must be different strings.
// ----------------------------------------------------------------------
int Base64Unescape(const char* src, int slen, char* dest, int szdest);
bool Base64Unescape(const char* src, int slen, string* dest);
inline bool Base64Unescape(const string& src, string* dest) {
  return Base64Unescape(src.data(), src.size(), dest);
}


int WebSafeBase64Unescape(const char* src, int slen, char* dest, int szdest);
bool WebSafeBase64Unescape(const char* src, int slen, string* dest);
bool WebSafeBase64Unescape(const string& src, string* dest);

// Return the length to use for the output buffer given to the base64 escape
// routines. Make sure to use the same value for do_padding in both.
// This function may return incorrect results if given input_len values that
// are extremely high, which should happen rarely.
int CalculateBase64EscapedLen(int input_len, bool do_padding);
// Use this version when calling Base64Escape without a do_padding arg.
int CalculateBase64EscapedLen(int input_len);
}  // namespace strings

#endif  // TOOLS_WINDOWS_CONVERTER_EXE_ESCAPING_H_
