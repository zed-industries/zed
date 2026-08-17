// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_WEBUI_BUNDLED_CACHED_METADATA_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_WEBUI_BUNDLED_CACHED_METADATA_HANDLER_H_

#include "third_party/blink/renderer/platform/loader/fetch/cached_metadata.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/cached_metadata_handler.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"

namespace blink {

// CodeCacheHandler for WebUI bundled script metadata. See crbug.com/378504631
// for design details.
class PLATFORM_EXPORT WebUIBundledCachedMetadataHandler final
    : public CachedMetadataHandler {
 public:
  WebUIBundledCachedMetadataHandler();
  ~WebUIBundledCachedMetadataHandler() override;

  // CachedMetadataHandler:
  void Trace(Visitor*) const override;
  void SetCachedMetadata(CodeCacheHost* code_cache_host,
                         uint32_t data_type_id,
                         base::span<const uint8_t> data) override;
  void SetSerializedCachedMetadata(mojo_base::BigBuffer data) override;
  void ClearCachedMetadata(CodeCacheHost* code_cache_host,
                           ClearCacheType cache_type) override;
  scoped_refptr<CachedMetadata> GetCachedMetadata(
      uint32_t data_type_id,
      GetCachedMetadataBehavior behavior) const override;
  String Encoding() const override;
  ServingSource GetServingSource() const override;
  void OnMemoryDump(WebProcessMemoryDump* pmd,
                    const String& dump_prefix) const override;
  size_t GetCodeCacheSize() const override;
  void DidUseCodeCache(bool was_rejected) override;

  bool did_use_code_cache_for_testing() const {
    return did_use_code_cache_for_testing_;
  }

 private:
  bool did_use_code_cache_for_testing_ = false;

  scoped_refptr<CachedMetadata> cached_metadata_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_WEBUI_BUNDLED_CACHED_METADATA_HANDLER_H_
