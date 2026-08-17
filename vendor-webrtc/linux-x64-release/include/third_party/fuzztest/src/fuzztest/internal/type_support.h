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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_TYPE_SUPPORT_H_
#define FUZZTEST_FUZZTEST_INTERNAL_TYPE_SUPPORT_H_

#include <array>
#include <cctype>
#include <cstddef>
#include <cstdint>
#include <limits>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <vector>

#include "absl/debugging/symbolize.h"
#include "absl/functional/function_ref.h"
#include "absl/numeric/int128.h"
#include "absl/strings/escaping.h"
#include "absl/strings/has_absl_stringify.h"
#include "absl/strings/match.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "absl/strings/strip.h"
#include "absl/time/time.h"
#include "./fuzztest/internal/domains/absl_helpers.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/printer.h"

namespace google::protobuf {
class EnumDescriptor;

template <typename E>
const EnumDescriptor* GetEnumDescriptor();
}  // namespace google::protobuf

namespace fuzztest::internal {

// Return a best effort printer for type `T`.
// This is useful for cases where the domain can't figure out how to print the
// value.
// It implements a good printer for common known types and fallbacks to an
// "unknown" printer to prevent compile time errors.
template <typename T>
decltype(auto) AutodetectTypePrinter();

// Returns true iff type `T` has a known printer that isn't UnknownPrinter.
template <typename T>
constexpr bool HasKnownPrinter();

// If `prefix` is present in `name`, consume everything until the rightmost
// occurrence of `prefix` and return true. Otherwise, return false.
constexpr bool ConsumePrefixUntil(absl::string_view& name,
                                  absl::string_view prefix) {
  size_t pos = name.rfind(prefix);
  if (name.npos == pos) return false;
  name.remove_prefix(pos + prefix.size());
  return true;
}

constexpr bool ConsumeUnnecessaryNamespacePrefix(absl::string_view& name) {
  constexpr std::array prefixes = {
      // GCC adds the function name in which the type was defined e.g.,
      // {anonymous}::MyFunction()::MyStruct{}.
      "()::",  // This needs to be first in the list, otherwise we'd remove
               // {anonymous}:: and stop.
      // Various anonymous namespace prefixes different compilers use:
      "{anonymous}::",
      "(anonymous namespace)::",
      "<unnamed>::",
  };
  for (absl::string_view p : prefixes) {
    if (ConsumePrefixUntil(name, p)) return true;
  }
  return false;
}

template <typename T>
constexpr auto GetTypeName() {
  absl::string_view name, prefix, suffix;
  name = __PRETTY_FUNCTION__;
#if defined(__clang__)
  prefix = "GetTypeName() [T = ";
  suffix = "]";
#elif defined(__GNUC__)
  prefix = "GetTypeName() [with T = ";
  suffix = "]";
#else
  return "<TYPE>"
#endif
  // First we remove the prefix and suffix to get a fully qualified type name.
  ConsumePrefixUntil(name, prefix);
  absl::ConsumeSuffix(&name, suffix);
  // Then we remove any unnecessary namespaces from the type name.
  ConsumeUnnecessaryNamespacePrefix(name);
  return name;
}

template <typename T>
absl::string_view GetTypeNameIfUserDefined() {
  using CleanT = std::remove_cv_t<std::remove_reference_t<T>>;
  absl::string_view type_name = GetTypeName<CleanT>();
  // Exclude aggregate types like `std::pair`, `std::tuple`, and `std::array`,
  // for which we don't want to print a long and unwieldy type name.
  if (type_name == "<TYPE>" || absl::StartsWith(type_name, "std::")) {
    return "";
  }
  return type_name;
}

template <typename T>
inline constexpr bool has_absl_stringify_v = absl::HasAbslStringify<T>::value;

struct IntegralPrinter {
  template <typename T>
  void PrintUserValue(T v, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) const {
    if constexpr (std::is_enum_v<T>) {
      // TODO(sbenzaquen): Try to use enum labels where possible.
      // Use static_cast<> when printing source code to avoid init conversion.
      switch (mode) {
        case domain_implementor::PrintMode::kHumanReadable:
          absl::Format(out, "%s{", GetTypeName<T>());
          break;
        case domain_implementor::PrintMode::kSourceCode:
          absl::Format(out, "static_cast<%s>(", GetTypeName<T>());
          break;
      }
      PrintUserValue(static_cast<std::underlying_type_t<T>>(v), out, mode);
      switch (mode) {
        case domain_implementor::PrintMode::kHumanReadable:
          absl::Format(out, "}");
          break;
        case domain_implementor::PrintMode::kSourceCode:
          absl::Format(out, ")");
          break;
      }
    } else if constexpr (std::is_signed_v<T>) {
      // Cast to [u]int128 to cover all integral types.
      PrintUserValue(static_cast<absl::int128>(v), out, mode);
    } else {
      PrintUserValue(static_cast<absl::uint128>(v), out, mode);
    }
  }

