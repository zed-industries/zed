// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRINTING_WEB_PRINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRINTING_WEB_PRINTER_H_

#include "third_party/blink/public/mojom/printing/web_printing.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class WebPrintDocumentDescription;
class WebPrintJob;
class WebPrintJobTemplateAttributes;
class WebPrinterAttributes;

class MODULES_EXPORT WebPrinter : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  WebPrinter(ExecutionContext* execution_context,
             mojom::blink::WebPrinterInfoPtr printer_info);
  ~WebPrinter() override;

  WebPrinterAttributes* cachedAttributes() const { return attributes_; }
  ScriptPromise<WebPrinterAttributes> fetchAttributes(
      ScriptState* script_state,
      ExceptionState& exception_state);
  ScriptPromise<WebPrintJob> printJob(
      ScriptState* script_state,
      const String& job_name,
      const WebPrintDocumentDescription* document,
      const WebPrintJobTemplateAttributes* pjt_attributes,
      ExceptionState& exception_state);

  void Trace(Visitor* visitor) const override;

 private:
  void OnFetchAttributes(ScriptPromiseResolver<WebPrinterAttributes>*,
                         mojom::blink::WebPrinterFetchResultPtr result);

  void OnPrint(ScriptPromiseResolver<WebPrintJob>* resolver,
               mojom::blink::WebPrintResultPtr result);

  Member<WebPrinterAttributes> attributes_;
  Member<ScriptPromiseResolver<WebPrinterAttributes>>
      fetch_attributes_resolver_;
  HeapMojoRemote<mojom::blink::WebPrinter> printer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRINTING_WEB_PRINTER_H_
