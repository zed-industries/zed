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

#pragma once

#include <memory>

#include "api/candidate.h"

namespace livekit_ffi {
class Candidate;
}
#include "webrtc-sys/src/candidate.rs.h"

// cricket::Candidate
namespace livekit_ffi {

class Candidate {
 public:
  explicit Candidate(const cricket::Candidate& candidate);

 private:
  cricket::Candidate candidate_;
};

static std::shared_ptr<Candidate> _shared_candidate() {
  return nullptr;
}

}  // namespace livekit_ffi
