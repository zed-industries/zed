// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_MODULE_RECORD_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_MODULE_RECORD_H_

#include "third_party/blink/renderer/bindings/core/v8/module_request.h"
#include "third_party/blink/renderer/bindings/core/v8/script_evaluation_result.h"
#include "third_party/blink/renderer/bindings/core/v8/script_source_location_type.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_code_cache.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/cached_metadata_handler.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8.h"

namespace blink {

class KURL;
class ModuleScriptCreationParams;
class ScriptFetchOptions;
class ScriptState;
class ScriptValue;

// ModuleRecordProduceCacheData is a parameter object for
// ModuleRecord::ProduceCache().
class CORE_EXPORT ModuleRecordProduceCacheData final
    : public GarbageCollected<ModuleRecordProduceCacheData> {
 public:
  ModuleRecordProduceCacheData(v8::Isolate*,
                               CachedMetadataHandler*,
                               V8CodeCache::ProduceCacheOptions,
                               v8::Local<v8::Module>);

  void Trace(Visitor*) const;

  CachedMetadataHandler* CacheHandler() const { return cache_handler_.Get(); }
  V8CodeCache::ProduceCacheOptions GetProduceCacheOptions() const {
    return produce_cache_options_;
  }
  v8::Local<v8::UnboundModuleScript> UnboundScript(v8::Isolate* isolate) const {
    return unbound_script_.Get(isolate);
  }

 private:
  Member<CachedMetadataHandler> cache_handler_;
  V8CodeCache::ProduceCacheOptions produce_cache_options_;
  TraceWrapperV8Reference<v8::UnboundModuleScript> unbound_script_;
};

// TODO(rikaf): Add a class level comment
class CORE_EXPORT ModuleRecord final {
  STATIC_ONLY(ModuleRecord);

 public:
  static v8::Local<v8::Module> Compile(
      ScriptState*,
      const ModuleScriptCreationParams& params,
      const ScriptFetchOptions&,
      const TextPosition&,
      mojom::blink::V8CacheOptions = mojom::blink::V8CacheOptions::kDefault,
      ModuleRecordProduceCacheData** out_produce_cache_data = nullptr);

  // Returns exception, if any.
  static ScriptValue Instantiate(ScriptState*,
                                 v8::Local<v8::Module> record,
                                 const KURL& source_url);

  static void ReportException(ScriptState*, v8::Local<v8::Value> exception);

  static Vector<ModuleRequest> ModuleRequests(ScriptState*,
                                              v8::Local<v8::Module> record);

  static v8::Local<v8::Value> V8Namespace(v8::Local<v8::Module> record);

  // ToBlinkImportAttributes deserializes v8::FixedArray encoded import
  // attributes to blink::ImportAttribute. When
  // |v8_import_attributes_has_positions| is set to true, it expects [key1,
  // value1, position1, key2, value2, position2, ...] encoding used in
  // v8::ModuleRequest::GetImportAttributes(). When it is set to false, it
  // expects [key1, value1, key2, value2, ...] encoding used in the
  // |HostImportModuleDynamically| callback.
  static Vector<ImportAttribute> ToBlinkImportAttributes(
      v8::Local<v8::Context> context,
      v8::Local<v8::Module> record,
      v8::Local<v8::FixedArray> v8_import_attributes,
      bool v8_import_attributes_has_positions);

 private:
  static v8::MaybeLocal<v8::Module> ResolveModuleCallback(
      v8::Local<v8::Context>,
      v8::Local<v8::String> specifier,
      v8::Local<v8::FixedArray> import_attributes,
      v8::Local<v8::Module> referrer);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_MODULE_RECORD_H_
