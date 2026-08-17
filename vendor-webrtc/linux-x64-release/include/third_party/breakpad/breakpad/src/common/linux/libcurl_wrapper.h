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

// A wrapper for libcurl to do HTTP Uploads, to support easy mocking
// and unit testing of the HTTPUpload class.

#ifndef COMMON_LINUX_LIBCURL_WRAPPER_H_
#define COMMON_LINUX_LIBCURL_WRAPPER_H_

#include <string>
#include <map>

#include "common/using_std_string.h"
#include "third_party/curl/curl.h"

namespace google_breakpad {

// This class is only safe to be used on single-threaded code because of its
// usage of libcurl's curl_global_cleanup().
class LibcurlWrapper {
 public:
  LibcurlWrapper();
  virtual ~LibcurlWrapper();
  virtual bool Init();
  virtual bool SetProxy(const string& proxy_host,
                        const string& proxy_userpwd);
  virtual bool AddFile(const string& upload_file_path,
                       const string& basename);
  virtual bool SendRequest(const string& url,
                           const std::map<string, string>& parameters,
                           long* http_status_code,
                           string* http_header_data,
                           string* http_response_data);
  bool SendGetRequest(const string& url,
                      long* http_status_code,
                      string* http_header_data,
                      string* http_response_data);
  bool SendPutRequest(const string& url,
                      const string& path,
                      long* http_status_code,
                      string* http_header_data,
                      string* http_response_data);
  bool SendSimplePostRequest(const string& url,
                             const string& body,
                             const string& content_type,
                             long* http_status_code,
                             string* http_header_data,
                             string* http_response_data);

 private:
  // This function initializes class state corresponding to function
  // pointers into the CURL library.
  bool SetFunctionPointers();

  bool SendRequestInner(const string& url,
                        long* http_status_code,
                        string* http_header_data,
                        string* http_response_data);

  void Reset();

  bool CheckInit();

  bool init_ok_;                 // Whether init succeeded
  void* curl_lib_;               // Pointer to result of dlopen() on
                                 // curl library
  string last_curl_error_;  // The text of the last error when
                                 // dealing
  // with CURL.

  CURL* curl_;                   // Pointer for handle for CURL calls.

  CURL* (*easy_init_)(void);

  // Stateful pointers for calling into curl_formadd()
  struct curl_httppost* formpost_;
  struct curl_httppost* lastptr_;
  struct curl_slist* headerlist_;

  // Function pointers into CURL library
  CURLcode (*easy_setopt_)(CURL*, CURLoption, ...);
  CURLFORMcode (*formadd_)(struct curl_httppost**,
                           struct curl_httppost**, ...);
  struct curl_slist* (*slist_append_)(struct curl_slist*, const char*);
  void (*slist_free_all_)(struct curl_slist*);
  CURLcode (*easy_perform_)(CURL*);
  const char* (*easy_strerror_)(CURLcode);
  void (*easy_cleanup_)(CURL*);
  CURLcode (*easy_getinfo_)(CURL*, CURLINFO info, ...);
  void (*easy_reset_)(CURL*);
  void (*formfree_)(struct curl_httppost*);
  void (*global_cleanup_)(void);

};
}

#endif  // COMMON_LINUX_LIBCURL_WRAPPER_H_
