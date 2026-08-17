// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TEST_FAKE_IMAGE_CAPTURER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TEST_FAKE_IMAGE_CAPTURER_H_

#include "base/memory/weak_ptr.h"
#include "media/capture/mojom/image_capture.mojom-blink.h"
#include "mojo/public/cpp/bindings/receiver_set.h"

namespace blink {
class ExecutionContext;

void BindFakeImageCapturer(ExecutionContext* context);

class FakeImageCapture : public media::mojom::blink::ImageCapture {
 public:
  void RegisterBinding(ExecutionContext* context);

  void Bind(mojo::ScopedMessagePipeHandle handle);

  void GetPhotoState(const WTF::String& source_id,
                     GetPhotoStateCallback callback) override;

  void SetPhotoOptions(const WTF::String& source_id,
                       media::mojom::blink::PhotoSettingsPtr settings,
                       SetPhotoOptionsCallback callback) override {}

  void TakePhoto(const WTF::String& source_id,
                 TakePhotoCallback callback) override {}

 private:
  mojo::ReceiverSet<media::mojom::blink::ImageCapture> receivers_;
  base::WeakPtrFactory<FakeImageCapture> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TEST_FAKE_IMAGE_CAPTURER_H_
