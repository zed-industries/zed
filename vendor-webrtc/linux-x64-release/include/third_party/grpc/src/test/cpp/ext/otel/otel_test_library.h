//
//
// Copyright 2023 gRPC authors.
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

#ifndef GRPC_TEST_CPP_EXT_OTEL_OTEL_TEST_LIBRARY_H
#define GRPC_TEST_CPP_EXT_OTEL_OTEL_TEST_LIBRARY_H

#include <grpc/support/port_platform.h>
#include <grpcpp/generic/generic_stub.h>
#include <grpcpp/grpcpp.h>

#include <atomic>
#include <thread>

#include "absl/functional/any_invocable.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "opentelemetry/metrics/provider.h"
#include "opentelemetry/sdk/metrics/meter_provider.h"
#include "opentelemetry/sdk/metrics/metric_reader.h"
#include "opentelemetry/version.h"
#include "src/core/config/core_configuration.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/cpp/ext/otel/otel_plugin.h"
#include "test/core/test_util/test_config.h"
#include "test/cpp/end2end/test_service_impl.h"

OPENTELEMETRY_BEGIN_NAMESPACE
namespace sdk {
namespace metrics {

// GTest uses `PrintTo` functions to print values. OTel's PointDataAttributes
// doesn't include one of these, so we add one ourselves in their namespace.
void PrintTo(const PointDataAttributes& point_data_attributes,
             std::ostream* os);

}  // namespace metrics
}  // namespace sdk
OPENTELEMETRY_END_NAMESPACE

namespace grpc {
namespace testing {

class MockMetricReader : public opentelemetry::sdk::metrics::MetricReader {
 public:
  opentelemetry::sdk::metrics::AggregationTemporality GetAggregationTemporality(
      opentelemetry::sdk::metrics::InstrumentType) const noexcept override {
    return opentelemetry::sdk::metrics::AggregationTemporality::kDelta;
  }

  bool OnForceFlush(std::chrono::microseconds) noexcept override {
    return true;
  }

  bool OnShutDown(std::chrono::microseconds) noexcept override { return true; }

  void OnInitialized() noexcept override {}
};

class OpenTelemetryPluginEnd2EndTest : public ::testing::Test {
 protected:
  struct Options {
   public:
    Options& set_metric_names(std::vector<absl::string_view> names) {
      metric_names = std::move(names);
      return *this;
    }

    Options& set_resource(const opentelemetry::sdk::resource::Resource& res) {
      resource = std::make_unique<opentelemetry::sdk::resource::Resource>(res);
      return *this;
    }

    Options& set_use_meter_provider(bool flag) {
      use_meter_provider = flag;
      return *this;
    }

    Options& set_labels_to_inject(
        std::map<
            grpc_core::ClientCallTracer::CallAttemptTracer::OptionalLabelKey,
            grpc_core::RefCountedStringValue>
            labels) {
      labels_to_inject = std::move(labels);
      return *this;
    }

    Options& set_service_config(std::string svc_cfg) {
      service_config = std::move(svc_cfg);
      return *this;
    }

    Options& set_channel_scope_filter(
        absl::AnyInvocable<bool(
            const OpenTelemetryPluginBuilder::ChannelScope& /*scope*/) const>
            func) {
      channel_scope_filter = std::move(func);
      return *this;
    }

    Options& set_server_selector(
        absl::AnyInvocable<bool(const grpc_core::ChannelArgs& /*channel_args*/)
                               const>
            func) {
      server_selector = std::move(func);
      return *this;
    }

    Options& set_target_attribute_filter(
        absl::AnyInvocable<bool(absl::string_view /*target*/) const> func) {
      target_attribute_filter = std::move(func);
      return *this;
    }

    Options& set_generic_method_attribute_filter(
        absl::AnyInvocable<bool(absl::string_view /*generic_method*/) const>
            func) {
      generic_method_attribute_filter = std::move(func);
      return *this;
    }

