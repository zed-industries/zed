// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_CONTENT_SECURITY_POLICY_PARSERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_CONTENT_SECURITY_POLICY_PARSERS_H_

#include "third_party/blink/renderer/platform/crypto.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

// Check if value matches
// https://w3c.github.io/webappsec-csp/#grammardef-serialized-policy. We also
// allow a trailing ';', repeated ';'s, and a trailing ';' followed by spaces.
//
// Note: this is currently used only for checking validity of the csp attribute,
// otherwise all CSP parsing is performed by the network parser.
PLATFORM_EXPORT
bool MatchesTheSerializedCSPGrammar(const String& value);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_CONTENT_SECURITY_POLICY_PARSERS_H_
