// Copyright 2024 The gRPC Authors.
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

#ifndef GRPC_SRC_CORE_TELEMETRY_METRICS_H
#define GRPC_SRC_CORE_TELEMETRY_METRICS_H

#include <grpc/support/metrics.h>
#include <grpc/support/port_platform.h>

#include <cstdint>
#include <memory>
#include <type_traits>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/functional/function_ref.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/core/util/no_destruct.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"

namespace grpc_core {

constexpr absl::string_view kMetricLabelTarget = "grpc.target";

// A global registry of instruments(metrics). This API is designed to be used
// to register instruments (Counter, Histogram, and Gauge) as part of program
// startup, before the execution of the main function (during dynamic
// initialization time). Using this API after the main function begins may
// result into missing instruments. This API is thread-unsafe.
//
// The registration of instruments is done through the templated
// RegistrationBuilder API and gets back a handle with an opaque type. At
// runtime, the handle should be used with the StatsPluginGroup API to record
// metrics for the instruments.
//
// At dynamic initialization time:
//   const auto kMetricHandle =
//       GlobalInstrumentsRegistry::RegisterUInt64Counter(
//           "name",
//           "description",
//           "unit", /*enable_by_default=*/false)
//           .Labels(kLabel1, kLabel2, kLabel3)
//           .OptionalLabels(kOptionalLabel1, kOptionalLabel2)
//           .Build();
//
// At runtime time:
//   stats_plugin_group.AddCounter(kMetricHandle, 1,
//       {"label_value_1", "label_value_2", "label_value_3"},
//       {"optional_label_value_1", "optional_label_value_2"});
//
class GlobalInstrumentsRegistry {
 public:
  enum class ValueType {
    kUndefined,
    kInt64,
    kUInt64,
    kDouble,
  };
  enum class InstrumentType {
    kUndefined,
    kCounter,
    kHistogram,
    kCallbackGauge,
  };
  using InstrumentID = uint32_t;
  struct GlobalInstrumentDescriptor {
    ValueType value_type;
    InstrumentType instrument_type;
    InstrumentID index;
    bool enable_by_default;
    absl::string_view name;
    absl::string_view description;
    absl::string_view unit;
    std::vector<absl::string_view> label_keys;
    std::vector<absl::string_view> optional_label_keys;
  };
  struct GlobalInstrumentHandle {
    // This is the index for the corresponding registered instrument that
    // StatsPlugins can use to uniquely identify an instrument in the current
    // process. Though this is not guaranteed to be stable between different
    // runs or between different versions.
    InstrumentID index;
  };

  template <ValueType V, InstrumentType I, size_t M, size_t N>
  struct TypedGlobalInstrumentHandle : public GlobalInstrumentHandle {};

  template <ValueType V, InstrumentType I, std::size_t M, std::size_t N>
  class RegistrationBuilder {
   public:
    template <typename... Args>
    RegistrationBuilder<V, I, sizeof...(Args), N> Labels(Args&&... args) {
      return RegistrationBuilder<V, I, sizeof...(Args), N>(
          name_, description_, unit_, enable_by_default_,
          std::array<absl::string_view, sizeof...(Args)>({args...}),
          optional_label_keys_);
    }

    template <typename... Args>
    RegistrationBuilder<V, I, M, sizeof...(Args)> OptionalLabels(
        Args&&... args) {
      return RegistrationBuilder<V, I, M, sizeof...(Args)>(
          name_, description_, unit_, enable_by_default_, label_keys_,
          std::array<absl::string_view, sizeof...(Args)>({args...}));
    }

    TypedGlobalInstrumentHandle<V, I, M, N> Build() {
      TypedGlobalInstrumentHandle<V, I, M, N> handle;
      handle.index = RegisterInstrument(V, I, name_, description_, unit_,
                                        enable_by_default_, label_keys_,
                                        optional_label_keys_);
      return handle;
    }

