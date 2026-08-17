// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_BLOB_URL_NULL_ORIGIN_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_BLOB_URL_NULL_ORIGIN_MAP_H_

#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/unguessable_token.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/thread_specific.h"

namespace blink {

class KURL;
class SecurityOrigin;

// BlobURLNullOriginMap contains pairs of blob URL and security origin that is
// serialized into "null". An instance of this class is per-thread, and created
// when GetInstace() is called for the first time.
//
// When a blob URL is created in an opaque origin or something whose
// SecurityOrigin::SerializesAsNull() returns true, the origin is serialized
// into the URL as "null". Since that makes it impossible to parse the origin
// back out and compare it against a context's origin (to check if a context is
// allowed to dereference the URL), this class stores a map of blob URL to such
// an origin.
class PLATFORM_EXPORT BlobURLNullOriginMap {
 public:
  // Returns a thread-specific instance. The instance is created when this
  // function is called for the first time.
  static ThreadSpecific<BlobURLNullOriginMap>& GetInstance();

  // Adds a pair of |blob_url| and |origin| to the map. |blob_url| and |origin|
  // must have the same "null" origin.
  void Add(const KURL& blob_url, SecurityOrigin* origin);

  // Removes a "null" origin keyed with |blob_url| from the map. |blob_url| must
  // have the "null" origin.
  void Remove(const KURL& blob_url);

  // Returns a "null" origin keyed with |blob_url| from the map. |blob_url| must
  // have the "null" origin.
  SecurityOrigin* Get(const KURL& blob_url);

 private:
  friend class ThreadSpecific<BlobURLNullOriginMap>;

  HashMap<String, scoped_refptr<SecurityOrigin>> blob_url_null_origin_map_;
};

// BlobURLOpaqueOriginNonceMap contains pairs of blob URL and opaque security
// origin's nonce. This is used for comparing opaque origins in a thread-safe
// way. An instance of this class is singleton, and can safely be accessed from
// any threads.
//
// BlobURLNullOriginMap above does not work for the case where the blob URL is
// registered in an opaque origin, and then a network request is sent to the URL
// from a different thread because the map contains non-thread-safe
// SecurityOrigin. For example, this happens on dedicated worker construction
// that loads a top-level worker script on a worker thread.
//
// To handle the case, BlobURLOpaqueOriginNonceMap keeps SecurityOrigin::Nonce
// instead of SecurityOrigin. The nonce is uniquely assigned to SecurityOrigin
// when it is constructed as an opaque origin, and SecurityOrigin instances
// (isolated-)copied from the same opaque origin share the same nonce. The nonce
// is thread-safe, so it is feasible to compare opaque origins over threads.
//
// TODO(nhiroki): Unify BlobURLNullOriginMap and BlobURLOpaqueOriginNonceMap.
// Making BlobURLNullOriginMap thread-safe could be possible solution, but
// actually it should be quite hard and not practical. This is because
// SecurityOrigin is not thread-safe, and widely used with an assumption that it
// is not shared among threads. Instead, we could stop using
// BlobURLNullOriginMap, and use BlobURLNullOriginMap in any case.
class PLATFORM_EXPORT BlobURLOpaqueOriginNonceMap {
 public:
  // Returns the singleton instance of this class. The instance is created when
  // this function is called for the first time.
  static BlobURLOpaqueOriginNonceMap& GetInstance();

  // Returns an opaque origin's nonce keyed with |blob_url| from the map.
  // |blob_url| must have the opaque origin.
  base::UnguessableToken Get(const KURL& blob_url) LOCKS_EXCLUDED(lock_);

 private:
  friend class BlobURLNullOriginMap;

  // Adds a pair of |blob_url| and |origin|'s nonce to the map. |blob_url| and
  // |origin| must have the same opaque origin. Only called from
  // BlobURLNullOriginMap::Add().
  void Add(const KURL& blob_url, SecurityOrigin* origin) LOCKS_EXCLUDED(lock_);

  // Removes an opaque origin's nonce keyed with |blob_url| from the map.
  // |blob_url| must have the opaque origin. Only called from
  // BlobURLNullOriginMap::Remove().
  void Remove(const KURL& blob_url) LOCKS_EXCLUDED(lock_);

  HashMap<String, base::UnguessableToken> blob_url_opaque_origin_nonce_map_
      GUARDED_BY(lock_);

  base::Lock lock_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_BLOB_URL_NULL_ORIGIN_MAP_H_
