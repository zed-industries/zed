// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FEATURE_LIST_H_
#define BASE_FEATURE_LIST_H_

#include <atomic>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/flat_map.h"
#include "base/containers/flat_set.h"
#include "base/dcheck_is_on.h"
#include "base/feature_list_buildflags.h"
#include "base/gtest_prod_util.h"
#include "base/logging.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock.h"
#include "build/build_config.h"

namespace base {

class FieldTrial;
class FieldTrialList;
class PersistentMemoryAllocator;
class FeatureVisitor;

// Specifies whether a given feature is enabled or disabled by default.
// NOTE: The actual runtime state may be different, due to a field trial or a
// command line switch.
enum FeatureState {
  FEATURE_DISABLED_BY_DEFAULT,
  FEATURE_ENABLED_BY_DEFAULT,
};

// Recommended macros for declaring and defining features and parameters:
//
// - `kFeature` is the C++ identifier that will be used for the `base::Feature`.
// - `name` is the feature name, which must be globally unique. This name is
//   used to enable/disable features via experiments and command-line flags.
//   Names should use CamelCase-style naming, e.g. "MyGreatFeature".
// - `default_state` is the default state to use for the feature, i.e.
//   `base::FEATURE_DISABLED_BY_DEFAULT` or `base::FEATURE_ENABLED_BY_DEFAULT`.
//   As noted above, the actual runtime state may differ from the default state,
//   due to field trials or command-line switches.

// Provides a forward declaration for `kFeature` in a header file, e.g.
//
//   BASE_DECLARE_FEATURE(kMyFeature);
//
// If the feature needs to be marked as exported, i.e. it is referenced by
// multiple components, then write:
//
//   COMPONENT_EXPORT(MY_COMPONENT) BASE_DECLARE_FEATURE(kMyFeature);
#define BASE_DECLARE_FEATURE(kFeature) \
  extern constinit const base::Feature kFeature

// Provides a definition for `kFeature` with `name` and `default_state`, e.g.
//
//   BASE_FEATURE(kMyFeature, "MyFeature", base::FEATURE_DISABLED_BY_DEFAULT);
//
// Features should *not* be defined in header files; do not use this macro in
// header files.
#define BASE_FEATURE(feature, name, default_state) \
  constinit const base::Feature feature(           \
      name, default_state, base::internal::FeatureMacroHandshake::kSecret)

// Provides a forward declaration for `feature_object_name` in a header file,
// e.g.
//
//   BASE_DECLARE_FEATURE_PARAM(int, kMyFeatureParam);
//
// If the feature needs to be marked as exported, i.e. it is referenced by
// multiple components, then write:
//
//   COMPONENT_EXPORT(MY_COMPONENT)
//   BASE_DECLARE_FEATURE_PARAM(int, kMyFeatureParam);
//
// This macro enables optimizations to make the second and later calls faster,
// but requires additional memory uses. If you obtain the parameter only once,
// you can instantiate base::FeatureParam directly, or can call
// base::GetFieldTrialParamByFeatureAsInt or equivalent functions for other
// types directly.
#define BASE_DECLARE_FEATURE_PARAM(T, feature_object_name) \
  extern constinit const base::FeatureParam<T> feature_object_name

// Provides a definition for `feature_object_name` with `T`, `feature`, `name`
// and `default_value`, with an internal parsed value cache, e.g.
//
//   BASE_FEATURE_PARAM(int, kMyFeatureParam, &kMyFeature, "my_feature_param",
//                      0);
//
// `T` is a parameter type, one of bool, int, size_t, double, std::string, and
// base::TimeDelta. Enum types are not supported for now.
//
// It should *not* be defined in header files; do not use this macro in header
// files.
//
// WARNING: If the feature is not enabled, the parameter is not set, or set to
// an invalid value (per the param type), then Get() will return the default
// value passed to this C++ macro. In particular this will typically return the
// default value regardless of the server-side config in control groups.
#define BASE_FEATURE_PARAM(T, feature_object_name, feature, name,       \
                           default_value)                               \
  namespace field_trial_params_internal {                               \
  T GetFeatureParamWithCacheFor##feature_object_name(                   \
      const base::FeatureParam<T>* feature_param) {                     \
    static const typename base::internal::FeatureParamTraits<           \
        T>::CacheStorageType storage =                                  \
        base::internal::FeatureParamTraits<T>::ToCacheStorageType(      \
            feature_param->GetWithoutCache());                          \
    return base::internal::FeatureParamTraits<T>::FromCacheStorageType( \
        storage);                                                       \
  }                                                                     \
  } /* field_trial_params_internal */                                   \
  constinit const base::FeatureParam<T> feature_object_name(            \
      feature, name, default_value,                                     \
      &field_trial_params_internal::                                    \
          GetFeatureParamWithCacheFor##feature_object_name)

// Same as BASE_FEATURE_PARAM() but used for enum type parameters with on extra
// argument, `options`. See base::FeatureParam<Enum> template declaration in
// //base/metrics/field_trial_params.h for `options`' details.
#define BASE_FEATURE_ENUM_PARAM(T, feature_object_name, feature, name, \
                                default_value, options)                \
  namespace field_trial_params_internal {                              \
  T GetFeatureParamWithCacheFor##feature_object_name(                  \
      const base::FeatureParam<T>* feature_param) {                    \
    static const T param = feature_param->GetWithoutCache();           \
    return param;                                                      \
  }                                                                    \
  } /* field_trial_params_internal */                                  \
  constinit const base::FeatureParam<T> feature_object_name(           \
      feature, name, default_value, options,                           \
      &field_trial_params_internal::                                   \
          GetFeatureParamWithCacheFor##feature_object_name)

// Secret handshake to (try to) ensure all places that construct a base::Feature
// go through the helper `BASE_FEATURE()` macro above.
namespace internal {
enum class FeatureMacroHandshake { kSecret };
}

// The Feature struct is used to define the default state for a feature. There
// must only ever be one struct instance for a given feature name—generally
// defined as a constant global variable or file static. Declare and define
// features using the `BASE_DECLARE_FEATURE()` and `BASE_FEATURE()` macros
// above, as there are some subtleties involved.
//
// Feature constants are internally mutable, as this allows them to contain a
// mutable member to cache their override state, while still remaining declared
// as const. This cache member allows for significantly faster IsEnabled()
// checks.
//
// However, the "Mutable Constants" check [1] detects this as a regression,
// because this usually means that a readonly symbol is put in writable memory
// when readonly memory would be more efficient.
//
// The performance gains of the cache are large enough to offset the downsides
// to having the symbols in bssdata rather than rodata. Use LOGICALLY_CONST to
// suppress the "Mutable Constants" check.
//
// [1]:
// https://crsrc.org/c/docs/speed/binary_size/android_binary_size_trybot.md#Mutable-Constants
struct BASE_EXPORT LOGICALLY_CONST Feature {
  constexpr Feature(const char* name,
                    FeatureState default_state,
                    internal::FeatureMacroHandshake)
      : name(name), default_state(default_state) {
#if BUILDFLAG(ENABLE_BANNED_BASE_FEATURE_PREFIX)
    if (std::string_view(name).find(BUILDFLAG(BANNED_BASE_FEATURE_PREFIX)) ==
        0) {
      LOG(FATAL) << "Invalid feature name " << name << " starts with "
                 << BUILDFLAG(BANNED_BASE_FEATURE_PREFIX);
    }
#endif  // BUILDFLAG(ENABLE_BANNED_BASE_FEATURE_PREFIX)
  }

  // Non-copyable since:
  // - there should be only one `Feature` instance per unique name.
  // - a `Feature` contains internal cached state about the override state.
  Feature(const Feature&) = delete;
  Feature& operator=(const Feature&) = delete;

  // The name of the feature. This should be unique to each feature and is used
  // for enabling/disabling features via command line flags and experiments.
  // It is strongly recommended to use CamelCase style for feature names, e.g.
  // "MyGreatFeature".
  const char* const name;

  // The default state (i.e. enabled or disabled) for this feature.
  // NOTE: The actual runtime state may be different, due to a field trial or a
  // command line switch.
  const FeatureState default_state;

 private:
  friend class FeatureList;

  // A packed value where the first 8 bits represent the `OverrideState` of this
  // feature, and the last 16 bits are a caching context ID used to allow
  // ScopedFeatureLists to invalidate these cached values in testing. A value of
  // 0 in the caching context ID field indicates that this value has never been
  // looked up and cached, a value of 1 indicates this value contains the cached
  // `OverrideState` that was looked up via `base::FeatureList`, and any other
  // value indicate that this cached value is only valid for a particular
  // ScopedFeatureList instance.
  //
  // Packing these values into a uint32_t makes it so that atomic operations
  // performed on this fields can be lock free.
  //
  // The override state stored in this field is only used if the current
  // `FeatureList::caching_context_` field is equal to the lower 16 bits of the
  // packed cached value. Otherwise, the override state is looked up in the
  // feature list and the cache is updated.
  mutable std::atomic<uint32_t> cached_value = 0;
};

#if BUILDFLAG(DCHECK_IS_CONFIGURABLE)
// DCHECKs have been built-in, and are configurable at run-time to be fatal, or
// not, via a DcheckIsFatal feature. We define the Feature here since it is
// checked in FeatureList::SetInstance(). See https://crbug.com/596231.
BASE_EXPORT BASE_DECLARE_FEATURE(kDCheckIsFatalFeature);
#endif  // BUILDFLAG(DCHECK_IS_CONFIGURABLE)

// The FeatureList class is used to determine whether a given feature is on or
// off. It provides an authoritative answer, taking into account command-line
// overrides and experimental control.
//
// The basic use case is for any feature that can be toggled (e.g. through
// command-line or an experiment) to have a defined Feature struct, e.g.:
//
//   const base::Feature kMyGreatFeature {
//     "MyGreatFeature", base::FEATURE_ENABLED_BY_DEFAULT
//   };
//
// Then, client code that wishes to query the state of the feature would check:
//
//   if (base::FeatureList::IsEnabled(kMyGreatFeature)) {
//     // Feature code goes here.
//   }
//
// Behind the scenes, the above call would take into account any command-line
// flags to enable or disable the feature, any experiments that may control it
// and finally its default state (in that order of priority), to determine
// whether the feature is on.
//
// Features can be explicitly forced on or off by specifying a list of comma-
// separated feature names via the following command-line flags:
//
//   --enable-features=Feature5,Feature7
//   --disable-features=Feature1,Feature2,Feature3
//
// To enable/disable features in a test, do NOT append --enable-features or
// --disable-features to the command-line directly. Instead, use
// ScopedFeatureList. See base/test/scoped_feature_list.h for details.
//
// After initialization (which should be done single-threaded), the FeatureList
// API is thread safe.
//
// Note: This class is a singleton, but does not use base/memory/singleton.h in
// order to have control over its initialization sequence. Specifically, the
// intended use is to create an instance of this class and fully initialize it,
// before setting it as the singleton for a process, via SetInstance().
class BASE_EXPORT FeatureList {
 public:
  FeatureList();
  FeatureList(const FeatureList&) = delete;
  FeatureList& operator=(const FeatureList&) = delete;
  ~FeatureList();

  // Used by common test fixture classes to prevent abuse of ScopedFeatureList
  // after multiple threads have started.
  class BASE_EXPORT ScopedDisallowOverrides {
   public:
    explicit ScopedDisallowOverrides(const char* reason);
    ScopedDisallowOverrides(const ScopedDisallowOverrides&) = delete;
    ScopedDisallowOverrides& operator=(const ScopedDisallowOverrides&) = delete;
    ~ScopedDisallowOverrides();

   private:
#if DCHECK_IS_ON()
    const char* const previous_reason_;
#endif
  };

  // Specifies whether a feature override enables or disables the feature.
  enum OverrideState {
    OVERRIDE_USE_DEFAULT,
    OVERRIDE_DISABLE_FEATURE,
    OVERRIDE_ENABLE_FEATURE,
  };

  // Accessor class, used to look up features by _name_ rather than by Feature
  // object.
  // Should only be used in limited cases. See ConstructAccessor() for details.
  class BASE_EXPORT Accessor {
   public:
    Accessor(const Accessor&) = delete;
    Accessor& operator=(const Accessor&) = delete;

    // Looks up the feature, returning only its override state, rather than
    // falling back on a default value (since there is no default value given).
    // Callers of this MUST ensure that there is a consistent, compile-time
    // default value associated.
    FeatureList::OverrideState GetOverrideStateByFeatureName(
        std::string_view feature_name);

    // Look up the feature, and, if present, populate |params|.
    // See GetFieldTrialParams in field_trial_params.h for more documentation.
    bool GetParamsByFeatureName(std::string_view feature_name,
                                std::map<std::string, std::string>* params);

   private:
    // Allow FeatureList to construct this class.
    friend class FeatureList;

    explicit Accessor(FeatureList* feature_list);

    // Unowned pointer to the FeatureList object we use to look up feature
    // enablement.
    raw_ptr<FeatureList, DanglingUntriaged> feature_list_;
  };

  // Describes a feature override. The first member is a Feature that will be
  // overridden with the state given by the second member.
  using FeatureOverrideInfo =
      std::pair<const std::reference_wrapper<const Feature>, OverrideState>;

  // Initializes feature overrides via command-line flags `--enable-features=`
  // and `--disable-features=`, each of which is a comma-separated list of
  // features to enable or disable, respectively. This function also allows
  // users to set a feature's field trial params via `--enable-features=`. Must
  // only be invoked during the initialization phase (before
  // FinalizeInitialization() has been called).
  //
  // If a feature appears on both lists, then it will be disabled. If
  // a list entry has the format "FeatureName<TrialName" then this
  // initialization will also associate the feature state override with the
  // named field trial, if it exists. If a list entry has the format
  // "FeatureName:k1/v1/k2/v2", "FeatureName<TrialName:k1/v1/k2/v2" or
  // "FeatureName<TrialName.GroupName:k1/v1/k2/v2" then this initialization will
  // also associate the feature state override with the named field trial and
  // its params. If the feature params part is provided but trial and/or group
  // isn't, this initialization will also create a synthetic trial, named
  // "Study" followed by the feature name, i.e. "StudyFeature", and group, named
  // "Group" followed by the feature name, i.e. "GroupFeature", for the params.
  // If a feature name is prefixed with the '*' character, it will be created
  // with OVERRIDE_USE_DEFAULT - which is useful for associating with a trial
  // while using the default state.
  void InitFromCommandLine(const std::string& enable_features,
                           const std::string& disable_features);

  // Initializes feature overrides through the field trial allocator, which
  // we're using to store the feature names, their override state, and the name
  // of the associated field trial.
  void InitFromSharedMemory(PersistentMemoryAllocator* allocator);

  // Returns true if the state of |feature_name| has been overridden (regardless
  // of whether the overridden value is the same as the default value) for any
  // reason (e.g. command line or field trial).
  bool IsFeatureOverridden(const std::string& feature_name) const;

  // Returns true if the state of |feature_name| has been overridden via
  // |InitFromCommandLine()|. This includes features explicitly
  // disabled/enabled with --disable-features and --enable-features, as well as
  // any extra feature overrides that depend on command line switches.
  bool IsFeatureOverriddenFromCommandLine(
      const std::string& feature_name) const;

  // Returns true if the state |feature_name| has been overridden by
  // |InitFromCommandLine()| and the state matches |state|.
  bool IsFeatureOverriddenFromCommandLine(const std::string& feature_name,
                                          OverrideState state) const;

  // Associates a field trial for reporting purposes corresponding to the
  // command-line setting the feature state to |for_overridden_state|. The trial
  // will be activated when the state of the feature is first queried. This
  // should be called during registration, after InitFromCommandLine() has
  // been called but before the instance is registered via SetInstance().
  void AssociateReportingFieldTrial(const std::string& feature_name,
                                    OverrideState for_overridden_state,
                                    FieldTrial* field_trial);

  // Registers a field trial to override the enabled state of the specified
  // feature to `override_state`. Command-line overrides still take precedence
  // over field trials, so this will have no effect if the feature is being
  // overridden from the command-line. The associated field trial will be
  // activated when the feature state for this feature is queried. This should
  // be called during registration, after InitFromCommandLine() has been
  // called but before the instance is registered via SetInstance().
  void RegisterFieldTrialOverride(const std::string& feature_name,
                                  OverrideState override_state,
                                  FieldTrial* field_trial);

  // Adds extra overrides (not associated with a field trial). Should be called
  // before SetInstance().
  // The ordering of calls with respect to InitFromCommandLine(),
  // RegisterFieldTrialOverride(), etc. matters. The first call wins out,
  // because the `overrides_` map uses emplace(), which retains the first
  // inserted entry and does not overwrite it on subsequent calls to emplace().
  //
  // If `replace_use_default_overrides` is true, if there is an existing entry
  // with type OVERRIDE_USE_DEFAULT, that entry will be replaced.
  void RegisterExtraFeatureOverrides(
      const std::vector<FeatureOverrideInfo>& extra_overrides,
      bool replace_use_default_overrides = false);

  // Loops through feature overrides and serializes them all into |allocator|.
  void AddFeaturesToAllocator(PersistentMemoryAllocator* allocator);

  // Returns comma-separated lists of feature names (in the same format that is
  // accepted by InitFromCommandLine()) corresponding to features that
  // have been overridden - either through command-line or via FieldTrials. For
  // those features that have an associated FieldTrial, the output entry will be
  // of the format "FeatureName<TrialName" (|include_group_name|=false) or
  // "FeatureName<TrialName.GroupName" (if |include_group_name|=true), where
  // "TrialName" is the name of the FieldTrial and "GroupName" is the group
  // name of the FieldTrial. Features that have overrides with
  // OVERRIDE_USE_DEFAULT will be added to |enable_overrides| with a '*'
  // character prefix. Must be called only after the instance has been
  // initialized and registered.
  void GetFeatureOverrides(std::string* enable_overrides,
                           std::string* disable_overrides,
                           bool include_group_names = false) const;

  // Like GetFeatureOverrides(), but only returns overrides that were specified
  // explicitly on the command-line, omitting the ones from field trials.
  void GetCommandLineFeatureOverrides(std::string* enable_overrides,
                                      std::string* disable_overrides) const;

  // Returns the field trial associated with the given feature |name|. Used for
  // getting the FieldTrial without requiring a struct Feature.
  base::FieldTrial* GetAssociatedFieldTrialByFeatureName(
      std::string_view name) const;

  // DO NOT USE outside of internal field trial implementation code. Instead use
  // GetAssociatedFieldTrialByFeatureName(), which performs some additional
  // validation.
  //
  // Returns whether the given feature |name| is associated with a field trial.
  // If the given feature |name| does not exist, return false. Unlike
  // GetAssociatedFieldTrialByFeatureName(), this function must be called during
  // |FeatureList| initialization; the returned value will report whether the
  // provided |name| has been used so far.
  bool HasAssociatedFieldTrialByFeatureName(std::string_view name) const;

  // Get associated field trial for the given feature |name| only if override
  // enables it.
  FieldTrial* GetEnabledFieldTrialByFeatureName(std::string_view name) const;

  // Construct an accessor allowing access to GetOverrideStateByFeatureName().
  // This can only be called before the FeatureList is initialized, and is
  // intended for very narrow use.
  // If you're tempted to use it, do so only in consultation with feature_list
  // OWNERS.
  std::unique_ptr<Accessor> ConstructAccessor();

  // Returns whether the given `feature` is enabled.
  //
  // If no `FeatureList` instance is registered, this will:
  // - DCHECK(), if FailOnFeatureAccessWithoutFeatureList() was called.
  //     TODO(crbug.com/40237050): Change the DCHECK to a CHECK when we're
  //     confident that all early accesses have been fixed. We don't want to
  //     get many crash reports from the field in the meantime.
  // - Return the default state, otherwise. Registering a `FeatureList` later
  //   will fail.
  //
  // TODO(crbug.com/40237050): Make early FeatureList access fail on iOS,
  // Android and ChromeOS. This currently only works on Windows, Mac and Linux.
  //
  // A feature with a given name must only have a single corresponding Feature
  // instance, which is checked in builds with DCHECKs enabled.
  static bool IsEnabled(const Feature& feature);

  // Some characters are not allowed to appear in feature names or the
  // associated field trial names, as they are used as special characters for
  // command-line serialization. This function checks that the strings are ASCII
  // (since they are used in command-line API functions that require ASCII) and
  // whether there are any reserved characters present, returning true if the
  // string is valid.
  static bool IsValidFeatureOrFieldTrialName(std::string_view name);

  // If the given |feature| is overridden, returns its enabled state; otherwise,
  // returns an empty optional. Must only be called after the singleton instance
  // has been registered via SetInstance(). Additionally, a feature with a given
  // name must only have a single corresponding Feature struct, which is checked
  // in builds with DCHECKs enabled.
  static std::optional<bool> GetStateIfOverridden(const Feature& feature);

  // Returns the field trial associated with the given |feature|. Must only be
  // called after the singleton instance has been registered via SetInstance().
  static FieldTrial* GetFieldTrial(const Feature& feature);

  // Splits a comma-separated string containing feature names into a vector. The
  // resulting pieces point to parts of |input|.
  static std::vector<std::string_view> SplitFeatureListString(
      std::string_view input);

  // Checks and parses the |enable_feature| (e.g.
  // FeatureName<Study.Group:param1/value1/) obtained by applying
  // SplitFeatureListString() to the |enable_features| flag, and sets
  // |feature_name| to be the feature's name, |study_name| and |group_name| to
  // be the field trial name and its group name if the field trial is specified
  // or field trial parameters are given, |params| to be the field trial
  // parameters if exists.
  static bool ParseEnableFeatureString(std::string_view enable_feature,
                                       std::string* feature_name,
                                       std::string* study_name,
                                       std::string* group_name,
                                       std::string* params);

  // Initializes and sets an instance of FeatureList with feature overrides via
  // command-line flags |enable_features| and |disable_features| if one has not
  // already been set from command-line flags. Returns true if an instance did
  // not previously exist. See InitFromCommandLine() for more details
  // about |enable_features| and |disable_features| parameters.
  static bool InitInstance(const std::string& enable_features,
                           const std::string& disable_features);

  // Like the above, but also adds extra overrides. If a feature appears in
  // |extra_overrides| and also |enable_features| or |disable_features|, the
  // disable/enable will supersede the extra overrides.
  static bool InitInstance(
      const std::string& enable_features,
      const std::string& disable_features,
      const std::vector<FeatureOverrideInfo>& extra_overrides);

  // Returns the singleton instance of FeatureList. Will return null until an
  // instance is registered via SetInstance().
  static FeatureList* GetInstance();

  // Registers the given |instance| to be the singleton feature list for this
  // process. This should only be called once and |instance| must not be null.
  // Note: If you are considering using this for the purposes of testing, take
  // a look at using base/test/scoped_feature_list.h instead.
  static void SetInstance(std::unique_ptr<FeatureList> instance);

  // Registers the given `instance` to be the temporary singleton feature list
  // for this process. While the given `instance` is the singleton feature list,
  // only the state of features matching `allowed_feature_names` can be checked.
  // Attempting to query other feature will behave as if no feature list was set
  // at all. It is expected that this instance is replaced using `SetInstance`
  // with an instance without limitations as soon as practical.
  static void SetEarlyAccessInstance(
      std::unique_ptr<FeatureList> instance,
      base::flat_set<std::string> allowed_feature_names);

  // Clears the previously-registered singleton instance for tests and returns
  // the old instance.
  // Note: Most tests should never call this directly. Instead consider using
  // base::test::ScopedFeatureList.
  static std::unique_ptr<FeatureList> ClearInstanceForTesting();

  // Sets a given (initialized) |instance| to be the singleton feature list,
  // for testing. Existing instance must be null. This is primarily intended
  // to support base::test::ScopedFeatureList helper class.
  static void RestoreInstanceForTesting(std::unique_ptr<FeatureList> instance);

  // After calling this, an attempt to access feature state when no FeatureList
  // is registered will DCHECK.
  //
  // TODO(crbug.com/40237050): Change the DCHECK to a CHECK when we're confident
  // that all early accesses have been fixed. We don't want to get many crash
  // reports from the field in the meantime.
  //
  // Note: This isn't the default behavior because accesses are tolerated in
  // processes that never register a FeatureList.
  static void FailOnFeatureAccessWithoutFeatureList();

  // Returns the first feature that was accessed before a FeatureList was
  // registered that allows accessing the feature.
  static const Feature* GetEarlyAccessedFeatureForTesting();

  // Resets the state of the early feature access tracker.
  static void ResetEarlyFeatureAccessTrackerForTesting();

  // Adds a feature to the early allowed feature access list for tests. Should
  // only be called on a FeatureList that was set with SetEarlyAccessInstance().
  void AddEarlyAllowedFeatureForTesting(std::string feature_name);

  // Allows a visitor to record override state, parameters, and field trial
  // associated with each feature. Optionally, provide a prefix which filters
  // the visited features.
  //
  // NOTE: This is intended only for the special case of needing to get all
  // overrides. This use case is specific to CrOS-Ash and V8. Most users should
  // call IsEnabled() to query a feature's state.
  static void VisitFeaturesAndParams(FeatureVisitor& visitor,
                                     std::string_view filter_prefix = "");

 private:
  FRIEND_TEST_ALL_PREFIXES(FeatureListTest, CheckFeatureIdentity);
  FRIEND_TEST_ALL_PREFIXES(FeatureListTest,
                           StoreAndRetrieveFeaturesFromSharedMemory);
  FRIEND_TEST_ALL_PREFIXES(FeatureListTest,
                           StoreAndRetrieveAssociatedFeaturesFromSharedMemory);
  // Allow Accessor to access GetOverrideStateByFeatureName().
  friend class Accessor;

  struct OverrideEntry {
    // The overridden enable (on/off) state of the feature.
    OverrideState overridden_state;

    // An optional associated field trial, which will be activated when the
    // state of the feature is queried for the first time. Weak pointer to the
    // FieldTrial object that is owned by the FieldTrialList singleton.
    raw_ptr<base::FieldTrial> field_trial;

    // Specifies whether the feature's state is overridden by |field_trial|.
    // If it's not, and |field_trial| is not null, it means it is simply an
    // associated field trial for reporting purposes (and |overridden_state|
    // came from the command-line).
    bool overridden_by_field_trial;

    // TODO(asvitkine): Expand this as more support is added.

    // Constructs an OverrideEntry for the given |overridden_state|. If
    // |field_trial| is not null, it implies that |overridden_state| comes from
    // the trial, so |overridden_by_field_trial| will be set to true.
    OverrideEntry(OverrideState overridden_state, FieldTrial* field_trial);
  };

  // Returns the override for the field trial associated with the given feature
  // |name| or null if the feature is not found.
  const base::FeatureList::OverrideEntry* GetOverrideEntryByFeatureName(
      std::string_view name) const;

  // Finalizes the initialization state of the FeatureList, so that no further
  // overrides can be registered. This is called by SetInstance() on the
  // singleton feature list that is being registered.
  void FinalizeInitialization();

  // Returns whether the given |feature| is enabled. This is invoked by the
  // public FeatureList::IsEnabled() static function on the global singleton.
  // Requires the FeatureList to have already been fully initialized.
  bool IsFeatureEnabled(const Feature& feature) const;

  // Returns whether the given |feature| is enabled. This is invoked by the
  // public FeatureList::GetStateIfOverridden() static function on the global
  // singleton. Requires the FeatureList to have already been fully initialized.
  std::optional<bool> IsFeatureEnabledIfOverridden(
      const Feature& feature) const;

  // Returns the override state of a given |feature|. If the feature was not
  // overridden, returns OVERRIDE_USE_DEFAULT. Performs any necessary callbacks
  // for when the feature state has been observed, e.g. activating field trials.
  OverrideState GetOverrideState(const Feature& feature) const;

  // Same as GetOverrideState(), but without a default value.
  OverrideState GetOverrideStateByFeatureName(
      std::string_view feature_name) const;

  // Returns the field trial associated with the given |feature|. This is
  // invoked by the public FeatureList::GetFieldTrial() static function on the
  // global singleton. Requires the FeatureList to have already been fully
  // initialized.
  base::FieldTrial* GetAssociatedFieldTrial(const Feature& feature) const;

  // For each feature name in comma-separated list of strings |feature_list|,
  // registers an override with the specified |overridden_state|. Also, will
  // associate an optional named field trial if the entry is of the format
  // "FeatureName<TrialName".
  void RegisterOverridesFromCommandLine(const std::string& feature_list,
                                        OverrideState overridden_state);

  // Registers an override for feature |feature_name|. The override specifies
  // whether the feature should be on or off (via |overridden_state|), which
  // will take precedence over the feature's default state. If |field_trial| is
  // not null, registers the specified field trial object to be associated with
  // the feature, which will activate the field trial when the feature state is
  // queried.
  //
  // If an override is already registered for the given feature, it will not be
  // changed, unless `replace_use_default_overrides` is true and the existing
  // entry has type OVERRIDE_USE_DEFAULT.
  void RegisterOverride(std::string_view feature_name,
                        OverrideState overridden_state,
                        FieldTrial* field_trial,
                        bool replace_use_default_overrides = false);

  // Implementation of GetFeatureOverrides() with a parameter that specifies
  // whether only command-line enabled overrides should be emitted. See that
  // function's comments for more details.
  void GetFeatureOverridesImpl(std::string* enable_overrides,
                               std::string* disable_overrides,
                               bool command_line_only,
                               bool include_group_name = false) const;

  // Verifies that there's only a single definition of a Feature struct for a
  // given feature name. Keeps track of the first seen Feature struct for each
  // feature. Returns false when called on a Feature struct with a different
  // address than the first one it saw for that feature name. Used only from
  // DCHECKs and tests. This is const because it's called from const getters and
  // doesn't modify externally visible state.
  bool CheckFeatureIdentity(const Feature& feature) const;

  // Returns true if this feature list was set with SetEarlyAccessInstance().
  bool IsEarlyAccessInstance() const;

  // Returns if this feature list instance allows access to the given feature.
  // If a this feature list was set with SetEarlyAccessInstance(), only the
  // features in `allowed_feature_names_` can be checked.
  bool AllowFeatureAccess(const Feature& feature) const;

  // Map from feature name to an OverrideEntry struct for the feature, if it
  // exists.
  base::flat_map<std::string, OverrideEntry> overrides_;

  // Locked map that keeps track of seen features, to ensure a single feature is
  // only defined once. This verification is only done in builds with DCHECKs
  // enabled. This is mutable as it's not externally visible and needs to be
  // usable from const getters.
  mutable Lock feature_identity_tracker_lock_;
  mutable std::map<std::string, const Feature*, std::less<>>
      feature_identity_tracker_ GUARDED_BY(feature_identity_tracker_lock_);

  // Tracks the associated FieldTrialList for DCHECKs. This is used to catch
  // the scenario where multiple FieldTrialList are used with the same
  // FeatureList - which can lead to overrides pointing to invalid FieldTrial
  // objects.
  raw_ptr<base::FieldTrialList> field_trial_list_ = nullptr;

  // Whether this object has been fully initialized. This gets set to true as a
  // result of FinalizeInitialization().
  bool initialized_ = false;

  // Whether this object has been initialized from command line.
  bool initialized_from_command_line_ = false;

  // Used when querying `base::Feature` state to determine if the cached value
  // in the `Feature` object is populated and valid. See the comment on
  // `base::Feature::cached_value` for more details.
  const uint16_t caching_context_;

  // If this instance was set with SetEarlyAccessInstance(), this set contains
  // the names of the features whose state is allowed to be checked. Attempting
  // to check the state of a feature not on this list will behave as if no
  // feature list was initialized at all.
  base::flat_set<std::string> allowed_feature_names_;
};

}  // namespace base

#endif  // BASE_FEATURE_LIST_H_
