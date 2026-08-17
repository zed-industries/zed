// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_TESTING_FAKE_BLOB_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_TESTING_FAKE_BLOB_REGISTRY_H_

#include "mojo/public/cpp/system/data_pipe_drainer.h"
#include "third_party/blink/public/mojom/blob/blob_registry.mojom-blink.h"

namespace blink {

// Mocked BlobRegistry implementation for testing. Simply keeps track of all
// blob registrations and blob lookup requests, binding each blob request to a
// FakeBlob instance with the correct uuid.
class FakeBlobRegistry : public mojom::blink::BlobRegistry {
 public:
  explicit FakeBlobRegistry(bool support_binary_blob_bodies = false);
  ~FakeBlobRegistry() override;

  struct Registration {
    String uuid;
    String content_type;
    String content_disposition;
    Vector<mojom::blink::DataElementPtr> elements;
  };
  Vector<Registration> registrations;

 private:
  void Register(mojo::PendingReceiver<mojom::blink::Blob>,
                const String& uuid,
                const String& content_type,
                const String& content_disposition,
                Vector<mojom::blink::DataElementPtr> elements,
                RegisterCallback) override;

  void RegisterFromStream(
      const String& content_type,
      const String& content_disposition,
      uint64_t expected_length,
      mojo::ScopedDataPipeConsumerHandle,
      mojo::PendingAssociatedRemote<mojom::blink::ProgressClient>,
      RegisterFromStreamCallback) override;

  class DataPipeDrainerClient;
  std::unique_ptr<DataPipeDrainerClient> drainer_client_;
  std::unique_ptr<mojo::DataPipeDrainer> drainer_;

  // Set to true to allow tests to read blobs that contain binary data.  Updates
  // `Register()` to create fake blobs with binary bodies.  Not implemented for
  // `RegisterFromStream()` or any other type of body content.
  bool support_binary_blob_bodies_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_TESTING_FAKE_BLOB_REGISTRY_H_
