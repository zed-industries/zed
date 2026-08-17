// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_GOODS_DOM_WINDOW_DIGITAL_GOODS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_GOODS_DOM_WINDOW_DIGITAL_GOODS_H_

#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/digital_goods/digital_goods.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {
class DigitalGoodsService;
class LocalDOMWindow;
class ScriptState;

class DOMWindowDigitalGoods final
    : public GarbageCollected<DOMWindowDigitalGoods>,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  DOMWindowDigitalGoods(LocalDOMWindow& window);

  // IDL Interface:
  static ScriptPromise<DigitalGoodsService> getDigitalGoodsService(
      ScriptState*,
      LocalDOMWindow&,
      const String& payment_method,
      ExceptionState&);

  ScriptPromise<DigitalGoodsService> GetDigitalGoodsService(
      ScriptState*,
      LocalDOMWindow&,
      const String& payment_method,
      ExceptionState&);
  void Trace(Visitor* visitor) const override;

 private:
  HeapMojoRemote<payments::mojom::blink::DigitalGoodsFactory> mojo_service_;

  static DOMWindowDigitalGoods* FromState(LocalDOMWindow*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_GOODS_DOM_WINDOW_DIGITAL_GOODS_H_
