// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_PERMISSION_ELEMENT_STRINGS_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_PERMISSION_ELEMENT_STRINGS_MAP_H_

#include <stdint.h>

#include <optional>
#include <string_view>

namespace blink {

std::optional<uint16_t> GetPermissionElementMessageId(
    std::string_view language_code,
    uint16_t base_message);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_PERMISSION_ELEMENT_STRINGS_MAP_H_
