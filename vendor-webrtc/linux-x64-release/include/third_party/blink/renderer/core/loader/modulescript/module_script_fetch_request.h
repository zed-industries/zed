// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_FETCH_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_FETCH_REQUEST_H_

#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/platform/loader/fetch/script_fetch_options.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/referrer.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

enum class ModuleType;

// A ModuleScriptFetchRequest essentially serves as a "parameter object" for
// Modulator::Fetch{Single,NewSingle}.
class ModuleScriptFetchRequest final {
  STACK_ALLOCATED();

 public:
  // Referrer is set only for internal module script fetch algorithms triggered
  // from ModuleTreeLinker to fetch descendant module scripts.
  ModuleScriptFetchRequest(const KURL& url,
                           ModuleType module_type,
                           mojom::blink::RequestContextType context_type,
                           network::mojom::RequestDestination destination,
                           const ScriptFetchOptions& options,
                           const String& referrer_string,
                           const TextPosition& referrer_position)
      : url_(url),
        expected_module_type_(module_type),
        context_type_(context_type),
        destination_(destination),
        options_(options),
        referrer_string_(referrer_string),
        referrer_position_(referrer_position) {}

  static ModuleScriptFetchRequest CreateForTest(const KURL& url,
                                                ModuleType module_type) {
    return ModuleScriptFetchRequest(
        url, module_type, mojom::blink::RequestContextType::SCRIPT,
        network::mojom::RequestDestination::kScript, ScriptFetchOptions(),
        Referrer::ClientReferrerString(), TextPosition::MinimumPosition());
  }
  ~ModuleScriptFetchRequest() = default;

  const KURL& Url() const { return url_; }
  ModuleType GetExpectedModuleType() const { return expected_module_type_; }
  mojom::blink::RequestContextType ContextType() const { return context_type_; }
  network::mojom::RequestDestination Destination() const {
    return destination_;
  }
  const ScriptFetchOptions& Options() const { return options_; }
  const String& ReferrerString() const { return referrer_string_; }
  const TextPosition& GetReferrerPosition() const { return referrer_position_; }

 private:
  const KURL url_;
  const ModuleType expected_module_type_;
  const mojom::blink::RequestContextType context_type_;
  const network::mojom::RequestDestination destination_;
  const ScriptFetchOptions options_;
  const String referrer_string_;
  const TextPosition referrer_position_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_FETCH_REQUEST_H_
