// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_WRITING_ASSISTANCE_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_WRITING_ASSISTANCE_BASE_H_

#include "base/functional/callback_helpers.h"
#include "base/metrics/histogram_functions.h"
#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "services/network/public/mojom/permissions_policy/permissions_policy_feature.mojom-blink.h"
#include "third_party/blink/public/mojom/ai/ai_common.mojom-blink.h"
#include "third_party/blink/public/mojom/ai/model_streaming_responder.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/core/streams/readable_stream.h"
#include "third_party/blink/renderer/modules/ai/ai_interface_proxy.h"
#include "third_party/blink/renderer/modules/ai/ai_metrics.h"
#include "third_party/blink/renderer/modules/ai/ai_writing_assistance_create_client.h"
#include "third_party/blink/renderer/modules/ai/availability.h"
#include "third_party/blink/renderer/modules/ai/exception_helpers.h"
#include "third_party/blink/renderer/modules/ai/model_execution_responder.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace base {
class SequencedTaskRunner;
}  // namespace base

namespace blink {

class ReadableStream;

using CanCreateCallback =
    base::OnceCallback<void(mojom::blink::ModelAvailabilityCheckResult)>;

// TODO(crbug.com/402442890): Consider consolidating into one remote client
// and replace it with template class.
template <typename V8SessionObjectType,
          typename AIMojoClient,
          typename AIMojoCreateClient,
          typename CreateCoreOptions,
          typename CreateOptions,
          typename ExecuteOptions>
class AIWritingAssistanceBase : public ExecutionContextClient {
 public:
  AIWritingAssistanceBase(ExecutionContext* execution_context,
                          scoped_refptr<base::SequencedTaskRunner> task_runner,
                          mojo::PendingRemote<AIMojoClient> pending_remote,
                          CreateOptions* options,
                          bool echo_whitespace_input)
      : ExecutionContextClient(execution_context),
        remote_(execution_context),
        options_(options),
        task_runner_(std::move(task_runner)),
        metric_session_type_(GetSessionType()),
        echo_whitespace_input_(echo_whitespace_input) {
    remote_.Bind(std::move(pending_remote), task_runner_);
  }

  void Trace(Visitor* visitor) const override {
    ExecutionContextClient::Trace(visitor);
    visitor->Trace(remote_);
    visitor->Trace(options_);
  }

  static ScriptPromise<V8Availability> availability(
      ScriptState* script_state,
      CreateCoreOptions* options,
      ExceptionState& exception_state) {
    if (!script_state->ContextIsValid()) {
      ThrowInvalidContextException(exception_state);
      return ScriptPromise<V8Availability>();
    }

    auto* resolver =
        MakeGarbageCollected<ScriptPromiseResolver<V8Availability>>(
            script_state);
    auto promise = resolver->Promise();
    ExecutionContext* execution_context = ExecutionContext::From(script_state);

    // Return unavailable for cross-origin iframe access with no permission
    // policy.
    if (auto* window = DynamicTo<LocalDOMWindow>(execution_context)) {
      if (window->IsCrossSiteSubframeIncludingScheme() &&
          !window->IsFeatureEnabled(GetPermissionsPolicy())) {
        resolver->Resolve(AvailabilityToV8(Availability::kUnavailable));
        return promise;
      }
    }

    HeapMojoRemote<mojom::blink::AIManager>& ai_manager_remote =
        AIInterfaceProxy::GetAIManagerRemote(execution_context);

    if (!ai_manager_remote.is_connected()) {
      RejectPromiseWithInternalError(resolver);
      return promise;
    }

    RemoteCanCreate(
        ai_manager_remote, options,
        WTF::BindOnce(
            [](ScriptPromiseResolver<V8Availability>* resolver,
               ExecutionContext* execution_context,
               mojom::blink::ModelAvailabilityCheckResult result) {
              Availability availability = HandleModelAvailabilityCheckResult(
                  execution_context, GetSessionType(), result);
              resolver->Resolve(AvailabilityToV8(availability));
            },
            WrapPersistent(resolver), WrapPersistent(execution_context)));
    return promise;
  }

