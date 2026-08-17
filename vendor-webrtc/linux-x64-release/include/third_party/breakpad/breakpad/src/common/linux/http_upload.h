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
// request using libcurl.  It currently supports requests that contain
// a set of string parameters (key/value pairs), and a file to upload.

#ifndef COMMON_LINUX_HTTP_UPLOAD_H__
#define COMMON_LINUX_HTTP_UPLOAD_H__

#include <map>
#include <string>

#include "common/using_std_string.h"

namespace google_breakpad {

using std::map;

class HTTPUpload {
 public:
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
  // If the send fails, a description of the error will be
  // returned in error_description.
  static bool SendRequest(const string& url,
                          const map<string, string>& parameters,
                          const map<string, string>& files,
                          const string& proxy,
                          const string& proxy_user_pwd,
                          const string& ca_certificate_file,
                          string* response_body,
                          long* response_code,
                          string* error_description);

 private:
  // Checks that the given list of parameters has only printable
  // ASCII characters in the parameter name, and does not contain
  // any quote (") characters.  Returns true if so.
  static bool CheckParameters(const map<string, string>& parameters);

  // Checks the curl_lib parameter points to a valid curl lib.
  static bool CheckCurlLib(void* curl_lib);

  // No instances of this class should be created.
  // Disallow all constructors, destructors, and operator=.
  HTTPUpload();
  explicit HTTPUpload(const HTTPUpload&);
  void operator=(const HTTPUpload&);
  ~HTTPUpload();
};

}  // namespace google_breakpad

#endif  // COMMON_LINUX_HTTP_UPLOAD_H__
