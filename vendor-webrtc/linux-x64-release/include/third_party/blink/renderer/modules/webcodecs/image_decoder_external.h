// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_IMAGE_DECODER_EXTERNAL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_IMAGE_DECODER_EXTERNAL_H_

#include <memory>

#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/image_decoder_core.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/weak_cell.h"
#include "third_party/blink/renderer/platform/image-decoders/image_decoder.h"
#include "third_party/blink/renderer/platform/loader/fetch/bytes_consumer.h"
#include "third_party/blink/renderer/platform/wtf/sequence_bound.h"

namespace blink {

class DOMException;
class ExceptionState;
class ScriptState;
class ImageDecodeOptions;
class ImageDecoderInit;
class ImageDecodeResult;
class ImageTrackList;
class ReadableStreamBytesConsumer;

class MODULES_EXPORT ImageDecoderExternal final
    : public ScriptWrappable,
      public ActiveScriptWrappable<ImageDecoderExternal>,
      public BytesConsumer::Client,
      public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ImageDecoderExternal* Create(ScriptState*,
                                      const ImageDecoderInit*,
                                      ExceptionState&);

  ImageDecoderExternal(ScriptState*, const ImageDecoderInit*, ExceptionState&);
  ~ImageDecoderExternal() override;

  static ScriptPromise<IDLBoolean> isTypeSupported(ScriptState*, String type);

  // image_decoder.idl implementation.
  ScriptPromise<ImageDecodeResult> decode(
      const ImageDecodeOptions* options = nullptr);
  void reset(DOMException* exception = nullptr);
  void close();
  String type() const;
  bool complete() const;
  ScriptPromise<IDLUndefined> completed(ScriptState* script_state);
  ImageTrackList& tracks() const;

  // BytesConsumer::Client implementation.
  void OnStateChange() override;
  String DebugName() const override;

  // GarbageCollected override.
  void Trace(Visitor*) const override;

  // ExecutionContextLifecycleObserver override.
  void ContextDestroyed() override;

  // ScriptWrappable override.
  bool HasPendingActivity() const override;

  // Called by ImageTrack to change the current track.
  void UpdateSelectedTrack();

 private:
  void MaybeSatisfyPendingDecodes();

  void OnDecodeReady(
      std::unique_ptr<ImageDecoderCore::ImageDecodeResult> result);

  void DecodeMetadata();
  void OnMetadata(ImageDecoderCore::ImageMetadata metadata);

  void SetFailed();
  void CloseInternal(DOMException*);

  Member<ScriptState> script_state_;

  // Used when a ReadableStream is provided.
  Member<ReadableStreamBytesConsumer> consumer_;
  size_t bytes_read_ = 0u;

  // Mime type provided at construction time. Cleared upon close().
  String mime_type_;

  // Copy of |preferAnimation| from |init_data_|.
  std::optional<bool> prefer_animation_;

  // Currently configured AnimationOption for |decoder_|.
  ImageDecoder::AnimationOption animation_option_ =
      ImageDecoder::AnimationOption::kUnspecified;

  // Set to true upon ImageDecodeCore::Decode() or
  // ImageDecoderCore::DecodeMetadata() failure. Once true, never cleared.
  bool failed_ = false;

  // Set to true either during construction or upon
  // ImageDecoderCore::DecodeMetadata() indicating it has received all data.
  // Once true, never cleared.
  bool data_complete_ = false;

  // Internal value used by OnStateChange() to ensure we don't Append() after
  // the ReadableStream becomes complete.
  bool internal_data_complete_ = false;

  // Set to true when close() has been called. Once set, never cleared.
  bool closed_ = false;

  // Number of ImageDecoderCore::Decode() calls in flight. Decremented during
  // OnDecodeReady() or zeroed out by reset() or close().
  size_t num_submitted_decodes_ = 0u;

  // Number of outstanding calls to DecodeMetadata(). Required to ensure the
  // class isn't destructed while we have outstanding WeakPtrs.
  int pending_metadata_requests_ = 0;

  // The workhorse which actually does the decoding. Bound to another sequence.
  scoped_refptr<base::SequencedTaskRunner> decode_task_runner_;
  std::unique_ptr<WTF::SequenceBound<ImageDecoderCore>> decoder_;

  // List of tracks in this image. Filled in during OnMetadata().
  Member<ImageTrackList> tracks_;

  // Set to true if we make it out of the constructor without an exception.
  bool construction_succeeded_ = false;

  // Pending decode() requests.
  struct DecodeRequest final : public GarbageCollected<DecodeRequest> {
    DecodeRequest(ScriptPromiseResolver<ImageDecodeResult>* resolver,
                  uint32_t frame_index,
                  bool complete_frames_only);
    ~DecodeRequest();
    void Trace(Visitor*) const;
    bool IsFinal() const;

    Member<ScriptPromiseResolver<ImageDecodeResult>> resolver;
    uint32_t frame_index;
    bool complete_frames_only;
    bool pending = false;
    std::optional<size_t> bytes_read_index;
    Member<ImageDecodeResult> result;
    std::unique_ptr<base::AtomicFlag> abort_flag;

    String range_error_message;
    Member<DOMException> exception;
  };
  HeapVector<Member<DecodeRequest>> pending_decodes_;

  using CompletedProperty = ScriptPromiseProperty<IDLUndefined, DOMException>;
  Member<CompletedProperty> completed_property_;

  // WeakPtrFactory used only for decode() requests. Invalidated upon decoding
  // errors or a call to reset().
  WeakCellFactory<ImageDecoderExternal> decode_weak_factory_{this};

  // WeakPtrFactory for all other cancelable tasks.
  WeakCellFactory<ImageDecoderExternal> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_IMAGE_DECODER_EXTERNAL_H_
