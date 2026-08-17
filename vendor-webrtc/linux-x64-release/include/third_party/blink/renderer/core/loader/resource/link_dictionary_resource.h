// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_LINK_DICTIONARY_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_LINK_DICTIONARY_RESOURCE_H_

#include "third_party/blink/renderer/platform/loader/fetch/resource.h"

namespace blink {

class FetchParameters;
class ResourceFetcher;
class FeatureContext;

bool CompressionDictionaryTransportFullyEnabled(const FeatureContext*);

// This is the implementation of Resource for <link rel='dictionary'>.
class LinkDictionaryResource final : public Resource {
 public:
  static Resource* Fetch(FetchParameters&, ResourceFetcher*);

  LinkDictionaryResource(const ResourceRequest&, const ResourceLoaderOptions&);
  ~LinkDictionaryResource() override;

 private:
  class Factory final : public NonTextResourceFactory {
   public:
    Factory();

    Resource* Create(const ResourceRequest& request,
                     const ResourceLoaderOptions& options) const override;
  };
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_LINK_DICTIONARY_RESOURCE_H_
