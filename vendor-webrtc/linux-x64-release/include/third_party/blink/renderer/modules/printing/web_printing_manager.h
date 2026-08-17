// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRINTING_WEB_PRINTING_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRINTING_WEB_PRINTING_MANAGER_H_

#include "third_party/blink/public/mojom/printing/web_printing.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ExceptionState;
class NavigatorBase;
class WebPrinter;

class MODULES_EXPORT WebPrintingManager : public ScriptWrappable,
                                          public Supplement<NavigatorBase> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  // Getter for navigator.printing
  static WebPrintingManager* GetWebPrintingManager(NavigatorBase&);

  explicit WebPrintingManager(NavigatorBase&);

  // navigator.printing.getPrinters()
  ScriptPromise<IDLSequence<WebPrinter>> getPrinters(ScriptState*,
                                                     ExceptionState&);

  // ScriptWrappable:
  void Trace(Visitor*) const override;

 private:
  mojom::blink::WebPrintingService* GetPrintingService();
  void OnPrintersRetrieved(ScriptPromiseResolver<IDLSequence<WebPrinter>>*,
                           mojom::blink::GetPrintersResultPtr result);

  HeapMojoRemote<mojom::blink::WebPrintingService> printing_service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRINTING_WEB_PRINTING_MANAGER_H_
