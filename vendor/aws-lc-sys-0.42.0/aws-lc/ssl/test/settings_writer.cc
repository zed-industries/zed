// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include "settings_writer.h"

#include <stdio.h>

#include <openssl/ssl.h>

#include "fuzzer_tags.h"
#include "test_config.h"


SettingsWriter::SettingsWriter() {}

bool SettingsWriter::Init(int i, const TestConfig *config,
                          SSL_SESSION *session) {
  if (config->write_settings.empty()) {
    return true;
  }
  // Treat write_settings as a path prefix for each connection in the run.
  char buf[DECIMAL_SIZE(int)];
  snprintf(buf, sizeof(buf), "%d", i);
  path_ = config->write_settings + buf;

  if (!CBB_init(cbb_.get(), 64)) {
    return false;
  }

  if (session != nullptr) {
    uint8_t *data;
    size_t len;
    if (!SSL_SESSION_to_bytes(session, &data, &len)) {
      return false;
    }
    bssl::UniquePtr<uint8_t> free_data(data);
    CBB child;
    if (!CBB_add_u16(cbb_.get(), kSessionTag) ||
        !CBB_add_u24_length_prefixed(cbb_.get(), &child) ||
        !CBB_add_bytes(&child, data, len) || !CBB_flush(cbb_.get())) {
      return false;
    }
  }

  if (config->is_server &&
      (config->require_any_client_certificate || config->verify_peer) &&
      !CBB_add_u16(cbb_.get(), kRequestClientCert)) {
    return false;
  }

  return true;
}

bool SettingsWriter::Commit() {
  if (path_.empty()) {
    return true;
  }

  uint8_t *settings;
  size_t settings_len;
  if (!CBB_add_u16(cbb_.get(), kDataTag) ||
      !CBB_finish(cbb_.get(), &settings, &settings_len)) {
    return false;
  }
  bssl::UniquePtr<uint8_t> free_settings(settings);

  struct FileCloser {
    void operator()(FILE *f) const { fclose(f); }
  };
  using ScopedFILE = std::unique_ptr<FILE, FileCloser>;
  ScopedFILE file(fopen(path_.c_str(), "w"));
  if (!file) {
    return false;
  }

  return fwrite(settings, settings_len, 1, file.get()) == 1;
}

bool SettingsWriter::WriteHandoff(bssl::Span<const uint8_t> handoff) {
  return WriteData(kHandoffTag, handoff);
}

bool SettingsWriter::WriteHandback(bssl::Span<const uint8_t> handback) {
  return WriteData(kHandbackTag, handback);
}

bool SettingsWriter::WriteHints(bssl::Span<const uint8_t> hints) {
  return WriteData(kHintsTag, hints);
}

bool SettingsWriter::WriteData(uint16_t tag, bssl::Span<const uint8_t> data) {
  if (path_.empty()) {
    return true;
  }

  CBB child;
  if (!CBB_add_u16(cbb_.get(), tag) ||
      !CBB_add_u24_length_prefixed(cbb_.get(), &child) ||
      !CBB_add_bytes(&child, data.data(), data.size()) ||
      !CBB_flush(cbb_.get())) {
    return false;
  }
  return true;
}
