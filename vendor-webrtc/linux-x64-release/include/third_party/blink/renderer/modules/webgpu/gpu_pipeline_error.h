// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_PIPELINE_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_PIPELINE_ERROR_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_gpu_pipeline_error_reason.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class GPUPipelineErrorInit;

class MODULES_EXPORT GPUPipelineError : public DOMException {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructor exposed to script. Called by the V8 bindings.
  static GPUPipelineError* Create(String message,
                                  const GPUPipelineErrorInit* options);

  // For creating a GPUPipelineError from C++. Typically this will be
  // immediately passed to ScriptPromiseResolverBase::Reject.
  static v8::Local<v8::Value> Create(v8::Isolate*,
                                     const String& message,
                                     V8GPUPipelineErrorReason::Enum reason);

  // Use one of the Create() methods instead. This constructor has to be public
  // so that it can be used with MakeGarbageCollected<> inside the Create
  // methods.
  GPUPipelineError(const String& message,
                   V8GPUPipelineErrorReason::Enum reason);

  V8GPUPipelineErrorReason reason() const;

 private:
  const V8GPUPipelineErrorReason::Enum reason_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_PIPELINE_ERROR_H_