  void PrintUserValue(bool v, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) const;
  void PrintUserValue(char v, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) const;
  void PrintUserValue(absl::uint128 v, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) const;
  void PrintUserValue(absl::int128 v, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) const;
};

struct FloatingPrinter {
  void PrintUserValue(float v, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) const;
  void PrintUserValue(double v, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) const;
  void PrintUserValue(long double v, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) const;
};

struct StringPrinter {
  template <typename T>
  void PrintUserValue(const T& v, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) const {
    switch (mode) {
      case domain_implementor::PrintMode::kHumanReadable:
        absl::Format(out, "\"");
        for (char c : v) {
          if (std::isprint(c)) {
            absl::Format(out, "%c", c);
          } else {
            absl::Format(out, "\\%03o", c);
          }
        }
        absl::Format(out, "\"");
        break;
      case domain_implementor::PrintMode::kSourceCode: {
        // Make sure to properly C-escape strings when printing source code, and
        // explicitly construct a std::string of the right length if there is an
        // embedded NULL character.
        const std::string input(v.data(), v.data() + v.size());
        const std::string escaped = absl::CEscape(input);
        if constexpr (std::is_same_v<T, std::vector<uint8_t>>) {
          absl::Format(out, "fuzztest::ToByteArray(std::string(\"%s\", %d))",
                       escaped, v.size());
        } else if (absl::StrContains(input, '\0')) {
          absl::Format(out, "std::string(\"%s\", %d)", escaped, v.size());
        } else {
          absl::Format(out, "\"%s\"", escaped);
        }
        break;
      }
    }
  }
};

template <typename DomainT, typename... Inner>
struct AggregatePrinter {
  const DomainT& domain;
  const std::tuple<Inner...>& inner;
  absl::string_view type_name;

  void PrintCorpusValue(const corpus_type_t<DomainT>& v,
                        domain_implementor::RawSink out,
                        domain_implementor::PrintMode mode) const {
    if (mode == domain_implementor::PrintMode::kHumanReadable) {
      // In human-readable mode, prefer formatting with Abseil if possible.
      if constexpr (has_absl_stringify_v<value_type_t<DomainT>>) {
        absl::Format(out, "%v", domain.GetValue(v));
        return;
      }
    }

    absl::Format(out, "%s", type_name);
    PrintFormattedAggregateValue(
        v, out, mode, "{", "}",
        [](absl::FormatRawSink out, size_t idx, absl::string_view element) {
          if (idx > 0) absl::Format(out, ", ");
          absl::Format(out, "%s", element);
        });
  }

  void PrintFormattedAggregateValue(
      const corpus_type_t<DomainT>& v, domain_implementor::RawSink out,
      domain_implementor::PrintMode mode, absl::string_view prefix,
      absl::string_view suffix,
      absl::FunctionRef<void(absl::FormatRawSink, size_t, absl::string_view)>
          element_formatter) const {
    auto bound = internal::BindAggregate(
        v, std::integral_constant<int, sizeof...(Inner)>{});

    const auto print_one = [&](auto I) {
      std::string str_out;
      domain_implementor::PrintValue(std::get<I>(inner), std::get<I>(bound),
                                     &str_out, mode);
      element_formatter(out, I, str_out);
    };

    absl::Format(out, "%s", prefix);
    ApplyIndex<sizeof...(Inner)>([&](auto... Is) { (print_one(Is), ...); });
    absl::Format(out, "%s", suffix);
  }
};

template <typename... Inner>
struct VariantPrinter {
  const std::tuple<Inner...>& inner;

