// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_DECRYPT_CONFIG_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_DECRYPT_CONFIG_UTIL_H_

#include <optional>

#include "media/base/encryption_scheme.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_decrypt_config.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace media {
class DecryptConfig;
}

namespace blink {

// Attempts to create a media::DecryptConfig given the JS version. Returns
// nullptr if no config can be created.
MODULES_EXPORT std::unique_ptr<media::DecryptConfig> CreateMediaDecryptConfig(
    const DecryptConfig& js_config);

// Attempts to create a media::EncryptionScheme value given the JS version.
// Returns std::nullopt if the scheme is unrecognized.
MODULES_EXPORT std::optional<media::EncryptionScheme> ToMediaEncryptionScheme(
    const String& scheme);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_DECRYPT_CONFIG_UTIL_H_
