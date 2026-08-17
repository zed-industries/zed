// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_ERROR_H_

#include "services/webnn/public/mojom/webnn_context_provider.mojom-blink.h"
#include "services/webnn/public/mojom/webnn_error.mojom-blink.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"

namespace blink {

DOMExceptionCode WebNNErrorCodeToDOMExceptionCode(
    webnn::mojom::blink::Error::Code error_code);

template <typename MojoResultType>
mojo::StructPtr<MojoResultType> ToError(
    const webnn::mojom::blink::Error::Code& error_code,
    const WTF::String& error_message) {
  return MojoResultType::NewError(
      webnn::mojom::blink::Error::New(error_code, error_message));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ML_WEBNN_ML_ERROR_H_
