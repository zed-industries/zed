//
//
// Copyright 2016 gRPC authors.
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
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_LIB_SURFACE_CHANNEL_INIT_H
#define GRPC_SRC_CORE_LIB_SURFACE_CHANNEL_INIT_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <initializer_list>
#include <memory>
#include <ostream>
#include <utility>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/log/check.h"
#include "src/core/call/call_filters.h"
#include "src/core/call/interception_chain.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/channel_stack_builder.h"
#include "src/core/lib/surface/channel_stack_type.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/unique_type_name.h"

/// This module provides a way for plugins (and the grpc core library itself)
/// to register mutators for channel stacks.
/// It also provides a universal entry path to run those mutators to build
/// a channel stack for various subsystems.

namespace grpc_core {

// HACK HACK HACK
// Right now grpc_channel_filter has a bunch of dependencies high in the stack,
// but this code needs to live as a dependency of CoreConfiguration so we need
// to be careful to ensure no dependency loops.
//
// We absolutely must be able to get the name from a filter - for stability and
// for debuggability.
//
// So we export this function, and have it filled in by the higher level code at
// static initialization time.
//
// TODO(ctiller): remove this. When we define a FilterFactory type, that type
// can be specified with the right constraints to be depended upon by this code,
// and that type can export a `string_view Name()` method.
extern UniqueTypeName (*NameFromChannelFilter)(const grpc_channel_filter*);

class ChannelInit {
 private:
  // Version constraints: filters can be registered against a specific version
  // of the stack (V2 || V3), or registered for any stack.
  enum class Version : uint8_t {
    kAny,
    kV2,
    kV3,
  };
  static const char* VersionToString(Version version) {
    switch (version) {
      case Version::kAny:
        return "Any";
      case Version::kV2:
        return "V2";
      case Version::kV3:
        return "V3";
    }
    return "Unknown";
  }
  template <typename Sink>
  friend void AbslStringify(Sink& sink, Version version) {
    sink.Append(VersionToString(version));
  }
  friend std::ostream& operator<<(std::ostream& out, Version version) {
    return out << VersionToString(version);
  }
  static bool SkipV3(Version version) {
    switch (version) {
      case Version::kAny:
      case Version::kV3:
        return false;
      case Version::kV2:
        return true;
    }
    GPR_UNREACHABLE_CODE(return false);
  }
  static bool SkipV2(Version version) {
    switch (version) {
      case Version::kAny:
      case Version::kV2:
        return false;
      case Version::kV3:
        return true;
    }
    GPR_UNREACHABLE_CODE(return false);
  }

 public:
  // Predicate for if a filter registration applies
  using InclusionPredicate = absl::AnyInvocable<bool(const ChannelArgs&) const>;
  // Post processor for the channel stack - applied in PostProcessorSlot order
  using PostProcessor = absl::AnyInvocable<void(ChannelStackBuilder&) const>;
  // Function that can be called to add a filter to a stack builder
  using FilterAdder = void (*)(InterceptionChainBuilder&);
  // Post processing slots - up to one PostProcessor per slot can be registered
  // They run after filters registered are added to the channel stack builder,
  // but before Build is called - allowing ad-hoc mutation to the channel stack.
  enum class PostProcessorSlot : uint8_t {
    kAuthSubstitution,
    kXdsChannelStackModifier,
    kCount
  };
  static const char* PostProcessorSlotName(PostProcessorSlot slot) {
    switch (slot) {
      case PostProcessorSlot::kAuthSubstitution:
        return "AuthSubstitution";
      case PostProcessorSlot::kXdsChannelStackModifier:
        return "XdsChannelStackModifier";
      case PostProcessorSlot::kCount:
        return "---count---";
    }
    return "Unknown";
  }
  template <typename Sink>
  friend void AbslStringify(Sink& sink, PostProcessorSlot slot) {
    sink.Append(PostProcessorSlotName(slot));
  }
  // Ordering priorities.
  // Most filters should use the kDefault priority.
  // Filters that need to appear before the default priority should use kTop,
  // filters that need to appear later should use the kBottom priority.
  // Explicit before/after ordering between filters dominates: eg, if a filter
  // with kBottom priority is marked as *BEFORE* a kTop filter, then the first
  // filter will appear before the second.
  // It is an error to have two filters with kTop (or two with kBottom)
  // available at the same time. If this occurs, the filters should be
  // explicitly marked with a before/after relationship.
  enum class Ordering : uint8_t { kTop, kDefault, kBottom };
  static const char* OrderingToString(Ordering ordering) {
    switch (ordering) {
      case Ordering::kTop:
        return "Top";
      case Ordering::kDefault:
        return "Default";
      case Ordering::kBottom:
        return "Bottom";
    }
    return "Unknown";
  }
  template <typename Sink>
  friend void AbslStringify(Sink& sink, Ordering ordering) {
    sink.Append(OrderingToString(ordering));
  };
  friend std::ostream& operator<<(std::ostream& out, Ordering ordering) {
    return out << OrderingToString(ordering);
  }