   private:
    friend class GlobalInstrumentsRegistry;

    RegistrationBuilder(absl::string_view name, absl::string_view description,
                        absl::string_view unit, bool enable_by_default)
        : name_(name),
          description_(description),
          unit_(unit),
          enable_by_default_(enable_by_default) {}

    RegistrationBuilder(absl::string_view name, absl::string_view description,
                        absl::string_view unit, bool enable_by_default,
                        std::array<absl::string_view, M> label_keys,
                        std::array<absl::string_view, N> optional_label_keys)
        : name_(name),
          description_(description),
          unit_(unit),
          enable_by_default_(enable_by_default),
          label_keys_(std::move(label_keys)),
          optional_label_keys_(std::move(optional_label_keys)) {}

    absl::string_view name_;
    absl::string_view description_;
    absl::string_view unit_;
    bool enable_by_default_;
    std::array<absl::string_view, M> label_keys_;
    std::array<absl::string_view, N> optional_label_keys_;
  };

  // Creates instrument in the GlobalInstrumentsRegistry.
  static RegistrationBuilder<ValueType::kUInt64, InstrumentType::kCounter, 0, 0>
  RegisterUInt64Counter(absl::string_view name, absl::string_view description,
                        absl::string_view unit, bool enable_by_default) {
    return RegistrationBuilder<ValueType::kUInt64, InstrumentType::kCounter, 0,
                               0>(name, description, unit, enable_by_default);
  }
  static RegistrationBuilder<ValueType::kDouble, InstrumentType::kCounter, 0, 0>
  RegisterDoubleCounter(absl::string_view name, absl::string_view description,
                        absl::string_view unit, bool enable_by_default) {
    return RegistrationBuilder<ValueType::kDouble, InstrumentType::kCounter, 0,
                               0>(name, description, unit, enable_by_default);
  }
  static RegistrationBuilder<ValueType::kUInt64, InstrumentType::kHistogram, 0,
                             0>
  RegisterUInt64Histogram(absl::string_view name, absl::string_view description,
                          absl::string_view unit, bool enable_by_default) {
    return RegistrationBuilder<ValueType::kUInt64, InstrumentType::kHistogram,
                               0, 0>(name, description, unit,
                                     enable_by_default);
  }
  static RegistrationBuilder<ValueType::kDouble, InstrumentType::kHistogram, 0,
                             0>
  RegisterDoubleHistogram(absl::string_view name, absl::string_view description,
                          absl::string_view unit, bool enable_by_default) {
    return RegistrationBuilder<ValueType::kDouble, InstrumentType::kHistogram,
                               0, 0>(name, description, unit,
                                     enable_by_default);
  }
  static RegistrationBuilder<ValueType::kInt64, InstrumentType::kCallbackGauge,
                             0, 0>
  RegisterCallbackInt64Gauge(absl::string_view name,
                             absl::string_view description,
                             absl::string_view unit, bool enable_by_default) {
    return RegistrationBuilder<ValueType::kInt64,
                               InstrumentType::kCallbackGauge, 0, 0>(
        name, description, unit, enable_by_default);
  }
  static RegistrationBuilder<ValueType::kDouble, InstrumentType::kCallbackGauge,
                             0, 0>
  RegisterCallbackDoubleGauge(absl::string_view name,
                              absl::string_view description,
                              absl::string_view unit, bool enable_by_default) {
    return RegistrationBuilder<ValueType::kDouble,
                               InstrumentType::kCallbackGauge, 0, 0>(
        name, description, unit, enable_by_default);
  }

  static void ForEach(
      absl::FunctionRef<void(const GlobalInstrumentDescriptor&)> f);
  static const GlobalInstrumentDescriptor& GetInstrumentDescriptor(
      GlobalInstrumentHandle handle);
  static std::optional<GlobalInstrumentsRegistry::GlobalInstrumentHandle>
  FindInstrumentByName(absl::string_view name);

