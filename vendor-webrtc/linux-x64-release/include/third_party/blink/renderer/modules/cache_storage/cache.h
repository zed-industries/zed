// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_associated_remote.h"
#include "third_party/blink/public/mojom/cache_storage/cache_storage.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_cache_query_options.h"
#include "third_party/blink/renderer/core/fetch/global_fetch.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace mojo {

using blink::mojom::blink::CacheQueryOptions;
using blink::mojom::blink::CacheQueryOptionsPtr;

template <>
struct TypeConverter<CacheQueryOptionsPtr, const blink::CacheQueryOptions*> {
  static CacheQueryOptionsPtr Convert(const blink::CacheQueryOptions* input) {
    CacheQueryOptionsPtr output = CacheQueryOptions::New();
    output->ignore_search = input->ignoreSearch();
    output->ignore_method = input->ignoreMethod();
    output->ignore_vary = input->ignoreVary();
    return output;
  }
};

}  // namespace mojo

namespace blink {

class AbortController;
class CacheStorageBlobClientList;
class ExceptionState;
class Response;
class Request;
class ScriptState;
class V8UnionResponseOrUndefined;

class MODULES_EXPORT Cache : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Cache(GlobalFetch::ScopedFetcher*,
        CacheStorageBlobClientList* blob_client_list,
        mojo::PendingAssociatedRemote<mojom::blink::CacheStorageCache>,
        ExecutionContext* execution_context,
        TaskType task_type);

  Cache(const Cache&) = delete;
  Cache& operator=(const Cache&) = delete;

  // From Cache.idl:
  ScriptPromise<V8UnionResponseOrUndefined> match(
      ScriptState* script_state,
      const V8RequestInfo* request,
      const CacheQueryOptions* options,
      ExceptionState& exception_state);
  ScriptPromise<IDLSequence<Response>> matchAll(ScriptState*, ExceptionState&);
  ScriptPromise<IDLSequence<Response>> matchAll(
      ScriptState* script_state,
      const V8RequestInfo* request,
      const CacheQueryOptions* options,
      ExceptionState& exception_state);
  ScriptPromise<IDLUndefined> add(ScriptState* script_state,
                                  const V8RequestInfo* request,
                                  ExceptionState& exception_state);
  ScriptPromise<IDLUndefined> addAll(
      ScriptState* script_state,
      const HeapVector<Member<V8RequestInfo>>& requests,
      ExceptionState& exception_state);
  ScriptPromise<IDLBoolean> Delete(ScriptState* script_state,
                                   const V8RequestInfo* request,
                                   const CacheQueryOptions* options,
                                   ExceptionState& exception_state);
  ScriptPromise<IDLUndefined> put(ScriptState* script_state,
                                  const V8RequestInfo* request,
                                  Response* response,
                                  ExceptionState& exception_state);
  ScriptPromise<IDLSequence<Request>> keys(ScriptState*, ExceptionState&);
  ScriptPromise<IDLSequence<Request>> keys(ScriptState* script_state,
                                           const V8RequestInfo* request,
                                           const CacheQueryOptions* options,
                                           ExceptionState& exception_state);

  void Trace(Visitor*) const override;

 protected:
  // Virtual for testing.
  virtual AbortController* CreateAbortController(ScriptState*);

 private:
  class BarrierCallbackForPutResponse;
  class BarrierCallbackForPutComplete;
  class CodeCacheHandleCallbackForPut;
  class ResponseBodyLoader;
  class FetchResolveHandler;
  class FetchRejectHandler;

  ScriptPromise<V8UnionResponseOrUndefined> MatchImpl(ScriptState*,
                                                      const Request*,
                                                      const CacheQueryOptions*,
                                                      ExceptionState&);
  ScriptPromise<IDLSequence<Response>> MatchAllImpl(ScriptState*,
                                                    const Request*,
                                                    const CacheQueryOptions*,
                                                    ExceptionState&);
  ScriptPromise<IDLUndefined> AddAllImpl(ScriptState*,
                                         const String& method_name,
                                         const HeapVector<Member<Request>>&,
                                         ExceptionState&);
  ScriptPromise<IDLBoolean> DeleteImpl(ScriptState*,
                                       const Request*,
                                       const CacheQueryOptions*,
                                       ExceptionState&);
  void PutImpl(ScriptPromiseResolver<IDLUndefined>*,
               const String& method_name,
               const HeapVector<Member<Request>>&,
               const HeapVector<Member<Response>>&,
               const WTF::Vector<scoped_refptr<BlobDataHandle>>& blob_list,
               ExceptionState&,
               int64_t trace_id);
  ScriptPromise<IDLSequence<Request>> KeysImpl(ScriptState*,
                                               const Request*,
                                               const CacheQueryOptions*,
                                               ExceptionState&);

  Member<GlobalFetch::ScopedFetcher> scoped_fetcher_;
  Member<CacheStorageBlobClientList> blob_client_list_;

  // TODO(https://crbug.com/356202294): Stop using
  // `kForceWithoutContextObserver`.
  HeapMojoAssociatedRemote<mojom::blink::CacheStorageCache,
                           HeapMojoWrapperMode::kForceWithoutContextObserver>
      cache_remote_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_H_
