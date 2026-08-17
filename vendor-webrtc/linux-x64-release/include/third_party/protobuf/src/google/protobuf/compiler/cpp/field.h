// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_CPP_FIELD_H__
#define GOOGLE_PROTOBUF_COMPILER_CPP_FIELD_H__

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/log/absl_check.h"
#include "absl/strings/string_view.h"
#include "absl/types/optional.h"
#include "absl/types/span.h"
#include "google/protobuf/compiler/cpp/helpers.h"
#include "google/protobuf/compiler/cpp/options.h"
#include "google/protobuf/cpp_features.pb.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace cpp {

// Customization points for each field codegen type. See FieldGenerator to
// see how each of these functions is used.
//
// TODO: Make every function except the dtor in this generator
// non-pure-virtual. A generator with no implementation should be able to
// automatically not contribute any code to the message it is part of, as a
// matter of clean composability.
class FieldGeneratorBase {
 public:
  // `GeneratorFunction` defines a subset of generator functions that may have
  // additional optimizations or requirements such as 'uses a local `arena`
  // variable instead of calling GetArena()'
  enum class GeneratorFunction { kMergeFrom };

  FieldGeneratorBase(const FieldDescriptor* field, const Options& options,
                     MessageSCCAnalyzer* scc_analyzer);

  FieldGeneratorBase(const FieldGeneratorBase&) = delete;
  FieldGeneratorBase& operator=(const FieldGeneratorBase&) = delete;

  virtual ~FieldGeneratorBase() = 0;

  // Returns true if this field should be placed in the cold 'Split' section.
  bool should_split() const { return should_split_; }

  // Returns true if this field is trivial. (int, float, double, enum, bool)
  bool is_trivial() const { return is_trivial_; }

  // Returns true if the field value itself is trivial, i.e., the field is
  // trivial, or a (raw) pointer value to a singular, non lazy message.
  bool has_trivial_value() const { return has_trivial_value_; }

  // Returns true if the provided field has a trivial zero default.
  // I.e., the field can be initialized with `memset(&field, 0, sizeof(field))`
  bool has_trivial_zero_default() const { return has_trivial_zero_default_; }

  // Returns true if the provided field can be initialized with `= {}`.
  bool has_brace_default_assign() const { return has_brace_default_assign_; }

  // Returns true if the field is a singular or repeated message.
  // This includes group message types. To explicitly check if a message
  // type is a group type, use the `is_group()` function,
  bool is_message() const { return is_message_; }

  // Returns true if the field is a group message field (TYPE_GROUP).
  bool is_group() const { return is_group_; }

  // Returns true if the field is a weak message
  bool is_weak() const { return is_weak_; }

  // Returns true if the field is a lazy message.
  bool is_lazy() const { return is_lazy_; }

  // Returns true if the field is a foreign message field.
  bool is_foreign() const { return is_foreign_; }

  // Returns true if the field is a string field.
  bool is_string() const { return is_string_; }

  // Returns true if the field API uses bytes (void) instead of chars.
  bool is_bytes() const { return is_bytes_; }

  // Returns true if this field is part of a oneof field.
  bool is_oneof() const { return is_oneof_; }

  // Returns true if the field should be inlined instead of dynamically
  // allocated. Applies to string and message value.
  bool is_inlined() const { return is_inlined_; }

  // Returns true if this field has an appropriate default constexpr
  // constructor. i.e., there is no need for an explicit initializer.
  bool has_default_constexpr_constructor() const {
    return has_default_constexpr_constructor_;
  }

  // Returns true if this generator requires an 'arena' parameter on the
  // given generator function.
  virtual bool RequiresArena(GeneratorFunction) const { return false; }

  virtual std::vector<io::Printer::Sub> MakeVars() const { return {}; }

  virtual void GeneratePrivateMembers(io::Printer* p) const = 0;

  virtual void GenerateStaticMembers(io::Printer* p) const {}

  virtual void GenerateAccessorDeclarations(io::Printer* p) const = 0;

  virtual void GenerateInlineAccessorDefinitions(io::Printer* p) const = 0;

  virtual void GenerateNonInlineAccessorDefinitions(io::Printer* p) const {}

  virtual void GenerateClearingCode(io::Printer* p) const = 0;

  virtual void GenerateMessageClearingCode(io::Printer* p) const {
    GenerateClearingCode(p);
  }

  virtual void GenerateMergingCode(io::Printer* p) const = 0;

  virtual void GenerateCopyConstructorCode(io::Printer* p) const;

