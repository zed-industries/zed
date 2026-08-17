// Copyright 2018 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef HEADER_TEST_HANDSHAKE
#define HEADER_TEST_HANDSHAKE

#include <functional>

#include <openssl/base.h>

#include "settings_writer.h"


#if defined(OPENSSL_LINUX) && !defined(OPENSSL_ANDROID)
#define HANDSHAKER_SUPPORTED
#endif

// RetryAsync is called after a failed operation on |ssl| with return code
// |ret|. If the operation should be retried, it simulates one asynchronous
// event and returns true. Otherwise it returns false.
bool RetryAsync(SSL *ssl, int ret);

// CheckIdempotentError runs |func|, an operation on |ssl|, ensuring that
// errors are idempotent.
int CheckIdempotentError(const char *name, SSL *ssl, std::function<int()> func);

#if defined(HANDSHAKER_SUPPORTED)
// DoSplitHandshake delegates the SSL handshake to a separate process, called
// the handshaker.  This process proxies I/O between the handshaker and the
// client, using the |BIO| from |ssl|.  After a successful handshake, |ssl| is
// replaced with a new |SSL| object, in a way that is intended to be invisible
// to the caller.
bool DoSplitHandshake(bssl::UniquePtr<SSL> *ssl, SettingsWriter *writer,
                      bool is_resume);

// GetHandshakeHint requests a handshake hint from the handshaker process and
// configures the result on |ssl|. It returns true on success and false on
// error.
bool GetHandshakeHint(SSL *ssl, SettingsWriter *writer, bool is_resume,
                      const SSL_CLIENT_HELLO *client_hello);

// The protocol between the proxy and the handshaker is defined by these
// single-character prefixes. |kControlMsgDone| uses 'H' for compatibility with
// older binaries.
constexpr char kControlMsgWantRead = 'R';        // Handshaker wants data
constexpr char kControlMsgWriteCompleted = 'W';  // Proxy has sent data
constexpr char kControlMsgDone = 'H';            // Proxy should resume control
constexpr char kControlMsgError = 'E';           // Handshaker hit an error

// The protocol between the proxy and handshaker uses these file descriptors.
constexpr int kFdControl = 3;            // Bi-directional dgram socket.
constexpr int kFdProxyToHandshaker = 4;  // Uni-directional pipe.
constexpr int kFdHandshakerToProxy = 5;  // Uni-directional pipe.
#endif  // HANDSHAKER_SUPPORTED

#endif  // HEADER_TEST_HANDSHAKE
