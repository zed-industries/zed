/*
    Copyright (C) 1998 Lars Knoll (knoll@mpi-hd.mpg.de)
    Copyright (C) 2001 Dirk Mueller <mueller@kde.org>
    Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc. All
    rights reserved.
    Copyright (C) 2009 Torch Mobile Inc. http://www.torchmobile.com/

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.

    This class provides all functionality needed for loading images, style
    sheets and html pages from the web. It has a memory cache for these objects.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_FETCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_FETCHER_H_

#include <memory>
#include <optional>
#include <string_view>
#include <utility>

#include "base/task/single_thread_task_runner.h"
#include "base/unguessable_token.h"
#include "services/metrics/public/cpp/mojo_ukm_recorder.h"
#include "third_party/blink/public/common/features.h"
#include "third_party/blink/public/common/subresource_load_metrics.h"
#include "third_party/blink/public/mojom/blob/blob_registry.mojom-blink.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/service_worker/controller_service_worker_mode.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/loader/fetch/early_hints_preload_entry.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_parameters.h"
#include "third_party/blink/renderer/platform/loader/fetch/loader_freeze_mode.h"
#include "third_party/blink/renderer/platform/loader/fetch/preload_key.h"
#include "third_party/blink/renderer/platform/loader/fetch/render_blocking_behavior.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_priority.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_scheduler.h"
#include "third_party/blink/renderer/platform/loader/fetch/service_worker_router_info.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/url_loader.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/mojo/mojo_binding_context.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace network {
struct ResourceRequest;
}  // namespace network

namespace blink {

enum class ResourceType : uint8_t;
class BackForwardCacheLoaderHelper;
class CodeCacheHost;
class DetachableConsoleLogger;
class DetachableUseCounter;
class DetachableResourceFetcherProperties;
class FetchContext;
class FrameOrWorkerScheduler;
class MHTMLArchive;
class KURL;
class Resource;
class ResourceError;
class ResourceLoadObserver;
class SubresourceWebBundle;
class SubresourceWebBundleList;
struct ResourceFetcherInit;
struct ResourceLoaderOptions;

// The ResourceFetcher provides a per-context interface to the MemoryCache and
// enforces a bunch of security checks and rules for resource revalidation. Its
// lifetime is roughly per-DocumentLoader, in that it is generally created in
// the DocumentLoader constructor and loses its ability to generate network
// requests when the DocumentLoader is destroyed.
//
// It is also created for workers and worklets.
//
// Documents also hold a pointer to ResourceFetcher for their lifetime (and will
// create one if they are initialized without a LocalFrame), so a Document can
// keep a ResourceFetcher alive past detach if scripts still reference the
// Document.
class PLATFORM_EXPORT ResourceFetcher
    : public GarbageCollected<ResourceFetcher>,
      public MemoryPressureListener {
  USING_PRE_FINALIZER(ResourceFetcher, ClearPreloads);

 public:
  // An abstract interface for creating loaders.
  class LoaderFactory : public GarbageCollected<LoaderFactory> {
   public:
    virtual ~LoaderFactory() = default;

    virtual void Trace(Visitor*) const {}

    // Create a URLLoader for given the request information and task runners.
    // TODO(yuzus): Take only unfreezable task runner once both
    // URLLoaderClientImpl and ResponseBodyLoader use unfreezable task runner.
    virtual std::unique_ptr<URLLoader> CreateURLLoader(
        const network::ResourceRequest&,
        const ResourceLoaderOptions&,
        scoped_refptr<base::SingleThreadTaskRunner> freezable_task_runner,
        scoped_refptr<base::SingleThreadTaskRunner> unfreezable_task_runner,
        BackForwardCacheLoaderHelper*,
        const std::optional<base::UnguessableToken>&
            service_worker_race_network_request_token,
        bool is_from_origin_dirty_style_sheet) = 0;

    // Get a code cache host to fetch data from code caches.
    virtual CodeCacheHost* GetCodeCacheHost() = 0;
  };

  // ResourceFetcher creators are responsible for setting consistent objects
  // in ResourceFetcherInit to ensure correctness of this ResourceFetcher.
  explicit ResourceFetcher(const ResourceFetcherInit&);
  ResourceFetcher(const ResourceFetcher&) = delete;
  ResourceFetcher& operator=(const ResourceFetcher&) = delete;
  ~ResourceFetcher() override;
  void Trace(Visitor*) const override;

  // - This function returns the same object throughout this fetcher's
  //   entire life.
  // - The returned object remains valid after ClearContext() is called.
  const DetachableResourceFetcherProperties& GetProperties() const {
    return *properties_;
  }

  // Returns true if optimizations for loading 1x1 transparent placeholder
  // images is enabled.
  bool IsSimplifyLoadingTransparentPlaceholderImageEnabled();

  // Returns whether this fetcher is detached from the associated context.
  bool IsDetached() const;

  // Returns whether RequestResource or EmulateLoadStartedForInspector are being
  // called.
  bool IsInRequestResource() const { return is_in_request_resource_; }

  // Returns the observer object associated with this fetcher.
  ResourceLoadObserver* GetResourceLoadObserver() {
    // When detached, we must have a null observer.
    DCHECK(!IsDetached() || !resource_load_observer_);
    return resource_load_observer_.Get();
  }
  // This must be called right after construction.
  void SetResourceLoadObserver(ResourceLoadObserver* observer) {
    DCHECK(!IsDetached());
    DCHECK(!resource_load_observer_);
    resource_load_observer_ = observer;
  }

  // Triggers or defers a fetch based on the given FetchParameters (if there
  // isn't a suitable Resource already cached) and registers the given
  // ResourceClient with the Resource. Guaranteed to return a non-null Resource
  // of the subtype specified by ResourceFactory::GetType().
  Resource* RequestResource(FetchParameters&,
                            const ResourceFactory&,
                            ResourceClient*);

  // Returns the task runner used by this fetcher, and loading operations
  // this fetcher initiates. The returned task runner will keep working even
  // after ClearContext is called.
  const scoped_refptr<base::SingleThreadTaskRunner>& GetTaskRunner() const {
    return freezable_task_runner_;
  }

  // Create a loader. This cannot be called after ClearContext is called.
  std::unique_ptr<URLLoader> CreateURLLoader(
      const network::ResourceRequest&,
      const ResourceLoaderOptions&,
      const mojom::blink::RequestContextType,
      const RenderBlockingBehavior,
      const std::optional<base::UnguessableToken>&
          service_worker_race_network_request_token,
      bool is_from_origin_dirty_style_sheet);
  // Get a code cache host. This cannot be called after ClearContext is called.
  CodeCacheHost* GetCodeCacheHost();

  Resource* CachedResource(const KURL&) const;
  bool ResourceHasBeenEmulatedLoadStartedForInspector(const KURL&) const;

  // Registers an callback to be called with the resource priority of the fetch
  // made to the specified URL. When `new_load_only` is set to false,
  // this will also search for Resources alive in Oilpan heap, and their
  // fetched priority.
  void AddPriorityObserverForTesting(const KURL&,
                                     base::OnceCallback<void(int)>,
                                     bool new_load_only = false);

  using DocumentResourceMap = HeapHashMap<String, WeakMember<Resource>>;
  // Note: This function is defined for devtools. Do not use this function in
  // non-inspector/non-tent-only contexts.
  const DocumentResourceMap& AllResources() const {
    return cached_resources_map_;
  }

  const HeapHashSet<Member<Resource>> MoveResourceStrongReferences();
  bool HasStrongReferenceForTesting(Resource* resource);

  enum class ImageLoadBlockingPolicy {
    kDefault,
    kForceNonBlockingLoad,
  };

  // Binds the given Resource instance to this ResourceFetcher instance to
  // start loading the Resource actually.
  // Usually, RequestResource() calls this method internally, but needs to
  // call this method explicitly on cases such as ResourceNeedsLoad() returning
  // false.
  bool StartLoad(Resource*, bool is_potentially_unused_preload = false);

  void SetAutoLoadImages(bool);

  FetchContext& Context() const;
  void ClearContext();
  DetachableUseCounter& GetUseCounter() { return *use_counter_; }
  DetachableConsoleLogger& GetConsoleLogger() { return *console_logger_; }

  int BlockingRequestCount() const;
  int NonblockingRequestCount() const;
  int ActiveRequestCount() const;

  enum ClearPreloadsPolicy {
    kClearAllPreloads,
    kClearSpeculativeMarkupPreloads
  };

  void EnableIsPreloadedForTest();
  bool IsPreloadedForTest(const KURL&) const;

  int CountPreloads() const { return preloads_.size(); }
  void ClearPreloads(ClearPreloadsPolicy = kClearAllPreloads);
  void ScheduleWarnUnusedPreloads(
      base::OnceCallback<void(Vector<KURL> unused_preloads)> callback);

  MHTMLArchive* Archive() const { return archive_.Get(); }

  // Set the deferring state of each loader owned by this ResourceFetcher. This
  // method must be called when the page freezing state changes.
  // TODO(yhirano): Rename this to a more easily recognizable name.
  void SetDefersLoading(LoaderFreezeMode);

  void StopFetching();

  bool ShouldDeferImageLoad(const KURL&) const;

  void RecordResourceTimingOnRedirect(Resource*,
                                      const ResourceResponse&,
                                      const KURL& new_url);

  enum LoaderFinishType { kDidFinishLoading, kDidFinishFirstPartInMultipart };
  void HandleLoaderFinish(Resource*,
                          base::TimeTicks finish_time,
                          LoaderFinishType,
                          uint32_t inflight_keepalive_bytes);
  void HandleLoaderError(Resource*,
                         base::TimeTicks finish_time,
                         const ResourceError&,
                         uint32_t inflight_keepalive_bytes);
  blink::mojom::ControllerServiceWorkerMode IsControlledByServiceWorker() const;

  // Returns a cache identifier for MemoryCache.
  // `url` is used for finding a matching WebBundle.
  // If `skip_service_worker` is true, the identifier won't be a ServiceWorker's
  // identifier to keep the cache separated.
  String GetCacheIdentifier(const KURL& url, bool skip_service_worker) const;

  // If `url` exists as a resource in a subresource bundle in this frame,
  // returns its UnguessableToken; otherwise, returns std::nullopt.
  std::optional<base::UnguessableToken> GetSubresourceBundleToken(
      const KURL& url) const;

  std::optional<KURL> GetSubresourceBundleSourceUrl(const KURL& url) const;

  enum IsImageSet { kImageNotImageSet, kImageIsImageSet };

  [[nodiscard]] static mojom::blink::RequestContextType DetermineRequestContext(
      ResourceType,
      IsImageSet);

  static network::mojom::RequestDestination DetermineRequestDestination(
      ResourceType);

  void UpdateImagePrioritiesAndSpeculativeDecodes();

  // Returns whether the given resource is contained as a preloaded resource.
  bool ContainsAsPreload(Resource*) const;

  void RemovePreload(Resource*);

  void LoosenLoadThrottlingPolicy() { scheduler_->LoosenThrottlingPolicy(); }

  // Workaround for https://crbug.com/666214.
  // TODO(hiroshige): Remove this hack.
  void EmulateLoadStartedForInspector(Resource*,
                                      mojom::blink::RequestContextType,
                                      network::mojom::RequestDestination,
                                      const AtomicString& initiator_name);

  // This is called from leak detectors (Real-world leak detector & web test
  // leak detector) to clean up loaders after page navigation before instance
  // counting.
  void PrepareForLeakDetection();

  using ResourceFetcherSet = GCedHeapHashSet<WeakMember<ResourceFetcher>>;
  static const ResourceFetcherSet& MainThreadFetchers();

  mojom::blink::BlobRegistry* GetBlobRegistry();

  FrameOrWorkerScheduler* GetFrameOrWorkerScheduler();

  ResourceLoadPriority ComputeLoadPriorityForTesting(
      ResourceType type,
      const ResourceRequestHead& request,
      ResourcePriority::VisibilityStatus visibility_statue,
      FetchParameters::DeferOption defer_option,
      FetchParameters::SpeculativePreloadType speculative_preload_type,
      RenderBlockingBehavior render_blocking_behavior,
      mojom::blink::ScriptType script_type,
      bool is_link_preload,
      const std::optional<float> resource_width = std::nullopt,
      const std::optional<float> resource_height = std::nullopt,
      bool is_potentially_lcp_element = false,
      bool is_potentially_lcp_influencer = false) {
    return ComputeLoadPriority(
        type, request, visibility_statue, defer_option,
        speculative_preload_type, render_blocking_behavior, script_type,
        is_link_preload, resource_width, resource_height,
        is_potentially_lcp_element, is_potentially_lcp_influencer);
  }

  void SetThrottleOptionOverride(
      ResourceLoadScheduler::ThrottleOptionOverride throttle_option_override) {
    scheduler_->SetThrottleOptionOverride(throttle_option_override);
  }

  SubresourceWebBundleList* GetOrCreateSubresourceWebBundleList();

  BackForwardCacheLoaderHelper* GetBackForwardCacheLoaderHelper() {
    return back_forward_cache_loader_helper_.Get();
  }

  void SetEarlyHintsPreloadedResources(
      HashMap<KURL, EarlyHintsPreloadEntry> resources) {
    unused_early_hints_preloaded_resources_ = std::move(resources);
  }

  // Access the UKMRecorder.
  ukm::MojoUkmRecorder* UkmRecorder();

  void CancelWebBundleSubresourceLoadersFor(
      const base::UnguessableToken& web_bundle_token);

  void OnMemoryPressure(
      base::MemoryPressureListener::MemoryPressureLevel) override;

  void MaybeRecordLCPPSubresourceMetrics(const KURL& document_url);

  // For every image resource that was deferred, check to see if state has
  // changed such that the load should no longer be deferred.
  void ReloadImagesIfNotDeferred();

  // Populates the provided request's permissions policy.
  void PopulateResourceRequestPermissionsPolicy(
      network::ResourceRequest* request);

  // Check if a resource is preloaded by earlyhints when response received.
  void MarkEarlyHintConsumedIfNeeded(uint64_t inspector_id,
                                     Resource* resource,
                                     const ResourceResponse& response);

  void EnableDeferUnusedPreloadForTesting() {
    defer_unused_preload_enabled_for_testing_ = true;
  }

  using LcppDeferUnusedPreloadPreloadedReason =
      features::LcppDeferUnusedPreloadPreloadedReason;
  void SetDeferUnusedPreloadPreloadedReasonForTesting(
      LcppDeferUnusedPreloadPreloadedReason reason) {
    defer_unused_preload_preloaded_reason_for_testing_ = reason;
  }

  using LcppDeferUnusedPreloadExcludedResourceType =
      features::LcppDeferUnusedPreloadExcludedResourceType;
  void SetDeferUnusedPreloadExcludedResourceType(
      LcppDeferUnusedPreloadExcludedResourceType excluded_resource_type) {
    defer_unused_preload_excluded_resource_type_for_testing_ =
        excluded_resource_type;
  }

 private:
  friend class ResourceCacheValidationSuppressor;
  class ResourcePrepareHelper;

  enum class StopFetchingTarget {
    kExcludingKeepaliveLoaders,
    kIncludingKeepaliveLoaders,
  };

  enum class DeferPolicy {
    kNoDefer,
    // kDefer doesn't start loading in `ResourceRequest()`. This option is used
    // to defer a non-link preload font, or image loading is disabled.
    kDefer,
    // kDeferAndSchedule doesn't start loading immediately in
    // `ResourceRequest()`, but schedule the loading task instead. This option
    // is used by the LCPP feature, DeferUnusedPreload.
    kDeferAndSchedule,
  };

  bool StartLoad(Resource*,
                 ResourceRequestBody,
                 ImageLoadBlockingPolicy,
                 RenderBlockingBehavior);

  void InitializeRevalidation(ResourceRequest&, Resource*);
  void AddToMemoryCacheIfNeeded(const FetchParameters&, Resource*);
  Resource* CreateResourceForLoading(const FetchParameters&,
                                     const ResourceFactory&);
  void StorePerformanceTimingInitiatorInformation(Resource*,
                                                  RenderBlockingBehavior);
  ResourceLoadPriority ComputeLoadPriority(
      ResourceType,
      const ResourceRequestHead&,
      ResourcePriority::VisibilityStatus,
      FetchParameters::DeferOption = FetchParameters::DeferOption::kNoDefer,
      FetchParameters::SpeculativePreloadType =
          FetchParameters::SpeculativePreloadType::kNotSpeculative,
      RenderBlockingBehavior = RenderBlockingBehavior::kNonBlocking,
      mojom::blink::ScriptType script_type = mojom::blink::ScriptType::kClassic,
      bool is_link_preload = false,
      const std::optional<float> resource_width = std::nullopt,
      const std::optional<float> resource_height = std::nullopt,
      bool is_potentially_lcp_element = false,
      bool is_potentially_lcp_influencer = false);
  ResourceLoadPriority AdjustImagePriority(
      const ResourceLoadPriority priority_so_far,
      const ResourceType type,
      const ResourceRequestHead& resource_request,
      const FetchParameters::SpeculativePreloadType speculative_preload_type,
      const bool is_link_preload,
      const std::optional<float> resource_width,
      const std::optional<float> resource_height,
      const bool is_potentially_lcp_element);

  std::optional<ResourceRequestBlockedReason>
  UpdateRequestForTransparentPlaceholderImage(FetchParameters& params);

  Resource* CreateResourceForStaticData(const FetchParameters&,
                                        const ResourceFactory&);
  Resource* ResourceForBlockedRequest(const FetchParameters&,
                                      const ResourceFactory&,
                                      ResourceRequestBlockedReason,
                                      ResourceClient*);

  Resource* MatchPreload(
      const FetchParameters& params,
      ResourceType,
      HeapHashMap<PreloadKey, Member<Resource>>::iterator it);
  void PrintPreloadMismatch(Resource*, Resource::MatchStatus);
  void InsertAsPreloadIfNecessary(Resource*,
                                  const FetchParameters& params,
                                  ResourceType);

  bool IsImageResourceDisallowedToBeReused(const Resource&) const;

  void StopFetchingInternal(StopFetchingTarget);
  void StopFetchingIncludingKeepaliveLoaders();

  void MaybeSaveResourceToStrongReference(Resource* resource);

  void MaybeStartSpeculativeImageDecode();
  void SpeculativeImageDecodeFinished();

  enum class RevalidationPolicy {
    kUse,
    kRevalidate,
    kReload,
    kLoad,
    kMaxValue = kLoad,
  };
  // The Blink.MemoryCache.RevalationPolicy UMA uses the following enum
  // rather than RevalidationPolicy to record the deferred resources in
  // the resource fetcher.
  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum class RevalidationPolicyForMetrics {
    kUse,
    kRevalidate,
    kReload,
    kLoad,
    kDefer,
    kPreviouslyDeferredLoad,
    kMaxValue = kPreviouslyDeferredLoad,
  };

  // Friends required for accessing RevalidationPolicyForMetrics
  FRIEND_TEST(ImageResourceCounterTest, RevalidationPolicyMetrics);
  FRIEND_TEST(FontResourceTest, RevalidationPolicyMetrics);

  // A wrapper just for placing a trace_event macro.
  RevalidationPolicy DetermineRevalidationPolicy(
      ResourceType,
      const FetchParameters&,
      const Resource& existing_resource,
      bool is_static_data) const;
  static const char* GetNameFor(RevalidationPolicy policy);
  // Determines a RevalidationPolicy given a FetchParameters and an existing
  // resource retrieved from the memory cache (can be a newly constructed one
  // for a static data).
  // The first member of the returned value is the revalidation policy. The
  // second member of the returned value is a string explaining the reason for
  // trace events. Its extent is unbound.
  std::pair<RevalidationPolicy, const char*>
  DetermineRevalidationPolicyInternal(ResourceType,
                                      const FetchParameters&,
                                      const Resource& existing_resource,
                                      bool is_static_data) const;

  void MakePreloadedResourceBlockOnloadIfNeeded(Resource*,
                                                const FetchParameters&);
  void MoveResourceLoaderToNonBlocking(ResourceLoader*);
  void RemoveResourceLoader(ResourceLoader*);

  void DidLoadResourceFromMemoryCache(Resource*,
                                      const ResourceRequest&,
                                      bool is_static_data,
                                      RenderBlockingBehavior);

  DeferPolicy GetDeferPolicy(ResourceType, const FetchParameters& params) const;

  bool ResourceNeedsLoad(Resource*, RevalidationPolicy, DeferPolicy) const;

  static bool ResourceAlreadyLoadStarted(Resource*, RevalidationPolicy);

  void ResourceTimingReportTimerFired(TimerBase*);

  static RevalidationPolicyForMetrics MapToPolicyForMetrics(RevalidationPolicy,
                                                            Resource*,
                                                            DeferPolicy);

  void UpdateMemoryCacheStats(Resource*,
                              RevalidationPolicyForMetrics,
                              const FetchParameters&,
                              const ResourceFactory&,
                              bool is_static_data) const;

  void ScheduleStaleRevalidate(Resource* stale_resource);
  void RevalidateStaleResource(Resource* stale_resource);

  void WarnUnusedPreloads(
      base::OnceCallback<void(Vector<KURL> unused_preloads)> callback);

  void RemoveResourceStrongReference(Resource* resource);

  KURL PrepareRequestForWebBundle(ResourceRequest& resource_request) const;

  void AttachWebBundleTokenIfNeeded(ResourceRequest&) const;

  // Information about a resource fetch that had started but not completed yet.
  // Would be added to the response data when the response arrives.
  struct PendingResourceTimingInfo {
    base::TimeTicks start_time;
    AtomicString initiator_type;
    RenderBlockingBehavior render_blocking_behavior;
    base::TimeTicks redirect_end_time;
    bool is_null() const { return start_time.is_null(); }
  };

  // A resource fetch that was completed, scheduled to be added to the
  // performance timeline in a batch.
  struct ScheduledResourceTimingInfo {
    mojom::blink::ResourceTimingInfoPtr info;
    AtomicString initiator_type;
  };

  void PopulateAndAddResourceTimingInfo(Resource* resource,
                                        const PendingResourceTimingInfo& info,
                                        base::TimeTicks response_end);
  SubresourceWebBundle* GetMatchingBundle(const KURL& url) const;
  void UpdateServiceWorkerSubresourceMetrics(
      ResourceType resource_type,
      bool handled_by_serviceworker,
      const blink::ServiceWorkerRouterInfo* router_info);

  void RecordResourceHistogram(std::string_view prefix,
                               ResourceType type,
                               RevalidationPolicyForMetrics policy) const;

  void ScheduleLoadingPotentiallyUnusedPreload(Resource*);
  void StartLoadAndFinishIfFailed(Resource*,
                                  bool is_potentially_unused_preload);
  void ScheduleStartLoadAndFinishIfFailed(Resource* resource,
                                          bool is_potentially_unused_preload);

  bool IsPotentiallyUnusedPreload(ResourceType type,
                                  const FetchParameters& params) const;

  Member<DetachableResourceFetcherProperties> properties_;
  Member<ResourceLoadObserver> resource_load_observer_;
  Member<FetchContext> context_;
  scoped_refptr<base::SingleThreadTaskRunner> freezable_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> unfreezable_task_runner_;
  const Member<DetachableUseCounter> use_counter_;
  const Member<DetachableConsoleLogger> console_logger_;
  Member<LoaderFactory> loader_factory_;
  const Member<ResourceLoadScheduler> scheduler_;
  Member<BackForwardCacheLoaderHelper> back_forward_cache_loader_helper_;

  // Weak reference to all the fetched resources.
  DocumentResourceMap cached_resources_map_;

  // When a resource is in the global memory cache but not in the
  // cached_resources_map_ and it is referenced (e.g. when the StyleEngine
  // processes a @font-face rule), the resource will be emulated via
  // `EmulateLoadStartedForInspector` so that it shows up in DevTools.
  // In order to ensure that this only occurs once per resource, we keep
  // a weak reference to all emulated resources and only emulate resources
  // that have not been previously emulated.
  DocumentResourceMap emulated_load_started_for_inspector_resources_map_;

  // document_resource_strong_refs_ keeps strong references for fonts, images,
  // scripts and stylesheets within their freshness lifetime.
  HeapHashSet<Member<Resource>> document_resource_strong_refs_;
  size_t document_resource_strong_refs_total_size_ = 0;

  // |not_loaded_image_resources_| is a subset of all image resources for the
  // document where |Resource::IsLoaded| might be false. The is used for
  // performance optimizations and might still contain images which are actually
  // loaded.
  HeapHashSet<WeakMember<Resource>> not_loaded_image_resources_;
  HeapHashSet<WeakMember<Resource>> speculative_decode_candidate_images_;

  HeapHashMap<PreloadKey, Member<Resource>> preloads_;
  HeapVector<Member<Resource>> matched_preloads_;

  // Keeps preloads which are deferred to start loading based on the LCPP
  // signal of potentially unused preloads, in order to prevent subsequent
  // resource loading to the same resource from being scheduled, and record the
  // total count of deferred preloads.
  HeapHashMap<PreloadKey, Member<Resource>> deferred_preloads_;

  Member<MHTMLArchive> archive_;

  HeapTaskRunnerTimer<ResourceFetcher> resource_timing_report_timer_;

  TaskHandle unused_preloads_timer_;

  using PendingResourceTimingInfoMap =
      HeapHashMap<Member<Resource>, PendingResourceTimingInfo>;
  PendingResourceTimingInfoMap resource_timing_info_map_;

  Vector<ScheduledResourceTimingInfo> scheduled_resource_timing_reports_;

  HeapHashSet<Member<ResourceLoader>> loaders_;
  HeapHashSet<Member<ResourceLoader>> non_blocking_loaders_;

  HashMap<KURL, EarlyHintsPreloadEntry> unused_early_hints_preloaded_resources_;

  std::unique_ptr<HashSet<String>> preloaded_urls_for_test_;

  // TODO(altimin): Move FrameScheduler to oilpan.
  base::WeakPtr<FrameOrWorkerScheduler> frame_or_worker_scheduler_;

  // Timeout timer for keepalive requests.
  TaskHandle keepalive_loaders_task_handle_;

  uint32_t inflight_keepalive_bytes_ = 0;

  HeapMojoRemote<mojom::blink::BlobRegistry> blob_registry_remote_;

  // Lazily initialized when the first <script type=webbundle> is inserted.
  Member<SubresourceWebBundleList> subresource_web_bundles_;

  // The context lifecycle notifier. Used for GC lifetime management
  // purpose of the ResourceLoader used internally.
  Member<ContextLifecycleNotifier> context_lifecycle_notifier_;

  // This is not in the bit field below because we want to use AutoReset.
  bool is_in_request_resource_ = false;

  bool auto_load_images_ : 1;
  bool allow_stale_resources_ : 1;
  bool image_fetched_ : 1;
  bool stale_while_revalidate_enabled_ : 1;
  bool speculative_decode_in_flight_ : 1;
  // 27 bits left (decrease the count when you add bit fields above)

  static constexpr uint32_t kKeepaliveInflightBytesQuota = 64 * 1024;

  std::unique_ptr<ukm::MojoUkmRecorder> ukm_recorder_;

  SubresourceLoadMetrics subresource_load_metrics_;

  // Number of of not-small images that get a priority boost.
  // TODO(http://crbug.com/1431169): change this to a const after the
  // feature flag is removed.
  uint32_t boosted_image_target_ = 0;

  // Number of images that have had their priority boosted by heuristics.
  uint32_t boosted_image_count_ = 0;

  // Area (in pixels) below which an image is considered "small"
  uint32_t small_image_max_size_ = 0;

  // Number of resources that have had their priority boosted based on LCPP
  // signals.
  uint32_t potentially_lcp_resource_priority_boosts_ = 0;

  bool defer_unused_preload_enabled_for_testing_ = false;
  LcppDeferUnusedPreloadPreloadedReason
      defer_unused_preload_preloaded_reason_for_testing_;
  features::LcppDeferUnusedPreloadExcludedResourceType
      defer_unused_preload_excluded_resource_type_for_testing_;
};

class ResourceCacheValidationSuppressor {
  STACK_ALLOCATED();

 public:
  explicit ResourceCacheValidationSuppressor(ResourceFetcher* loader)
      : loader_(loader), previous_state_(false) {
    if (loader_) {
      previous_state_ = loader_->allow_stale_resources_;
      loader_->allow_stale_resources_ = true;
    }
  }
  ResourceCacheValidationSuppressor(const ResourceCacheValidationSuppressor&) =
      delete;
  ResourceCacheValidationSuppressor& operator=(
      const ResourceCacheValidationSuppressor&) = delete;
  ~ResourceCacheValidationSuppressor() {
    if (loader_) {
      loader_->allow_stale_resources_ = previous_state_;
    }
  }

 private:
  ResourceFetcher* loader_;
  bool previous_state_;
};

// Used for ResourceFetcher construction.
// Creators of ResourceFetcherInit are responsible for setting consistent
// members to ensure the correctness of ResourceFetcher.
struct PLATFORM_EXPORT ResourceFetcherInit final {
  STACK_ALLOCATED();

 public:
  // |context| and |task_runner| must not be null.
  // |loader_factory| can be null if |properties.IsDetached()| is true.
  // This takes two types of task runners: freezable and unfreezable one.
  // |unfreezable_task_runner| is used for handling incoming resource load from
  // outside the renderer via Mojo (i.e. by URLLoaderClientImpl and
  // ResponseBodyLoader) so that network loading can make progress even when a
  // frame is frozen, while |freezable_task_runner| is used for everything else.
  ResourceFetcherInit(
      DetachableResourceFetcherProperties& properties,
      FetchContext* context,
      scoped_refptr<base::SingleThreadTaskRunner> freezable_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> unfreezable_task_runner,
      ResourceFetcher::LoaderFactory* loader_factory,
      ContextLifecycleNotifier* context_lifecycle_notifier,
      BackForwardCacheLoaderHelper* back_forward_cache_loader_helper = nullptr);
  ResourceFetcherInit(const ResourceFetcherInit&) = delete;
  ResourceFetcherInit& operator=(const ResourceFetcherInit&) = delete;

  DetachableResourceFetcherProperties* const properties;
  FetchContext* const context;
  const scoped_refptr<base::SingleThreadTaskRunner> freezable_task_runner;
  const scoped_refptr<base::SingleThreadTaskRunner> unfreezable_task_runner;
  ResourceFetcher::LoaderFactory* const loader_factory;
  ContextLifecycleNotifier* const context_lifecycle_notifier;
  DetachableUseCounter* use_counter = nullptr;
  DetachableConsoleLogger* console_logger = nullptr;
  ResourceLoadScheduler::ThrottlingPolicy initial_throttling_policy =
      ResourceLoadScheduler::ThrottlingPolicy::kNormal;
  MHTMLArchive* archive = nullptr;
  FrameOrWorkerScheduler* frame_or_worker_scheduler = nullptr;
  ResourceLoadScheduler::ThrottleOptionOverride throttle_option_override =
      ResourceLoadScheduler::ThrottleOptionOverride::kNone;
  LoadingBehaviorObserver* loading_behavior_observer = nullptr;
  BackForwardCacheLoaderHelper* back_forward_cache_loader_helper = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_FETCHER_H_
