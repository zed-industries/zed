// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_INSTALLED_SCRIPTS_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_INSTALLED_SCRIPTS_MANAGER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/network/content_security_policy_response_headers.h"
#include "third_party/blink/renderer/platform/network/http_header_map.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// InstalledScriptsManager provides the scripts of workers that have been
// installed. Currently it is only used for installed service workers.
class InstalledScriptsManager {
  USING_FAST_MALLOC(InstalledScriptsManager);

 public:
  InstalledScriptsManager() = default;

  class CORE_EXPORT ScriptData {
    USING_FAST_MALLOC(ScriptData);

   public:
    ScriptData() = default;
    ScriptData(const KURL& script_url,
               String source_text,
               std::unique_ptr<Vector<uint8_t>> meta_data,
               std::unique_ptr<CrossThreadHTTPHeaderMapData>);
    ScriptData(const ScriptData&) = delete;
    ScriptData& operator=(const ScriptData&) = delete;
    ScriptData(ScriptData&& other) = default;
    ScriptData& operator=(ScriptData&& other) = default;

    String TakeSourceText() { return std::move(source_text_); }
    std::unique_ptr<Vector<uint8_t>> TakeMetaData() {
      return std::move(meta_data_);
    }

    ContentSecurityPolicyResponseHeaders
    GetContentSecurityPolicyResponseHeaders();
    String GetReferrerPolicy();
    String GetHttpContentType();
    std::unique_ptr<Vector<String>> CreateOriginTrialTokens();

   private:
    KURL script_url_;
    String source_text_;
    std::unique_ptr<Vector<uint8_t>> meta_data_;
    HTTPHeaderMap headers_;
  };

  // Used on the main or worker thread. Returns true if the script has been
  // installed.
  virtual bool IsScriptInstalled(const KURL& script_url) const = 0;

  // Used on the worker thread. Returning nullptr indicates an error
  // happened while receiving the script from the browser process.
  // This can block if the script has not been received from the browser process
  // yet.
  virtual std::unique_ptr<ScriptData> GetScriptData(const KURL& script_url) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_INSTALLED_SCRIPTS_MANAGER_H_
