/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_DEVICE_FILE_AUDIO_DEVICE_FACTORY_H_
#define AUDIO_DEVICE_FILE_AUDIO_DEVICE_FACTORY_H_

#include <stdint.h>

#include "absl/strings/string_view.h"

namespace webrtc {

class FileAudioDevice;

// This class is used by audio_device_impl.cc when WebRTC is compiled with
// WEBRTC_DUMMY_FILE_DEVICES. The application must include this file and set the
// filenames to use before the audio device module is initialized. This is
// intended for test tools which use the audio device module.
class FileAudioDeviceFactory {
 public:
  static FileAudioDevice* CreateFileAudioDevice();

  // The input file must be a readable 48k stereo raw file. The output
  // file must be writable. The strings will be copied.
  static void SetFilenamesToUse(absl::string_view inputAudioFilename,
                                absl::string_view outputAudioFilename);

 private:
  enum : uint32_t { MAX_FILENAME_LEN = 512 };
  static bool _isConfigured;
  static char _inputAudioFilename[MAX_FILENAME_LEN];
  static char _outputAudioFilename[MAX_FILENAME_LEN];
};

}  // namespace webrtc

#endif  // AUDIO_DEVICE_FILE_AUDIO_DEVICE_FACTORY_H_