  template <typename T>
  void PrintCorpusValue(const T& v, domain_implementor::RawSink out,
                        domain_implementor::PrintMode mode) const {
    // The source code version will work as long as the types are unambiguous.
    // Printing the whole variant type to call the explicit constructor might be
    // an issue.
    Switch<sizeof...(Inner)>(v.index(), [&](auto I) {
      domain_implementor::PrintValue(std::get<I>(inner), std::get<I>(v), out,
                                     mode);
    });
  }
};

struct ProtobufPrinter {
  template <typename T>
  void PrintUserValue(const T& val, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) const {
    if constexpr (Requires<T>([](auto v) -> decltype(*v) {})) {
      // Deref if necessary.
      return PrintUserValue(*val, out, mode);
    } else {
      static constexpr absl::string_view kProtoParser = "ParseTestProto";

      std::string textproto = absl::StrCat(val);
      switch (mode) {
        case domain_implementor::PrintMode::kHumanReadable:
          absl::Format(out, "(%s)", textproto);
          break;
        case domain_implementor::PrintMode::kSourceCode:
          absl::Format(out, "%s(R\"pb(%s)pb\")", kProtoParser, textproto);
          break;
      }
    }
  }
};

template <typename D>
struct ProtobufEnumPrinter {
  D descriptor;

  template <typename T>
  void PrintUserValue(const T& v, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) const {
    if (auto vd = descriptor->FindValueByNumber(v); vd != nullptr) {
      // For enums nested inside a message, protoc generates an enum named
      // `<MessageName>_<EnumName>` and aliases for each label of the form
      // `<MessageName>::<LabelN>`, so we can strip the trailing `_<EnumName>`
      // and append `::<Label>`.
      //
      // For top-level enums in C++11, the enumerators are local to the enum,
      // so leave the name untouched to print `<Enum>::<Label>`.
      absl::string_view type_name = GetTypeName<T>();
      absl::string_view enum_name = descriptor->name();
      if (absl::EndsWith(type_name, absl::StrCat("_", enum_name))) {
        type_name.remove_suffix(enum_name.size() + 1);
      }
      absl::Format(out, "%s::%s", type_name, vd->name());
      if (mode == domain_implementor::PrintMode::kHumanReadable) {
        absl::Format(out, " (");
        IntegralPrinter{}.PrintUserValue(static_cast<int64_t>(v), out, mode);
        absl::Format(out, ")");
      }
      return;
    }
    // Fall back on regular enum printer.
    IntegralPrinter{}.PrintUserValue(v, out, mode);
  }
};

struct MonostatePrinter {
  template <typename T>
  void PrintUserValue(const T&, domain_implementor::RawSink out,
                      domain_implementor::PrintMode) const {
    absl::Format(out, "%s{}", GetTypeNameIfUserDefined<T>());
  }
};

template <typename Domain, typename Inner>
struct ContainerPrinter {
  const Inner& inner_domain;

  void PrintCorpusValue(const corpus_type_t<Domain>& val,
                        domain_implementor::RawSink out,
                        domain_implementor::PrintMode mode) const {
    absl::Format(out, "{");
    bool first = true;
    for (const auto& v : val) {
      if (!first) absl::Format(out, ", ");
      first = false;
      domain_implementor::PrintValue(inner_domain, v, out, mode);
    }
    absl::Format(out, "}");
  }
};

template <typename F>
constexpr bool HasFunctionName() {
  return std::is_function_v<std::remove_pointer_t<F>>;
}

inline void ConsumeFileAndLineNumber(absl::string_view& v) {
  // We're essentially matching the regexp "[^:]\:\d+ ", but manually since we
  // don't want to introduce a dependency on RE2.
  absl::string_view::size_type pos = 0;
  while (pos < v.size()) {
    pos = v.find(':', pos);
    if (pos == v.npos) return;
    // Skip the colon.
    ++pos;
    if (pos >= v.size() || !std::isdigit(v[pos])) {
      // Colon not followed by a digit. Skip any subsequent colons and continue.
      while (pos < v.size() && v[pos] == ':') ++pos;
      continue;
    }
    // Skip the digits.
    ++pos;
    while (pos < v.size() && std::isdigit(v[pos])) ++pos;
    if (pos >= v.size() || v[pos] != ' ') continue;
    // Skip the space.
    ++pos;
    v.remove_prefix(pos);
    return;
  }
}

template <typename F>
std::string GetFunctionName(const F& f, absl::string_view default_name) {
  if constexpr (HasFunctionName<F>()) {
    char buffer[1024];
    if (absl::Symbolize(reinterpret_cast<const void*>(f), buffer,
                        sizeof(buffer))) {
      absl::string_view v = buffer;
      absl::ConsumeSuffix(&v, "()");
      ConsumeFileAndLineNumber(v);
      ConsumeUnnecessaryNamespacePrefix(v);
      return std::string(v);
    }
  }
  return std::string(default_name);
}

template <typename Mapper, typename... Inner>
struct MappedPrinter {
  const Mapper& mapper;
  const std::tuple<Inner...>& inner;
  absl::string_view map_fn_name;

