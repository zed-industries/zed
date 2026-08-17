// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_ASYNC_CALLBACK_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_ASYNC_CALLBACK_HELPER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/fileapi/file_error.h"
#include "third_party/blink/renderer/modules/filesystem/directory_entry.h"
#include "third_party/blink/renderer/modules/filesystem/dom_file_system.h"
#include "third_party/blink/renderer/modules/filesystem/entry.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

class AsyncCallbackHelper {
  STATIC_ONLY(AsyncCallbackHelper);

 public:
  template <typename CallbackParam, typename V8Callback>
  static base::OnceCallback<void(CallbackParam*)> SuccessCallback(
      V8Callback* success_callback) {
    if (!success_callback)
      return base::OnceCallback<void(CallbackParam*)>();

    auto success_callback_wrapper = WTF::BindOnce(
        [](V8Callback* persistent_callback, CallbackParam* param) {
          persistent_callback->InvokeAndReportException(nullptr, param);
        },
        WrapPersistent(success_callback));
    return success_callback_wrapper;
  }

  static base::OnceCallback<void(base::File::Error)> ErrorCallback(
      V8ErrorCallback* error_callback) {
    if (!error_callback)
      return base::OnceCallback<void(base::File::Error)>();

    return WTF::BindOnce(
        [](V8ErrorCallback* persistent_callback, base::File::Error error) {
          persistent_callback->InvokeAndReportException(
              nullptr, file_error::CreateDOMException(error));
        },
        WrapPersistent(error_callback));
  }

  // The method below is not templatized, to be used exclusively for
  // VoidCallbacks.
  static base::OnceCallback<void()> VoidSuccessCallback(
      V8VoidCallback* success_callback) {
    if (!success_callback)
      return VoidCallbacks::SuccessCallback();

    auto success_callback_wrapper = WTF::BindOnce(
        [](V8VoidCallback* persistent_callback) {
          persistent_callback->InvokeAndReportException(nullptr);
        },
        WrapPersistent(success_callback));
    return success_callback_wrapper;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_ASYNC_CALLBACK_HELPER_H_
