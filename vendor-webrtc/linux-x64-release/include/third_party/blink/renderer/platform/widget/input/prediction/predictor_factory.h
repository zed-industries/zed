// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_PREDICTION_PREDICTOR_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_PREDICTION_PREDICTOR_FACTORY_H_

#include "third_party/blink/public/platform/web_common.h"
#include "ui/base/prediction/input_predictor.h"

namespace blink {

namespace input_prediction {

enum class PredictorType {
  kScrollPredictorTypeLsq,
  kScrollPredictorTypeKalman,
  kScrollPredictorTypeLinearFirst,
  kScrollPredictorTypeLinearSecond,
  kScrollPredictorTypeLinearResampling,
  kScrollPredictorTypeEmpty
};

}  // namespace input_prediction

class PredictorFactory {
 public:
  // Returns the PredictorType associated to the given predictor
  // name if found, otherwise returns kScrollPredictorTypeEmpty
  static input_prediction::PredictorType GetPredictorTypeFromName(
      const std::string& predictor_name);

  // Returns the predictor designed by its type if found, otherwise returns
  // PredictorEmpty
  static std::unique_ptr<ui::InputPredictor> GetPredictor(
      input_prediction::PredictorType predictor_type);

  // Returns the feature enabled kalman predictor options
  static unsigned int GetKalmanPredictorOptions();

  // Predictor options cache
  static unsigned int predictor_options_;

 private:
  PredictorFactory() = delete;
  ~PredictorFactory() = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_PREDICTION_PREDICTOR_FACTORY_H_