  class FilterRegistration {
   public:
    // TODO(ctiller): Remove grpc_channel_filter* arg when that can be
    // deprecated (once filter stack is removed).
    explicit FilterRegistration(UniqueTypeName name,
                                const grpc_channel_filter* filter,
                                FilterAdder filter_adder,
                                SourceLocation registration_source)
        : name_(name),
          filter_(filter),
          filter_adder_(filter_adder),
          registration_source_(registration_source) {}
    FilterRegistration(const FilterRegistration&) = delete;
    FilterRegistration& operator=(const FilterRegistration&) = delete;

    // Ensure that this filter is placed *after* the filters listed here.
    // By Build() time all filters listed here must also be registered against
    // the same channel stack type as this registration.
    template <typename Filter>
    FilterRegistration& After() {
      return After({UniqueTypeNameFor<Filter>()});
    }
    // Ensure that this filter is placed *before* the filters listed here.
    // By Build() time all filters listed here must also be registered against
    // the same channel stack type as this registration.
    template <typename Filter>
    FilterRegistration& Before() {
      return Before({UniqueTypeNameFor<Filter>()});
    }

    // Ensure that this filter is placed *after* the filters listed here.
    // By Build() time all filters listed here must also be registered against
    // the same channel stack type as this registration.
    // TODO(ctiller): remove in favor of the version that does not mention
    // grpc_channel_filter
    FilterRegistration& After(std::initializer_list<UniqueTypeName> filters);
    // Ensure that this filter is placed *before* the filters listed here.
    // By Build() time all filters listed here must also be registered against
    // the same channel stack type as this registration.
    // TODO(ctiller): remove in favor of the version that does not mention
    // grpc_channel_filter
    FilterRegistration& Before(std::initializer_list<UniqueTypeName> filters);
    // Add a predicate for this filters inclusion.
    // If the predicate returns true the filter will be included in the stack.
    // Predicates do not affect the ordering of the filter stack: we first
    // topologically sort (once, globally) and only later apply predicates
    // per-channel creation.
    // Multiple predicates can be added to each registration.
    FilterRegistration& If(InclusionPredicate predicate);
    FilterRegistration& IfNot(InclusionPredicate predicate);
    // Add a predicate that only includes this filter if a channel arg is
    // present.
    FilterRegistration& IfHasChannelArg(const char* arg);
    // Add a predicate that only includes this filter if a boolean channel arg
    // is true (with default_value being used if the argument is not present).
    FilterRegistration& IfChannelArg(const char* arg, bool default_value);
    // Mark this filter as being terminal.
    // Exactly one terminal filter will be added at the end of each filter
    // stack.
    // If multiple are defined they are tried in registration order, and the
    // first terminal filter whose predicates succeed is selected.
    FilterRegistration& Terminal() {
      terminal_ = true;
      return *this;
    }
    // Ensure this filter appears at the top of the stack.
    // Effectively adds a 'Before' constraint on every other filter.
    // Adding this to more than one filter will cause a loop.
    FilterRegistration& BeforeAll() {
      before_all_ = true;
      return *this;
    }
    // Add a predicate that ensures this filter does not appear in the minimal
    // stack.
    FilterRegistration& ExcludeFromMinimalStack();
    FilterRegistration& SkipV3() {
      CHECK_EQ(version_, Version::kAny);
      version_ = Version::kV2;
      return *this;
    }
    FilterRegistration& SkipV2() {
      CHECK_EQ(version_, Version::kAny);
      version_ = Version::kV3;
      return *this;
    }
    // Request this filter be placed as high as possible in the stack (given
    // before/after constraints).
    FilterRegistration& FloatToTop() {
      CHECK_EQ(ordering_, Ordering::kDefault);
      ordering_ = Ordering::kTop;
      return *this;
    }
    // Request this filter be placed as low as possible in the stack (given
    // before/after constraints).
    FilterRegistration& SinkToBottom() {
      CHECK_EQ(ordering_, Ordering::kDefault);
      ordering_ = Ordering::kBottom;
      return *this;
    }

   private:
    friend class ChannelInit;
    const UniqueTypeName name_;
    const grpc_channel_filter* const filter_;
    const FilterAdder filter_adder_;
    std::vector<UniqueTypeName> after_;
    std::vector<UniqueTypeName> before_;
    std::vector<InclusionPredicate> predicates_;
    bool terminal_ = false;
    bool before_all_ = false;
    Version version_ = Version::kAny;
    Ordering ordering_ = Ordering::kDefault;
    SourceLocation registration_source_;
  };

