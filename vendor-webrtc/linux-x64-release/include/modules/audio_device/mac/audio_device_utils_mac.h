/*
 * Copyright 2024 LiveKit
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

#ifndef MEDIA_AUDIO_MAC_CORE_AUDIO_UTIL_MAC_H_
#define MEDIA_AUDIO_MAC_CORE_AUDIO_UTIL_MAC_H_

#include <AudioUnit/AudioUnit.h>
#include <CoreAudio/CoreAudio.h>

#include <optional>
#include <string>
#include <vector>

namespace webrtc {
namespace mac_audio_utils {

std::vector<AudioObjectID> GetAllAudioDeviceIDs();

std::optional<AudioObjectID> GetDefaultOutputDeviceID();

std::optional<AudioObjectID> GetDefaultInputDeviceID();

std::vector<AudioObjectID> GetRelatedDeviceIDs(AudioObjectID device_id);

std::optional<std::string> GetDeviceUniqueID(AudioObjectID device_id);

std::optional<std::string> GetDeviceName(AudioObjectID device_id);

std::optional<std::string> GetDeviceLabel(AudioObjectID device_id,
                                          bool is_input);

uint32_t GetNumStreams(AudioObjectID device_id, bool is_input);

std::optional<uint32_t> GetDeviceSource(AudioObjectID device_id, bool is_input);

std::optional<uint32_t> GetDeviceTransportType(AudioObjectID device_id);

bool IsInputDevice(AudioObjectID device_id);

bool IsOutputDevice(AudioObjectID device_id);

}  // namespace mac_audio_utils
}  // namespace webrtc

#endif  // MEDIA_AUDIO_MAC_CORE_AUDIO_UTIL_MAC_H_
