// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_TESTING_FAKE_FILE_BACKED_BLOB_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_TESTING_FAKE_FILE_BACKED_BLOB_FACTORY_H_

#include "mojo/public/cpp/bindings/self_owned_receiver.h"
#include "third_party/blink/public/mojom/blob/file_backed_blob_factory.mojom-blink.h"

namespace blink {

// Mocked FileBackedBlobFactory implementation used for testing. It simply keeps
// track of file backed blob registrations, binding each blob request to a
// FakeBlob instance with the correct uuid.
class FakeFileBackedBlobFactory : public mojom::blink::FileBackedBlobFactory {
 public:
  FakeFileBackedBlobFactory();
  ~FakeFileBackedBlobFactory() override;

  void RegisterBlob(mojo::PendingReceiver<mojom::blink::Blob> blob,
                    const String& uuid,
                    const String& content_type,
                    mojom::blink::DataElementFilePtr file) override;

  void RegisterBlobSync(mojo::PendingReceiver<mojom::blink::Blob> blob,
                        const String& uuid,
                        const String& content_type,
                        mojom::blink::DataElementFilePtr file,
                        RegisterBlobSyncCallback callback) override;

  struct Registration {
    String uuid;
    String content_type;
    mojom::blink::DataElementFilePtr file;
  };
  Vector<Registration> registrations;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_TESTING_FAKE_FILE_BACKED_BLOB_FACTORY_H_
