/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_DEPRECATED_STORAGE_QUOTA_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_DEPRECATED_STORAGE_QUOTA_H_

#include "third_party/blink/public/mojom/quota/quota_manager_host.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/bindings/exception_code.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class ExecutionContext;
class ScriptState;
class V8StorageErrorCallback;
class V8StorageQuotaCallback;
class V8StorageUsageCallback;

class DeprecatedStorageQuota final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static void EnqueueStorageErrorCallback(ScriptState*,
                                          V8StorageErrorCallback*,
                                          DOMExceptionCode);

  explicit DeprecatedStorageQuota(ExecutionContext*);

  void queryUsageAndQuota(ScriptState*,
                          V8StorageUsageCallback*,
                          V8StorageErrorCallback* = nullptr);

  void requestQuota(ScriptState*,
                    uint64_t new_quota_in_bytes,
                    V8StorageQuotaCallback* = nullptr,
                    V8StorageErrorCallback* = nullptr);

  void Trace(Visitor*) const override;

 private:
  // Binds the interface (if not already bound) with the given interface
  // provider, and returns it,
  mojom::blink::QuotaManagerHost* GetQuotaHost(ExecutionContext*);

  HeapMojoRemote<mojom::blink::QuotaManagerHost> quota_host_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_DEPRECATED_STORAGE_QUOTA_H_
