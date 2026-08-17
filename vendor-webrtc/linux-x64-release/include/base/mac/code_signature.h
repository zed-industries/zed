// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MAC_CODE_SIGNATURE_H_
#define BASE_MAC_CODE_SIGNATURE_H_

#include <Security/Security.h>
#include <mach/mach.h>
#include <unistd.h>

#include <string_view>

#include "base/apple/scoped_cftyperef.h"
#include "base/base_export.h"
#include "base/types/expected.h"

namespace base::mac {

enum class SignatureValidationType {
  // Verify that the running application has a valid code signature and
  // that it is unchanged from the copy on disk.
  DynamicAndStatic,

  // Verify that the running application has a valid code signature.
  // Do not verify that the application matches the copy on disk.
  // The contents of the Info.plist of the process must be provided.
  DynamicOnly,
};

// Returns whether `process` has a valid code signature that fulfills
// `requirement`.
BASE_EXPORT
OSStatus ProcessIsSignedAndFulfillsRequirement(
    audit_token_t process,
    SecRequirementRef requirement,
    SignatureValidationType validation_type =
        SignatureValidationType::DynamicAndStatic,
    std::string_view info_plist_xml = {});

// Returns whether the process with PID `pid` has a valid code signature
// that fulfills `requirement`.
//
// DEPRECATED: Do not use this function in new code. Use
// `ProcessIsSignedAndFulfillsRequirement` instead. Process IDs do not uniquely
// identify a process so it is impossible to make trust decisions based on them.
BASE_EXPORT
OSStatus ProcessIdIsSignedAndFulfillsRequirement_DoNotUse(
    pid_t pid,
    SecRequirementRef requirement,
    SignatureValidationType validation_type =
        SignatureValidationType::DynamicAndStatic,
    std::string_view info_plist_xml = {});

// Create a SecRequirementRef from a requirement string.
//
// Returns a null reference if the requirement string was invalid.
BASE_EXPORT
base::apple::ScopedCFTypeRef<SecRequirementRef> RequirementFromString(
    std::string_view requirement_string);

// Return a SecCodeRef representing the current process.
//
// Validation performed against this code object will validate the running
// process only, and will not verify that the application matches the copy on
// disk.
BASE_EXPORT
base::expected<base::apple::ScopedCFTypeRef<SecCodeRef>, OSStatus>
DynamicCodeObjectForCurrentProcess();

}  // namespace base::mac

#endif  // BASE_MAC_CODE_SIGNATURE_H_
