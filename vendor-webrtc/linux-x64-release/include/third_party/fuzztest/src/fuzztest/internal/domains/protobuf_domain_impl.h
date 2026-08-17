// Copyright 2022 Google LLC
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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_PROTOBUF_DOMAIN_IMPL_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_PROTOBUF_DOMAIN_IMPL_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/base/const_init.h"
#include "absl/base/no_destructor.h"
#include "absl/base/thread_annotations.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/random/bit_gen_ref.h"
#include "absl/random/random.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "absl/synchronization/mutex.h"
#include "absl/types/span.h"
#include "./fuzztest/domain_core.h"
#include "./fuzztest/internal/any.h"
#include "./fuzztest/internal/domains/arbitrary_impl.h"
#include "./fuzztest/internal/domains/container_of_impl.h"
#include "./fuzztest/internal/domains/domain.h"
#include "./fuzztest/internal/domains/domain_base.h"
#include "./fuzztest/internal/domains/element_of_impl.h"
#include "./fuzztest/internal/domains/optional_of_impl.h"
#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/serialization.h"
#include "./fuzztest/internal/status.h"
#include "./fuzztest/internal/type_support.h"

// GetMessage is a Windows macro. Undefine it here to avoid code clutter.
#ifdef _WIN32
#pragma push_macro("GetMessage")
#undef GetMessage
#endif

namespace google::protobuf {
class EnumDescriptor;

template <typename E>
const EnumDescriptor* GetEnumDescriptor();
}  // namespace google::protobuf

namespace fuzztest::internal {

// Sniff the API to get the types we need without naming them directly.
// This allows for a soft dependency on proto without having to #include its
// headers.
template <typename Message>
using ProtobufReflection =
    std::remove_pointer_t<decltype(std::declval<Message>().GetReflection())>;
template <typename Message>
using ProtobufDescriptor =
    std::remove_pointer_t<decltype(std::declval<Message>().GetDescriptor())>;
template <typename Message>
using ProtobufFieldDescriptor = std::remove_pointer_t<
    decltype(std::declval<Message>().GetDescriptor()->field(0))>;
template <typename Message>
using ProtobufOneofDescriptor = std::remove_pointer_t<
    decltype(std::declval<Message>().GetDescriptor()->oneof_decl(0))>;

template <typename Message, typename V>
class ProtocolBufferAccess;

#define FUZZTEST_INTERNAL_PROTO_ACCESS_(cpp_type, Camel)                      \
  template <typename Message>                                                 \
  class ProtocolBufferAccess<Message, cpp_type> {                             \
   public:                                                                    \
    using Reflection = ProtobufReflection<Message>;                           \
    using FieldDescriptor = ProtobufFieldDescriptor<Message>;                 \
    using value_type = cpp_type;                                              \
                                                                              \
    ProtocolBufferAccess(Message* message, const FieldDescriptor* field)      \
        : message_(message),                                                  \
          reflection_(message->GetReflection()),                              \
          field_(field) {}                                                    \
                                                                              \
    static auto GetField(const Message& message,                              \
                         const FieldDescriptor* field) {                      \
      return message.GetReflection()->Get##Camel(message, field);             \
    }                                                                         \
    void SetField(const cpp_type& value) {                                    \
      return reflection_->Set##Camel(message_, field_, value);                \
    }                                                                         \
    static auto GetRepeatedField(const Message& message,                      \
                                 const FieldDescriptor* field, int index) {   \
      return message.GetReflection()->GetRepeated##Camel(message, field,      \
                                                         index);              \
    }                                                                         \
    void SetRepeatedField(int index, const cpp_type& value) {                 \
      return reflection_->SetRepeated##Camel(message_, field_, index, value); \
    }                                                                         \
    void AddRepeatedField(const cpp_type& value) {                            \
      return reflection_->Add##Camel(message_, field_, value);                \
    }                                                                         \
                                                                              \
   private:                                                                   \
    Message* message_;                                                        \
    Reflection* reflection_;                                                  \
    const FieldDescriptor* field_;                                            \
  }
FUZZTEST_INTERNAL_PROTO_ACCESS_(int32_t, Int32);
FUZZTEST_INTERNAL_PROTO_ACCESS_(uint32_t, UInt32);
FUZZTEST_INTERNAL_PROTO_ACCESS_(int64_t, Int64);
FUZZTEST_INTERNAL_PROTO_ACCESS_(uint64_t, UInt64);
FUZZTEST_INTERNAL_PROTO_ACCESS_(float, Float);
FUZZTEST_INTERNAL_PROTO_ACCESS_(double, Double);
FUZZTEST_INTERNAL_PROTO_ACCESS_(std::string, String);
FUZZTEST_INTERNAL_PROTO_ACCESS_(bool, Bool);
#undef FUZZTEST_INTERNAL_PROTO_ACCESS_

struct ProtoEnumTag;
struct ProtoMessageTag;

// Dynamic to static dispatch visitation.
// It will invoke:
//   visitor.VisitSingular<type>(field)  // for singular fields
//   visitor.VisitRepeated<type>(field)  // for repeated fields
// where `type` is:
//  - For bool, integrals, floating point and string: their C++ type.
//  - For enum: the tag type ProtoEnumTag.
//  - For message: the tag type ProtoMessageTag.
template <typename FieldDescriptor, typename Visitor>
auto VisitProtobufField(const FieldDescriptor* field, Visitor visitor) {
  if (field->is_repeated()) {
    switch (field->cpp_type()) {
      case FieldDescriptor::CPPTYPE_BOOL:
        return visitor.template VisitRepeated<bool>(field);
      case FieldDescriptor::CPPTYPE_INT32:
        return visitor.template VisitRepeated<int32_t>(field);
      case FieldDescriptor::CPPTYPE_UINT32:
        return visitor.template VisitRepeated<uint32_t>(field);
      case FieldDescriptor::CPPTYPE_INT64:
        return visitor.template VisitRepeated<int64_t>(field);
      case FieldDescriptor::CPPTYPE_UINT64:
        return visitor.template VisitRepeated<uint64_t>(field);
      case FieldDescriptor::CPPTYPE_FLOAT:
        return visitor.template VisitRepeated<float>(field);
      case FieldDescriptor::CPPTYPE_DOUBLE:
        return visitor.template VisitRepeated<double>(field);
      case FieldDescriptor::CPPTYPE_STRING:
        return visitor.template VisitRepeated<std::string>(field);
      case FieldDescriptor::CPPTYPE_ENUM:
        return visitor.template VisitRepeated<ProtoEnumTag>(field);
      case FieldDescriptor::CPPTYPE_MESSAGE:
        return visitor.template VisitRepeated<ProtoMessageTag>(field);
    }
  } else {
    switch (field->cpp_type()) {
      case FieldDescriptor::CPPTYPE_BOOL:
        return visitor.template VisitSingular<bool>(field);
      case FieldDescriptor::CPPTYPE_INT32:
        return visitor.template VisitSingular<int32_t>(field);
      case FieldDescriptor::CPPTYPE_UINT32:
        return visitor.template VisitSingular<uint32_t>(field);
      case FieldDescriptor::CPPTYPE_INT64:
        return visitor.template VisitSingular<int64_t>(field);
      case FieldDescriptor::CPPTYPE_UINT64:
        return visitor.template VisitSingular<uint64_t>(field);
      case FieldDescriptor::CPPTYPE_FLOAT:
        return visitor.template VisitSingular<float>(field);
      case FieldDescriptor::CPPTYPE_DOUBLE:
        return visitor.template VisitSingular<double>(field);
      case FieldDescriptor::CPPTYPE_STRING:
        return visitor.template VisitSingular<std::string>(field);
      case FieldDescriptor::CPPTYPE_ENUM:
        return visitor.template VisitSingular<ProtoEnumTag>(field);
      case FieldDescriptor::CPPTYPE_MESSAGE:
        return visitor.template VisitSingular<ProtoMessageTag>(field);
    }
  }
}

template <typename T>
using Predicate = std::function<bool(const T*)>;

template <typename T>
Predicate<T> IncludeAll() {
  return [](const T*) { return true; };
}

template <typename T>
Predicate<T> IsOptional() {
  return [](const T* field) { return field->is_optional(); };
}

template <typename T>
Predicate<T> IsRepeated() {
  return [](const T* field) { return field->is_repeated(); };
}

template <typename T>
Predicate<T> And(Predicate<T> lhs, Predicate<T> rhs) {
  return [lhs = std::move(lhs), rhs = std::move(rhs)](const T* field) {
    return lhs(field) && rhs(field);
  };
}

template <typename T>
std::function<Domain<T>(Domain<T>)> Identity() {
  return [](Domain<T> domain) -> Domain<T> { return domain; };
}

template <typename Message>
class ProtoPolicy {
  using FieldDescriptor = ProtobufFieldDescriptor<Message>;
  using Filter = std::function<bool(const FieldDescriptor*)>;

 public:
  ProtoPolicy()
      : optional_policies_({{/*filter=*/IncludeAll<FieldDescriptor>(),
                             /*value=*/OptionalPolicy::kWithNull}}) {
    static int64_t next_id = 0;
    id_ = next_id++;
  }

  void SetOptionalPolicy(OptionalPolicy optional_policy) {
    SetOptionalPolicy(IncludeAll<FieldDescriptor>(), optional_policy);
  }

  void SetOptionalPolicy(Filter filter, OptionalPolicy optional_policy) {
    if (optional_policy == OptionalPolicy::kAlwaysNull) {
      max_repeated_fields_sizes_.push_back(
          {/*filter=*/And(IsRepeated<FieldDescriptor>(), filter), /*value=*/0});
      min_repeated_fields_sizes_.push_back(
          {/*filter=*/And(IsRepeated<FieldDescriptor>(), filter), /*value=*/0});
    } else if (optional_policy == OptionalPolicy::kWithoutNull) {
      // TODO(b/298168054): Ideally, min/max should get updated only if the
      // fields are not already always set.
      min_repeated_fields_sizes_.push_back(
          {/*filter=*/And(IsRepeated<FieldDescriptor>(), filter), /*value=*/1});
      max_repeated_fields_sizes_.push_back(
          {/*filter=*/And(IsRepeated<FieldDescriptor>(), filter),
           /*value=*/fuzztest::internal::kDefaultContainerMaxSize});
    }
    optional_policies_.push_back(
        {/*filter=*/std::move(filter), /*value=*/optional_policy});
  }

  void SetMinRepeatedFieldsSize(int64_t min_size) {
    SetMinRepeatedFieldsSize(IncludeAll<FieldDescriptor>(), min_size);
  }

  void SetMinRepeatedFieldsSize(Filter filter, int64_t min_size) {
    min_repeated_fields_sizes_.push_back(
        {/*filter=*/std::move(filter), /*value=*/min_size});
  }

  void SetMaxRepeatedFieldsSize(int64_t max_size) {
    SetMaxRepeatedFieldsSize(IncludeAll<FieldDescriptor>(), max_size);
  }

  void SetMaxRepeatedFieldsSize(Filter filter, int64_t max_size) {
    max_repeated_fields_sizes_.push_back(
        {/*filter=*/std::move(filter), /*value=*/max_size});
  }

  OptionalPolicy GetOptionalPolicy(const FieldDescriptor* field) const {
    FUZZTEST_INTERNAL_CHECK(
        field->is_optional(),
        "GetOptionalPolicy should apply to optional fields only!");
    std::optional<OptionalPolicy> result =
        GetPolicyValue(optional_policies_, field);
    FUZZTEST_INTERNAL_CHECK(result.has_value(), "optional policy is not set!");
    return *result;
  }

  std::optional<int64_t> GetMinRepeatedFieldSize(
      const FieldDescriptor* field) const {
    FUZZTEST_INTERNAL_CHECK(
        field->is_repeated(),
        "GetMinRepeatedFieldSize should apply to repeated fields only!");
    auto min = GetPolicyValue(min_repeated_fields_sizes_, field);
    auto max = GetPolicyValue(max_repeated_fields_sizes_, field);
    if (min.has_value() && max.has_value()) {
      FUZZTEST_INTERNAL_CHECK(*min <= *max, "Repeated field ",
                              field->full_name(), " size range is not valid!");
    }
    return min;
  }

