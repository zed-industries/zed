// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Calculator registration.

#ifndef MEDIAPIPE_FRAMEWORK_CALCULATOR_REGISTRY_H_
#define MEDIAPIPE_FRAMEWORK_CALCULATOR_REGISTRY_H_

#include <memory>

#include "mediapipe/framework/calculator_base.h"

// Macro for registering calculators.
#define REGISTER_CALCULATOR(name)                                       \
  REGISTER_FACTORY_FUNCTION_QUALIFIED(                                  \
      mediapipe::CalculatorBaseRegistry, calculator_registration, name, \
      std::make_unique<mediapipe::internal::CalculatorBaseFactoryFor<name>>)

#endif  // MEDIAPIPE_FRAMEWORK_CALCULATOR_REGISTRY_H_