  virtual void GenerateSwappingCode(io::Printer* p) const = 0;

  virtual void GenerateConstructorCode(io::Printer* p) const = 0;

  virtual void GenerateDestructorCode(io::Printer* p) const {}

  virtual void GenerateArenaDestructorCode(io::Printer* p) const {
    ABSL_CHECK(NeedsArenaDestructor() == ArenaDtorNeeds::kNone)
        << field_->cpp_type_name();
  }

  // Generates constexpr member initialization code, e.g.: `foo_{5}`.
  // The default implementation generates the following code:
  // - repeated fields and maps: `field_{}`
  // - all other fields: `field_{<default value>}`
  virtual void GenerateMemberConstexprConstructor(io::Printer* p) const;

  // Generates member initialization code, e.g.: `foo_(5)`.
  // The default implementation generates the following code:
  // - repeated fields and maps: `field_{visibility, arena}`
  // - split repeated fields (RawPtr): `field_{}`
  // - all other fields: `field_{<default value>}`
  virtual void GenerateMemberConstructor(io::Printer* p) const;

  // Generates member copy initialization code, e.g.: `foo_(5)`.
  // The default implementation generates the following code:
  // - repeated fields and maps: `field_{visibility, arena, from.field_}`
  // - all other fields: `field_{from.field_}`
  virtual void GenerateMemberCopyConstructor(io::Printer* p) const;

  // Generates 'placement new' copy construction code used to
  // explicitly copy initialize oneof field values.
  // The default implementation checks the current field to not be repeated,
  // an extension or a map, and generates the following code:
  // - `field_ = from.field_`
  virtual void GenerateOneofCopyConstruct(io::Printer* p) const;

  virtual void GenerateAggregateInitializer(io::Printer* p) const;

  virtual void GenerateConstexprAggregateInitializer(io::Printer* p) const;

  virtual void GenerateCopyAggregateInitializer(io::Printer* p) const;

  virtual void GenerateSerializeWithCachedSizesToArray(
      io::Printer* p) const = 0;

  virtual void GenerateByteSize(io::Printer* p) const = 0;

  virtual void GenerateIsInitialized(io::Printer* p) const {
    ABSL_CHECK(!NeedsIsInitialized());
  }
  virtual bool NeedsIsInitialized() const { return false; }

  virtual bool IsInlined() const { return false; }

  virtual ArenaDtorNeeds NeedsArenaDestructor() const {
    return ArenaDtorNeeds::kNone;
  }

 protected:
  const FieldDescriptor* field_;
  const Options& options_;
  MessageSCCAnalyzer* scc_;
  absl::flat_hash_map<absl::string_view, std::string> variables_;

  pb::CppFeatures::StringType GetDeclaredStringType() const;

 private:
  bool should_split_ = false;
  bool is_trivial_ = false;
  bool has_trivial_value_ = false;
  bool has_trivial_zero_default_ = false;
  bool has_brace_default_assign_ = false;
  bool is_message_ = false;
  bool is_group_ = false;
  bool is_string_ = false;
  bool is_bytes_ = false;
  bool is_inlined_ = false;
  bool is_foreign_ = false;
  bool is_lazy_ = false;
  bool is_weak_ = false;
  bool is_oneof_ = false;
  bool has_default_constexpr_constructor_ = false;
};

inline FieldGeneratorBase::~FieldGeneratorBase() = default;

class FieldGenerator {
 private:
  // This function must be defined here so that the inline definitions below
  // can see it, which is required because it has deduced return type.
  auto PushVarsForCall(io::Printer* p) const {
    // NOTE: we use a struct here because:
    // * We want to ensure that order of evaluation below is well-defined,
    //   which {...} guarantees but (...) does not.
    // * We do not require C++17 as of writing and therefore cannot use
    //   std::tuple with CTAD.
    // * std::make_tuple uses (...), not {...}.
    struct Vars {
      decltype(p->WithVars(field_vars_)) cleanup1;
      decltype(p->WithVars(tracker_vars_)) cleanup2;
      decltype(p->WithVars(per_generator_vars_)) cleanup3;
    };

    return Vars{p->WithVars(field_vars_), p->WithVars(tracker_vars_),
                p->WithVars(per_generator_vars_)};
  }

 public:
  using GeneratorFunction = FieldGeneratorBase::GeneratorFunction;

  FieldGenerator(const FieldGenerator&) = delete;
  FieldGenerator& operator=(const FieldGenerator&) = delete;
  FieldGenerator(FieldGenerator&&) = default;
  FieldGenerator& operator=(FieldGenerator&&) = default;

