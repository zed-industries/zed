// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_BUNDLE_SCRIPT_WEB_BUNDLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_BUNDLE_SCRIPT_WEB_BUNDLE_H_

#include <variant>

#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/web_bundle/script_web_bundle_error.h"
#include "third_party/blink/renderer/core/loader/web_bundle/script_web_bundle_rule.h"
#include "third_party/blink/renderer/core/script/script_element_base.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/loader/fetch/subresource_web_bundle.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}

namespace blink {

class Document;
class KURL;
class WebBundleLoader;

// ScriptLoader creates this for a script whose type is "webbundle".
//
// https://github.com/WICG/webpackage/blob/main/explainers/subresource-loading.md#script-based-api
class CORE_EXPORT ScriptWebBundle final
    : public GarbageCollected<ScriptWebBundle>,
      public SubresourceWebBundle {
 public:
  static std::variant<ScriptWebBundle*, ScriptWebBundleError>
  CreateOrReuseInline(ScriptElementBase&, const String& inline_text);

  ScriptWebBundle(ScriptElementBase& element,
                  Document& element_document,
                  const ScriptWebBundleRule& rule);

  void Trace(Visitor* visitor) const override;

  // SubresourceWebBundle overrides:
  bool CanHandleRequest(const KURL& url) const override;
  String GetCacheIdentifier() const override;
  const KURL& GetBundleUrl() const override;
  const base::UnguessableToken& WebBundleToken() const override;
  void NotifyLoadingFinished() override;
  void OnWebBundleError(const String& message) const override;
  bool IsScriptWebBundle() const override;
  bool WillBeReleased() const override;
  network::mojom::CredentialsMode GetCredentialsMode() const override;

  void CreateBundleLoaderAndRegister();
  void ReleaseBundleLoaderAndUnregister();

  void WillReleaseBundleLoaderAndUnregister();
  void ReusedWith(ScriptElementBase& element, ScriptWebBundleRule rule);
  class ReleaseResourceTask;

 private:
  // Returns true if the bundle's URL and |element_document_|'s frame have the
  // same origin. Note: Since a redirect is not supported yet, this check is
  // done by checking the bundle's URL specified in ScriptWebBundleRule.
  bool IsSameOriginBundle() const;

  bool will_be_released_ = false;
  // We store both |element_| and |element_document_| here,
  // because we need |element_| for dispatching load/error events
  // and creating/releasing WebBundleLoader we want to be extra
  // careful that we're dealing with the correct document, so
  // we explicitly capture it at the time of PrepareScript().
  WeakMember<ScriptElementBase> element_;
  WeakMember<Document> element_document_;
  ScriptWebBundleRule rule_;
  Member<WebBundleLoader> bundle_loader_;
};

template <>
struct DowncastTraits<ScriptWebBundle> {
  static bool AllowFrom(const SubresourceWebBundle& subresource_web_bundle) {
    return subresource_web_bundle.IsScriptWebBundle();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_BUNDLE_SCRIPT_WEB_BUNDLE_H_
