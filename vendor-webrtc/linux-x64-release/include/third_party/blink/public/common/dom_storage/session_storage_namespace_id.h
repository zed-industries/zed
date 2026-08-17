// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_DOM_STORAGE_SESSION_STORAGE_NAMESPACE_ID_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_DOM_STORAGE_SESSION_STORAGE_NAMESPACE_ID_H_

#include <string>

#include "third_party/blink/public/common/common_export.h"

namespace blink {

// The length of session storage namespace ids.
constexpr const size_t kSessionStorageNamespaceIdLength = 36;

using SessionStorageNamespaceId = std::string;

// Allocates a unique session storage namespace id.
BLINK_COMMON_EXPORT SessionStorageNamespaceId
AllocateSessionStorageNamespaceId();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_DOM_STORAGE_SESSION_STORAGE_NAMESPACE_ID_H_