  std::optional<int64_t> GetMaxRepeatedFieldSize(
      const FieldDescriptor* field) const {
    FUZZTEST_INTERNAL_CHECK(
        field->is_repeated(),
        "GetMaxRepeatedFieldSize should apply to repeated fields only!");
    auto min = GetPolicyValue(min_repeated_fields_sizes_, field);
    auto max = GetPolicyValue(max_repeated_fields_sizes_, field);
    if (min.has_value() && max.has_value()) {
      FUZZTEST_INTERNAL_CHECK(*min <= *max, "Repeated field ",
                              field->full_name(), " size range is not valid!");
    }
    return max;
  }

  int64_t id() const { return id_; }

 private:
  int64_t id_;

  template <typename T>
  struct FilterToValue {
    Filter filter;
    T value;
  };

  template <typename T>
  std::optional<T> GetPolicyValue(
      const std::vector<FilterToValue<T>>& filter_to_values,
      const FieldDescriptor* field) const {
    // Return the policy that is not overwritten.
    for (int i = filter_to_values.size() - 1; i >= 0; --i) {
      if (!filter_to_values[i].filter(field)) continue;
      if constexpr (std::is_same_v<T, Domain<std::unique_ptr<Message>>>) {
        absl::BitGen gen;
        auto domain = filter_to_values[i].value;
        auto obj = domain.GetValue(domain.Init(gen));
        auto* descriptor = obj->GetDescriptor();
        FUZZTEST_INTERNAL_CHECK_PRECONDITION(
            descriptor->full_name() == field->message_type()->full_name(),
            "Input domain does not match the expected message type. The "
            "domain produced a message of type `",
            descriptor->full_name(),
            "` but the field needs a message of type `",
            field->message_type()->full_name(), "`.");
      }
      return filter_to_values[i].value;
    }
    return std::nullopt;
  }
  std::vector<FilterToValue<OptionalPolicy>> optional_policies_;
  std::vector<FilterToValue<int64_t>> min_repeated_fields_sizes_;
  std::vector<FilterToValue<int64_t>> max_repeated_fields_sizes_;

#define FUZZTEST_INTERNAL_POLICY_MEMBERS(Camel, cpp)                          \
 private:                                                                     \
  std::vector<FilterToValue<Domain<cpp>>> domains_for_##Camel##_;             \
  std::vector<FilterToValue<std::function<Domain<cpp>(Domain<cpp>)>>>         \
      transformers_for_##Camel##_;                                            \
                                                                              \
 public:                                                                      \
  void SetDefaultDomainFor##Camel##s(                                         \
      Domain<MakeDependentType<cpp, Message>> domain) {                       \
    domains_for_##Camel##_.push_back(                                         \
        {/*filter=*/IncludeAll<FieldDescriptor>(),                            \
         /*value=*/std::move(domain)});                                       \
  }                                                                           \
  void SetDefaultDomainFor##Camel##s(                                         \
      Filter filter, Domain<MakeDependentType<cpp, Message>> domain) {        \
    domains_for_##Camel##_.push_back(                                         \
        {/*filter=*/std::move(filter), /*value=*/std::move(domain)});         \
  }                                                                           \
  void SetDomainTransformerFor##Camel##s(                                     \
      Filter filter, std::function<Domain<MakeDependentType<cpp, Message>>(   \
                         Domain<MakeDependentType<cpp, Message>>)>            \
                         transformer) {                                       \
    transformers_for_##Camel##_.push_back(                                    \
        {/*filter=*/std::move(filter), /*value=*/std::move(transformer)});    \
  }                                                                           \
  std::optional<Domain<MakeDependentType<cpp, Message>>>                      \
      GetDefaultDomainFor##Camel##s(const FieldDescriptor* field) const {     \
    return GetPolicyValue(domains_for_##Camel##_, field);                     \
  }                                                                           \
  std::optional<std::function<Domain<MakeDependentType<cpp, Message>>(        \
      Domain<MakeDependentType<cpp, Message>>)>>                              \
      GetDomainTransformerFor##Camel##s(const FieldDescriptor* field) const { \
    return GetPolicyValue(transformers_for_##Camel##_, field);                \
  }

  FUZZTEST_INTERNAL_POLICY_MEMBERS(Bool, bool)
  FUZZTEST_INTERNAL_POLICY_MEMBERS(Int32, int32_t)
  FUZZTEST_INTERNAL_POLICY_MEMBERS(UInt32, uint32_t)
  FUZZTEST_INTERNAL_POLICY_MEMBERS(Int64, int64_t)
  FUZZTEST_INTERNAL_POLICY_MEMBERS(UInt64, uint64_t)
  FUZZTEST_INTERNAL_POLICY_MEMBERS(Float, float)
  FUZZTEST_INTERNAL_POLICY_MEMBERS(Double, double)
  FUZZTEST_INTERNAL_POLICY_MEMBERS(String, std::string)
  FUZZTEST_INTERNAL_POLICY_MEMBERS(Enum, int)
  FUZZTEST_INTERNAL_POLICY_MEMBERS(Protobuf, std::unique_ptr<Message>)
};

template <typename Prototype>
class PrototypePtr {
 public:
  PrototypePtr(std::function<const Prototype*()> prototype_factory)
      : prototype_factory_(std::move(prototype_factory)), prototype_(nullptr) {}
  PrototypePtr(const Prototype* prototype)
      : prototype_factory_(), prototype_(prototype) {
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(prototype != nullptr,
                                         "Prototype should not be nullptr");
  }

  PrototypePtr& operator=(const PrototypePtr<Prototype>& other) = default;
  PrototypePtr(const PrototypePtr<Prototype>& other) = default;

  const Prototype* Get() const {
    if (!prototype_) prototype_ = prototype_factory_();
    return prototype_;
  }

 private:
  std::function<const Prototype*()> prototype_factory_;
  mutable const Prototype* prototype_;
};

