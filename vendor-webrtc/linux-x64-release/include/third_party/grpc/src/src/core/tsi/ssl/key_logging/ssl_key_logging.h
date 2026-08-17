// Copyright 2021 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_TSI_SSL_KEY_LOGGING_SSL_KEY_LOGGING_H
#define GRPC_SRC_CORE_TSI_SSL_KEY_LOGGING_SSL_KEY_LOGGING_H

#include <grpc/grpc_security.h>
#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/sync.h>
#include <openssl/ssl.h>

#include <iostream>
#include <map>

#include "absl/base/thread_annotations.h"
#include "src/core/util/memory.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/sync.h"

namespace tsi {

class TlsSessionKeyLoggerCache
    : public grpc_core::RefCounted<TlsSessionKeyLoggerCache> {
 public:
  TlsSessionKeyLoggerCache();
  ~TlsSessionKeyLoggerCache() override;

  // A helper class which facilitates appending Tls session keys into a file.
  // The instance is bound to a file meaning only one instance of this object
  // can ever exist for a given file path.
  class TlsSessionKeyLogger
      : public grpc_core::RefCounted<TlsSessionKeyLogger> {
   public:
    // Instantiates a TlsSessionKeyLogger instance bound to a specific path.
    TlsSessionKeyLogger(
        std::string tls_session_key_log_file_path,
        grpc_core::RefCountedPtr<TlsSessionKeyLoggerCache> cache);
    ~TlsSessionKeyLogger() override;

    // Not copyable nor assignable.
    TlsSessionKeyLogger(const TlsSessionKeyLogger&) = delete;
    TlsSessionKeyLogger& operator=(const TlsSessionKeyLogger&) = delete;
    // Writes session keys into the file in the NSS key logging format.
    // This is called upon completion of a handshake. The associated ssl_context
    // is also provided here to support future extensions such as logging
    // keys only when connections are made by certain IPs etc.
    void LogSessionKeys(SSL_CTX* ssl_context,
                        const std::string& session_keys_info);

   private:
    grpc_core::Mutex lock_;  // protects appends to file
    FILE* fd_ ABSL_GUARDED_BY(lock_);
    std::string tls_session_key_log_file_path_;
    grpc_core::RefCountedPtr<TlsSessionKeyLoggerCache> cache_;
  };
  // Creates and returns a TlsSessionKeyLogger instance.
  static grpc_core::RefCountedPtr<TlsSessionKeyLogger> Get(
      std::string tls_session_key_log_file_path);

 private:
  std::map<std::string, TlsSessionKeyLogger*> tls_session_key_logger_map_;
};

}  // namespace tsi

#endif  // GRPC_SRC_CORE_TSI_SSL_KEY_LOGGING_SSL_KEY_LOGGING_H
