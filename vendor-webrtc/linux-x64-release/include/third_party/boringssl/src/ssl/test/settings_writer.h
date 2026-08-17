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

#ifndef HEADER_SETTINGS_WRITER
#define HEADER_SETTINGS_WRITER

#include <string>

#include <openssl/bytestring.h>
#include <openssl/ssl.h>

#include "test_config.h"

struct SettingsWriter {
 public:
  SettingsWriter();

  // Init initializes the writer for a new connection, given by |i|.  Each
  // connection gets a unique output file.
  bool Init(int i, const TestConfig *config, SSL_SESSION *session);

  // Commit writes the buffered data to disk.
  bool Commit();

  bool WriteHandoff(bssl::Span<const uint8_t> handoff);
  bool WriteHandback(bssl::Span<const uint8_t> handback);
  bool WriteHints(bssl::Span<const uint8_t> hints);

 private:
  bool WriteData(uint16_t tag, bssl::Span<const uint8_t> data);

  std::string path_;
  bssl::ScopedCBB cbb_;
};

#endif  // HEADER_SETTINGS_WRITER
