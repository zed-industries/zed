// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_STORAGE_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_STORAGE_ERROR_H_

#include "third_party/blink/public/mojom/cache_storage/cache_storage.mojom-blink.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ScriptPromiseResolverBase;

// Reject the |resolver| with the appropriate error given |web_error|.
// When no |message| is provided, the standard one is chosen.
void RejectCacheStorageWithError(ScriptPromiseResolverBase* resolver,
                                 mojom::blink::CacheStorageError web_error,
                                 const String& message = String());
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_STORAGE_ERROR_H_