// Domain for std::unique_ptr<Message>, where the prototype is accepted as a
// constructor argument.
template <typename Message>
class ProtobufDomainUntypedImpl
    : public domain_implementor::DomainBase<
          ProtobufDomainUntypedImpl<Message>, std::unique_ptr<Message>,
          absl::flat_hash_map<int, GenericDomainCorpusType>> {
  using Descriptor = ProtobufDescriptor<Message>;
  using FieldDescriptor = ProtobufFieldDescriptor<Message>;
  using OneofDescriptor = ProtobufOneofDescriptor<Message>;

 public:
  using typename ProtobufDomainUntypedImpl::DomainBase::corpus_type;
  using typename ProtobufDomainUntypedImpl::DomainBase::value_type;

  explicit ProtobufDomainUntypedImpl(PrototypePtr<Message> prototype,
                                     bool use_lazy_initialization)
      : prototype_(std::move(prototype)),
        use_lazy_initialization_(use_lazy_initialization),
        policy_(),
        customized_fields_(),
        always_set_oneofs_(),
        uncustomizable_oneofs_(),
        unset_oneof_fields_() {}

  ProtobufDomainUntypedImpl(const ProtobufDomainUntypedImpl& other)
      : prototype_(other.prototype_),
        use_lazy_initialization_(other.use_lazy_initialization_) {
    absl::MutexLock l(&other.mutex_);
    domains_ = other.domains_;
    policy_ = other.policy_;
    customized_fields_ = other.customized_fields_;
    always_set_oneofs_ = other.always_set_oneofs_;
    uncustomizable_oneofs_ = other.uncustomizable_oneofs_;
    unset_oneof_fields_ = other.unset_oneof_fields_;
  }

  corpus_type Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    const auto* descriptor = prototype_.Get()->GetDescriptor();
    FUZZTEST_INTERNAL_CHECK(
        !IsCustomizedRecursivelyOnly() || !IsInfinitelyRecursive(descriptor),
        absl::StrCat("Cannot set recursive fields for ",
                     descriptor->full_name(), " by default."));
    corpus_type val;
    absl::flat_hash_map<int, int> oneof_to_field;

    // TODO(b/241124202): Use a valid proto with minimum size.
    for (const FieldDescriptor* field : GetProtobufFields(descriptor)) {
      if (auto* oneof = field->containing_oneof()) {
        if (!oneof_to_field.contains(oneof->index())) {
          oneof_to_field[oneof->index()] =
              SelectAFieldIndexInOneof(oneof, prng);
        }
        if (oneof_to_field[oneof->index()] != field->index()) continue;
      } else if (IsCustomizedRecursivelyOnly()) {
        if (!MustBeSet(field) && IsFieldFinitelyRecursive(field)) {
          // We avoid initializing non-required recursive fields by default (if
          // they are not explicitly customized). Otherwise, the initialization
          // may never terminate. If a proto has only non-required recursive
          // fields, the initialization will be deterministic, which violates
          // the assumption on domain Init. However, such cases should be
          // extremely rare and breaking the assumption would not have severe
          // consequences.
          continue;
        }
        if (MustBeUnset(field)) continue;
      }
      VisitProtobufField(field, InitializeVisitor{prng, *this, val});
    }
    return val;
  }

  void Mutate(corpus_type& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    if (GetFieldCount(prototype_.Get()->GetDescriptor()) == 0) return;
    // TODO(JunyangShao): Maybe make CountNumberOfFields static.
    uint64_t total_weight = CountNumberOfFields(val);
    uint64_t selected_weight = absl::Uniform(absl::IntervalClosedClosed, prng,
                                             uint64_t{1}, total_weight);
    MutateSelectedField(val, prng, metadata, only_shrink, selected_weight);
  }

  auto GetPrinter() const { return ProtobufPrinter{}; }

  value_type GetValue(const corpus_type& value) const {
    value_type out(prototype_.Get()->New());

    for (auto& [number, data] : value) {
      if (IsMetadataEntry(number)) continue;
      auto* field = GetField(number);
      VisitProtobufField(field, GetValueVisitor{*out, *this, data});
    }

    return out;
  }

  std::optional<corpus_type> FromValue(const value_type& value) const {
    return FromValue(*value);
  }

  std::optional<corpus_type> FromValue(const Message& value) const {
    corpus_type ret;
    auto* reflection = value.GetReflection();
    std::vector<const FieldDescriptor*> fields;
    reflection->ListFields(value, &fields);
    for (auto field : fields) {
      if (!VisitProtobufField(field, FromValueVisitor{value, ret, *this}))
        return std::nullopt;
    }
    return ret;
  }

  std::optional<corpus_type> ParseCorpus(const IRObject& obj) const {
    corpus_type out;
    auto subs = obj.Subs();
    if (!subs) return std::nullopt;
    absl::flat_hash_set<int> present_fields;
    present_fields.reserve(subs->size());
    out.reserve(subs->size());
    for (const auto& sub : *subs) {
      auto pair_subs = sub.Subs();
      if (!pair_subs || pair_subs->size() != 2) return std::nullopt;
      FieldDescriptor* field = nullptr;
      if (auto number = (*pair_subs)[0].GetScalar<int>(); number.has_value()) {
        field = GetField(*number);
      } else if (auto name = (*pair_subs)[0].GetScalar<std::string>();
                 name.has_value()) {
        field = GetFieldWithoutCheck(std::string(*name));
      }
      if (!field) return std::nullopt;
      present_fields.insert(field->number());
      std::optional<GenericDomainCorpusType> inner_parsed;
      VisitProtobufField(field,
                         ParseVisitor{*this, (*pair_subs)[1], inner_parsed});
      if (!inner_parsed) return std::nullopt;
      out[field->number()] = *std::move(inner_parsed);
    }
    for (const FieldDescriptor* field :
         GetProtobufFields(prototype_.Get()->GetDescriptor())) {
      if (present_fields.contains(field->number())) continue;
      std::optional<GenericDomainCorpusType> inner_parsed;
      IRObject unset_value;
      if (field->is_repeated()) {
        unset_value = IRObject(std::vector<IRObject>{});
      } else {
        unset_value = IRObject(std::vector<IRObject>{IRObject(0)});
      }
      VisitProtobufField(field, ParseVisitor{*this, unset_value, inner_parsed});
      if (!inner_parsed) return std::nullopt;
    }
    return out;
  }

  IRObject SerializeCorpus(const corpus_type& v) const {
    IRObject out;
    auto& subs = out.MutableSubs();
    subs.reserve(v.size());
    for (auto& [number, inner] : v) {
      if (IsMetadataEntry(number)) continue;
      auto* field = GetField(number);
      FUZZTEST_INTERNAL_CHECK(field, "Field not found by number: ", number);
      IRObject& pair = subs.emplace_back();
      auto& pair_subs = pair.MutableSubs();
      pair_subs.reserve(2);
      pair_subs.emplace_back(std::string(GetFieldName(field)));
      VisitProtobufField(
          field, SerializeVisitor{*this, inner, pair_subs.emplace_back()});
    }
    return out;
  }

  uint64_t CountNumberOfFields(corpus_type& val) {
    if (auto it = val.find(kFieldCountIndex); it != val.end()) {
      return it->second.template GetAs<uint64_t>();
    }
    uint64_t total_weight = 0;
    auto descriptor = prototype_.Get()->GetDescriptor();
    if (GetFieldCount(descriptor) == 0) return total_weight;

    for (const FieldDescriptor* field : GetProtobufFields(descriptor)) {
      if (field->containing_oneof() &&
          GetOneofFieldPolicy(field) == OptionalPolicy::kAlwaysNull) {
        continue;
      }
      if (IsCustomizedRecursivelyOnly() && MustBeUnset(field)) continue;
      ++total_weight;

      if (field->cpp_type() == FieldDescriptor::CPPTYPE_MESSAGE) {
        auto val_it = val.find(field->number());
        if (val_it == val.end()) continue;
        if (field->is_repeated()) {
          total_weight +=
              GetSubDomain<ProtoMessageTag, true>(field).CountNumberOfFields(
                  val_it->second);
        } else {
          total_weight +=
              GetSubDomain<ProtoMessageTag, false>(field).CountNumberOfFields(
                  val_it->second);
        }
      }
    }
    val[kFieldCountIndex] =
        GenericDomainCorpusType(std::in_place_type<uint64_t>, total_weight);
    return total_weight;
  }

  uint64_t MutateSelectedField(
      corpus_type& val, absl::BitGenRef prng,
      const domain_implementor::MutationMetadata& metadata, bool only_shrink,
      uint64_t selected_field_index) {
    uint64_t field_counter = 0;
    auto descriptor = prototype_.Get()->GetDescriptor();
    if (GetFieldCount(descriptor) == 0) return field_counter;
    int64_t fields_count = CountNumberOfFields(val);
    if (fields_count < selected_field_index) return fields_count;
    val.erase(kFieldCountIndex);  // Mutation invalidates the cache value.

    for (const FieldDescriptor* field : GetProtobufFields(descriptor)) {
      if (field->containing_oneof() &&
          GetOneofFieldPolicy(field) == OptionalPolicy::kAlwaysNull) {
        continue;
      }
      if (IsCustomizedRecursivelyOnly() && MustBeUnset(field)) continue;
      ++field_counter;
      if (field_counter == selected_field_index) {
        VisitProtobufField(
            field, MutateVisitor{prng, metadata, only_shrink, *this, val});
        return field_counter;
      }

      if (field->cpp_type() == FieldDescriptor::CPPTYPE_MESSAGE) {
        auto val_it = val.find(field->number());
        if (val_it == val.end()) continue;
        if (field->is_repeated()) {
          field_counter +=
              GetSubDomain<ProtoMessageTag, true>(field).MutateSelectedField(
                  val_it->second, prng, metadata, only_shrink,
                  selected_field_index - field_counter);
        } else {
          field_counter +=
              GetSubDomain<ProtoMessageTag, false>(field).MutateSelectedField(
                  val_it->second, prng, metadata, only_shrink,
                  selected_field_index - field_counter);
        }
      }
      if (field_counter >= selected_field_index) return field_counter;
    }
    return field_counter;
  }

  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    auto* descriptor = prototype_.Get()->GetDescriptor();
    for (int i = 0; i < descriptor->oneof_decl_count(); ++i) {
      auto oneof = descriptor->oneof_decl(i);
      absl::Status status = ValidateOneof(corpus_value, oneof);
      if (!status.ok()) return status;
    }
    for (const FieldDescriptor* field :
         GetProtobufFields(prototype_.Get()->GetDescriptor())) {
      if (field->containing_oneof()) continue;
      auto field_number_value = corpus_value.find(field->number());
      const GenericDomainCorpusType* inner_corpus_value =
          (field_number_value != corpus_value.end())
              ? &field_number_value->second
              : nullptr;
      absl::Status result;
      VisitProtobufField(field,
                         ValidateVisitor{*this, inner_corpus_value, result});
      if (!result.ok()) return result;
    }
    return absl::OkStatus();
  }

  ProtobufDomainUntypedImpl&& WithFieldsAlwaysSet(
      std::function<bool(const FieldDescriptor*)> filter =
          IncludeAll<FieldDescriptor>()) && {
    policy_.SetOptionalPolicy(std::move(filter), OptionalPolicy::kWithoutNull);
    return std::move(*this);
  }

  ProtobufDomainUntypedImpl&& WithFieldsUnset(
      std::function<bool(const FieldDescriptor*)> filter =
          IncludeAll<FieldDescriptor>()) && {
    policy_.SetOptionalPolicy(std::move(filter), OptionalPolicy::kAlwaysNull);
    return std::move(*this);
  }

  ProtobufDomainUntypedImpl&& WithOptionalFieldsAlwaysSet(
      std::function<bool(const FieldDescriptor*)> filter =
          IncludeAll<FieldDescriptor>()) && {
    policy_.SetOptionalPolicy(
        And(IsOptional<FieldDescriptor>(), std::move(filter)),
        OptionalPolicy::kWithoutNull);
    return std::move(*this);
  }

  ProtobufDomainUntypedImpl&& WithOptionalFieldsUnset(
      std::function<bool(const FieldDescriptor*)> filter =
          IncludeAll<FieldDescriptor>()) && {
    policy_.SetOptionalPolicy(
        And(IsOptional<FieldDescriptor>(), std::move(filter)),
        OptionalPolicy::kAlwaysNull);
    return std::move(*this);
  }

  ProtobufDomainUntypedImpl&& WithRepeatedFieldsAlwaysSet(
      std::function<bool(const FieldDescriptor*)> filter =
          IncludeAll<FieldDescriptor>()) && {
    policy_.SetOptionalPolicy(
        And(IsRepeated<FieldDescriptor>(), std::move(filter)),
        OptionalPolicy::kWithoutNull);
    return std::move(*this);
  }

  ProtobufDomainUntypedImpl&& WithRepeatedFieldsUnset(
      std::function<bool(const FieldDescriptor*)> filter =
          IncludeAll<FieldDescriptor>()) && {
    policy_.SetOptionalPolicy(
        And(IsRepeated<FieldDescriptor>(), std::move(filter)),
        OptionalPolicy::kAlwaysNull);
    return std::move(*this);
  }

  ProtobufDomainUntypedImpl&& WithRepeatedFieldsSize(int64_t size) && {
    return std::move(*this)
        .WithRepeatedFieldsMinSize(size)
        .WithRepeatedFieldsMaxSize(size);
  }

  ProtobufDomainUntypedImpl&& WithRepeatedFieldsSize(
      std::function<bool(const FieldDescriptor*)> filter, int64_t size) && {
    return std::move(*this)
        .WithRepeatedFieldsMinSize(filter, size)
        .WithRepeatedFieldsMaxSize(filter, size);
  }

  ProtobufDomainUntypedImpl&& WithRepeatedFieldsMinSize(int64_t min_size) && {
    policy_.SetMinRepeatedFieldsSize(IncludeAll<FieldDescriptor>(), min_size);
    return std::move(*this);
  }

  ProtobufDomainUntypedImpl&& WithRepeatedFieldsMinSize(
      std::function<bool(const FieldDescriptor*)> filter, int64_t min_size) && {
    policy_.SetMinRepeatedFieldsSize(std::move(filter), min_size);
    return std::move(*this);
  }

  ProtobufDomainUntypedImpl&& WithRepeatedFieldsMaxSize(int64_t max_size) && {
    policy_.SetMaxRepeatedFieldsSize(IncludeAll<FieldDescriptor>(), max_size);
    return std::move(*this);
  }

  ProtobufDomainUntypedImpl&& WithRepeatedFieldsMaxSize(
      std::function<bool(const FieldDescriptor*)> filter, int64_t max_size) && {
    policy_.SetMaxRepeatedFieldsSize(std::move(filter), max_size);
    return std::move(*this);
  }

#define FUZZTEST_INTERNAL_WITH_FIELD(Camel, cpp, TAG)                        \
  using Camel##type = MakeDependentType<cpp, Message>;                       \
  ProtobufDomainUntypedImpl&& With##Camel##Fields(                           \
      Domain<Camel##type> domain)&& {                                        \
    policy_.SetDefaultDomainFor##Camel##s(IncludeAll<FieldDescriptor>(),     \
                                          std::move(domain));                \
    return std::move(*this);                                                 \
  }                                                                          \
  ProtobufDomainUntypedImpl&& With##Camel##Fields(                           \
      std::function<bool(const FieldDescriptor*)>&& filter,                  \
      Domain<Camel##type> domain)&& {                                        \
    policy_.SetDefaultDomainFor##Camel##s(std::move(filter),                 \
                                          std::move(domain));                \
    return std::move(*this);                                                 \
  }                                                                          \
  ProtobufDomainUntypedImpl&& WithOptional##Camel##Fields(                   \
      Domain<Camel##type> domain)&& {                                        \
    policy_.SetDefaultDomainFor##Camel##s(IsOptional<FieldDescriptor>(),     \
                                          std::move(domain));                \
    return std::move(*this);                                                 \
  }                                                                          \
  ProtobufDomainUntypedImpl&& WithOptional##Camel##Fields(                   \
      std::function<bool(const FieldDescriptor*)>&& filter,                  \
      Domain<Camel##type> domain)&& {                                        \
    policy_.SetDefaultDomainFor##Camel##s(                                   \
        And(IsOptional<FieldDescriptor>(), std::move(filter)),               \
        std::move(domain));                                                  \
    return std::move(*this);                                                 \
  }                                                                          \
  ProtobufDomainUntypedImpl&& WithRepeated##Camel##Fields(                   \
      Domain<Camel##type> domain)&& {                                        \
    policy_.SetDefaultDomainFor##Camel##s(IsRepeated<FieldDescriptor>(),     \
                                          std::move(domain));                \
    return std::move(*this);                                                 \
  }                                                                          \
  ProtobufDomainUntypedImpl&& WithRepeated##Camel##Fields(                   \
      std::function<bool(const FieldDescriptor*)>&& filter,                  \
      Domain<Camel##type> domain)&& {                                        \
    policy_.SetDefaultDomainFor##Camel##s(                                   \
        And(IsRepeated<FieldDescriptor>(), std::move(filter)),               \
        std::move(domain));                                                  \
    return std::move(*this);                                                 \
  }                                                                          \
  ProtobufDomainUntypedImpl&& With##Camel##FieldsTransformed(                \
      std::function<Domain<Camel##type>(Domain<Camel##type>)>&&              \
          transformer)&& {                                                   \
    policy_.SetDomainTransformerFor##Camel##s(IncludeAll<FieldDescriptor>(), \
                                              std::move(transformer));       \
    return std::move(*this);                                                 \
  }                                                                          \
  ProtobufDomainUntypedImpl&& With##Camel##FieldsTransformed(                \
      std::function<bool(const FieldDescriptor*)>&& filter,                  \
      std::function<Domain<Camel##type>(Domain<Camel##type>)>&&              \
          transformer)&& {                                                   \
    policy_.SetDomainTransformerFor##Camel##s(std::move(filter),             \
                                              std::move(transformer));       \
    return std::move(*this);                                                 \
  }

  FUZZTEST_INTERNAL_WITH_FIELD(Bool, bool, bool)
  FUZZTEST_INTERNAL_WITH_FIELD(Int32, int32_t, int32_t)
  FUZZTEST_INTERNAL_WITH_FIELD(UInt32, uint32_t, uint32_t)
  FUZZTEST_INTERNAL_WITH_FIELD(Int64, int64_t, int64_t)
  FUZZTEST_INTERNAL_WITH_FIELD(UInt64, uint64_t, uint64_t)
  FUZZTEST_INTERNAL_WITH_FIELD(Float, float, float)
  FUZZTEST_INTERNAL_WITH_FIELD(Double, double, double)
  FUZZTEST_INTERNAL_WITH_FIELD(String, std::string, std::string)
  FUZZTEST_INTERNAL_WITH_FIELD(Enum, int, ProtoEnumTag)
  FUZZTEST_INTERNAL_WITH_FIELD(Protobuf, std::unique_ptr<Message>,
                               ProtoMessageTag)

