// Copyright 2009 Google LLC
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


#ifndef COMMON_LINUX_GOOGLE_CRASHDUMP_UPLOADER_H_
#define COMMON_LINUX_GOOGLE_CRASHDUMP_UPLOADER_H_

#include <map>
#include <memory>
#include <string>

#include "common/linux/libcurl_wrapper.h"
#include "common/using_std_string.h"

namespace google_breakpad {

class GoogleCrashdumpUploader {
 public:
  GoogleCrashdumpUploader(const string& product,
                          const string& version,
                          const string& guid,
                          const string& ptime,
                          const string& ctime,
                          const string& email,
                          const string& comments,
                          const string& minidump_pathname,
                          const string& crash_server,
                          const string& proxy_host,
                          const string& proxy_userpassword);

  GoogleCrashdumpUploader(const string& product,
                          const string& version,
                          const string& guid,
                          const string& ptime,
                          const string& ctime,
                          const string& email,
                          const string& comments,
                          const string& minidump_pathname,
                          const string& crash_server,
                          const string& proxy_host,
                          const string& proxy_userpassword,
                          std::unique_ptr<LibcurlWrapper> http_layer);

  void Init(const string& product,
            const string& version,
            const string& guid,
            const string& ptime,
            const string& ctime,
            const string& email,
            const string& comments,
            const string& minidump_pathname,
            const string& crash_server,
            const string& proxy_host,
            const string& proxy_userpassword,
            std::unique_ptr<LibcurlWrapper> http_layer);
  bool Upload(int* http_status_code,
              string* http_response_header,
              string* http_response_body);

 private:
  bool CheckRequiredParametersArePresent();

  std::unique_ptr<LibcurlWrapper> http_layer_;
  string product_;
  string version_;
  string guid_;
  string ptime_;
  string ctime_;
  string email_;
  string comments_;
  string minidump_pathname_;

  string crash_server_;
  string proxy_host_;
  string proxy_userpassword_;

  std::map<string, string> parameters_;
};
}

#endif  // COMMON_LINUX_GOOGLE_CRASHDUMP_UPLOADER_H_