  static ScriptPromise<V8SessionObjectType> create(
      ScriptState* script_state,
      CreateOptions* options,
      ExceptionState& exception_state) {
    if (!script_state->ContextIsValid()) {
      ThrowInvalidContextException(exception_state);
      return ScriptPromise<V8SessionObjectType>();
    }
    CHECK(options);
    auto* resolver =
        MakeGarbageCollected<ScriptPromiseResolver<V8SessionObjectType>>(
            script_state);
    auto promise = resolver->Promise();
    AbortSignal* signal = options->getSignalOr(nullptr);
    if (signal && signal->aborted()) {
      resolver->Reject(signal->reason(script_state));
      return promise;
    }

    // Block cross-origin iframe access with no permission policy.
    auto* context = ExecutionContext::From(script_state);
    if (auto* window = DynamicTo<LocalDOMWindow>(context)) {
      if (window->GetFrame() &&
          window->GetFrame()->IsCrossOriginToOutermostMainFrame() &&
          !window->IsFeatureEnabled(GetPermissionsPolicy())) {
        resolver->Reject(MakeGarbageCollected<DOMException>(
            DOMExceptionCode::kNotAllowedError,
            kExceptionMessageCrossOriginAccess));
        return promise;
      }
    }

    ExecutionContext* execution_context = ExecutionContext::From(script_state);
    HeapMojoRemote<mojom::blink::AIManager>& ai_manager_remote =
        AIInterfaceProxy::GetAIManagerRemote(execution_context);

    if (!ai_manager_remote.is_connected()) {
      RejectPromiseWithInternalError(resolver);
      return promise;
    }

    MakeGarbageCollected<AIWritingAssistanceCreateClient<
        AIMojoClient, AIMojoCreateClient, CreateOptions, V8SessionObjectType>>(
        script_state, resolver, options)
        ->Create();
    return promise;
  }

  ScriptPromise<IDLString> execute(ScriptState* script_state,
                                   const String& input,
                                   const ExecuteOptions* options,
                                   ExceptionState& exception_state,
                                   AIMetrics::AIAPI metric_api_name) {
    if (!script_state->ContextIsValid()) {
      ThrowInvalidContextException(exception_state);
      return ScriptPromise<IDLString>();
    }
    if (!remote_) {
      ThrowSessionDestroyedException(exception_state);
      return ScriptPromise<IDLString>();
    }

    base::UmaHistogramEnumeration(
        AIMetrics::GetAIAPIUsageMetricName(metric_session_type_),
        metric_api_name);
    base::UmaHistogramCounts1M(
        AIMetrics::GetAISessionRequestSizeMetricName(metric_session_type_),
        static_cast<int>(input.CharactersSizeInBytes()));

    CHECK(options);
    auto* resolver =
        MakeGarbageCollected<ScriptPromiseResolver<IDLString>>(script_state);
    auto promise = resolver->Promise();

    AbortSignal* signal = options->getSignalOr(nullptr);
    if (signal && signal->aborted()) {
      resolver->Reject(signal->reason(script_state));
      return promise;
    }

    String trimmed_input = input.StripWhiteSpace();
    if (trimmed_input.empty()) {
      resolver->Resolve(echo_whitespace_input_ ? input : trimmed_input);
      return promise;
    }

    const String trimmed_context =
        options->getContextOr(g_empty_string).StripWhiteSpace();
    auto pending_remote = CreateModelExecutionResponder(
        script_state, signal, resolver, task_runner_, metric_session_type_,
        /*complete_callback=*/base::DoNothing(),
        /*overflow_callback=*/base::DoNothing());
    remoteExecute(trimmed_input, trimmed_context, std::move(pending_remote));
    return promise;
  }

  // TODO(crbug.com/402442890): Refactor common code between `execute()` and
  // `executeStreaming()`.
  ReadableStream* executeStreaming(ScriptState* script_state,
                                   const String& input,
                                   const ExecuteOptions* options,
                                   ExceptionState& exception_state,
                                   AIMetrics::AIAPI metric_api_name) {
    if (!script_state->ContextIsValid()) {
      ThrowInvalidContextException(exception_state);
      return nullptr;
    }
    if (!remote_) {
      ThrowSessionDestroyedException(exception_state);
      return nullptr;
    }

    base::UmaHistogramEnumeration(
        AIMetrics::GetAIAPIUsageMetricName(metric_session_type_),
        metric_api_name);
    base::UmaHistogramCounts1M(
        AIMetrics::GetAISessionRequestSizeMetricName(metric_session_type_),
        static_cast<int>(input.CharactersSizeInBytes()));

    CHECK(options);
    AbortSignal* signal = options->getSignalOr(nullptr);
    if (HandleAbortSignal(signal, script_state, exception_state)) {
      return nullptr;
    }

    String trimmed_input = input.StripWhiteSpace();
    if (trimmed_input.empty()) {
      return CreateEmptyReadableStream(script_state, metric_session_type_);
    }

    const String trimmed_context =
        options->getContextOr(g_empty_string).StripWhiteSpace();
    auto [readable_stream, pending_remote] =
        CreateModelExecutionStreamingResponder(
            script_state, signal, task_runner_, metric_session_type_,
            /*complete_callback=*/base::DoNothing(),
            /*overflow_callback=*/base::DoNothing());
    remoteExecute(trimmed_input, trimmed_context, std::move(pending_remote));
    return readable_stream;
  }

