// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_PRELOAD_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_PRELOAD_HELPER_H_

#include <optional>

#include "third_party/blink/renderer/platform/loader/fetch/resource.h"

namespace blink {

class AlternateSignedExchangeResourceInfo;
class Document;
class PendingLinkPreload;
class LocalFrame;
struct LinkLoadParameters;
struct ViewportDescription;

// PreloadHelper is a helper class for preload, module preload, prefetch,
// DNS prefetch, and preconnect triggered by <link> elements and "Link" HTTP
// response headers.
class PreloadHelper final {
  STATIC_ONLY(PreloadHelper);

 public:
  enum class LoadLinksFromHeaderMode {
    kDocumentBeforeCommit,
    kDocumentAfterCommitWithoutViewport,
    kDocumentAfterCommitWithViewport,
    kDocumentAfterLoadCompleted,
    kSubresourceFromMemoryCache,
    kSubresourceNotFromMemoryCache,
  };

  static void LoadLinksFromHeader(
      const String& header_value,
      const KURL& base_url,
      LocalFrame&,
      Document*,  // can be nullptr
      LoadLinksFromHeaderMode,
      const ViewportDescription*,  // can be nullptr
      std::unique_ptr<AlternateSignedExchangeResourceInfo>,
      const base::UnguessableToken*
          recursive_prefetch_token /* can be nullptr */);
  static Resource* StartPreload(ResourceType, FetchParameters&, Document&);

  // Currently only used for UseCounter.
  enum LinkCaller {
    kLinkCalledFromHeader,
    kLinkCalledFromMarkup,
  };

  static void DnsPrefetchIfNeeded(const LinkLoadParameters&,
                                  Document*,
                                  LocalFrame*,
                                  LinkCaller);
  static void PreconnectIfNeeded(const LinkLoadParameters&,
                                 Document*,
                                 LocalFrame*,
                                 LinkCaller);
  static void PrefetchIfNeeded(const LinkLoadParameters&,
                               Document&,
                               PendingLinkPreload*);
  static void PreloadIfNeeded(const LinkLoadParameters&,
                              Document&,
                              const KURL& base_url,
                              LinkCaller,
                              const ViewportDescription*,
                              ParserDisposition,
                              PendingLinkPreload*);
  static void ModulePreloadIfNeeded(const LinkLoadParameters&,
                                    Document&,
                                    const ViewportDescription*,
                                    PendingLinkPreload*);
  static void FetchCompressionDictionaryIfNeeded(const LinkLoadParameters&,
                                                 Document&,
                                                 PendingLinkPreload*);

  static std::optional<ResourceType> GetResourceTypeFromAsAttribute(
      const String& as);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_PRELOAD_HELPER_H_
