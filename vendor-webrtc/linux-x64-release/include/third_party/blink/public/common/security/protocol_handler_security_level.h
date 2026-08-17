// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SECURITY_PROTOCOL_HANDLER_SECURITY_LEVEL_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SECURITY_PROTOCOL_HANDLER_SECURITY_LEVEL_H_
namespace blink {

// Levels of security used for Navigator's (un)registerProtocolHandler.
// This is an increasing hierarchy, starting from the default HTML5 behavior and
// for which each higher level removes a security restriction.
enum class ProtocolHandlerSecurityLevel {
  // Default behavior per the HTML5 specification.
  // https://html.spec.whatwg.org/multipage/system-state.html#normalize-protocol-handler-parameters
  kStrict,

  // Similar to kStrict, but allows URLs with non-HTTP(S) schemes.
  kSameOrigin,

  // Allow registration calls to cross-origin HTTP/HTTPS URLs.
  kUntrustedOrigins,

  // Allow extension features: ext+foo schemes and chrome-extension:// URLs.
  kExtensionFeatures,
};

inline ProtocolHandlerSecurityLevel ProtocolHandlerSecurityLevelFrom(
    int security_level) {
  return security_level < 0 ||
                 security_level >
                     static_cast<int>(
                         ProtocolHandlerSecurityLevel::kExtensionFeatures)
             ? ProtocolHandlerSecurityLevel::kStrict
             : static_cast<ProtocolHandlerSecurityLevel>(security_level);
}

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SECURITY_PROTOCOL_HANDLER_SECURITY_LEVEL_H_