    Options& add_plugin_option(
        std::unique_ptr<grpc::internal::InternalOpenTelemetryPluginOption>
            option) {
      plugin_options.push_back(std::move(option));
      return *this;
    }

    Options& add_optional_label(absl::string_view optional_label_key) {
      optional_label_keys.emplace(optional_label_key);
      return *this;
    }

    Options& add_per_channel_stats_plugin(
        std::shared_ptr<grpc::experimental::OpenTelemetryPlugin> plugin) {
      per_channel_stats_plugins.emplace_back(std::move(plugin));
      return *this;
    }

    Options& add_per_server_stats_plugin(
        std::shared_ptr<grpc::experimental::OpenTelemetryPlugin> plugin) {
      per_server_stats_plugins.emplace_back(std::move(plugin));
      return *this;
    }

    std::vector<absl::string_view> metric_names;
    // TODO(yashykt): opentelemetry::sdk::resource::Resource doesn't have a copy
    // assignment operator so wrapping it in a unique_ptr till it is fixed.
    std::unique_ptr<opentelemetry::sdk::resource::Resource> resource =
        std::make_unique<opentelemetry::sdk::resource::Resource>(
            opentelemetry::sdk::resource::Resource::Create({}));
    std::unique_ptr<grpc::internal::LabelsInjector> labels_injector;
    bool use_meter_provider = true;
    std::map<grpc_core::ClientCallTracer::CallAttemptTracer::OptionalLabelKey,
             grpc_core::RefCountedStringValue>
        labels_to_inject;
    std::string service_config;
    absl::AnyInvocable<bool(
        const OpenTelemetryPluginBuilder::ChannelScope& /*scope*/) const>
        channel_scope_filter;
    absl::AnyInvocable<bool(const grpc_core::ChannelArgs& /*channel_args*/)
                           const>
        server_selector;
    absl::AnyInvocable<bool(absl::string_view /*target*/) const>
        target_attribute_filter;
    absl::AnyInvocable<bool(absl::string_view /*generic_method*/) const>
        generic_method_attribute_filter;
    std::vector<
        std::unique_ptr<grpc::internal::InternalOpenTelemetryPluginOption>>
        plugin_options;
    absl::flat_hash_set<absl::string_view> optional_label_keys;
    std::vector<std::shared_ptr<grpc::experimental::OpenTelemetryPlugin>>
        per_channel_stats_plugins;
    std::vector<std::shared_ptr<grpc::experimental::OpenTelemetryPlugin>>
        per_server_stats_plugins;
  };

  class MetricsCollectorThread {
   public:
    using ResultType = absl::flat_hash_map<
        std::string,
        std::vector<opentelemetry::sdk::metrics::PointDataAttributes>>;
    MetricsCollectorThread(OpenTelemetryPluginEnd2EndTest* test,
                           grpc_core::Duration interval, int iterations,
                           std::function<bool(const ResultType&)> predicate);
    ~MetricsCollectorThread();
    const ResultType& Stop();

   private:
    void Run();

    OpenTelemetryPluginEnd2EndTest* test_;
    grpc_core::Duration interval_;
    int iterations_;
    std::function<bool(const ResultType&)> predicate_;
    ResultType data_points_;
    std::atomic_bool finished_{false};
    std::thread thread_;
  };

  static std::shared_ptr<opentelemetry::sdk::metrics::MetricReader>
  ConfigureOTBuilder(
      OpenTelemetryPluginEnd2EndTest::Options options,
      grpc::internal::OpenTelemetryPluginBuilderImpl* ot_builder);

  // Note that we can't use SetUp() here since we want to send in parameters.
  void Init(Options config);

  void TearDown() override;

  void ResetStub(std::shared_ptr<Channel> channel);

  void SendRPC();
  void SendGenericRPC();

