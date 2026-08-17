// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_SVG_DOCUMENT_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_SVG_DOCUMENT_RESOURCE_H_

#include "third_party/blink/renderer/core/loader/resource/text_resource.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class AgentGroupScheduler;
class FetchParameters;
class ResourceFetcher;
class SVGResourceDocumentContent;

class SVGDocumentResource final : public TextResource {
 public:
  static SVGDocumentResource* Fetch(FetchParameters&,
                                    ResourceFetcher*,
                                    AgentGroupScheduler&);

  SVGDocumentResource(const ResourceRequest&,
                      const ResourceLoaderOptions&,
                      const TextResourceDecoderOptions&,
                      SVGResourceDocumentContent*);

  void NotifyStartLoad() override;
  void Finish(base::TimeTicks finish_time,
              base::SingleThreadTaskRunner*) override;
  void FinishAsError(const ResourceError&,
                     base::SingleThreadTaskRunner*) override;

  SVGResourceDocumentContent* GetContent() const { return content_.Get(); }

  void Trace(Visitor*) const override;

 private:
  void DestroyDecodedDataForFailedRevalidation() override;

  Member<SVGResourceDocumentContent> content_;
};

template <>
struct DowncastTraits<SVGDocumentResource> {
  static bool AllowFrom(const Resource& resource) {
    return resource.GetType() == ResourceType::kSVGDocument;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_SVG_DOCUMENT_RESOURCE_H_
