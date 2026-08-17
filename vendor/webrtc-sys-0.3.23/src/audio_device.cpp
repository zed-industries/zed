/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "livekit/audio_device.h"

const int kSampleRate = 48000;
const int kChannels = 2;
const int kBytesPerSample = kChannels * sizeof(int16_t);
const int kSamplesPer10Ms = kSampleRate / 100;

namespace livekit_ffi {

AudioDevice::AudioDevice(webrtc::TaskQueueFactory* task_queue_factory)
    : task_queue_factory_(task_queue_factory),
      data_(kSamplesPer10Ms * kChannels) {}

AudioDevice::~AudioDevice() {
  Terminate();
}

int32_t AudioDevice::ActiveAudioLayer(AudioLayer* audioLayer) const {
  *audioLayer = AudioLayer::kDummyAudio;
  return 0;
}

int32_t AudioDevice::RegisterAudioCallback(webrtc::AudioTransport* transport) {
  webrtc::MutexLock lock(&mutex_);
  audio_transport_ = transport;
  return 0;
}

int32_t AudioDevice::Init() {
  webrtc::MutexLock lock(&mutex_);
  if (initialized_)
    return 0;

  audio_queue_ =
      task_queue_factory_->CreateTaskQueue(
          "AudioDevice", webrtc::TaskQueueFactory::Priority::NORMAL);

  audio_task_ =
      webrtc::RepeatingTaskHandle::Start(audio_queue_.get(), [this]() {
        webrtc::MutexLock lock(&mutex_);

        if (playing_) {
          int64_t elapsed_time_ms = -1;
          int64_t ntp_time_ms = -1;
          size_t n_samples_out = 0;
          void* data = data_.data();

          // Request the AudioData, otherwise WebRTC will ignore the packets.
          // 10ms of audio data.
          audio_transport_->NeedMorePlayData(
              kSamplesPer10Ms, kBytesPerSample, kChannels, kSampleRate, data,
              n_samples_out, &elapsed_time_ms, &ntp_time_ms);
        }

        return webrtc::TimeDelta::Millis(10);
      });

  initialized_ = true;
  return 0;
}

int32_t AudioDevice::Terminate() {
  {
    webrtc::MutexLock lock(&mutex_);
    if (!initialized_)
      return 0;

    initialized_ = false;
  }
  audio_queue_ = nullptr;
  return 0;
}

bool AudioDevice::Initialized() const {
  webrtc::MutexLock lock(&mutex_);
  return initialized_;
}

int16_t AudioDevice::PlayoutDevices() {
  return 0;
}

int16_t AudioDevice::RecordingDevices() {
  return 0;
}

int32_t AudioDevice::PlayoutDeviceName(uint16_t index,
                                       char name[webrtc::kAdmMaxDeviceNameSize],
                                       char guid[webrtc::kAdmMaxGuidSize]) {
  return 0;
}

int32_t AudioDevice::RecordingDeviceName(
    uint16_t index,
    char name[webrtc::kAdmMaxDeviceNameSize],
    char guid[webrtc::kAdmMaxGuidSize]) {
  return 0;
}

int32_t AudioDevice::SetPlayoutDevice(uint16_t index) {
  return 0;
}

int32_t AudioDevice::SetPlayoutDevice(WindowsDeviceType device) {
  return 0;
}

int32_t AudioDevice::SetRecordingDevice(uint16_t index) {
  return 0;
}

int32_t AudioDevice::SetRecordingDevice(WindowsDeviceType device) {
  return 0;
}

int32_t AudioDevice::PlayoutIsAvailable(bool* available) {
  return 0;
}

int32_t AudioDevice::InitPlayout() {
  return 0;
}

bool AudioDevice::PlayoutIsInitialized() const {
  return false;
}

int32_t AudioDevice::RecordingIsAvailable(bool* available) {
  return 0;
}

int32_t AudioDevice::InitRecording() {
  return 0;
}

bool AudioDevice::RecordingIsInitialized() const {
  return false;
}

int32_t AudioDevice::StartPlayout() {
  webrtc::MutexLock lock(&mutex_);
  playing_ = true;
  return 0;
}

int32_t AudioDevice::StopPlayout() {
  webrtc::MutexLock lock(&mutex_);
  playing_ = false;
  return 0;
}

bool AudioDevice::Playing() const {
  webrtc::MutexLock lock(&mutex_);
  return playing_;
}

int32_t AudioDevice::StartRecording() {
  return 0;
}

int32_t AudioDevice::StopRecording() {
  return 0;
}

bool AudioDevice::Recording() const {
  return false;
}

int32_t AudioDevice::InitSpeaker() {
  return 0;
}

bool AudioDevice::SpeakerIsInitialized() const {
  return false;
}

int32_t AudioDevice::InitMicrophone() {
  return 0;
}

bool AudioDevice::MicrophoneIsInitialized() const {
  return false;
}

int32_t AudioDevice::SpeakerVolumeIsAvailable(bool* available) {
  return 0;
}

int32_t AudioDevice::SetSpeakerVolume(uint32_t volume) {
  return 0;
}

int32_t AudioDevice::SpeakerVolume(uint32_t* volume) const {
  return 0;
}

int32_t AudioDevice::MaxSpeakerVolume(uint32_t* maxVolume) const {
  return 0;
}

int32_t AudioDevice::MinSpeakerVolume(uint32_t* minVolume) const {
  return 0;
}

int32_t AudioDevice::MicrophoneVolumeIsAvailable(bool* available) {
  return 0;
}

int32_t AudioDevice::SetMicrophoneVolume(uint32_t volume) {
  return 0;
}

int32_t AudioDevice::MicrophoneVolume(uint32_t* volume) const {
  return 0;
}

int32_t AudioDevice::MaxMicrophoneVolume(uint32_t* maxVolume) const {
  return 0;
}

int32_t AudioDevice::MinMicrophoneVolume(uint32_t* minVolume) const {
  return 0;
}

int32_t AudioDevice::SpeakerMuteIsAvailable(bool* available) {
  return 0;
}

int32_t AudioDevice::SetSpeakerMute(bool enable) {
  return 0;
}

int32_t AudioDevice::SpeakerMute(bool* enabled) const {
  return 0;
}

int32_t AudioDevice::MicrophoneMuteIsAvailable(bool* available) {
  return 0;
}

int32_t AudioDevice::SetMicrophoneMute(bool enable) {
  return 0;
}

int32_t AudioDevice::MicrophoneMute(bool* enabled) const {
  return 0;
}

int32_t AudioDevice::StereoPlayoutIsAvailable(bool* available) const {
  *available = true;
  return 0;
}

int32_t AudioDevice::SetStereoPlayout(bool enable) {
  return 0;
}

int32_t AudioDevice::StereoPlayout(bool* enabled) const {
  return 0;
}

int32_t AudioDevice::StereoRecordingIsAvailable(bool* available) const {
  return 0;
}

int32_t AudioDevice::SetStereoRecording(bool enable) {
  return 0;
}

int32_t AudioDevice::StereoRecording(bool* enabled) const {
  *enabled = true;
  return 0;
}

int32_t AudioDevice::PlayoutDelay(uint16_t* delayMS) const {
  return 0;
}

bool AudioDevice::BuiltInAECIsAvailable() const {
  return false;
}

bool AudioDevice::BuiltInAGCIsAvailable() const {
  return false;
}

bool AudioDevice::BuiltInNSIsAvailable() const {
  return false;
}

int32_t AudioDevice::EnableBuiltInAEC(bool enable) {
  return 0;
}

int32_t AudioDevice::EnableBuiltInAGC(bool enable) {
  return 0;
}

int32_t AudioDevice::EnableBuiltInNS(bool enable) {
  return 0;
}

#if defined(WEBRTC_IOS)
int AudioDevice::GetPlayoutAudioParameters(
    webrtc::AudioParameters* params) const {
  return 0;
}

int AudioDevice::GetRecordAudioParameters(
    webrtc::AudioParameters* params) const {
  return 0;
}
#endif  // WEBRTC_IOS

int32_t AudioDevice::SetObserver(webrtc::AudioDeviceObserver* observer) {
  return 0;
}

}  // namespace livekit_ffi
