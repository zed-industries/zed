// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_IMMUTABLE_MAKE_FIELD_GENS_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_IMMUTABLE_MAKE_FIELD_GENS_H__

#include "google/protobuf/compiler/java/context.h"
#include "google/protobuf/compiler/java/generator_common.h"
#include "google/protobuf/compiler/java/full/field_generator.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

FieldGeneratorMap<ImmutableFieldGenerator> MakeImmutableFieldGenerators(
    const Descriptor* descriptor, Context* context);

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_IMMUTABLE_MAKE_FIELD_GENS_H__