#undef FUZZTEST_INTERNAL_WITH_FIELD

  template <typename Protobuf>
  ProtobufDomainUntypedImpl&& WithProtobufFields(Domain<Protobuf> domain) && {
    return std::move(*this).WithProtobufFields(
        ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename Protobuf>
  ProtobufDomainUntypedImpl&& WithProtobufFields(
      std::function<bool(const FieldDescriptor*)>&& filter,
      Domain<Protobuf> domain) && {
    return std::move(*this).WithProtobufFields(
        std::move(filter), ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename Protobuf>
  ProtobufDomainUntypedImpl&& WithOptionalProtobufFields(
      Domain<Protobuf> domain) && {
    return std::move(*this).WithOptionalProtobufFields(
        ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename Protobuf>
  ProtobufDomainUntypedImpl&& WithOptionalProtobufFields(
      std::function<bool(const FieldDescriptor*)>&& filter,
      Domain<Protobuf> domain) && {
    return std::move(*this).WithOptionalProtobufFields(
        std::move(filter), ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename Protobuf>
  ProtobufDomainUntypedImpl&& WithRepeatedProtobufFields(
      Domain<Protobuf> domain) && {
    return std::move(*this).WithRepeatedProtobufFields(
        ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename Protobuf>
  ProtobufDomainUntypedImpl&& WithRepeatedProtobufFields(
      std::function<bool(const FieldDescriptor*)>&& filter,
      Domain<Protobuf> domain) && {
    return std::move(*this).WithRepeatedProtobufFields(
        std::move(filter), ToUntypedProtoDomain(std::move(domain)));
  }

 private:
  template <typename, typename>
  friend class ProtobufDomainImpl;

  template <typename Inner>
  Domain<std::unique_ptr<Message>> ToUntypedProtoDomain(Inner inner_domain) {
    return ReversibleMap(
        [](value_type_t<Inner> proto_message) -> std::unique_ptr<Message> {
          return {std::make_unique<value_type_t<Inner>>(proto_message)};
        },
        [](const std::unique_ptr<Message>& proto_message)
            -> std::optional<std::tuple<value_type_t<Inner>>> {
          return *static_cast<std::add_pointer_t<value_type_t<Inner>>>(
              proto_message.get());
        },
        std::move(inner_domain));
  }

  template <typename T>
  static void InitializeFieldValue(absl::BitGenRef prng,
                                   const ProtobufDomainUntypedImpl& self,
                                   const FieldDescriptor* field,
                                   corpus_type& val) {
    auto& domain = self.GetSubDomain<T, false>(field);
    val[field->number()] = domain.Init(prng);
    if (auto* oneof = field->containing_oneof()) {
      // Clear the other parts of the oneof. They are unnecessary to
      // have and mutating them would have no effect.
      for (int i = 0; i < oneof->field_count(); ++i) {
        if (i != field->index_in_oneof()) {
          val.erase(oneof->field(i)->number());
        }
      }
    }
  }

  struct InitializeVisitor {
    absl::BitGenRef prng;
    ProtobufDomainUntypedImpl& self;
    corpus_type& val;

    template <typename T>
    void VisitSingular(const FieldDescriptor* field) {
      InitializeFieldValue<T>(prng, self, field, val);
    }

    template <typename T>
    void VisitRepeated(const FieldDescriptor* field) {
      auto& domain = self.GetSubDomain<T, true>(field);
      val[field->number()] = domain.Init(prng);
    }
  };

  template <typename OneofDescriptor>
  int SelectAFieldIndexInOneof(const OneofDescriptor* oneof,
                               absl::BitGenRef prng) {
    std::vector<int> fields;
    for (int i = 0; i < oneof->field_count(); ++i) {
      OptionalPolicy policy = GetOneofFieldPolicy(oneof->field(i));
      if (policy == OptionalPolicy::kAlwaysNull) continue;
      if (IsCustomizedRecursivelyOnly()) {
        if (IsFieldFinitelyRecursive(oneof->field(i))) continue;
        if (MustBeUnset(oneof->field(i))) continue;
      }
      fields.push_back(i);
    }
    if (fields.empty()) {  // This can happen if all fields are unset.
      return -1;
    }
    uint64_t selected =
        absl::Uniform(absl::IntervalClosedOpen, prng, size_t{0}, fields.size());
    return oneof->field(fields[selected])->index();
  }

  struct MutateVisitor {
    absl::BitGenRef prng;
    const domain_implementor::MutationMetadata& metadata;
    bool only_shrink;
    ProtobufDomainUntypedImpl& self;
    corpus_type& val;

    template <typename T>
    void VisitSingular(const FieldDescriptor* field) {
      auto& domain = self.GetSubDomain<T, false>(field);
      auto it = val.find(field->number());
      const bool is_present = it != val.end();

      if (is_present) {
        // Mutate the element
        domain.Mutate(it->second, prng, metadata, only_shrink);
        return;
      }

      // Add the element
      if (!only_shrink) {
        InitializeFieldValue<T>(prng, self, field, val);
      }
    }

    template <typename T>
    void VisitRepeated(const FieldDescriptor* field) {
      auto& domain = self.GetSubDomain<T, true>(field);
      auto it = val.find(field->number());
      const bool is_present = it != val.end();

      if (!is_present) {
        if (!only_shrink) {
          val[field->number()] = domain.Init(prng);
        }
      } else if (field->is_map()) {
        // field of synthetic messages of the form:
        //
        // message {
        //   optional key_type key = 1;
        //   optional value_type value = 2;
        // }
        //
        // The generic code is not doing dedup and it would only happen some
        // time after GetValue when the map field is synchronized between
        // reflection and codegen. Let's do it eagerly to drop dead entries so
        // that we don't keep mutating them later.
        //
        // TODO(b/231212420): Improve mutation to not add duplicate keys on the
        // first place. The current hack is very inefficient.
        // Switch the inner domain for maps to use flat_hash_map instead.
        corpus_type corpus_copy;
        auto& copy = corpus_copy[field->number()] = it->second;
        domain.Mutate(copy, prng, metadata, only_shrink);
        auto v = self.GetValue(corpus_copy);
        // We need to roundtrip through serialization to really dedup. The
        // reflection API alone doesn't cut it.
        v->ParsePartialFromString(v->SerializePartialAsString());
        if (v->GetReflection()->FieldSize(*v, field) ==
            domain.GetValue(copy).size()) {
          // The number of entries is the same, so accept the change.
          it->second = std::move(copy);
        }
      } else {
        domain.Mutate(it->second, prng, metadata, only_shrink);
      }
    }
  };

  struct GetValueVisitor {
    Message& message;
    const ProtobufDomainUntypedImpl& self;
    const GenericDomainCorpusType& data;

    template <typename T>
    void VisitSingular(const FieldDescriptor* field) {
      auto& domain = self.GetSubDomain<T, false>(field);
      auto value = domain.GetValue(data);
      if (!value.has_value()) {
        FUZZTEST_INTERNAL_CHECK_PRECONDITION(
            !field->is_required(), "required field '",
            std::string(field->full_name()), "' cannot have null values.");
        message.GetReflection()->ClearField(&message, field);
        return;
      }
      if constexpr (std::is_same_v<T, ProtoMessageTag>) {
        message.GetReflection()->SetAllocatedMessage(&message, value->release(),
                                                     field);
      } else if constexpr (std::is_same_v<T, ProtoEnumTag>) {
        message.GetReflection()->SetEnumValue(&message, field, *value);
      } else {
        ProtocolBufferAccess<Message, T>(&message, field).SetField(*value);
      }
    }

    template <typename T>
    void VisitRepeated(const FieldDescriptor* field) {
      auto& domain = self.GetSubDomain<T, true>(field);
      if constexpr (std::is_same_v<T, ProtoMessageTag>) {
        for (auto& v : domain.GetValue(data)) {
          message.GetReflection()->AddAllocatedMessage(&message, field,
                                                       v.release());
        }
      } else if constexpr (std::is_same_v<T, ProtoEnumTag>) {
        for (const auto& v : domain.GetValue(data)) {
          message.GetReflection()->AddEnumValue(&message, field, v);
        }
      } else {
        for (const auto& v : domain.GetValue(data)) {
          ProtocolBufferAccess<Message, T>(&message, field).AddRepeatedField(v);
        }
      }
    }
  };

  struct FromValueVisitor {
    const Message& message;
    corpus_type& out;
    const ProtobufDomainUntypedImpl& self;

    // TODO(sbenzaquen): We might want to try avoid these copies. On the other hand,
    // FromValue is not called much so it might be ok.
    template <typename T>
    bool VisitSingular(const FieldDescriptor* field) {
      auto& domain = self.GetSubDomain<T, false>(field);
      value_type_t<std::decay_t<decltype(domain)>> inner_value;
      auto* reflection = message.GetReflection();
      if constexpr (std::is_same_v<T, ProtoMessageTag>) {
        const auto& child = reflection->GetMessage(message, field);
        inner_value = std::unique_ptr<Message>(child.New());
        (*inner_value)->CopyFrom(child);
      } else if constexpr (std::is_same_v<T, ProtoEnumTag>) {
        inner_value = reflection->GetEnum(message, field)->number();
      } else {
        inner_value =
            ProtocolBufferAccess<Message, T>::GetField(message, field);
      }
      auto inner = domain.FromValue(inner_value);
      if (!inner) return false;
      out[field->number()] = *std::move(inner);
      return true;
    }

    template <typename T>
    bool VisitRepeated(const FieldDescriptor* field) {
      auto& domain = self.GetSubDomain<T, true>(field);
      value_type_t<std::decay_t<decltype(domain)>> inner_value;
      auto* reflection = message.GetReflection();
      const int size = reflection->FieldSize(message, field);
      for (int i = 0; i < size; ++i) {
        if constexpr (std::is_same_v<T, ProtoMessageTag>) {
          const auto& child = reflection->GetRepeatedMessage(message, field, i);
          auto* copy = child.New();
          copy->CopyFrom(child);
          inner_value.emplace_back(copy);
        } else if constexpr (std::is_same_v<T, ProtoEnumTag>) {
          inner_value.push_back(
              reflection->GetRepeatedEnum(message, field, i)->number());
        } else {
          inner_value.push_back(
              ProtocolBufferAccess<Message, T>::GetRepeatedField(message, field,
                                                                 i));
        }
      }

      auto inner = domain.FromValue(inner_value);
      if (!inner) return false;
      out[field->number()] = *std::move(inner);
      return true;
    }
  };

  struct ParseVisitor {
    const ProtobufDomainUntypedImpl& self;
    const IRObject& obj;
    std::optional<GenericDomainCorpusType>& out;

    template <typename T>
    void VisitSingular(const FieldDescriptor* field) {
      out = self.GetSubDomain<T, false>(field).ParseCorpus(obj);
    }

    template <typename T>
    void VisitRepeated(const FieldDescriptor* field) {
      out = self.GetSubDomain<T, true>(field).ParseCorpus(obj);
    }
  };

  struct SerializeVisitor {
    const ProtobufDomainUntypedImpl& self;
    const GenericDomainCorpusType& corpus_value;
    IRObject& out;

    template <typename T>
    void VisitSingular(const FieldDescriptor* field) {
      out = self.GetSubDomain<T, false>(field).SerializeCorpus(corpus_value);
    }
    template <typename T>
    void VisitRepeated(const FieldDescriptor* field) {
      out = self.GetSubDomain<T, true>(field).SerializeCorpus(corpus_value);
    }
  };

  struct ValidateVisitor {
    const ProtobufDomainUntypedImpl& self;
    // nullptr indicates that the field is not set.
    const GenericDomainCorpusType* corpus_value;
    absl::Status& out;

    template <typename T>
    void VisitSingular(const FieldDescriptor* field) {
      if (corpus_value) {
        out = self.GetSubDomain<T, /*is_repeated=*/false>(field)
                  .ValidateCorpusValue(*corpus_value);
      } else {
        out = self.GetSubDomain<T, /*is_repeated=*/false>(field)
                  .ValidateCorpusValue(
                      *self.GetSubDomain<T, /*is_repeated=*/false>(field)
                           .FromValue(std::nullopt));
      }
      if (out.ok()) return;
      out =
          Prefix(out, absl::StrCat("Invalid value for field ", field->name()));
    }

    template <typename T>
    void VisitRepeated(const FieldDescriptor* field) {
      if (corpus_value) {
        out = self.GetSubDomain<T, /*is_repeated=*/true>(field)
                  .ValidateCorpusValue(*corpus_value);
      } else {
        out = self.GetSubDomain<T, /*is_repeated=*/true>(field)
                  .ValidateCorpusValue(
                      *self.GetSubDomain<T, /*is_repeated=*/true>(field)
                           .FromValue({}));
      }
      if (out.ok()) return;
      out =
          Prefix(out, absl::StrCat("Invalid value for field ", field->name()));
    }
  };

  absl::Status ValidateOneof(const corpus_type& corpus_value,
                             const OneofDescriptor* oneof) const {
    int set_fields_count = 0;
    for (int i = 0; i < oneof->field_count(); ++i) {
      auto field_number_value = corpus_value.find(oneof->field(i)->number());
      if (field_number_value == corpus_value.end()) continue;
      const GenericDomainCorpusType* inner_corpus_value =
          &field_number_value->second;
      absl::Status status;
      VisitProtobufField(oneof->field(i),
                         ValidateVisitor{*this, inner_corpus_value, status});
      if (!status.ok()) return status;
      ++set_fields_count;
    }
    if (set_fields_count == 0 && IsOneofAlwaysSet(oneof->index())) {
      return absl::InvalidArgumentError(
          absl::StrCat("Oneof ", oneof->name(), " is not set"));
    }
    if (set_fields_count > 1) {
      return absl::InvalidArgumentError(absl::StrCat(
          "Oneof ", oneof->name(), " has been set multiple times"));
    }
    return absl::OkStatus();
  }

  template <typename Inner>
  struct WithFieldVisitor {
    Inner domain;
    ProtobufDomainUntypedImpl& self;

    template <bool is_repeated, typename T>
    Descriptor* GetDescriptor(const T& val) const {
      auto v = domain.GetValue(val);
      if constexpr (is_repeated) {
        if (v.empty()) return nullptr;
        return v[0]->GetDescriptor();
      } else {
        if (!v.has_value()) return nullptr;
        return (*v)->GetDescriptor();
      }
    }

    template <typename T, typename DomainT, bool is_repeated>
    void ApplyDomain(const FieldDescriptor* field) {
      if constexpr (!std::is_constructible_v<DomainT, Inner>) {
        FUZZTEST_INTERNAL_CHECK_PRECONDITION(
            (std::is_constructible_v<DomainT, Inner>),
            "Input domain does not match field `", field->full_name(),
            "` type.");
      } else {
        if constexpr (std::is_same_v<T, ProtoMessageTag>) {
          // Verify that the type matches.
          absl::BitGen gen;
          std::string full_name;
          constexpr int kMaxTry = 10;
          auto val = domain.Init(gen);
          auto descriptor = GetDescriptor<is_repeated>(val);
          for (int i = 0; !descriptor && i < kMaxTry; ++i) {
            domain.Mutate(val, gen, {}, false);
            descriptor = GetDescriptor<is_repeated>(val);
          }
          FUZZTEST_INTERNAL_CHECK_PRECONDITION(
              !descriptor ||
                  descriptor->full_name() == field->message_type()->full_name(),
              "Input domain does not match the expected message type. The "
              "domain produced a message of type `",
              descriptor->full_name(),
              "` but the field needs a message of type `",
              field->message_type()->full_name(), "`.");
        }
        absl::MutexLock l(&self.mutex_);
        auto res = self.domains_.try_emplace(field->number(),
                                             std::in_place_type<DomainT>,
                                             std::forward<Inner>(domain));
        FUZZTEST_INTERNAL_CHECK_PRECONDITION(res.second, "Domain for field `",
                                             field->full_name(),
                                             "` has been set multiple times.");
      }
    }

    template <typename T>
    void VisitSingular(const FieldDescriptor* field) {
      using DomainT = decltype(self.GetDefaultDomainForField<T, false>(field));
      ApplyDomain<T, DomainT, /*is_repeated=*/false>(field);
    }

    template <typename T>
    void VisitRepeated(const FieldDescriptor* field) {
      using DomainT = decltype(self.GetDefaultDomainForField<T, true>(field));
      ApplyDomain<T, DomainT, /*is_repeated=*/true>(field);
    }
  };

  template <typename Inner>
  void WithField(absl::string_view field_name, Inner&& domain) {
    auto* field = GetField(field_name);
    VisitProtobufField(
        field, WithFieldVisitor<Inner&&>{std::forward<Inner>(domain), *this});
    customized_fields_.insert(field->index());
  }

  auto GetFieldWithoutCheck(absl::string_view field_name) const {
    auto* field = prototype_.Get()->GetDescriptor()->FindFieldByName(
        std::string(field_name));
    if (field == nullptr) {
      field = prototype_.Get()->GetReflection()->FindKnownExtensionByName(
          std::string(field_name));
    }
    return field;
  }

  auto GetField(absl::string_view field_name) const {
    auto* field = GetFieldWithoutCheck(field_name);
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(field != nullptr,
                                         "Invalid field name '",
                                         std::string(field_name), "'.");
    return field;
  }

  auto GetField(int number) const {
    auto* field = prototype_.Get()->GetDescriptor()->FindFieldByNumber(number);
    if (field == nullptr) {
      field =
          prototype_.Get()->GetReflection()->FindKnownExtensionByNumber(number);
    }
    return field;
  }

  static bool IsMessageSet(const Descriptor* descriptor) {
    // MessageSet needs a special handling because it's a centralized proto that
    // is extended by many protos and can have a huge number of fields. Fuzzing
    // such a message could be quite expensive and leads to inefficient fuzzing.
    return descriptor->full_name() == "google.protobuf.bridge.MessageSet";
  }

  static auto GetFieldCount(const Descriptor* descriptor) {
    std::vector<const FieldDescriptor*> extensions;
    if (!IsMessageSet(descriptor)) {
      descriptor->file()->pool()->FindAllExtensions(descriptor, &extensions);
    }
    return descriptor->field_count() + extensions.size();
  }

  static auto GetProtobufFields(const Descriptor* descriptor) {
    std::vector<const FieldDescriptor*> fields;
    fields.reserve(descriptor->field_count());
    for (int i = 0; i < descriptor->field_count(); ++i) {
      fields.push_back(descriptor->field(i));
    }
    if (!IsMessageSet(descriptor)) {
      descriptor->file()->pool()->FindAllExtensions(descriptor, &fields);
    }
    return fields;
  }

  static auto GetFieldName(const FieldDescriptor* field) {
    return field->is_extension() ? field->full_name() : field->name();
  }

  void WithOneofField(absl::string_view field_name, OptionalPolicy policy) {
    const FieldDescriptor* field = GetField(field_name);
    if (!field->containing_oneof()) return;
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(
        policy != OptionalPolicy::kWithoutNull ||
            field->containing_oneof()->field_count() <= 1,
        "Cannot always set oneof field ", field_name,
        " (try using WithOneofAlwaysSet).");
    if (policy == OptionalPolicy::kAlwaysNull) {
      MarkOneofFieldAsUnset(field);
    }
  }

  void WithOneofFieldWithoutNullnessConfiguration(
      absl::string_view field_name) {
    const FieldDescriptor* field = GetField(field_name);
    auto* oneof = field->containing_oneof();
    if (!oneof) return;
    uncustomizable_oneofs_.insert(oneof->index());
  }

  void SetOneofAlwaysSet(absl::string_view oneof_name) {
    const std::string name(oneof_name);
    auto* oneof = prototype_.Get()->GetDescriptor()->FindOneofByName(name);
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(oneof != nullptr,
                                         "Invalid oneof name '", name, "'.");
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(
        !always_set_oneofs_.contains(oneof->index()), "oneof '", name,
        "' is AlwaysSet before.");
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(
        !uncustomizable_oneofs_.contains(oneof->index()),
        "WithOneofAlwaysSet(\"", name,
        "\") should be called before customizing sub-fields.");
    always_set_oneofs_.insert(oneof->index());
  }

  bool IsOneofAlwaysSet(int oneof_index) const {
    return always_set_oneofs_.contains(oneof_index);
  }

  struct WithFieldNullnessVisitor {
    ProtobufDomainUntypedImpl& self;
    OptionalPolicy policy;

    template <typename T>
    auto VisitSingular(const FieldDescriptor* field) {
      auto inner_domain =
          self.GetBaseDomainForFieldType<T>(field, /*use_policy=*/true);
      auto domain = self.GetOuterDomainForField</*is_repeated=*/false>(
          field, std::move(inner_domain));
      if (policy == OptionalPolicy::kAlwaysNull) {
        domain.SetAlwaysNull();
      } else if (policy == OptionalPolicy::kWithoutNull) {
        domain.SetWithoutNull();
      }
      self.WithField(self.GetFieldName(field), domain);
    }

    template <typename T>
    auto VisitRepeated(const FieldDescriptor* field) {
      auto inner_domain =
          self.GetBaseDomainForFieldType<T>(field, /*use_policy=*/true);
      auto domain = self.GetOuterDomainForField</*is_repeated=*/true>(
          field, std::move(inner_domain));
      if (policy == OptionalPolicy::kAlwaysNull) {
        domain.WithMaxSize(0);
      } else if (policy == OptionalPolicy::kWithoutNull) {
        domain.WithMinSize(1);
      }
      self.WithField(self.GetFieldName(field), domain);
    }
  };

  void WithFieldNullness(absl::string_view field_name, OptionalPolicy policy) {
    const FieldDescriptor* field = GetField(field_name);
    WithOneofField(field_name, policy);
    VisitProtobufField(field, WithFieldNullnessVisitor{*this, policy});
  }

  struct WithRepeatedFieldSizeVisitor {
    ProtobufDomainUntypedImpl& self;
    std::optional<int64_t> min_size;
    std::optional<int64_t> max_size;

    template <typename T>
    auto VisitSingular(const FieldDescriptor* field) {
      FUZZTEST_INTERNAL_CHECK_PRECONDITION(
          false,
          "Customizing repeated field size is not applicable to non-repeated "
          "field ",
          field->full_name(), ".");
    }

    template <typename T>
    auto VisitRepeated(const FieldDescriptor* field) {
      auto inner_domain =
          self.GetBaseDomainForFieldType<T>(field, /*use_policy=*/true);
      auto domain = self.GetOuterDomainForField</*is_repeated=*/true>(
          field, std::move(inner_domain));
      if (min_size.has_value()) {
        domain.WithMinSize(*min_size);
      }
      if (max_size.has_value()) {
        domain.WithMaxSize(*max_size);
      }
      self.WithField(self.GetFieldName(field), domain);
    }
  };

  void SetRepeatedFieldSize(absl::string_view field_name,
                            std::optional<int64_t> min_size,
                            std::optional<int64_t> max_size) {
    const FieldDescriptor* field = GetField(field_name);
    VisitProtobufField(field,
                       WithRepeatedFieldSizeVisitor{*this, min_size, max_size});
  }

  void SetPolicy(ProtoPolicy<Message> policy) {
    CheckIfPolicyCanBeUpdated();
    policy_ = policy;
  }

  ProtoPolicy<Message>& GetPolicy() {
    CheckIfPolicyCanBeUpdated();
    return policy_;
  }

  template <typename T>
  auto GetFieldTypeDefaultDomain(absl::string_view field_name) const {
    auto* field = GetField(field_name);
    return GetBaseDomainForFieldType<T>(field, /*use_policy=*/true);
  }

  template <bool is_repeated, typename Inner>
  auto GetOuterDomainForField(const FieldDescriptor* field, Inner domain,
                              bool use_policy = true) const {
    if constexpr (is_repeated) {
      return ModifyDomainForRepeatedFieldRule(
          std::move(domain),
          use_policy ? policy_.GetMinRepeatedFieldSize(field) : std::nullopt,
          use_policy ? policy_.GetMaxRepeatedFieldSize(field) : std::nullopt);
    } else if (IsRequired(field)) {
      return ModifyDomainForRequiredFieldRule(std::move(domain));
    } else {
      return ModifyDomainForOptionalFieldRule(
          std::move(domain),
          use_policy
              ? (field->containing_oneof() ? GetOneofFieldPolicy(field)
                                           : policy_.GetOptionalPolicy(field))
              : OptionalPolicy::kWithNull);
    }
  }

  void MarkOneofFieldAsUnset(const FieldDescriptor* field) {
    unset_oneof_fields_.insert(field->index());
  }

  OptionalPolicy GetOneofFieldPolicy(const FieldDescriptor* field) const {
    FUZZTEST_INTERNAL_CHECK(
        field->containing_oneof(),
        "GetOneofFieldPolicy should apply to oneof fields only! ",
        field->full_name());
    auto field_policy = policy_.GetOptionalPolicy(field);
    // Field being unset via a policy overwrites the oneof policy.
    if (field_policy == OptionalPolicy::kAlwaysNull) return field_policy;
    auto result = unset_oneof_fields_.find(field->index());
    // Field being unset via field customization overwrites the oneof policy.
    if (result != unset_oneof_fields_.end()) return OptionalPolicy::kAlwaysNull;
    // Policy is set at oneof level.
    if (IsOneofAlwaysSet(field->containing_oneof()->index())) {
      return OptionalPolicy::kWithoutNull;
    }
    return field_policy;
  }

  void CheckIfPolicyCanBeUpdated() const {
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(
        customized_fields_.empty(),
        "All singular modifiers (i.e., .With_Field_()) should come after "
        "plural modifiers (i.e., .With_Fields_()). Consider reordering .With_ "
        "modifiers.");
  }
  // Get the existing domain for `field`, if exists.
  // Otherwise, create the appropriate `Arbitrary<>` domain for the field and
  // return it.
  template <typename T, bool is_repeated>
  auto& GetSubDomain(const FieldDescriptor* field) const {
    using DomainT = decltype(GetDefaultDomainForField<T, is_repeated>(field));
    // Do the operation under a lock to prevent race conditions in `const`
    // methods.
    absl::MutexLock l(&mutex_);
    auto it = domains_.find(field->number());
    if (it == domains_.end()) {
      it = domains_
               .try_emplace(field->number(), std::in_place_type<DomainT>,
                            GetDomainForField<T, is_repeated>(field))
               .first;
    }
    return it->second.template GetAs<DomainT>();
  }

  // Simple wrapper that converts a Domain<T> into a Domain<vector<T>>.
  template <typename T>
  static auto ModifyDomainForRepeatedFieldRule(
      const Domain<T>& d, std::optional<int64_t> min_size,
      std::optional<int64_t> max_size) {
    auto result = ContainerOfImpl<std::vector<T>, Domain<T>>(d);
    if (min_size.has_value()) {
      result.WithMinSize(*min_size);
    }
    if (max_size.has_value()) {
      result.WithMaxSize(*max_size);
    }
    return result;
  }

  template <typename T>
  static auto ModifyDomainForOptionalFieldRule(const Domain<T>& d,
                                               OptionalPolicy optional_policy) {
    auto result = OptionalOfImpl<std::optional<T>>(d);
    if (optional_policy == OptionalPolicy::kWithoutNull) {
      result.SetWithoutNull();
    } else if (optional_policy == OptionalPolicy::kAlwaysNull) {
      result.SetAlwaysNull();
    }
    return result;
  }

  template <typename T>
  static auto ModifyDomainForRequiredFieldRule(const Domain<T>& d) {
    return OptionalOfImpl<std::optional<T>>(d).SetWithoutNull();
  }

  // Returns the default "base domain" for a `field` solely based on its type
  // (i.e., int32/string), but ignoring the field rule (i.e., the
  // repeated/optional/required specifiers).
  template <typename T>
  auto GetBaseDefaultDomainForFieldType(const FieldDescriptor* field) const {
    if constexpr (std::is_same_v<T, std::string>) {
      if (field->type() == FieldDescriptor::TYPE_STRING) {
        // Can only use UTF-8. For now, simplify as just ASCII.
        return Domain<T>(AsciiString());
      }
    }

    if constexpr (std::is_same_v<T, ProtoEnumTag>) {
      // For enums, build the list of valid labels.
      auto* e = field->enum_type();
      std::vector<int> values;
      values.reserve(e->value_count());
      for (int i = 0; i < e->value_count(); ++i) {
        values.push_back(e->value(i)->number());
      }
      // Delay instantiation. The Domain class is not fully defined at this
      // point yet, and neither is ElementOfImpl.
      using LazyInt = MakeDependentType<int, T>;
      return Domain<LazyInt>(ElementOfImpl<LazyInt>(std::move(values)));
    } else if constexpr (std::is_same_v<T, ProtoMessageTag>) {
      auto result = ProtobufDomainUntypedImpl(
          prototype_.Get()->GetReflection()->GetMessageFactory()->GetPrototype(
              field->message_type()),
          use_lazy_initialization_);
      result.SetPolicy(policy_);
      return Domain<std::unique_ptr<Message>>(result);
    } else {
      return Domain<T>(ArbitraryImpl<T>());
    }
  }

  template <typename T>
  auto GetBaseDomainForFieldType(const FieldDescriptor* field,
                                 bool use_policy) const {
    auto default_domain = GetBaseDefaultDomainForFieldType<T>(field);
    if (!use_policy) {
      return default_domain;
    }
#define FUZZTEST_INTERNAL_RETURN_BASE_DOMAIN_IF_PROVIDED(Camel, type)       \
  if constexpr (std::is_same_v<T, type>) {                                  \
    auto default_domain_in_policy =                                         \
        policy_.GetDefaultDomainFor##Camel##s(field);                       \
    if (default_domain_in_policy.has_value()) {                             \
      default_domain = std::move(*default_domain_in_policy);                \
    }                                                                       \
    auto transformer_in_policy =                                            \
        policy_.GetDomainTransformerFor##Camel##s(field);                   \
    if (transformer_in_policy.has_value()) {                                \
      default_domain = (*transformer_in_policy)(std::move(default_domain)); \
    }                                                                       \
    return default_domain;                                                  \
  }

    FUZZTEST_INTERNAL_RETURN_BASE_DOMAIN_IF_PROVIDED(Bool, bool)
    FUZZTEST_INTERNAL_RETURN_BASE_DOMAIN_IF_PROVIDED(Int32, int32_t)
    FUZZTEST_INTERNAL_RETURN_BASE_DOMAIN_IF_PROVIDED(UInt32, uint32_t)
    FUZZTEST_INTERNAL_RETURN_BASE_DOMAIN_IF_PROVIDED(Int64, int64_t)
    FUZZTEST_INTERNAL_RETURN_BASE_DOMAIN_IF_PROVIDED(UInt64, uint64_t)
    FUZZTEST_INTERNAL_RETURN_BASE_DOMAIN_IF_PROVIDED(Float, float)
    FUZZTEST_INTERNAL_RETURN_BASE_DOMAIN_IF_PROVIDED(Double, double)
    FUZZTEST_INTERNAL_RETURN_BASE_DOMAIN_IF_PROVIDED(String, std::string)
    FUZZTEST_INTERNAL_RETURN_BASE_DOMAIN_IF_PROVIDED(Enum, ProtoEnumTag)
    FUZZTEST_INTERNAL_RETURN_BASE_DOMAIN_IF_PROVIDED(Protobuf, ProtoMessageTag)

    return GetBaseDefaultDomainForFieldType<T>(field);
  }

  template <typename T, bool is_repeated>
  auto GetDomainForField(const FieldDescriptor* field,
                         bool use_policy = true) const {
    auto base_domain = GetBaseDomainForFieldType<T>(field, use_policy);
    using field_cpptype = value_type_t<std::decay_t<decltype(base_domain)>>;
    if constexpr (is_repeated) {
      return Domain<std::vector<field_cpptype>>(
          GetOuterDomainForField<is_repeated>(field, base_domain, use_policy));
    } else {
      return Domain<std::optional<field_cpptype>>(
          GetOuterDomainForField<is_repeated>(field, base_domain, use_policy));
    }
  }

  template <typename T, bool is_repeated>
  auto GetDefaultDomainForField(const FieldDescriptor* field) const {
    return GetDomainForField<T, is_repeated>(field, /*use_policy=*/false);
  }

  // Analysis type for protobuf recursions.
  enum class RecursionType {
    // The proto contains a proto of type P, that must contain another P.
    kInfinitelyRecursive,
    // The proto contains a proto of type P, that can contain another P.
    kFinitelyRecursive,
  };

  // Returns true if there are subprotos in the `descriptor` that form an
  // infinite recursion.
  bool IsInfinitelyRecursive(const Descriptor* descriptor) const {
    FUZZTEST_INTERNAL_CHECK(IsCustomizedRecursivelyOnly(), "Internal error.");
    absl::flat_hash_set<const FieldDescriptor*> parents;
    return IsProtoRecursive(/*field=*/nullptr, parents,
                            RecursionType::kInfinitelyRecursive, descriptor);
  }

  // Returns true if there are subfields in the `field` that form an
  // infinite recursion of the form: F0 -> F1 -> ... -> Fs -> ... -> Fn -> Fs,
  // because all Fi-s have to be set (e.g., Fi is a required field, or is
  // customized using `WithFieldsAlwaysSet`).
  bool IsInfinitelyRecursive(const FieldDescriptor* field) const {
    FUZZTEST_INTERNAL_CHECK(IsCustomizedRecursivelyOnly(), "Internal error.");
    absl::flat_hash_set<const FieldDescriptor*> parents;
    return IsProtoRecursive(field, parents,
                            RecursionType::kInfinitelyRecursive);
  }

  bool IsFieldFinitelyRecursive(const FieldDescriptor* field) {
    FUZZTEST_INTERNAL_CHECK(IsCustomizedRecursivelyOnly(), "Internal error.");
    if (!field->message_type()) return false;
    ABSL_CONST_INIT static absl::Mutex mutex(absl::kConstInit);
    static absl::NoDestructor<
        absl::flat_hash_map<std::pair<int64_t, const FieldDescriptor*>, bool>>
        cache ABSL_GUARDED_BY(mutex);
    {
      absl::MutexLock l(&mutex);
      auto it = cache->find({policy_.id(), field});
      if (it != cache->end()) return it->second;
    }
    absl::flat_hash_set<const FieldDescriptor*> parents;
    bool result =
        IsProtoRecursive(field, parents, RecursionType::kFinitelyRecursive);
    absl::MutexLock l(&mutex);
    cache->insert({{policy_.id(), field}, result});
    return result;
  }

  // corpus_type is a map from field number to values. number -1 is reserved for
  // storing the field count.
  static constexpr int64_t kFieldCountIndex = -1;

  static bool IsMetadataEntry(int64_t index) {
    return index == kFieldCountIndex;
  }

  bool IsOneofRecursive(const OneofDescriptor* oneof,
                        absl::flat_hash_set<const FieldDescriptor*>& parents,
                        RecursionType recursion_type) const {
    bool is_oneof_recursive = false;
    for (int i = 0; i < oneof->field_count(); ++i) {
      const auto* field = oneof->field(i);
      const auto field_policy = policy_.GetOptionalPolicy(field);
      if (field_policy == OptionalPolicy::kAlwaysNull) continue;
      if (recursion_type == RecursionType::kInfinitelyRecursive) {
        is_oneof_recursive = field_policy != OptionalPolicy::kWithNull &&
                             field->message_type() &&
                             IsProtoRecursive(field, parents, recursion_type);
        if (!is_oneof_recursive) {
          return false;
        }
      } else {
        if (field->message_type() &&
            IsProtoRecursive(field, parents, recursion_type)) {
          return true;
        }
      }
    }
    return is_oneof_recursive;
  }

  bool MustBeSet(const FieldDescriptor* field) const {
    FUZZTEST_INTERNAL_CHECK(IsCustomizedRecursivelyOnly(), "Internal error.");
    if (IsRequired(field)) {
      return true;
    } else if (field->containing_oneof()) {
      return GetOneofFieldPolicy(field) == OptionalPolicy::kWithoutNull;
    } else if (field->is_optional()) {
      return policy_.GetOptionalPolicy(field) == OptionalPolicy::kWithoutNull;
    } else if (field->is_repeated()) {
      return policy_.GetMinRepeatedFieldSize(field).has_value() &&
             *policy_.GetMinRepeatedFieldSize(field) > 0;
    }
    FUZZTEST_INTERNAL_CHECK(false,
                            "Field is not optional, repeated, or required");
    return false;
  }

  bool MustBeUnset(const FieldDescriptor* field) const {
    FUZZTEST_INTERNAL_CHECK(IsCustomizedRecursivelyOnly(), "Internal error.");
    if (field->message_type() && IsInfinitelyRecursive(field)) {
      absl::FPrintF(
          GetStderr(),
          "[!] Infinite recursion detected for %s and it remains unset.\n",
          field->full_name());
      return true;
    }
    if (IsRequired(field)) {
      return false;
    } else if (field->containing_oneof()) {
      return GetOneofFieldPolicy(field) == OptionalPolicy::kAlwaysNull;
    } else if (field->is_optional()) {
      return policy_.GetOptionalPolicy(field) == OptionalPolicy::kAlwaysNull;
    } else if (field->is_repeated()) {
      return policy_.GetMaxRepeatedFieldSize(field).has_value() &&
             *policy_.GetMaxRepeatedFieldSize(field) == 0;
    }
    FUZZTEST_INTERNAL_CHECK(false,
                            "Field is not optional, repeated, or required");
    return false;
  }

  // If `field` is nullptr, all fields of `descriptor` are checked.
  bool IsProtoRecursive(const FieldDescriptor* field,
                        absl::flat_hash_set<const FieldDescriptor*>& parents,
                        RecursionType recursion_type,
                        const Descriptor* descriptor = nullptr) const {
    if (field != nullptr) {
      if (parents.contains(field)) return true;
      parents.insert(field);
      descriptor = field->message_type();
    } else {
      FUZZTEST_INTERNAL_CHECK(descriptor,
                              "one of field or descriptor must be non-null!");
    }
    for (int i = 0; i < descriptor->oneof_decl_count(); ++i) {
      const auto* oneof = descriptor->oneof_decl(i);
      if (IsOneofRecursive(oneof, parents, recursion_type)) {
        if (field != nullptr) parents.erase(field);
        return true;
      }
    }
    for (const FieldDescriptor* subfield : GetProtobufFields(descriptor)) {
      if (subfield->containing_oneof()) continue;
      if (!subfield->message_type()) continue;
      if (auto default_domain = policy_.GetDefaultDomainForProtobufs(subfield);
          default_domain != std::nullopt) {  // For handling WithProtobufFields.
        // If this field is recursive, it will be detected when initializing
        // its default domain. Otherwise, this field can always be set safely.
        absl::BitGen prng;
        default_domain->Init(prng);
        continue;
      }
      if (recursion_type == RecursionType::kInfinitelyRecursive) {
        if (!MustBeSet(subfield)) continue;
      } else {
        if (MustBeUnset(subfield)) continue;
      }
      if (IsProtoRecursive(subfield, parents, recursion_type)) {
        if (field != nullptr) parents.erase(field);
        return true;
      }
    }
    if (field != nullptr) parents.erase(field);
    return false;
  }

  bool IsRequired(const FieldDescriptor* field) const {
    return field->is_required() || IsMapValueMessage(field);
  }

  static bool IsMapValueMessage(const FieldDescriptor* field) {
    return field->message_type() &&
           field->containing_type()->map_value() == field;
  }

  // Returns true if all domain customizations are defined recursively (through
  // policy) and no individual field is customized at the top level. This check
  // would be useful in recursion analysis. In particular, recursion analysis
  // is only meaningful when all customizations are also recursive.
  bool IsCustomizedRecursivelyOnly() const {
    return customized_fields_.empty() && always_set_oneofs_.empty() &&
           uncustomizable_oneofs_.empty() && unset_oneof_fields_.empty();
  }

  PrototypePtr<Message> prototype_;
  bool use_lazy_initialization_;

  mutable absl::Mutex mutex_;
  mutable absl::flat_hash_map<int, CopyableAny> domains_
      ABSL_GUARDED_BY(mutex_);

  ProtoPolicy<Message> policy_;
  absl::flat_hash_set<int> customized_fields_;
  absl::flat_hash_set<int> always_set_oneofs_;
  absl::flat_hash_set<int> uncustomizable_oneofs_;
  absl::flat_hash_set<int> unset_oneof_fields_;
};

// Domain for `T` where `T` is a Protobuf message type.
// It is a small wrapper around `ProtobufDomainUntypedImpl` to make its API more
// convenient.
template <typename T,
          typename UntypedImpl = ProtobufDomainUntypedImpl<typename T::Message>>
class ProtobufDomainImpl
    : public domain_implementor::DomainBase<ProtobufDomainImpl<T>, T,
                                            corpus_type_t<UntypedImpl>> {
 public:
  using typename ProtobufDomainImpl::DomainBase::corpus_type;
  using typename ProtobufDomainImpl::DomainBase::value_type;
  using FieldDescriptor = ProtobufFieldDescriptor<typename T::Message>;

  corpus_type Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    return inner_.Init(prng);
  }

  uint64_t CountNumberOfFields(corpus_type& val) {
    return inner_.CountNumberOfFields(val);
  }

  uint64_t MutateNumberOfProtoFields(corpus_type& val) {
    return inner_.MutateNumberOfProtoFields(val);
  }

  void Mutate(corpus_type& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    inner_.Mutate(val, prng, metadata, only_shrink);
  }

  value_type GetValue(const corpus_type& v) const {
    auto inner_v = inner_.GetValue(v);
    return std::move(static_cast<T&>(*inner_v));
  }

  std::optional<corpus_type> FromValue(const value_type& value) const {
    return inner_.FromValue(value);
  }

  auto GetPrinter() const { return ProtobufPrinter{}; }

  std::optional<corpus_type> ParseCorpus(const IRObject& obj) const {
    return inner_.ParseCorpus(obj);
  }

  IRObject SerializeCorpus(const corpus_type& v) const {
    return inner_.SerializeCorpus(v);
  }

  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    return inner_.ValidateCorpusValue(corpus_value);
  }

  // Provide a conversion to the type that WithMessageField wants.
  // Makes it easier on the user.
  operator Domain<std::unique_ptr<typename T::Message>>() const {
    return inner_;
  }

  ProtobufDomainImpl&& Self() && { return std::move(*this); }

  ProtobufDomainImpl&& WithFieldsAlwaysSet(
      std::function<bool(const FieldDescriptor*)> filter =
          IncludeAll<FieldDescriptor>()) && {
    inner_.GetPolicy().SetOptionalPolicy(std::move(filter),
                                         OptionalPolicy::kWithoutNull);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithFieldsUnset(
      std::function<bool(const FieldDescriptor*)> filter =
          IncludeAll<FieldDescriptor>()) && {
    inner_.GetPolicy().SetOptionalPolicy(std::move(filter),
                                         OptionalPolicy::kAlwaysNull);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithOptionalFieldsAlwaysSet(
      std::function<bool(const FieldDescriptor*)> filter =
          IncludeAll<FieldDescriptor>()) && {
    inner_.GetPolicy().SetOptionalPolicy(
        And(IsOptional<FieldDescriptor>(), std::move(filter)),
        OptionalPolicy::kWithoutNull);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithOptionalFieldsUnset(
      std::function<bool(const FieldDescriptor*)> filter =
          IncludeAll<FieldDescriptor>()) && {
    inner_.GetPolicy().SetOptionalPolicy(
        And(IsOptional<FieldDescriptor>(), std::move(filter)),
        OptionalPolicy::kAlwaysNull);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithRepeatedFieldsAlwaysSet(
      std::function<bool(const FieldDescriptor*)> filter =
          IncludeAll<FieldDescriptor>()) && {
    inner_.GetPolicy().SetOptionalPolicy(
        And(IsRepeated<FieldDescriptor>(), std::move(filter)),
        OptionalPolicy::kWithoutNull);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithRepeatedFieldsUnset(
      std::function<bool(const FieldDescriptor*)> filter =
          IncludeAll<FieldDescriptor>()) && {
    inner_.GetPolicy().SetOptionalPolicy(
        And(IsRepeated<FieldDescriptor>(), std::move(filter)),
        OptionalPolicy::kAlwaysNull);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithRepeatedFieldsSize(int64_t size) && {
    return std::move(*this)
        .WithRepeatedFieldsMinSize(size)
        .WithRepeatedFieldsMaxSize(size);
  }

  ProtobufDomainImpl&& WithRepeatedFieldsSize(
      std::function<bool(const FieldDescriptor*)> filter, int64_t size) && {
    return std::move(*this)
        .WithRepeatedFieldsMinSize(filter, size)
        .WithRepeatedFieldsMaxSize(filter, size);
  }

  ProtobufDomainImpl&& WithRepeatedFieldsMinSize(int64_t min_size) && {
    inner_.GetPolicy().SetMinRepeatedFieldsSize(IncludeAll<FieldDescriptor>(),
                                                min_size);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithRepeatedFieldsMinSize(
      std::function<bool(const FieldDescriptor*)> filter, int64_t min_size) && {
    inner_.GetPolicy().SetMinRepeatedFieldsSize(std::move(filter), min_size);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithRepeatedFieldsMaxSize(int64_t max_size) && {
    inner_.GetPolicy().SetMaxRepeatedFieldsSize(IncludeAll<FieldDescriptor>(),
                                                max_size);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithRepeatedFieldsMaxSize(
      std::function<bool(const FieldDescriptor*)> filter, int64_t max_size) && {
    inner_.GetPolicy().SetMaxRepeatedFieldsSize(std::move(filter), max_size);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithFieldUnset(absl::string_view field) && {
    inner_.WithFieldNullness(field, OptionalPolicy::kAlwaysNull);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithFieldAlwaysSet(absl::string_view field) && {
    inner_.WithFieldNullness(field, OptionalPolicy::kWithoutNull);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithRepeatedFieldSize(
      absl::string_view field_name, std::optional<int64_t> min_size,
      std::optional<int64_t> max_size) && {
    inner_.SetRepeatedFieldSize(field_name, min_size, max_size);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithRepeatedFieldMinSize(absl::string_view field_name,
                                                int64_t min_size) && {
    inner_.SetRepeatedFieldSize(field_name, min_size, std::nullopt);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithRepeatedFieldMaxSize(absl::string_view field_name,
                                                int64_t max_size) && {
    inner_.SetRepeatedFieldSize(field_name, std::nullopt, max_size);
    return std::move(*this);
  }

  ProtobufDomainImpl&& WithOneofAlwaysSet(absl::string_view oneof_name) && {
    inner_.SetOneofAlwaysSet(oneof_name);
    return std::move(*this);
  }

#define FUZZTEST_INTERNAL_WITH_FIELD(Camel, cpp, TAG)                          \
  using Camel##type = MakeDependentType<cpp, T>;                               \
  ProtobufDomainImpl&& With##Camel##Field(absl::string_view field,             \
                                          Domain<Camel##type> domain)&& {      \
    const FieldDescriptor* descriptor = inner_.GetField(field);                \
    if (descriptor->is_repeated()) {                                           \
      inner_.WithField(                                                        \
          field, inner_.template GetOuterDomainForField</*is_repeated=*/true>( \
                     descriptor, std::move(domain)));                          \
    } else {                                                                   \
      inner_.WithOneofFieldWithoutNullnessConfiguration(field);                \
      inner_.WithField(                                                        \
          field,                                                               \
          inner_.template GetOuterDomainForField</*is_repeated=*/false>(       \
              descriptor, std::move(domain)));                                 \
    }                                                                          \
    return std::move(*this);                                                   \
  }                                                                            \
  ProtobufDomainImpl&& With##Camel##FieldAlwaysSet(                            \
      absl::string_view field, Domain<Camel##type> domain)&& {                 \
    const FieldDescriptor* descriptor = inner_.GetField(field);                \
    if (descriptor->is_repeated()) {                                           \
      inner_.WithField(                                                        \
          field,                                                               \
          SequenceContainerOfImpl<std::vector<Camel##type>, decltype(domain)>( \
              std::move(domain))                                               \
              .WithMinSize(1));                                                \
    } else {                                                                   \
      inner_.WithOneofField(field, OptionalPolicy::kWithoutNull);              \
      inner_.WithField(                                                        \
          field, OptionalOfImpl<std::optional<Camel##type>>(std::move(domain)) \
                     .SetWithoutNull());                                       \
    }                                                                          \
    return std::move(*this);                                                   \
  }                                                                            \
  /* TODO(b/271123298): Remove the following two methods and replace them with \
  WithField(Unset/AlwaysSet) */                                                \
  ProtobufDomainImpl&& With##Camel##FieldUnset(absl::string_view field)&& {    \
    auto default_domain =                                                      \
        inner_.template GetFieldTypeDefaultDomain<TAG>(field);                 \
    inner_.WithOneofField(field, OptionalPolicy::kAlwaysNull);                 \
    inner_.WithField(field, OptionalOfImpl<std::optional<Camel##type>>(        \
                                std::move(default_domain))                     \
                                .SetAlwaysNull());                             \
    return std::move(*this);                                                   \
  }                                                                            \
  ProtobufDomainImpl&& With##Camel##FieldAlwaysSet(                            \
      absl::string_view field)&& {                                             \
    return std::move(*this).With##Camel##FieldAlwaysSet(                       \
        field, inner_.template GetFieldTypeDefaultDomain<TAG>(field));         \
  }                                                                            \
  ProtobufDomainImpl&& WithOptional##Camel##Field(                             \
      absl::string_view field,                                                 \
      Domain<MakeDependentType<std::optional<cpp>, T>> domain)&& {             \
    FailIfIsOneof(field);                                                      \
    inner_.WithField(field, std::move(domain));                                \
    return std::move(*this);                                                   \
  }                                                                            \
  ProtobufDomainImpl&& WithRepeated##Camel##Field(                             \
      absl::string_view field,                                                 \
      Domain<MakeDependentType<std::vector<cpp>, T>> domain)&& {               \
    inner_.WithField(field, std::move(domain));                                \
    return std::move(*this);                                                   \
  }                                                                            \
  ProtobufDomainImpl&& With##Camel##Fields(Domain<Camel##type> domain)&& {     \
    inner_.GetPolicy().SetDefaultDomainFor##Camel##s(                          \
        IncludeAll<FieldDescriptor>(), std::move(domain));                     \
    return std::move(*this);                                                   \
  }                                                                            \
  ProtobufDomainImpl&& With##Camel##Fields(                                    \
      std::function<bool(const FieldDescriptor*)>&& filter,                    \
      Domain<Camel##type> domain)&& {                                          \
    inner_.GetPolicy().SetDefaultDomainFor##Camel##s(std::move(filter),        \
                                                     std::move(domain));       \
    return std::move(*this);                                                   \
  }                                                                            \
  ProtobufDomainImpl&& WithOptional##Camel##Fields(                            \
      Domain<Camel##type> domain)&& {                                          \
    inner_.GetPolicy().SetDefaultDomainFor##Camel##s(                          \
        IsOptional<FieldDescriptor>(), std::move(domain));                     \
    return std::move(*this);                                                   \
  }                                                                            \
  ProtobufDomainImpl&& WithOptional##Camel##Fields(                            \
      std::function<bool(const FieldDescriptor*)>&& filter,                    \
      Domain<Camel##type> domain)&& {                                          \
    inner_.GetPolicy().SetDefaultDomainFor##Camel##s(                          \
        And(IsOptional<FieldDescriptor>(), std::move(filter)),                 \
        std::move(domain));                                                    \
    return std::move(*this);                                                   \
  }                                                                            \
  ProtobufDomainImpl&& WithRepeated##Camel##Fields(                            \
      Domain<Camel##type> domain)&& {                                          \
    inner_.GetPolicy().SetDefaultDomainFor##Camel##s(                          \
        IsRepeated<FieldDescriptor>(), std::move(domain));                     \
    return std::move(*this);                                                   \
  }                                                                            \
  ProtobufDomainImpl&& WithRepeated##Camel##Fields(                            \
      std::function<bool(const FieldDescriptor*)>&& filter,                    \
      Domain<Camel##type> domain)&& {                                          \
    inner_.GetPolicy().SetDefaultDomainFor##Camel##s(                          \
        And(IsRepeated<FieldDescriptor>(), std::move(filter)),                 \
        std::move(domain));                                                    \
    return std::move(*this);                                                   \
  }                                                                            \
  ProtobufDomainImpl&& With##Camel##FieldsTransformed(                         \
      std::function<Domain<Camel##type>(Domain<Camel##type>)>&&                \
          transformer)&& {                                                     \
    inner_.GetPolicy().SetDomainTransformerFor##Camel##s(                      \
        IncludeAll<FieldDescriptor>(), std::move(transformer));                \
    return std::move(*this);                                                   \
  }                                                                            \
  ProtobufDomainImpl&& With##Camel##FieldsTransformed(                         \
      std::function<bool(const FieldDescriptor*)>&& filter,                    \
      std::function<Domain<Camel##type>(Domain<Camel##type>)>&&                \
          transformer)&& {                                                     \
    inner_.GetPolicy().SetDomainTransformerFor##Camel##s(                      \
        std::move(filter), std::move(transformer));                            \
    return std::move(*this);                                                   \
  }

  FUZZTEST_INTERNAL_WITH_FIELD(Bool, bool, bool)
  FUZZTEST_INTERNAL_WITH_FIELD(Int32, int32_t, int32_t)
  FUZZTEST_INTERNAL_WITH_FIELD(UInt32, uint32_t, uint32_t)
  FUZZTEST_INTERNAL_WITH_FIELD(Int64, int64_t, int64_t)
  FUZZTEST_INTERNAL_WITH_FIELD(UInt64, uint64_t, uint64_t)
  FUZZTEST_INTERNAL_WITH_FIELD(Float, float, float)
  FUZZTEST_INTERNAL_WITH_FIELD(Double, double, double)
  FUZZTEST_INTERNAL_WITH_FIELD(String, std::string, std::string)
  FUZZTEST_INTERNAL_WITH_FIELD(Enum, int, ProtoEnumTag)
  FUZZTEST_INTERNAL_WITH_FIELD(Protobuf, std::unique_ptr<typename T::Message>,
                               ProtoMessageTag)

