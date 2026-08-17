// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_RECORD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_RECORD_H_

#include "third_party/blink/public/mojom/background_fetch/background_fetch.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Request;
class Response;
class ScriptState;

class MODULES_EXPORT BackgroundFetchRecord final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // The record can be in one of these states. |kSettled| can mean success or
  // failure based on whether or not there's a valid response to settle the
  // responseReady promise with.
  enum class State {
    kPending,
    kAborted,
    kSettled,
  };

  BackgroundFetchRecord(Request* request, ScriptState* script_state);
  ~BackgroundFetchRecord() override;

  Request* request() const;
  ScriptPromise<Response> responseReady(ScriptState* script_state);

  // Updates |record_state_| from kPending to kAborted or kSettled. Must be
  // called when |record_state_| is kPending.
  void UpdateState(State updated_state);

  // Resolve the responseReady promise with |response|, and update
  // |record_state_|. Must be called when |record_state_| is kPending.
  // Must not be called with a null response;
  void SetResponseAndUpdateState(mojom::blink::FetchAPIResponsePtr& response);

  bool IsRecordPending();
  void Trace(Visitor* visitor) const override;

  void OnRequestCompleted(mojom::blink::FetchAPIResponsePtr response);
  const KURL& ObservedUrl() const;

 private:
  using ResponseReadyProperty = ScriptPromiseProperty<Response, DOMException>;

  // Resolves a pending |response_ready_property_| with |response|, if it's not
  // null.
  // If |response| is null, we do nothing if the record isn't final yet. If
  // |record_state_| is State::kSettled in this case, we reject the promise.
  // This is because the record will not be updated with a valid |response|.
  void ResolveResponseReadyProperty(Response* response);

  Member<Request> request_;
  Member<ResponseReadyProperty> response_ready_property_;

  // Since BackgroundFetchRecord can only be accessed from the world that
  // created it, there's no danger of ScriptState leaking across worlds.
  Member<ScriptState> script_state_;

  State record_state_ = State::kPending;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_RECORD_H_
