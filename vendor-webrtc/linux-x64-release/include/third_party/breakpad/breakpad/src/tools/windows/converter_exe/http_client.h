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

#ifndef TOOLS_CRASH_CONVERTER_WINDOWS_HTTP_CLIENT_H_
#define TOOLS_CRASH_CONVERTER_WINDOWS_HTTP_CLIENT_H_

#include <tchar.h>
#include <windows.h>
#include <vector>

typedef void* HttpHandle;

namespace crash {

// HttpClient provides an abstract layer for HTTP APIs. The actual
// implementation can be based on either WinHttp or WinInet.
class HttpClient {
 public:
  enum AccessType {
    ACCESS_TYPE_PRECONFIG,
    ACCESS_TYPE_DIRECT,
    ACCESS_TYPE_PROXY,
  };

  virtual ~HttpClient() {}

  virtual bool CrackUrl(const TCHAR* url,
                        DWORD flags,
                        TCHAR* scheme,
                        size_t scheme_buffer_length,
                        TCHAR* host,
                        size_t host_buffer_length,
                        TCHAR* uri,
                        size_t uri_buffer_length,
                        int* port) const = 0;
  virtual bool Open(const TCHAR* user_agent,
                    DWORD access_type,
                    const TCHAR* proxy_name,
                    const TCHAR* proxy_bypass,
                    HttpHandle* session_handle) const = 0;
  virtual bool Connect(HttpHandle session_handle,
                       const TCHAR* server,
                       int port,
                       HttpHandle* connection_handle) const = 0;
  virtual bool OpenRequest(HttpHandle connection_handle,
                           const TCHAR* verb,
                           const TCHAR* uri,
                           const TCHAR* version,
                           const TCHAR* referrer,
                           bool is_secure,
                           HttpHandle* request_handle) const = 0;
  virtual bool SendRequest(HttpHandle request_handle,
                           const TCHAR* headers,
                           DWORD headers_length) const = 0;
  virtual bool ReceiveResponse(HttpHandle request_handle) const = 0;
  virtual bool GetHttpStatusCode(HttpHandle request_handle,
                                 int* status_code) const = 0;
  virtual bool GetContentLength(HttpHandle request_handle,
                                DWORD* content_length) const = 0;
  virtual bool ReadData(HttpHandle request_handle,
                        void* buffer,
                        DWORD buffer_length,
                        DWORD* bytes_read) const = 0;
  virtual bool Close(HttpHandle handle) const = 0;

  static const DWORD kUnknownContentLength = (DWORD)-1;
};

}  // namespace crash

#endif  // TOOLS_CRASH_CONVERTER_WINDOWS_HTTP_CLIENT_H_