  template <typename CorpusT>
  void PrintCorpusValue(const CorpusT& corpus_value,
                        domain_implementor::RawSink out,
                        domain_implementor::PrintMode mode) const {
    auto value = ApplyIndex<sizeof...(Inner)>([&](auto... I) {
      return mapper(std::get<I>(inner).GetValue(std::get<I>(corpus_value))...);
    });

    switch (mode) {
      case domain_implementor::PrintMode::kHumanReadable: {
        // In human readable mode we try and print the user value.
        AutodetectTypePrinter<decltype(value)>().PrintUserValue(value, out,
                                                                mode);
        break;
      }
      case domain_implementor::PrintMode::kSourceCode:
        if constexpr (!HasFunctionName<Mapper>() &&
                      HasKnownPrinter<decltype(value)>()) {
          if (map_fn_name.empty()) {
            // Fall back on printing the user value if the mapping function is
            // unknown (e.g. a lambda) and the value has a useful printer.
            AutodetectTypePrinter<decltype(value)>().PrintUserValue(value, out,
                                                                    mode);
            break;
          }
        }

        // In source code mode we print the mapping expression.
        // This should give a better chance of valid code, given that the result
        // of the mapping function can easily be a user defined type we can't
        // generate otherwise.
        absl::string_view default_name =
            map_fn_name.empty() ? "<MAPPING_FUNCTION>" : map_fn_name;
        absl::Format(out, "%s(", GetFunctionName(mapper, default_name));
        const auto print_one = [&](auto I) {
          if (I != 0) absl::Format(out, ", ");
          domain_implementor::PrintValue(std::get<I>(inner),
                                         std::get<I>(corpus_value), out, mode);
        };
        ApplyIndex<sizeof...(Inner)>([&](auto... Is) { (print_one(Is), ...); });
        absl::Format(out, ")");
    }
  }
};

template <typename FlatMapper, typename... Inner>
struct FlatMappedPrinter {
  const FlatMapper& mapper;
  const std::tuple<Inner...>& inner;

