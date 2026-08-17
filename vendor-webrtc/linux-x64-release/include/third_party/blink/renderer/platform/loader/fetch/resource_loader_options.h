/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOADER_OPTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOADER_OPTIONS_H_

#include "base/memory/scoped_refptr.h"
#include "base/memory/stack_allocated.h"
#include "base/types/strong_alias.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "services/network/public/mojom/content_security_policy.mojom-blink-forward.h"
#include "services/network/public/mojom/url_loader_factory.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_initiator_info.h"
#include "third_party/blink/renderer/platform/loader/fetch/integrity_metadata.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

enum DataBufferingPolicy : uint8_t { kBufferData, kDoNotBufferData };

enum SynchronousPolicy : uint8_t {
  kRequestSynchronously,
  kRequestAsynchronously
};

// Was the request generated from a "parser-inserted" element?
// https://html.spec.whatwg.org/C/#parser-inserted
enum ParserDisposition : uint8_t { kParserInserted, kNotParserInserted };

enum CacheAwareLoadingEnabled : uint8_t {
  kNotCacheAwareLoadingEnabled,
  kIsCacheAwareLoadingEnabled
};

// This class is thread-bound. Do not copy/pass an instance across threads.
struct PLATFORM_EXPORT ResourceLoaderOptions final {
  DISALLOW_NEW();

 public:
  // We define constructors, destructor, and assignment operator in
  // resource_loader_options.cc because they require the full definition of
  // URLLoaderFactory for |url_loader_factory| data member, and we'd like
  // to avoid to include huge url_loader_factory.mojom-blink.h.
  explicit ResourceLoaderOptions(const DOMWrapperWorld* world);
  ResourceLoaderOptions(const ResourceLoaderOptions& other);
  ResourceLoaderOptions& operator=(const ResourceLoaderOptions& other);
  ResourceLoaderOptions(ResourceLoaderOptions&& other);
  ResourceLoaderOptions& operator=(ResourceLoaderOptions&& other);
  ~ResourceLoaderOptions();

  void Trace(Visitor* visitor) const { visitor->Trace(world_for_csp); }

  FetchInitiatorInfo initiator_info;

  DataBufferingPolicy data_buffering_policy;

  network::mojom::CSPDisposition content_security_policy_option;
  SynchronousPolicy synchronous_policy;

  String content_security_policy_nonce;
  IntegrityMetadataSet integrity_metadata;
  ParserDisposition parser_disposition;
  CacheAwareLoadingEnabled cache_aware_loading_enabled;

  // The world in which this request initiated. This will be used for CSP checks
  // if specified. If null, the CSP bound to the FetchContext is used.
  Member<const DOMWrapperWorld> world_for_csp;

  // If not null, this URLLoaderFactory should be used to load this resource
  // rather than whatever factory the system might otherwise use.
  // Used for example for loading blob: URLs.
  scoped_refptr<base::RefCountedData<
      mojo::PendingRemote<network::mojom::blink::URLLoaderFactory>>>
      url_loader_factory;

  // Used by DevTools to emulate unsupported image types. See crbug.com/1130556.
  scoped_refptr<base::RefCountedData<HashSet<String>>>
      unsupported_image_mime_types;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOADER_OPTIONS_H_
