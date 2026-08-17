// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_LOADING_BEHAVIOR_FLAG_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_LOADING_BEHAVIOR_FLAG_H_

namespace blink {

// This enum tracks certain behavior Blink exhibits when loading a page. This is
// for use in metrics collection by the loading team, to evaluate experimental
// features and potential areas of improvement in the loading stack. The main
// consumer is the page_load_metrics component, which sends bit flags to the
// browser process for histogram splitting.
enum LoadingBehaviorFlag {
  kLoadingBehaviorNone = 0,
  // Indicates that the page used the document.write evaluator to preload scan
  // for resources inserted via document.write.
  // DEPRECATED, feature has been turned down.
  kLoadingBehaviorDocumentWriteEvaluator = 1 << 0,
  // Indicates that the page is controlled by a Service Worker.
  kLoadingBehaviorServiceWorkerControlled = 1 << 1,
  // Indicates that the page has a synchronous, cross-origin document.written
  // script.
  kLoadingBehaviorDocumentWriteBlock = 1 << 2,
  // Indicates that the page is a reload and has a synchronous, cross-origin
  // document.written script.
  // DEPRECATED, metrics using this have been removed.
  kLoadingBehaviorDocumentWriteBlockReload = 1 << 3,
  // The page loaded external CSS that generated a PreloadRequest via the
  // CSSPreloaderResourceClient.
  kLoadingBehaviorCSSPreloadFound = 1 << 4,
  // Indicates that the page has a synchronous, same-origin document.written
  // script with different protocol.
  // DEPRECATED, metrics using this have been removed.
  kLoadingBehaviorDocumentWriteBlockDifferentScheme = 1 << 5,
  // Indicates that a subresource on the page matched the subresource filtering
  // rules.
  kLoadingBehaviorSubresourceFilterMatch = 1 << 6,
  // Indicates that the page is an AMP document, with <html amp> tag.
  kLoadingBehaviorAmpDocumentLoaded = 1 << 7,
  // Indicates that an async script was ready to execute before the script
  // element's node document has finished parsing.
  kLoadingBehaviorAsyncScriptReadyBeforeDocumentFinishedParsing = 1 << 8,
  // Indicates that competing low priority requests were delayed. See
  // https://crbug.com/1112515 for details.
  kLoadingBehaviorCompetingLowPriorityRequestsDelayed = 1 << 9,
  // Indicates that the page is controlled by a Service Worker, but
  // the fetch handler is considered skippable.
  kLoadingBehaviorServiceWorkerFetchHandlerSkippable = 1 << 10,
  // Indicates that the main resource fetch for the page controlled by
  // a service worker at the navigation time fallback to network.
  kLoadingBehaviorServiceWorkerMainResourceFetchFallback = 1 << 11,
  // Indicates that the RaceNetworkRequest is dispatched for the main and
  // subresources. crbug.com/1420517 for more details.
  kLoadingBehaviorServiceWorkerRaceNetworkRequest = 1 << 12,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_LOADING_BEHAVIOR_FLAG_H_
