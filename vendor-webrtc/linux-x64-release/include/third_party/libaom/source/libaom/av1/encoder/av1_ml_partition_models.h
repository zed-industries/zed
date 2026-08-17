/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_AV1_ML_PARTITION_MODELS_H_
#define AOM_AV1_ENCODER_AV1_ML_PARTITION_MODELS_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "av1/encoder/ml.h"

// TODO(kyslov): Replace with proper weights after training AV1 models

#define FEATURES 6
static const float av1_var_part_nn_weights_64_layer0[FEATURES * 8] = {
  0.35755366f,  0.86281112f,  -0.20871686f, 0.0409634f,   0.97305766f,
  0.75510254f,  0.04860447f,  0.77095283f,  -0.44105278f, -0.3755049f,
  -0.08456618f, 1.1821136f,   -0.73956301f, 1.30016453f,  0.45566902f,
  0.4742967f,   0.44213975f,  0.4876028f,   0.26720522f,  -0.34429858f,
  -0.25148252f, -0.49623932f, -0.46747941f, -0.36656624f, 0.10213375f,
  0.60262819f,  -0.54788715f, -0.27272022f, 1.0995462f,   -0.36338376f,
  -0.64836313f, 0.16057039f,  1.02782791f,  0.9985311f,   0.90607883f,
  0.80570411f,  -0.07750863f, -0.74006402f, 1.72839526f,  1.72355343f,
  1.69288916f,  1.59102043f,  0.14140216f,  -1.47262839f, 0.4262519f,
  -0.33805936f, -0.02449707f, 0.67203692f
};

static const float av1_var_part_nn_bias_64_layer0[8] = {
  0.39995694f, 0.65593756f, 1.12876737f,  1.28790576f,
  0.53468556f, 0.3177908f,  -0.74388266f, -1.81131248f
};

static const float av1_var_part_nn_weights_64_layer1[8] = {
  -1.31174053f, 0.69696917f, 0.78721456f, 0.45326379f,
  0.79258322f,  1.74626188f, -5.41831f,   3.33887435f
};

static const float av1_var_part_nn_bias_64_layer1[1] = { -0.90951047f };

static const float av1_var_part_means_64[FEATURES] = {
  5.36750249f, 11.58023127f, 0.25550964f, 0.23809917f, 0.24650665f, 0.22117687f
};
static const float av1_var_part_vars_64[FEATURES] = {
  0.89599769f, 2.2686018f, 0.02568608f, 0.02523411f, 0.02443085f, 0.01922085f
};

static const NN_CONFIG av1_var_part_nnconfig_64 = {
  FEATURES,  // num_inputs
  1,         // num_outputs
  1,         // num_hidden_layers
  {
      8,
  },  // num_hidden_nodes
  {
      av1_var_part_nn_weights_64_layer0,
      av1_var_part_nn_weights_64_layer1,
  },
  {
      av1_var_part_nn_bias_64_layer0,
      av1_var_part_nn_bias_64_layer1,
  },
};

static const float av1_var_part_nn_weights_32_layer0[FEATURES * 8] = {
  0.97886049f,  -1.66262011f, 0.94902798f,  0.7080922f,   0.91181186f,
  0.35222601f,  -0.04428585f, 0.42086472f,  -0.0206325f,  -0.77937809f,
  -0.70947522f, -1.24463119f, 0.23739497f,  -1.34327359f, 0.01024804f,
  0.4544633f,   -0.96907661f, 0.67279522f,  0.23180693f,  1.54063368f,
  -0.15700707f, 0.18597331f,  0.34167589f,  0.40736558f,  0.69213366f,
  -1.33584593f, 1.21190814f,  1.26725267f,  1.21284802f,  1.26611399f,
  0.17546514f,  -0.30248399f, -1.32589316f, -1.37432674f, -1.37423023f,
  -1.26890855f, 0.12166347f,  -0.94565678f, -1.47475267f, -0.69279948f,
  -0.10166587f, -0.23489881f, 0.57123565f,  0.80051137f,  -1.28411946f,
  -1.36576732f, -1.30257508f, -1.30575106f
};

