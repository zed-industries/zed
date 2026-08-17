/*
    Copyright (C) 1998 Lars Knoll (knoll@mpi-hd.mpg.de)
    Copyright (C) 2001 Dirk Mueller <mueller@kde.org>
    Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
    Copyright (C) 2004, 2005, 2006, 2007 Apple Inc. All rights reserved.

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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RAW_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RAW_RESOURCE_H_

#include <memory>
#include <variant>

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_client.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class BytesConsumer;
class BufferingBytesConsumer;
class FetchParameters;
class RawResourceClient;
class ResourceFetcher;

class PLATFORM_EXPORT RawResource final : public Resource {
 public:
  static RawResource* FetchSynchronously(FetchParameters&,
                                         ResourceFetcher*,
                                         RawResourceClient* = nullptr);
  static RawResource* Fetch(FetchParameters&,
                            ResourceFetcher*,
                            RawResourceClient*);
  static RawResource* FetchMedia(FetchParameters&,
                                 ResourceFetcher*,
                                 RawResourceClient*);
  static RawResource* FetchTextTrack(FetchParameters&,
                                     ResourceFetcher*,
                                     RawResourceClient*);
  static RawResource* FetchManifest(FetchParameters&,
                                    ResourceFetcher*,
                                    RawResourceClient*);

  // Exposed for testing
  static RawResource* CreateForTest(const ResourceRequest& request,
                                    ResourceType type) {
    ResourceLoaderOptions options(nullptr /* world */);
    return MakeGarbageCollected<RawResource>(request, type, options);
  }
  static RawResource* CreateForTest(const KURL& url,
                                    scoped_refptr<const SecurityOrigin> origin,
                                    ResourceType type) {
    ResourceRequest request(url);
    request.SetRequestorOrigin(std::move(origin));
    return CreateForTest(request, type);
  }

  RawResource(const ResourceRequest&,
              ResourceType,
              const ResourceLoaderOptions&);

  // Resource implementation
  bool WillFollowRedirect(const ResourceRequest&,
                          const ResourceResponse&) override;

  void SetSerializedCachedMetadata(mojo_base::BigBuffer data) override;

  scoped_refptr<BlobDataHandle> DownloadedBlob() const;

  void Trace(Visitor* visitor) const override;

 private:
  class RawResourceFactory : public NonTextResourceFactory {
   public:
    explicit RawResourceFactory(ResourceType type)
        : NonTextResourceFactory(type) {}

    Resource* Create(const ResourceRequest& request,
                     const ResourceLoaderOptions& options) const override {
      return MakeGarbageCollected<RawResource>(request, type_, options);
    }
  };
  class PreloadBytesConsumerClient;

  // Resource implementation
  void DidAddClient(ResourceClient*) override;
  void AppendData(
      std::variant<SegmentedBuffer, base::span<const char>>) override;

  bool ShouldIgnoreHTTPStatusCodeErrors() const override { return true; }

  void WillNotFollowRedirect() override;
  void ResponseReceived(const ResourceResponse&) override;
  void ResponseBodyReceived(
      ResponseBodyLoaderDrainableInterface&,
      scoped_refptr<base::SingleThreadTaskRunner> loader_task_runner) override;
  void DidSendData(uint64_t bytes_sent,
                   uint64_t total_bytes_to_be_sent) override;
  void DidDownloadData(uint64_t) override;
  void DidDownloadToBlob(scoped_refptr<BlobDataHandle>) override;
  void MatchPreload(const FetchParameters&) override;

  scoped_refptr<BlobDataHandle> downloaded_blob_;

  // Used for preload matching.
  Member<BufferingBytesConsumer> bytes_consumer_for_preload_;
  // True when this was initiated as a preload, and matched with a request
  // without UseStreamOnResponse.
  bool matched_with_non_streaming_destination_ = false;
};

// TODO(yhirano): Recover #if ENABLE_SECURITY_ASSERT when we stop adding
// RawResources to MemoryCache.
inline bool IsRawResource(ResourceType type) {
  return type == ResourceType::kRaw || type == ResourceType::kTextTrack ||
         type == ResourceType::kAudio || type == ResourceType::kVideo ||
         type == ResourceType::kManifest;
}
inline bool IsRawResource(const Resource& resource) {
  return IsRawResource(resource.GetType());
}
inline RawResource* ToRawResource(Resource* resource) {
  SECURITY_DCHECK(!resource || IsRawResource(*resource));
  return static_cast<RawResource*>(resource);
}

class PLATFORM_EXPORT RawResourceClient : public ResourceClient {
 public:
  bool IsRawResourceClient() const final { return true; }

  // The order of the callbacks is as follows:
  // [Case 1] A successful load:
  // 0+  RedirectReceived() and/or DataSent()
  // 1   ResponseReceived()
  // 0-1 SetSerializedCachedMetadata()
  // One of:
  //   0+  DataReceived()
  //   0+  DataDownloaded()
  //   0-1 ResponseBodyReceived()
  // 1   NotifyFinished() with ErrorOccurred() = false
  // [Case 2] When redirect is blocked:
  // 0+  RedirectReceived() and/or DataSent()
  // 1   RedirectBlocked()
  // 1   NotifyFinished() with ErrorOccurred() = true
  // [Case 3] Other failures:
  //     NotifyFinished() with ErrorOccurred() = true is called at any time
  //     (unless NotifyFinished() is already called).
  // In all cases:
  //     No callbacks are made after NotifyFinished() or
  //     RemoveClient() is called.
  virtual void DataSent(Resource*,
                        uint64_t /* bytesSent */,
                        uint64_t /* totalBytesToBeSent */) {}
  virtual void ResponseBodyReceived(Resource*, BytesConsumer&) {}
  virtual void ResponseReceived(Resource*, const ResourceResponse&) {}
  virtual void CachedMetadataReceived(Resource*, mojo_base::BigBuffer) {}
  virtual bool RedirectReceived(Resource*,
                                const ResourceRequest&,
                                const ResourceResponse&) {
    return true;
  }
  virtual void RedirectBlocked() {}
  virtual void DataDownloaded(Resource*, uint64_t) {}
  // Called for requests that had DownloadToBlob set to true. Can be called with
  // null if creating the blob failed for some reason (but the download itself
  // otherwise succeeded). Could also not be called at all if the downloaded
  // resource ended up being zero bytes.
  virtual void DidDownloadToBlob(Resource*, scoped_refptr<BlobDataHandle>);
};

// Checks the sequence of callbacks of RawResourceClient. This can be used only
// when a RawResourceClient is added as a client to at most one RawResource.
class PLATFORM_EXPORT RawResourceClientStateChecker final {
  DISALLOW_NEW();

 public:
  RawResourceClientStateChecker();

  // Call before addClient()/removeClient() is called.
  void WillAddClient();
  void WillRemoveClient();

  // Call RawResourceClientStateChecker::f() at the beginning of
  // RawResourceClient::f().
  void RedirectReceived();
  void RedirectBlocked();
  void DataSent();
  void ResponseReceived();
  void ResponseBodyReceived();
  void SetSerializedCachedMetadata();
  void DataReceived();
  void DataDownloaded();
  void DidDownloadToBlob();
  void NotifyFinished(Resource*);

 private:
  enum State {
    kNotAddedAsClient,
    kStarted,
    kRedirectBlocked,
    kResponseReceived,
    kDataReceivedAsBytesConsumer,
    kDataReceived,
    kDataDownloaded,
    kDidDownloadToBlob,
    kNotifyFinished,
    kDetached,
  };
  State state_ = kNotAddedAsClient;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RAW_RESOURCE_H_
