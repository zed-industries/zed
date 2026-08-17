// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_ACCESS_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_ACCESS_ERROR_H_

#include "third_party/blink/public/mojom/file_system_access/file_system_access_error.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"

namespace blink {
namespace file_system_access_error {

// Rejects `resolver` with an appropriate exception if `status` represents an
// error. Resolves `resolver` with undefined otherwise.
void ResolveOrReject(ScriptPromiseResolver<IDLUndefined>* resolver,
                     const mojom::blink::FileSystemAccessError& status);

// Rejects `resolver` with an appropriate exception if `status` represents an
// error. DCHECKs otherwise.
void Reject(ScriptPromiseResolverBase* resolver,
            const mojom::blink::FileSystemAccessError& error);

}  // namespace file_system_access_error
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_ACCESS_ERROR_H_
