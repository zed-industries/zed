// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_SPECULATION_RULES_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_SPECULATION_RULES_RESOURCE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/resource/text_resource.h"

namespace blink {

class FetchParameters;
class ResourceFetcher;

// This is the implementation of Resource for the Speculation-Rules header.
class CORE_EXPORT SpeculationRulesResource final : public TextResource {
 public:
  static SpeculationRulesResource* Fetch(FetchParameters&, ResourceFetcher*);

  SpeculationRulesResource(const ResourceRequest&,
                           const ResourceLoaderOptions&);
  ~SpeculationRulesResource() override;

 private:
  class Factory final : public ResourceFactory {
   public:
    Factory();

    Resource* Create(
        const ResourceRequest& request,
        const ResourceLoaderOptions& options,
        const TextResourceDecoderOptions& decoder_options) const override;
  };
};

template <>
struct DowncastTraits<SpeculationRulesResource> {
  static bool AllowFrom(const Resource& resource) {
    return resource.GetType() == ResourceType::kSpeculationRules;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_SPECULATION_RULES_RESOURCE_H_
