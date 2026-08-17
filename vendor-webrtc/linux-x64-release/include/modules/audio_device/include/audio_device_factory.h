/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_INCLUDE_AUDIO_DEVICE_FACTORY_H_
#define MODULES_AUDIO_DEVICE_INCLUDE_AUDIO_DEVICE_FACTORY_H_

#include <memory>

#include "api/audio/audio_device.h"
#include "api/task_queue/task_queue_factory.h"

namespace webrtc {

// Creates an AudioDeviceModule (ADM) for Windows based on the Core Audio API.
// The creating thread must be a COM thread; otherwise nullptr will be returned.
// By default `automatic_restart` is set to true and it results in support for
// automatic restart of audio if e.g. the existing device is removed. If set to
// false, no attempt to restart audio is performed under these conditions.
//
// Example (assuming webrtc namespace):
//
//  public:
//   webrtc::scoped_refptr<AudioDeviceModule> CreateAudioDevice() {
//     task_queue_factory_ = CreateDefaultTaskQueueFactory();
//     // Tell COM that this thread shall live in the MTA.
//     com_initializer_ = std::make_unique<ScopedCOMInitializer>(
//         ScopedCOMInitializer::kMTA);
//     if (!com_initializer_->Succeeded()) {
//       return nullptr;
//     }
//     // Create the ADM with support for automatic restart if devices are
//     // unplugged.
//     return CreateWindowsCoreAudioAudioDeviceModule(
//         task_queue_factory_.get());
//   }
//
//   private:
//    std::unique_ptr<ScopedCOMInitializer> com_initializer_;
//    std::unique_ptr<TaskQueueFactory> task_queue_factory_;
//
webrtc::scoped_refptr<AudioDeviceModule>
CreateWindowsCoreAudioAudioDeviceModule(TaskQueueFactory* task_queue_factory,
                                        bool automatic_restart = true);

webrtc::scoped_refptr<AudioDeviceModuleForTest>
CreateWindowsCoreAudioAudioDeviceModuleForTest(
    TaskQueueFactory* task_queue_factory,
    bool automatic_restart = true);

}  // namespace webrtc

#endif  //  MODULES_AUDIO_DEVICE_INCLUDE_AUDIO_DEVICE_FACTORY_H_