  template <typename CorpusT>
  void PrintCorpusValue(const CorpusT& corpus_value,
                        domain_implementor::RawSink out,
                        domain_implementor::PrintMode mode) const {
    auto output_domain = ApplyIndex<sizeof...(Inner)>([&](auto... I) {
      return mapper(
          // the first field of `corpus_value` is the output value, so skip it
          std::get<I>(inner).GetValue(std::get<I + 1>(corpus_value))...);
    });

    // Delegate to the output domain's printer.
    domain_implementor::PrintValue(output_domain, std::get<0>(corpus_value),
                                   out, mode);
  }
};

struct AutodetectAggregatePrinter {
  template <typename T>
  void PrintUserValue(const T& v, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) {
    if (mode == domain_implementor::PrintMode::kHumanReadable) {
      // In human-readable mode, prefer formatting with Abseil if possible.
      if constexpr (has_absl_stringify_v<T>) {
        absl::Format(out, "%v", v);
        return;
      }
    }
    std::tuple bound = DetectBindAggregate(v);
    const auto print_one = [&](auto I) {
      if (I > 0) absl::Format(out, ", ");
      AutodetectTypePrinter<
          std::remove_reference_t<std::tuple_element_t<I, decltype(bound)>>>()
          .PrintUserValue(std::get<I>(bound), out, mode);
    };
    absl::Format(out, "%s{", GetTypeNameIfUserDefined<T>());
    ApplyIndex<std::tuple_size_v<decltype(bound)>>(
        [&](auto... Is) { (print_one(Is), ...); });
    absl::Format(out, "}");
  }
};

struct DurationPrinter {
  void PrintUserValue(const absl::Duration duration,
                      domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) {
    switch (mode) {
      case domain_implementor::PrintMode::kHumanReadable:
        absl::Format(out, "%s", absl::FormatDuration(duration));
        break;
      case domain_implementor::PrintMode::kSourceCode:
        if (duration == absl::InfiniteDuration()) {
          absl::Format(out, "absl::InfiniteDuration()");
        } else if (duration == -absl::InfiniteDuration()) {
          absl::Format(out, "-absl::InfiniteDuration()");
        } else if (duration == absl::ZeroDuration()) {
          absl::Format(out, "absl::ZeroDuration()");
        } else {
          uint32_t ticks = GetTicks(duration);
          int64_t secs = GetSeconds(duration);
          if (ticks == 0) {
            absl::Format(out, "absl::Seconds(%d)", secs);
          } else if (ticks % 4 == 0) {
            absl::Format(out, "absl::Seconds(%d) + absl::Nanoseconds(%u)", secs,
                         ticks / 4);
          } else {
            absl::Format(out,
                         "absl::Seconds(%d) + (absl::Nanoseconds(1) / 4) * %u",
                         secs, ticks);
          }
        }
        break;
    }
  }
};

struct TimePrinter {
  void PrintUserValue(const absl::Time time, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) {
    switch (mode) {
      case domain_implementor::PrintMode::kHumanReadable:
        absl::Format(out, "%s", absl::FormatTime(time, absl::UTCTimeZone()));
        break;
      case domain_implementor::PrintMode::kSourceCode:
        if (time == absl::InfinitePast()) {
          absl::Format(out, "absl::InfinitePast()");
        } else if (time == absl::InfiniteFuture()) {
          absl::Format(out, "absl::InfiniteFuture()");
        } else if (time == absl::UnixEpoch()) {
          absl::Format(out, "absl::UnixEpoch()");
        } else {
          absl::Format(out, "absl::UnixEpoch() + ");
          DurationPrinter{}.PrintUserValue(time - absl::UnixEpoch(), out, mode);
        }
        break;
    }
  }
};

struct UnknownPrinter {
  template <typename T>
  void PrintUserValue(const T& v, domain_implementor::RawSink out,
                      domain_implementor::PrintMode mode) {
    if (mode == domain_implementor::PrintMode::kHumanReadable) {
      // Try formatting with Abseil. We can't guarantee a good source code
      // result, but it should be ok for human readable.
      if constexpr (has_absl_stringify_v<T>) {
        absl::Format(out, "%v", v);
        return;
      }
      // Some standard types have operator<<.
      if constexpr (std::is_scalar_v<T> || is_std_complex_v<T>) {
        absl::Format(out, "%s", absl::FormatStreamed(v));
        return;
      }
    }
    absl::Format(out, "<unprintable value>");
  }
};

template <typename T>
decltype(auto) AutodetectTypePrinter() {
  if constexpr (is_protocol_buffer_enum_v<T>) {
    return ProtobufEnumPrinter<const google::protobuf::EnumDescriptor*>{
        google::protobuf::GetEnumDescriptor<T>()};
  } else if constexpr (std::numeric_limits<T>::is_integer ||
                       std::is_enum_v<T>) {
    return IntegralPrinter{};
  } else if constexpr (std::is_floating_point_v<T>) {
    return FloatingPrinter{};
  } else if constexpr (std::is_convertible_v<T, absl::string_view> ||
                       std::is_convertible_v<T, std::string_view> ||
                       std::is_same_v<T, std::vector<uint8_t>>) {
    return StringPrinter{};
  } else if constexpr (is_monostate_v<T>) {
    return MonostatePrinter{};
  } else if constexpr (is_protocol_buffer_v<T>) {
    return ProtobufPrinter{};
  } else if constexpr (is_bindable_aggregate_v<T>) {
    return AutodetectAggregatePrinter{};
  } else if constexpr (std::is_same_v<T, absl::Duration>) {
    return DurationPrinter{};
  } else if constexpr (std::is_same_v<T, absl::Time>) {
    return TimePrinter{};
  } else {
    return UnknownPrinter{};
  }
}

template <typename T>
constexpr bool HasKnownPrinter() {
  return !std::is_convertible_v<decltype(AutodetectTypePrinter<T>()),
                                UnknownPrinter>;
}

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_TYPE_SUPPORT_H_
