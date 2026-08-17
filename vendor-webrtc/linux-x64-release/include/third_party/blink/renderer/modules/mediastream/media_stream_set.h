// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_SET_H_

#include "third_party/blink/renderer/modules/mediastream/media_stream.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

enum class UserMediaRequestType;

using MediaStreamSetInitializedCallback =
    base::OnceCallback<void(MediaStreamVector)>;

class MODULES_EXPORT MediaStreamSet final
    : public GarbageCollected<MediaStreamSet>,
      public ExecutionContextClient {
 public:
  static MediaStreamSet* Create(
      ExecutionContext* context,
      const GCedMediaStreamDescriptorVector& stream_descriptors,
      UserMediaRequestType request_type,
      MediaStreamSetInitializedCallback callback);

  MediaStreamSet(ExecutionContext* context,
                 const GCedMediaStreamDescriptorVector& stream_descriptors,
                 UserMediaRequestType request_type,
                 MediaStreamSetInitializedCallback callback);
  ~MediaStreamSet() = default;

  void Trace(Visitor*) const override;

 private:
  void InitializeGetAllScreensMediaStreams(
      ExecutionContext* context,
      const GCedMediaStreamDescriptorVector& stream_descriptors);
  void OnMediaStreamInitialized(MediaStream*);
  void OnMediaStreamSetInitialized();

  const size_t media_streams_to_initialize_count_;
  MediaStreamVector initialized_media_streams_;
  MediaStreamSetInitializedCallback media_streams_initialized_callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_SET_H_
