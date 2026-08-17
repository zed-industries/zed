/*
    Copyright (C) 2010 Rob Buis <rwlbuis@gmail.com>
    Copyright (C) 2011 Cosmin Truta <ctruta@gmail.com>
    Copyright (C) 2012 University of Szeged
    Copyright (C) 2012 Renata Hodovan <reni@webkit.org>

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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_DOCUMENT_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_DOCUMENT_CACHE_H_

#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class FetchParameters;
class SVGResourceDocumentContent;

class CORE_EXPORT SVGResourceDocumentCache final
    : public GarbageCollected<SVGResourceDocumentCache> {
 public:
  explicit SVGResourceDocumentCache(
      scoped_refptr<base::SingleThreadTaskRunner>);

  // The key is "URL (without fragment)" and the request mode (kSameOrigin or
  // kCors - other modes should be filtered by AllowedRequestMode).
  using CacheKey = std::pair<WTF::String, network::mojom::blink::RequestMode>;

  static CacheKey MakeCacheKey(const FetchParameters& params);

  SVGResourceDocumentContent* Get(const CacheKey& key);
  void Put(const CacheKey& key, SVGResourceDocumentContent* content);

  void WillBeDestroyed();

  void Trace(Visitor*) const;

 private:
  void DisposeUnobserved();
  void ProcessCustomWeakness(const LivenessBroker&);

  HeapHashMap<CacheKey, Member<SVGResourceDocumentContent>> entries_;
  scoped_refptr<base::SingleThreadTaskRunner> dispose_task_runner_;
  bool dispose_task_pending_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_DOCUMENT_CACHE_H_