static const float av1_var_part_nn_bias_32_layer0[8] = {
  -1.6301435f, 0.61879037f, -1.68612662f, 1.66960165f,
  -0.0838243f, 0.32253287f, -0.65755282f, 0.96661531f
};

static const float av1_var_part_nn_weights_32_layer1[8] = {
  1.99257161f,  0.7331492f,  1.33539961f,  1.13501456f,
  -2.21154528f, 1.85858542f, -0.85565298f, -1.96410246f
};

static const float av1_var_part_nn_bias_32_layer1[1] = { -0.14880827f };

static const float av1_var_part_means_32[FEATURES] = {
  5.36360686f, 9.88421868f, 0.23543671f, 0.23621205f, 0.23409667f, 0.22855539f
};

static const float av1_var_part_vars_32[FEATURES] = {
  0.89077225f, 2.32312894f, 0.02167654f, 0.02392842f, 0.02466495f, 0.02047641f
};

static const NN_CONFIG av1_var_part_nnconfig_32 = {
  FEATURES,  // num_inputs
  1,         // num_outputs
  1,         // num_hidden_layers
  {
      8,
  },  // num_hidden_nodes
  {
      av1_var_part_nn_weights_32_layer0,
      av1_var_part_nn_weights_32_layer1,
  },
  {
      av1_var_part_nn_bias_32_layer0,
      av1_var_part_nn_bias_32_layer1,
  },
};

static const float av1_var_part_nn_weights_16_layer0[FEATURES * 8] = {
  0.45118305f,  -0.22068295f, 0.4604435f,   -0.1446326f,  -0.15765035f,
  0.42260198f,  -0.0945916f,  0.49544996f,  0.62781567f,  -0.41564372f,
  -0.39103292f, 0.44407624f,  0.48382613f,  -0.85424238f, -0.00961433f,
  0.25383582f,  0.14403897f,  0.00901859f,  -0.83201967f, -0.19323284f,
  0.59271213f,  0.69487457f,  0.6897112f,   0.62768521f,  0.9204492f,
  -1.42448347f, -0.16491054f, -0.10114424f, -0.1069687f,  -0.11289049f,
  0.26290832f,  -0.41850393f, 0.17239733f,  0.41770622f,  0.43725942f,
  0.19362467f,  -0.35955731f, -0.899446f,   0.49726389f,  0.66569571f,
  0.65893982f,  0.53199654f,  -0.1158694f,  -0.26472603f, 0.4155923f,
  0.15059544f,  0.09596755f,  0.26247133f
};

static const float av1_var_part_nn_bias_16_layer0[8] = {
  1.64486321f, -0.11851574f, 1.29322833f,  -0.61193136f,
  0.33027532f, 1.04197232f,  -0.80716674f, 0.88681233f
};

static const float av1_var_part_nn_weights_16_layer1[8] = {
  -1.02832118f, 0.72800106f, -0.42904783f, 1.44490586f,
  -1.03888227f, -0.9023916f, -1.51543102f, -0.43059521f
};

static const float av1_var_part_nn_bias_16_layer1[1] = { -0.85087946f };

static const float av1_var_part_means_16[FEATURES] = {
  5.32551326f, 8.218448f, 0.21954822f, 0.22808377f, 0.23019798f, 0.22320699f
};

static const float av1_var_part_vars_16[FEATURES] = { 0.86806032f, 2.39938956f,
                                                      0.01958579f, 0.02437927f,
                                                      0.02420755f, 0.0192003f };

static const NN_CONFIG av1_var_part_nnconfig_16 = {
  FEATURES,  // num_inputs
  1,         // num_outputs
  1,         // num_hidden_layers
  {
      8,
  },  // num_hidden_nodes
  {
      av1_var_part_nn_weights_16_layer0,
      av1_var_part_nn_weights_16_layer1,
  },
  {
      av1_var_part_nn_bias_16_layer0,
      av1_var_part_nn_bias_16_layer1,
  },
};

#undef FEATURES

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_AV1_ML_PARTITION_MODELS_H_