 private:
  friend class GlobalInstrumentsRegistryTestPeer;

  GlobalInstrumentsRegistry() = delete;

  static std::vector<GlobalInstrumentsRegistry::GlobalInstrumentDescriptor>&
  GetInstrumentList();
  static InstrumentID RegisterInstrument(
      ValueType value_type, InstrumentType instrument_type,
      absl::string_view name, absl::string_view description,
      absl::string_view unit, bool enable_by_default,
      absl::Span<const absl::string_view> label_keys,
      absl::Span<const absl::string_view> optional_label_keys);
};

// An interface for implementing callback-style metrics.
// To be implemented by stats plugins.
class CallbackMetricReporter {
 public:
  virtual ~CallbackMetricReporter() = default;

  template <std::size_t M, std::size_t N>
  void Report(
      GlobalInstrumentsRegistry::TypedGlobalInstrumentHandle<
          GlobalInstrumentsRegistry::ValueType::kInt64,
          GlobalInstrumentsRegistry::InstrumentType::kCallbackGauge, M, N>
          handle,
      int64_t value, std::array<absl::string_view, M> label_values,
      std::array<absl::string_view, N> optional_values) {
    ReportInt64(handle, value, label_values, optional_values);
  }
  template <std::size_t M, std::size_t N>
  void Report(
      GlobalInstrumentsRegistry::TypedGlobalInstrumentHandle<
          GlobalInstrumentsRegistry::ValueType::kDouble,
          GlobalInstrumentsRegistry::InstrumentType::kCallbackGauge, M, N>
          handle,
      double value, std::array<absl::string_view, M> label_values,
      std::array<absl::string_view, N> optional_values) {
    ReportDouble(handle, value, label_values, optional_values);
  }

 private:
  virtual void ReportInt64(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle, int64_t value,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) = 0;
  virtual void ReportDouble(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle, double value,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_values) = 0;
};

class RegisteredMetricCallback;

// The StatsPlugin interface.
class StatsPlugin {
 public:
  // A general-purpose way for stats plugin to store per-channel or per-server
  // state.
  class ScopeConfig {
   public:
    virtual ~ScopeConfig() = default;

    // NOTE: This is safe to invoke ONLY if both ScopeConfig objects
    // come from the same StatsPlugin.
    virtual int Compare(const ScopeConfig& other) const = 0;
  };

  virtual ~StatsPlugin() = default;

  // Whether this stats plugin is enabled for the channel specified by \a scope.
  // Returns true and a channel-specific ScopeConfig which may then be used to
  // configure the ClientCallTracer in GetClientCallTracer().
  virtual std::pair<bool, std::shared_ptr<ScopeConfig>> IsEnabledForChannel(
      const experimental::StatsPluginChannelScope& scope) const = 0;
  // Whether this stats plugin is enabled for the server specified by \a args.
  // Returns true and a server-specific ScopeConfig which may then be used to
  // configure the ServerCallTracer in GetServerCallTracer().
  virtual std::pair<bool, std::shared_ptr<ScopeConfig>> IsEnabledForServer(
      const ChannelArgs& args) const = 0;
  // Gets a scope config for the client channel specified by \a scope. Note that
  // the stats plugin should have been enabled for the channel.
  virtual std::shared_ptr<StatsPlugin::ScopeConfig> GetChannelScopeConfig(
      const experimental::StatsPluginChannelScope& scope) const = 0;
  // Gets a scope config for the server specified by \a args. Note that the
  // stats plugin should have been enabled for the server.
  virtual std::shared_ptr<StatsPlugin::ScopeConfig> GetServerScopeConfig(
      const ChannelArgs& args) const = 0;

