// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_CPP_FIELD_GENERATORS_GENERATORS_H__
#define GOOGLE_PROTOBUF_COMPILER_CPP_FIELD_GENERATORS_GENERATORS_H__

#include <memory>

#include "google/protobuf/compiler/cpp/field.h"
#include "google/protobuf/compiler/cpp/helpers.h"
#include "google/protobuf/compiler/cpp/options.h"
#include "google/protobuf/descriptor.h"

// The functions in this file construct FieldGeneratorBase objects for
// generating different "codegen types" of fields. The logic for selecting the
// correct choice of generator lives in compiler/cpp/field.cc; this is merely
// the API that file uses for constructing generators.
//
// Functions are of the form `Make<card><kind>Generator()`, where <card> is
// `Singular`, `Repeated`, or `Oneof`, and <kind> is the field type, plus
// `MakeMapGenerator()`, since map fields are always repeated message fields.
//
// The returned pointers are never null.

namespace google {
namespace protobuf {
namespace compiler {
namespace cpp {
std::unique_ptr<FieldGeneratorBase> MakeSinguarPrimitiveGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeRepeatedPrimitiveGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeSinguarEnumGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeRepeatedEnumGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeSinguarStringGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeRepeatedStringGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeSingularStringViewGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeRepeatedStringViewGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeSinguarMessageGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeRepeatedMessageGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeOneofMessageGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeMapGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeSingularCordGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

std::unique_ptr<FieldGeneratorBase> MakeOneofCordGenerator(
    const FieldDescriptor* desc, const Options& options,
    MessageSCCAnalyzer* scc);

}  // namespace cpp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_FIELD_GENERATORS_GENERATORS_H__
