/*
    Copyright (C) 1998 Lars Knoll (knoll@mpi-hd.mpg.de)
    Copyright (C) 2001 Dirk Mueller <mueller@kde.org>
    Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
    Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc. All
    rights reserved.

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
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_H_

#include <memory>
#include <optional>
#include <variant>

#include "base/auto_reset.h"
#include "base/containers/span.h"
#include "base/functional/callback.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "mojo/public/cpp/base/big_buffer.h"
#include "third_party/blink/public/mojom/loader/code_cache.mojom-blink-forward.h"
#include "third_party/blink/public/platform/scheduler/web_scoped_virtual_time_pauser.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/bindings/parkable_string.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_counted_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/instrumentation/memory_pressure_listener.h"
#include "third_party/blink/renderer/platform/instrumentation/tracing/web_process_memory_dump.h"
#include "third_party/blink/renderer/platform/loader/fetch/integrity_metadata.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_client.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_error.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_priority.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_priority.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_response.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_status.h"
#include "third_party/blink/renderer/platform/loader/fetch/text_resource_decoder_options.h"
#include "third_party/blink/renderer/platform/loader/integrity_report.h"
#include "third_party/blink/renderer/platform/loader/subresource_integrity.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_counted_set.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace base {
class Clock;
}

namespace blink {

class BackgroundResponseProcessorFactory;
class BlobDataHandle;
class FetchParameters;
class ResourceFinishObserver;
class ResourceLoader;
class ResponseBodyLoaderDrainableInterface;
class SecurityOrigin;

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
// When adding a new type, append it at the end, and also update the
// `ResourceType` enum in `tools/metrics/histograms/enums.xml`.
enum class ResourceType : uint8_t {
  // We do not have kMainResource anymore, which used to have zero value.
  kImage = 1,
  kCSSStyleSheet = 2,
  kScript = 3,
  kFont = 4,
  kRaw = 5,
  kSVGDocument = 6,
  kXSLStyleSheet = 7,
  kLinkPrefetch = 8,
  kTextTrack = 9,
  kAudio = 10,
  kVideo = 11,
  kManifest = 12,
  kSpeculationRules = 13,
  kMock = 14,  // Only for testing
  kDictionary = 15,
  kMaxValue = kDictionary
};

// A resource that is held in the cache. Classes who want to use this object
// should derive from ResourceClient, to get the function calls in case the
// requested data has arrived. This class also does the actual communication
// with the loader to obtain the resource from the network.
class PLATFORM_EXPORT Resource : public GarbageCollected<Resource>,
                                 public MemoryPressureListener {
 public:
  // An enum representing whether a resource match with another resource.
  // There are three kinds of status.
  // - kOk, which represents the success.
  // - kUnknownFailure, which represents miscellaneous failures. This includes
  //   failures which cannot happen for preload matching (for example,
  //   a failure due to non-cacheable request method cannot be happen for
  //   preload matching).
  // - other specific error status
  enum class MatchStatus {
    // Match succeeds.
    kOk,

    // Match fails because of an unknown reason.
    kUnknownFailure,

    // Subresource integrity value doesn't match.
    kIntegrityMismatch,

    // Match fails because the new request wants to load the content
    // as a blob.
    kBlobRequest,

    // Match fails because loading image is disabled.
    kImageLoadingDisabled,

    // Match fails due to different synchronous flags.
    kSynchronousFlagDoesNotMatch,

    // Match fails due to different request modes.
    kRequestModeDoesNotMatch,

    // Match fails due to different request credentials modes.
    kRequestCredentialsModeDoesNotMatch,

    // Match fails because keepalive flag is set on either requests.
    kKeepaliveSet,

    // Match fails due to different request methods.
    kRequestMethodDoesNotMatch,

    // Match fails due to different script types.
    kScriptTypeDoesNotMatch,
  };

  Resource(const Resource&) = delete;
  Resource& operator=(const Resource&) = delete;
  ~Resource() override;

  void Trace(Visitor*) const override;

  virtual WTF::TextEncoding Encoding() const { return WTF::TextEncoding(); }
  // If a BackgroundResponseProcessor consumed the body data on the background
  // thread, this method is called with a SegmentedBuffer data. Otherwise, it is
  // called with a span<const char> data several times.
  virtual void AppendData(
      std::variant<SegmentedBuffer, base::span<const char>>);
  virtual void FinishAsError(const ResourceError&,
                             base::SingleThreadTaskRunner*);

  void SetLinkPreload(bool is_link_preload) { link_preload_ = is_link_preload; }
  bool IsLinkPreload() const { return link_preload_; }

  const ResourceError& GetResourceError() const {
    DCHECK(error_);
    return *error_;
  }

  uint64_t InspectorId() const { return LastResourceRequest().InspectorId(); }

  virtual bool ShouldIgnoreHTTPStatusCodeErrors() const { return false; }

  const ResourceRequestHead& GetResourceRequest() const {
    return resource_request_;
  }
  const ResourceRequestHead& LastResourceRequest() const;
  const ResourceResponse& LastResourceResponse() const;
  // Returns zero if there are no redirects.
  size_t RedirectChainSize() const;

  virtual void SetRevalidatingRequest(const ResourceRequestHead&);

  // This url can have a fragment, but it can match resources that differ by the
  // fragment only.
  const KURL& Url() const { return GetResourceRequest().Url(); }
  ResourceType GetType() const { return static_cast<ResourceType>(type_); }
  const ResourceLoaderOptions& Options() const { return options_; }
  ResourceLoaderOptions& MutableOptions() { return options_; }

  void DidChangePriority(ResourceLoadPriority, int intra_priority_value);

  void UpdateResourceWidth(const AtomicString& resource_width);

  virtual void UpdateResourceInfoFromObservers() {}

  // Returns two priorities:
  // - `first` is the priority with the fix of https://crbug.com/1369823.
  // - `second` is the priority without the fix, ignoring the priority from
  //   ImageLoader.
  virtual std::pair<ResourcePriority, ResourcePriority> PriorityFromObservers()
      const {
    return std::make_pair(ResourcePriority(), ResourcePriority());
  }

  virtual bool HasNonDegenerateSizeForDecode() const { return false; }

  // If this Resource is already finished when AddClient is called, the
  // ResourceClient will be notified asynchronously by a task scheduled
  // on the given base::SingleThreadTaskRunner. Otherwise, the given
  // base::SingleThreadTaskRunner is unused.
  void AddClient(ResourceClient*, base::SingleThreadTaskRunner*);
  void RemoveClient(ResourceClient*);

  // If this Resource is already finished when AddFinishObserver is called, the
  // ResourceFinishObserver will be notified asynchronously by a task scheduled
  // on the given base::SingleThreadTaskRunner. Otherwise, the given
  // base::SingleThreadTaskRunner is unused.
  void AddFinishObserver(ResourceFinishObserver*,
                         base::SingleThreadTaskRunner*);
  void RemoveFinishObserver(ResourceFinishObserver*);

  bool IsUnusedPreload() const { return is_unused_preload_; }

  ResourceStatus GetStatus() const { return status_; }
  void SetStatus(ResourceStatus status) { status_ = status; }
  virtual ResourceStatus GetContentStatus() const { return status_; }

  size_t size() const { return EncodedSize() + DecodedSize() + OverheadSize(); }

  // Returns the size of content (response body) before decoding. Adding a new
  // usage of this function is not recommended (See the TODO below).
  //
  // TODO(hiroshige): Now EncodedSize/DecodedSize states are inconsistent and
  // need to be refactored (crbug/643135).
  size_t EncodedSize() const { return encoded_size_; }

  size_t DecodedSize() const { return decoded_size_; }
  size_t OverheadSize() const { return overhead_size_; }
  virtual size_t CodeCacheSize() const { return 0; }

  bool IsLoaded() const { return status_ > ResourceStatus::kPending; }

  bool IsLoading() const { return status_ == ResourceStatus::kPending; }
  bool StillNeedsLoad() const { return status_ < ResourceStatus::kPending; }

  void SetLoader(ResourceLoader*);
  ResourceLoader* Loader() const { return loader_.Get(); }

  bool IsLoadEventBlockingResourceType() const;

  // Computes the status of an object after loading. Updates the expire date on
  // the cache entry file
  virtual void Finish(base::TimeTicks finish_time,
                      base::SingleThreadTaskRunner*);
  void FinishForTest() { Finish(base::TimeTicks(), nullptr); }

  virtual scoped_refptr<const SharedBuffer> ResourceBuffer() const {
    return data_;
  }
  void SetResourceBuffer(scoped_refptr<SharedBuffer>);

  virtual bool WillFollowRedirect(const ResourceRequest&,
                                  const ResourceResponse&);

  // Called when a redirect response was received but a decision has already
  // been made to not follow it.
  virtual void WillNotFollowRedirect() {}

  virtual void ResponseReceived(const ResourceResponse&);
  virtual void ResponseBodyReceived(
      ResponseBodyLoaderDrainableInterface& body_loader,
      scoped_refptr<base::SingleThreadTaskRunner> loader_task_runner) {}
  virtual void DidReceiveDecodedData(
      const String& data,
      std::unique_ptr<ParkableStringImpl::SecureDigest> digest) {}
  void SetResponse(const ResourceResponse&);
  const ResourceResponse& GetResponse() const { return response_; }
  ResourceResponse& GetMutableResponseForTesting() { return response_; }

  // Sets the serialized metadata retrieved from the platform's cache.
  // The default implementation does nothing. Subclasses interested in the data
  // should implement the resource-specific behavior.
  virtual void SetSerializedCachedMetadata(mojo_base::BigBuffer data);

  AtomicString HttpContentType() const;

  bool WasCanceled() const { return error_ && error_->IsCancellation(); }
  bool ErrorOccurred() const {
    return status_ == ResourceStatus::kLoadError ||
           status_ == ResourceStatus::kDecodeError;
  }
  bool LoadFailedOrCanceled() const { return !!error_; }

  DataBufferingPolicy GetDataBufferingPolicy() const {
    return options_.data_buffering_policy;
  }
  void SetDataBufferingPolicy(DataBufferingPolicy);

  void MarkAsPreload();
  // Returns true if |this| resource is matched with the given parameters.
  virtual void MatchPreload(const FetchParameters&);

  bool CanReuseRedirectChain(UseCounter& use_counter) const;
  bool MustRevalidateDueToCacheHeaders(bool allow_stale,
                                       UseCounter& use_counter) const;
  bool ShouldRevalidateStaleResponse(UseCounter& use_counter) const;
  virtual bool CanUseCacheValidator() const;
  base::TimeDelta FreshnessLifetime(UseCounter& use_counter) const;
  bool IsCacheValidator() const {
    return revalidation_status_ == RevalidationStatus::kRevalidating;
  }
  bool HasSuccessfulRevalidation() const {
    return revalidation_status_ == RevalidationStatus::kRevalidated;
  }
  bool HasCacheControlNoStoreHeader() const;
  bool MustReloadDueToVaryHeader(const ResourceRequest& new_request) const;

  // Returns true if any response returned from the upstream in the redirect
  // chain is stale and requires triggering async stale revalidation. Once
  // revalidation is started SetStaleRevalidationStarted() should be called.
  bool StaleRevalidationRequested() const;

  // Returns true if any response returned from the upstream in the redirect
  // chain accessed the network.
  bool NetworkAccessed() const;

  // Set that stale revalidation has been started so that subsequent
  // requests won't trigger it again. When stale revalidation is completed
  // this resource will be removed from the MemoryCache so there is no
  // need to reset it back to false.
  bool StaleRevalidationStarted() const { return stale_revalidation_started_; }
  void SetStaleRevalidationStarted() { stale_revalidation_started_ = true; }

  const IntegrityMetadataSet& IntegrityMetadata() const {
    return options_.integrity_metadata;
  }
  bool PassedIntegrityChecks() const {
    return integrity_disposition_ == ResourceIntegrityDisposition::kPassed;
  }

  // Caching makes it possible for a resource that was requested with integrity
  // metadata to be reused for a request that didn't itself require integrity
  // checks. In that case, the integrity check can be skipped, as the integrity
  // may not have been "meant" for this specific request. If the resource is
  // being served from the preload cache however, we know any associated
  // integrity metadata and checks were destined for this request, so we cannot
  // skip the integrity check.
  //
  // We also force integrity checks for resources that declare their own
  // integrity information via an `Unencoded-Digest` header. Those should be
  // checked regardless of any given page's assertion through `integrity`
  // attributes.
  bool ForceIntegrityChecks() const;

  const IntegrityReport& IntegrityReport() const { return integrity_report_; }
  bool MustRefetchDueToIntegrityMetadata(const FetchParameters&) const;

  bool IsAlive() const { return is_alive_; }

  void SetCacheIdentifier(const String& cache_identifier) {
    cache_identifier_ = cache_identifier;
  }
  String CacheIdentifier() const { return cache_identifier_; }

  // https://fetch.spec.whatwg.org/#concept-request-origin
  const scoped_refptr<const SecurityOrigin>& GetOrigin() const;

  virtual void DidSendData(uint64_t /* bytesSent */,
                           uint64_t /* totalBytesToBeSent */) {}
  virtual void DidDownloadData(uint64_t) {}
  virtual void DidDownloadToBlob(scoped_refptr<BlobDataHandle>);

  base::TimeTicks LoadResponseEnd() const { return load_response_end_; }

  base::TimeTicks MemoryCacheLastAccessed() const {
    return memory_cache_last_accessed_;
  }

  void SetEncodedDataLength(int64_t value) {
    response_.SetEncodedDataLength(value);
  }
  void SetEncodedBodyLength(int64_t value) {
    response_.SetEncodedBodyLength(value);
  }
  void SetDecodedBodyLength(int64_t value) {
    response_.SetDecodedBodyLength(value);
  }

  // Returns |kOk| when |this| can be resused for the given arguments.
  MatchStatus CanReuse(const FetchParameters& params) const;

  // TODO(yhirano): Remove this once out-of-blink CORS is fully enabled.
  void SetResponseType(network::mojom::FetchResponseType response_type) {
    response_.SetType(response_type);
  }

  // If cache-aware loading is activated, this callback is called when the first
  // disk-cache-only request failed due to cache miss. After this callback,
  // cache-aware loading is deactivated and a reload with original request will
  // be triggered right away in ResourceLoader.
  virtual void WillReloadAfterDiskCacheMiss() {}

  // Used by the MemoryCache to reduce the memory consumption of the entry.
  void Prune();

  virtual void OnMemoryDump(WebMemoryDumpLevelOfDetail,
                            WebProcessMemoryDump*) const;

  // Used to notify ImageResourceContent of the start of actual loading.
  // JavaScript calls or client/observer notifications are disallowed inside
  // NotifyStartLoad().
  virtual void NotifyStartLoad() {
    CHECK_EQ(status_, ResourceStatus::kNotStarted);
    status_ = ResourceStatus::kPending;
  }

  static const char* ResourceTypeToString(
      ResourceType,
      const AtomicString& fetch_initiator_name);

  class ProhibitAddRemoveClientInScope : public base::AutoReset<bool> {
   public:
    ProhibitAddRemoveClientInScope(Resource* resource)
        : AutoReset(&resource->is_add_remove_client_prohibited_, true) {}
  };

  class RevalidationStartForbiddenScope : public base::AutoReset<bool> {
   public:
    RevalidationStartForbiddenScope(Resource* resource)
        : AutoReset(&resource->is_revalidation_start_forbidden_, true) {}
  };

  WebScopedVirtualTimePauser& VirtualTimePauser() {
    return virtual_time_pauser_;
  }

  // The caller owns the |clock| which must outlive the Resource.
  static void SetClockForTesting(const base::Clock* clock);

  size_t CalculateOverheadSizeForTest() const {
    return CalculateOverheadSize();
  }

  // Sets the ResourceRequest to be tagged as an ad.
  void SetIsAdResource();

  void DidRemoveClientOrObserver();

  void SetIsPreloadedByEarlyHints() { is_preloaded_by_early_hints_ = true; }

  bool IsPreloadedByEarlyHints() const { return is_preloaded_by_early_hints_; }

  virtual std::unique_ptr<BackgroundResponseProcessorFactory>
  MaybeCreateBackgroundResponseProcessorFactory();

  virtual bool HasClientsOrObservers() const {
    return !clients_.empty() || !clients_awaiting_callback_.empty() ||
           !finished_clients_.empty() || !finish_observers_.empty();
  }

 protected:
  Resource(const ResourceRequestHead&,
           ResourceType,
           const ResourceLoaderOptions&);

  virtual void NotifyDataReceived(base::span<const char> data);
  virtual void NotifyFinished();

  void MarkClientFinished(ResourceClient*);
  virtual void DestroyDecodedDataForFailedRevalidation() {}

  void SetEncodedSize(size_t);
  void SetDecodedSize(size_t);

  void FinishPendingClients();

  virtual void DidAddClient(ResourceClient*);
  void WillAddClientOrObserver();

  virtual void AllClientsAndObserversRemoved();

  bool HasClient(ResourceClient* client) const {
    return clients_.Contains(client) ||
           clients_awaiting_callback_.Contains(client) ||
           finished_clients_.Contains(client);
  }

  bool IsSuccessfulRevalidationResponse(
      const ResourceResponse& response) const {
    return IsCacheValidator() && response.HttpStatusCode() == 304;
  }

  struct RedirectPair {
    DISALLOW_NEW();

   public:
    explicit RedirectPair(const ResourceRequestHead& request,
                          const ResourceResponse& redirect_response)
        : request_(request), redirect_response_(redirect_response) {}

    ResourceRequestHead request_;
    ResourceResponse redirect_response_;
  };
  const Vector<RedirectPair>& RedirectChain() const { return redirect_chain_; }

  virtual void DestroyDecodedDataIfPossible() {}

  // Returns the memory dump name used for tracing. See Resource::OnMemoryDump.
  String GetMemoryDumpName() const;

  const HeapHashCountedSet<WeakMember<ResourceClient>>& Clients() const {
    return clients_;
  }

  void SetCachePolicyBypassingCache();
  void ClearRangeRequestHeader();

  SharedBuffer* Data() const { return data_.get(); }
  void ClearData();

  virtual void SetEncoding(const String&) {}

 private:
  friend class ResourceLoader;
  friend class MemoryCache;
  FRIEND_TEST_ALL_PREFIXES(MemoryCacheStrongReferenceTest, ResourceTimeout);

  void RevalidationSucceeded(const ResourceResponse&);
  void RevalidationFailed();

  size_t CalculateOverheadSize() const;

  String ReasonNotDeletable() const;

  // MemoryPressureListener overrides:
  void OnPurgeMemory() override;

  void CheckResourceIntegrity();
  void TriggerNotificationForFinishObservers(base::SingleThreadTaskRunner*);

  // Only call this from the MemoryCache. Calling it from anything else will
  // upset the MemoryCache's LRU.
  void UpdateMemoryCacheLastAccessedTime();

  void AppendDataImpl(SegmentedBuffer&&);
  void AppendDataImpl(base::span<const char>);

  ResourceType type_;
  ResourceStatus status_ = ResourceStatus::kNotStarted;

  std::optional<ResourceError> error_;

  base::TimeTicks load_response_end_;
  base::TimeTicks memory_cache_last_accessed_;

  size_t encoded_size_ = 0u;
  size_t decoded_size_ = 0u;

  String cache_identifier_;

  bool link_preload_ = false;
  bool is_alive_ = false;
  bool is_add_remove_client_prohibited_ = false;
  bool is_revalidation_start_forbidden_ = false;
  bool is_unused_preload_ = false;
  bool stale_revalidation_started_ = false;
  bool is_preloaded_by_early_hints_ = false;

  enum class RevalidationStatus {
    kNoRevalidatingOrFailed,  // not in revalidate procedure or
                              // revalidate failed.
    kRevalidating,            // in revalidate process, waiting for
                              // network response
    kRevalidated,             // revalidate success by 304 Not Modified
  };
  RevalidationStatus revalidation_status_ =
      RevalidationStatus::kNoRevalidatingOrFailed;

  ResourceIntegrityDisposition integrity_disposition_ =
      ResourceIntegrityDisposition::kNotChecked;
  class IntegrityReport integrity_report_;

  // Ordered list of all redirects followed while fetching this resource.
  Vector<RedirectPair> redirect_chain_;

  HeapHashCountedSet<WeakMember<ResourceClient>> clients_;
  HeapHashCountedSet<WeakMember<ResourceClient>> clients_awaiting_callback_;
  HeapHashCountedSet<WeakMember<ResourceClient>> finished_clients_;
  HeapHashSet<WeakMember<ResourceFinishObserver>> finish_observers_;

  ResourceLoaderOptions options_;

  base::Time response_timestamp_;

  TaskHandle async_finish_pending_clients_task_;

  ResourceRequestHead resource_request_;

  // Resource::CalculateOverheadSize() is affected by changes in
  // |m_resourceRequest.url()|, but |m_overheadSize| is not updated after
  // initial |m_resourceRequest| is given, to reduce MemoryCache manipulation
  // and thus potential bugs. crbug.com/594644
  const size_t overhead_size_;

  Member<ResourceLoader> loader_;
  ResourceResponse response_;

  scoped_refptr<SharedBuffer> data_;

  WebScopedVirtualTimePauser virtual_time_pauser_;
};

class ResourceFactory {
  STACK_ALLOCATED();

 public:
  virtual Resource* Create(const ResourceRequest&,
                           const ResourceLoaderOptions&,
                           const TextResourceDecoderOptions&) const = 0;

  ResourceType GetType() const { return type_; }
  TextResourceDecoderOptions::ContentType ContentType() const {
    return content_type_;
  }

 protected:
  explicit ResourceFactory(ResourceType type,
                           TextResourceDecoderOptions::ContentType content_type)
      : type_(type), content_type_(content_type) {}

  ResourceType type_;
  TextResourceDecoderOptions::ContentType content_type_;
};

class NonTextResourceFactory : public ResourceFactory {
 protected:
  explicit NonTextResourceFactory(ResourceType type)
      : ResourceFactory(type, TextResourceDecoderOptions::kPlainTextContent) {}

  virtual Resource* Create(const ResourceRequest&,
                           const ResourceLoaderOptions&) const = 0;

  Resource* Create(const ResourceRequest& request,
                   const ResourceLoaderOptions& options,
                   const TextResourceDecoderOptions&) const final {
    return Create(request, options);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_H_
