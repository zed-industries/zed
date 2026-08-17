// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRUST_TOKEN_ATTRIBUTE_PARSING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRUST_TOKEN_ATTRIBUTE_PARSING_H_

#include <memory>

#include "services/network/public/mojom/trust_tokens.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class JSONValue;

namespace internal {

// Given a JSON representation of a Trust Token parameters struct, constructs
// and returns the represented struct if the JSON representation is valid;
// returns nullopt otherwise.
CORE_EXPORT network::mojom::blink::TrustTokenParamsPtr TrustTokenParamsFromJson(
    std::unique_ptr<JSONValue> in);

}  // namespace internal

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRUST_TOKEN_ATTRIBUTE_PARSING_H_
