// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_SUBRESOURCE_LOAD_METRICS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_SUBRESOURCE_LOAD_METRICS_H_

#include "base/time/time.h"

namespace blink {

// Metrics on subresource load handled by service workers.
struct ServiceWorkerSubresourceLoadMetrics {
  // True if image subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool image_handled = false;
  // True if image subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool image_fallback = false;

  // True if CSS style sheet subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool css_handled = false;
  // True if CSS style sheet subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool css_fallback = false;

  // True if script subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool script_handled = false;
  // True if script subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool script_fallback = false;

  // True if font subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool font_handled = false;
  // True if font subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool font_fallback = false;

  // True if raw subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool raw_handled = false;
  // True if raw subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool raw_fallback = false;

  // True if SVG document subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool svg_handled = false;
  // True if SVG document subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool svg_fallback = false;

  // True if XSL style sheet subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool xsl_handled = false;
  // True if XSL style sheet subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool xsl_fallback = false;

  // True if link prefetch subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool link_prefetch_handled = false;
  // True if link prefetch subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool link_prefetch_fallback = false;

  // True if text track subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool text_track_handled = false;
  // True if text track subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool text_track_fallback = false;

  // True if audio subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool audio_handled = false;
  // True if audio subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool audio_fallback = false;

  // True if video subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool video_handled = false;
  // True if video subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool video_fallback = false;

  // True if manifest subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool manifest_handled = false;
  // True if manifest subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool manifest_fallback = false;

  // True if speculation rules subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool speculation_rules_handled = false;
  // True if speculation rules subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool speculation_rules_fallback = false;

  // True if mock subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool mock_handled = false;
  // True if mock subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool mock_fallback = false;

  // True if dictionary subresource is handled by a service worker.
  // i.e. the service worker called `respondWith` to return the resource.
  bool dictionary_handled = false;
  // True if dictionary subresource is not handled by a service worker.
  // i.e. the service worker did not call `respondWith`, and network fallback.
  bool dictionary_fallback = false;

  // Total number of sub-resources which were matched to
  // `RouterSourceEnum.cache` in ServiceWorker Static Routing API, from
  // navigation start until onload event.
  uint32_t matched_network_router_source_count = 0;

  // Total number of sub-resources which were matched to
  // `RouterSourceEnum.fetch-event` in ServiceWorker Static Routing API, from
  // navigation start until onload event.
  uint32_t matched_fetch_event_router_source_count = 0;

  // Total number of sub-resources which were matched to
  // `RouterSourceEnum.network` in ServiceWorker Static Routing API, from
  // navigation start until onload event.
  uint32_t matched_cache_router_source_count = 0;

  // Total number of sub-resources which were matched to the
  // `RouterSourceEnum.race-network-and-fetch-handler` in ServiceWorker Static
  // Routing API, from navigation start till onload event.
  uint32_t matched_race_network_and_fetch_router_source_count = 0;

  // Total router evaluation time of ServiceWorker Static Routing API
  // for sub-resources.
  base::TimeDelta total_router_evaluation_time_for_subresources;

  // Total cache lookup time of ServiceWorker Static Routing API
  // for sub-resources.
  base::TimeDelta total_cache_lookup_time_for_subresources;

  bool operator==(const ServiceWorkerSubresourceLoadMetrics& other) const {
    return image_handled == other.image_handled &&
           image_fallback == other.image_fallback &&
           css_handled == other.css_handled &&
           css_fallback == other.css_fallback &&
           script_handled == other.script_handled &&
           script_fallback == other.script_fallback &&
           font_handled == other.font_handled &&
           font_fallback == other.font_fallback &&
           raw_handled == other.raw_handled &&
           raw_fallback == other.raw_fallback &&
           svg_handled == other.svg_handled &&
           svg_fallback == other.svg_fallback &&
           xsl_handled == other.xsl_handled &&
           xsl_fallback == other.xsl_fallback &&
           link_prefetch_handled == other.link_prefetch_handled &&
           link_prefetch_fallback == other.link_prefetch_fallback &&
           text_track_handled == other.text_track_handled &&
           text_track_fallback == other.text_track_fallback &&
           audio_handled == other.audio_handled &&
           audio_fallback == other.audio_fallback &&
           video_handled == other.video_handled &&
           video_fallback == other.video_fallback &&
           manifest_handled == other.manifest_handled &&
           manifest_fallback == other.manifest_fallback &&
           speculation_rules_handled == other.speculation_rules_handled &&
           speculation_rules_fallback == other.speculation_rules_fallback &&
           mock_handled == other.mock_handled &&
           mock_fallback == other.mock_fallback &&
           dictionary_handled == other.dictionary_handled &&
           dictionary_fallback == other.dictionary_fallback &&
           matched_network_router_source_count ==
               other.matched_network_router_source_count &&
           matched_fetch_event_router_source_count ==
               other.matched_fetch_event_router_source_count &&
           matched_cache_router_source_count ==
               other.matched_cache_router_source_count &&
           matched_race_network_and_fetch_router_source_count ==
               other.matched_race_network_and_fetch_router_source_count &&
           total_router_evaluation_time_for_subresources ==
               other.total_router_evaluation_time_for_subresources &&
           total_cache_lookup_time_for_subresources ==
               other.total_cache_lookup_time_for_subresources;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_SUBRESOURCE_LOAD_METRICS_H_
