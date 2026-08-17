// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_PREDICTION_FILTER_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_PREDICTION_FILTER_FACTORY_H_

#include <unordered_map>

#include "base/feature_list.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/widget/input/prediction/predictor_factory.h"
#include "ui/base/prediction/input_filter.h"

namespace blink {

namespace test {
class FilterFactoryTest;
}  // namespace test

namespace input_prediction {

enum class FilterType {
  kEmpty,
  kOneEuro,
};
}  // namespace input_prediction

// Structure used as key in the unordered_map to store different filter params
// in function of a trio {Filter, Predictor, Feature}
struct FilterParamMapKey {
  bool operator==(const FilterParamMapKey& other) const {
    return filter_type == other.filter_type &&
           predictor_type == other.predictor_type;
  }
  input_prediction::FilterType filter_type;
  input_prediction::PredictorType predictor_type;
};

// Used to compute a hash value for FilterParamMapKey so it can be used as key
// in a hashmap
struct FilterParamMapKeyHash {
  std::size_t operator()(const FilterParamMapKey& k) const {
    return std::hash<input_prediction::FilterType>{}(k.filter_type) ^
           std::hash<input_prediction::PredictorType>{}(k.predictor_type);
  }
};

using FilterParams = std::unordered_map<std::string, double>;
using FilterParamsMap ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/1404327)") =
    std::unordered_map<FilterParamMapKey, FilterParams, FilterParamMapKeyHash>;

// FilterFactory is a class containing methods to create filters.
// It defines filters name and type constants. It also reads filter settings
// from fieldtrials if needed.
class PLATFORM_EXPORT FilterFactory {
 public:
  FilterFactory(const base::Feature& feature,
                const input_prediction::PredictorType predictor_type,
                const input_prediction::FilterType filter_type);
  ~FilterFactory();

  // Returns the FilterType associated to the given filter
  // name if found, otherwise returns kFilterTypeEmpty
  static input_prediction::FilterType GetFilterTypeFromName(
      const std::string& filter_name);

  // Creates and returns a filter. The filter type and params are set when
  // creating the factory.
  std::unique_ptr<ui::InputFilter> CreateFilter();

 private:
  friend class test::FilterFactoryTest;

  // Predictor type used to decide parameters when creating the filter.
  input_prediction::PredictorType predictor_type_;
  // Filter type used to create the filter.
  input_prediction::FilterType filter_type_;

  // Map storing filter parameters for a pair {FilterType, PredictorType}.
  // Currently the map is only storing values from fieldtrials params, but
  // default parameters might be added for a specific predictor/filter pair
  // in the future.
  FilterParamsMap filter_params_map_;

  // Initialize the filter_params_map_ with values from fieldtrials params for
  // a given feature, predictor and filter.
  // Might initialize some default values for specific predictor/filter pair in
  // the future.
  void LoadFilterParams(const base::Feature& feature,
                        const input_prediction::PredictorType predictor_type,
                        const input_prediction::FilterType filter_type);

  // Gets filter params for a specific key couple {FilterType, PredictorType}
  // If params are found, update the given filter_params map.
  void GetFilterParams(const input_prediction::FilterType filter_type,
                       const input_prediction::PredictorType predictor_type,
                       FilterParams* filter_params);
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_PREDICTION_FILTER_FACTORY_H_
