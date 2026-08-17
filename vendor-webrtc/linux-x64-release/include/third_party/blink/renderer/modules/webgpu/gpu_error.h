// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_ERROR_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class GPUError : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GPUError* Create(const String& message);
  explicit GPUError(const String& message);

  GPUError(const GPUError&) = delete;
  GPUError& operator=(const GPUError&) = delete;

  // gpu_error.idl {{{
  const String& message() const;
  // }}} End of WebIDL binding implementation.

 private:
  String message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_ERROR_H_
