// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_GOODS_DIGITAL_GOODS_SERVICE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_GOODS_DIGITAL_GOODS_SERVICE_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/digital_goods/digital_goods.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
class ItemDetails;
class PurchaseDetails;
class ScriptState;

class DigitalGoodsService final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit DigitalGoodsService(
      ExecutionContext* context,
      mojo::PendingRemote<payments::mojom::blink::DigitalGoods> pending_remote);
  ~DigitalGoodsService() override;

  // IDL Interface:
  ScriptPromise<IDLSequence<ItemDetails>> getDetails(
      ScriptState*,
      const Vector<String>& item_ids);
  ScriptPromise<IDLSequence<PurchaseDetails>> listPurchases(ScriptState*);
  ScriptPromise<IDLSequence<PurchaseDetails>> listPurchaseHistory(ScriptState*);
  ScriptPromise<IDLUndefined> consume(ScriptState*,
                                      const String& purchase_token);

  void Trace(Visitor* visitor) const override;

 private:
  HeapMojoRemote<payments::mojom::blink::DigitalGoods> mojo_service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_GOODS_DIGITAL_GOODS_SERVICE_H_
