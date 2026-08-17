// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRINTING_WEB_PRINTING_TYPE_CONVERTERS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRINTING_WEB_PRINTING_TYPE_CONVERTERS_H_

#include "mojo/public/cpp/bindings/type_converter.h"
#include "third_party/blink/public/mojom/printing/web_printing.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_web_print_job_state.h"

namespace blink {
class WebPrinterAttributes;
class WebPrintJobTemplateAttributes;
}  // namespace blink

namespace mojo {

template <>
struct TypeConverter<blink::WebPrinterAttributes*,
                     blink::mojom::blink::WebPrinterAttributesPtr> {
  static blink::WebPrinterAttributes* Convert(
      const blink::mojom::blink::WebPrinterAttributesPtr&);
};

template <>
struct TypeConverter<blink::mojom::blink::WebPrintJobTemplateAttributesPtr,
                     blink::WebPrintJobTemplateAttributes*> {
  static blink::mojom::blink::WebPrintJobTemplateAttributesPtr Convert(
      const blink::WebPrintJobTemplateAttributes*);
};

template <>
struct TypeConverter<blink::V8WebPrintJobState::Enum,
                     blink::mojom::blink::WebPrintJobState> {
  static blink::V8WebPrintJobState::Enum Convert(
      const blink::mojom::blink::WebPrintJobState&);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRINTING_WEB_PRINTING_TYPE_CONVERTERS_H_
