// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_CALLBACK_INVOKE_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_CALLBACK_INVOKE_HELPER_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/native_value_traits_impl.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/bindings/v8_value_or_script_wrappable_adapter.h"
#include "third_party/blink/renderer/platform/scheduler/public/task_attribution_tracker.h"
#include "v8/include/v8.h"

namespace blink {

class CallbackFunctionBase;
class CallbackFunctionWithTaskAttributionBase;
class CallbackInterfaceBase;

namespace bindings {

enum class CallbackInvokeHelperMode {
  kDefault,
  kConstructorCall,
  kLegacyTreatNonObjectAsNull,
};

enum class CallbackReturnTypeIsPromise { kNo, kYes };

// This class helps implement the generated Blink-V8 bindings of IDL callback
// functions and IDL callback interfaces.  This class implements the following
// algorithms of Web IDL.
//
// https://webidl.spec.whatwg.org/#call-a-user-objects-operation
// 3.11. Callback interfaces
// To call a user object's operation
//
// https://webidl.spec.whatwg.org/#invoke-a-callback-function
// 3.12. Invoking callback functions
// To invoke a callback function type value
//
// https://webidl.spec.whatwg.org/#construct-a-callback-function
// 3.12. Invoking callback functions
// To construct a callback functions type value
template <class CallbackBase,
          CallbackInvokeHelperMode mode = CallbackInvokeHelperMode::kDefault,
          CallbackReturnTypeIsPromise return_type_is_promise =
              CallbackReturnTypeIsPromise::kNo>
class CallbackInvokeHelper final {
  STACK_ALLOCATED();

 public:
  CallbackInvokeHelper(CallbackBase* callback,
                       const char* class_like_name,
                       const char* property_name)
      : callback_(callback),
        class_like_name_(class_like_name),
        property_name_(property_name),
        // step: Prepare to run script with relevant settings.
        callback_relevant_context_scope_(
            callback->CallbackRelevantScriptState()),
        // step: Prepare to run a callback with stored settings.
        backup_incumbent_scope_(
            callback->IncumbentScriptState()->GetContext()) {}

  bool PrepareForCall(V8ValueOrScriptWrappableAdapter callback_this);

  bool Call(int argc, v8::Local<v8::Value>* argv);

  v8::Local<v8::Value> V8Result() { return result_; }

  template <typename IDLReturnType, typename ReturnType>
  v8::Maybe<ReturnType> Result() {
    DCHECK(!aborted_);
    v8::Isolate* isolate = callback_->GetIsolate();
    v8::TryCatch try_catch(isolate);
    auto&& result = NativeValueTraits<IDLReturnType>::NativeValue(
        isolate, result_, PassThroughException(isolate));
    if (try_catch.HasCaught()) [[unlikely]] {
      ApplyContextToException(
          callback_->CallbackRelevantScriptState(), try_catch.Exception(),
          ExceptionContext(v8::ExceptionContext::kOperation, class_like_name_,
                           property_name_));
      try_catch.ReThrow();
      return v8::Nothing<ReturnType>();
    }
    return v8::Just<ReturnType>(result);
  }

 private:
  bool CallInternal(int argc, v8::Local<v8::Value>* argv);
  bool Abort() {
    aborted_ = true;
    return false;
  }

  CallbackBase* callback_;
  const char* class_like_name_;
  const char* property_name_;
  v8::Local<v8::Function> function_;
  v8::Local<v8::Value> callback_this_;
  v8::Local<v8::Value> result_;
  bool aborted_ = false;

  ScriptState::Scope callback_relevant_context_scope_;
  v8::Context::BackupIncumbentScope backup_incumbent_scope_;
  std::optional<scheduler::TaskAttributionTracker::TaskScope>
      task_attribution_scope_;
};

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CallbackInvokeHelper<CallbackFunctionBase,
                         CallbackInvokeHelperMode::kDefault>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CallbackInvokeHelper<CallbackFunctionBase,
                         CallbackInvokeHelperMode::kDefault,
                         CallbackReturnTypeIsPromise::kYes>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CallbackInvokeHelper<CallbackFunctionBase,
                         CallbackInvokeHelperMode::kConstructorCall>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CallbackInvokeHelper<CallbackFunctionBase,
                         CallbackInvokeHelperMode::kConstructorCall,
                         CallbackReturnTypeIsPromise::kYes>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CallbackInvokeHelper<CallbackFunctionBase,
                         CallbackInvokeHelperMode::kLegacyTreatNonObjectAsNull>;

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CallbackInvokeHelper<CallbackFunctionWithTaskAttributionBase,
                         CallbackInvokeHelperMode::kDefault>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CallbackInvokeHelper<CallbackFunctionWithTaskAttributionBase,
                         CallbackInvokeHelperMode::kDefault,
                         CallbackReturnTypeIsPromise::kYes>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CallbackInvokeHelper<CallbackFunctionWithTaskAttributionBase,
                         CallbackInvokeHelperMode::kConstructorCall>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CallbackInvokeHelper<CallbackFunctionWithTaskAttributionBase,
                         CallbackInvokeHelperMode::kConstructorCall,
                         CallbackReturnTypeIsPromise::kYes>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CallbackInvokeHelper<CallbackFunctionWithTaskAttributionBase,
                         CallbackInvokeHelperMode::kLegacyTreatNonObjectAsNull>;

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CallbackInvokeHelper<CallbackInterfaceBase>;

}  // namespace bindings

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_CALLBACK_INVOKE_HELPER_H_
