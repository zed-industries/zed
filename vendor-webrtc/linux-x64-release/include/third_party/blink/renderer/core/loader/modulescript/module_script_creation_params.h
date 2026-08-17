// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_CREATION_PARAMS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_CREATION_PARAMS_H_

#include "base/check_op.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/bindings/core/v8/script_source_location_type.h"
#include "third_party/blink/renderer/bindings/core/v8/script_streamer.h"
#include "third_party/blink/renderer/core/script/modulator.h"
#include "third_party/blink/renderer/platform/bindings/parkable_string.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/cached_metadata_handler.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// Spec module types. Return value of
// "https://html.spec.whatwg.org/#module-type-from-module-request".
enum class ModuleType { kInvalid, kJavaScriptOrWasm, kJSON, kCSS };

// Non-standard internal module types. The spec defines javascript-or-wasm
// as the module type for module requests without an explicit type and uses
// the MIME type to disambiguate between JavaScript and Wasm modules.
// We don't pass the MIME type to `ModuleScript`, instead, we resolve the
// kJavascriptOrWasm type to kJavaScript or kWasm before calling the
// ModuleScriptCreationParams constructor.
enum class ResolvedModuleType { kJSON, kCSS, kJavaScript, kWasm };

// ModuleScriptCreationParams contains parameters for creating ModuleScript.
class ModuleScriptCreationParams {
  DISALLOW_NEW();

 public:
  ModuleScriptCreationParams(
      const KURL& source_url,
      const KURL& base_url,
      ScriptSourceLocationType source_location_type,
      const ResolvedModuleType module_type,
      const ParkableString& source_text,
      CachedMetadataHandler* cache_handler,
      network::mojom::ReferrerPolicy response_referrer_policy,
      ScriptStreamer* script_streamer = nullptr,
      ScriptStreamer::NotStreamingReason not_streaming_reason =
          ScriptStreamer::NotStreamingReason::kStreamingDisabled)
      : source_url_(source_url),
        base_url_(base_url),
        source_location_type_(source_location_type),
        module_type_(module_type),
        is_isolated_(false),
        source_text_(source_text),
        isolated_source_text_(),
        cache_handler_(cache_handler),
        response_referrer_policy_(response_referrer_policy),
        script_streamer_(script_streamer),
        not_streaming_reason_(not_streaming_reason) {
    DCHECK(source_location_type == ScriptSourceLocationType::kExternalFile ||
           source_location_type == ScriptSourceLocationType::kInline);
    // https://html.spec.whatwg.org/multipage/webappapis.html#concept-script-base-url
    if (source_location_type == ScriptSourceLocationType::kExternalFile) {
      DCHECK_EQ(source_url, base_url);
    }
    DCHECK_EQ(
        !script_streamer,
        not_streaming_reason != ScriptStreamer::NotStreamingReason::kInvalid);
  }

  ~ModuleScriptCreationParams() = default;

  ModuleScriptCreationParams IsolatedCopy() const {
    String isolated_source_text = isolated_source_text_
                                      ? isolated_source_text_
                                      : GetSourceText().ToString();
    return ModuleScriptCreationParams(
        SourceURL(), BaseURL(), source_location_type_, GetModuleType(),
        isolated_source_text, response_referrer_policy_);
  }

  ResolvedModuleType GetModuleType() const { return module_type_; }

  const KURL& SourceURL() const { return source_url_; }
  const KURL& BaseURL() const { return base_url_; }

  const ParkableString& GetSourceText() const {
    if (is_isolated_) {
      source_text_ = ParkableString(isolated_source_text_.ReleaseImpl());
      isolated_source_text_ = String();
      is_isolated_ = false;
    }
    return source_text_;
  }

  ScriptSourceLocationType SourceLocationType() const {
    return source_location_type_;
  }

  ModuleScriptCreationParams CopyWithClearedSourceText() const {
    return ModuleScriptCreationParams(
        source_url_, base_url_, source_location_type_, module_type_,
        ParkableString(), /*cache_handler=*/nullptr, response_referrer_policy_,
        /*script_streamer=*/nullptr,
        ScriptStreamer::NotStreamingReason::kStreamingDisabled);
  }

  CachedMetadataHandler* CacheHandler() const { return cache_handler_; }

  bool IsSafeToSendToAnotherThread() const { return is_isolated_; }

  ScriptStreamer* GetScriptStreamer() const { return script_streamer_; }
  ScriptStreamer::NotStreamingReason NotStreamingReason() const {
    return not_streaming_reason_;
  }

  static String ModuleTypeToString(const ModuleType module_type);

  network::mojom::ReferrerPolicy ResponseReferrerPolicy() const {
    return response_referrer_policy_;
  }

 private:
  // Creates an isolated copy.
  ModuleScriptCreationParams(
      const KURL& source_url,
      const KURL& base_url,
      ScriptSourceLocationType source_location_type,
      const ResolvedModuleType& module_type,
      const String& isolated_source_text,
      network::mojom::ReferrerPolicy response_referrer_policy)
      : source_url_(source_url),
        base_url_(base_url),
        source_location_type_(source_location_type),
        module_type_(module_type),
        is_isolated_(true),
        source_text_(),
        isolated_source_text_(isolated_source_text),
        response_referrer_policy_(response_referrer_policy),
        // The ScriptStreamer is intentionally cleared since it cannot be passed
        // across threads. This only disables script streaming on worklet
        // top-level scripts where the ModuleScriptCreationParams is
        // passed across threads.
        script_streamer_(nullptr),
        not_streaming_reason_(
            ScriptStreamer::NotStreamingReason::kStreamingDisabled) {}

  const KURL source_url_;
  const KURL base_url_;
  const ScriptSourceLocationType source_location_type_;
  const ResolvedModuleType module_type_;

  // Mutable because an isolated copy can become bound to a thread when
  // calling GetSourceText().
  mutable bool is_isolated_;
  mutable ParkableString source_text_;
  mutable String isolated_source_text_;

  // |cache_handler_| is cleared when crossing thread boundaries.
  Persistent<CachedMetadataHandler> cache_handler_;

  // This is the referrer policy specified in the `Referrer-Policy` header on
  // the response associated with the module script that `this` represents. This
  // will always be `kDefault` if there is no referrer policy sent in the
  // response. Consumers of this policy are responsible for detecting this.
  const network::mojom::ReferrerPolicy response_referrer_policy_;

  // |script_streamer_| is cleared when crossing thread boundaries.
  Persistent<ScriptStreamer> script_streamer_;
  const ScriptStreamer::NotStreamingReason not_streaming_reason_;
};

}  // namespace blink

namespace WTF {

// Creates a deep copy because |script_streamer_| is not
// cross-thread-transfer-safe.
template <>
struct CrossThreadCopier<blink::ModuleScriptCreationParams> {
  static blink::ModuleScriptCreationParams Copy(
      const blink::ModuleScriptCreationParams& params) {
    return params.IsolatedCopy();
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_CREATION_PARAMS_H_
