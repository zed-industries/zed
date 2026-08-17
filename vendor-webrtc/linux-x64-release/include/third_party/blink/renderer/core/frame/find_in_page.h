// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FIND_IN_PAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FIND_IN_PAGE_H_

#include "build/build_config.h"
#include "mojo/public/cpp/bindings/associated_receiver.h"
#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/frame/find_in_page.mojom-blink.h"
#include "third_party/blink/public/platform/interface_registry.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/web/web_local_frame.h"
#include "third_party/blink/public/web/web_plugin_container.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/finder/text_finder.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace blink {

class WebLocalFrameImpl;

class CORE_EXPORT FindInPage final : public GarbageCollected<FindInPage>,
                                     public mojom::blink::FindInPage {
 public:
  FindInPage(WebLocalFrameImpl& frame, InterfaceRegistry* interface_registry);
  FindInPage(const FindInPage&) = delete;
  FindInPage& operator=(const FindInPage&) = delete;

  bool FindInternal(int identifier,
                    const String& search_text,
                    const mojom::blink::FindOptions&,
                    bool wrap_within_frame,
                    bool* active_now = nullptr);

  // Overrides the tickmarks from the client. Note that these values are in
  // layout space, which means they differ by device scale factor from the
  // CSS space.
  void SetTickmarks(const WebElement& target,
                    const std::vector<gfx::Rect>& tickmarks_in_layout_space);

  int FindMatchMarkersVersion() const;

#if BUILDFLAG(IS_ANDROID)
  // Returns the bounding box of the active find-in-page match marker or an
  // empty rect if no such marker exists. The rect is returned in find-in-page
  // coordinates.
  gfx::RectF ActiveFindMatchRect();
#endif

  void ReportFindInPageMatchCount(int request_id, int count, bool final_update);

  void ReportFindInPageSelection(int request_id,
                                 int active_match_ordinal,
                                 const gfx::Rect& selection_rect,
                                 bool final_update);

  // mojom::blink::FindInPage overrides
  void Find(int request_id,
            const String& search_text,
            mojom::blink::FindOptionsPtr) final;
  void StopFinding(mojom::StopFindAction action) final;
  void ClearActiveFindMatch() final;
  void SetClient(mojo::PendingRemote<mojom::blink::FindInPageClient>) final;
#if BUILDFLAG(IS_ANDROID)
  void GetNearestFindResult(const gfx::PointF&,
                            GetNearestFindResultCallback) final;

  void ActivateNearestFindResult(int request_id, const gfx::PointF&) final;
#endif
#if BUILDFLAG(IS_ANDROID)
  void FindMatchRects(int current_version, FindMatchRectsCallback) final;
#endif

  TextFinder* GetTextFinder() const;

  // Returns the text finder object if it already exists.
  // Otherwise creates it and then returns.
  TextFinder& EnsureTextFinder();

  void SetPluginFindHandler(WebPluginContainer* plugin);

  WebPluginContainer* PluginFindHandler() const;

  WebPlugin* GetWebPluginForFind();

  void BindToReceiver(
      mojo::PendingAssociatedReceiver<mojom::blink::FindInPage> receiver);

  void Dispose();

  void Trace(Visitor* visitor) const {
    visitor->Trace(text_finder_);
    visitor->Trace(frame_);
    visitor->Trace(client_);
    visitor->Trace(receiver_);
  }

 private:
  // Will be initialized after first call to ensureTextFinder().
  Member<TextFinder> text_finder_;

  WebPluginContainer* plugin_find_handler_;

  const Member<WebLocalFrameImpl> frame_;

  HeapMojoRemote<mojom::blink::FindInPageClient> client_{nullptr};

  HeapMojoAssociatedReceiver<mojom::blink::FindInPage, FindInPage> receiver_{
      this, nullptr};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FIND_IN_PAGE_H_
