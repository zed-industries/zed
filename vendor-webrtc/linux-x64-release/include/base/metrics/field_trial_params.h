// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_FIELD_TRIAL_PARAMS_H_
#define BASE_METRICS_FIELD_TRIAL_PARAMS_H_

#include <array>
#include <map>
#include <string>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/feature_list.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/no_destructor.h"
#include "base/notreached.h"
#include "base/time/time.h"

namespace base {

namespace internal {

BASE_EXPORT bool IsFeatureParamWithCacheEnabled();

// A traits struct to manage the type for the default value in the following
// FeatureParam<> template. `std::string` needs to use a string literal instead
// of `std::string` to realize compile time construction.
template <typename T>
struct FeatureParamTraits {
  using DefaultValueType = T;
  using CacheStorageType = T;
  static CacheStorageType ToCacheStorageType(const T& value) { return value; }
  static constexpr T FromCacheStorageType(const CacheStorageType& storage) {
    return storage;
  }
};

template <>
struct FeatureParamTraits<std::string> {
  using DefaultValueType = const char*;
  using CacheStorageType = NoDestructor<std::string>;
  static CacheStorageType ToCacheStorageType(const std::string& value) {
    return CacheStorageType(value);
  }
  static constexpr std::string FromCacheStorageType(
      const CacheStorageType& storage) {
    return *storage;
  }
};

}  // namespace internal

// Key-value mapping type for field trial parameters.
typedef std::map<std::string, std::string> FieldTrialParams;

// Param string decoding function for AssociateFieldTrialParamsFromString().
typedef std::string (*FieldTrialParamsDecodeStringFunc)(const std::string& str);

// Shared declaration for various FeatureParam<T> types.
//
// This template is defined for the following types T:
//   bool
//   int
//   size_t
//   double
//   std::string
//   enum types
//   base::TimeDelta
//
// Attempting to use it with any other type is a compile error.
//
// Getting a param value from a FeatureParam<T> will have the same semantics as
// GetFieldTrialParamValueByFeature(), see that function's comments for details.
// `cache_getter` is used to provide a dedicated getter that is used to give a
// local cache to the FeatureParam. Usually, this is automatically generated and
// provided via BASE_FEATURE_PARAM() or BASE_FEATURE_ENUM_PARAM() macro.
//
// Example to declares a double-valued parameter.
//
//     constexpr FeatureParam<double> kAssistantTriggerThreshold = {
//         &kAssistantFeature, "trigger_threshold", 0.10};
//
// Equivalent using the caching macro (see base/feature_list.h):
//
//     BASE_FEATURE_PARAM(double, kAssistantTriggerThreshold,
//                        &kAssistantFeature, "trigger_threshold", 0.10);
//
// If the feature is not enabled, the parameter is not set, or set to an invalid
// value, then Get() will return the default value.
template <typename T, bool IsEnum = std::is_enum_v<T>>
struct FeatureParam {
  using DefaultValueType =
      typename internal::FeatureParamTraits<T>::DefaultValueType;

  // Prevent use of FeatureParam<> with unsupported types (e.g. void*). Uses T
  // in its definition so that evaluation is deferred until the template is
  // instantiated.
  static_assert(std::is_same_v<bool, T> || std::is_same_v<int, T> ||
                    std::is_same_v<size_t, T> || std::is_same_v<double, T> ||
                    std::is_same_v<std::string, T> ||
                    std::is_same_v<base::TimeDelta, T>,
                "Unsupported FeatureParam<> type");

  constexpr FeatureParam(const Feature* feature,
                         const char* name,
                         DefaultValueType default_value,
                         T (*cache_getter)(const FeatureParam<T>*) = nullptr)
      : feature(feature),
        name(name),
        default_value(default_value),
        cache_getter(cache_getter) {}

  // Calling Get() or GetWithoutCache() will activate the field trial associated
  // with |feature|. See GetFieldTrialParamValueByFeature() for more details.
  BASE_EXPORT T Get() const {
    if (internal::IsFeatureParamWithCacheEnabled() && cache_getter) {
      return cache_getter(this);
    }
    return GetWithoutCache();
  }
  BASE_EXPORT T GetWithoutCache() const;

