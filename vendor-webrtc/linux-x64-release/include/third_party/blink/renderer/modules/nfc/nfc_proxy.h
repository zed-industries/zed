// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NFC_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NFC_PROXY_H_

#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/device/public/mojom/nfc.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace blink {

class LocalDOMWindow;
class NDEFReader;

// This is a proxy class used by NDEFReader(s) to connect
// to implementation of device::mojom::blink::NFC interface.
class MODULES_EXPORT NFCProxy final : public GarbageCollected<NFCProxy>,
                                      public Supplement<LocalDOMWindow>,
                                      public device::mojom::blink::NFCClient {
 public:
  static const char kSupplementName[];
  static NFCProxy* From(LocalDOMWindow&);

  explicit NFCProxy(LocalDOMWindow&);
  ~NFCProxy() override;

  void Trace(Visitor*) const override;

  // There is no matching RemoveWriter() method because writers are
  // automatically removed from the weak hash set when they are garbage
  // collected.
  void AddWriter(NDEFReader*);

  void StartReading(NDEFReader*,
                    device::mojom::blink::NFC::WatchCallback);
  void StopReading(NDEFReader*);
  bool IsReading(const NDEFReader*);
  void Push(device::mojom::blink::NDEFMessagePtr,
            device::mojom::blink::NDEFWriteOptionsPtr,
            device::mojom::blink::NFC::PushCallback);
  void CancelPush();
  void MakeReadOnly(device::mojom::blink::NFC::MakeReadOnlyCallback);
  void CancelMakeReadOnly();

 private:
  // Implementation of device::mojom::blink::NFCClient.
  void OnWatch(const Vector<uint32_t>&,
               const String&,
               device::mojom::blink::NDEFMessagePtr) override;
  void OnError(device::mojom::blink::NDEFErrorPtr) override;

  void OnReaderRegistered(NDEFReader*,
                          uint32_t watch_id,
                          device::mojom::blink::NFC::WatchCallback,
                          device::mojom::blink::NDEFErrorPtr);

  void EnsureMojoConnection();

  // This could only happen when the embedder does not implement NFC interface.
  void OnMojoConnectionError();

  // Identifies watch requests tied to a given Mojo connection of NFC interface,
  // i.e. |nfc_|. Incremented each time a watch request is made.
  uint32_t next_watch_id_ = 1;

  // The <NDEFReader, WatchId> map keeps all readers that have started reading.
  // The watch id comes from |next_watch_id_|.
  using ReaderMap = HeapHashMap<WeakMember<NDEFReader>, uint32_t>;
  ReaderMap readers_;

  using WriterSet = HeapHashSet<WeakMember<NDEFReader>>;
  WriterSet writers_;

  HeapMojoRemote<device::mojom::blink::NFC> nfc_remote_;
  HeapMojoReceiver<device::mojom::blink::NFCClient, NFCProxy> client_receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NFC_PROXY_H_
