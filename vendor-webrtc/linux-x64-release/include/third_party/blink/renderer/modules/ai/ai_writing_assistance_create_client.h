// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_WRITING_ASSISTANCE_CREATE_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_WRITING_ASSISTANCE_CREATE_CLIENT_H_

#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_create_monitor_callback.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/ai/ai_context_observer.h"
#include "third_party/blink/renderer/modules/ai/ai_utils.h"
#include "third_party/blink/renderer/modules/ai/create_monitor.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"

namespace blink {

template <typename AIMojoClient,
          typename AIMojoCreateClient,
          typename CreateOptions,
          typename V8SessionObjectType>
class AIWritingAssistanceCreateClient
    : public GarbageCollected<
          AIWritingAssistanceCreateClient<AIMojoClient,
                                          AIMojoCreateClient,
                                          CreateOptions,
                                          V8SessionObjectType>>,
      public AIMojoCreateClient,
      public ExecutionContextClient,
      public AIContextObserver<V8SessionObjectType> {
 public:
  AIWritingAssistanceCreateClient(
      ScriptState* script_state,
      ScriptPromiseResolver<V8SessionObjectType>* resolver,
      CreateOptions* options)
      : ExecutionContextClient(ExecutionContext::From(script_state)),
        AIContextObserver<V8SessionObjectType>(script_state,
                                               this,
                                               resolver,
                                               options->getSignalOr(nullptr)),
        receiver_(this, GetExecutionContext()),
        options_(options),
        task_runner_(
            GetExecutionContext()->GetTaskRunner(TaskType::kInternalDefault)) {
    if (options->hasMonitor()) {
      monitor_ = MakeGarbageCollected<CreateMonitor>(GetExecutionContext(),
                                                     task_runner_);
      std::ignore = options->monitor()->Invoke(nullptr, monitor_);
      HeapMojoRemote<mojom::blink::AIManager>& ai_manager_remote =
          AIInterfaceProxy::GetAIManagerRemote(GetExecutionContext());
      ai_manager_remote->AddModelDownloadProgressObserver(
          monitor_->BindRemote());
    }
  }
  ~AIWritingAssistanceCreateClient() override = default;

  AIWritingAssistanceCreateClient(const AIWritingAssistanceCreateClient&) =
      delete;
  AIWritingAssistanceCreateClient& operator=(
      const AIWritingAssistanceCreateClient&) = delete;

  void Trace(Visitor* visitor) const override {
    ExecutionContextClient::Trace(visitor);
    AIContextObserver<V8SessionObjectType>::Trace(visitor);
    visitor->Trace(receiver_);
    visitor->Trace(options_);
    visitor->Trace(monitor_);
  }

  void Create() {
    mojo::PendingRemote<AIMojoCreateClient> client_remote;
    receiver_.Bind(client_remote.InitWithNewPipeAndPassReceiver(),
                   task_runner_);
    RemoteCreate(std::move(client_remote));
  }

  // AIMojoCreateClient:
  void OnResult(mojo::PendingRemote<AIMojoClient> pending_remote) override {
    if (!this->GetResolver()) {
      return;
    }
    if (pending_remote && monitor_) {
      // Ensure that a download completion event is sent.
      monitor_->OnDownloadProgressUpdate(kNormalizedDownloadProgressMax,
                                         kNormalizedDownloadProgressMax);
    }

    if (GetExecutionContext() && pending_remote) {
      this->GetResolver()->Resolve(MakeGarbageCollected<V8SessionObjectType>(
          GetExecutionContext(), task_runner_, std::move(pending_remote),
          options_));
    } else {
      this->GetResolver()->RejectWithDOMException(
          DOMExceptionCode::kInvalidStateError,
          kExceptionMessageUnableToCreateSession);
    }
    this->Cleanup();
  }

  void OnError(mojom::blink::AIManagerCreateClientError error) override {
    if (!this->GetResolver()) {
      return;
    }

    using mojom::blink::AIManagerCreateClientError;

    switch (error) {
      case AIManagerCreateClientError::kUnableToCreateSession:
      case AIManagerCreateClientError::kUnableToCalculateTokenSize: {
        this->GetResolver()->RejectWithDOMException(
            DOMExceptionCode::kInvalidStateError,
            kExceptionMessageUnableToCreateSession);
        break;
      }
      case AIManagerCreateClientError::kInitialInputTooLarge: {
        this->GetResolver()->RejectWithDOMException(
            DOMExceptionCode::kQuotaExceededError,
            kExceptionMessageInputTooLarge);
        break;
      }
      case AIManagerCreateClientError::kUnsupportedLanguage: {
        this->GetResolver()->RejectWithDOMException(
            DOMExceptionCode::kNotSupportedError,
            kExceptionMessageUnsupportedLanguages);
        break;
      }
    }
    this->Cleanup();
  }

  // AIContextObserver:
  void ResetReceiver() override { receiver_.reset(); }

 protected:
  // Runs Create* for the session type; defined in template specializations.
  void RemoteCreate(mojo::PendingRemote<AIMojoCreateClient> client_remote);

  HeapMojoReceiver<AIMojoCreateClient, AIWritingAssistanceCreateClient>
      receiver_;
  Member<CreateOptions> options_;
  Member<CreateMonitor> monitor_;
  scoped_refptr<base::SequencedTaskRunner> task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_WRITING_ASSISTANCE_CREATE_CLIENT_H_
