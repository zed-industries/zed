// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PRIVACY_BUDGET_IDENTIFIABILITY_DIGEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PRIVACY_BUDGET_IDENTIFIABILITY_DIGEST_HELPERS_H_

#include "third_party/blink/public/common/privacy_budget/identifiable_token.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

// Provide helpers for blink-internal types to use with IdentifiabilityToken()

namespace blink {

// For benign strings only (i.e. those where the string is not sensitive).
PLATFORM_EXPORT IdentifiableToken
IdentifiabilityBenignStringToken(const String&);

// For sensitive strings, this function narrows the hash width to 16 bits.
PLATFORM_EXPORT IdentifiableToken
IdentifiabilitySensitiveStringToken(const String&);

// For benign strings only (i.e. those where the string is not sensitive). Token
// construction is additionally case-insensitive (using Unicode CaseFolding).
PLATFORM_EXPORT IdentifiableToken
IdentifiabilityBenignCaseFoldingStringToken(const String&);

// For sensitive strings, this function narrows the hash width to 16 bits. Token
// construction is additionally case-insensitive (using Unicode CaseFolding).
PLATFORM_EXPORT IdentifiableToken
IdentifiabilitySensitiveCaseFoldingStringToken(const String&);

// For vectors of benign strings only (i.e. those where the string is not
// sensitive).
PLATFORM_EXPORT IdentifiableToken
IdentifiabilityBenignStringVectorToken(const Vector<String>&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PRIVACY_BUDGET_IDENTIFIABILITY_DIGEST_HELPERS_H_
