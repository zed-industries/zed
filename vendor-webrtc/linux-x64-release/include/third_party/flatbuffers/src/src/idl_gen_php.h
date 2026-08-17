/*
 * Copyright 2023 Google Inc. All rights reserved.
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

#ifndef FLATBUFFERS_IDL_GEN_PHP_H_
#define FLATBUFFERS_IDL_GEN_PHP_H_

#include "flatbuffers/code_generator.h"

namespace flatbuffers {

// Constructs a new Php code generator.
std::unique_ptr<CodeGenerator> NewPhpCodeGenerator();

}  // namespace flatbuffers

#endif  // FLATBUFFERS_IDL_GEN_PHP_H_
