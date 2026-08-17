/*
    Copyright (C) 1998 Lars Knoll (knoll@mpi-hd.mpg.de)
    Copyright (C) 2001 Dirk Mueller <mueller@kde.org>
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

    This class provides all functionality needed for loading images, style
    sheets and html pages from the web. It has a memory cache for these objects.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_CLIENT_H_

#include "base/containers/span.h"
#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class Resource;

class PLATFORM_EXPORT ResourceClient : public GarbageCollectedMixin {
  USING_PRE_FINALIZER(ResourceClient, Prefinalize);

 public:
  ResourceClient() = default;
  virtual ~ResourceClient() = default;

  // DataReceived() is called each time a chunk of data is received.
  // For cache hits, the data is replayed before NotifyFinished() is called.
  // For successful revalidation responses, the data is NOT replayed, because
  // the Resource may not be in an entirely consistent state in the middle of
  // completing the revalidation, when DataReceived() would have to be called.
  // Some RawResourceClients depends on receiving all bytes via DataReceived(),
  // but RawResources forbid revalidation attempts, so they still are guaranteed
  // to get all data via DataReceived().
  virtual void DataReceived(Resource*, base::span<const char> /*data*/) {}
  virtual void NotifyFinished(Resource*) {}

  virtual bool IsFontResourceClient() const { return false; }

  virtual bool IsRawResourceClient() const { return false; }

  Resource* GetResource() const { return resource_.Get(); }

  bool FinishedFromMemoryCache() const { return finished_from_memory_cache_; }
  void SetHasFinishedFromMemoryCache() { finished_from_memory_cache_ = true; }

  // Name for debugging, e.g. shown in memory-infra.
  virtual String DebugName() const = 0;

  void Trace(Visitor* visitor) const override;

 protected:
  void ClearResource() { SetResource(nullptr, nullptr); }

 private:
  // ResourceFetcher is primarily responsible for calling SetResource() with a
  // non-null Resource*. ResourceClient subclasses are responsible for calling
  // ClearResource().
  friend class ResourceFetcher;
  // CSSFontFaceSrcValue only ever requests a Resource once, and acts as an
  // intermediate caching layer of sorts. It needs to be able to register
  // additional clients.
  friend class CSSFontFaceSrcValue;

  FRIEND_TEST_ALL_PREFIXES(ResourceTest, GarbageCollection);

  void SetResource(Resource* new_resource,
                   base::SingleThreadTaskRunner* task_runner);

  void Prefinalize();

  Member<Resource> resource_;

  // If true, the Resource was already available from the memory cache when this
  // ResourceClient was setup, so that the request finished immediately.
  bool finished_from_memory_cache_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_CLIENT_H_
