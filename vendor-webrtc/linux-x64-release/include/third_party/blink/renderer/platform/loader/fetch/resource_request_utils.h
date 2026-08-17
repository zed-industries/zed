// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_REQUEST_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_REQUEST_UTILS_H_

#include <optional>

#include "third_party/blink/public/platform/resource_request_blocked_reason.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_client_settings_object.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_context.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_parameters.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource.h"

namespace blink {

// This method simply takes in information about a ResourceRequest, and returns
// if the resource should be loaded in parallel (incremental) or sequentially
// for protocols that support multiplexing and HTTP extensible priorities
// (RFC 9218).
// Most content types can be operated on with partial data (document parsing,
// images, media, etc) but a few need to be complete before they can be
// processed.
bool BLINK_PLATFORM_EXPORT ShouldLoadIncremental(ResourceType type);

void SetReferrer(ResourceRequest& request,
                 const FetchClientSettingsObject& fetch_client_settings_object);

// Returns the adjusted version of `priority` according to the given
// `fetch_priority_hint` and `render_blocking_behavior`.
ResourceLoadPriority BLINK_PLATFORM_EXPORT
AdjustPriorityWithPriorityHintAndRenderBlocking(
    ResourceLoadPriority priority,
    ResourceType type,
    mojom::blink::FetchPriorityHint fetch_priority_hint,
    RenderBlockingBehavior render_blocking_behavior);

// Used by PrepareResourceRequestForCacheAccess() and
// UpgradeResourceRequestForLoader().
class ResourceRequestContext {
 public:
  // Computes the ResourceLoadPriority. This is called if the priority was not
  // set.
  virtual ResourceLoadPriority ComputeLoadPriority(
      const FetchParameters& params) = 0;

  // Called to record a trace.
  virtual void RecordTrace() = 0;

 protected:
  virtual ~ResourceRequestContext() = default;
};

// Prepares the underlying ResourceRequest for `params` with enough information
// to do a cache lookup. If a cached value is not used,
// UpgradeResourceRequestForLoader() must be called.
//
// `bundle_url_for_uuid_resources` is an optional bundle URL for
// uuid-in-package: resources for security checks. Should only be set when the
// request is WebBundle.
//
// Returns std::nullopt if loading the ResourceRequest in `params` is not
// blocked. Otherwise, returns a blocked reason.
// This method may modify the ResourceRequest in `params` according to
// `context` and `resource_type`.
BLINK_PLATFORM_EXPORT std::optional<ResourceRequestBlockedReason>
PrepareResourceRequestForCacheAccess(
    ResourceType type,
    const FetchClientSettingsObject& fetch_client_settings_object,
    const KURL& bundle_url_for_uuid_resources,
    ResourceRequestContext& resource_request_context,
    FetchContext& context,
    FetchParameters& params);

BLINK_PLATFORM_EXPORT void UpgradeResourceRequestForLoader(
    ResourceType resource_type,
    FetchParameters& params,
    FetchContext& context,
    ResourceRequestContext& resource_request_context,
    WebScopedVirtualTimePauser& virtual_time_pauser);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_REQUEST_UTILS_H_