  class Builder {
   public:
    // Register a builder in the normal filter registration pass.
    // This occurs first during channel build time.
    // The FilterRegistration methods can be called to declaratively define
    // properties of the filter being registered.
    // TODO(ctiller): remove in favor of the version that does not mention
    // grpc_channel_filter
    FilterRegistration& RegisterFilter(grpc_channel_stack_type type,
                                       UniqueTypeName name,
                                       const grpc_channel_filter* filter,
                                       FilterAdder filter_adder = nullptr,
                                       SourceLocation registration_source = {});
    FilterRegistration& RegisterFilter(
        grpc_channel_stack_type type, const grpc_channel_filter* filter,
        SourceLocation registration_source = {}) {
      CHECK(filter != nullptr);
      return RegisterFilter(type, NameFromChannelFilter(filter), filter,
                            nullptr, registration_source);
    }
    template <typename Filter>
    FilterRegistration& RegisterFilter(
        grpc_channel_stack_type type, SourceLocation registration_source = {}) {
      return RegisterFilter(
          type, UniqueTypeNameFor<Filter>(), &Filter::kFilter,
          [](InterceptionChainBuilder& builder) { builder.Add<Filter>(); },
          registration_source);
    }

    // Filter does not participate in v3
    template <typename Filter>
    FilterRegistration& RegisterV2Filter(
        grpc_channel_stack_type type, SourceLocation registration_source = {}) {
      return RegisterFilter(type, &Filter::kFilter, registration_source)
          .SkipV3();
    }

    // Register a post processor for the builder.
    // These run after the main graph has been placed into the builder.
    // At most one filter per slot per channel stack type can be added.
    // If at all possible, prefer to use the RegisterFilter() mechanism to add
    // filters to the system - this should be a last resort escape hatch.
    void RegisterPostProcessor(grpc_channel_stack_type type,
                               PostProcessorSlot slot,
                               PostProcessor post_processor) {
      auto& slot_value = post_processors_[type][static_cast<int>(slot)];
      CHECK(slot_value == nullptr);
      slot_value = std::move(post_processor);
    }

    /// Finalize registration.
    ChannelInit Build();

   private:
    std::vector<std::unique_ptr<FilterRegistration>>
        filters_[GRPC_NUM_CHANNEL_STACK_TYPES];
    PostProcessor post_processors_[GRPC_NUM_CHANNEL_STACK_TYPES]
                                  [static_cast<int>(PostProcessorSlot::kCount)];
  };

  /// Construct a channel stack of some sort: see channel_stack.h for details
  /// \a builder is the channel stack builder to build into.
  GRPC_MUST_USE_RESULT
  bool CreateStack(ChannelStackBuilder* builder) const;

  void AddToInterceptionChainBuilder(grpc_channel_stack_type type,
                                     InterceptionChainBuilder& builder) const;

 private:
  // The type of object returned by a filter's Create method.
  template <typename T>
  using CreatedType =
      typename decltype(T::Create(ChannelArgs(), {}))::value_type;

  class DependencyTracker;

  struct Filter {
    Filter(UniqueTypeName name, const grpc_channel_filter* filter,
           FilterAdder filter_adder, std::vector<InclusionPredicate> predicates,
           Version version, Ordering ordering,
           SourceLocation registration_source)
        : name(name),
          filter(filter),
          filter_adder(filter_adder),
          predicates(std::move(predicates)),
          registration_source(registration_source),
          version(version),
          ordering(ordering) {}
    UniqueTypeName name;
    const grpc_channel_filter* filter;
    const FilterAdder filter_adder;
    std::vector<InclusionPredicate> predicates;
    SourceLocation registration_source;
    Version version;
    Ordering ordering;
    bool CheckPredicates(const ChannelArgs& args) const;
  };
  struct StackConfig {
    std::vector<Filter> filters;
    std::vector<Filter> terminators;
    std::vector<PostProcessor> post_processors;
  };

  StackConfig stack_configs_[GRPC_NUM_CHANNEL_STACK_TYPES];

  static StackConfig BuildStackConfig(
      const std::vector<std::unique_ptr<FilterRegistration>>& registrations,
      PostProcessor* post_processors, grpc_channel_stack_type type);
  static void PrintChannelStackTrace(
      grpc_channel_stack_type type,
      const std::vector<std::unique_ptr<ChannelInit::FilterRegistration>>&
          registrations,
      const DependencyTracker& dependencies, const std::vector<Filter>& filters,
      const std::vector<Filter>& terminal_filters);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SURFACE_CHANNEL_INIT_H
