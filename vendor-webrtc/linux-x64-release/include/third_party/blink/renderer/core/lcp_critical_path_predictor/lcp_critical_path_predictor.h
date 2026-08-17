// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LCP_CRITICAL_PATH_PREDICTOR_LCP_CRITICAL_PATH_PREDICTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LCP_CRITICAL_PATH_PREDICTOR_LCP_CRITICAL_PATH_PREDICTOR_H_

#include <optional>

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/mojom/lcp_critical_path_predictor/lcp_critical_path_predictor.mojom-blink.h"
#include "third_party/blink/public/mojom/lcp_critical_path_predictor/lcp_critical_path_predictor.mojom-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/lcp_critical_path_predictor/element_locator.pb.h"
#include "third_party/blink/renderer/core/lcp_critical_path_predictor/lcp_script_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class LocalFrame;
class Element;
enum class ResourceType : uint8_t;

// The LCPCriticalPathPredictor optimizes page load experience by utilizing
// data collected by previous page loads. It sources hint data to various parts
// of Blink to optimize perceived page load speed, and sends the signals
// collected from the current page load to be persisted to the database.
class CORE_EXPORT LCPCriticalPathPredictor final
    : public GarbageCollected<LCPCriticalPathPredictor> {
 public:
  explicit LCPCriticalPathPredictor(LocalFrame& frame);
  ~LCPCriticalPathPredictor();

  LCPCriticalPathPredictor(const LCPCriticalPathPredictor&) = delete;
  LCPCriticalPathPredictor& operator=(const LCPCriticalPathPredictor&) = delete;

  // Member functions invoked in LCPP hint consumption path (read path):

  // Checks if `this` has some hint data available for the page load.
  // Meant to be used as preconditions on metrics.
  bool HasAnyHintData() const;

  void set_lcp_element_locators(
      const std::vector<std::string>& lcp_element_locator_strings);

  const Vector<ElementLocator>& lcp_element_locators() {
    return lcp_element_locators_;
  }

  bool IsElementMatchingLocator(const Element& element);

  void set_lcp_influencer_scripts(HashSet<KURL> scripts);

  const HashSet<KURL>& lcp_influencer_scripts() {
    return lcp_influencer_scripts_;
  }

  void set_fetched_fonts(Vector<KURL> fonts);

  void set_preconnected_origins(const Vector<url::Origin>& origins);

  void set_unused_preloads(Vector<KURL> preloads);

  const Vector<KURL>& fetched_fonts() { return fetched_fonts_; }

  const Vector<KURL>& unused_preloads() { return unused_preloads_; }

  void enable_testing();

  void Reset();

  bool IsLcpInfluencerScript(const KURL& url);

  // Member functions invoked in LCPP hint production path (write path):

  void OnLargestContentfulPaintUpdated(
      const Element& lcp_element,
      std::optional<const KURL> maybe_image_url);
  void OnFontFetched(const KURL& url);
  void OnStartPreload(const KURL& url, const ResourceType& resource_type);
  void OnOutermostMainFrameDocumentLoad();
  void OnWarnedUnusedPreloads(const Vector<KURL>& unused_preloads);

  using LCPCallback = base::OnceCallback<void(const Element*)>;
  void AddLCPPredictedCallback(LCPCallback callback);
  void AddLCPPredictedCallback(base::OnceClosure);

  void Trace(Visitor*) const;

 private:
  LocalFrame& GetFrame() { return *frame_.Get(); }
  mojom::blink::LCPCriticalPathPredictorHost& GetHost();
  void MayRunPredictedCallbacks(const Element* lcp_element);

  Member<LocalFrame> frame_;
  HeapMojoRemote<mojom::blink::LCPCriticalPathPredictorHost> host_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // LCPP hints for consumption (read path):

  Vector<ElementLocator> lcp_element_locators_;
  Vector<std::string> lcp_element_locator_strings_;
  HashSet<KURL> lcp_influencer_scripts_;
  Vector<KURL> fetched_fonts_;
  Vector<url::Origin> preconnected_origins_;
  Vector<KURL> unused_preloads_;

  // Callbacks are called when predicted LCP is painted. Never called if
  // prediction is incorrect.
  Vector<LCPCallback> lcp_predicted_callbacks_;
  bool are_predicted_callbacks_called_ = false;
  bool has_lcp_occurred_ = false;
  bool is_outermost_main_frame_document_loaded_ = false;
  bool has_sent_unused_preloads_ = false;

  bool report_timing_predictor_for_testing_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LCP_CRITICAL_PATH_PREDICTOR_LCP_CRITICAL_PATH_PREDICTOR_H_
