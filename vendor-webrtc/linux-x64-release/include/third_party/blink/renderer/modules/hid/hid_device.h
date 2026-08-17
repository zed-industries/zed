// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_HID_HID_DEVICE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_HID_HID_DEVICE_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "services/device/public/mojom/hid.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/hid/hid.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_hid_report_item.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_piece.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
class DOMDataView;
class ExecutionContext;
class HIDCollectionInfo;
class ScriptState;

class MODULES_EXPORT HIDDevice
    : public EventTarget,
      public ExecutionContextLifecycleObserver,
      public ActiveScriptWrappable<HIDDevice>,
      public device::mojom::blink::HidConnectionClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // ServiceInterface provides a pure-virtual HID service interface for
  // HIDDevice creators to, for example, open a device.
  class ServiceInterface : public GarbageCollectedMixin {
   public:
    virtual void Connect(
        const String& device_guid,
        mojo::PendingRemote<device::mojom::blink::HidConnectionClient>
            connection_client,
        device::mojom::blink::HidManager::ConnectCallback callback) = 0;
    virtual void Forget(device::mojom::blink::HidDeviceInfoPtr device_info,
                        mojom::blink::HidService::ForgetCallback callback) = 0;
  };

  HIDDevice(ServiceInterface* parent,
            device::mojom::blink::HidDeviceInfoPtr info,
            ExecutionContext* execution_context);
  ~HIDDevice() override;

  // EventTarget:
  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;

  // device::mojom::blink::HidConnectionClient:
  void OnInputReport(uint8_t report_id, const Vector<uint8_t>& buffer) override;

  // Web-exposed interfaces:
  DEFINE_ATTRIBUTE_EVENT_LISTENER(inputreport, kInputreport)

  bool opened() const;
  uint16_t vendorId() const;
  uint16_t productId() const;
  String productName() const;
  const HeapVector<Member<HIDCollectionInfo>>& collections() const;

  ScriptPromise<IDLUndefined> open(ScriptState* script_state,
                                   ExceptionState& exception_state);
  ScriptPromise<IDLUndefined> close(ScriptState*);
  ScriptPromise<IDLUndefined> forget(ScriptState*,
                                     ExceptionState& exception_state);
  ScriptPromise<IDLUndefined> sendReport(ScriptState*,
                                         uint8_t report_id,
                                         base::span<const uint8_t> data);
  ScriptPromise<IDLUndefined> sendFeatureReport(ScriptState*,
                                                uint8_t report_id,
                                                base::span<const uint8_t> data);
  ScriptPromise<NotShared<DOMDataView>> receiveFeatureReport(ScriptState*,
                                                             uint8_t report_id);

  // ExecutionContextLifecycleObserver:
  void ContextDestroyed() override;

  // ActiveScriptWrappable:
  bool HasPendingActivity() const override;

  void UpdateDeviceInfo(device::mojom::blink::HidDeviceInfoPtr info);
  void ResetIsForgotten();

  static HIDReportItem* ToHIDReportItem(
      const device::mojom::blink::HidReportItem& report_item);

  void Trace(Visitor*) const override;

 private:
  bool EnsureNoDeviceChangeInProgress(
      ScriptPromiseResolverBase* resolver) const;
  bool EnsureDeviceIsNotForgotten(ScriptPromiseResolverBase* resolver) const;

  void OnServiceConnectionError();

  void FinishOpen(ScriptPromiseResolver<IDLUndefined>*,
                  mojo::PendingRemote<device::mojom::blink::HidConnection>);
  void FinishForget(ScriptPromiseResolver<IDLUndefined>*);
  void FinishSendReport(ScriptPromiseResolver<IDLUndefined>*, bool success);
  void FinishSendFeatureReport(ScriptPromiseResolver<IDLUndefined>*,
                               bool success);
  void FinishReceiveFeatureReport(
      ScriptPromiseResolver<NotShared<DOMDataView>>*,
      bool success,
      const std::optional<Vector<uint8_t>>&);

  void MarkRequestComplete(ScriptPromiseResolverBase*);

  Member<ServiceInterface> parent_;
  device::mojom::blink::HidDeviceInfoPtr device_info_;
  HeapMojoRemote<device::mojom::blink::HidConnection> connection_;
  HeapMojoReceiver<device::mojom::blink::HidConnectionClient, HIDDevice>
      receiver_;
  HeapHashSet<Member<ScriptPromiseResolverBase>> device_requests_;
  HeapVector<Member<HIDCollectionInfo>> collections_;
  bool device_state_change_in_progress_ = false;
  bool device_is_forgotten_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_HID_HID_DEVICE_H_
