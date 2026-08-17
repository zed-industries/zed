// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_LANGUAGE_MODEL_PARAMS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_LANGUAGE_MODEL_PARAMS_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class LanguageModelParams final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  LanguageModelParams(uint64_t default_top_k,
                      uint64_t max_top_k,
                      float default_temperature,
                      float max_temperature);

  void Trace(Visitor* visitor) const override;

  // ai_language_model_params.idl implementation
  uint64_t defaultTopK() const { return default_top_k_; }
  uint64_t maxTopK() const { return max_top_k_; }
  float defaultTemperature() const { return default_temperature_; }
  float maxTemperature() const { return max_temperature_; }

 private:
  const uint64_t default_top_k_;
  const uint64_t max_top_k_;
  const float default_temperature_;
  const float max_temperature_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_LANGUAGE_MODEL_PARAMS_H_