  ScriptPromise<IDLDouble> measureInputUsage(ScriptState* script_state,
                                             const String& input,
                                             const ExecuteOptions* options,
                                             ExceptionState& exception_state) {
    if (!script_state->ContextIsValid()) {
      ThrowInvalidContextException(exception_state);
      return ScriptPromise<IDLDouble>();
    }

    if (!remote_) {
      ThrowSessionDestroyedException(exception_state);
      return ScriptPromise<IDLDouble>();
    }

    auto* resolver =
        MakeGarbageCollected<ScriptPromiseResolver<IDLDouble>>(script_state);
    auto promise = resolver->Promise();
    CHECK(options);
    AbortSignal* signal = options->getSignalOr(nullptr);
    if (signal && signal->aborted()) {
      resolver->Reject(signal->reason(script_state));
      return promise;
    }

    remote_->MeasureUsage(
        input, options->getContextOr(g_empty_string),
        WTF::BindOnce(
            [](ScriptPromiseResolver<IDLDouble>* resolver, AbortSignal* signal,
               std::optional<uint64_t> usage) {
              ExecutionContext* context = resolver->GetExecutionContext();
              if (!context) {
                return;
              }
              if (signal && signal->aborted()) {
                resolver->Reject(signal->reason(resolver->GetScriptState()));
                return;
              }
              if (!usage.has_value()) {
                resolver->Reject(DOMException::Create(
                    kExceptionMessageUnableToCalculateUsage,
                    DOMException::GetErrorName(
                        DOMExceptionCode::kOperationError)));
                return;
              }
              resolver->Resolve(static_cast<double>(usage.value()));
            },
            WrapPersistent(resolver),
            WrapPersistent(options->getSignalOr(nullptr))));

    return promise;
  }

  void destroy(ScriptState* script_state, ExceptionState& exception_state) {
    if (!script_state->ContextIsValid()) {
      ThrowInvalidContextException(exception_state);
      return;
    }

    base::UmaHistogramEnumeration(
        AIMetrics::GetAIAPIUsageMetricName(metric_session_type_),
        AIMetrics::AIAPI::kSessionDestroy);

    remote_.reset();
  }

  String sharedContext() const {
    return options_->getSharedContextOr(g_empty_string);
  }

  std::optional<Vector<String>> expectedInputLanguages() const {
    if (options_->hasExpectedInputLanguages()) {
      return options_->expectedInputLanguages();
    }
    return std::nullopt;
  }

  std::optional<Vector<String>> expectedContextLanguages() const {
    if (options_->hasExpectedContextLanguages()) {
      return options_->expectedContextLanguages();
    }
    return std::nullopt;
  }

  String outputLanguage() const {
    return options_->getOutputLanguageOr(String());
  }

  double inputQuota() const {
    return static_cast<double>(
        mojom::blink::kWritingAssistanceMaxInputTokenSize);
  }

 protected:
  // Executes a writing assistance task on the remote `AIMojoClient`,
  // such as `Summarize()`, `Write()`, or `Rewrite()`.
  // Called by `execute()` or `executeStreaming()`.
  virtual void remoteExecute(
      const String& input,
      const String& context,
      mojo::PendingRemote<blink::mojom::blink::ModelStreamingResponder>
          responder) = 0;

  // Returns the session type; defined in template specializations.
  static AIMetrics::AISessionType GetSessionType();

  // Returns permission policy feature for session type.
  static network::mojom::PermissionsPolicyFeature GetPermissionsPolicy();

  // Runs CanCreate* for the session type; defined in template specializations.
  static void RemoteCanCreate(
      HeapMojoRemote<mojom::blink::AIManager>& ai_manager_remote,
      CreateCoreOptions* options,
      CanCreateCallback callback);

  HeapMojoRemote<AIMojoClient> remote_;
  Member<CreateOptions> options_;

 private:
  scoped_refptr<base::SequencedTaskRunner> task_runner_;
  AIMetrics::AISessionType metric_session_type_;
  // Whether to echo back the original input if it only contains whitespace.
  // If false, it returns an empty string.
  bool echo_whitespace_input_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_WRITING_ASSISTANCE_BASE_H_
