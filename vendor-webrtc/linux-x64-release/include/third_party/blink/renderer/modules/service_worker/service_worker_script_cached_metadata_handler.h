// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_SCRIPT_CACHED_METADATA_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_SCRIPT_CACHED_METADATA_HANDLER_H_

#include <stdint.h>
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/cached_metadata_handler.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CachedMetadata;
class ServiceWorkerGlobalScope;

class ServiceWorkerScriptCachedMetadataHandler : public CachedMetadataHandler {
 public:
  ServiceWorkerScriptCachedMetadataHandler(
      ServiceWorkerGlobalScope*,
      const KURL& script_url,
      std::unique_ptr<Vector<uint8_t>> meta_data);
  ~ServiceWorkerScriptCachedMetadataHandler() override;
  void Trace(Visitor*) const override;
  void SetCachedMetadata(CodeCacheHost*,
                         uint32_t data_type_id,
                         base::span<const uint8_t>) override;
  void SetSerializedCachedMetadata(mojo_base::BigBuffer data) override;
  void ClearCachedMetadata(CodeCacheHost*, ClearCacheType) override;
  scoped_refptr<CachedMetadata> GetCachedMetadata(
      uint32_t data_type_id,
      GetCachedMetadataBehavior behavior = kCrashIfUnchecked) const override;
  String Encoding() const override;
  void OnMemoryDump(WebProcessMemoryDump* pmd,
                    const String& dump_prefix) const override;
  size_t GetCodeCacheSize() const override;

 private:
  Member<ServiceWorkerGlobalScope> global_scope_;
  KURL script_url_;
  scoped_refptr<CachedMetadata> cached_metadata_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_SCRIPT_CACHED_METADATA_HANDLER_H_