  // Adds \a value to the uint64 counter specified by \a handle. \a label_values
  // and \a optional_label_values specify attributes that are associated with
  // this measurement and must match with their corresponding keys in
  // GlobalInstrumentsRegistry::RegisterUInt64Counter().
  virtual void AddCounter(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle, uint64_t value,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_label_values) = 0;
  // Adds \a value to the double counter specified by \a handle. \a label_values
  // and \a optional_label_values specify attributes that are associated with
  // this measurement and must match with their corresponding keys in
  // GlobalInstrumentsRegistry::RegisterDoubleCounter().
  virtual void AddCounter(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle, double value,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_label_values) = 0;
  // Records a uint64 \a value to the histogram specified by \a handle. \a
  // label_values and \a optional_label_values specify attributes that are
  // associated with this measurement and must match with their corresponding
  // keys in GlobalInstrumentsRegistry::RegisterUInt64Histogram().
  virtual void RecordHistogram(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle, uint64_t value,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_label_values) = 0;
  // Records a double \a value to the histogram specified by \a handle. \a
  // label_values and \a optional_label_values specify attributes that are
  // associated with this measurement and must match with their corresponding
  // keys in GlobalInstrumentsRegistry::RegisterDoubleHistogram().
  virtual void RecordHistogram(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle, double value,
      absl::Span<const absl::string_view> label_values,
      absl::Span<const absl::string_view> optional_label_values) = 0;
  // Adds a callback to be invoked when the stats plugin wants to
  // populate the corresponding metrics (see callback->metrics() for list).
  virtual void AddCallback(RegisteredMetricCallback* callback) = 0;
  // Removes a callback previously added via AddCallback().  The stats
  // plugin may not use the callback after this method returns.
  virtual void RemoveCallback(RegisteredMetricCallback* callback) = 0;
  // Returns true if instrument \a handle is enabled.
  virtual bool IsInstrumentEnabled(
      GlobalInstrumentsRegistry::GlobalInstrumentHandle handle) const = 0;

  // Gets a ClientCallTracer associated with this stats plugin which can be used
  // in a call.
  virtual ClientCallTracer* GetClientCallTracer(
      const Slice& path, bool registered_method,
      std::shared_ptr<ScopeConfig> scope_config) = 0;
  // Gets a ServerCallTracer associated with this stats plugin which can be used
  // in a call.
  virtual ServerCallTracer* GetServerCallTracer(
      std::shared_ptr<ScopeConfig> scope_config) = 0;

