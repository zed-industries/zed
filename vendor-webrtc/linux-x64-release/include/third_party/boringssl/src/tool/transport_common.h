// Copyright 2014 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_TOOL_TRANSPORT_COMMON_H
#define OPENSSL_HEADER_TOOL_TRANSPORT_COMMON_H

#include <openssl/ssl.h>
#include <string.h>

#include <string>

// InitSocketLibrary calls the Windows socket init functions, if needed.
bool InitSocketLibrary();

// Connect sets |*out_sock| to be a socket connected to the destination given
// in |hostname_and_port|, which should be of the form "www.example.com:123".
// It returns true on success and false otherwise.
bool Connect(int *out_sock, const std::string &hostname_and_port);

class Listener {
 public:
  Listener() {}
  ~Listener();

  // Init initializes the listener to listen on |port|, which should be of the
  // form "123".
  bool Init(const std::string &port);

  // Accept sets |*out_sock| to be a socket connected to the listener.
  bool Accept(int *out_sock);

 private:
  int server_sock_ = -1;

  Listener(const Listener &) = delete;
  Listener &operator=(const Listener &) = delete;
};

bool VersionFromString(uint16_t *out_version, const std::string &version);

void PrintConnectionInfo(BIO *bio, const SSL *ssl);

bool SocketSetNonBlocking(int sock, bool is_non_blocking);

// PrintSSLError prints information about the most recent SSL error to stderr.
// |ssl_err| must be the output of |SSL_get_error| and the |SSL| object must be
// connected to socket from |Connect|.
void PrintSSLError(FILE *file, const char *msg, int ssl_err, int ret);

bool TransferData(SSL *ssl, int sock);

// DoSMTPStartTLS performs the SMTP STARTTLS mini-protocol over |sock|. It
// returns true on success and false otherwise.
bool DoSMTPStartTLS(int sock);

// DoHTTPTunnel sends an HTTP CONNECT request over |sock|. It returns true on
// success and false otherwise.
bool DoHTTPTunnel(int sock, const std::string &hostname_and_port);

#endif  // !OPENSSL_HEADER_TOOL_TRANSPORT_COMMON_H
