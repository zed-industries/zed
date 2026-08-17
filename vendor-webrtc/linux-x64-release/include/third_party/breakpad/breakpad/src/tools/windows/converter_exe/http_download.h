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

#ifndef TOOLS_WINDOWS_CONVERTER_EXE_HTTP_DOWNLOAD_H_
#define TOOLS_WINDOWS_CONVERTER_EXE_HTTP_DOWNLOAD_H_

#include <map>
#include <string>
#include "tools/windows/converter_exe/winhttp_client.h"

namespace crash {

using std::map;
using std::string;
using std::wstring;

class HTTPDownload {
 public:
  // Retrieves the resource located at |url|, a http or https URL, via WinInet.
  // The request is fetched with GET request; the optional |parameters| are
  // appended to the URL.  Returns true on success, placing the content of the
  // retrieved resource in |content|.  Returns false on failure.  HTTP status
  // codes other than 200 cause Download to return false.  If |status_code| is
  // supplied, it will be set to the value of the HTTP status code, if an HTTP
  // transaction occurs.  If Download fails before a transaction can occur,
  // |status_code| will be set to 0.  Any failures will result in messages
  // being printed to stderr.
  static bool Download(const wstring& url,
                       const map<wstring, wstring>* parameters,
                       string *content, int *status_code);
 private:
  static HttpClient* CreateHttpClient(const wchar_t*);
};

}  // namespace crash

#endif  // TOOLS_WINDOWS_CONVERTER_EXE_HTTP_DOWNLOAD_H_
