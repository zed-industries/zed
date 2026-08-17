// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CONTENT_SECURITY_POLICY_VIOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CONTENT_SECURITY_POLICY_VIOLATION_TYPE_H_

namespace blink {
// This covers the possible values of a violation's 'resource', as defined in
// https://w3c.github.io/webappsec-csp/#violation-resource. By the time we
// generate a report, we're guaranteed that the value isn't 'null', so we
// don't need that state in this enum.
//
// Trusted Types violation's 'resource' values are defined in
// https://wicg.github.io/trusted-types/dist/spec/#csp-violation-object-hdr.
enum ContentSecurityPolicyViolationType {
  kInlineViolation,
  kEvalViolation,
  kURLViolation,
  kSRIViolation,
  kTrustedTypesSinkViolation,
  kTrustedTypesPolicyViolation,
  kWasmEvalViolation
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CONTENT_SECURITY_POLICY_VIOLATION_TYPE_H_
