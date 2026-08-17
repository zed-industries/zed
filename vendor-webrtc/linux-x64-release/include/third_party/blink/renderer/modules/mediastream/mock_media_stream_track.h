// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_TRACK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_TRACK_H_

#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_media_stream_track_state.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_media_track_capabilities.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_media_track_constraints.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_media_track_settings.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_track.h"

namespace blink {

class DOMException;

class MockMediaStreamTrack : public blink::MediaStreamTrack {
 public:
  String kind() const override { return kind_; }
  void SetKind(const String& kind) { kind_ = kind; }

  String id() const override { return id_; }
  void SetId(const String& id) { id_ = id; }

  String label() const override { return label_; }
  void SetLabel(const String& label) { label_ = label; }

  bool enabled() const override { return enabled_; }
  void setEnabled(bool enabled) override { enabled_ = enabled; }

  bool muted() const override { return muted_; }
  void SetMuted(bool muted) { muted_ = muted; }

  String ContentHint() const override { return content_hint_; }
  void SetContentHint(const String& content_hint) override {
    content_hint_ = content_hint;
  }
  V8MediaStreamTrackState readyState() const override { return ready_state_; }
  void SetReadyState(V8MediaStreamTrackState::Enum ready_state) {
    ready_state_ = V8MediaStreamTrackState(ready_state);
  }

  MediaTrackCapabilities* getCapabilities() const override {
    return capabilities_.Get();
  }
  void SetCapabilities(MediaTrackCapabilities* capabilities) {
    capabilities_ = capabilities;
  }

  MediaTrackConstraints* getConstraints() const override {
    return constraints_.Get();
  }
  void SetConstraints(MediaTrackConstraints* constraints) {
    constraints_ = constraints;
  }
  ScriptPromise<IDLUndefined> applyConstraints(
      ScriptState* state,
      const MediaTrackConstraints* constraints) override {
    return applyConstraintsScriptState(state, constraints);
  }
  void applyConstraints(ScriptPromiseResolver<IDLUndefined>* resolver,
                        const MediaTrackConstraints* constraints) override {
    applyConstraintsResolver(resolver, constraints);
  }

  MediaTrackSettings* getSettings() const override { return settings_.Get(); }
  void SetSettings(MediaTrackSettings* settings) { settings_ = settings; }

  CaptureHandle* getCaptureHandle() const override {
    return capture_handle_.Get();
  }
  void SetCaptureHandle(CaptureHandle* capture_handle) {
    capture_handle_ = capture_handle;
  }

  MediaStreamSource::ReadyState GetReadyState() override {
    return ready_state_enum_;
  }
  void SetReadyState(MediaStreamSource::ReadyState ready_state_enum) {
    ready_state_enum_ = ready_state_enum;
  }

  MediaStreamComponent* Component() const override { return component_.Get(); }
  void SetComponent(MediaStreamComponent* component) { component_ = component; }

  bool Ended() const override { return ended_; }
  void SetEnded(bool ended) { ended_ = ended; }

  const AtomicString& InterfaceName() const override;

  ExecutionContext* GetExecutionContext() const override {
    return context_.Get();
  }
  void SetExecutionContext(ExecutionContext* context) { context_ = context; }

  bool HasPendingActivity() const override { return false; }

  std::unique_ptr<AudioSourceProvider> CreateWebAudioSource(
      int context_sample_rate,
      base::TimeDelta platform_buffer_duration) override {
    return nullptr;
  }

  ImageCapture* GetImageCapture() override { return nullptr; }

  std::optional<const MediaStreamDevice> device() const override {
    return device_;
  }
  void SetDevice(const MediaStreamDevice& device) { device_ = device; }

  MOCK_METHOD1(stopTrack, void(ExecutionContext*));
  MOCK_METHOD1(clone, MediaStreamTrack*(ExecutionContext*));
  MOCK_METHOD0(
      stats,
      V8UnionMediaStreamTrackAudioStatsOrMediaStreamTrackVideoStats*());
  MOCK_METHOD2(applyConstraintsScriptState,
               ScriptPromise<IDLUndefined>(ScriptState*,
                                           const MediaTrackConstraints*));
  MOCK_METHOD2(applyConstraintsResolver,
               void(ScriptPromiseResolver<IDLUndefined>*,
                    const MediaTrackConstraints*));
  MOCK_METHOD1(SetInitialConstraints, void(const MediaConstraints&));
  MOCK_METHOD1(SetConstraints, void(const MediaConstraints&));
  MOCK_METHOD1(RegisterMediaStream, void(MediaStream*));
  MOCK_METHOD1(UnregisterMediaStream, void(MediaStream*));
  MOCK_METHOD1(RegisterSink, void(SpeechRecognitionMediaStreamAudioSink*));
  MOCK_METHOD2(AddedEventListener,
               void(const AtomicString&, RegisteredEventListener&));
  MOCK_METHOD1(BeingTransferred, void(const base::UnguessableToken&));
  MOCK_CONST_METHOD1(TransferAllowed, bool(String&));

#if !BUILDFLAG(IS_ANDROID)
  MOCK_METHOD5(
      SendWheel,
      void(double, double, int, int, base::OnceCallback<void(DOMException*)>));
  MOCK_METHOD1(
      GetZoomLevel,
      void(base::OnceCallback<void(std::optional<int>, const String&)>));
  MOCK_METHOD0(CloseFocusWindowOfOpportunity, void());
  MOCK_METHOD2(SetZoomLevel,
               void(int, base::OnceCallback<void(DOMException*)>));
#endif

  MOCK_METHOD1(AddObserver, void(Observer*));

  void Trace(Visitor* visitor) const override {
    MediaStreamTrack::Trace(visitor);
    visitor->Trace(capabilities_);
    visitor->Trace(constraints_);
    visitor->Trace(settings_);
    visitor->Trace(capture_handle_);
    visitor->Trace(component_);
    visitor->Trace(context_);
  }

 private:
  String kind_;
  String id_;
  String label_;
  bool enabled_;
  bool muted_;
  String content_hint_;
  V8MediaStreamTrackState ready_state_ =
      V8MediaStreamTrackState(V8MediaStreamTrackState::Enum::kLive);
  Member<MediaTrackCapabilities> capabilities_;
  Member<MediaTrackConstraints> constraints_;
  Member<MediaTrackSettings> settings_;
  Member<CaptureHandle> capture_handle_;
  MediaStreamSource::ReadyState ready_state_enum_;
  Member<MediaStreamComponent> component_;
  bool ended_;
  std::optional<MediaStreamDevice> device_;
  WeakMember<ExecutionContext> context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_TRACK_H_