  // TODO(yijiem): This is an optimization for the StatsPlugin to create its own
  // representation of the label_values and use it multiple times. We would
  // change AddCounter and RecordHistogram to take RefCountedPtr<LabelValueSet>
  // and also change the StatsPluginsGroup to support this.
  // Use the StatsPlugin to get a representation of label values that can be
  // saved for multiple uses later.
  // virtual RefCountedPtr<LabelValueSet> MakeLabelValueSet(
  //     absl::Span<absl::string_view> label_values) = 0;
};

// A global registry of stats plugins. It has shared ownership to the registered
// stats plugins. This API is supposed to be used during runtime after the main
// function begins. This API is thread-safe.
class GlobalStatsPluginRegistry {
 public:
  // A stats plugin group object is how the code in gRPC normally interacts with
  // stats plugins. They got a stats plugin group which contains all the stats
  // plugins for a specific scope and all operations on the stats plugin group
  // will be applied to all the stats plugins within the group.
  class StatsPluginGroup
      : public std::enable_shared_from_this<StatsPluginGroup> {
   public:
    // Adds a stats plugin and a scope config (per-channel or per-server) to the
    // group.
    void AddStatsPlugin(std::shared_ptr<StatsPlugin> plugin,
                        std::shared_ptr<StatsPlugin::ScopeConfig> config) {
      PluginState plugin_state;
      plugin_state.plugin = std::move(plugin);
      plugin_state.scope_config = std::move(config);
      plugins_state_.push_back(std::move(plugin_state));
    }
    // Adds a counter in all stats plugins within the group. See the StatsPlugin
    // interface for more documentation and valid types.
    template <std::size_t M, std::size_t N>
    void AddCounter(
        GlobalInstrumentsRegistry::TypedGlobalInstrumentHandle<
            GlobalInstrumentsRegistry::ValueType::kUInt64,
            GlobalInstrumentsRegistry::InstrumentType::kCounter, M, N>
            handle,
        uint64_t value, std::array<absl::string_view, M> label_values,
        std::array<absl::string_view, N> optional_values) {
      for (auto& state : plugins_state_) {
        state.plugin->AddCounter(handle, value, label_values, optional_values);
      }
    }
    template <std::size_t M, std::size_t N>
    void AddCounter(
        GlobalInstrumentsRegistry::TypedGlobalInstrumentHandle<
            GlobalInstrumentsRegistry::ValueType::kDouble,
            GlobalInstrumentsRegistry::InstrumentType::kCounter, M, N>
            handle,
        double value, std::array<absl::string_view, M> label_values,
        std::array<absl::string_view, N> optional_values) {
      for (auto& state : plugins_state_) {
        state.plugin->AddCounter(handle, value, label_values, optional_values);
      }
    }
    // Records a value to a histogram in all stats plugins within the group. See
    // the StatsPlugin interface for more documentation and valid types.
    template <std::size_t M, std::size_t N>
    void RecordHistogram(
        GlobalInstrumentsRegistry::TypedGlobalInstrumentHandle<
            GlobalInstrumentsRegistry::ValueType::kUInt64,
            GlobalInstrumentsRegistry::InstrumentType::kHistogram, M, N>
            handle,
        uint64_t value, std::array<absl::string_view, M> label_values,
        std::array<absl::string_view, N> optional_values) {
      for (auto& state : plugins_state_) {
        state.plugin->RecordHistogram(handle, value, label_values,
                                      optional_values);
      }
    }
    template <std::size_t M, std::size_t N>
    void RecordHistogram(
        GlobalInstrumentsRegistry::TypedGlobalInstrumentHandle<
            GlobalInstrumentsRegistry::ValueType::kDouble,
            GlobalInstrumentsRegistry::InstrumentType::kHistogram, M, N>
            handle,
        double value, std::array<absl::string_view, M> label_values,
        std::array<absl::string_view, N> optional_values) {
      for (auto& state : plugins_state_) {
        state.plugin->RecordHistogram(handle, value, label_values,
                                      optional_values);
      }
    }
    // Returns true if any of the stats plugins in the group have enabled \a
    // handle.
    bool IsInstrumentEnabled(
        GlobalInstrumentsRegistry::GlobalInstrumentHandle handle) const {
      for (auto& state : plugins_state_) {
        if (state.plugin->IsInstrumentEnabled(handle)) {
          return true;
        }
      }
      return false;
    }

    size_t size() const { return plugins_state_.size(); }

    // Registers a callback to be used to populate callback metrics.
    // The callback will update the specified metrics.  The callback
    // will be invoked no more often than min_interval.  Multiple callbacks may
    // be registered for the same metrics, as long as no two callbacks report
    // data for the same set of labels in which case the behavior is undefined.
    //
    // The returned object is a handle that allows the caller to control
    // the lifetime of the callback; when the returned object is
    // destroyed, the callback is de-registered.  The returned object
    // must not outlive the StatsPluginGroup object that created it.
    template <typename... Args>
    GRPC_MUST_USE_RESULT std::unique_ptr<RegisteredMetricCallback>
    RegisterCallback(absl::AnyInvocable<void(CallbackMetricReporter&)> callback,
                     Duration min_interval, Args... args);

    // Adds all available client call tracers associated with the stats plugins
    // within the group to \a call_context.
    void AddClientCallTracers(const Slice& path, bool registered_method,
                              Arena* arena);
    // Adds all available server call tracers associated with the stats plugins
    // within the group to \a call_context.
    void AddServerCallTracers(Arena* arena);

    static absl::string_view ChannelArgName() {
      return "grpc.internal.stats_plugin_group";
    }
    static int ChannelArgsCompare(const StatsPluginGroup* a,
                                  const StatsPluginGroup* b);

   private:
    friend class RegisteredMetricCallback;

    struct PluginState {
      std::shared_ptr<StatsPlugin::ScopeConfig> scope_config;
      std::shared_ptr<StatsPlugin> plugin;
    };

    template <GlobalInstrumentsRegistry::ValueType V,
              GlobalInstrumentsRegistry::InstrumentType I, size_t M, size_t N>
    static constexpr void AssertIsCallbackGaugeHandle(
        GlobalInstrumentsRegistry::TypedGlobalInstrumentHandle<V, I, M, N>) {
      static_assert(V == GlobalInstrumentsRegistry::ValueType::kInt64 ||
                        V == GlobalInstrumentsRegistry::ValueType::kDouble,
                    "ValueType must be kInt64 or kDouble");
      static_assert(
          I == GlobalInstrumentsRegistry::InstrumentType::kCallbackGauge,
          "InstrumentType must be kCallbackGauge");
    }

    std::vector<PluginState> plugins_state_;
  };

