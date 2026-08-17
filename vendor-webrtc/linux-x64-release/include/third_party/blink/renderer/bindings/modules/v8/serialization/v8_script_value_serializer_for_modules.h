// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_SERIALIZATION_V8_SCRIPT_VALUE_SERIALIZER_FOR_MODULES_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_SERIALIZATION_V8_SCRIPT_VALUE_SERIALIZER_FOR_MODULES_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/platform/web_crypto_algorithm.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/v8_script_value_serializer.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace media {
class AudioBuffer;
class DecoderBuffer;
}

namespace blink {

class CropTarget;
class FileSystemHandle;
class MediaSourceHandleImpl;
class RestrictionTarget;
class RTCEncodedAudioFrame;
class RTCEncodedVideoFrame;
class RTCDataChannel;
class VideoFrameHandle;
class WebCryptoKey;

// Extends V8ScriptValueSerializer with support for modules/ types.
class MODULES_EXPORT V8ScriptValueSerializerForModules final
    : public V8ScriptValueSerializer {
 public:
  // |object_index| is for use in exception messages.
  static bool ExtractTransferable(v8::Isolate*,
                                  v8::Local<v8::Value>,
                                  wtf_size_t object_index,
                                  Transferables&,
                                  ExceptionState&);

  explicit V8ScriptValueSerializerForModules(
      ScriptState* script_state,
      const SerializedScriptValue::SerializeOptions& options)
      : V8ScriptValueSerializer(script_state, options) {}

 protected:
  bool WriteDOMObject(ScriptWrappable*, ExceptionState&) override;

 private:
  void WriteOneByte(uint8_t byte) { WriteRawBytes(&byte, 1); }
  bool WriteCryptoKey(const WebCryptoKey&, ExceptionState&);
  bool WriteFileSystemHandle(SerializationTag tag,
                             FileSystemHandle* file_system_handle);
  bool WriteRTCEncodedAudioFrame(RTCEncodedAudioFrame*);
  bool WriteRTCEncodedVideoFrame(RTCEncodedVideoFrame*);
  bool WriteVideoFrameHandle(scoped_refptr<VideoFrameHandle>);
  bool WriteMediaAudioBuffer(scoped_refptr<media::AudioBuffer>);
  bool WriteDecoderBuffer(scoped_refptr<media::DecoderBuffer> data,
                          bool for_audio);
  bool WriteMediaStreamTrack(MediaStreamTrack* track,
                             ScriptWrappable::TypeDispatcher& dispatcher,
                             ExceptionState& exception_state);
  bool WriteCropTarget(CropTarget*);
  bool WriteRestrictionTarget(RestrictionTarget*);
  bool WriteMediaSourceHandle(MediaSourceHandleImpl* handle,
                              ExceptionState& exception_state);

  bool WriteRTCDataChannel(RTCDataChannel*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_SERIALIZATION_V8_SCRIPT_VALUE_SERIALIZER_FOR_MODULES_H_
