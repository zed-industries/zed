// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

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
