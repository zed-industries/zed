// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_CODE_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_CODE_CACHE_H_

#include <stdint.h>

#include "third_party/blink/public/mojom/v8_cache_options.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_source_location_type.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_compile_hints_common.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/v8_binding_macros.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/cached_metadata_handler.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace WTF {
class TextEncoding;
class TextPosition;
}  // namespace WTF

namespace blink {

class CachedMetadata;
class CodeCacheHost;
class ClassicScript;
class KURL;
class ModuleRecordProduceCacheData;
class ScriptState;

class CORE_EXPORT V8CodeCache final {
  STATIC_ONLY(V8CodeCache);

 public:
  enum class OpaqueMode {
    kOpaque,
    kNotOpaque,
  };

  enum class ProduceCacheOptions {
    kNoProduceCache,
    kSetTimeStamp,
    kProduceCodeCache,
  };

  static uint32_t TagForBundledCodeCache();
  static uint32_t TagForCodeCache(const CachedMetadataHandler*);
  static uint32_t TagForTimeStamp(const CachedMetadataHandler*);
  static uint32_t TagForCompileHints(const CachedMetadataHandler*);
  static void SetCacheTimeStamp(CodeCacheHost*, CachedMetadataHandler*);

  static uint64_t GetTimestamp();

  // Returns true iff the CachedMetadataHandler contains a hot time stamp or a
  // compile hints cache containing a hot timestamp.
  static bool HasHotTimestamp(const CachedMetadataHandler* cache_handler);
  static bool HasHotTimestamp(const CachedMetadata& data,
                              const String& encoding);

  // Returns true iff the CachedMetadataHandler contains a code cache
  // that can be consumed by V8.
  static bool HasCodeCache(
      const CachedMetadataHandler*,
      CachedMetadataHandler::GetCachedMetadataBehavior behavior =
          CachedMetadataHandler::kCrashIfUnchecked);
  static bool HasCodeCache(const CachedMetadata& data, const String& encoding);

  static bool HasCompileHints(
      const CachedMetadataHandler*,
      CachedMetadataHandler::GetCachedMetadataBehavior behavior =
          CachedMetadataHandler::kCrashIfUnchecked);
  static bool HasHotCompileHints(const CachedMetadata& data,
                                 const String& encoding);

  // `can_use_crowdsourced_compile_hints` may be set to true only if we're
  // compiling a script in a LocalMainFrame.
  static std::tuple<v8::ScriptCompiler::CompileOptions,
                    ProduceCacheOptions,
                    v8::ScriptCompiler::NoCacheReason>
  GetCompileOptions(
      mojom::blink::V8CacheOptions cache_options,
      const ClassicScript&,
      bool might_generate_crowdsourced_compile_hints = false,
      bool can_use_crowdsourced_compile_hints = false,
      v8_compile_hints::MagicCommentMode v8_compile_hints_magic_comment_mode =
          v8_compile_hints::MagicCommentMode::kNone);
  static std::tuple<v8::ScriptCompiler::CompileOptions,
                    ProduceCacheOptions,
                    v8::ScriptCompiler::NoCacheReason>
  GetCompileOptions(
      mojom::blink::V8CacheOptions cache_options,
      const CachedMetadataHandler*,
      size_t source_text_length,
      ScriptSourceLocationType,
      const KURL& url,
      bool might_generate_crowdsourced_compile_hints = false,
      bool can_use_crowdsourced_compile_hints = false,
      v8_compile_hints::MagicCommentMode v8_compile_hints_magic_comment_mode =
          v8_compile_hints::MagicCommentMode::kNone);

  static bool IsFull(const CachedMetadata* metadata);

  static scoped_refptr<CachedMetadata> GetCachedMetadata(
      const CachedMetadataHandler* cache_handler,
      CachedMetadataHandler::GetCachedMetadataBehavior behavior =
          CachedMetadataHandler::kCrashIfUnchecked);
  static scoped_refptr<CachedMetadata> GetCachedMetadataForCompileHints(
      const CachedMetadataHandler* cache_handler,
      CachedMetadataHandler::GetCachedMetadataBehavior behavior =
          CachedMetadataHandler::kCrashIfUnchecked);
  static std::unique_ptr<v8::ScriptCompiler::CachedData> CreateCachedData(
      scoped_refptr<CachedMetadata>);
  static std::unique_ptr<v8::ScriptCompiler::CachedData> CreateCachedData(
      const CachedMetadataHandler*);

  static void ProduceCache(v8::Isolate*,
                           CodeCacheHost*,
                           v8::Local<v8::Script>,
                           CachedMetadataHandler*,
                           size_t source_text_length,
                           const KURL& source_url,
                           const WTF::TextPosition& source_start_position,
                           ProduceCacheOptions);
  static void ProduceCache(v8::Isolate*,
                           CodeCacheHost*,
                           ModuleRecordProduceCacheData*,
                           size_t source_text_length,
                           const KURL& source_url,
                           const WTF::TextPosition& source_start_position);

  static scoped_refptr<CachedMetadata> GenerateFullCodeCache(
      ScriptState*,
      const String& script_string,
      const KURL& source_url,
      const WTF::TextEncoding&,
      OpaqueMode);

  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum class GetMetadataType {
    kNone = 0,
    kHotTimestamp = 1,
    kColdTimestamp = 2,
    kLocalCompileHintsWithHotTimestamp = 3,
    kLocalCompileHintsWithColdTimestamp = 4,
    kCodeCache = 5,
    kMaxValue = kCodeCache
  };

  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum class SetMetadataType {
    kTimestamp = 0,
    kLocalCompileHintsAtFMP = 1,
    kLocalCompileHintsAtInteractive = 2,
    kCodeCache = 3,
    kMaxValue = kCodeCache
  };

  static void RecordCacheGetStatistics(
      const CachedMetadataHandler* cache_handler);
  static void RecordCacheGetStatistics(const CachedMetadata* cached_metadata,
                                       const String& encoding);
  static void RecordCacheGetStatistics(GetMetadataType metadata_type);

  static void RecordCacheSetStatistics(SetMetadataType metadata_type);

 private:
  static std::tuple<v8::ScriptCompiler::CompileOptions,
                    ProduceCacheOptions,
                    v8::ScriptCompiler::NoCacheReason>
  GetCompileOptionsInternal(mojom::blink::V8CacheOptions cache_options,
                            const CachedMetadataHandler*,
                            size_t source_text_length,
                            ScriptSourceLocationType,
                            const KURL& url,
                            bool might_generate_crowdsourced_compile_hints,
                            bool can_use_crowdsourced_compile_hints);
};

inline base::span<const uint8_t> ToSpan(
    const v8::ScriptCompiler::CachedData& data) {
  // SAFETY: v8::ScriptCompiler::CachedData ensures its `data` and `length`
  // are safe.
  return UNSAFE_BUFFERS(
      base::span(data.data, static_cast<size_t>(data.length)));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_CODE_CACHE_H_