  // Properties: see FieldGeneratorBase for documentation
  bool should_split() const { return impl_->should_split(); }
  bool is_trivial() const { return impl_->is_trivial(); }
  // Returns true if the field has trivial copy construction.
  bool has_trivial_copy() const { return is_trivial(); }
  bool has_trivial_value() const { return impl_->has_trivial_value(); }
  bool has_trivial_zero_default() const {
    return impl_->has_trivial_zero_default();
  }
  bool has_brace_default_assign() const {
    return impl_->has_brace_default_assign();
  }
  bool is_message() const { return impl_->is_message(); }
  bool is_group() const { return impl_->is_group(); }
  bool is_weak() const { return impl_->is_weak(); }
  bool is_lazy() const { return impl_->is_lazy(); }
  bool is_foreign() const { return impl_->is_foreign(); }
  bool is_string() const { return impl_->is_string(); }
  bool is_bytes() const { return impl_->is_bytes(); }
  bool is_oneof() const { return impl_->is_oneof(); }
  bool is_inlined() const { return impl_->is_inlined(); }
  bool has_default_constexpr_constructor() const {
    return impl_->has_default_constexpr_constructor();
  }

  // Requirements: see FieldGeneratorBase for documentation
  bool RequiresArena(GeneratorFunction function) const {
    return impl_->RequiresArena(function);
  }

  // Prints private members needed to represent this field.
  //
  // These are placed inside the class definition.
  void GeneratePrivateMembers(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GeneratePrivateMembers(p);
  }

  // Prints static members needed to represent this field.
  //
  // These are placed inside the class definition.
  void GenerateStaticMembers(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateStaticMembers(p);
  }

  void GenerateMemberConstructor(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateMemberConstructor(p);
  }

  void GenerateMemberCopyConstructor(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateMemberCopyConstructor(p);
  }

  void GenerateOneofCopyConstruct(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateOneofCopyConstruct(p);
  }

  void GenerateMemberConstexprConstructor(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateMemberConstexprConstructor(p);
  }

  // Generates declarations for all of the accessor functions related to this
  // field.
  //
  // These are placed inside the class definition.
  void GenerateAccessorDeclarations(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateAccessorDeclarations(p);
  }

  // Generates inline definitions of accessor functions for this field.
  //
  // These are placed in namespace scope in the header after all class
  // definitions.
  void GenerateInlineAccessorDefinitions(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateInlineAccessorDefinitions(p);
  }

  // Generates definitions of accessors that aren't inlined.
  //
  // These are placed in namespace scope in the .cc file.
  void GenerateNonInlineAccessorDefinitions(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateNonInlineAccessorDefinitions(p);
  }

  // Generates statements which clear the field.
  //
  // This is used to define the clear_$name$() method.
  void GenerateClearingCode(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateClearingCode(p);
  }

  // Generates statements which clear the field as part of the Clear() method
  // for the whole message.
  //
  // For message types which have field presence bits,
  // MessageGenerator::GenerateClear will have already checked the presence
  // bits.
  void GenerateMessageClearingCode(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateMessageClearingCode(p);
  }

  // Generates statements which merge the contents of the field from the current
  // message to the target message, which is stored in the generated code
  // variable `from`.
  //
  // This is used to fill in the MergeFrom method for the whole message.
  //
  // Details of this usage can be found in message.cc under the
  // GenerateMergeFrom method.
  void GenerateMergingCode(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateMergingCode(p);
  }

  // Generates a copy constructor
  //
  // TODO: Document this properly.
  void GenerateCopyConstructorCode(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateCopyConstructorCode(p);
  }

  // Generates statements which swap this field and the corresponding field of
  // another message, which is stored in the generated code variable `other`.
  //
  // This is used to define the Swap method. Details of usage can be found in
  // message.cc under the GenerateSwap method.
  void GenerateSwappingCode(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateSwappingCode(p);
  }

  // Generates initialization code for private members declared by
  // GeneratePrivateMembers().
  //
  // These go into the message class's SharedCtor() method, invoked by each of
  // the generated constructors.
  void GenerateConstructorCode(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateConstructorCode(p);
  }

  // Generates any code that needs to go in the class's SharedDtor() method,
  // invoked by the destructor.
  void GenerateDestructorCode(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateDestructorCode(p);
  }

