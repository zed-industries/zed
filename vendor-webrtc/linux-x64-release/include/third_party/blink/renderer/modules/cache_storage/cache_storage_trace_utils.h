// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_STORAGE_TRACE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_STORAGE_TRACE_UTILS_H_

#include <memory>
#include <string>

#include "third_party/blink/public/mojom/cache_storage/cache_storage.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_response.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class TracedValue;

// The following are a set of helper functions to convert a cache_storage
// related value into something that can be passed to the TRACE_EVENT*
// macros.
//
// Note, these are designed to use WTF::WTF::String, blink mojo types, and
// blink::TracedValue.  Unforfortunately these types are not usable in
// content, so these routines must be duplicated there as well.

std::unique_ptr<TracedValue> CacheStorageTracedValue(const WTF::String& string);

std::unique_ptr<TracedValue> CacheStorageTracedValue(
    const mojom::blink::FetchAPIRequestPtr& request);

std::unique_ptr<TracedValue> CacheStorageTracedValue(
    const WTF::Vector<mojom::blink::FetchAPIRequestPtr>& requests);

std::unique_ptr<TracedValue> CacheStorageTracedValue(
    const mojom::blink::CacheQueryOptionsPtr& options);

std::unique_ptr<TracedValue> CacheStorageTracedValue(
    const mojom::blink::MultiCacheQueryOptionsPtr& options);

std::string CacheStorageTracedValue(mojom::blink::CacheStorageError error);

std::unique_ptr<TracedValue> CacheStorageTracedValue(
    const mojom::blink::FetchAPIResponsePtr& response);

std::unique_ptr<TracedValue> CacheStorageTracedValue(
    const WTF::Vector<mojom::blink::FetchAPIResponsePtr>& responses);

std::unique_ptr<TracedValue> CacheStorageTracedValue(
    const mojom::blink::BatchOperationPtr& op);

std::unique_ptr<TracedValue> CacheStorageTracedValue(
    const WTF::Vector<WTF::String>& string_list);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_STORAGE_TRACE_UTILS_H_
