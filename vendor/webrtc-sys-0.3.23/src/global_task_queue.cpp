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

#include "livekit/global_task_queue.h"

#include "api/task_queue/default_task_queue_factory.h"
#include "api/task_queue/task_queue_factory.h"

namespace livekit_ffi {

webrtc::TaskQueueFactory* GetGlobalTaskQueueFactory() {
  static std::unique_ptr<webrtc::TaskQueueFactory> global_task_queue_factory =
      webrtc::CreateDefaultTaskQueueFactory();
  return global_task_queue_factory.get();
}

}  // namespace livekit_ffi
