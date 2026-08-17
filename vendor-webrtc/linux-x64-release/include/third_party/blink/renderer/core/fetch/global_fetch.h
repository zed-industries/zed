// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_GLOBAL_FETCH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_GLOBAL_FETCH_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/fetch/request.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class ExceptionState;
class LocalDOMWindow;
class NavigatorBase;
class RequestInit;
class DeferredRequestInit;
class Response;
class ScriptState;
class WorkerGlobalScope;
class FetchLaterResult;

class CORE_EXPORT GlobalFetch {
  STATIC_ONLY(GlobalFetch);

 public:
  class CORE_EXPORT ScopedFetcher : public GarbageCollectedMixin {
   public:
    virtual ~ScopedFetcher();

    virtual ScriptPromise<Response> Fetch(ScriptState*,
                                          const V8RequestInfo*,
                                          const RequestInit*,
                                          ExceptionState&) = 0;

    virtual FetchLaterResult* FetchLater(ScriptState*,
                                         const V8RequestInfo*,
                                         const DeferredRequestInit*,
                                         ExceptionState&);

    // Returns the number of fetch() method calls in the associated execution
    // context.  This is used for metrics.
    virtual uint32_t FetchCount() const = 0;

    // A wrapper to expose `FetchLaterManager::UpdateDeferredBytesQuota()`.
    // This method should only be called when `FetchLater()` is available.
    virtual void UpdateDeferredBytesQuota(const KURL& url,
                                          uint64_t& quota_for_url_origin,
                                          uint64_t& total_quota) const;

    static ScopedFetcher* From(LocalDOMWindow&);
    static ScopedFetcher* From(WorkerGlobalScope&);
    static ScopedFetcher* From(NavigatorBase& navigator);

    void Trace(Visitor*) const override;
  };

  static ScriptPromise<Response> fetch(ScriptState* script_state,
                                       LocalDOMWindow& window,
                                       const V8RequestInfo* input,
                                       const RequestInit* init,
                                       ExceptionState& exception_state);
  static ScriptPromise<Response> fetch(ScriptState* script_state,
                                       WorkerGlobalScope& worker,
                                       const V8RequestInfo* input,
                                       const RequestInit* init,
                                       ExceptionState& exception_state);

  static FetchLaterResult* fetchLater(ScriptState* script_state,
                                      LocalDOMWindow& window,
                                      const V8RequestInfo* input,
                                      const DeferredRequestInit* init,
                                      ExceptionState& exception_state);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_GLOBAL_FETCH_H_
