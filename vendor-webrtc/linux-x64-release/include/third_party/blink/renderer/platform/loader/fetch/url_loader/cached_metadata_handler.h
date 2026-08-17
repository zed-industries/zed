// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_CACHED_METADATA_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_CACHED_METADATA_HANDLER_H_

#include <stdint.h>

#include "base/containers/span.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/loader/code_cache.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace base {
class Time;
}

namespace mojo_base {
class BigBuffer;
}

namespace blink {

class CachedMetadata;
class CodeCacheHost;
class ParkableString;
class ResourceResponse;
class WebProcessMemoryDump;

// A callback for sending the serialized data of cached metadata to the
// persistent storage.
// TODO(pasko): rename this class to CachedMetadataPersister.
class PLATFORM_EXPORT CachedMetadataSender {
  USING_FAST_MALLOC(CachedMetadataSender);

 public:
  static std::unique_ptr<CachedMetadataSender> Create(
      const ResourceResponse&,
      mojom::blink::CodeCacheType,
      scoped_refptr<const SecurityOrigin> requestor_origin);

  static void SendToCodeCacheHost(CodeCacheHost*,
                                  mojom::blink::CodeCacheType,
                                  WTF::String,
                                  base::Time,
                                  const String&,
                                  base::span<const uint8_t>);

  virtual ~CachedMetadataSender() = default;
  virtual void Send(CodeCacheHost* code_cache_host,
                    base::span<const uint8_t>) = 0;

  // IsServedFromCacheStorage is used to alter caching strategy to be more
  // aggressive. See V8CodeCache::GetCompileOptions() for an example.
  virtual bool IsServedFromCacheStorage() = 0;
};

PLATFORM_EXPORT bool ShouldUseIsolatedCodeCache(
    mojom::blink::RequestContextType,
    const ResourceResponse&);

// Handler class for caching operations.
class CachedMetadataHandler : public GarbageCollected<CachedMetadataHandler> {
 public:
  enum ClearCacheType {
    // Clears the in-memory cache, but doesn't update persistent storage. The
    // old cached metadata is considered invalid.
    kClearLocally,

    // Discards the in-memory cache for memory reduction, preventing any further
    // uses or updates. The cached metadata will no longer be available, but
    // should not be considered invalid.
    kDiscardLocally,

    // Clears the metadata in both memory and persistent storage via
    // blink::Platform::CacheMetadata.
    kClearPersistentStorage
  };

  // Enum for marking serialized cached metadatas so that the deserializers
  // do not conflict. Do not remove or reorder entries, because old versions
  // could persist in cache storage after an upgrade.
  enum CachedMetadataType : uint32_t {
    // Replaced by kSingleEntryWithTag around 06/2023.
    // kSingleEntry = 0,  // the metadata is a single CachedMetadata entry

    // This was used for inline code cache, but the feature was removed around
    // 10/2022.
    // kSourceKeyedMap = 1,  // the metadata is multiple CachedMetadata
    // entries keyed by a source string.

    // Replaced by kSingleEntryWithHashAndPadding around 06/2023.
    // kSingleEntryWithHash = 2,  // the metadata is a content hash followed by
    //                            // a single CachedMetadata entry

    kSingleEntryWithTag = 3,  // The header contains an 8-byte tag; the metadata
                              // is a single CachedMetadata entry.

    kSingleEntryWithHashAndPadding = 4,  // The metadata is four bytes of
                                         // padding and a content hash followed
                                         // by a single CachedMetadata entry.
  };

  // Defines mutually exclusive serving sources for the handler's cached
  // metadata.
  enum class ServingSource {
    // Served by ServiceWorker CacheStorage.
    kCacheStorage,

    // Served by the static cached metadata from the application's resource
    // bundle for WebUI code.
    kWebUIBundledCache,

    // Served by other means, typically the platform's CodeCacheHost. Note: This
    // enum should be expanded as necessary.
    kOther,
  };

  virtual ~CachedMetadataHandler() = default;
  virtual void Trace(Visitor* visitor) const {}

  // Reset existing metadata. Subclasses can ignore setting new metadata after
  // clearing with |kDiscardLocally| to save memory.
  virtual void ClearCachedMetadata(CodeCacheHost* code_cache_host,
                                   ClearCacheType cache_type) = 0;

  // Returns the encoding to which the cache is specific.
  virtual String Encoding() const = 0;

  // The source serving the handler's cached metadata.
  virtual ServingSource GetServingSource() const {
    return ServingSource::kOther;
  }

  // Dump cache size kept in memory.
  virtual void OnMemoryDump(WebProcessMemoryDump* pmd,
                            const String& dump_prefix) const = 0;

  virtual size_t GetCodeCacheSize() const = 0;

  // Caches the given metadata associated with this resource and suggests that
  // the embedding platform's `code_cache_host` persist it. This is called by
  // blink following resource compilation by v8.
  // The `data_type_id` is a pseudo-randomly chosen identifier that is used to
  // distinguish data generated by the caller.
  virtual void SetCachedMetadata(CodeCacheHost* code_cache_host,
                                 uint32_t data_type_id,
                                 base::span<const uint8_t> data) = 0;

  // Caches the given serlialized metadata associated with this resource. This
  // is called by embedders with metadata retrieved from the platform's cache
  // during loading.
  // This is different to SetCachedMetadata() which caches the metadata created
  // following resource compilation by V8. Note that this does NOT suggest that
  // the platform persist it
  virtual void SetSerializedCachedMetadata(mojo_base::BigBuffer data) = 0;

  // Defines how GetCodeCache should behave if the metadata handler requires a
  // hash check but Check() hasn't yet been called.
  enum GetCachedMetadataBehavior {
    // HasCodeCache should crash the program with a runtime CHECK().
    kCrashIfUnchecked,

    // HasCodeCache should return true if the metadata handler contains data,
    // even though that data might be stale because we haven't yet validated
    // that it matches the current version of the script resource.
    kAllowUnchecked,
  };

  // Returns cached metadata of the given type associated with this resource.
  // This cached metadata can be pruned at any time.
  virtual scoped_refptr<CachedMetadata> GetCachedMetadata(
      uint32_t data_type_id,
      GetCachedMetadataBehavior behavior = kCrashIfUnchecked) const = 0;

  // Whether this cached metadata is required to contain a source text hash,
  // which is used in V8CodeCache to check whether the text of a newly-loaded
  // script matches the text when the code cache entry was written.
  virtual bool HashRequired() const { return false; }

  // If the handler requires source hashing, then Check does the following:
  // 1. If cached metadata is present, check the hash on the cached metadata,
  //    and clear it on a mismatch.
  // 2. Remember the source hash so that it will be included on any future calls
  //    that commit data to persistent storage.
  // Calling Check multiple times with different source_text is disallowed.
  virtual void Check(CodeCacheHost* code_cache_host,
                     const ParkableString& source_text) {
    // Do nothing.
  }

  // Called if the code cache data was used. `was_rejected` is true if V8
  // consumed but rejected the code cache. Intended for logging.
  virtual void DidUseCodeCache(bool was_rejected) {}

  // Called when a code cache will eventually be generated. It won't necessarily
  // be generated immediately. Intended for logging.
  virtual void WillProduceCodeCache() {}

 protected:
  CachedMetadataHandler() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_CACHED_METADATA_HANDLER_H_
