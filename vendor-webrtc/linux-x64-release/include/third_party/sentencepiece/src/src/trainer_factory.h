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

#ifndef TRAINER_FACTORY_H_
#define TRAINER_FACTORY_H_

#include <memory>

#include "sentencepiece_model.pb.h"
#include "trainer_interface.h"

namespace sentencepiece {

class TrainerFactory {
 public:
  // Creates Trainer instance from |trainer_spec| and |normalizer_spec|.
  static std::unique_ptr<TrainerInterface> Create(
      const TrainerSpec& trainer_spec,
      const NormalizerSpec& normalizer_spec,
      const NormalizerSpec& denormalizer_spec);
};
}  // namespace sentencepiece
#endif  // TRAINER_FACTORY_H_
