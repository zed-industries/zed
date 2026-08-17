// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_UTILS_H_

#include "third_party/blink/public/mojom/cache_storage/cache_storage.mojom-blink.h"

namespace blink {

class CacheStorageBlobClientList;
class Response;
class ScriptState;

// A utility function to deserialize an eagerly read cache_storage response
// into a web-exposed fetch Response object.  The resulting Response will
// have a DataPipeBytesConsumer body and a side_data_blob which can be used
// to read any code cache.
Response* CreateEagerResponse(ScriptState* script_state,
                              mojom::blink::EagerResponsePtr eager_response,
                              CacheStorageBlobClientList* client_list);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_UTILS_H_