#undef FUZZTEST_INTERNAL_WITH_FIELD

  // The following methods automatically cast Domain<Proto> to
  // Domain<unique_ptr<Message>>

  template <typename Protobuf>
  ProtobufDomainImpl&& WithProtobufField(absl::string_view field,
                                         Domain<Protobuf> domain) && {
    return std::move(*this).WithProtobufField(
        field, inner_.ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename Protobuf>
  ProtobufDomainImpl&& WithProtobufFieldAlwaysSet(absl::string_view field,
                                                  Domain<Protobuf> domain) && {
    return std::move(*this).WithProtobufFieldAlwaysSet(
        field, inner_.ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename Protobuf>
  ProtobufDomainImpl&& WithProtobufFields(Domain<Protobuf> domain) && {
    return std::move(*this).WithProtobufFields(
        inner_.ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename Protobuf>
  ProtobufDomainImpl&& WithProtobufFields(
      std::function<bool(const FieldDescriptor*)>&& filter,
      Domain<Protobuf> domain) && {
    return std::move(*this).WithProtobufFields(
        std::move(filter), inner_.ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename Protobuf>
  ProtobufDomainImpl&& WithOptionalProtobufFields(Domain<Protobuf> domain) && {
    return std::move(*this).WithOptionalProtobufFields(
        inner_.ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename Protobuf>
  ProtobufDomainImpl&& WithOptionalProtobufFields(
      std::function<bool(const FieldDescriptor*)>&& filter,
      Domain<Protobuf> domain) && {
    return std::move(*this).WithOptionalProtobufFields(
        std::move(filter), inner_.ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename Protobuf>
  ProtobufDomainImpl&& WithRepeatedProtobufFields(Domain<Protobuf> domain) && {
    return std::move(*this).WithRepeatedProtobufFields(
        inner_.ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename Protobuf>
  ProtobufDomainImpl&& WithRepeatedProtobufFields(
      std::function<bool(const FieldDescriptor*)>&& filter,
      Domain<Protobuf> domain) && {
    return std::move(*this).WithRepeatedProtobufFields(
        std::move(filter), inner_.ToUntypedProtoDomain(std::move(domain)));
  }

  template <typename OptionalProtobufDomain>
  ProtobufDomainImpl&& WithOptionalProtobufField(
      absl::string_view field, OptionalProtobufDomain domain) && {
    return std::move(*this).WithOptionalProtobufField(
        field, ToOptionalUntypedProtoDomain(std::move(domain)));
  }

  template <typename RepeatedProtobufDomain>
  ProtobufDomainImpl&& WithRepeatedProtobufField(
      absl::string_view field, RepeatedProtobufDomain domain) && {
    return std::move(*this).WithRepeatedProtobufField(
        field, ToRepeatedUntypedProtoDomain(std::move(domain)));
  }

 private:
  void FailIfIsOneof(absl::string_view field) {
    const FieldDescriptor* descriptor = inner_.GetField(field);
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(
        !descriptor->containing_oneof(), "Cannot customize oneof field ", field,
        " with WithOptional<Type>Field (try using "
        "WithOneofAlwaysSet or WithOptional<Type>Unset).");
  }

  template <typename Inner>
  Domain<std::optional<std::unique_ptr<typename T::Message>>>
  ToOptionalUntypedProtoDomain(Inner inner_domain) {
    return ReversibleMap(
        [](value_type_t<Inner> proto_message)
            -> std::optional<std::unique_ptr<typename T::Message>> {
          if (!proto_message.has_value()) return std::nullopt;
          return std::make_unique<
              std::remove_reference_t<decltype(*proto_message)>>(
              *proto_message);
        },
        [](const std::optional<std::unique_ptr<typename T::Message>>&
               proto_message)
            -> std::optional<std::tuple<value_type_t<Inner>>> {
          if (!proto_message.has_value()) return std::nullopt;
          return *static_cast<
              std::add_pointer_t<typename value_type_t<Inner>::value_type>>(
              proto_message->get());
        },
        std::move(inner_domain));
  }

  template <typename Inner>
  Domain<std::vector<std::unique_ptr<typename T::Message>>>
  ToRepeatedUntypedProtoDomain(Inner inner_domain) {
    return ReversibleMap(
        [](value_type_t<Inner> proto_message)
            -> std::vector<std::unique_ptr<typename T::Message>> {
          std::vector<std::unique_ptr<typename T::Message>> result;
          for (auto& entry : proto_message) {
            result.push_back(
                std::make_unique<std::remove_const_t<
                    std::remove_reference_t<decltype(entry)>>>(entry));
          }
          return result;
        },
        [](const std::vector<std::unique_ptr<typename T::Message>>&
               proto_message)
            -> std::optional<std::tuple<value_type_t<Inner>>> {
          value_type_t<Inner> result;
          for (auto& entry : proto_message) {
            result.push_back(
                *(static_cast<std::add_pointer_t<
                      typename value_type_t<Inner>::value_type>>(entry.get())));
          }
          return result;
        },
        std::move(inner_domain));
  }

  UntypedImpl inner_{&T::default_instance(), /*use_lazy_initialization=*/true};
};

template <typename T>
class ArbitraryImpl<T, std::enable_if_t<is_protocol_buffer_v<T>>>
    : public ProtobufDomainImpl<T> {};

template <typename T>
class ArbitraryImpl<T, std::enable_if_t<is_protocol_buffer_enum_v<T>>>
    : public domain_implementor::DomainBase<ArbitraryImpl<T>> {
 public:
  using typename ArbitraryImpl::DomainBase::value_type;

  value_type Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    const int index = absl::Uniform(prng, 0, descriptor()->value_count());
    return static_cast<T>(descriptor()->value(index)->number());
  }

  void Mutate(value_type& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata&, bool only_shrink) {
    if (only_shrink) {
      std::vector<int> numbers;
      for (int i = 0; i < descriptor()->value_count(); ++i) {
        if (int n = descriptor()->value(i)->number(); n < val)
          numbers.push_back(n);
      }
      if (numbers.empty()) return;
      size_t idx = absl::Uniform<size_t>(prng, 0, numbers.size());
      val = static_cast<T>(numbers[idx]);
      return;
    } else if (descriptor()->value_count() == 1) {
      return;
    }

    // Make sure Mutate really mutates.
    const T prev = val;
    do {
      val = Init(prng);
    } while (val == prev);
  }

  auto GetPrinter() const {
    return ProtobufEnumPrinter<decltype(descriptor())>{descriptor()};
  }

  absl::Status ValidateCorpusValue(const value_type&) const {
    return absl::OkStatus();  // Any number is fine.
  }

 private:
  auto descriptor() const {
    static auto const descriptor_ = google::protobuf::GetEnumDescriptor<T>();
    return descriptor_;
  }
};

}  // namespace fuzztest::internal

#ifdef _WIN32
#pragma pop_macro("GetMessage")
#endif

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_PROTOBUF_DOMAIN_IMPL_H_