  // RAW_PTR_EXCLUSION: Using raw_ptr here would make FeatureParam lose its
  // trivial destructor, causing it to be classified as a non-trivial class
  // member in contexts where it's included. This can complicate code and
  // impact performance. base::Features are expected to be globals with
  // static lifetime, so this is pretty much always safe.
  RAW_PTR_EXCLUSION const Feature* const feature;
  const char* const name;
  const DefaultValueType default_value;
  T (*const cache_getter)(const FeatureParam<T>*);
};

// Declarations for GetWithoutCache() specializations and explicit
// instantiations for the FeatureParam<> to ensure instantiating them
// in the base components to export them.
template <>
bool FeatureParam<bool>::GetWithoutCache() const;
template struct FeatureParam<bool>;
template struct internal::FeatureParamTraits<bool>;

template <>
int FeatureParam<int>::GetWithoutCache() const;
template struct FeatureParam<int>;
template struct internal::FeatureParamTraits<int>;

template <>
size_t FeatureParam<size_t>::GetWithoutCache() const;
template struct FeatureParam<size_t>;
template struct internal::FeatureParamTraits<size_t>;

template <>
double FeatureParam<double>::GetWithoutCache() const;
template struct FeatureParam<double>;
template struct internal::FeatureParamTraits<double>;

template <>
std::string FeatureParam<std::string>::GetWithoutCache() const;
template struct FeatureParam<std::string>;

template <>
TimeDelta FeatureParam<TimeDelta>::GetWithoutCache() const;
template struct FeatureParam<TimeDelta>;
template struct internal::FeatureParamTraits<TimeDelta>;

// Feature param declaration for an enum, with associated options. Example:
//
//     constexpr FeatureParam<ShapeEnum>::Option kShapeParamOptions[] = {
//         {SHAPE_CIRCLE, "circle"},
//         {SHAPE_CYLINDER, "cylinder"},
//         {SHAPE_PAPERCLIP, "paperclip"}};
//     constexpr FeatureParam<ShapeEnum> kAssistantShapeParam = {
//         &kAssistantFeature, "shape", SHAPE_CIRCLE, &kShapeParamOptions};
//
// With this declaration, the parameter may be set to "circle", "cylinder", or
// "paperclip", and that will be translated to one of the three enum values. By
// default, or if the param is set to an unknown value, the parameter will be
// assumed to be SHAPE_CIRCLE.
template <typename Enum>
struct FeatureParam<Enum, true> {
  struct Option {
    constexpr Option(Enum value, const char* name) : value(value), name(name) {}

    const Enum value;
    const char* const name;
  };

  template <size_t option_count>
  constexpr FeatureParam(
      const Feature* feature,
      const char* name,
      const Enum default_value,
      const std::array<Option, option_count>& options,
      Enum (*cache_getter)(const FeatureParam<Enum>*) = nullptr)
      : feature(feature),
        name(name),
        default_value(default_value),
        options(options.data()),
        option_count(option_count),
        cache_getter(cache_getter) {
    static_assert(option_count >= 1, "FeatureParam<enum> has no options");
  }

  template <size_t option_count>
  constexpr FeatureParam(
      const Feature* feature,
      const char* name,
      const Enum default_value,
      const Option (*options)[option_count],
      Enum (*cache_getter)(const FeatureParam<Enum>*) = nullptr)
      : feature(feature),
        name(name),
        default_value(default_value),
        options(*options),
        option_count(option_count),
        cache_getter(cache_getter) {
    static_assert(option_count >= 1, "FeatureParam<enum> has no options");
  }

  // Calling Get() or GetWithoutCache() will activate the field trial associated
  // with |feature|. See GetFieldTrialParamValueByFeature() for more details.
  Enum Get() const {
    if (internal::IsFeatureParamWithCacheEnabled() && cache_getter) {
      return cache_getter(this);
    }
    return GetWithoutCache();
  }
  Enum GetWithoutCache() const {
    return GetFieldTrialParamByFeatureAsEnum(
        *feature, name, default_value,
        UNSAFE_TODO(base::span(&*options, option_count)));
  }

  // Returns the param-string for the given enum value.
  std::string GetName(Enum value) const {
    for (size_t i = 0; i < option_count; ++i) {
      if (value == options[i].value) {
        return options[i].name;
      }
    }
    NOTREACHED();
  }

