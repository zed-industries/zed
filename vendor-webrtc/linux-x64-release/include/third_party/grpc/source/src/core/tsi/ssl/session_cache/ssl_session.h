//
//
// Copyright 2018 gRPC authors.
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
//
//

#ifndef GRPC_SRC_CORE_TSI_SSL_SESSION_CACHE_SSL_SESSION_H
#define GRPC_SRC_CORE_TSI_SSL_SESSION_CACHE_SSL_SESSION_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <openssl/ssl.h>

#include <memory>

#include "src/core/util/ref_counted.h"

// The main purpose of code here is to provide means to cache SSL sessions
// in a way that they can be shared between connections.
//
// SSL_SESSION stands for single instance of session and is not generally safe
// to share between SSL contexts with different lifetimes. It happens because
// not all SSL implementations guarantee immutability of SSL_SESSION object.
// See SSL_SESSION documentation in BoringSSL and OpenSSL for more details.

namespace tsi {

struct SslSessionDeleter {
  void operator()(SSL_SESSION* session) { SSL_SESSION_free(session); }
};

typedef std::unique_ptr<SSL_SESSION, SslSessionDeleter> SslSessionPtr;

/// SslCachedSession is an immutable thread-safe storage for single session
/// representation. It provides means to share SSL session data (e.g. TLS
/// ticket) between encrypted connections regardless of SSL context lifetime.
class SslCachedSession {
 public:
  // Not copyable nor movable.
  SslCachedSession(const SslCachedSession&) = delete;
  SslCachedSession& operator=(const SslCachedSession&) = delete;

  /// Create single cached instance of \a session.
  static std::unique_ptr<SslCachedSession> Create(SslSessionPtr session);

  virtual ~SslCachedSession() = default;

  /// Returns a copy of previously cached session.
  virtual SslSessionPtr CopySession() const = 0;

 protected:
  SslCachedSession() = default;
};

}  // namespace tsi

#endif  // GRPC_SRC_CORE_TSI_SSL_SESSION_CACHE_SSL_SESSION_H
