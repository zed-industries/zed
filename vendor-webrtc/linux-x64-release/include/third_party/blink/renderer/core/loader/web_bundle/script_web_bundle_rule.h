// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_BUNDLE_SCRIPT_WEB_BUNDLE_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_BUNDLE_SCRIPT_WEB_BUNDLE_RULE_H_

#include <variant>

#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/web_bundle/script_web_bundle_error.h"
#include "third_party/blink/renderer/platform/loader/fetch/console_logger.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/kurl_hash.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

// Represents a rule which is used in <script type=webbundle>.
// Explainer:
// https://github.com/WICG/webpackage/blob/main/explainers/subresource-loading.md
class CORE_EXPORT ScriptWebBundleRule final {
 public:
  static std::variant<ScriptWebBundleRule, ScriptWebBundleError> ParseJson(
      const String& inline_text,
      const KURL& base_url,
      ConsoleLogger* logger);

  ScriptWebBundleRule(const KURL& source_url,
                      network::mojom::CredentialsMode credentialsl_mode,
                      HashSet<KURL> scope_urls,
                      HashSet<KURL> resource_urls);

  bool ResourcesOrScopesMatch(const KURL& url) const;
  const KURL& source_url() const { return source_url_; }
  const HashSet<KURL>& scope_urls() const { return scope_urls_; }
  const HashSet<KURL>& resource_urls() const { return resource_urls_; }
  network::mojom::CredentialsMode credentials_mode() const {
    return credentials_mode_;
  }

 private:
  KURL source_url_;
  network::mojom::CredentialsMode credentials_mode_;
  HashSet<KURL> scope_urls_;
  HashSet<KURL> resource_urls_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_BUNDLE_SCRIPT_WEB_BUNDLE_RULE_H_
