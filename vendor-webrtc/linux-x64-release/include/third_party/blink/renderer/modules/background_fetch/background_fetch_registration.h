// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_REGISTRATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_REGISTRATION_H_

#include "third_party/blink/public/mojom/background_fetch/background_fetch.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class BackgroundFetchRecord;
class CacheQueryOptions;
class ExceptionState;
class ScriptState;
class ServiceWorkerRegistration;
class V8BackgroundFetchFailureReason;
class V8BackgroundFetchResult;

// Represents an individual Background Fetch registration. Gives developers
// access to its properties, options, and enables them to abort the fetch.
class BackgroundFetchRegistration final
    : public EventTarget,
      public ActiveScriptWrappable<BackgroundFetchRegistration>,
      public blink::mojom::blink::BackgroundFetchRegistrationObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  BackgroundFetchRegistration(
      ServiceWorkerRegistration* service_worker_registration,
      mojom::blink::BackgroundFetchRegistrationPtr registration);

  ~BackgroundFetchRegistration() override;

  // BackgroundFetchRegistrationObserver implementation.
  void OnProgress(uint64_t upload_total,
                  uint64_t uploaded,
                  uint64_t download_total,
                  uint64_t downloaded,
                  mojom::BackgroundFetchResult result,
                  mojom::BackgroundFetchFailureReason failure_reason) override;
  void OnRecordsUnavailable() override;

  // Called when the |request| is complete. |response| points to the response
  // received, if any.
  void OnRequestCompleted(mojom::blink::FetchAPIRequestPtr request,
                          mojom::blink::FetchAPIResponsePtr response) override;

  // Web Exposed attribute defined in the IDL file. Corresponds to the
  // |developer_id| used elsewhere in the codebase.
  String id() const;
  ScriptPromise<BackgroundFetchRecord> match(ScriptState* script_state,
                                             const V8RequestInfo* request,
                                             const CacheQueryOptions* options,
                                             ExceptionState& exception_state);
  ScriptPromise<IDLSequence<BackgroundFetchRecord>> matchAll(
      ScriptState* scrip_state,
      ExceptionState& exception_state);
  ScriptPromise<IDLSequence<BackgroundFetchRecord>> matchAll(
      ScriptState* script_state,
      const V8RequestInfo* request,
      const CacheQueryOptions* options,
      ExceptionState& exception_state);

  uint64_t uploadTotal() const;
  uint64_t uploaded() const;
  uint64_t downloadTotal() const;
  uint64_t downloaded() const;
  bool recordsAvailable() const;
  V8BackgroundFetchResult result() const;
  V8BackgroundFetchFailureReason failureReason() const;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(progress, kProgress)

  ScriptPromise<IDLBoolean> abort(ScriptState*, ExceptionState&);

  // EventTarget implementation.
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  void Trace(Visitor* visitor) const override;

  // Keeps the object alive until there are non-zero number of |observers_|.
  bool HasPendingActivity() const final;

  void UpdateUI(
      const String& in_title,
      const SkBitmap& in_icon,
      mojom::blink::BackgroundFetchRegistrationService::UpdateUICallback
          callback);

 private:
  void DidAbort(ScriptPromiseResolver<IDLBoolean>* resolver,
                mojom::blink::BackgroundFetchError error);
  void MatchImpl(ScriptState* script_state,
                 ScriptPromiseResolverBase*,
                 const V8RequestInfo* request,
                 mojom::blink::CacheQueryOptionsPtr cache_query_options,
                 ExceptionState& exception_state,
                 bool match_all);

  void DidGetMatchingRequests(
      ScriptPromiseResolverBase* resolver,
      bool return_all,
      Vector<mojom::blink::BackgroundFetchSettledFetchPtr> settled_fetches);

  // Updates the |record| with a |response|, if one is available, else marks
  // the |record|'s request as aborted or failed.
  void UpdateRecord(BackgroundFetchRecord* record,
                    mojom::blink::FetchAPIResponsePtr& response);

  bool IsAborted();

  Member<ServiceWorkerRegistration> registration_;

  // Corresponds to IDL 'id' attribute. Not unique - an active registration can
  // have the same |developer_id_| as one or more inactive registrations.
  String developer_id_;

  uint64_t upload_total_;
  uint64_t uploaded_;
  uint64_t download_total_;
  uint64_t downloaded_;
  bool records_available_ = true;
  mojom::BackgroundFetchResult result_;
  mojom::BackgroundFetchFailureReason failure_reason_;
  HeapVector<Member<BackgroundFetchRecord>> observers_;

  HeapMojoRemote<mojom::blink::BackgroundFetchRegistrationService>
      registration_service_;

  HeapMojoReceiver<blink::mojom::blink::BackgroundFetchRegistrationObserver,
                   BackgroundFetchRegistration>
      observer_receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_REGISTRATION_H_