  const raw_ptr<const base::Feature> feature;
  const char* const name;
  const Enum default_value;
  // TODO(crbug.com/40284755): Remove AllowPtrArithmetic if possible after
  // unsafe buffers have been evaluated.
  const raw_ptr<const Option, AllowPtrArithmetic> options;
  const size_t option_count;
  Enum (*const cache_getter)(const FeatureParam<Enum>*);
};

// Unescapes special characters from the given string. Used in
// AssociateFieldTrialParamsFromString() as one of the feature params decoding
// functions.
BASE_EXPORT std::string UnescapeValue(const std::string& value);

// Logs a warning that a feature param expected to map to an enum was an unknown
// non-empty value.
BASE_EXPORT void LogInvalidEnumValue(const Feature& feature,
                                     const std::string& param_name,
                                     const std::string& value_as_string,
                                     int default_value_as_int);

// Associates the specified set of key-value |params| with the field trial
// specified by |trial_name| and |group_name|. Fails and returns false if the
// specified field trial already has params associated with it or the trial
// is already active (group() has been called on it). Thread safe.
BASE_EXPORT bool AssociateFieldTrialParams(const std::string& trial_name,
                                           const std::string& group_name,
                                           const FieldTrialParams& params);

// Provides a mechanism to associate multiple set of params to multiple groups
// with a formatted string as returned by FieldTrialList::AllParamsToString().
// |decode_data_func| allows specifying a custom decoding function.
BASE_EXPORT bool AssociateFieldTrialParamsFromString(
    const std::string& params_string,
    FieldTrialParamsDecodeStringFunc decode_data_func);

// Retrieves the set of key-value |params| for the specified field trial, based
// on its selected group. If the field trial does not exist or its selected
// group does not have any parameters associated with it, returns false and
// does not modify |params|. Calling this function will result in the field
// trial being marked as active if found (i.e. group() will be called on it),
// if it wasn't already. Thread safe.
BASE_EXPORT bool GetFieldTrialParams(const std::string& trial_name,
                                     FieldTrialParams* params);

// Retrieves the set of key-value |params| for the field trial associated with
// the specified |feature|. A feature is associated with at most one field
// trial and selected group. See  base/feature_list.h for more information on
// features. If the feature is not enabled, or if there's no associated params,
// returns false and does not modify |params|. Calling this function will
// result in the associated field trial being marked as active if found (i.e.
// group() will be called on it), if it wasn't already. Thread safe.
BASE_EXPORT bool GetFieldTrialParamsByFeature(const base::Feature& feature,
                                              FieldTrialParams* params);

// Retrieves a specific parameter value corresponding to |param_name| for the
// specified field trial, based on its selected group. If the field trial does
// not exist or the specified parameter does not exist, returns an empty
// string. Calling this function will result in the field trial being marked as
// active if found (i.e. group() will be called on it), if it wasn't already.
// Thread safe.
BASE_EXPORT std::string GetFieldTrialParamValue(const std::string& trial_name,
                                                const std::string& param_name);

// Retrieves a specific parameter value corresponding to |param_name| for the
// field trial associated with the specified |feature|. A feature is associated
// with at most one field trial and selected group. See base/feature_list.h for
// more information on features. If the feature is not enabled, or the
// specified parameter does not exist, returns an empty string. Calling this
// function will result in the associated field trial being marked as active if
// found (i.e. group() will be called on it), if it wasn't already. Thread safe.
BASE_EXPORT std::string GetFieldTrialParamValueByFeature(
    const base::Feature& feature,
    const std::string& param_name);

// Same as GetFieldTrialParamValueByFeature(). But internally relies on
// GetFieldTrialParamsByFeature to handle empty values in the map, and returns
// |default_value| only if |param_name| is not found in the map.
BASE_EXPORT std::string GetFieldTrialParamByFeatureAsString(
    const base::Feature& feature,
    const std::string& param_name,
    const std::string& default_value);

// Same as GetFieldTrialParamValueByFeature(). On top of that, it converts the
// string value into an int using base::StringToInt() and returns it, if
// successful. Otherwise, it returns |default_value|. If the string value is not
// empty and the conversion does not succeed, it produces a warning to LOG.
BASE_EXPORT int GetFieldTrialParamByFeatureAsInt(const base::Feature& feature,
                                                 const std::string& param_name,
                                                 int default_value);

// Same as GetFieldTrialParamValueByFeature(). On top of that, it converts the
// string value into a double using base::StringToDouble() and returns it, if
// successful. Otherwise, it returns |default_value|. If the string value is not
// empty and the conversion does not succeed, it produces a warning to LOG.
BASE_EXPORT double GetFieldTrialParamByFeatureAsDouble(
    const base::Feature& feature,
    const std::string& param_name,
    double default_value);

// Same as GetFieldTrialParamValueByFeature(). On top of that, it converts the
// string value into a boolean and returns it, if successful. Otherwise, it
// returns |default_value|. The only string representations accepted here are
// "true" and "false". If the string value is not empty and the conversion does
// not succeed, it produces a warning to LOG.
BASE_EXPORT bool GetFieldTrialParamByFeatureAsBool(
    const base::Feature& feature,
    const std::string& param_name,
    bool default_value);

// Same as GetFieldTrialParamValueByFeature(). On top of that, it converts the
// string value into a base::TimeDelta and returns it, if successful. Otherwise,
// it returns `default_value`. If the string value is not empty and the
// conversion does not succeed, it produces a warning to LOG.
BASE_EXPORT base::TimeDelta GetFieldTrialParamByFeatureAsTimeDelta(
    const Feature& feature,
    const std::string& param_name,
    base::TimeDelta default_value);

// Same as GetFieldTrialParamValueByFeature(). On top of that, it converts the
// string value into an Enum and returns it, if successful. Otherwise, it
// returns `default_value`. If the string value is not empty and the conversion
// does not succeed, it produces a warning to LOG.
template <typename Enum>
Enum GetFieldTrialParamByFeatureAsEnum(
    const Feature& feature,
    const std::string& param_name,
    Enum default_value,
    base::span<const typename FeatureParam<Enum, true>::Option> options) {
  std::string value = GetFieldTrialParamValueByFeature(feature, param_name);
  if (value.empty()) {
    return default_value;
  }
  for (const auto& option : options) {
    if (value == option.name) {
      return option.value;
    }
  }
  LogInvalidEnumValue(feature, param_name, value,
                      static_cast<int>(default_value));
  return default_value;
}

}  // namespace base

#endif  // BASE_METRICS_FIELD_TRIAL_PARAMS_H_
