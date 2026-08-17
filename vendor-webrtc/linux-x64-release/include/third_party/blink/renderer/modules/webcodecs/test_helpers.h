// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_TEST_HELPERS_H_

#include <optional>
#include <string_view>

#include "media/base/decrypt_config.h"
#include "media/base/encryption_scheme.h"
#include "third_party/blink/renderer/modules/webcodecs/array_buffer_util.h"

namespace blink {

// Copies a string data into a DOMArrayBuffer.
AllowSharedBufferSource* StringToBuffer(std::string_view data);

// Copies decoder buffer data into a std::string
std::string BufferToString(const media::DecoderBuffer& buffer);

// Creates a media::DecryptConfig with some simple test values. Returns nullptr
// for EncryptionScheme::kUnencrypted.
std::unique_ptr<media::DecryptConfig> CreateTestDecryptConfig(
    media::EncryptionScheme scheme,
    std::optional<media::EncryptionPattern> pattern = std::nullopt);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_TEST_HELPERS_H_