  // Generates a manual destructor invocation for use when the message is on an
  // arena.
  //
  // The code that this method generates will be executed inside a
  // shared-for-the-whole-message-class method registered with OwnDestructor().
  void GenerateArenaDestructorCode(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateArenaDestructorCode(p);
  }

  // Generates initialization code for private members declared by
  // GeneratePrivateMembers().
  //
  // These go into the SharedCtor's aggregate initialization of the _impl_
  // struct and must follow the syntax `decltype($field$){$default$}`.
  // Does not include `:` or `,` separators. Default values should be specified
  // here when possible.
  //
  // NOTE: We use `decltype($field$)` for both explicit construction and the
  // fact that it's self-documenting. Pre-C++17, copy elision isn't guaranteed
  // in aggregate initialization so a valid copy/move constructor must exist
  // (even though it's not used). Because of this, we need to comment out the
  // decltype and fallback to implicit construction.
  void GenerateAggregateInitializer(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateAggregateInitializer(p);
  }

  // Generates constinit initialization code for private members declared by
  // GeneratePrivateMembers().
  //
  // These go into the constexpr constructor's aggregate initialization of the
  // _impl_ struct and must follow the syntax `/*decltype($field$)*/{}` (see
  // above). Does not include `:` or `,` separators.
  void GenerateConstexprAggregateInitializer(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateConstexprAggregateInitializer(p);
  }

  // Generates copy initialization code for private members declared by
  // GeneratePrivateMembers().
  //
  // These go into the copy constructor's aggregate initialization of the _impl_
  // struct and must follow the syntax `decltype($field$){from.$field$}` (see
  // above). Does not include `:` or `,` separators.
  void GenerateCopyAggregateInitializer(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateCopyAggregateInitializer(p);
  }

  // Generates statements to serialize this field directly to the array
  // `target`, which are placed within the message's
  // SerializeWithCachedSizesToArray() method.
  //
  // This must also advance `target` past the written bytes.
  void GenerateSerializeWithCachedSizesToArray(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateSerializeWithCachedSizesToArray(p);
  }

  // Generates statements to compute the serialized size of this field, which
  // are placed in the message's ByteSize() method.
  void GenerateByteSize(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateByteSize(p);
  }

  // Generates lines to call IsInitialized() for eligible message fields. Non
  // message fields won't need to override this function.
  void GenerateIsInitialized(io::Printer* p) const {
    auto vars = PushVarsForCall(p);
    impl_->GenerateIsInitialized(p);
  }

  bool NeedsIsInitialized() const { return impl_->NeedsIsInitialized(); }

  // TODO: Document this properly.
  bool IsInlined() const { return impl_->IsInlined(); }

  // TODO: Document this properly.
  ArenaDtorNeeds NeedsArenaDestructor() const {
    return impl_->NeedsArenaDestructor();
  }

 private:
  friend class FieldGeneratorTable;
  FieldGenerator(const FieldDescriptor* field, const Options& options,
                 MessageSCCAnalyzer* scc_analyzer,
                 absl::optional<uint32_t> hasbit_index,
                 absl::optional<uint32_t> inlined_string_index);

  std::unique_ptr<FieldGeneratorBase> impl_;
  std::vector<io::Printer::Sub> field_vars_;
  std::vector<io::Printer::Sub> tracker_vars_;
  std::vector<io::Printer::Sub> per_generator_vars_;
};

// Convenience class which constructs FieldGeneratorBases for a Descriptor.
class FieldGeneratorTable {
 public:
  explicit FieldGeneratorTable(const Descriptor* descriptor)
      : descriptor_(descriptor) {}

  FieldGeneratorTable(const FieldGeneratorTable&) = delete;
  FieldGeneratorTable& operator=(const FieldGeneratorTable&) = delete;

  void Build(const Options& options, MessageSCCAnalyzer* scc_analyzer,
             absl::Span<const int32_t> has_bit_indices,
             absl::Span<const int32_t> inlined_string_indices);

  const FieldGenerator& get(const FieldDescriptor* field) const {
    ABSL_CHECK_EQ(field->containing_type(), descriptor_);
    ABSL_DCHECK_GE(field->index(), 0);
    return fields_[static_cast<size_t>(field->index())];
  }

 private:
  const Descriptor* descriptor_;
  std::vector<FieldGenerator> fields_;
};

// Returns variables common to all fields.
//
// TODO: Make this function .cc-private.
std::vector<io::Printer::Sub> FieldVars(const FieldDescriptor* field,
                                        const Options& opts);
}  // namespace cpp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_FIELD_H__
