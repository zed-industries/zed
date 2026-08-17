// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_RUST_ACCESSORS_ACCESSOR_GENERATOR_H__
#define GOOGLE_PROTOBUF_COMPILER_RUST_ACCESSORS_ACCESSOR_GENERATOR_H__

#include <memory>
#include <string>
#include <utility>

#include "absl/log/absl_check.h"
#include "google/protobuf/compiler/rust/accessors/accessor_case.h"
#include "google/protobuf/compiler/rust/context.h"
#include "google/protobuf/compiler/rust/naming.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace rust {

class AccessorGenerator {
 public:
  AccessorGenerator() = default;
  virtual ~AccessorGenerator() = default;

  AccessorGenerator(const AccessorGenerator&) = delete;
  AccessorGenerator(AccessorGenerator&&) = delete;
  AccessorGenerator& operator=(const AccessorGenerator&) = delete;
  AccessorGenerator& operator=(AccessorGenerator&&) = delete;

  // Constructs a generator for the given field.
  //
  // Returns `nullptr` if there is no known generator for this field.
  static std::unique_ptr<AccessorGenerator> For(Context& ctx,
                                                const FieldDescriptor& field);

  void GenerateMsgImpl(Context& ctx, const FieldDescriptor& field,
                       AccessorCase accessor_case) const {
    ctx.Emit({{"comment", FieldInfoComment(ctx, field)}}, R"rs(
      // $comment$
    )rs");
    InMsgImpl(ctx, field, accessor_case);
    ctx.printer().PrintRaw("\n");
  }
  void GenerateExternC(Context& ctx, const FieldDescriptor& field) const {
    ABSL_CHECK(ctx.is_cpp());
    InExternC(ctx, field);
    ctx.printer().PrintRaw("\n");
  }
  void GenerateThunkCc(Context& ctx, const FieldDescriptor& field) const {
    ABSL_CHECK(ctx.is_cpp());
    InThunkCc(ctx, field);
    ctx.printer().PrintRaw("\n");
  }

 private:
  // Note: the virtual functions are duplicated as non-virtual public functions,
  // so that we can customize prologue and epilogue behavior for these
  // functions. For example, consider calling `field.printer.WithVars()` as a
  // prologue to inject variables automatically.

  // Called inside the `impl Msg {}`, `impl MsgMut {}` and `impl MsgView`
  // blocks.
  virtual void InMsgImpl(Context& ctx, const FieldDescriptor& field,
                         AccessorCase accessor_case) const {}

  // Called inside of a message's `extern "C" {}` block.
  virtual void InExternC(Context& ctx, const FieldDescriptor& field) const {}

  // Called inside of an `extern "C" {}` block in the  `.thunk.cc` file, if such
  // a file is being generated.
  virtual void InThunkCc(Context& ctx, const FieldDescriptor& field) const {}
};

class SingularScalar final : public AccessorGenerator {
 public:
  ~SingularScalar() override = default;
  void InMsgImpl(Context& ctx, const FieldDescriptor& field,
                 AccessorCase accessor_case) const override;
  void InExternC(Context& ctx, const FieldDescriptor& field) const override;
  void InThunkCc(Context& ctx, const FieldDescriptor& field) const override;
};

class SingularString final : public AccessorGenerator {
 public:
  ~SingularString() override = default;
  void InMsgImpl(Context& ctx, const FieldDescriptor& field,
                 AccessorCase accessor_case) const override;
  void InExternC(Context& ctx, const FieldDescriptor& field) const override;
  void InThunkCc(Context& ctx, const FieldDescriptor& field) const override;
};

class SingularCord final : public AccessorGenerator {
 public:
  ~SingularCord() override = default;
  void InMsgImpl(Context& ctx, const FieldDescriptor& field,
                 AccessorCase accessor_case) const override;
  void InExternC(Context& ctx, const FieldDescriptor& field) const override;
  void InThunkCc(Context& ctx, const FieldDescriptor& field) const override;
};

class SingularMessage final : public AccessorGenerator {
 public:
  ~SingularMessage() override = default;
  void InMsgImpl(Context& ctx, const FieldDescriptor& field,
                 AccessorCase accessor_case) const override;
  void InExternC(Context& ctx, const FieldDescriptor& field) const override;
  void InThunkCc(Context& ctx, const FieldDescriptor& field) const override;
};

class RepeatedField final : public AccessorGenerator {
 public:
  ~RepeatedField() override = default;
  void InMsgImpl(Context& ctx, const FieldDescriptor& field,
                 AccessorCase accessor_case) const override;
  void InExternC(Context& ctx, const FieldDescriptor& field) const override;
  void InThunkCc(Context& ctx, const FieldDescriptor& field) const override;
};

class UnsupportedField final : public AccessorGenerator {
 public:
  ~UnsupportedField() override = default;
  void InMsgImpl(Context& ctx, const FieldDescriptor& field,
                 AccessorCase accessor_case) const override;
};

class Map final : public AccessorGenerator {
 public:
  ~Map() override = default;
  void InMsgImpl(Context& ctx, const FieldDescriptor& field,
                 AccessorCase accessor_case) const override;
  void InExternC(Context& ctx, const FieldDescriptor& field) const override;
  void InThunkCc(Context& ctx, const FieldDescriptor& field) const override;
};

}  // namespace rust
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_RUST_ACCESSORS_ACCESSOR_GENERATOR_H__