  std::pair<std::shared_ptr<grpc::experimental::OpenTelemetryPlugin>,
            std::shared_ptr<opentelemetry::sdk::metrics::MetricReader>>
  BuildOpenTelemetryPlugin(OpenTelemetryPluginEnd2EndTest::Options options);

  std::shared_ptr<opentelemetry::sdk::metrics::MetricReader>
  BuildAndRegisterOpenTelemetryPlugin(
      OpenTelemetryPluginEnd2EndTest::Options options);

  absl::flat_hash_map<
      std::string,
      std::vector<opentelemetry::sdk::metrics::PointDataAttributes>>
  ReadCurrentMetricsData(
      absl::AnyInvocable<
          bool(const absl::flat_hash_map<
               std::string,
               std::vector<opentelemetry::sdk::metrics::PointDataAttributes>>&)>
          continue_predicate,
      opentelemetry::sdk::metrics::MetricReader* reader = nullptr);

  const absl::string_view kMethodName = "grpc.testing.EchoTestService/Echo";
  const absl::string_view kGenericMethodName = "foo/bar";
  std::map<grpc_core::ClientCallTracer::CallAttemptTracer::OptionalLabelKey,
           grpc_core::RefCountedStringValue>
      labels_to_inject_;
  std::shared_ptr<opentelemetry::sdk::metrics::MetricReader> reader_;
  std::string server_address_;
  std::string canonical_server_address_;
  CallbackTestServiceImpl service_;
  std::unique_ptr<grpc::Server> server_;
  std::unique_ptr<EchoTestService::Stub> stub_;
  std::unique_ptr<grpc::GenericStub> generic_stub_;
};

template <typename T>
void PopulateLabelMap(
    T label_keys, T label_values,
    std::unordered_map<std::string,
                       opentelemetry::sdk::common::OwnedAttributeValue>*
        label_maps) {
  for (size_t i = 0; i < label_keys.size(); ++i) {
    (*label_maps)[std::string(label_keys[i])] = std::string(label_values[i]);
  }
}

MATCHER_P4(AttributesEq, label_keys, label_values, optional_label_keys,
           optional_label_values, "") {
  std::unordered_map<std::string,
                     opentelemetry::sdk::common::OwnedAttributeValue>
      label_map;
  PopulateLabelMap(label_keys, label_values, &label_map);
  PopulateLabelMap(optional_label_keys, optional_label_values, &label_map);
  return ::testing::ExplainMatchResult(
      ::testing::UnorderedElementsAreArray(label_map),
      arg.attributes.GetAttributes(), result_listener);
}

template <typename T>
struct Extract;

template <template <typename> class T, typename U>
struct Extract<const T<U>> {
  using Type = U;
};

MATCHER_P(CounterResultEq, value_matcher, "") {
  return ::testing::ExplainMatchResult(
      ::testing::VariantWith<opentelemetry::sdk::metrics::SumPointData>(
          ::testing::Field(&opentelemetry::sdk::metrics::SumPointData::value_,
                           ::testing::VariantWith<
                               typename Extract<decltype(value_matcher)>::Type>(
                               value_matcher))),
      arg.point_data, result_listener);
}

MATCHER_P4(HistogramResultEq, sum_matcher, min_matcher, max_matcher, count,
           "") {
  return ::testing::ExplainMatchResult(
      ::testing::VariantWith<
          opentelemetry::sdk::metrics::HistogramPointData>(::testing::AllOf(
          ::testing::Field(
              &opentelemetry::sdk::metrics::HistogramPointData::sum_,
              ::testing::VariantWith<
                  typename Extract<decltype(sum_matcher)>::Type>(sum_matcher)),
          ::testing::Field(
              &opentelemetry::sdk::metrics::HistogramPointData::min_,
              ::testing::VariantWith<
                  typename Extract<decltype(min_matcher)>::Type>(min_matcher)),
          ::testing::Field(
              &opentelemetry::sdk::metrics::HistogramPointData::max_,
              ::testing::VariantWith<
                  typename Extract<decltype(max_matcher)>::Type>(max_matcher)),
          ::testing::Field(
              &opentelemetry::sdk::metrics::HistogramPointData::count_,
              ::testing::Eq(count)))),
      arg.point_data, result_listener);
}

MATCHER_P(GaugeResultIs, value_matcher, "") {
  return ::testing::ExplainMatchResult(
      ::testing::VariantWith<opentelemetry::sdk::metrics::LastValuePointData>(
          ::testing::AllOf(
              ::testing::Field(
                  &opentelemetry::sdk::metrics::LastValuePointData::value_,
                  ::testing::VariantWith<
                      typename Extract<decltype(value_matcher)>::Type>(
                      value_matcher)),
              ::testing::Field(&opentelemetry::sdk::metrics::
                                   LastValuePointData::is_lastvalue_valid_,
                               ::testing::IsTrue()))),
      arg.point_data, result_listener);
}

// This check might subject to system clock adjustment.
MATCHER_P(GaugeResultLaterThan, prev_timestamp, "") {
  return ::testing::ExplainMatchResult(
      ::testing::VariantWith<opentelemetry::sdk::metrics::LastValuePointData>(
          ::testing::Field(
              &opentelemetry::sdk::metrics::LastValuePointData::sample_ts_,
              ::testing::Property(
                  &opentelemetry::common::SystemTimestamp::time_since_epoch,
                  ::testing::Gt(prev_timestamp.time_since_epoch())))),
      arg.point_data, result_listener);
}

MATCHER_P7(GaugeDataIsIncrementalForSpecificMetricAndLabelSet, metric_name,
           label_key, label_value, optional_label_key, optional_label_value,
           default_value, greater_than, "") {
  std::unordered_map<std::string,
                     opentelemetry::sdk::common::OwnedAttributeValue>
      label_map;
  PopulateLabelMap(label_key, label_value, &label_map);
  PopulateLabelMap(optional_label_key, optional_label_value, &label_map);
  opentelemetry::common::SystemTimestamp prev_timestamp;
  auto prev_value = default_value;
  size_t prev_index = 0;
  auto& data = arg.at(metric_name);
  bool result = true;
  for (size_t i = 1; i < data.size(); ++i) {
    if (::testing::Matches(::testing::UnorderedElementsAreArray(
            data[i - 1].attributes.GetAttributes()))(label_map)) {
      // Update the previous value for the same associated label values.
      prev_value = opentelemetry::nostd::get<decltype(prev_value)>(
          opentelemetry::nostd::get<
              opentelemetry::sdk::metrics::LastValuePointData>(
              data[i - 1].point_data)
              .value_);
      prev_index = i - 1;
      prev_timestamp = opentelemetry::nostd::get<
                           opentelemetry::sdk::metrics::LastValuePointData>(
                           data[i - 1].point_data)
                           .sample_ts_;
    }
    if (!::testing::Matches(::testing::UnorderedElementsAreArray(
            data[i].attributes.GetAttributes()))(label_map)) {
      // Skip values that do not have the same associated label values.
      continue;
    }
    *result_listener << " Comparing data[" << i << "] with data[" << prev_index
                     << "] ";
    if (greater_than) {
      result &= ::testing::ExplainMatchResult(
          ::testing::AllOf(
              AttributesEq(label_key, label_value, optional_label_key,
                           optional_label_value),
              GaugeResultIs(::testing::Gt(prev_value)),
              GaugeResultLaterThan(prev_timestamp)),
          data[i], result_listener);
    } else {
      result &= ::testing::ExplainMatchResult(
          ::testing::AllOf(
              AttributesEq(label_key, label_value, optional_label_key,
                           optional_label_value),
              GaugeResultIs(::testing::Ge(prev_value)),
              GaugeResultLaterThan(prev_timestamp)),
          data[i], result_listener);
    }
  }
  return result;
}

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_EXT_OTEL_OTEL_TEST_LIBRARY_H
