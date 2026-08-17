// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.!

#ifndef CHAR_MODEL_TRAINER_H_
#define CHAR_MODEL_TRAINER_H_

#include "sentencepiece_model.pb.h"
#include "trainer_interface.h"

namespace sentencepiece {
namespace character {

// Trainer class for character model.
class Trainer : public TrainerInterface {
 public:
  Trainer(const TrainerSpec& trainer_spec,
          const NormalizerSpec& normalizer_spec,
          const NormalizerSpec& denormalizer_spec)
      : TrainerInterface::TrainerInterface(trainer_spec,
                                           normalizer_spec,
                                           denormalizer_spec) {}

  util::Status Train() override;
};
}  // namespace character
}  // namespace sentencepiece
#endif  // CHAR_MODEL_TRAINER_H_
