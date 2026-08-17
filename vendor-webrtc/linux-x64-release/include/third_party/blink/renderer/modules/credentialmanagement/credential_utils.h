// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIAL_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIAL_UTILS_H_

namespace blink {

class ScriptPromiseResolverBase;

// Checks non-page-origin-related security requirements for
// navigator.credentials and navigator.identity requests. Rejects the promise
// and returns false if the check fails.
bool CheckGenericSecurityRequirementsForCredentialsContainerRequest(
    ScriptPromiseResolverBase*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIAL_UTILS_H_