  // Registers a stats plugin with the global stats plugin registry.
  static void RegisterStatsPlugin(std::shared_ptr<StatsPlugin> plugin);

  // The following functions can be invoked to get a StatsPluginGroup for
  // a specified scope.
  static std::shared_ptr<StatsPluginGroup> GetStatsPluginsForChannel(
      const experimental::StatsPluginChannelScope& scope);
  static std::shared_ptr<StatsPluginGroup> GetStatsPluginsForServer(
      const ChannelArgs& args);

 private:
  struct GlobalStatsPluginNode {
    std::shared_ptr<StatsPlugin> plugin;
    GlobalStatsPluginNode* next = nullptr;
  };
  friend class GlobalStatsPluginRegistryTestPeer;

  GlobalStatsPluginRegistry() = default;

  static std::atomic<GlobalStatsPluginNode*> plugins_;
};

// A metric callback that is registered with a stats plugin group.
class RegisteredMetricCallback {
 public:
  RegisteredMetricCallback(
      GlobalStatsPluginRegistry::StatsPluginGroup& stats_plugin_group,
      absl::AnyInvocable<void(CallbackMetricReporter&)> callback,
      std::vector<GlobalInstrumentsRegistry::GlobalInstrumentHandle> metrics,
      Duration min_interval);

  ~RegisteredMetricCallback();

  // Invokes the callback.  The callback will report metric data via reporter.
  void Run(CallbackMetricReporter& reporter) { callback_(reporter); }

  // Returns the set of metrics that this callback will modify.
  const std::vector<GlobalInstrumentsRegistry::GlobalInstrumentHandle>&
  metrics() const {
    return metrics_;
  }

  // Returns the minimum interval at which a stats plugin may invoke the
  // callback.
  Duration min_interval() const { return min_interval_; }

 private:
  GlobalStatsPluginRegistry::StatsPluginGroup& stats_plugin_group_;
  absl::AnyInvocable<void(CallbackMetricReporter&)> callback_;
  std::vector<GlobalInstrumentsRegistry::GlobalInstrumentHandle> metrics_;
  Duration min_interval_;
};

template <typename... Args>
inline std::unique_ptr<RegisteredMetricCallback>
GlobalStatsPluginRegistry::StatsPluginGroup::RegisterCallback(
    absl::AnyInvocable<void(CallbackMetricReporter&)> callback,
    Duration min_interval, Args... args) {
  (AssertIsCallbackGaugeHandle(args), ...);
  return std::make_unique<RegisteredMetricCallback>(
      *this, std::move(callback),
      std::vector<GlobalInstrumentsRegistry::GlobalInstrumentHandle>{args...},
      min_interval);
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_TELEMETRY_METRICS_H
