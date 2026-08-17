// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_TESTING_FAKE_BLOB_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_TESTING_FAKE_BLOB_H_

#include "base/memory/raw_ptr.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/mojom/blob/blob.mojom-blink.h"

namespace blink {

// Mocked Blob implementation for testing. Implements all methods except for
// ReadRange and ReadSideData.
class FakeBlob : public mojom::blink::Blob {
 public:
  struct State {
    bool did_initiate_read_operation = false;
  };

  FakeBlob(const String& uuid,
           const String& body = String(),
           State* state = nullptr);
  FakeBlob(const String& uuid,
           const Vector<uint8_t>& body_bytes,
           State* state = nullptr);

  void Clone(mojo::PendingReceiver<mojom::blink::Blob>) override;
  void AsDataPipeGetter(
      mojo::PendingReceiver<network::mojom::blink::DataPipeGetter>) override;
  void ReadRange(uint64_t offset,
                 uint64_t length,
                 mojo::ScopedDataPipeProducerHandle,
                 mojo::PendingRemote<mojom::blink::BlobReaderClient>) override;
  void ReadAll(mojo::ScopedDataPipeProducerHandle,
               mojo::PendingRemote<mojom::blink::BlobReaderClient>) override;
  void Load(
      mojo::PendingReceiver<network::mojom::blink::URLLoader>,
      const String& method,
      const net::HttpRequestHeaders&,
      mojo::PendingRemote<network::mojom::blink::URLLoaderClient>) override;
  void ReadSideData(ReadSideDataCallback) override;
  void CaptureSnapshot(CaptureSnapshotCallback) override;
  void GetInternalUUID(GetInternalUUIDCallback) override;

 protected:
  String uuid_;
  Vector<uint8_t> body_;
  raw_ptr<State, DanglingUntriaged> state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_TESTING_FAKE_BLOB_H_
