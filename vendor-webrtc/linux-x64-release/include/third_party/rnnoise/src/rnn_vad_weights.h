#ifndef THIRD_PARTY_RNNOISE_SRC_RNN_VAD_WEIGHTS_H_
#define THIRD_PARTY_RNNOISE_SRC_RNN_VAD_WEIGHTS_H_

#include <cstdint>
#include <cstring>

namespace rnnoise {

// Weights scaling factor.
const float kWeightsScale = 1.f / 256.f;

// Input layer (dense).
const size_t kInputLayerInputSize = 42;
const size_t kInputLayerOutputSize = 24;
const size_t kInputLayerWeights = kInputLayerInputSize * kInputLayerOutputSize;
extern const int8_t kInputDenseWeights[kInputLayerWeights];
extern const int8_t kInputDenseBias[kInputLayerOutputSize];

// Hidden layer (GRU).
const size_t kHiddenLayerOutputSize = 24;
const size_t kHiddenLayerWeights =
    3 * kInputLayerOutputSize * kHiddenLayerOutputSize;
const size_t kHiddenLayerBiases = 3 * kHiddenLayerOutputSize;
extern const int8_t kHiddenGruWeights[kHiddenLayerWeights];
extern const int8_t kHiddenGruRecurrentWeights[kHiddenLayerWeights];
extern const int8_t kHiddenGruBias[kHiddenLayerBiases];

// Output layer (dense).
const size_t kOutputLayerOutputSize = 1;
const size_t kOutputLayerWeights =
    kHiddenLayerOutputSize * kOutputLayerOutputSize;
extern const int8_t kOutputDenseWeights[kOutputLayerWeights];
extern const int8_t kOutputDenseBias[kOutputLayerOutputSize];

}  // namespace rnnoise

#endif  // THIRD_PARTY_RNNOISE_SRC_RNN_VAD_WEIGHTS_H_
