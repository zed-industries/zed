// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_MEDIA_RECORDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_MEDIA_RECORDER_H_

#include <memory>
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_media_recorder_options.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/mediarecorder/media_recorder_handler.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class Blob;
class BlobData;
enum class DOMExceptionCode;
class ExceptionState;
class V8BitrateMode;
class V8RecordingState;

class MODULES_EXPORT MediaRecorder
    : public EventTarget,
      public ActiveScriptWrappable<MediaRecorder>,
      public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum class State { kInactive = 0, kRecording, kPaused };

  static MediaRecorder* Create(ExecutionContext* context,
                               MediaStream* stream,
                               ExceptionState& exception_state);
  static MediaRecorder* Create(ExecutionContext* context,
                               MediaStream* stream,
                               const MediaRecorderOptions* options,
                               ExceptionState& exception_state);

  MediaRecorder(ExecutionContext* context,
                MediaStream* stream,
                const MediaRecorderOptions* options,
                ExceptionState& exception_state);
  ~MediaRecorder() override;

  MediaStream* stream() const { return stream_.Get(); }
  const String& mimeType() const { return mime_type_; }
  V8RecordingState state() const;
  uint32_t videoBitsPerSecond() const { return video_bits_per_second_; }
  uint32_t audioBitsPerSecond() const { return audio_bits_per_second_; }
  V8BitrateMode audioBitrateMode() const;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(start, kStart)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(stop, kStop)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(dataavailable, kDataavailable)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pause, kPause)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(resume, kResume)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)

  void start(ExceptionState& exception_state);
  void start(int time_slice, ExceptionState& exception_state);
  void stop(ExceptionState& exception_state);
  void pause(ExceptionState& exception_state);
  void resume(ExceptionState& exception_state);
  void requestData(ExceptionState& exception_state);

  static bool isTypeSupported(ExecutionContext* context, const String& type);

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  // ScriptWrappable
  bool HasPendingActivity() const final { return state_ != State::kInactive; }

  virtual void WriteData(base::span<const uint8_t> data,
                         bool last_in_slice,
                         ErrorEvent* error_event);
  virtual void OnError(DOMExceptionCode code, const String& message);

  // This causes an invalid modification error to be sent and recording to be
  // stopped if recording is not inactive.
  void OnStreamChanged(const String& message);

  // Causes recording to be stopped, remaining data to be written, and onstop to
  // be sent, unless recording isn't active in which case nothing happens.
  void OnAllTracksEnded();

  void Trace(Visitor* visitor) const override;

  void UpdateAudioBitrate(uint32_t bits_per_second);

 private:
  void CreateBlobEvent(Blob* blob);

  void StopRecording(ErrorEvent* error_event);
  void ScheduleDispatchEvent(Event* event);
  void DispatchScheduledEvent();

  Member<MediaStream> stream_;
  String mime_type_;
  uint32_t audio_bits_per_second_{0};
  uint32_t video_bits_per_second_{0};
  std::optional<uint32_t> overall_bits_per_second_;

  State state_ = State::kInactive;
  bool first_write_received_ = false;
  std::unique_ptr<BlobData> blob_data_;
  std::optional<base::TimeTicks> blob_event_first_chunk_timecode_;
  Member<MediaRecorderHandler> recorder_handler_;
  HeapVector<Member<Event>> scheduled_events_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_MEDIA_RECORDER_H_
