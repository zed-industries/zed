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

// HTTPUpload provides a "nice" API to send a multipart HTTP(S) POST
// request using wininet.  It currently supports requests that contain
// a set of string parameters (key/value pairs), and a file to upload.

#ifndef COMMON_WINDOWS_HTTP_UPLOAD_H_
#define COMMON_WINDOWS_HTTP_UPLOAD_H_

#pragma warning(push)
// Disable exception handler warnings.
#pragma warning(disable : 4530)

#include <windows.h>
#include <wininet.h>

#include <map>
#include <string>

namespace google_breakpad {

using std::string;
using std::wstring;
using std::map;

class HTTPUpload {
 public:
  // Sends a PUT request containing the data in |path| to the given
  // URL. The data is encoded via the deflate algorithm, if support for such
  // is available at build-time.
  // Only HTTP(S) URLs are currently supported.  Returns true on success.
  // If the request is successful and response_body is non-NULL,
  // the response body will be returned in response_body.
  // If response_code is non-NULL, it will be set to the HTTP response code
  // received (or 0 if the request failed before getting an HTTP response).
  static bool SendPutRequest(
      const wstring& url,
      const wstring& path,
      int* timeout_ms,
      wstring* response_body,
      int* response_code);

  // Sends a GET request to the given URL.
  // Only HTTP(S) URLs are currently supported.  Returns true on success.
  // If the request is successful and response_body is non-NULL,
  // the response body will be returned in response_body.
  // If response_code is non-NULL, it will be set to the HTTP response code
  // received (or 0 if the request failed before getting an HTTP response).
  static bool SendGetRequest(
      const wstring& url,
      int* timeout_ms,
      wstring* response_body,
      int* response_code);

  // Sends the given sets of parameters and files as a multipart POST
  // request to the given URL.
  // Each key in |files| is the name of the file part of the request
  // (i.e. it corresponds to the name= attribute on an <input type="file">.
  // Parameter names must contain only printable ASCII characters,
  // and may not contain a quote (") character.
  // Only HTTP(S) URLs are currently supported.  Returns true on success.
  // If the request is successful and response_body is non-NULL,
  // the response body will be returned in response_body.
  // If response_code is non-NULL, it will be set to the HTTP response code
  // received (or 0 if the request failed before getting an HTTP response).
  static bool SendMultipartPostRequest(
      const wstring& url,
      const map<wstring, wstring>& parameters,
      const map<wstring, wstring>& files,
      int *timeout_ms,
      wstring *response_body,
      int *response_code);

  // Sends a POST request, with the body set to |body|, to the given URL.
  // Only HTTP(S) URLs are currently supported.  Returns true on success.
  // If the request is successful and response_body is non-NULL,
  // the response body will be returned in response_body.
  // If response_code is non-NULL, it will be set to the HTTP response code
  // received (or 0 if the request failed before getting an HTTP response).
  static bool SendSimplePostRequest(
      const wstring& url,
      const wstring& body,
      const wstring& content_type,
      int *timeout_ms,
      wstring *response_body,
      int *response_code);

 private:
  // No instances of this class should be created.
  // Disallow all constructors, destructors, and operator=.
  HTTPUpload();
  explicit HTTPUpload(const HTTPUpload&);
  void operator=(const HTTPUpload&);
  ~HTTPUpload();
};

}  // namespace google_breakpad

#pragma warning(pop)

#endif  // COMMON_WINDOWS_HTTP_UPLOAD_H_
