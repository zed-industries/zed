// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_CLASSIC_SCRIPT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_CLASSIC_SCRIPT_H_

#include "third_party/blink/renderer/bindings/core/v8/sanitize_script_errors.h"
#include "third_party/blink/renderer/bindings/core/v8/script_source_location_type.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/resource/script_resource.h"
#include "third_party/blink/renderer/core/script/script.h"
#include "third_party/blink/renderer/platform/loader/fetch/script_fetch_options.h"

namespace blink {

struct WebScriptSource;

class CORE_EXPORT ClassicScript final : public Script {
 public:
  // For `source_url`.
  static KURL StripFragmentIdentifier(const KURL&);

  // For scripts specified in the HTML spec or for tests.
  // Please leave spec comments and spec links that explain given argument
  // values at non-test callers.
  static ClassicScript* Create(
      const String& source_text,
      const KURL& source_url,
      const KURL& base_url,
      const ScriptFetchOptions&,
      ScriptSourceLocationType = ScriptSourceLocationType::kUnknown,
      SanitizeScriptErrors = SanitizeScriptErrors::kSanitize,
      CachedMetadataHandler* = nullptr,
      const TextPosition& start_position = TextPosition::MinimumPosition(),
      ScriptStreamer::NotStreamingReason =
          ScriptStreamer::NotStreamingReason::kInlineScript,
      InlineScriptStreamer* = nullptr);
  static ClassicScript* CreateFromResource(ScriptResource*,
                                           const ScriptFetchOptions&);

  // For scripts not specified in the HTML spec.
  //
  // New callers should use SanitizeScriptErrors::kSanitize as a safe default
  // value, while some existing callers uses kDoNotSanitize to preserve existing
  // behavior.
  // TODO(crbug/1112266): Use kSanitize for all existing callers if possible, or
  // otherwise add comments why kDoNotSanitize should be used.
  static ClassicScript* CreateUnspecifiedScript(
      const String& source_text,
      ScriptSourceLocationType = ScriptSourceLocationType::kUnknown,
      SanitizeScriptErrors = SanitizeScriptErrors::kSanitize);
  static ClassicScript* CreateUnspecifiedScript(
      const WebScriptSource&,
      SanitizeScriptErrors = SanitizeScriptErrors::kSanitize);

  // Use Create*() helpers above.
  ClassicScript(
      const ParkableString& source_text,
      const KURL& source_url,
      const KURL& base_url,
      const ScriptFetchOptions&,
      ScriptSourceLocationType,
      SanitizeScriptErrors,
      CachedMetadataHandler* = nullptr,
      const TextPosition& start_position = TextPosition::MinimumPosition(),
      ScriptStreamer* = nullptr,
      ScriptStreamer::NotStreamingReason =
          ScriptStreamer::NotStreamingReason::kInlineScript,
      ScriptCacheConsumer* = nullptr,
      const String& source_map_url = String());

  void Trace(Visitor*) const override;

  const ParkableString& SourceText() const { return source_text_; }

  ScriptSourceLocationType SourceLocationType() const {
    return source_location_type_;
  }

  SanitizeScriptErrors GetSanitizeScriptErrors() const {
    return sanitize_script_errors_;
  }

  CachedMetadataHandler* CacheHandler() const { return cache_handler_.Get(); }

  ScriptStreamer* Streamer() const { return streamer_.Get(); }
  ScriptStreamer::NotStreamingReason NotStreamingReason() const {
    return not_streaming_reason_;
  }

  ScriptCacheConsumer* CacheConsumer() const { return cache_consumer_.Get(); }

  const String& SourceMapUrl() const { return source_map_url_; }

  // Unlike RunScript(), callers of the following methods must enter a
  // v8::HandleScope before calling.
  [[nodiscard]] ScriptEvaluationResult RunScriptOnScriptStateAndReturnValue(
      ScriptState*,
      ExecuteScriptPolicy =
          ExecuteScriptPolicy::kDoNotExecuteScriptWhenScriptsDisabled,
      V8ScriptRunner::RethrowErrorsOption =
          V8ScriptRunner::RethrowErrorsOption::DoNotRethrow()) override;
  ScriptEvaluationResult RunScriptInIsolatedWorldAndReturnValue(
      LocalDOMWindow*,
      int32_t world_id);

  v8::ScriptOrigin CreateScriptOrigin(v8::Isolate* isolate) const;

 private:
  mojom::blink::ScriptType GetScriptType() const override {
    return mojom::blink::ScriptType::kClassic;
  }

  v8::Local<v8::Data> CreateHostDefinedOptions(v8::Isolate* isolate) const;

  const ParkableString source_text_;

  const ScriptSourceLocationType source_location_type_;

  const SanitizeScriptErrors sanitize_script_errors_;

  const Member<CachedMetadataHandler> cache_handler_;

  const Member<ScriptStreamer> streamer_;
  const ScriptStreamer::NotStreamingReason not_streaming_reason_;

  const Member<ScriptCacheConsumer> cache_consumer_;

  const String source_map_url_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_CLASSIC_SCRIPT_H_
