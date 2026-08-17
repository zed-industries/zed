//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_TEST_CORE_HANDSHAKE_SERVER_SSL_COMMON_H
#define GRPC_TEST_CORE_HANDSHAKE_SERVER_SSL_COMMON_H

bool server_ssl_test(const char* alpn_list[], unsigned int alpn_list_len,
                     const char* alpn_expected);

/// Cleans up the SSL library. To be called after the last call to
/// server_ssl_test returns. This is a NO-OP when gRPC is built against OpenSSL
/// versions > 1.0.2.
void CleanupSslLibrary();

#endif  // GRPC_TEST_CORE_HANDSHAKE_SERVER_SSL_COMMON_H
