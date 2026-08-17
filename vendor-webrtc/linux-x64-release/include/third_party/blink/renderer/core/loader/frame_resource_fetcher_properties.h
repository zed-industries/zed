// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FRAME_RESOURCE_FETCHER_PROPERTIES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FRAME_RESOURCE_FETCHER_PROPERTIES_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/loader_freeze_mode.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_fetcher_properties.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class Document;
class DocumentLoader;

// FrameResourceFetcherProperties is a ResourceFetcherProperties implementation
// for Frame.
class CORE_EXPORT FrameResourceFetcherProperties final
    : public ResourceFetcherProperties {
 public:
  FrameResourceFetcherProperties(DocumentLoader& document_loader,
                                 Document& document);
  ~FrameResourceFetcherProperties() override = default;

  void Trace(Visitor*) const override;

  // ResourceFetcherProperties implementation
  const FetchClientSettingsObject& GetFetchClientSettingsObject()
      const override {
    return *fetch_client_settings_object_;
  }
  bool IsOutermostMainFrame() const override;
  ControllerServiceWorkerMode GetControllerServiceWorkerMode() const override;
  int64_t ServiceWorkerId() const override;
  bool IsPaused() const override;
  LoaderFreezeMode FreezeMode() const override;
  bool IsDetached() const override { return false; }
  bool IsLoadComplete() const override;
  bool ShouldBlockLoadingSubResource() const override;
  bool IsSubframeDeprioritizationEnabled() const override;
  scheduler::FrameStatus GetFrameStatus() const override;
  int GetOutstandingThrottledLimit() const override;

 private:
  const Member<DocumentLoader> document_loader_;
  const Member<Document> document_;
  Member<const FetchClientSettingsObject> fetch_client_settings_object_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_FRAME_RESOURCE_FETCHER_PROPERTIES_H_
