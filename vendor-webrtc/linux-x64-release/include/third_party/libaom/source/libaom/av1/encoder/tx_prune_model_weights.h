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

/*! \file
 * Contains the details of the ML models used for pruning transform size. This
 * file is only included by av1/encoder/tx_search.c.
 */
#ifndef AOM_AV1_ENCODER_TX_PRUNE_MODEL_WEIGHTS_H_
#define AOM_AV1_ENCODER_TX_PRUNE_MODEL_WEIGHTS_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "av1/encoder/ml.h"

/***************************CONFIG_NN_V2 (New)********************************/
#if CONFIG_NN_V2
// Tx type model for 4x4 block.
static float av1_tx_type_nn_4x4_hor_layer0_weights[32] = {
  -1.64947f, -1.54497f, -1.62832f, -0.17774f, -2.89498f, -0.72498f, 0.72036f,
  0.17996f,  1.20000f,  -0.27654f, 0.77396f,  1.21684f,  -1.75909f, -0.51272f,
  -1.25923f, 0.35005f,  -0.04257f, -0.23389f, -0.41841f, -0.08229f, 0.09503f,
  2.73144f,  -0.16875f, -0.23482f, 0.02194f,  -0.26427f, 0.28049f,  0.21260f,
  1.35792f,  0.27733f,  0.88660f,  -0.68304f,
};

static float av1_tx_type_nn_4x4_hor_layer0_bias[8] = {
  1.38742f, 0.59540f,  -1.37622f, 1.92114f,
  0.00000f, -0.38998f, -0.32726f, -0.15650f,
};

static float av1_tx_type_nn_4x4_hor_layer1_weights[32] = {
  1.65254f,  1.00915f,  -0.89318f, -2.05142f, -0.23235f, 0.96781f,  -0.37145f,
  -0.21056f, 1.13891f,  0.38675f,  0.87739f,  -1.42697f, 0.48015f,  0.61883f,
  -0.03979f, 0.11487f,  0.48042f,  0.45200f,  -0.23242f, 0.75166f,  0.55458f,
  0.39452f,  -0.35285f, 1.59120f,  -1.49221f, -0.48349f, -0.64692f, 1.49297f,
  -0.26782f, -0.65416f, -0.10648f, 0.05568f,
};

static float av1_tx_type_nn_4x4_hor_layer1_bias[4] = {
  4.07177f,
  3.26961f,
  0.58083f,
  1.21199f,
};

static float av1_tx_type_nn_4x4_hor_layer0_out[8] = { 0 };
static float av1_tx_type_nn_4x4_hor_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_4x4_hor = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          4,                                      // num_inputs
          8,                                      // num_outputs
          av1_tx_type_nn_4x4_hor_layer0_weights,  // weights
          av1_tx_type_nn_4x4_hor_layer0_bias,     // bias
          RELU,                                   // activation
          av1_tx_type_nn_4x4_hor_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          8,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_4x4_hor_layer1_weights,
          av1_tx_type_nn_4x4_hor_layer1_bias,
          NONE,
          av1_tx_type_nn_4x4_hor_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                  // num_outputs
  av1_tx_type_nn_4x4_hor_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};

static float av1_tx_type_nn_4x4_ver_layer0_weights[32] = {
  -0.02032f, 2.61610f,  0.02098f,  -0.30217f, 0.12637f,  0.11017f,  -3.01996f,
  0.35144f,  1.93776f,  -0.20463f, 1.64102f,  -1.41986f, -3.66717f, -0.51655f,
  0.43910f,  0.37778f,  -1.02634f, 0.85337f,  -0.69753f, 1.00206f,  2.11784f,
  1.89427f,  1.92919f,  0.43201f,  -1.67358f, -1.67035f, -1.54623f, 0.16714f,
  -0.06589f, -0.28142f, -0.33118f, 1.72227f,
};

static float av1_tx_type_nn_4x4_ver_layer0_bias[8] = {
  -0.33685f, 0.22025f,  0.28140f, 0.56138f,
  0.93489f,  -1.77048f, 1.34989f, -0.93747f,
};

static float av1_tx_type_nn_4x4_ver_layer1_weights[32] = {
  -1.39506f, -1.06271f, -1.10886f, -1.69719f, 0.19699f,  -2.39850f, -1.26457f,
  0.75328f,  -1.26005f, -0.82738f, -0.12015f, -1.02702f, 1.40828f,  -2.37739f,
  -0.65639f, -0.71992f, -0.90453f, -1.12510f, -2.41362f, -1.16061f, -1.85577f,
  -0.99165f, -1.91366f, 0.16785f,  0.34776f,  0.58154f,  -0.18217f, -0.29257f,
  -0.86315f, -0.53336f, 0.30320f,  -1.32331f,
};

static float av1_tx_type_nn_4x4_ver_layer1_bias[4] = {
  -1.31519f,
  -3.26321f,
  1.71794f,
  -1.90778f,
};

static float av1_tx_type_nn_4x4_ver_layer0_out[8] = { 0 };
static float av1_tx_type_nn_4x4_ver_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_4x4_ver = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          4,                                      // num_inputs
          8,                                      // num_outputs
          av1_tx_type_nn_4x4_ver_layer0_weights,  // weights
          av1_tx_type_nn_4x4_ver_layer0_bias,     // bias
          RELU,                                   // activation
          av1_tx_type_nn_4x4_ver_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          8,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_4x4_ver_layer1_weights,
          av1_tx_type_nn_4x4_ver_layer1_bias,
          NONE,
          av1_tx_type_nn_4x4_ver_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                  // num_outputs
  av1_tx_type_nn_4x4_ver_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};
/******************************************************************************/

// Tx type model for 4x8 block.
static float av1_tx_type_nn_4x8_hor_layer0_weights[32] = {
  0.00218f,  -0.41880f, -0.61215f, -0.92588f, 0.54291f,  -0.10898f, 0.70691f,
  0.46819f,  -1.61598f, -0.08834f, -0.96839f, 1.18489f,  -0.45171f, -0.65445f,
  -0.32179f, -0.10399f, 1.04379f,  0.91895f,  0.85589f,  0.08267f,  1.35388f,
  -2.03096f, 0.08168f,  -0.06372f, -0.26732f, -0.48262f, -0.08682f, 2.44071f,
  -1.35896f, -1.17121f, 1.68866f,  0.10357f,
};

static float av1_tx_type_nn_4x8_hor_layer0_bias[8] = {
  2.93391f,  0.66831f, -0.21419f, 0.00000f,
  -0.72878f, 0.15127f, -1.46755f, 0.16658f,
};

static float av1_tx_type_nn_4x8_hor_layer1_weights[32] = {
  -1.52077f, -1.06243f, 0.35319f,  -0.49207f, 0.54524f,  0.44271f, 1.37117f,
  -0.38957f, -1.28889f, -0.57133f, 0.04658f,  0.62278f,  0.37984f, 0.33247f,
  1.65547f,  -0.56806f, -1.38645f, -0.76258f, 0.67926f,  0.08783f, -0.01443f,
  0.34950f,  1.45812f,  -0.51332f, -1.41331f, -0.16453f, 0.05755f, 0.31405f,
  -0.50191f, 0.18219f,  1.83664f,  -0.75276f,
};

static float av1_tx_type_nn_4x8_hor_layer1_bias[4] = {
  -1.17455f,
  -2.26089f,
  -1.79863f,
  -2.26333f,
};

static float av1_tx_type_nn_4x8_hor_layer0_out[8] = { 0 };
static float av1_tx_type_nn_4x8_hor_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_4x8_hor = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          4,                                      // num_inputs
          8,                                      // num_outputs
          av1_tx_type_nn_4x8_hor_layer0_weights,  // weights
          av1_tx_type_nn_4x8_hor_layer0_bias,     // bias
          RELU,                                   // activation
          av1_tx_type_nn_4x8_hor_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          8,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_4x8_hor_layer1_weights,
          av1_tx_type_nn_4x8_hor_layer1_bias,
          NONE,
          av1_tx_type_nn_4x8_hor_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                  // num_outputs
  av1_tx_type_nn_4x8_hor_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};

static float av1_tx_type_nn_4x8_ver_layer0_weights[128] = {
  -0.00952f, -0.98858f, -0.93181f, 1.39594f,  0.96559f,  0.18162f,  -0.76064f,
  -0.06066f, 0.07907f,  -0.09365f, -0.21313f, -0.02187f, -2.61707f, -2.68702f,
  -0.10982f, 0.18559f,  1.17049f,  1.11387f,  1.12697f,  1.05804f,  1.12764f,
  1.06318f,  1.12052f,  0.17406f,  1.83157f,  0.19362f,  0.46910f,  0.39608f,
  0.33342f,  0.40083f,  0.27645f,  1.06864f,  -4.06645f, -0.38775f, -0.11070f,
  0.03781f,  -0.09141f, 0.06185f,  -0.04852f, 0.20163f,  0.16784f,  0.16641f,
  -0.50941f, -0.61087f, 2.07008f,  -0.82381f, -0.85558f, 0.05528f,  -0.10535f,
  -2.81150f, 0.67038f,  0.43643f,  0.49062f,  -0.04465f, 0.90438f,  0.00977f,
  0.46272f,  1.59751f,  0.95234f,  0.35086f,  0.85624f,  0.73149f,  1.67779f,
  -2.21511f, -1.24746f, -1.09014f, -0.92441f, -1.22591f, -1.06961f, -0.95897f,
  -1.24956f, 0.73797f,  1.23275f,  -0.60064f, -0.07851f, 0.14397f,  0.22110f,
  -0.04422f, 0.14350f,  0.75926f,  0.35032f,  0.48104f,  2.81408f,  0.34662f,
  0.42090f,  0.35521f,  -1.36804f, -0.14974f, -0.47696f, -0.07892f, 0.36910f,
  0.32299f,  0.23916f,  0.06032f,  -0.17844f, -0.17558f, -1.42746f, -0.55828f,
  -1.00418f, -0.64823f, -0.73654f, -0.85197f, -1.50989f, 1.69385f,  -0.04973f,
  -0.09273f, 1.04249f,  0.79235f,  1.13229f,  0.99617f,  0.03851f,  0.56334f,
  0.90795f,  1.08296f,  0.58519f,  1.74765f,  0.63971f,  1.35951f,  0.07803f,
  -0.05127f, 0.26514f,  -0.84629f, -0.66343f, -2.10630f, 0.11017f,  2.18528f,
  -0.21958f, 0.05970f,
};

static float av1_tx_type_nn_4x8_ver_layer0_bias[16] = {
  0.04205f, 0.22260f, -1.03870f, -1.19568f, 0.44283f,  0.01143f,
  0.00235f, 4.26772f, 0.44364f,  -0.33199f, -0.39076f, -0.35129f,
  0.08288f, 0.18195f, -0.79890f, 0.10047f,
};

static float av1_tx_type_nn_4x8_ver_layer1_weights[64] = {
  -0.38193f, -0.12095f, 1.57802f,  0.34932f,  -0.47333f, -0.12304f, -0.01736f,
  -2.52445f, 0.18983f,  -0.64707f, -0.60889f, -0.53750f, 0.91666f,  -0.62823f,
  -0.13377f, -0.43594f, -0.38618f, -0.01328f, 0.97457f,  1.48589f,  -1.03238f,
  -0.33459f, -0.35108f, -2.42417f, 0.60229f,  0.06824f,  -0.75495f, 0.26902f,
  0.65311f,  -0.23887f, -0.44604f, -0.55800f, -0.33842f, 0.04259f,  -0.59589f,
  0.49738f,  -0.62301f, -0.30896f, -0.29602f, -2.57052f, 2.00943f,  -0.66490f,
  -0.76312f, 0.28256f,  1.06311f,  -0.38364f, -0.63508f, -0.57609f, -0.88765f,
  -1.04403f, -0.46531f, 0.34084f,  -1.20498f, -0.68352f, -0.72251f, -2.63242f,
  -0.68736f, -0.37904f, -1.32371f, 0.47288f,  1.51904f,  0.78372f,  -1.01830f,
  -1.01848f,
};

static float av1_tx_type_nn_4x8_ver_layer1_bias[4] = {
  -1.45955f,
  -2.08949f,
  -1.24813f,
  -1.55368f,
};

static float av1_tx_type_nn_4x8_ver_layer0_out[16] = { 0 };
static float av1_tx_type_nn_4x8_ver_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_4x8_ver = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          8,                                      // num_inputs
          16,                                     // num_outputs
          av1_tx_type_nn_4x8_ver_layer0_weights,  // weights
          av1_tx_type_nn_4x8_ver_layer0_bias,     // bias
          RELU,                                   // activation
          av1_tx_type_nn_4x8_ver_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          16,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_4x8_ver_layer1_weights,
          av1_tx_type_nn_4x8_ver_layer1_bias,
          NONE,
          av1_tx_type_nn_4x8_ver_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                  // num_outputs
  av1_tx_type_nn_4x8_ver_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};

/******************************************************************************/

// Tx type model for 8x4 block.
static float av1_tx_type_nn_8x4_hor_layer0_weights[128] = {
  -0.22492f, 0.13341f,  -4.03243f, -0.64015f, 0.02783f,  0.60466f,  -0.13335f,
  0.16828f,  0.12336f,  0.52904f,  1.18455f,  -0.32425f, 0.13052f,  0.93810f,
  -3.71165f, 0.02990f,  -4.63558f, 0.05666f,  0.03524f,  -0.07449f, -0.44006f,
  -0.33215f, -0.33713f, 0.08097f,  0.60873f,  0.29582f,  0.21696f,  -0.78729f,
  -0.16757f, -0.26567f, -0.00720f, -1.11226f, 1.58189f,  1.58463f,  1.48536f,
  1.54374f,  1.60069f,  1.46125f,  1.53932f,  0.05974f,  -1.82192f, 0.47043f,
  0.38090f,  0.20833f,  -0.05637f, 0.05183f,  0.01323f,  -0.25662f, 0.78634f,
  -0.55069f, -0.02975f, -1.29294f, -0.77192f, -2.34299f, -1.28074f, 0.77894f,
  -1.69740f, -1.66032f, -1.44323f, -1.55063f, -1.50845f, -1.23690f, -1.80663f,
  0.75079f,  2.32551f,  0.05878f,  0.80438f,  0.88584f,  0.69153f,  0.89060f,
  0.73660f,  0.87259f,  -0.00745f, -1.30044f, -0.59430f, 2.07270f,  1.03307f,
  -0.84697f, -1.19393f, 0.17549f,  -0.24978f, -3.67234f, 0.20781f,  -0.53946f,
  -0.05068f, 0.88274f,  1.30371f,  0.10288f,  0.07585f,  0.12259f,  -0.30815f,
  0.25437f,  -2.82096f, -2.69482f, 0.02370f,  0.12500f,  -0.21019f, -0.49220f,
  0.03638f,  -0.29795f, 0.28645f,  -0.48432f, -0.38584f, -0.32148f, -0.47197f,
  0.32437f,  0.32528f,  -0.19437f, 0.30383f,  -0.31879f, 0.26359f,  -0.12164f,
  -0.43647f, -0.08288f, -0.33438f, -0.63608f, -0.46647f, -0.46574f, 0.47806f,
  -0.49012f, -1.51234f, -1.13502f, -1.20470f, -1.02913f, -1.09182f, -0.93921f,
  -1.85523f, 0.92532f,
};

static float av1_tx_type_nn_8x4_hor_layer0_bias[16] = {
  0.36631f,  0.02901f,  0.64305f,  1.53074f, -1.40229f, 0.03852f,
  -0.05043f, 0.89632f,  -1.23312f, 0.07036f, 0.17070f,  0.56250f,
  -0.28958f, -0.32869f, -0.01704f, 0.68171f,
};

static float av1_tx_type_nn_8x4_hor_layer1_weights[64] = {
  -0.49441f, -0.31960f, -0.84946f, -0.85800f, -2.37767f, 0.81373f,  -0.73172f,
  -0.69337f, 0.88807f,  -0.49242f, -0.44717f, -0.11436f, 0.09978f,  0.15393f,
  0.17083f,  1.44850f,  -0.20582f, -0.04906f, 0.42990f,  -0.61939f, -1.09692f,
  -1.14885f, -1.36879f, -1.30828f, -0.59558f, -0.30903f, -0.08906f, 0.06953f,
  0.15383f,  -0.04193f, -0.54858f, 1.82676f,  -0.22411f, 0.05264f,  -0.45848f,
  -0.72985f, 0.87553f,  0.04116f,  -1.29774f, -2.63018f, 1.09089f,  -0.36048f,
  -0.16725f, 0.11627f,  0.49918f,  0.07539f,  0.00763f,  0.73706f,  0.87800f,
  0.57049f,  0.60969f,  1.02779f,  1.53339f,  -0.35915f, 0.06410f,  1.44582f,
  0.09698f,  0.71888f,  0.60594f,  0.84103f,  -0.50440f, -0.38825f, 0.15626f,
  -1.10654f,
};

static float av1_tx_type_nn_8x4_hor_layer1_bias[4] = {
  -0.92861f,
  -1.45151f,
  -1.33588f,
  -4.33853f,
};

static float av1_tx_type_nn_8x4_hor_layer0_out[16] = { 0 };
static float av1_tx_type_nn_8x4_hor_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_8x4_hor = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          8,                                      // num_inputs
          16,                                     // num_outputs
          av1_tx_type_nn_8x4_hor_layer0_weights,  // weights
          av1_tx_type_nn_8x4_hor_layer0_bias,     // bias
          RELU,                                   // activation
          av1_tx_type_nn_8x4_hor_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          16,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_8x4_hor_layer1_weights,
          av1_tx_type_nn_8x4_hor_layer1_bias,
          NONE,
          av1_tx_type_nn_8x4_hor_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                  // num_outputs
  av1_tx_type_nn_8x4_hor_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};

static float av1_tx_type_nn_8x4_ver_layer0_weights[32] = {
  -1.10946f, 1.86574f,  -1.59343f, 0.27018f, -1.70676f, -0.73982f, -0.19021f,
  -1.94208f, -2.29759f, -1.44402f, 0.28700f, -1.18340f, -1.50158f, -0.44175f,
  -1.36831f, 1.00374f,  2.59312f,  0.50291f, -0.71042f, -0.12238f, -0.15901f,
  -0.22807f, -0.67376f, -0.30215f, 0.54407f, -0.45538f, 1.18262f,  2.28687f,
  1.66212f,  1.70826f,  1.55182f,  0.12230f,
};

static float av1_tx_type_nn_8x4_ver_layer0_bias[8] = {
  0.10943f,  2.09789f, 2.16578f, 0.15766f,
  -0.42461f, 0.00000f, 1.22090f, -1.28717f,
};

static float av1_tx_type_nn_8x4_ver_layer1_weights[32] = {
  1.20426f,  -1.23237f, 2.41053f, -0.72488f, 1.25249f,  0.18018f,  -0.09586f,
  2.17901f,  0.15364f,  1.21535f, -0.38263f, -0.74309f, 0.50551f,  -0.54208f,
  0.59139f,  1.16095f,  0.55919f, -0.60183f, 1.18949f,  1.60787f,  0.54002f,
  -0.10712f, -0.16153f, 0.16207f, -0.32338f, 2.68712f,  -2.83483f, -0.27086f,
  -1.15005f, -0.39311f, 1.51236f, -1.68973f,
};

static float av1_tx_type_nn_8x4_ver_layer1_bias[4] = {
  1.81013f,
  1.10517f,
  2.90059f,
  0.95391f,
};

static float av1_tx_type_nn_8x4_ver_layer0_out[8] = { 0 };
static float av1_tx_type_nn_8x4_ver_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_8x4_ver = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          4,                                      // num_inputs
          8,                                      // num_outputs
          av1_tx_type_nn_8x4_ver_layer0_weights,  // weights
          av1_tx_type_nn_8x4_ver_layer0_bias,     // bias
          RELU,                                   // activation
          av1_tx_type_nn_8x4_ver_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          8,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_8x4_ver_layer1_weights,
          av1_tx_type_nn_8x4_ver_layer1_bias,
          NONE,
          av1_tx_type_nn_8x4_ver_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                  // num_outputs
  av1_tx_type_nn_8x4_ver_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};
/******************************************************************************/

// Tx type model for 8x8 block.
static float av1_tx_type_nn_8x8_hor_layer0_weights[128] = {
  -0.85529f, 0.37619f,  0.12754f,  0.08622f,  0.45278f,  0.54929f,  1.60651f,
  -0.62654f, -0.54929f, -0.10131f, -0.17569f, 0.13948f,  0.31695f,  -0.05616f,
  0.20483f,  -0.36448f, 2.27203f,  -0.33087f, 0.47679f,  0.86888f,  0.39370f,
  0.46239f,  0.01113f,  1.50327f,  -1.48226f, -1.69621f, -1.49777f, -1.38885f,
  -1.37753f, -1.22681f, -1.70576f, 0.51329f,  -1.65662f, 1.74197f,  -0.13579f,
  -0.13133f, -0.58396f, -0.55510f, -1.10709f, -2.34975f, 0.22445f,  -0.56491f,
  -0.83432f, 0.13492f,  1.32147f,  2.85285f,  0.13819f,  0.03792f,  -1.30792f,
  0.04155f,  -0.70644f, -0.43430f, -0.16212f, -0.86945f, -1.16976f, 1.68339f,
  0.29540f,  0.01137f,  -0.25335f, -0.16856f, 0.12028f,  0.05207f,  0.39357f,
  -0.01545f, -0.21980f, -1.94091f, -1.01315f, -0.68270f, -0.40590f, -0.67111f,
  2.08283f,  0.19291f,  -4.81426f, -0.65044f, -0.24598f, 0.06371f,  -0.10272f,
  -0.14502f, -0.06821f, 0.45202f,  0.21091f,  -0.80864f, 0.39255f,  1.79189f,
  1.80453f,  1.10484f,  1.17608f,  0.96901f,  -0.35871f, -0.94311f, 0.63147f,
  2.95157f,  0.45917f,  -0.42849f, -0.55643f, -0.06097f, 3.49299f,  -0.50972f,
  0.11075f,  -0.08405f, -0.09274f, -0.22694f, -0.42426f, 0.48632f,  -1.61074f,
  1.82998f,  0.37623f,  -1.20330f, -0.01142f, -1.33307f, -0.27492f, -2.23621f,
  1.38846f,  1.42085f,  1.42568f,  1.36152f,  1.46910f,  1.27473f,  1.34752f,
  0.12753f,  -1.08197f, -1.08280f, -0.79489f, -1.12338f, -1.06795f, -0.87857f,
  -0.99892f, 1.09823f,
};

static float av1_tx_type_nn_8x8_hor_layer0_bias[16] = {
  -0.49232f, -0.29685f, -1.44020f, 1.10940f,  1.16452f, -0.34862f,
  -0.38761f, -0.36243f, 0.21776f,  0.28234f,  2.34269f, -0.04104f,
  -0.26319f, 2.65579f,  -1.30137f, -0.01487f,
};

static float av1_tx_type_nn_8x8_hor_layer1_weights[64] = {
  -0.38058f, -0.41295f, -1.26884f, -0.75560f, -1.57450f, 0.56072f,  -1.42322f,
  -0.29106f, 0.07228f,  0.04391f,  1.61388f,  -0.03055f, 0.81637f,  2.06045f,
  0.27119f,  -0.48328f, -0.45528f, -0.60534f, -1.61209f, -0.78157f, -1.65034f,
  0.60958f,  -1.30523f, 0.25143f,  0.11398f,  0.37860f,  1.54829f,  0.02309f,
  0.67288f,  2.11447f,  0.44845f,  -0.70406f, -0.67897f, -0.38759f, -1.30383f,
  -1.22646f, -1.54571f, 0.60552f,  -1.52565f, 0.11469f,  0.17344f,  0.08622f,
  1.57906f,  -0.00909f, 0.81634f,  2.04909f,  1.26466f,  -1.45741f, -0.75229f,
  0.06200f,  -1.05835f, -0.66257f, -1.73766f, 0.99923f,  -1.87082f, 0.14580f,
  0.49525f,  0.46839f,  1.32203f,  0.33923f,  0.97001f,  2.38584f,  1.58811f,
  0.06161f,
};

static float av1_tx_type_nn_8x8_hor_layer1_bias[4] = {
  1.70385f,
  1.82373f,
  1.78496f,
  1.80826f,
};

static float av1_tx_type_nn_8x8_hor_layer0_out[16] = { 0 };
static float av1_tx_type_nn_8x8_hor_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_8x8_hor = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          8,                                      // num_inputs
          16,                                     // num_outputs
          av1_tx_type_nn_8x8_hor_layer0_weights,  // weights
          av1_tx_type_nn_8x8_hor_layer0_bias,     // bias
          RELU,                                   // activation
          av1_tx_type_nn_8x8_hor_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          16,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_8x8_hor_layer1_weights,
          av1_tx_type_nn_8x8_hor_layer1_bias,
          NONE,
          av1_tx_type_nn_8x8_hor_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                  // num_outputs
  av1_tx_type_nn_8x8_hor_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};

static float av1_tx_type_nn_8x8_ver_layer0_weights[128] = {
  -0.67016f, -1.72366f, -1.86576f, -1.50962f, -1.70419f, -1.73964f, -1.84615f,
  2.09681f,  -0.05081f, -0.61030f, 2.02541f,  0.60222f,  0.99936f,  2.02114f,
  -0.53893f, -0.23757f, 0.73566f,  0.25443f,  0.00132f,  -0.74036f, -0.75351f,
  -0.76964f, -1.71007f, -0.15770f, 1.60982f,  2.17638f,  0.90681f,  0.64973f,
  0.85914f,  0.58786f,  -1.46228f, 0.05187f,  1.18804f,  0.30850f,  0.29512f,
  0.40526f,  0.37635f,  0.32311f,  0.37471f,  1.12346f,  3.41856f,  -0.36653f,
  0.42537f,  -0.19240f, 0.00155f,  0.30826f,  -0.02116f, -0.53435f, -0.34829f,
  -0.52466f, -0.11521f, -0.29163f, -2.05689f, -2.87372f, -0.62626f, 0.09585f,
  -0.75257f, 0.10057f,  1.43474f,  0.89450f,  0.75900f,  1.11147f,  1.00558f,
  0.25886f,  2.22095f,  -0.17926f, 0.57161f,  0.39546f,  0.47846f,  0.40452f,
  0.54298f,  0.45814f,  -3.62788f, -3.02374f, 0.03716f,  -0.13937f, -0.09415f,
  -0.12463f, 0.05682f,  0.03672f,  1.20746f,  1.25003f,  1.27071f,  1.31883f,
  1.27473f,  1.34943f,  1.23158f,  0.09039f,  0.19388f,  0.63420f,  2.79612f,
  0.93803f,  -0.11323f, -0.02027f, 0.41286f,  -0.05979f, -3.80705f, -0.52451f,
  -0.77098f, -0.68132f, -0.65559f, -0.60975f, -1.26165f, 0.25582f,  0.05346f,
  0.61403f,  0.32140f,  -2.39831f, -1.42355f, 1.30541f,  1.02361f,  0.12930f,
  -1.61469f, -0.77036f, -0.59144f, 1.27769f,  1.52068f,  0.82137f,  1.83159f,
  -0.66626f, -0.69806f, -1.00564f, -0.85995f, -0.90889f, -0.84412f, -0.85712f,
  -1.29848f, 0.39308f,
};

static float av1_tx_type_nn_8x8_ver_layer0_bias[16] = {
  -0.14868f, -0.48343f, 3.94416f,  -0.78037f, -1.33789f, -0.60611f,
  0.51793f,  0.44030f,  -0.71563f, 0.22561f,  -1.19083f, -0.46149f,
  0.83015f,  0.06024f,  1.17180f,  0.65122f,
};

static float av1_tx_type_nn_8x8_ver_layer1_weights[64] = {
  -1.42711f, -0.21683f, 2.12061f,  0.20489f,  -0.50228f, -0.24770f, 0.23391f,
  1.03470f,  -0.44847f, -0.63225f, -0.21583f, -0.06467f, -0.21892f, -0.07786f,
  1.43322f,  0.00280f,  -1.53057f, -0.18912f, 1.95333f,  0.31151f,  -2.07601f,
  0.06776f,  0.25529f,  0.94800f,  -1.11453f, -0.20594f, -0.13281f, 0.01485f,
  0.17650f,  -0.07955f, 1.43734f,  -0.23193f, -2.06463f, -0.21238f, 2.13707f,
  0.30351f,  0.27594f,  -0.36245f, 0.19539f,  0.91045f,  -0.24068f, -0.37616f,
  0.88792f,  0.02947f,  -0.16903f, -0.04932f, 1.51293f,  -0.95967f, -1.62903f,
  0.05326f,  2.30703f,  0.64445f,  -1.09464f, -0.16623f, 1.00240f,  0.07548f,
  -0.50406f, 0.63854f,  1.02340f,  0.49833f,  0.13671f,  0.26722f,  2.09516f,
  -0.41305f,
};

static float av1_tx_type_nn_8x8_ver_layer1_bias[4] = {
  2.14067f,
  2.76699f,
  2.04233f,
  1.34803f,
};

static float av1_tx_type_nn_8x8_ver_layer0_out[16] = { 0 };
static float av1_tx_type_nn_8x8_ver_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_8x8_ver = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          8,                                      // num_inputs
          16,                                     // num_outputs
          av1_tx_type_nn_8x8_ver_layer0_weights,  // weights
          av1_tx_type_nn_8x8_ver_layer0_bias,     // bias
          RELU,                                   // activation
          av1_tx_type_nn_8x8_ver_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          16,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_8x8_ver_layer1_weights,
          av1_tx_type_nn_8x8_ver_layer1_bias,
          NONE,
          av1_tx_type_nn_8x8_ver_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                  // num_outputs
  av1_tx_type_nn_8x8_ver_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};
/******************************************************************************/

// Tx type model for 8x16 block.
static float av1_tx_type_nn_8x16_hor_layer0_weights[128] = {
  -1.61872f, -1.58520f, -1.41236f, -1.53255f, -1.59794f, -1.25769f, -1.90043f,
  0.73431f,  1.10135f,  0.47054f,  0.43230f,  -0.43009f, -0.09135f, -0.07289f,
  -0.38785f, 1.23775f,  -0.35312f, 0.73789f,  0.88864f,  0.75957f,  0.62579f,
  0.46974f,  0.21851f,  1.63821f,  -2.27289f, -0.68522f, -0.69814f, -0.84368f,
  -0.91320f, -0.63055f, -1.03296f, 0.55778f,  -0.00071f, 1.27539f,  1.60068f,
  1.40975f,  0.97372f,  0.92843f,  1.90853f,  0.12626f,  1.71953f,  1.41978f,
  -0.12234f, -1.27058f, 0.76207f,  0.02495f,  -0.67038f, -0.05255f, 1.72923f,
  1.47630f,  1.47058f,  1.47614f,  1.49354f,  1.66131f,  1.50801f,  0.17145f,
  -2.30947f, -2.10850f, -1.25636f, -0.24900f, 0.72602f,  1.26572f,  0.97865f,
  -0.65466f, 1.31129f,  0.26916f,  0.12139f,  -0.12761f, -0.39143f, -0.28134f,
  0.06584f,  2.24418f,  0.22516f,  0.05011f,  -0.01671f, -0.29476f, -0.40326f,
  0.21138f,  -0.11573f, -0.31154f, -0.36828f, 0.03694f,  -0.07172f, -0.63419f,
  -3.14351f, -1.23125f, 0.65311f,  -0.11406f, 1.97287f,  -0.10422f, 0.83896f,
  0.85033f,  0.49724f,  0.80482f,  0.51454f,  1.06447f,  0.76693f,  0.72599f,
  -0.78573f, -0.53950f, 0.40894f,  0.00086f,  0.10784f,  -0.70498f, 1.16395f,
  1.14597f,  1.13496f,  1.12177f,  1.02100f,  -1.37574f, -2.97144f, 0.33899f,
  0.42013f,  0.86327f,  2.31983f,  2.04008f,  0.95503f,  0.15081f,  0.11530f,
  -0.02574f, -4.77119f, 0.13257f,  -0.01704f, -0.23087f, -0.00825f, 0.07029f,
  -0.28136f, 0.42556f,
};

static float av1_tx_type_nn_8x16_hor_layer0_bias[16] = {
  0.93617f,  -0.24000f, -1.26821f, 0.78780f,  0.13690f, -0.21948f,
  -1.45162f, 0.44584f,  -1.92582f, -0.23169f, 0.56004f, -1.19937f,
  1.81560f,  -1.02643f, -0.81690f, 0.08302f,
};

static float av1_tx_type_nn_8x16_hor_layer1_weights[64] = {
  0.06696f,  -0.11538f, -1.42029f, 0.32965f,  0.81046f,  0.01146f,  1.20945f,
  -0.16899f, 0.53224f,  -0.40232f, 0.01786f,  -0.73242f, 1.29750f,  1.95185f,
  0.70143f,  1.43287f,  0.76220f,  0.79937f,  -1.79011f, -1.15178f, 0.42526f,
  -0.67519f, 0.77267f,  -0.30697f, 2.46004f,  -0.49828f, 0.02875f,  1.09972f,
  1.47662f,  0.61719f,  0.61417f,  -0.12363f, 2.53048f,  0.00418f,  -1.38964f,
  0.88117f,  0.39239f,  -0.19347f, -2.58600f, -0.33715f, 1.09323f,  -0.32127f,
  0.02456f,  -0.19125f, 1.12728f,  0.66502f,  0.34296f,  1.14897f,  0.29967f,
  1.19209f,  0.22108f,  -0.11975f, 1.49776f,  -1.34624f, -2.58478f, -1.34632f,
  1.53207f,  0.45634f,  -1.48476f, 0.17489f,  0.71790f,  -2.12086f, -1.21778f,
  -1.31243f,
};

static float av1_tx_type_nn_8x16_hor_layer1_bias[4] = {
  0.83359f,
  1.06875f,
  1.77645f,
  1.49570f,
};

static float av1_tx_type_nn_8x16_hor_layer0_out[16] = { 0 };
static float av1_tx_type_nn_8x16_hor_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_8x16_hor = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          8,                                       // num_inputs
          16,                                      // num_outputs
          av1_tx_type_nn_8x16_hor_layer0_weights,  // weights
          av1_tx_type_nn_8x16_hor_layer0_bias,     // bias
          RELU,                                    // activation
          av1_tx_type_nn_8x16_hor_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          16,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_8x16_hor_layer1_weights,
          av1_tx_type_nn_8x16_hor_layer1_bias,
          NONE,
          av1_tx_type_nn_8x16_hor_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                   // num_outputs
  av1_tx_type_nn_8x16_hor_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};

static float av1_tx_type_nn_8x16_ver_layer0_weights[128] = {
  0.32858f,  -1.28887f, 0.25632f,  -0.05262f, 2.69203f,  -0.07004f, 1.37337f,
  -0.05725f, -0.05659f, 0.05592f,  0.01039f,  -0.29343f, 1.58628f,  -0.30003f,
  -3.43118f, 0.00272f,  1.70928f,  -0.76348f, 0.05889f,  -0.03263f, -0.07724f,
  0.03523f,  -0.19890f, 1.18005f,  -0.03605f, -0.20530f, -4.00733f, 0.10210f,
  -0.05368f, -0.17650f, -0.15317f, 0.06499f,  0.56705f,  1.04341f,  0.62890f,
  0.73451f,  -0.22199f, 0.86659f,  0.78443f,  -0.61664f, -0.50606f, 0.30247f,
  0.14455f,  0.39276f,  0.49203f,  0.65019f,  0.12269f,  1.64080f,  1.68289f,
  1.42694f,  1.60825f,  1.58501f,  1.47252f,  1.62589f,  1.48218f,  0.17726f,
  -0.04884f, 0.35376f,  -0.04796f, 0.32589f,  0.35087f,  0.35258f,  -0.46103f,
  -0.31176f, -0.05203f, 0.07247f,  -0.26756f, 0.22019f,  0.03412f,  0.33773f,
  0.29811f,  -0.11140f, 0.12831f,  -0.44673f, -0.09858f, 0.07889f,  0.15137f,
  0.00347f,  -0.23394f, 0.08886f,  -0.31201f, -0.79912f, -0.51092f, 0.14123f,
  -1.09599f, -4.26020f, -0.68675f, -0.02842f, -1.54538f, -1.28977f, -1.30558f,
  -1.21074f, -1.37142f, -1.14743f, -1.85397f, 0.82985f,  -0.30681f, 0.04494f,
  -0.24023f, -4.18053f, -0.16096f, -0.55492f, -0.27882f, 0.05829f,  -0.41224f,
  -2.52088f, -0.56162f, -1.04547f, -1.70685f, -0.28842f, -1.43673f, -0.01468f,
  -3.20585f, -0.69120f, -0.43931f, -0.46270f, -0.65885f, -0.55884f, -0.75138f,
  0.36381f,  -5.70858f, -0.14548f, -0.15745f, -0.11812f, -0.07605f, -0.07693f,
  -0.12236f, 0.16075f,
};

static float av1_tx_type_nn_8x16_ver_layer0_bias[16] = {
  -0.35385f, 0.30491f,  -0.90011f, 0.42941f,  1.20928f, -0.88331f,
  -1.48818f, -0.34785f, -0.32668f, -0.22695f, 0.89188f, 0.65521f,
  0.57598f,  0.99819f,  0.75175f,  0.17044f,
};

static float av1_tx_type_nn_8x16_ver_layer1_weights[64] = {
  -0.62913f, -0.34304f, 0.42963f,  -0.17440f, -1.44092f, 0.69142f,  -1.36067f,
  0.52211f,  0.44658f,  -0.26501f, -0.41657f, 0.34428f,  -0.34390f, -0.58567f,
  -0.84097f, -1.96311f, -0.37215f, -0.22250f, -1.23811f, -0.07247f, -0.81731f,
  0.58755f,  -1.30559f, 0.39551f,  0.41743f,  -0.09940f, -0.33230f, 0.14458f,
  -0.25139f, -0.54517f, 0.13469f,  -0.38157f, -0.39109f, -0.18205f, 0.06834f,
  -0.08395f, -0.92187f, 0.56724f,  1.44381f,  0.53226f,  -0.22356f, 0.12285f,
  -0.29418f, -1.86749f, -0.22372f, -0.60204f, -0.87746f, -1.16936f, 0.56884f,
  0.62641f,  -0.11823f, 1.00395f,  1.64794f,  -0.64535f, 2.29322f,  -0.23397f,
  0.17251f,  -0.35927f, 0.65631f,  -0.26812f, 0.80128f,  0.85748f,  0.47404f,
  2.20547f,
};

static float av1_tx_type_nn_8x16_ver_layer1_bias[4] = {
  -0.44080f,
  -1.67455f,
  -1.46332f,
  -6.13206f,
};

static float av1_tx_type_nn_8x16_ver_layer0_out[16] = { 0 };
static float av1_tx_type_nn_8x16_ver_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_8x16_ver = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          8,                                       // num_inputs
          16,                                      // num_outputs
          av1_tx_type_nn_8x16_ver_layer0_weights,  // weights
          av1_tx_type_nn_8x16_ver_layer0_bias,     // bias
          RELU,                                    // activation
          av1_tx_type_nn_8x16_ver_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          16,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_8x16_ver_layer1_weights,
          av1_tx_type_nn_8x16_ver_layer1_bias,
          NONE,
          av1_tx_type_nn_8x16_ver_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                   // num_outputs
  av1_tx_type_nn_8x16_ver_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};
/******************************************************************************/

// Tx type model for 16x8 block.
static float av1_tx_type_nn_16x8_hor_layer0_weights[128] = {
  0.02600f,  0.09786f,  -1.05107f, -0.35594f, -0.15658f, 2.99828f,  -0.07106f,
  -0.10101f, -0.14412f, -0.83790f, -0.19434f, 2.28368f,  1.91727f,  -0.00956f,
  -0.90640f, 0.09174f,  1.58895f,  1.38945f,  1.49431f,  1.51381f,  1.44803f,
  1.53544f,  1.44694f,  0.17753f,  1.69735f,  -0.78652f, 0.31092f,  -0.23736f,
  0.02231f,  -0.09884f, -0.00493f, 1.21189f,  -1.94382f, -0.34629f, -0.58309f,
  0.72291f,  -0.30056f, 0.90660f,  -0.57495f, 3.07809f,  0.73644f,  1.43050f,
  1.34356f,  -0.66554f, 0.50102f,  -0.64305f, 0.42044f,  -1.66165f, -0.05733f,
  -2.51402f, -1.01067f, -0.33390f, -0.32986f, -0.92431f, 1.86281f,  -0.07290f,
  -0.26290f, -0.68941f, 1.81156f,  0.66125f,  -2.09974f, 0.17032f,  -0.67461f,
  -0.00876f, -1.50154f, 1.17153f,  1.00377f,  0.33022f,  0.74689f,  0.42878f,
  0.61725f,  -0.83967f, 0.09467f,  -0.39892f, 0.33863f,  0.10656f,  -0.09249f,
  -0.39757f, 0.48481f,  -0.35162f, 1.47014f,  1.67827f,  -1.84051f, 0.16291f,
  -0.50135f, -2.29911f, -0.42217f, -0.13358f, 1.45899f,  -0.14743f, -0.02763f,
  -0.28003f, -0.01364f, 0.21014f,  -0.29026f, -0.20198f, 1.38782f,  0.56731f,
  0.27489f,  0.43227f,  0.41326f,  0.42721f,  0.87720f,  -1.90067f, -5.04951f,
  -0.17638f, -0.58119f, -0.08954f, -0.13692f, -0.12325f, -0.38548f, 0.66462f,
  -1.42377f, -1.21917f, -1.38193f, -1.36539f, -1.39378f, -1.19629f, -1.59812f,
  0.28689f,  0.32394f,  0.52128f,  0.01013f,  -0.28948f, -0.26293f, -0.44331f,
  -0.36570f, -0.50757f,
};

static float av1_tx_type_nn_16x8_hor_layer0_bias[16] = {
  -0.08696f, -0.22110f, -1.43604f, -1.00451f, -1.51029f, 0.63736f,
  0.45260f,  0.16229f,  4.01393f,  -0.21748f, 0.36411f,  -0.08764f,
  -0.12329f, 0.08986f,  1.08117f,  -0.00220f,
};

static float av1_tx_type_nn_16x8_hor_layer1_weights[64] = {
  0.55824f,  -0.14648f, 0.81947f,  -0.45867f, -1.86078f, -0.17291f, 0.34849f,
  0.15153f,  1.75625f,  -0.25760f, 0.72015f,  -0.30059f, -0.57975f, 0.07609f,
  -0.02036f, 0.07912f,  0.57080f,  -0.13792f, 0.74184f,  -0.87669f, -1.87572f,
  -0.27270f, 0.39751f,  0.19652f,  2.03514f,  -0.32944f, 0.76251f,  0.04399f,
  -0.63175f, 0.37420f,  0.08309f,  0.04466f,  0.60255f,  -0.12820f, 1.66065f,
  -0.59496f, -1.94794f, -0.14847f, 0.39424f,  0.16273f,  1.80587f,  0.41197f,
  0.74691f,  -0.21217f, -0.63173f, 0.09510f,  -0.35538f, -0.04407f, 0.92847f,
  0.20141f,  1.68680f,  -0.56528f, -2.26960f, 0.12978f,  0.73748f,  0.42438f,
  2.00673f,  -0.40189f, 0.95423f,  0.23234f,  -0.80953f, 0.65814f,  0.49444f,
  -0.23347f,
};

static float av1_tx_type_nn_16x8_hor_layer1_bias[4] = {
  3.57175f,
  2.42612f,
  3.31259f,
  2.08287f,
};

static float av1_tx_type_nn_16x8_hor_layer0_out[16] = { 0 };
static float av1_tx_type_nn_16x8_hor_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_16x8_hor = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          8,                                       // num_inputs
          16,                                      // num_outputs
          av1_tx_type_nn_16x8_hor_layer0_weights,  // weights
          av1_tx_type_nn_16x8_hor_layer0_bias,     // bias
          RELU,                                    // activation
          av1_tx_type_nn_16x8_hor_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          16,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_16x8_hor_layer1_weights,
          av1_tx_type_nn_16x8_hor_layer1_bias,
          NONE,
          av1_tx_type_nn_16x8_hor_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                   // num_outputs
  av1_tx_type_nn_16x8_hor_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};

static float av1_tx_type_nn_16x8_ver_layer0_weights[128] = {
  0.46633f,  1.55328f,  -0.11230f, -0.29571f, 0.18814f,  -1.52430f, -2.34660f,
  0.08644f,  -1.97718f, -1.29140f, -1.12262f, -1.12985f, -1.25911f, -0.96506f,
  -1.57129f, 0.96021f,  1.34192f,  1.28623f,  1.21655f,  1.28758f,  1.25482f,
  1.30195f,  1.19190f,  0.09310f,  0.52072f,  0.91487f,  1.24100f,  1.61236f,
  1.72166f,  2.20750f,  1.62379f,  -1.43936f, 0.50665f,  0.40213f,  0.66502f,
  -1.66699f, -3.07618f, 0.05877f,  0.60987f,  -0.09995f, -0.10916f, 0.48049f,
  0.23812f,  0.39847f,  -0.21682f, -0.63455f, 0.33453f,  -0.67939f, -4.14355f,
  -0.62756f, -0.22502f, -0.17215f, 0.01062f,  0.27049f,  -0.10748f, 0.30945f,
  2.72445f,  -0.89181f, -0.06800f, 0.20595f,  -0.73385f, 0.04071f,  -1.30294f,
  1.83507f,  0.92570f,  0.69609f,  0.76285f,  0.69892f,  0.76409f,  0.63104f,
  0.73397f,  1.09575f,  -0.20129f, -0.24022f, -0.24599f, -0.59107f, -0.88755f,
  -0.68987f, -0.75495f, -1.31002f, -1.30237f, -0.94093f, -2.15678f, -1.49303f,
  -1.17498f, -1.39952f, -0.91270f, -0.05587f, 1.02381f,  -0.75580f, -0.65263f,
  -0.78996f, -0.71075f, -0.71018f, -0.70350f, -1.26196f, 2.34208f,  -0.53611f,
  0.19752f,  -0.16842f, -0.24828f, 0.21857f,  0.08222f,  -2.55894f, -1.75702f,
  0.11394f,  1.03083f,  0.79972f,  -1.54112f, -1.82341f, -0.57597f, -0.02077f,
  -0.39616f, -0.00995f, -0.12809f, 0.01188f,  -0.25117f, 0.09202f,  0.09336f,
  -0.05614f, -0.30039f, 0.25834f,  1.19944f,  1.22533f,  0.92330f,  0.75967f,
  -0.81945f, -0.41647f,
};

static float av1_tx_type_nn_16x8_ver_layer0_bias[16] = {
  0.17841f,  0.67315f,  -1.24450f, 3.13859f,  0.16203f, -0.14992f,
  0.29553f,  -1.15567f, -0.71421f, 1.15977f,  1.14585f, 3.02460f,
  -0.04510f, 0.48000f,  -0.09354f, -0.42422f,
};

static float av1_tx_type_nn_16x8_ver_layer1_weights[64] = {
  0.29912f,  -0.10009f, -1.11478f, 1.76812f,  -0.27719f, 0.52148f,  0.17622f,
  -1.17116f, 0.73397f,  -0.69279f, -0.11080f, 1.53751f,  -1.42003f, 0.14731f,
  0.13592f,  -0.04883f, 0.39186f,  -0.13655f, -0.43994f, 1.82759f,  -0.25601f,
  -0.15018f, 0.51920f,  -1.56070f, 0.31683f,  -0.79367f, -0.02904f, 1.28637f,
  -1.15203f, 0.26627f,  0.42828f,  -0.24258f, 0.38647f,  -0.83352f, 0.32553f,
  2.09522f,  -0.26822f, -0.42191f, 0.32825f,  -1.30748f, 1.50551f,  -0.52669f,
  0.20045f,  1.69318f,  -1.47839f, 0.30802f,  -0.07290f, -0.28106f, 0.68192f,
  -0.15522f, 1.12579f,  2.21921f,  0.09720f,  -0.50265f, 0.83165f,  -1.31721f,
  0.72422f,  -1.24952f, 0.61653f,  2.04117f,  -1.42406f, 0.52568f,  -0.46180f,
  -0.00873f,
};

static float av1_tx_type_nn_16x8_ver_layer1_bias[4] = {
  3.34981f,
  3.74710f,
  1.38339f,
  0.45176f,
};

static float av1_tx_type_nn_16x8_ver_layer0_out[16] = { 0 };
static float av1_tx_type_nn_16x8_ver_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_16x8_ver = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          8,                                       // num_inputs
          16,                                      // num_outputs
          av1_tx_type_nn_16x8_ver_layer0_weights,  // weights
          av1_tx_type_nn_16x8_ver_layer0_bias,     // bias
          RELU,                                    // activation
          av1_tx_type_nn_16x8_ver_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          16,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_16x8_ver_layer1_weights,
          av1_tx_type_nn_16x8_ver_layer1_bias,
          NONE,
          av1_tx_type_nn_16x8_ver_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                   // num_outputs
  av1_tx_type_nn_16x8_ver_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};
/******************************************************************************/

// Tx type model for 16x16 block.
static float av1_tx_type_nn_16x16_layer0_weights[128] = {
  1.26592f,  1.36313f,  1.30956f,  1.29926f,  1.48816f,  1.68851f,  1.32000f,
  0.13321f,  -0.22477f, -0.88906f, -0.19622f, 1.69605f,  1.22180f,  -1.57771f,
  -1.15765f, 0.05710f,  -1.13355f, -0.85486f, -0.99971f, -0.91571f, -1.06031f,
  -0.77952f, -1.15723f, 1.17809f,  1.35602f,  -0.05243f, -0.37596f, 0.26108f,
  0.17611f,  -0.10323f, 0.77279f,  -0.48911f, -0.79308f, 0.55112f,  0.43918f,
  0.27872f,  0.28714f,  0.45830f,  1.05689f,  0.03705f,  -2.49975f, -0.01940f,
  0.05709f,  0.07942f,  -0.13290f, -0.10359f, 0.00143f,  0.37303f,  0.96470f,
  0.53293f,  1.14459f,  0.89185f,  0.43378f,  0.47764f,  0.90924f,  0.15279f,
  -0.15361f, 0.02949f,  0.42240f,  0.68143f,  0.89588f,  0.73754f,  0.10974f,
  1.57755f,  -0.39870f, -0.32914f, 0.35638f,  0.34991f,  -0.00003f, -0.23373f,
  0.29630f,  -0.76699f, -0.01356f, 0.04234f,  0.84253f,  1.92078f,  0.93160f,
  0.71993f,  0.71604f,  0.76455f,  -1.59782f, 0.32332f,  1.11628f,  0.33062f,
  -0.03728f, -0.05710f, 0.80447f,  -0.14719f, 1.34658f,  -0.05718f, 0.64015f,
  0.21926f,  0.41653f,  0.12720f,  0.54092f,  1.39411f,  1.81819f,  -0.24513f,
  0.00955f,  0.38011f,  -0.57787f, -0.41759f, 0.68834f,  -0.31783f, -0.40607f,
  -0.10107f, -0.79374f, 0.75599f,  -0.16282f, -0.14490f, -0.20783f, -0.55019f,
  -0.13793f, -0.22293f, 0.18305f,  0.12445f,  0.56830f,  0.24567f,  0.09278f,
  0.70803f,  0.35803f,  -1.52676f, -0.89624f, 0.77665f,  0.19877f,  0.77175f,
  0.50355f,  0.08592f,
};

static float av1_tx_type_nn_16x16_layer0_bias[16] = {
  -1.31834f, 0.14346f,  -0.10062f, 0.84489f,  0.95617f,  -0.06720f,
  -0.68502f, -0.91442f, -0.31932f, 0.25276f,  -0.15138f, -1.57661f,
  -0.14062f, -0.42120f, 0.94573f,  -0.09287f,
};

static float av1_tx_type_nn_16x16_layer1_weights[64] = {
  -1.80333f, -1.06353f, 0.55139f,  0.74644f,  0.13747f, -0.93018f, -0.10286f,
  0.67133f,  0.24460f,  1.44583f,  0.02173f,  0.26037f, -0.73687f, 0.19566f,
  0.61846f,  -0.58601f, -1.03196f, -0.74415f, 0.30041f, -0.41967f, 1.08740f,
  0.96224f,  -0.59139f, 0.03813f,  0.05403f,  1.33427f, -0.54375f, -1.92181f,
  0.54704f,  0.13608f,  0.22151f,  -0.38076f, 1.18390f, -0.77508f, -1.84283f,
  1.00894f,  0.62318f,  -0.15296f, 1.27600f,  0.22822f, 0.12751f,  0.93910f,
  -0.28502f, 0.53912f,  -0.96889f, 0.10182f,  0.81508f, -0.43028f, 2.67386f,
  0.52204f,  0.49820f,  -0.41711f, 1.05038f,  1.12192f, 0.74349f,  -0.75417f,
  -0.03718f, -0.35769f, 0.89651f,  0.63236f,  0.54215f, -0.07894f, 0.48274f,
  1.08829f,
};

static float av1_tx_type_nn_16x16_layer1_bias[4] = {
  0.81986f,
  1.26865f,
  0.11118f,
  2.48404f,
};

static float av1_tx_type_nn_16x16_layer0_out[16] = { 0 };
static float av1_tx_type_nn_16x16_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_16x16 = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          8,                                    // num_inputs
          16,                                   // num_outputs
          av1_tx_type_nn_16x16_layer0_weights,  // weights
          av1_tx_type_nn_16x16_layer0_bias,     // bias
          RELU,                                 // activation
          av1_tx_type_nn_16x16_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          16,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_16x16_layer1_weights,
          av1_tx_type_nn_16x16_layer1_bias,
          NONE,
          av1_tx_type_nn_16x16_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                // num_outputs
  av1_tx_type_nn_16x16_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};
/******************************************************************************/

// Tx type model for 4x16 block.
static float av1_tx_type_nn_4x16_hor_layer0_weights[32] = {
  0.36539f,  0.25667f,  0.01491f,  -0.21959f, 2.55105f,  0.17615f, 1.79884f,
  1.65936f,  -0.44363f, 0.00706f,  -0.68004f, -0.64360f, 1.75760f, 1.91906f,
  1.47682f,  0.09650f,  -3.59244f, -0.35004f, 0.93295f,  0.25806f, -0.08154f,
  0.79332f,  0.79535f,  1.09467f,  1.57855f,  -0.51359f, 0.90553f, -1.67744f,
  -1.74563f, -0.88830f, -1.77603f, 2.15935f,
};

static float av1_tx_type_nn_4x16_hor_layer0_bias[8] = {
  -0.36435f, -2.22731f, -0.00837f, -1.34546f,
  0.62806f,  -0.20675f, 4.91940f,  -0.56079f,
};

static float av1_tx_type_nn_4x16_hor_layer1_weights[32] = {
  -0.57191f, -1.46418f, 0.67331f,  -1.15027f, 0.46288f,  0.81251f,  2.51768f,
  -0.27147f, 0.00761f,  -2.15214f, -0.69650f, -0.50808f, 0.92832f,  0.45668f,
  2.34201f,  -0.52941f, 0.51008f,  -1.55496f, -0.01371f, -0.12356f, 0.66624f,
  0.88043f,  2.64862f,  -1.28024f, -0.17578f, -1.80034f, -0.32217f, 0.89519f,
  1.28413f,  -0.30326f, 2.45329f,  -0.83335f,
};

static float av1_tx_type_nn_4x16_hor_layer1_bias[4] = {
  2.33198f,
  3.36245f,
  1.62603f,
  2.91056f,
};

static float av1_tx_type_nn_4x16_hor_layer0_out[8] = { 0 };
static float av1_tx_type_nn_4x16_hor_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_4x16_hor = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          4,                                       // num_inputs
          8,                                       // num_outputs
          av1_tx_type_nn_4x16_hor_layer0_weights,  // weights
          av1_tx_type_nn_4x16_hor_layer0_bias,     // bias
          RELU,                                    // activation
          av1_tx_type_nn_4x16_hor_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          8,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_4x16_hor_layer1_weights,
          av1_tx_type_nn_4x16_hor_layer1_bias,
          NONE,
          av1_tx_type_nn_4x16_hor_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                   // num_outputs
  av1_tx_type_nn_4x16_hor_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};

static float av1_tx_type_nn_4x16_ver_layer0_weights[128] = {
  1.61392f,  1.41239f,  1.47646f,  1.47325f,  1.46110f,  1.49208f,  1.49414f,
  0.12835f,  -0.76986f, 0.07087f,  -0.24572f, -0.93168f, 3.07935f,  -0.18183f,
  -0.09831f, -0.07703f, -0.03222f, -0.25473f, -0.06090f, 2.93713f,  -0.38711f,
  -0.12884f, -0.18329f, -0.06262f, -0.00327f, -0.02930f, -0.01641f, -0.00622f,
  -0.03305f, -4.07069f, -2.76643f, 0.04413f,  -1.03176f, -0.19217f, -0.44980f,
  -2.48615f, -2.58112f, -0.87695f, 0.16187f,  -0.04891f, -0.06854f, 1.08104f,
  0.75245f,  1.49302f,  0.63363f,  1.45715f,  0.92574f,  1.72029f,  0.33326f,
  3.86646f,  0.04422f,  0.41019f,  0.36212f,  0.56600f,  -1.01552f, 0.05128f,
  0.40454f,  -1.05100f, -0.47461f, -1.33168f, -0.46145f, -1.36870f, -0.88838f,
  -1.05358f, -0.18537f, -0.34357f, -0.03698f, 0.68905f,  0.41010f,  0.31223f,
  -0.43382f, -0.74715f, 2.03366f,  -0.30419f, 0.45747f,  0.09526f,  0.31678f,
  0.22915f,  0.21832f,  1.26385f,  -0.06814f, -0.71417f, -1.18947f, 0.03762f,
  0.10936f,  2.97396f,  -0.42638f, -0.03123f, -5.49756f, -0.17029f, -0.11323f,
  0.05173f,  -0.44274f, -0.15738f, 0.11311f,  0.43872f,  0.16837f,  -0.52849f,
  2.90050f,  -0.54735f, -0.29591f, 1.24030f,  0.21696f,  -0.04443f, -1.60877f,
  -1.36365f, -1.27432f, -1.52060f, -1.34397f, -1.13371f, -1.87554f, 0.80123f,
  0.42820f,  -0.14157f, -2.73963f, -0.68040f, -0.35236f, 0.14490f,  2.23477f,
  0.01370f,  -0.20426f, -1.51411f, -0.72293f, 0.64516f,  0.97638f,  0.32616f,
  -0.27975f, -0.01149f,
};

static float av1_tx_type_nn_4x16_ver_layer0_bias[16] = {
  -1.37863f, -0.05763f, -0.07041f, 0.15306f,  0.96026f,  -1.42105f,
  -0.55822f, 1.04845f,  -0.17662f, -1.25345f, -0.11927f, 0.49845f,
  -0.32530f, 0.73483f,  0.08322f,  -0.23890f,
};

static float av1_tx_type_nn_4x16_ver_layer1_weights[64] = {
  0.27194f,  0.50607f,  0.49229f,  -0.48192f, 0.15667f,  -1.38891f, 0.38102f,
  -0.58825f, -0.07337f, -0.52909f, 0.36975f,  0.28710f,  0.34992f,  -0.73630f,
  0.30386f,  -0.58822f, 0.36127f,  0.57950f,  0.55878f,  -0.42796f, 0.19967f,
  -1.45517f, 0.42529f,  -0.54630f, -0.38169f, -0.84899f, 0.41622f,  0.46935f,
  0.39077f,  -0.75448f, 0.31698f,  -0.76187f, 0.97765f,  0.57052f,  0.55825f,
  -0.54273f, 0.20466f,  -1.46347f, 0.41813f,  -0.55019f, -0.19948f, -0.57982f,
  0.41206f,  0.32373f,  0.38537f,  -1.11657f, 0.32887f,  -0.76911f, 1.12259f,
  0.72163f,  0.82603f,  0.37786f,  0.34976f,  -1.86642f, 0.59961f,  -0.16329f,
  -0.36631f, -0.56814f, 0.60410f,  0.53158f,  0.56389f,  -0.70508f, 0.51009f,
  -0.56513f,
};

static float av1_tx_type_nn_4x16_ver_layer1_bias[4] = {
  4.60896f,
  4.53551f,
  4.53124f,
  4.27435f,
};

static float av1_tx_type_nn_4x16_ver_layer0_out[16] = { 0 };
static float av1_tx_type_nn_4x16_ver_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_4x16_ver = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          8,                                       // num_inputs
          16,                                      // num_outputs
          av1_tx_type_nn_4x16_ver_layer0_weights,  // weights
          av1_tx_type_nn_4x16_ver_layer0_bias,     // bias
          RELU,                                    // activation
          av1_tx_type_nn_4x16_ver_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          16,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_4x16_ver_layer1_weights,
          av1_tx_type_nn_4x16_ver_layer1_bias,
          NONE,
          av1_tx_type_nn_4x16_ver_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                   // num_outputs
  av1_tx_type_nn_4x16_ver_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};
/******************************************************************************/

// Tx type model for 16x4 block.
static float av1_tx_type_nn_16x4_hor_layer0_weights[128] = {
  1.45347f,  -0.15743f, 0.44236f,  0.25808f,  0.33944f,  0.38678f,  0.24428f,
  1.67287f,  0.09539f,  -0.42940f, -0.31507f, -0.00154f, -2.98755f, -2.27744f,
  -0.49183f, 0.09333f,  -0.99026f, -0.22157f, 0.53701f,  0.60447f,  0.15686f,
  -0.04646f, 0.26341f,  2.12361f,  0.27090f,  -1.14716f, -0.64146f, -0.91604f,
  -0.75335f, -0.60056f, -1.25084f, 1.68473f,  -3.24075f, -4.03867f, -2.07877f,
  -0.02347f, 0.00333f,  -0.01259f, -0.00465f, 0.02526f,  0.36286f,  -0.10324f,
  2.12780f,  -0.74584f, -1.05052f, 1.78467f,  -0.55065f, -0.03326f, 2.46781f,
  1.18349f,  0.96015f,  1.01696f,  1.10584f,  1.07263f,  1.11531f,  -1.06413f,
  0.32389f,  -1.87360f, -0.14435f, 1.77926f,  1.09966f,  -0.12680f, -0.61386f,
  -0.09724f, -0.33095f, 1.12122f,  1.00791f,  1.52416f,  1.35004f,  1.32657f,
  0.60950f,  -1.13538f, -0.38654f, 0.06473f,  2.10669f,  0.27734f,  -0.38359f,
  -1.91455f, -1.22676f, 0.05786f,  0.97432f,  2.19967f,  0.50457f,  0.78976f,
  0.95183f,  -0.32414f, 0.49437f,  -0.04506f, 0.18993f,  -0.07971f, 0.23889f,
  -0.09872f, -0.66036f, 0.05377f,  2.69638f,  -0.08259f, -0.69210f, -1.08296f,
  -1.96504f, -2.31947f, -0.80161f, -0.80456f, -1.35556f, -0.05323f, -4.42658f,
  -0.30732f, -0.12043f, 0.11126f,  0.10771f,  -0.14956f, -0.02218f, 0.41016f,
  1.16599f,  1.14629f,  1.12881f,  1.18676f,  1.24677f,  1.28695f,  1.11270f,
  0.08233f,  1.75440f,  0.49228f,  -0.34858f, -0.17032f, 0.29288f,  0.47175f,
  0.19055f,  -1.56413f,
};

static float av1_tx_type_nn_16x4_hor_layer0_bias[16] = {
  -1.71227f, 0.47291f, -0.97536f, -0.66216f, 0.11729f,  -0.21451f,
  2.75281f,  0.04318f, 2.03965f,  0.14618f,  -0.70483f, -0.24517f,
  1.14048f,  0.33308f, -1.10886f, 0.41184f,
};

static float av1_tx_type_nn_16x4_hor_layer1_weights[64] = {
  -1.17079f, 0.19096f,  -1.05753f, -0.30803f, -1.21680f, -0.67255f, 1.60115f,
  0.05972f,  1.44759f,  -0.04068f, -0.26331f, 0.31400f,  0.96923f,  0.33443f,
  -0.77215f, -0.91316f, -1.78928f, 0.21483f,  -1.24008f, -0.46190f, -0.12127f,
  -0.62144f, 1.37593f,  0.08373f,  1.56215f,  0.00279f,  -0.14556f, 0.38710f,
  0.96228f,  0.66433f,  -0.51798f, -0.80738f, -0.18539f, 0.19377f,  -1.03090f,
  -1.51044f, -0.59485f, -0.62589f, 1.90742f,  0.09078f,  1.49113f,  0.00205f,
  -0.15918f, 0.40827f,  1.08553f,  0.43431f,  0.33519f,  -1.12669f, -1.10274f,
  0.80004f,  -1.83599f, -0.53134f, 2.00515f,  -0.32670f, 1.37124f,  0.51136f,
  1.62563f,  0.24787f,  0.31757f,  0.81751f,  1.57262f,  0.83214f,  1.04661f,
  -0.43819f,
};

static float av1_tx_type_nn_16x4_hor_layer1_bias[4] = {
  2.32575f,
  2.75703f,
  1.12304f,
  2.15567f,
};

static float av1_tx_type_nn_16x4_hor_layer0_out[16] = { 0 };
static float av1_tx_type_nn_16x4_hor_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_16x4_hor = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          8,                                       // num_inputs
          16,                                      // num_outputs
          av1_tx_type_nn_16x4_hor_layer0_weights,  // weights
          av1_tx_type_nn_16x4_hor_layer0_bias,     // bias
          RELU,                                    // activation
          av1_tx_type_nn_16x4_hor_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          16,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_16x4_hor_layer1_weights,
          av1_tx_type_nn_16x4_hor_layer1_bias,
          NONE,
          av1_tx_type_nn_16x4_hor_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                   // num_outputs
  av1_tx_type_nn_16x4_hor_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};

static float av1_tx_type_nn_16x4_ver_layer0_weights[32] = {
  0.26047f,  0.99930f,  1.16484f,  -0.28196f, -2.67483f, -0.21456f, -0.16854f,
  0.46375f,  1.47951f,  1.13735f,  1.12356f,  0.27385f,  0.50978f,  2.09967f,
  -1.47386f, 0.01950f,  -0.06362f, 0.26014f,  1.04544f,  -0.03099f, 0.07478f,
  -0.39701f, 0.05545f,  2.73633f,  -0.56305f, -0.02208f, -0.44517f, -0.00897f,
  -0.17967f, -0.96622f, 0.42635f,  -1.04784f,
};

static float av1_tx_type_nn_16x4_ver_layer0_bias[8] = {
  -0.52088f, 0.52844f,  -1.03655f, -0.30974f,
  2.59952f,  -1.93604f, 0.00000f,  2.51787f,
};

static float av1_tx_type_nn_16x4_ver_layer1_weights[32] = {
  0.10916f,  -0.21219f, -0.51340f, 0.69161f,  1.45988f,  -1.36942f, -0.40899f,
  1.05136f,  -0.08486f, 0.10008f,  -0.55304f, 0.88012f,  1.61177f,  -1.64507f,
  0.63428f,  1.15130f,  -0.17287f, -0.18592f, -0.01143f, 0.88293f,  1.73326f,
  -1.63624f, 0.09359f,  1.18393f,  0.26531f,  0.22378f,  0.15170f,  1.06965f,
  1.26814f,  -1.93873f, -0.00768f, 1.58309f,
};

static float av1_tx_type_nn_16x4_ver_layer1_bias[4] = {
  2.34713f,
  1.68667f,
  1.25488f,
  1.69812f,
};

static float av1_tx_type_nn_16x4_ver_layer0_out[8] = { 0 };
static float av1_tx_type_nn_16x4_ver_layer1_out[4] = { 0 };

static NN_CONFIG_V2 av1_tx_type_nnconfig_16x4_ver = {
  1,  // num_hidden_layers
  {
      // fc layer setting
      {
          // layer 0
          4,                                       // num_inputs
          8,                                       // num_outputs
          av1_tx_type_nn_16x4_ver_layer0_weights,  // weights
          av1_tx_type_nn_16x4_ver_layer0_bias,     // bias
          RELU,                                    // activation
          av1_tx_type_nn_16x4_ver_layer0_out,      // output
          NULL,
          NULL,
          NULL,
      },
      {
          8,  // num_inputs (!!same as num_outputs of last layer)
          4,
          av1_tx_type_nn_16x4_ver_layer1_weights,
          av1_tx_type_nn_16x4_ver_layer1_bias,
          NONE,
          av1_tx_type_nn_16x4_ver_layer1_out,
          NULL,
          NULL,
          NULL,
      },
  },
  4,                                   // num_outputs
  av1_tx_type_nn_16x4_ver_layer1_out,  // logits (!!same as last layer output)
  SOFTMAX_CROSS_ENTROPY,
};
/******************************************************************************/

// Map tx_size to its corresponding neural net model for tx type prediction.
static NN_CONFIG_V2 *av1_tx_type_nnconfig_map_hor[] = {
  &av1_tx_type_nnconfig_4x4_hor,   // 4x4 transform
  &av1_tx_type_nnconfig_8x8_hor,   // 8x8 transform
  &av1_tx_type_nnconfig_16x16,     // 16x16 transform
  NULL,                            // 32x32 transform
  NULL,                            // 64x64 transform
  &av1_tx_type_nnconfig_4x8_hor,   // 4x8 transform
  &av1_tx_type_nnconfig_8x4_hor,   // 8x4 transform
  &av1_tx_type_nnconfig_8x16_hor,  // 8x16 transform
  &av1_tx_type_nnconfig_16x8_hor,  // 16x8 transform
  NULL,                            // 16x32 transform
  NULL,                            // 32x16 transform
  NULL,                            // 32x64 transform
  NULL,                            // 64x32 transform
  &av1_tx_type_nnconfig_4x16_hor,  // 4x16 transform
  &av1_tx_type_nnconfig_16x4_hor,  // 16x4 transform
  NULL,                            // 8x32 transform
  NULL,                            // 32x8 transform
  NULL,                            // 16x64 transform
  NULL,                            // 64x16 transform
};

static NN_CONFIG_V2 *av1_tx_type_nnconfig_map_ver[] = {
  &av1_tx_type_nnconfig_4x4_ver,   // 4x4 transform
  &av1_tx_type_nnconfig_8x8_ver,   // 8x8 transform
  &av1_tx_type_nnconfig_16x16,     // 16x16 transform
  NULL,                            // 32x32 transform
  NULL,                            // 64x64 transform
  &av1_tx_type_nnconfig_4x8_ver,   // 4x8 transform
  &av1_tx_type_nnconfig_8x4_ver,   // 8x4 transform
  &av1_tx_type_nnconfig_8x16_ver,  // 8x16 transform
  &av1_tx_type_nnconfig_16x8_ver,  // 16x8 transform
  NULL,                            // 16x32 transform
  NULL,                            // 32x16 transform
  NULL,                            // 32x64 transform
  NULL,                            // 64x32 transform
  &av1_tx_type_nnconfig_4x16_ver,  // 4x16 transform
  &av1_tx_type_nnconfig_16x4_ver,  // 16x4 transform
  NULL,                            // 8x32 transform
  NULL,                            // 32x8 transform
  NULL,                            // 16x64 transform
  NULL,                            // 64x16 transform
};
#else
/******************************CONFIG_NN***************************************/
// Tx type model for 4x4 block.
static const float av1_tx_type_nn_weights_4x4_hor_layer0[32] = {
  -1.64947f, -1.54497f, -1.62832f, -0.17774f, -2.89498f, -0.72498f, 0.72036f,
  0.17996f,  1.20000f,  -0.27654f, 0.77396f,  1.21684f,  -1.75909f, -0.51272f,
  -1.25923f, 0.35005f,  -0.04257f, -0.23389f, -0.41841f, -0.08229f, 0.09503f,
  2.73144f,  -0.16875f, -0.23482f, 0.02194f,  -0.26427f, 0.28049f,  0.21260f,
  1.35792f,  0.27733f,  0.88660f,  -0.68304f,
};

static const float av1_tx_type_nn_bias_4x4_hor_layer0[8] = {
  1.38742f, 0.59540f,  -1.37622f, 1.92114f,
  0.00000f, -0.38998f, -0.32726f, -0.15650f,
};

static const float av1_tx_type_nn_weights_4x4_hor_layer1[32] = {
  1.65254f,  1.00915f,  -0.89318f, -2.05142f, -0.23235f, 0.96781f,  -0.37145f,
  -0.21056f, 1.13891f,  0.38675f,  0.87739f,  -1.42697f, 0.48015f,  0.61883f,
  -0.03979f, 0.11487f,  0.48042f,  0.45200f,  -0.23242f, 0.75166f,  0.55458f,
  0.39452f,  -0.35285f, 1.59120f,  -1.49221f, -0.48349f, -0.64692f, 1.49297f,
  -0.26782f, -0.65416f, -0.10648f, 0.05568f,
};

static const float av1_tx_type_nn_bias_4x4_hor_layer1[4] = {
  4.07177f,
  3.26961f,
  0.58083f,
  1.21199f,
};

static const NN_CONFIG av1_tx_type_nnconfig_4x4_hor = {
  4,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      8,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_4x4_hor_layer0,
    av1_tx_type_nn_weights_4x4_hor_layer1 },
  { av1_tx_type_nn_bias_4x4_hor_layer0, av1_tx_type_nn_bias_4x4_hor_layer1 }
};

static const float av1_tx_type_nn_weights_4x4_ver_layer0[32] = {
  -0.02032f, 2.61610f,  0.02098f,  -0.30217f, 0.12637f,  0.11017f,  -3.01996f,
  0.35144f,  1.93776f,  -0.20463f, 1.64102f,  -1.41986f, -3.66717f, -0.51655f,
  0.43910f,  0.37778f,  -1.02634f, 0.85337f,  -0.69753f, 1.00206f,  2.11784f,
  1.89427f,  1.92919f,  0.43201f,  -1.67358f, -1.67035f, -1.54623f, 0.16714f,
  -0.06589f, -0.28142f, -0.33118f, 1.72227f,
};

static const float av1_tx_type_nn_bias_4x4_ver_layer0[8] = {
  -0.33685f, 0.22025f,  0.28140f, 0.56138f,
  0.93489f,  -1.77048f, 1.34989f, -0.93747f,
};

static const float av1_tx_type_nn_weights_4x4_ver_layer1[32] = {
  -1.39506f, -1.06271f, -1.10886f, -1.69719f, 0.19699f,  -2.39850f, -1.26457f,
  0.75328f,  -1.26005f, -0.82738f, -0.12015f, -1.02702f, 1.40828f,  -2.37739f,
  -0.65639f, -0.71992f, -0.90453f, -1.12510f, -2.41362f, -1.16061f, -1.85577f,
  -0.99165f, -1.91366f, 0.16785f,  0.34776f,  0.58154f,  -0.18217f, -0.29257f,
  -0.86315f, -0.53336f, 0.30320f,  -1.32331f,
};

static const float av1_tx_type_nn_bias_4x4_ver_layer1[4] = {
  -1.31519f,
  -3.26321f,
  1.71794f,
  -1.90778f,
};

static const NN_CONFIG av1_tx_type_nnconfig_4x4_ver = {
  4,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      8,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_4x4_ver_layer0,
    av1_tx_type_nn_weights_4x4_ver_layer1 },
  { av1_tx_type_nn_bias_4x4_ver_layer0, av1_tx_type_nn_bias_4x4_ver_layer1 }
};
/******************************************************************************/

// Tx type model for 4x8 block.
static const float av1_tx_type_nn_weights_4x8_hor_layer0[32] = {
  0.00218f,  -0.41880f, -0.61215f, -0.92588f, 0.54291f,  -0.10898f, 0.70691f,
  0.46819f,  -1.61598f, -0.08834f, -0.96839f, 1.18489f,  -0.45171f, -0.65445f,
  -0.32179f, -0.10399f, 1.04379f,  0.91895f,  0.85589f,  0.08267f,  1.35388f,
  -2.03096f, 0.08168f,  -0.06372f, -0.26732f, -0.48262f, -0.08682f, 2.44071f,
  -1.35896f, -1.17121f, 1.68866f,  0.10357f,
};

static const float av1_tx_type_nn_bias_4x8_hor_layer0[8] = {
  2.93391f,  0.66831f, -0.21419f, 0.00000f,
  -0.72878f, 0.15127f, -1.46755f, 0.16658f,
};

static const float av1_tx_type_nn_weights_4x8_hor_layer1[32] = {
  -1.52077f, -1.06243f, 0.35319f,  -0.49207f, 0.54524f,  0.44271f, 1.37117f,
  -0.38957f, -1.28889f, -0.57133f, 0.04658f,  0.62278f,  0.37984f, 0.33247f,
  1.65547f,  -0.56806f, -1.38645f, -0.76258f, 0.67926f,  0.08783f, -0.01443f,
  0.34950f,  1.45812f,  -0.51332f, -1.41331f, -0.16453f, 0.05755f, 0.31405f,
  -0.50191f, 0.18219f,  1.83664f,  -0.75276f,
};

static const float av1_tx_type_nn_bias_4x8_hor_layer1[4] = {
  -1.17455f,
  -2.26089f,
  -1.79863f,
  -2.26333f,
};

static const NN_CONFIG av1_tx_type_nnconfig_4x8_hor = {
  4,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      8,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_4x8_hor_layer0,
    av1_tx_type_nn_weights_4x8_hor_layer1 },
  { av1_tx_type_nn_bias_4x8_hor_layer0, av1_tx_type_nn_bias_4x8_hor_layer1 }
};

static const float av1_tx_type_nn_weights_4x8_ver_layer0[128] = {
  -0.00952f, -0.98858f, -0.93181f, 1.39594f,  0.96559f,  0.18162f,  -0.76064f,
  -0.06066f, 0.07907f,  -0.09365f, -0.21313f, -0.02187f, -2.61707f, -2.68702f,
  -0.10982f, 0.18559f,  1.17049f,  1.11387f,  1.12697f,  1.05804f,  1.12764f,
  1.06318f,  1.12052f,  0.17406f,  1.83157f,  0.19362f,  0.46910f,  0.39608f,
  0.33342f,  0.40083f,  0.27645f,  1.06864f,  -4.06645f, -0.38775f, -0.11070f,
  0.03781f,  -0.09141f, 0.06185f,  -0.04852f, 0.20163f,  0.16784f,  0.16641f,
  -0.50941f, -0.61087f, 2.07008f,  -0.82381f, -0.85558f, 0.05528f,  -0.10535f,
  -2.81150f, 0.67038f,  0.43643f,  0.49062f,  -0.04465f, 0.90438f,  0.00977f,
  0.46272f,  1.59751f,  0.95234f,  0.35086f,  0.85624f,  0.73149f,  1.67779f,
  -2.21511f, -1.24746f, -1.09014f, -0.92441f, -1.22591f, -1.06961f, -0.95897f,
  -1.24956f, 0.73797f,  1.23275f,  -0.60064f, -0.07851f, 0.14397f,  0.22110f,
  -0.04422f, 0.14350f,  0.75926f,  0.35032f,  0.48104f,  2.81408f,  0.34662f,
  0.42090f,  0.35521f,  -1.36804f, -0.14974f, -0.47696f, -0.07892f, 0.36910f,
  0.32299f,  0.23916f,  0.06032f,  -0.17844f, -0.17558f, -1.42746f, -0.55828f,
  -1.00418f, -0.64823f, -0.73654f, -0.85197f, -1.50989f, 1.69385f,  -0.04973f,
  -0.09273f, 1.04249f,  0.79235f,  1.13229f,  0.99617f,  0.03851f,  0.56334f,
  0.90795f,  1.08296f,  0.58519f,  1.74765f,  0.63971f,  1.35951f,  0.07803f,
  -0.05127f, 0.26514f,  -0.84629f, -0.66343f, -2.10630f, 0.11017f,  2.18528f,
  -0.21958f, 0.05970f,
};

static const float av1_tx_type_nn_bias_4x8_ver_layer0[16] = {
  0.04205f, 0.22260f, -1.03870f, -1.19568f, 0.44283f,  0.01143f,
  0.00235f, 4.26772f, 0.44364f,  -0.33199f, -0.39076f, -0.35129f,
  0.08288f, 0.18195f, -0.79890f, 0.10047f,
};

static const float av1_tx_type_nn_weights_4x8_ver_layer1[64] = {
  -0.38193f, -0.12095f, 1.57802f,  0.34932f,  -0.47333f, -0.12304f, -0.01736f,
  -2.52445f, 0.18983f,  -0.64707f, -0.60889f, -0.53750f, 0.91666f,  -0.62823f,
  -0.13377f, -0.43594f, -0.38618f, -0.01328f, 0.97457f,  1.48589f,  -1.03238f,
  -0.33459f, -0.35108f, -2.42417f, 0.60229f,  0.06824f,  -0.75495f, 0.26902f,
  0.65311f,  -0.23887f, -0.44604f, -0.55800f, -0.33842f, 0.04259f,  -0.59589f,
  0.49738f,  -0.62301f, -0.30896f, -0.29602f, -2.57052f, 2.00943f,  -0.66490f,
  -0.76312f, 0.28256f,  1.06311f,  -0.38364f, -0.63508f, -0.57609f, -0.88765f,
  -1.04403f, -0.46531f, 0.34084f,  -1.20498f, -0.68352f, -0.72251f, -2.63242f,
  -0.68736f, -0.37904f, -1.32371f, 0.47288f,  1.51904f,  0.78372f,  -1.01830f,
  -1.01848f,
};

static const float av1_tx_type_nn_bias_4x8_ver_layer1[4] = {
  -1.45955f,
  -2.08949f,
  -1.24813f,
  -1.55368f,
};

static const NN_CONFIG av1_tx_type_nnconfig_4x8_ver = {
  8,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_4x8_ver_layer0,
    av1_tx_type_nn_weights_4x8_ver_layer1 },
  { av1_tx_type_nn_bias_4x8_ver_layer0, av1_tx_type_nn_bias_4x8_ver_layer1 }
};
/******************************************************************************/

// Tx type model for 8x4 block.
static const float av1_tx_type_nn_weights_8x4_hor_layer0[128] = {
  -0.22492f, 0.13341f,  -4.03243f, -0.64015f, 0.02783f,  0.60466f,  -0.13335f,
  0.16828f,  0.12336f,  0.52904f,  1.18455f,  -0.32425f, 0.13052f,  0.93810f,
  -3.71165f, 0.02990f,  -4.63558f, 0.05666f,  0.03524f,  -0.07449f, -0.44006f,
  -0.33215f, -0.33713f, 0.08097f,  0.60873f,  0.29582f,  0.21696f,  -0.78729f,
  -0.16757f, -0.26567f, -0.00720f, -1.11226f, 1.58189f,  1.58463f,  1.48536f,
  1.54374f,  1.60069f,  1.46125f,  1.53932f,  0.05974f,  -1.82192f, 0.47043f,
  0.38090f,  0.20833f,  -0.05637f, 0.05183f,  0.01323f,  -0.25662f, 0.78634f,
  -0.55069f, -0.02975f, -1.29294f, -0.77192f, -2.34299f, -1.28074f, 0.77894f,
  -1.69740f, -1.66032f, -1.44323f, -1.55063f, -1.50845f, -1.23690f, -1.80663f,
  0.75079f,  2.32551f,  0.05878f,  0.80438f,  0.88584f,  0.69153f,  0.89060f,
  0.73660f,  0.87259f,  -0.00745f, -1.30044f, -0.59430f, 2.07270f,  1.03307f,
  -0.84697f, -1.19393f, 0.17549f,  -0.24978f, -3.67234f, 0.20781f,  -0.53946f,
  -0.05068f, 0.88274f,  1.30371f,  0.10288f,  0.07585f,  0.12259f,  -0.30815f,
  0.25437f,  -2.82096f, -2.69482f, 0.02370f,  0.12500f,  -0.21019f, -0.49220f,
  0.03638f,  -0.29795f, 0.28645f,  -0.48432f, -0.38584f, -0.32148f, -0.47197f,
  0.32437f,  0.32528f,  -0.19437f, 0.30383f,  -0.31879f, 0.26359f,  -0.12164f,
  -0.43647f, -0.08288f, -0.33438f, -0.63608f, -0.46647f, -0.46574f, 0.47806f,
  -0.49012f, -1.51234f, -1.13502f, -1.20470f, -1.02913f, -1.09182f, -0.93921f,
  -1.85523f, 0.92532f,
};

static const float av1_tx_type_nn_bias_8x4_hor_layer0[16] = {
  0.36631f,  0.02901f,  0.64305f,  1.53074f, -1.40229f, 0.03852f,
  -0.05043f, 0.89632f,  -1.23312f, 0.07036f, 0.17070f,  0.56250f,
  -0.28958f, -0.32869f, -0.01704f, 0.68171f,
};

static const float av1_tx_type_nn_weights_8x4_hor_layer1[64] = {
  -0.49441f, -0.31960f, -0.84946f, -0.85800f, -2.37767f, 0.81373f,  -0.73172f,
  -0.69337f, 0.88807f,  -0.49242f, -0.44717f, -0.11436f, 0.09978f,  0.15393f,
  0.17083f,  1.44850f,  -0.20582f, -0.04906f, 0.42990f,  -0.61939f, -1.09692f,
  -1.14885f, -1.36879f, -1.30828f, -0.59558f, -0.30903f, -0.08906f, 0.06953f,
  0.15383f,  -0.04193f, -0.54858f, 1.82676f,  -0.22411f, 0.05264f,  -0.45848f,
  -0.72985f, 0.87553f,  0.04116f,  -1.29774f, -2.63018f, 1.09089f,  -0.36048f,
  -0.16725f, 0.11627f,  0.49918f,  0.07539f,  0.00763f,  0.73706f,  0.87800f,
  0.57049f,  0.60969f,  1.02779f,  1.53339f,  -0.35915f, 0.06410f,  1.44582f,
  0.09698f,  0.71888f,  0.60594f,  0.84103f,  -0.50440f, -0.38825f, 0.15626f,
  -1.10654f,
};

static const float av1_tx_type_nn_bias_8x4_hor_layer1[4] = {
  -0.92861f,
  -1.45151f,
  -1.33588f,
  -4.33853f,
};

static const NN_CONFIG av1_tx_type_nnconfig_8x4_hor = {
  8,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_8x4_hor_layer0,
    av1_tx_type_nn_weights_8x4_hor_layer1 },
  { av1_tx_type_nn_bias_8x4_hor_layer0, av1_tx_type_nn_bias_8x4_hor_layer1 }
};

static const float av1_tx_type_nn_weights_8x4_ver_layer0[32] = {
  -1.10946f, 1.86574f,  -1.59343f, 0.27018f, -1.70676f, -0.73982f, -0.19021f,
  -1.94208f, -2.29759f, -1.44402f, 0.28700f, -1.18340f, -1.50158f, -0.44175f,
  -1.36831f, 1.00374f,  2.59312f,  0.50291f, -0.71042f, -0.12238f, -0.15901f,
  -0.22807f, -0.67376f, -0.30215f, 0.54407f, -0.45538f, 1.18262f,  2.28687f,
  1.66212f,  1.70826f,  1.55182f,  0.12230f,
};

static const float av1_tx_type_nn_bias_8x4_ver_layer0[8] = {
  0.10943f,  2.09789f, 2.16578f, 0.15766f,
  -0.42461f, 0.00000f, 1.22090f, -1.28717f,
};

static const float av1_tx_type_nn_weights_8x4_ver_layer1[32] = {
  1.20426f,  -1.23237f, 2.41053f, -0.72488f, 1.25249f,  0.18018f,  -0.09586f,
  2.17901f,  0.15364f,  1.21535f, -0.38263f, -0.74309f, 0.50551f,  -0.54208f,
  0.59139f,  1.16095f,  0.55919f, -0.60183f, 1.18949f,  1.60787f,  0.54002f,
  -0.10712f, -0.16153f, 0.16207f, -0.32338f, 2.68712f,  -2.83483f, -0.27086f,
  -1.15005f, -0.39311f, 1.51236f, -1.68973f,
};

static const float av1_tx_type_nn_bias_8x4_ver_layer1[4] = {
  1.81013f,
  1.10517f,
  2.90059f,
  0.95391f,
};

static const NN_CONFIG av1_tx_type_nnconfig_8x4_ver = {
  4,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      8,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_8x4_ver_layer0,
    av1_tx_type_nn_weights_8x4_ver_layer1 },
  { av1_tx_type_nn_bias_8x4_ver_layer0, av1_tx_type_nn_bias_8x4_ver_layer1 }
};
/******************************************************************************/

// Tx type model for 8x8 block.
static const float av1_tx_type_nn_weights_8x8_hor_layer0[128] = {
  -0.85529f, 0.37619f,  0.12754f,  0.08622f,  0.45278f,  0.54929f,  1.60651f,
  -0.62654f, -0.54929f, -0.10131f, -0.17569f, 0.13948f,  0.31695f,  -0.05616f,
  0.20483f,  -0.36448f, 2.27203f,  -0.33087f, 0.47679f,  0.86888f,  0.39370f,
  0.46239f,  0.01113f,  1.50327f,  -1.48226f, -1.69621f, -1.49777f, -1.38885f,
  -1.37753f, -1.22681f, -1.70576f, 0.51329f,  -1.65662f, 1.74197f,  -0.13579f,
  -0.13133f, -0.58396f, -0.55510f, -1.10709f, -2.34975f, 0.22445f,  -0.56491f,
  -0.83432f, 0.13492f,  1.32147f,  2.85285f,  0.13819f,  0.03792f,  -1.30792f,
  0.04155f,  -0.70644f, -0.43430f, -0.16212f, -0.86945f, -1.16976f, 1.68339f,
  0.29540f,  0.01137f,  -0.25335f, -0.16856f, 0.12028f,  0.05207f,  0.39357f,
  -0.01545f, -0.21980f, -1.94091f, -1.01315f, -0.68270f, -0.40590f, -0.67111f,
  2.08283f,  0.19291f,  -4.81426f, -0.65044f, -0.24598f, 0.06371f,  -0.10272f,
  -0.14502f, -0.06821f, 0.45202f,  0.21091f,  -0.80864f, 0.39255f,  1.79189f,
  1.80453f,  1.10484f,  1.17608f,  0.96901f,  -0.35871f, -0.94311f, 0.63147f,
  2.95157f,  0.45917f,  -0.42849f, -0.55643f, -0.06097f, 3.49299f,  -0.50972f,
  0.11075f,  -0.08405f, -0.09274f, -0.22694f, -0.42426f, 0.48632f,  -1.61074f,
  1.82998f,  0.37623f,  -1.20330f, -0.01142f, -1.33307f, -0.27492f, -2.23621f,
  1.38846f,  1.42085f,  1.42568f,  1.36152f,  1.46910f,  1.27473f,  1.34752f,
  0.12753f,  -1.08197f, -1.08280f, -0.79489f, -1.12338f, -1.06795f, -0.87857f,
  -0.99892f, 1.09823f,
};

static const float av1_tx_type_nn_bias_8x8_hor_layer0[16] = {
  -0.49232f, -0.29685f, -1.44020f, 1.10940f,  1.16452f, -0.34862f,
  -0.38761f, -0.36243f, 0.21776f,  0.28234f,  2.34269f, -0.04104f,
  -0.26319f, 2.65579f,  -1.30137f, -0.01487f,
};

static const float av1_tx_type_nn_weights_8x8_hor_layer1[64] = {
  -0.38058f, -0.41295f, -1.26884f, -0.75560f, -1.57450f, 0.56072f,  -1.42322f,
  -0.29106f, 0.07228f,  0.04391f,  1.61388f,  -0.03055f, 0.81637f,  2.06045f,
  0.27119f,  -0.48328f, -0.45528f, -0.60534f, -1.61209f, -0.78157f, -1.65034f,
  0.60958f,  -1.30523f, 0.25143f,  0.11398f,  0.37860f,  1.54829f,  0.02309f,
  0.67288f,  2.11447f,  0.44845f,  -0.70406f, -0.67897f, -0.38759f, -1.30383f,
  -1.22646f, -1.54571f, 0.60552f,  -1.52565f, 0.11469f,  0.17344f,  0.08622f,
  1.57906f,  -0.00909f, 0.81634f,  2.04909f,  1.26466f,  -1.45741f, -0.75229f,
  0.06200f,  -1.05835f, -0.66257f, -1.73766f, 0.99923f,  -1.87082f, 0.14580f,
  0.49525f,  0.46839f,  1.32203f,  0.33923f,  0.97001f,  2.38584f,  1.58811f,
  0.06161f,
};

static const float av1_tx_type_nn_bias_8x8_hor_layer1[4] = {
  1.70385f,
  1.82373f,
  1.78496f,
  1.80826f,
};

static const NN_CONFIG av1_tx_type_nnconfig_8x8_hor = {
  8,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_8x8_hor_layer0,
    av1_tx_type_nn_weights_8x8_hor_layer1 },
  { av1_tx_type_nn_bias_8x8_hor_layer0, av1_tx_type_nn_bias_8x8_hor_layer1 }
};

static const float av1_tx_type_nn_weights_8x8_ver_layer0[128] = {
  -0.67016f, -1.72366f, -1.86576f, -1.50962f, -1.70419f, -1.73964f, -1.84615f,
  2.09681f,  -0.05081f, -0.61030f, 2.02541f,  0.60222f,  0.99936f,  2.02114f,
  -0.53893f, -0.23757f, 0.73566f,  0.25443f,  0.00132f,  -0.74036f, -0.75351f,
  -0.76964f, -1.71007f, -0.15770f, 1.60982f,  2.17638f,  0.90681f,  0.64973f,
  0.85914f,  0.58786f,  -1.46228f, 0.05187f,  1.18804f,  0.30850f,  0.29512f,
  0.40526f,  0.37635f,  0.32311f,  0.37471f,  1.12346f,  3.41856f,  -0.36653f,
  0.42537f,  -0.19240f, 0.00155f,  0.30826f,  -0.02116f, -0.53435f, -0.34829f,
  -0.52466f, -0.11521f, -0.29163f, -2.05689f, -2.87372f, -0.62626f, 0.09585f,
  -0.75257f, 0.10057f,  1.43474f,  0.89450f,  0.75900f,  1.11147f,  1.00558f,
  0.25886f,  2.22095f,  -0.17926f, 0.57161f,  0.39546f,  0.47846f,  0.40452f,
  0.54298f,  0.45814f,  -3.62788f, -3.02374f, 0.03716f,  -0.13937f, -0.09415f,
  -0.12463f, 0.05682f,  0.03672f,  1.20746f,  1.25003f,  1.27071f,  1.31883f,
  1.27473f,  1.34943f,  1.23158f,  0.09039f,  0.19388f,  0.63420f,  2.79612f,
  0.93803f,  -0.11323f, -0.02027f, 0.41286f,  -0.05979f, -3.80705f, -0.52451f,
  -0.77098f, -0.68132f, -0.65559f, -0.60975f, -1.26165f, 0.25582f,  0.05346f,
  0.61403f,  0.32140f,  -2.39831f, -1.42355f, 1.30541f,  1.02361f,  0.12930f,
  -1.61469f, -0.77036f, -0.59144f, 1.27769f,  1.52068f,  0.82137f,  1.83159f,
  -0.66626f, -0.69806f, -1.00564f, -0.85995f, -0.90889f, -0.84412f, -0.85712f,
  -1.29848f, 0.39308f,
};

static const float av1_tx_type_nn_bias_8x8_ver_layer0[16] = {
  -0.14868f, -0.48343f, 3.94416f,  -0.78037f, -1.33789f, -0.60611f,
  0.51793f,  0.44030f,  -0.71563f, 0.22561f,  -1.19083f, -0.46149f,
  0.83015f,  0.06024f,  1.17180f,  0.65122f,
};

static const float av1_tx_type_nn_weights_8x8_ver_layer1[64] = {
  -1.42711f, -0.21683f, 2.12061f,  0.20489f,  -0.50228f, -0.24770f, 0.23391f,
  1.03470f,  -0.44847f, -0.63225f, -0.21583f, -0.06467f, -0.21892f, -0.07786f,
  1.43322f,  0.00280f,  -1.53057f, -0.18912f, 1.95333f,  0.31151f,  -2.07601f,
  0.06776f,  0.25529f,  0.94800f,  -1.11453f, -0.20594f, -0.13281f, 0.01485f,
  0.17650f,  -0.07955f, 1.43734f,  -0.23193f, -2.06463f, -0.21238f, 2.13707f,
  0.30351f,  0.27594f,  -0.36245f, 0.19539f,  0.91045f,  -0.24068f, -0.37616f,
  0.88792f,  0.02947f,  -0.16903f, -0.04932f, 1.51293f,  -0.95967f, -1.62903f,
  0.05326f,  2.30703f,  0.64445f,  -1.09464f, -0.16623f, 1.00240f,  0.07548f,
  -0.50406f, 0.63854f,  1.02340f,  0.49833f,  0.13671f,  0.26722f,  2.09516f,
  -0.41305f,
};

static const float av1_tx_type_nn_bias_8x8_ver_layer1[4] = {
  2.14067f,
  2.76699f,
  2.04233f,
  1.34803f,
};

static const NN_CONFIG av1_tx_type_nnconfig_8x8_ver = {
  8,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_8x8_ver_layer0,
    av1_tx_type_nn_weights_8x8_ver_layer1 },
  { av1_tx_type_nn_bias_8x8_ver_layer0, av1_tx_type_nn_bias_8x8_ver_layer1 }
};
/******************************************************************************/

// Tx type model for 8x16 block.
static const float av1_tx_type_nn_weights_8x16_hor_layer0[128] = {
  -1.61872f, -1.58520f, -1.41236f, -1.53255f, -1.59794f, -1.25769f, -1.90043f,
  0.73431f,  1.10135f,  0.47054f,  0.43230f,  -0.43009f, -0.09135f, -0.07289f,
  -0.38785f, 1.23775f,  -0.35312f, 0.73789f,  0.88864f,  0.75957f,  0.62579f,
  0.46974f,  0.21851f,  1.63821f,  -2.27289f, -0.68522f, -0.69814f, -0.84368f,
  -0.91320f, -0.63055f, -1.03296f, 0.55778f,  -0.00071f, 1.27539f,  1.60068f,
  1.40975f,  0.97372f,  0.92843f,  1.90853f,  0.12626f,  1.71953f,  1.41978f,
  -0.12234f, -1.27058f, 0.76207f,  0.02495f,  -0.67038f, -0.05255f, 1.72923f,
  1.47630f,  1.47058f,  1.47614f,  1.49354f,  1.66131f,  1.50801f,  0.17145f,
  -2.30947f, -2.10850f, -1.25636f, -0.24900f, 0.72602f,  1.26572f,  0.97865f,
  -0.65466f, 1.31129f,  0.26916f,  0.12139f,  -0.12761f, -0.39143f, -0.28134f,
  0.06584f,  2.24418f,  0.22516f,  0.05011f,  -0.01671f, -0.29476f, -0.40326f,
  0.21138f,  -0.11573f, -0.31154f, -0.36828f, 0.03694f,  -0.07172f, -0.63419f,
  -3.14351f, -1.23125f, 0.65311f,  -0.11406f, 1.97287f,  -0.10422f, 0.83896f,
  0.85033f,  0.49724f,  0.80482f,  0.51454f,  1.06447f,  0.76693f,  0.72599f,
  -0.78573f, -0.53950f, 0.40894f,  0.00086f,  0.10784f,  -0.70498f, 1.16395f,
  1.14597f,  1.13496f,  1.12177f,  1.02100f,  -1.37574f, -2.97144f, 0.33899f,
  0.42013f,  0.86327f,  2.31983f,  2.04008f,  0.95503f,  0.15081f,  0.11530f,
  -0.02574f, -4.77119f, 0.13257f,  -0.01704f, -0.23087f, -0.00825f, 0.07029f,
  -0.28136f, 0.42556f,
};

static const float av1_tx_type_nn_bias_8x16_hor_layer0[16] = {
  0.93617f,  -0.24000f, -1.26821f, 0.78780f,  0.13690f, -0.21948f,
  -1.45162f, 0.44584f,  -1.92582f, -0.23169f, 0.56004f, -1.19937f,
  1.81560f,  -1.02643f, -0.81690f, 0.08302f,
};

static const float av1_tx_type_nn_weights_8x16_hor_layer1[64] = {
  0.06696f,  -0.11538f, -1.42029f, 0.32965f,  0.81046f,  0.01146f,  1.20945f,
  -0.16899f, 0.53224f,  -0.40232f, 0.01786f,  -0.73242f, 1.29750f,  1.95185f,
  0.70143f,  1.43287f,  0.76220f,  0.79937f,  -1.79011f, -1.15178f, 0.42526f,
  -0.67519f, 0.77267f,  -0.30697f, 2.46004f,  -0.49828f, 0.02875f,  1.09972f,
  1.47662f,  0.61719f,  0.61417f,  -0.12363f, 2.53048f,  0.00418f,  -1.38964f,
  0.88117f,  0.39239f,  -0.19347f, -2.58600f, -0.33715f, 1.09323f,  -0.32127f,
  0.02456f,  -0.19125f, 1.12728f,  0.66502f,  0.34296f,  1.14897f,  0.29967f,
  1.19209f,  0.22108f,  -0.11975f, 1.49776f,  -1.34624f, -2.58478f, -1.34632f,
  1.53207f,  0.45634f,  -1.48476f, 0.17489f,  0.71790f,  -2.12086f, -1.21778f,
  -1.31243f,
};

static const float av1_tx_type_nn_bias_8x16_hor_layer1[4] = {
  0.83359f,
  1.06875f,
  1.77645f,
  1.49570f,
};

static const NN_CONFIG av1_tx_type_nnconfig_8x16_hor = {
  8,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_8x16_hor_layer0,
    av1_tx_type_nn_weights_8x16_hor_layer1 },
  { av1_tx_type_nn_bias_8x16_hor_layer0, av1_tx_type_nn_bias_8x16_hor_layer1 }
};

static const float av1_tx_type_nn_weights_8x16_ver_layer0[128] = {
  0.32858f,  -1.28887f, 0.25632f,  -0.05262f, 2.69203f,  -0.07004f, 1.37337f,
  -0.05725f, -0.05659f, 0.05592f,  0.01039f,  -0.29343f, 1.58628f,  -0.30003f,
  -3.43118f, 0.00272f,  1.70928f,  -0.76348f, 0.05889f,  -0.03263f, -0.07724f,
  0.03523f,  -0.19890f, 1.18005f,  -0.03605f, -0.20530f, -4.00733f, 0.10210f,
  -0.05368f, -0.17650f, -0.15317f, 0.06499f,  0.56705f,  1.04341f,  0.62890f,
  0.73451f,  -0.22199f, 0.86659f,  0.78443f,  -0.61664f, -0.50606f, 0.30247f,
  0.14455f,  0.39276f,  0.49203f,  0.65019f,  0.12269f,  1.64080f,  1.68289f,
  1.42694f,  1.60825f,  1.58501f,  1.47252f,  1.62589f,  1.48218f,  0.17726f,
  -0.04884f, 0.35376f,  -0.04796f, 0.32589f,  0.35087f,  0.35258f,  -0.46103f,
  -0.31176f, -0.05203f, 0.07247f,  -0.26756f, 0.22019f,  0.03412f,  0.33773f,
  0.29811f,  -0.11140f, 0.12831f,  -0.44673f, -0.09858f, 0.07889f,  0.15137f,
  0.00347f,  -0.23394f, 0.08886f,  -0.31201f, -0.79912f, -0.51092f, 0.14123f,
  -1.09599f, -4.26020f, -0.68675f, -0.02842f, -1.54538f, -1.28977f, -1.30558f,
  -1.21074f, -1.37142f, -1.14743f, -1.85397f, 0.82985f,  -0.30681f, 0.04494f,
  -0.24023f, -4.18053f, -0.16096f, -0.55492f, -0.27882f, 0.05829f,  -0.41224f,
  -2.52088f, -0.56162f, -1.04547f, -1.70685f, -0.28842f, -1.43673f, -0.01468f,
  -3.20585f, -0.69120f, -0.43931f, -0.46270f, -0.65885f, -0.55884f, -0.75138f,
  0.36381f,  -5.70858f, -0.14548f, -0.15745f, -0.11812f, -0.07605f, -0.07693f,
  -0.12236f, 0.16075f,
};

static const float av1_tx_type_nn_bias_8x16_ver_layer0[16] = {
  -0.35385f, 0.30491f,  -0.90011f, 0.42941f,  1.20928f, -0.88331f,
  -1.48818f, -0.34785f, -0.32668f, -0.22695f, 0.89188f, 0.65521f,
  0.57598f,  0.99819f,  0.75175f,  0.17044f,
};

static const float av1_tx_type_nn_weights_8x16_ver_layer1[64] = {
  -0.62913f, -0.34304f, 0.42963f,  -0.17440f, -1.44092f, 0.69142f,  -1.36067f,
  0.52211f,  0.44658f,  -0.26501f, -0.41657f, 0.34428f,  -0.34390f, -0.58567f,
  -0.84097f, -1.96311f, -0.37215f, -0.22250f, -1.23811f, -0.07247f, -0.81731f,
  0.58755f,  -1.30559f, 0.39551f,  0.41743f,  -0.09940f, -0.33230f, 0.14458f,
  -0.25139f, -0.54517f, 0.13469f,  -0.38157f, -0.39109f, -0.18205f, 0.06834f,
  -0.08395f, -0.92187f, 0.56724f,  1.44381f,  0.53226f,  -0.22356f, 0.12285f,
  -0.29418f, -1.86749f, -0.22372f, -0.60204f, -0.87746f, -1.16936f, 0.56884f,
  0.62641f,  -0.11823f, 1.00395f,  1.64794f,  -0.64535f, 2.29322f,  -0.23397f,
  0.17251f,  -0.35927f, 0.65631f,  -0.26812f, 0.80128f,  0.85748f,  0.47404f,
  2.20547f,
};

static const float av1_tx_type_nn_bias_8x16_ver_layer1[4] = {
  -0.44080f,
  -1.67455f,
  -1.46332f,
  -6.13206f,
};

static const NN_CONFIG av1_tx_type_nnconfig_8x16_ver = {
  8,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_8x16_ver_layer0,
    av1_tx_type_nn_weights_8x16_ver_layer1 },
  { av1_tx_type_nn_bias_8x16_ver_layer0, av1_tx_type_nn_bias_8x16_ver_layer1 }
};
/******************************************************************************/

// Tx type model for 16x8 block.
static const float av1_tx_type_nn_weights_16x8_hor_layer0[128] = {
  0.02600f,  0.09786f,  -1.05107f, -0.35594f, -0.15658f, 2.99828f,  -0.07106f,
  -0.10101f, -0.14412f, -0.83790f, -0.19434f, 2.28368f,  1.91727f,  -0.00956f,
  -0.90640f, 0.09174f,  1.58895f,  1.38945f,  1.49431f,  1.51381f,  1.44803f,
  1.53544f,  1.44694f,  0.17753f,  1.69735f,  -0.78652f, 0.31092f,  -0.23736f,
  0.02231f,  -0.09884f, -0.00493f, 1.21189f,  -1.94382f, -0.34629f, -0.58309f,
  0.72291f,  -0.30056f, 0.90660f,  -0.57495f, 3.07809f,  0.73644f,  1.43050f,
  1.34356f,  -0.66554f, 0.50102f,  -0.64305f, 0.42044f,  -1.66165f, -0.05733f,
  -2.51402f, -1.01067f, -0.33390f, -0.32986f, -0.92431f, 1.86281f,  -0.07290f,
  -0.26290f, -0.68941f, 1.81156f,  0.66125f,  -2.09974f, 0.17032f,  -0.67461f,
  -0.00876f, -1.50154f, 1.17153f,  1.00377f,  0.33022f,  0.74689f,  0.42878f,
  0.61725f,  -0.83967f, 0.09467f,  -0.39892f, 0.33863f,  0.10656f,  -0.09249f,
  -0.39757f, 0.48481f,  -0.35162f, 1.47014f,  1.67827f,  -1.84051f, 0.16291f,
  -0.50135f, -2.29911f, -0.42217f, -0.13358f, 1.45899f,  -0.14743f, -0.02763f,
  -0.28003f, -0.01364f, 0.21014f,  -0.29026f, -0.20198f, 1.38782f,  0.56731f,
  0.27489f,  0.43227f,  0.41326f,  0.42721f,  0.87720f,  -1.90067f, -5.04951f,
  -0.17638f, -0.58119f, -0.08954f, -0.13692f, -0.12325f, -0.38548f, 0.66462f,
  -1.42377f, -1.21917f, -1.38193f, -1.36539f, -1.39378f, -1.19629f, -1.59812f,
  0.28689f,  0.32394f,  0.52128f,  0.01013f,  -0.28948f, -0.26293f, -0.44331f,
  -0.36570f, -0.50757f,
};

static const float av1_tx_type_nn_bias_16x8_hor_layer0[16] = {
  -0.08696f, -0.22110f, -1.43604f, -1.00451f, -1.51029f, 0.63736f,
  0.45260f,  0.16229f,  4.01393f,  -0.21748f, 0.36411f,  -0.08764f,
  -0.12329f, 0.08986f,  1.08117f,  -0.00220f,
};

static const float av1_tx_type_nn_weights_16x8_hor_layer1[64] = {
  0.55824f,  -0.14648f, 0.81947f,  -0.45867f, -1.86078f, -0.17291f, 0.34849f,
  0.15153f,  1.75625f,  -0.25760f, 0.72015f,  -0.30059f, -0.57975f, 0.07609f,
  -0.02036f, 0.07912f,  0.57080f,  -0.13792f, 0.74184f,  -0.87669f, -1.87572f,
  -0.27270f, 0.39751f,  0.19652f,  2.03514f,  -0.32944f, 0.76251f,  0.04399f,
  -0.63175f, 0.37420f,  0.08309f,  0.04466f,  0.60255f,  -0.12820f, 1.66065f,
  -0.59496f, -1.94794f, -0.14847f, 0.39424f,  0.16273f,  1.80587f,  0.41197f,
  0.74691f,  -0.21217f, -0.63173f, 0.09510f,  -0.35538f, -0.04407f, 0.92847f,
  0.20141f,  1.68680f,  -0.56528f, -2.26960f, 0.12978f,  0.73748f,  0.42438f,
  2.00673f,  -0.40189f, 0.95423f,  0.23234f,  -0.80953f, 0.65814f,  0.49444f,
  -0.23347f,
};

static const float av1_tx_type_nn_bias_16x8_hor_layer1[4] = {
  3.57175f,
  2.42612f,
  3.31259f,
  2.08287f,
};

static const NN_CONFIG av1_tx_type_nnconfig_16x8_hor = {
  8,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_16x8_hor_layer0,
    av1_tx_type_nn_weights_16x8_hor_layer1 },
  { av1_tx_type_nn_bias_16x8_hor_layer0, av1_tx_type_nn_bias_16x8_hor_layer1 }
};

static const float av1_tx_type_nn_weights_16x8_ver_layer0[128] = {
  0.46633f,  1.55328f,  -0.11230f, -0.29571f, 0.18814f,  -1.52430f, -2.34660f,
  0.08644f,  -1.97718f, -1.29140f, -1.12262f, -1.12985f, -1.25911f, -0.96506f,
  -1.57129f, 0.96021f,  1.34192f,  1.28623f,  1.21655f,  1.28758f,  1.25482f,
  1.30195f,  1.19190f,  0.09310f,  0.52072f,  0.91487f,  1.24100f,  1.61236f,
  1.72166f,  2.20750f,  1.62379f,  -1.43936f, 0.50665f,  0.40213f,  0.66502f,
  -1.66699f, -3.07618f, 0.05877f,  0.60987f,  -0.09995f, -0.10916f, 0.48049f,
  0.23812f,  0.39847f,  -0.21682f, -0.63455f, 0.33453f,  -0.67939f, -4.14355f,
  -0.62756f, -0.22502f, -0.17215f, 0.01062f,  0.27049f,  -0.10748f, 0.30945f,
  2.72445f,  -0.89181f, -0.06800f, 0.20595f,  -0.73385f, 0.04071f,  -1.30294f,
  1.83507f,  0.92570f,  0.69609f,  0.76285f,  0.69892f,  0.76409f,  0.63104f,
  0.73397f,  1.09575f,  -0.20129f, -0.24022f, -0.24599f, -0.59107f, -0.88755f,
  -0.68987f, -0.75495f, -1.31002f, -1.30237f, -0.94093f, -2.15678f, -1.49303f,
  -1.17498f, -1.39952f, -0.91270f, -0.05587f, 1.02381f,  -0.75580f, -0.65263f,
  -0.78996f, -0.71075f, -0.71018f, -0.70350f, -1.26196f, 2.34208f,  -0.53611f,
  0.19752f,  -0.16842f, -0.24828f, 0.21857f,  0.08222f,  -2.55894f, -1.75702f,
  0.11394f,  1.03083f,  0.79972f,  -1.54112f, -1.82341f, -0.57597f, -0.02077f,
  -0.39616f, -0.00995f, -0.12809f, 0.01188f,  -0.25117f, 0.09202f,  0.09336f,
  -0.05614f, -0.30039f, 0.25834f,  1.19944f,  1.22533f,  0.92330f,  0.75967f,
  -0.81945f, -0.41647f,
};

static const float av1_tx_type_nn_bias_16x8_ver_layer0[16] = {
  0.17841f,  0.67315f,  -1.24450f, 3.13859f,  0.16203f, -0.14992f,
  0.29553f,  -1.15567f, -0.71421f, 1.15977f,  1.14585f, 3.02460f,
  -0.04510f, 0.48000f,  -0.09354f, -0.42422f,
};

static const float av1_tx_type_nn_weights_16x8_ver_layer1[64] = {
  0.29912f,  -0.10009f, -1.11478f, 1.76812f,  -0.27719f, 0.52148f,  0.17622f,
  -1.17116f, 0.73397f,  -0.69279f, -0.11080f, 1.53751f,  -1.42003f, 0.14731f,
  0.13592f,  -0.04883f, 0.39186f,  -0.13655f, -0.43994f, 1.82759f,  -0.25601f,
  -0.15018f, 0.51920f,  -1.56070f, 0.31683f,  -0.79367f, -0.02904f, 1.28637f,
  -1.15203f, 0.26627f,  0.42828f,  -0.24258f, 0.38647f,  -0.83352f, 0.32553f,
  2.09522f,  -0.26822f, -0.42191f, 0.32825f,  -1.30748f, 1.50551f,  -0.52669f,
  0.20045f,  1.69318f,  -1.47839f, 0.30802f,  -0.07290f, -0.28106f, 0.68192f,
  -0.15522f, 1.12579f,  2.21921f,  0.09720f,  -0.50265f, 0.83165f,  -1.31721f,
  0.72422f,  -1.24952f, 0.61653f,  2.04117f,  -1.42406f, 0.52568f,  -0.46180f,
  -0.00873f,
};

static const float av1_tx_type_nn_bias_16x8_ver_layer1[4] = {
  3.34981f,
  3.74710f,
  1.38339f,
  0.45176f,
};

static const NN_CONFIG av1_tx_type_nnconfig_16x8_ver = {
  8,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_16x8_ver_layer0,
    av1_tx_type_nn_weights_16x8_ver_layer1 },
  { av1_tx_type_nn_bias_16x8_ver_layer0, av1_tx_type_nn_bias_16x8_ver_layer1 }
};
/******************************************************************************/

// Tx type model for 16x16 block.
static const float av1_tx_type_nn_weights_16x16_layer0[128] = {
  1.26592f,  1.36313f,  1.30956f,  1.29926f,  1.48816f,  1.68851f,  1.32000f,
  0.13321f,  -0.22477f, -0.88906f, -0.19622f, 1.69605f,  1.22180f,  -1.57771f,
  -1.15765f, 0.05710f,  -1.13355f, -0.85486f, -0.99971f, -0.91571f, -1.06031f,
  -0.77952f, -1.15723f, 1.17809f,  1.35602f,  -0.05243f, -0.37596f, 0.26108f,
  0.17611f,  -0.10323f, 0.77279f,  -0.48911f, -0.79308f, 0.55112f,  0.43918f,
  0.27872f,  0.28714f,  0.45830f,  1.05689f,  0.03705f,  -2.49975f, -0.01940f,
  0.05709f,  0.07942f,  -0.13290f, -0.10359f, 0.00143f,  0.37303f,  0.96470f,
  0.53293f,  1.14459f,  0.89185f,  0.43378f,  0.47764f,  0.90924f,  0.15279f,
  -0.15361f, 0.02949f,  0.42240f,  0.68143f,  0.89588f,  0.73754f,  0.10974f,
  1.57755f,  -0.39870f, -0.32914f, 0.35638f,  0.34991f,  -0.00003f, -0.23373f,
  0.29630f,  -0.76699f, -0.01356f, 0.04234f,  0.84253f,  1.92078f,  0.93160f,
  0.71993f,  0.71604f,  0.76455f,  -1.59782f, 0.32332f,  1.11628f,  0.33062f,
  -0.03728f, -0.05710f, 0.80447f,  -0.14719f, 1.34658f,  -0.05718f, 0.64015f,
  0.21926f,  0.41653f,  0.12720f,  0.54092f,  1.39411f,  1.81819f,  -0.24513f,
  0.00955f,  0.38011f,  -0.57787f, -0.41759f, 0.68834f,  -0.31783f, -0.40607f,
  -0.10107f, -0.79374f, 0.75599f,  -0.16282f, -0.14490f, -0.20783f, -0.55019f,
  -0.13793f, -0.22293f, 0.18305f,  0.12445f,  0.56830f,  0.24567f,  0.09278f,
  0.70803f,  0.35803f,  -1.52676f, -0.89624f, 0.77665f,  0.19877f,  0.77175f,
  0.50355f,  0.08592f,
};

static const float av1_tx_type_nn_bias_16x16_layer0[16] = {
  -1.31834f, 0.14346f,  -0.10062f, 0.84489f,  0.95617f,  -0.06720f,
  -0.68502f, -0.91442f, -0.31932f, 0.25276f,  -0.15138f, -1.57661f,
  -0.14062f, -0.42120f, 0.94573f,  -0.09287f,
};

static const float av1_tx_type_nn_weights_16x16_layer1[64] = {
  -1.80333f, -1.06353f, 0.55139f,  0.74644f,  0.13747f, -0.93018f, -0.10286f,
  0.67133f,  0.24460f,  1.44583f,  0.02173f,  0.26037f, -0.73687f, 0.19566f,
  0.61846f,  -0.58601f, -1.03196f, -0.74415f, 0.30041f, -0.41967f, 1.08740f,
  0.96224f,  -0.59139f, 0.03813f,  0.05403f,  1.33427f, -0.54375f, -1.92181f,
  0.54704f,  0.13608f,  0.22151f,  -0.38076f, 1.18390f, -0.77508f, -1.84283f,
  1.00894f,  0.62318f,  -0.15296f, 1.27600f,  0.22822f, 0.12751f,  0.93910f,
  -0.28502f, 0.53912f,  -0.96889f, 0.10182f,  0.81508f, -0.43028f, 2.67386f,
  0.52204f,  0.49820f,  -0.41711f, 1.05038f,  1.12192f, 0.74349f,  -0.75417f,
  -0.03718f, -0.35769f, 0.89651f,  0.63236f,  0.54215f, -0.07894f, 0.48274f,
  1.08829f,
};

static const float av1_tx_type_nn_bias_16x16_layer1[4] = {
  0.81986f,
  1.26865f,
  0.11118f,
  2.48404f,
};

static const NN_CONFIG av1_tx_type_nnconfig_16x16 = {
  8,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  {
      av1_tx_type_nn_weights_16x16_layer0,
      av1_tx_type_nn_weights_16x16_layer1,
  },
  {
      av1_tx_type_nn_bias_16x16_layer0,
      av1_tx_type_nn_bias_16x16_layer1,
  },
};
/******************************************************************************/

// Tx type model for 4x16 block.
static const float av1_tx_type_nn_weights_4x16_hor_layer0[32] = {
  0.36539f,  0.25667f,  0.01491f,  -0.21959f, 2.55105f,  0.17615f, 1.79884f,
  1.65936f,  -0.44363f, 0.00706f,  -0.68004f, -0.64360f, 1.75760f, 1.91906f,
  1.47682f,  0.09650f,  -3.59244f, -0.35004f, 0.93295f,  0.25806f, -0.08154f,
  0.79332f,  0.79535f,  1.09467f,  1.57855f,  -0.51359f, 0.90553f, -1.67744f,
  -1.74563f, -0.88830f, -1.77603f, 2.15935f,
};

static const float av1_tx_type_nn_bias_4x16_hor_layer0[8] = {
  -0.36435f, -2.22731f, -0.00837f, -1.34546f,
  0.62806f,  -0.20675f, 4.91940f,  -0.56079f,
};

static const float av1_tx_type_nn_weights_4x16_hor_layer1[32] = {
  -0.57191f, -1.46418f, 0.67331f,  -1.15027f, 0.46288f,  0.81251f,  2.51768f,
  -0.27147f, 0.00761f,  -2.15214f, -0.69650f, -0.50808f, 0.92832f,  0.45668f,
  2.34201f,  -0.52941f, 0.51008f,  -1.55496f, -0.01371f, -0.12356f, 0.66624f,
  0.88043f,  2.64862f,  -1.28024f, -0.17578f, -1.80034f, -0.32217f, 0.89519f,
  1.28413f,  -0.30326f, 2.45329f,  -0.83335f,
};

static const float av1_tx_type_nn_bias_4x16_hor_layer1[4] = {
  2.33198f,
  3.36245f,
  1.62603f,
  2.91056f,
};

static const NN_CONFIG av1_tx_type_nnconfig_4x16_hor = {
  4,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      8,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_4x16_hor_layer0,
    av1_tx_type_nn_weights_4x16_hor_layer1 },
  { av1_tx_type_nn_bias_4x16_hor_layer0, av1_tx_type_nn_bias_4x16_hor_layer1 }
};

static const float av1_tx_type_nn_weights_4x16_ver_layer0[128] = {
  1.61392f,  1.41239f,  1.47646f,  1.47325f,  1.46110f,  1.49208f,  1.49414f,
  0.12835f,  -0.76986f, 0.07087f,  -0.24572f, -0.93168f, 3.07935f,  -0.18183f,
  -0.09831f, -0.07703f, -0.03222f, -0.25473f, -0.06090f, 2.93713f,  -0.38711f,
  -0.12884f, -0.18329f, -0.06262f, -0.00327f, -0.02930f, -0.01641f, -0.00622f,
  -0.03305f, -4.07069f, -2.76643f, 0.04413f,  -1.03176f, -0.19217f, -0.44980f,
  -2.48615f, -2.58112f, -0.87695f, 0.16187f,  -0.04891f, -0.06854f, 1.08104f,
  0.75245f,  1.49302f,  0.63363f,  1.45715f,  0.92574f,  1.72029f,  0.33326f,
  3.86646f,  0.04422f,  0.41019f,  0.36212f,  0.56600f,  -1.01552f, 0.05128f,
  0.40454f,  -1.05100f, -0.47461f, -1.33168f, -0.46145f, -1.36870f, -0.88838f,
  -1.05358f, -0.18537f, -0.34357f, -0.03698f, 0.68905f,  0.41010f,  0.31223f,
  -0.43382f, -0.74715f, 2.03366f,  -0.30419f, 0.45747f,  0.09526f,  0.31678f,
  0.22915f,  0.21832f,  1.26385f,  -0.06814f, -0.71417f, -1.18947f, 0.03762f,
  0.10936f,  2.97396f,  -0.42638f, -0.03123f, -5.49756f, -0.17029f, -0.11323f,
  0.05173f,  -0.44274f, -0.15738f, 0.11311f,  0.43872f,  0.16837f,  -0.52849f,
  2.90050f,  -0.54735f, -0.29591f, 1.24030f,  0.21696f,  -0.04443f, -1.60877f,
  -1.36365f, -1.27432f, -1.52060f, -1.34397f, -1.13371f, -1.87554f, 0.80123f,
  0.42820f,  -0.14157f, -2.73963f, -0.68040f, -0.35236f, 0.14490f,  2.23477f,
  0.01370f,  -0.20426f, -1.51411f, -0.72293f, 0.64516f,  0.97638f,  0.32616f,
  -0.27975f, -0.01149f,
};

static const float av1_tx_type_nn_bias_4x16_ver_layer0[16] = {
  -1.37863f, -0.05763f, -0.07041f, 0.15306f,  0.96026f,  -1.42105f,
  -0.55822f, 1.04845f,  -0.17662f, -1.25345f, -0.11927f, 0.49845f,
  -0.32530f, 0.73483f,  0.08322f,  -0.23890f,
};

static const float av1_tx_type_nn_weights_4x16_ver_layer1[64] = {
  0.27194f,  0.50607f,  0.49229f,  -0.48192f, 0.15667f,  -1.38891f, 0.38102f,
  -0.58825f, -0.07337f, -0.52909f, 0.36975f,  0.28710f,  0.34992f,  -0.73630f,
  0.30386f,  -0.58822f, 0.36127f,  0.57950f,  0.55878f,  -0.42796f, 0.19967f,
  -1.45517f, 0.42529f,  -0.54630f, -0.38169f, -0.84899f, 0.41622f,  0.46935f,
  0.39077f,  -0.75448f, 0.31698f,  -0.76187f, 0.97765f,  0.57052f,  0.55825f,
  -0.54273f, 0.20466f,  -1.46347f, 0.41813f,  -0.55019f, -0.19948f, -0.57982f,
  0.41206f,  0.32373f,  0.38537f,  -1.11657f, 0.32887f,  -0.76911f, 1.12259f,
  0.72163f,  0.82603f,  0.37786f,  0.34976f,  -1.86642f, 0.59961f,  -0.16329f,
  -0.36631f, -0.56814f, 0.60410f,  0.53158f,  0.56389f,  -0.70508f, 0.51009f,
  -0.56513f,
};

static const float av1_tx_type_nn_bias_4x16_ver_layer1[4] = {
  4.60896f,
  4.53551f,
  4.53124f,
  4.27435f,
};

static const NN_CONFIG av1_tx_type_nnconfig_4x16_ver = {
  8,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_4x16_ver_layer0,
    av1_tx_type_nn_weights_4x16_ver_layer1 },
  { av1_tx_type_nn_bias_4x16_ver_layer0, av1_tx_type_nn_bias_4x16_ver_layer1 }
};
/******************************************************************************/

// Tx type model for 16x4 block.
static const float av1_tx_type_nn_weights_16x4_hor_layer0[128] = {
  1.45347f,  -0.15743f, 0.44236f,  0.25808f,  0.33944f,  0.38678f,  0.24428f,
  1.67287f,  0.09539f,  -0.42940f, -0.31507f, -0.00154f, -2.98755f, -2.27744f,
  -0.49183f, 0.09333f,  -0.99026f, -0.22157f, 0.53701f,  0.60447f,  0.15686f,
  -0.04646f, 0.26341f,  2.12361f,  0.27090f,  -1.14716f, -0.64146f, -0.91604f,
  -0.75335f, -0.60056f, -1.25084f, 1.68473f,  -3.24075f, -4.03867f, -2.07877f,
  -0.02347f, 0.00333f,  -0.01259f, -0.00465f, 0.02526f,  0.36286f,  -0.10324f,
  2.12780f,  -0.74584f, -1.05052f, 1.78467f,  -0.55065f, -0.03326f, 2.46781f,
  1.18349f,  0.96015f,  1.01696f,  1.10584f,  1.07263f,  1.11531f,  -1.06413f,
  0.32389f,  -1.87360f, -0.14435f, 1.77926f,  1.09966f,  -0.12680f, -0.61386f,
  -0.09724f, -0.33095f, 1.12122f,  1.00791f,  1.52416f,  1.35004f,  1.32657f,
  0.60950f,  -1.13538f, -0.38654f, 0.06473f,  2.10669f,  0.27734f,  -0.38359f,
  -1.91455f, -1.22676f, 0.05786f,  0.97432f,  2.19967f,  0.50457f,  0.78976f,
  0.95183f,  -0.32414f, 0.49437f,  -0.04506f, 0.18993f,  -0.07971f, 0.23889f,
  -0.09872f, -0.66036f, 0.05377f,  2.69638f,  -0.08259f, -0.69210f, -1.08296f,
  -1.96504f, -2.31947f, -0.80161f, -0.80456f, -1.35556f, -0.05323f, -4.42658f,
  -0.30732f, -0.12043f, 0.11126f,  0.10771f,  -0.14956f, -0.02218f, 0.41016f,
  1.16599f,  1.14629f,  1.12881f,  1.18676f,  1.24677f,  1.28695f,  1.11270f,
  0.08233f,  1.75440f,  0.49228f,  -0.34858f, -0.17032f, 0.29288f,  0.47175f,
  0.19055f,  -1.56413f,
};

static const float av1_tx_type_nn_bias_16x4_hor_layer0[16] = {
  -1.71227f, 0.47291f, -0.97536f, -0.66216f, 0.11729f,  -0.21451f,
  2.75281f,  0.04318f, 2.03965f,  0.14618f,  -0.70483f, -0.24517f,
  1.14048f,  0.33308f, -1.10886f, 0.41184f,
};

static const float av1_tx_type_nn_weights_16x4_hor_layer1[64] = {
  -1.17079f, 0.19096f,  -1.05753f, -0.30803f, -1.21680f, -0.67255f, 1.60115f,
  0.05972f,  1.44759f,  -0.04068f, -0.26331f, 0.31400f,  0.96923f,  0.33443f,
  -0.77215f, -0.91316f, -1.78928f, 0.21483f,  -1.24008f, -0.46190f, -0.12127f,
  -0.62144f, 1.37593f,  0.08373f,  1.56215f,  0.00279f,  -0.14556f, 0.38710f,
  0.96228f,  0.66433f,  -0.51798f, -0.80738f, -0.18539f, 0.19377f,  -1.03090f,
  -1.51044f, -0.59485f, -0.62589f, 1.90742f,  0.09078f,  1.49113f,  0.00205f,
  -0.15918f, 0.40827f,  1.08553f,  0.43431f,  0.33519f,  -1.12669f, -1.10274f,
  0.80004f,  -1.83599f, -0.53134f, 2.00515f,  -0.32670f, 1.37124f,  0.51136f,
  1.62563f,  0.24787f,  0.31757f,  0.81751f,  1.57262f,  0.83214f,  1.04661f,
  -0.43819f,
};

static const float av1_tx_type_nn_bias_16x4_hor_layer1[4] = {
  2.32575f,
  2.75703f,
  1.12304f,
  2.15567f,
};

static const NN_CONFIG av1_tx_type_nnconfig_16x4_hor = {
  8,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_16x4_hor_layer0,
    av1_tx_type_nn_weights_16x4_hor_layer1 },
  { av1_tx_type_nn_bias_16x4_hor_layer0, av1_tx_type_nn_bias_16x4_hor_layer1 }
};

static const float av1_tx_type_nn_weights_16x4_ver_layer0[32] = {
  0.26047f,  0.99930f,  1.16484f,  -0.28196f, -2.67483f, -0.21456f, -0.16854f,
  0.46375f,  1.47951f,  1.13735f,  1.12356f,  0.27385f,  0.50978f,  2.09967f,
  -1.47386f, 0.01950f,  -0.06362f, 0.26014f,  1.04544f,  -0.03099f, 0.07478f,
  -0.39701f, 0.05545f,  2.73633f,  -0.56305f, -0.02208f, -0.44517f, -0.00897f,
  -0.17967f, -0.96622f, 0.42635f,  -1.04784f,
};

static const float av1_tx_type_nn_bias_16x4_ver_layer0[8] = {
  -0.52088f, 0.52844f,  -1.03655f, -0.30974f,
  2.59952f,  -1.93604f, 0.00000f,  2.51787f,
};

static const float av1_tx_type_nn_weights_16x4_ver_layer1[32] = {
  0.10916f,  -0.21219f, -0.51340f, 0.69161f,  1.45988f,  -1.36942f, -0.40899f,
  1.05136f,  -0.08486f, 0.10008f,  -0.55304f, 0.88012f,  1.61177f,  -1.64507f,
  0.63428f,  1.15130f,  -0.17287f, -0.18592f, -0.01143f, 0.88293f,  1.73326f,
  -1.63624f, 0.09359f,  1.18393f,  0.26531f,  0.22378f,  0.15170f,  1.06965f,
  1.26814f,  -1.93873f, -0.00768f, 1.58309f,
};

static const float av1_tx_type_nn_bias_16x4_ver_layer1[4] = {
  2.34713f,
  1.68667f,
  1.25488f,
  1.69812f,
};

static const NN_CONFIG av1_tx_type_nnconfig_16x4_ver = {
  4,  // num_inputs
  4,  // num_outputs
  1,  // num_hidden_layers
  {
      8,
  },  // num_hidden_nodes
  { av1_tx_type_nn_weights_16x4_ver_layer0,
    av1_tx_type_nn_weights_16x4_ver_layer1 },
  { av1_tx_type_nn_bias_16x4_ver_layer0, av1_tx_type_nn_bias_16x4_ver_layer1 }
};
/******************************************************************************/

// Map tx_size to its corresponding neural net model for tx type prediction.
static const NN_CONFIG *const av1_tx_type_nnconfig_map_hor[] = {
  &av1_tx_type_nnconfig_4x4_hor,   // 4x4 transform
  &av1_tx_type_nnconfig_8x8_hor,   // 8x8 transform
  &av1_tx_type_nnconfig_16x16,     // 16x16 transform
  NULL,                            // 32x32 transform
  NULL,                            // 64x64 transform
  &av1_tx_type_nnconfig_4x8_hor,   // 4x8 transform
  &av1_tx_type_nnconfig_8x4_hor,   // 8x4 transform
  &av1_tx_type_nnconfig_8x16_hor,  // 8x16 transform
  &av1_tx_type_nnconfig_16x8_hor,  // 16x8 transform
  NULL,                            // 16x32 transform
  NULL,                            // 32x16 transform
  NULL,                            // 32x64 transform
  NULL,                            // 64x32 transform
  &av1_tx_type_nnconfig_4x16_hor,  // 4x16 transform
  &av1_tx_type_nnconfig_16x4_hor,  // 16x4 transform
  NULL,                            // 8x32 transform
  NULL,                            // 32x8 transform
  NULL,                            // 16x64 transform
  NULL,                            // 64x16 transform
};

static const NN_CONFIG *const av1_tx_type_nnconfig_map_ver[] = {
  &av1_tx_type_nnconfig_4x4_ver,   // 4x4 transform
  &av1_tx_type_nnconfig_8x8_ver,   // 8x8 transform
  &av1_tx_type_nnconfig_16x16,     // 16x16 transform
  NULL,                            // 32x32 transform
  NULL,                            // 64x64 transform
  &av1_tx_type_nnconfig_4x8_ver,   // 4x8 transform
  &av1_tx_type_nnconfig_8x4_ver,   // 8x4 transform
  &av1_tx_type_nnconfig_8x16_ver,  // 8x16 transform
  &av1_tx_type_nnconfig_16x8_ver,  // 16x8 transform
  NULL,                            // 16x32 transform
  NULL,                            // 32x16 transform
  NULL,                            // 32x64 transform
  NULL,                            // 64x32 transform
  &av1_tx_type_nnconfig_4x16_ver,  // 4x16 transform
  &av1_tx_type_nnconfig_16x4_ver,  // 16x4 transform
  NULL,                            // 8x32 transform
  NULL,                            // 32x8 transform
  NULL,                            // 16x64 transform
  NULL,                            // 64x16 transform
};
#endif  // CONFIG_NN_V2

// Tx split model for 4x8 block.
static const float av1_tx_split_nn_weights_4x8_layer0[8 * 16] = {
  0.068650f,  -0.732073f, -0.040361f, 0.322550f,  -0.021123f, 0.212518f,
  -0.350546f, 0.435987f,  -0.111756f, -0.401568f, 0.069548f,  -0.313000f,
  0.073918f,  -0.373805f, -0.775810f, -0.124753f, 0.181094f,  -0.602641f,
  -0.026219f, -0.350112f, 0.020599f,  -0.311752f, -0.476482f, -0.669465f,
  -0.310921f, 0.348869f,  -0.115984f, 0.154250f,  0.200485f,  -0.016689f,
  0.020392f,  0.413810f,  0.634064f,  -0.627530f, 0.399178f,  -0.012284f,
  0.472030f,  0.091087f,  -0.706100f, -0.447944f, -0.274226f, 0.445656f,
  0.309339f,  0.505522f,  0.038496f,  -0.152809f, 0.408684f,  -0.068151f,
  0.271612f,  0.353233f,  -0.150365f, 0.075212f,  -0.035096f, 0.346615f,
  0.124382f,  0.477072f,  0.216288f,  0.070548f,  -0.106362f, 0.681613f,
  -0.145502f, -0.218631f, -0.099248f, -0.001983f, -0.196819f, -0.969045f,
  0.063009f,  -0.123053f, 0.104875f,  -0.137581f, -0.282933f, -0.003624f,
  -0.315659f, -0.333523f, -0.503000f, -0.100063f, -0.536711f, -0.059978f,
  -0.670248f, -0.353762f, 0.181109f,  0.289715f,  -0.071206f, 0.261141f,
  0.052796f,  -0.114554f, -0.139214f, -0.261380f, 0.075984f,  -0.647925f,
  -0.099528f, -0.677814f, 0.015712f,  -0.389385f, -0.095622f, -0.165117f,
  -0.109454f, -0.175240f, -0.393914f, 0.212330f,  0.037822f,  0.248280f,
  0.180197f,  0.110493f,  -0.525727f, -0.092329f, -0.524029f, -0.407364f,
  -0.542373f, -0.435626f, -0.912194f, 0.062794f,  0.160433f,  0.741485f,
  -0.103659f, -0.119327f, -0.055275f, 0.334358f,  0.014713f,  0.046327f,
  0.831114f,  -0.576682f, 0.354369f,  -0.082088f, 0.452331f,  0.039730f,
  -0.792429f, -0.385862f,
};

static const float av1_tx_split_nn_bias_4x8_layer0[16] = {
  0.238621f,  2.186830f,  1.383035f,  -0.867139f, 1.257119f, -0.351571f,
  -0.240650f, -0.971692f, 2.744843f,  1.116991f,  0.139062f, -0.165332f,
  0.262171f,  -1.598153f, -1.427340f, -1.602306f,
};

static const float av1_tx_split_nn_weights_4x8_layer1[16] = {
  -0.367134f, 1.373058f, -0.897039f, -0.326819f, -0.734030f, -0.290413f,
  -0.501249f, 0.505321f, -0.537692f, -0.767893f, 0.268697f,  0.278987f,
  0.085082f,  0.614986f, 0.847904f,  0.637578f,
};

static const float av1_tx_split_nn_bias_4x8_layer1[1] = {
  0.20586078f,
};

static const NN_CONFIG av1_tx_split_nnconfig_4x8 = {
  8,  // num_inputs
  1,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  {
      av1_tx_split_nn_weights_4x8_layer0,
      av1_tx_split_nn_weights_4x8_layer1,
  },
  {
      av1_tx_split_nn_bias_4x8_layer0,
      av1_tx_split_nn_bias_4x8_layer1,
  },
};
/******************************************************************************/

// Tx split model for 8x8 block.
static const float av1_tx_split_nn_weights_8x8_layer0[144] = {
  0.177983f,  -0.938386f, -0.074460f, -0.221843f, -0.073182f, -0.295155f,
  -0.098202f, -0.279510f, 0.001054f,  -0.119319f, -1.835282f, -0.581507f,
  -1.222222f, -1.049006f, -0.807508f, -0.454252f, -0.774879f, -0.180607f,
  -0.886976f, -0.231971f, -0.824677f, -0.351872f, -1.323819f, 0.235378f,
  0.015331f,  -0.341818f, 0.145549f,  -0.348362f, 0.147647f,  -0.323400f,
  0.047558f,  -0.553025f, -0.295485f, -0.330368f, -0.530605f, -0.407516f,
  0.447740f,  0.782381f,  -0.179164f, -0.584675f, -0.052645f, 0.038656f,
  -0.096783f, 0.038342f,  -0.170762f, -0.405844f, -0.552665f, -0.509866f,
  0.757204f,  -1.296465f, 0.631015f,  0.009265f,  0.646192f,  0.044523f,
  0.653161f,  0.033820f,  0.849639f,  -0.068555f, -1.036085f, -0.511652f,
  0.104693f,  -1.458690f, 0.286051f,  -0.089800f, 0.381564f,  -0.302640f,
  0.304465f,  -0.268706f, 0.432603f,  -0.117914f, -2.070031f, -0.565696f,
  -0.073027f, -1.783570f, -0.318144f, -0.320990f, -0.343966f, -0.140996f,
  -0.322977f, -0.232147f, -0.373210f, -0.158266f, -1.922305f, -0.634373f,
  0.101894f,  -0.221847f, 0.018412f,  -0.423887f, -0.266684f, -0.444930f,
  -0.196237f, 0.106638f,  -0.065834f, -0.538401f, -0.280772f, -0.620348f,
  1.089957f,  -0.799928f, 0.504112f,  -0.165763f, 0.578741f,  -0.172653f,
  0.547316f,  -0.143484f, 0.717220f,  -0.297190f, -1.237854f, -0.074819f,
  -0.977304f, -0.484092f, -0.646427f, -0.451443f, -0.612126f, -0.224475f,
  -0.731608f, -0.257077f, -0.665857f, -0.346742f, -1.216372f, 0.227267f,
  0.231249f,  -1.693073f, -0.035899f, 0.380845f,  -0.058476f, 0.409405f,
  -0.066679f, 0.406731f,  -0.068501f, 0.396748f,  0.639462f,  0.150834f,
  -0.418659f, -1.421931f, 0.101889f,  0.083573f,  0.129746f,  0.134460f,
  0.081185f,  0.127420f,  0.083664f,  0.051096f,  1.361688f,  0.386093f,
};

static const float av1_tx_split_nn_bias_8x8_layer0[12] = {
  4.280443f, 2.218902f, -0.256953f, 3.161431f,  2.082548f, 2.506052f,
  2.563224f, 1.421976f, -1.627813f, -1.436085f, 2.297265f, 1.500469f,
};

static const float av1_tx_split_nn_weights_8x8_layer1[12] = {
  1.178833f,  -0.428527f, -0.078737f, 0.381434f, -0.466895f, -0.901745f,
  -0.766968f, -0.356663f, 0.450146f,  0.509370f, -0.356604f, -0.443506f,
};

static const float av1_tx_split_nn_bias_8x8_layer1[1] = {
  -0.156294f,
};

static const NN_CONFIG av1_tx_split_nnconfig_8x8 = {
  12,  // num_inputs
  1,   // num_outputs
  1,   // num_hidden_layers
  {
      12,
  },  // num_hidden_nodes
  {
      av1_tx_split_nn_weights_8x8_layer0,
      av1_tx_split_nn_weights_8x8_layer1,
  },
  {
      av1_tx_split_nn_bias_8x8_layer0,
      av1_tx_split_nn_bias_8x8_layer1,
  },
};
/******************************************************************************/

// Tx split model for 8x16 block.
static const float av1_tx_split_nn_weights_8x16_layer0[8 * 64] = {
  0.374660f,  0.218905f,  -0.139779f, 0.212141f,  0.056517f,  0.051114f,
  0.042860f,  -0.273258f, -0.340809f, 0.138983f,  -0.216996f, -0.241519f,
  -0.123244f, 0.078577f,  -0.472273f, -0.194201f, 0.125056f,  0.239761f,
  -0.332782f, 0.174782f,  -0.211400f, -0.129795f, 0.062195f,  0.113176f,
  -0.008869f, 0.140764f,  0.059833f,  0.163826f,  0.359293f,  -0.109797f,
  -0.022091f, -0.059536f, -0.188226f, 0.179709f,  0.031386f,  0.164790f,
  0.214364f,  0.198555f,  0.152262f,  -0.242980f, 0.319367f,  -0.136902f,
  0.046524f,  -0.043591f, 0.342178f,  -0.011757f, -0.014286f, 0.072871f,
  -0.278314f, -0.345303f, -0.252103f, -0.107154f, -0.235101f, -0.106739f,
  -0.120865f, -0.160042f, 0.240028f,  0.112902f,  -0.141587f, -0.703012f,
  -0.136591f, 0.318993f,  -0.154417f, -0.054668f, 0.192870f,  0.176166f,
  -0.029965f, 0.266942f,  -0.178384f, 0.038680f,  0.134403f,  -0.002426f,
  0.534825f,  -0.070923f, 0.413281f,  0.418148f,  0.093729f,  0.016454f,
  0.305358f,  -0.040512f, 0.069904f,  -0.227588f, -0.362220f, -0.031604f,
  -0.394901f, 0.071506f,  -0.342833f, -0.142550f, -0.164005f, 0.182600f,
  0.213062f,  0.076805f,  0.278758f,  0.125613f,  -0.035552f, 0.040971f,
  0.182785f,  -0.227961f, -0.105413f, -0.074949f, -0.084629f, -0.254767f,
  0.114657f,  0.047121f,  0.195902f,  0.264759f,  0.017799f,  0.210230f,
  0.150749f,  -0.142142f, 0.182494f,  -0.142415f, -0.259782f, -0.114830f,
  -0.198826f, 0.000061f,  -0.375668f, -0.276656f, -0.373202f, 0.210298f,
  0.422680f,  0.066960f,  0.351106f,  -0.209034f, 0.367195f,  -0.110274f,
  0.115573f,  -0.066642f, -0.389673f, -0.260447f, 0.056949f,  -0.180425f,
  0.069922f,  -0.153506f, -0.097053f, -0.111757f, 0.094069f,  0.144837f,
  -0.052984f, -0.506681f, -0.034474f, 0.279057f,  -0.105025f, 0.006656f,
  -0.125017f, -0.114096f, 0.103153f,  -0.117402f, -0.359472f, 0.072534f,
  0.110291f,  0.003088f,  -0.456897f, 0.038331f,  -0.322298f, 0.113942f,
  -0.119916f, -0.194392f, 0.093167f,  0.193459f,  0.074671f,  0.033602f,
  0.004440f,  -0.179578f, -0.036637f, -0.216172f, -0.296530f, -0.318992f,
  0.319160f,  -0.066218f, 0.291246f,  0.181292f,  0.089914f,  0.025273f,
  0.303128f,  0.019063f,  0.078545f,  -0.396919f, 0.014065f,  -0.122121f,
  0.037107f,  -0.151886f, -0.299392f, -0.172207f, -0.124571f, -0.232553f,
  0.102970f,  -0.225040f, 0.061059f,  -0.258188f, -0.469871f, -0.099607f,
  -0.061524f, -0.213700f, 0.070237f,  -0.289134f, -0.238225f, 0.256403f,
  -0.119344f, 0.067782f,  -0.398983f, -0.123975f, -0.200205f, -0.047038f,
  0.026569f,  0.031037f,  0.094302f,  -0.101239f, 0.433307f,  -0.303612f,
  0.088537f,  -0.164436f, 0.202471f,  -0.048592f, -0.251904f, 0.122577f,
  -0.309874f, -0.263405f, -0.292503f, 0.216589f,  0.035378f,  0.136599f,
  -0.145844f, -0.018211f, 0.174084f,  -0.449941f, -0.001428f, 0.064134f,
  0.039652f,  0.111083f,  -0.246076f, -0.204733f, 0.056559f,  -0.000123f,
  0.104049f,  0.138512f,  -0.128309f, 0.087855f,  0.232784f,  0.247138f,
  0.162766f,  0.154829f,  0.313605f,  -0.164115f, -0.050844f, 0.156549f,
  0.185279f,  -0.238962f, -0.308281f, -0.179592f, -0.193262f, 0.201670f,
  -0.203399f, -0.096831f, -0.127867f, 0.310674f,  -0.008181f, 0.004078f,
  -0.211038f, -0.193480f, -0.185639f, -0.150202f, -0.204858f, -0.240758f,
  0.114268f,  -0.032535f, -0.052403f, -0.234333f, -0.064072f, -0.208444f,
  -0.352853f, -0.224001f, -0.156330f, 0.215436f,  0.171846f,  0.291849f,
  0.108832f,  0.046991f,  -0.127801f, 0.032485f,  0.141493f,  0.123319f,
  -0.057250f, 0.315346f,  -0.061317f, -0.465086f, -0.130179f, -0.217841f,
  -0.239089f, -0.073251f, -0.327718f, 0.054905f,  -0.283169f, -0.028900f,
  0.071450f,  0.270072f,  0.248891f,  0.088052f,  0.253319f,  0.122808f,
  0.175490f,  -0.147805f, 0.089169f,  -0.045457f, -0.330788f, 0.099791f,
  -0.137376f, -0.195977f, -0.350942f, -0.284930f, -0.559037f, 0.030504f,
  0.162554f,  -0.199100f, -0.050453f, -0.131320f, -0.077863f, -0.066253f,
  -0.379723f, -0.424047f, -0.081182f, -0.252261f, -0.102815f, 0.058240f,
  -0.182036f, 0.176772f,  -0.070823f, 0.216054f,  -0.211533f, -0.232992f,
  0.279346f,  0.117984f,  0.236674f,  0.126625f,  -0.046220f, 0.044919f,
  0.278492f,  0.083944f,  0.180512f,  0.217994f,  0.401170f,  -0.064417f,
  0.011636f,  -0.139597f, -0.050020f, -0.268438f, -0.032803f, 0.024908f,
  -0.085713f, -0.012984f, -0.055192f, -0.338657f, 0.045826f,  -0.312849f,
  -0.023393f, -0.168800f, -0.030886f, -0.131816f, -0.253542f, -0.104812f,
  -0.354389f, 0.169464f,  0.094151f,  -0.217122f, -0.456397f, 0.211478f,
  0.219232f,  -0.155519f, -0.353700f, -0.264759f, -0.034709f, 0.034409f,
  -0.148639f, -0.132850f, -0.216791f, -0.118492f, 0.173721f,  -0.144181f,
  0.335028f,  0.176439f,  0.105980f,  0.169390f,  0.155615f,  -0.040618f,
  -0.176029f, 0.155569f,  -0.184833f, -0.171099f, -0.178663f, -0.032051f,
  -0.434334f, 0.092238f,  -0.263103f, 0.061804f,  -0.172957f, 0.005962f,
  -0.100176f, 0.125898f,  0.048092f,  -0.088141f, 0.247196f,  -0.221601f,
  -0.114474f, -0.124410f, -0.156393f, -0.181782f, -0.083562f, 0.034937f,
  0.403401f,  -0.046200f, 0.322259f,  0.219678f,  0.109850f,  0.051837f,
  0.196861f,  -0.019118f, 0.248818f,  -0.137567f, 0.127862f,  0.052293f,
  0.298726f,  0.275788f,  0.015344f,  0.058714f,  0.283691f,  -0.053794f,
  -0.123270f, -0.227761f, -0.141744f, -0.268515f, -0.007189f, -0.242117f,
  -0.252396f, -0.069017f, 0.034803f,  -0.003388f, -0.262577f, 0.062115f,
  -0.298393f, 0.215415f,  -0.153615f, 0.289902f,  0.085886f,  -0.504290f,
  0.077178f,  0.150861f,  -0.228848f, -0.261020f, 0.198204f,  0.162113f,
  0.346418f,  -0.286950f, 0.354756f,  -0.226419f, 0.024720f,  0.208037f,
  0.107286f,  -0.110849f, 0.104415f,  -0.207725f, 0.063932f,  -0.037748f,
  -0.167037f, -0.068282f, 0.320815f,  -0.051884f, 0.099989f,  -0.078388f,
  0.127071f,  0.046675f,  -0.336571f, -0.273080f, 0.264694f,  -0.007352f,
  -0.093828f, 0.094773f,  -0.144434f, 0.091795f,  -0.031615f, 0.056914f,
  0.064673f,  -0.136669f, 0.344734f,  0.225926f,  0.283451f,  -0.068354f,
  0.030572f,  0.180784f,  -0.378047f, -0.092962f, -0.083291f, 0.038970f,
  0.052094f,  -0.017932f, 0.216302f,  -0.184396f, 0.079888f,  0.210406f,
  -0.020627f, 0.244744f,  0.336972f,  -0.182914f, -0.220976f, -0.304225f,
  -0.330974f, -0.370868f, -0.084935f, -0.136489f, -0.210082f, -0.188088f,
  -0.408768f, 0.184693f,
};

static const float av1_tx_split_nn_bias_8x16_layer0[64] = {
  -0.274107f, 0.445751f,  0.234359f,  0.291593f,  0.163298f,  0.183707f,
  -0.548839f, -0.190779f, -0.163346f, -0.669028f, 0.399209f,  -0.354974f,
  0.000000f,  -0.254630f, 0.220149f,  0.371104f,  0.789759f,  0.270300f,
  0.195126f,  -0.206958f, 0.917708f,  -0.256232f, 1.131933f,  1.178944f,
  0.461270f,  0.246169f,  -0.818614f, -0.111986f, 0.759355f,  0.154889f,
  0.470299f,  -1.025250f, 0.678678f,  0.959346f,  -0.164105f, 0.544079f,
  -0.448733f, 0.649221f,  -0.536672f, 0.962758f,  -0.256427f, 0.808664f,
  -0.118694f, 0.684873f,  -0.015635f, -0.046469f, 0.075481f,  0.412647f,
  0.454456f,  -0.107169f, 0.775235f,  -0.261629f, -1.194849f, 0.010093f,
  -0.231289f, 0.658286f,  -0.769320f, 0.564545f,  0.482962f,  -0.131378f,
  -0.255844f, -0.078400f, 0.476752f,  0.643001f,
};

static const float av1_tx_split_nn_weights_8x16_layer1[64] = {
  -0.145065f, -0.145101f, 0.174786f,  0.196692f,  0.102025f,  -0.087735f,
  0.386353f,  -0.660539f, -0.183940f, 0.490045f,  -0.276404f, -0.145669f,
  0.209846f,  -0.085574f, -0.156821f, -0.377450f, -0.950010f, 0.450709f,
  -0.108545f, -0.261181f, 1.435606f,  -0.176621f, -1.158548f, 2.035680f,
  0.218069f,  -0.138629f, 0.305958f,  -0.277194f, -0.602468f, 0.203873f,
  0.120720f,  0.216095f,  -0.434502f, -0.579746f, -0.239450f, 0.755529f,
  0.545643f,  0.232091f,  0.330169f,  0.988136f,  -0.070465f, -0.345584f,
  -0.162455f, -0.617064f, 0.123881f,  -0.201098f, 0.222756f,  0.112932f,
  0.048647f,  -0.147890f, 0.394584f,  -0.262148f, 0.280564f,  -0.195432f,
  -0.047515f, 1.133410f,  0.255415f,  -0.299032f, -0.397807f, -0.153246f,
  -0.256734f, 0.177370f,  0.213522f,  -0.530158f,
};

static const float av1_tx_split_nn_bias_8x16_layer1[1] = {
  0.14910713f,
};

static const NN_CONFIG av1_tx_split_nnconfig_8x16 = {
  8,  // num_inputs
  1,  // num_outputs
  1,  // num_hidden_layers
  {
      64,
  },  // num_hidden_nodes
  {
      av1_tx_split_nn_weights_8x16_layer0,
      av1_tx_split_nn_weights_8x16_layer1,
  },
  {
      av1_tx_split_nn_bias_8x16_layer0,
      av1_tx_split_nn_bias_8x16_layer1,
  },
};
/******************************************************************************/

// Tx split model for 16x16 block.
static const float av1_tx_split_nn_weights_16x16_layer0[12 * 24] = {
  -0.177215f, -0.297166f, 0.299924f,  0.207878f,  0.216871f,  0.173264f,
  0.295464f,  0.048395f,  0.154731f,  0.305880f,  0.056787f,  -0.166617f,
  0.115653f,  -0.529477f, -0.073995f, -0.211746f, -0.018169f, 0.000788f,
  -0.024940f, -0.007055f, 0.001392f,  0.021678f,  -1.594600f, -0.099593f,
  0.332930f,  0.103574f,  0.158249f,  0.182601f,  0.332665f,  0.226207f,
  -0.139566f, 0.185531f,  0.099074f,  -0.185654f, -0.203121f, -0.285678f,
  -0.313453f, -0.294452f, -0.143707f, -0.031265f, -0.453030f, -0.061874f,
  -0.066150f, -0.099058f, -0.458879f, 0.127544f,  0.338314f,  -0.161350f,
  0.030091f,  -0.075528f, 0.004320f,  0.353690f,  -0.013480f, -0.420402f,
  -0.004659f, -0.329401f, -0.001745f, 0.227384f,  -0.055183f, 0.121405f,
  0.160340f,  0.143603f,  -0.221813f, 0.079107f,  -0.657639f, -0.084348f,
  -0.303414f, 0.046774f,  -0.367679f, 0.060005f,  0.168645f,  0.084421f,
  -0.133625f, 0.301375f,  0.079412f,  -0.419303f, 0.017235f,  0.068637f,
  0.018384f,  -0.428325f, -0.019753f, 0.149444f,  -0.474836f, -0.287162f,
  0.198083f,  0.028292f,  -0.299092f, -0.005849f, -0.256245f, 0.233277f,
  -0.217561f, -0.264003f, 0.269411f,  0.207032f,  -0.339411f, -0.198431f,
  -0.028521f, 0.158076f,  0.177116f,  0.345702f,  -0.145132f, 0.064623f,
  -0.090867f, 0.288816f,  -0.263198f, -0.071028f, -0.044546f, 0.380017f,
  -0.014100f, -0.271192f, -0.318559f, 0.129015f,  -0.050314f, -0.093355f,
  -0.578498f, 0.099090f,  -0.133080f, -0.029975f, -0.059828f, -0.157765f,
  -0.321153f, -0.343671f, -0.242959f, 0.128304f,  0.017170f,  0.072787f,
  -0.475838f, -0.003806f, -0.068615f, 0.150556f,  -0.159903f, -0.416513f,
  0.218794f,  -0.290456f, -0.084569f, -0.170014f, -0.044414f, -0.153069f,
  -0.077329f, -0.089747f, -0.096526f, 0.537952f,  0.134725f,  -0.006469f,
  -0.323335f, -0.168183f, -0.107163f, -0.139954f, 0.011286f,  -0.021712f,
  -0.513992f, 0.259135f,  -0.319808f, 0.077811f,  0.104613f,  0.370571f,
  0.185244f,  0.065530f,  -0.091098f, -0.573741f, 0.111934f,  0.437417f,
  -0.123691f, 0.220641f,  -0.024783f, -0.149460f, -0.354185f, -0.134127f,
  0.038015f,  -0.380596f, 0.250980f,  0.142208f,  0.135170f,  -0.131129f,
  -0.357556f, -0.530945f, 0.159672f,  -0.147025f, -0.377829f, -0.504508f,
  -0.492870f, 0.020753f,  0.142818f,  0.025172f,  0.086140f,  0.091283f,
  0.087491f,  -0.186415f, 0.177785f,  -0.195121f, -1.191148f, -0.477102f,
  0.023371f,  0.227004f,  -0.023502f, -0.242913f, -0.074398f, -0.153480f,
  0.162900f,  0.415509f,  -0.162565f, -0.131709f, -0.258852f, -0.252027f,
  -0.080845f, -0.330274f, 0.021874f,  0.232398f,  0.069277f,  0.220567f,
  -0.024237f, -0.366771f, 0.081673f,  -0.429906f, -0.302170f, 0.061045f,
  0.352777f,  -0.230376f, 0.408153f,  0.064758f,  0.142051f,  0.007219f,
  0.622878f,  0.212577f,  0.036489f,  0.081150f,  -0.284767f, 0.107763f,
  -0.529786f, -0.072190f, -0.300421f, -0.287959f, -0.568900f, 0.011547f,
  -0.131696f, -0.356854f, -0.587962f, -0.026598f, 0.405829f,  0.057565f,
  0.414265f,  -0.159155f, 0.221456f,  0.146314f,  0.265776f,  -0.006516f,
  0.473978f,  -0.186431f, 0.288672f,  -0.060437f, 0.083380f,  -0.205641f,
  0.360016f,  0.222041f,  0.420011f,  0.024579f,  0.377546f,  0.250380f,
  -0.069900f, 0.296743f,  0.073532f,  -0.243225f, -0.374987f, -0.387288f,
  -0.237255f, -0.287013f, 0.417831f,  -0.252988f, -0.257652f, -0.066775f,
  -0.253926f, 0.057841f,  0.346133f,  -0.157797f, -0.406028f, -0.286893f,
  0.274507f,  -0.452561f, 0.143381f,  -0.097755f, 0.021242f,  0.034561f,
  0.044115f,  0.004065f,  0.066729f,  0.043558f,  0.102991f,  -0.477574f,
};

static const float av1_tx_split_nn_bias_16x16_layer0[24] = {
  -0.479033f, 1.467402f,  -0.366291f, 0.372511f,  0.715322f,  -0.605500f,
  0.176848f,  0.032318f,  0.237429f,  -0.046047f, 0.452082f,  0.451805f,
  -0.822845f, 0.636762f,  -0.057350f, 1.163978f,  0.728287f,  0.603654f,
  -0.245519f, -0.893569f, -1.428185f, 0.808870f,  -0.076159f, 1.231976f,
};

static const float av1_tx_split_nn_weights_16x16_layer1[24] = {
  -0.176161f, 1.670188f, -0.180755f, -0.321326f, 0.249728f,  -0.170504f,
  -0.538432f, 0.033893f, 0.149842f,  0.404140f,  -0.377812f, 0.338838f,
  -0.176091f, 0.249844f, -0.362533f, 1.412460f,  0.196862f,  0.278194f,
  -0.140444f, 0.297746f, 0.172533f,  0.116470f,  -0.151656f, -0.603250f,
};

static const float av1_tx_split_nn_bias_16x16_layer1[1] = {
  0.184803f,
};

static const NN_CONFIG av1_tx_split_nnconfig_16x16 = {
  12,  // num_inputs
  1,   // num_outputs
  1,   // num_hidden_layers
  {
      24,
  },  // num_hidden_nodes
  {
      av1_tx_split_nn_weights_16x16_layer0,
      av1_tx_split_nn_weights_16x16_layer1,
  },
  {
      av1_tx_split_nn_bias_16x16_layer0,
      av1_tx_split_nn_bias_16x16_layer1,
  },
};
/******************************************************************************/

// Tx split model for 32x32 block.
static const float av1_tx_split_nn_weights_32x32_layer0[12 * 32] = {
  -0.439303f, 0.004813f,  -0.365052f, -0.116868f, -0.356716f, -0.196537f,
  -0.196770f, -0.076096f, 0.357004f,  -0.044909f, -0.112910f, -0.129081f,
  0.156725f,  -0.386346f, 0.038971f,  0.160696f,  0.204923f,  -0.384333f,
  -0.319546f, 0.028179f,  -0.250524f, -0.289669f, -0.284138f, -0.258963f,
  -0.180854f, -0.000807f, -0.029620f, -0.353134f, 0.212408f,  0.141414f,
  0.303016f,  0.098066f,  0.482455f,  0.036069f,  -0.166279f, 0.210119f,
  -0.086337f, -0.023550f, -0.250796f, -0.183945f, -0.393856f, 0.170608f,
  -0.306403f, 0.026318f,  -0.277296f, 0.092684f,  -0.033584f, -0.018371f,
  -0.025043f, -0.257659f, -0.139163f, -0.206949f, -0.190105f, 0.028053f,
  0.361851f,  -0.364726f, -0.096771f, -0.184166f, -0.433228f, -0.182191f,
  -0.097051f, 0.259172f,  0.016432f,  0.259358f,  0.145059f,  0.037196f,
  0.091581f,  -0.219644f, 0.140384f,  -0.446837f, -0.234531f, 0.149508f,
  -0.083429f, 0.186189f,  -0.099890f, -0.111277f, 0.495214f,  0.085053f,
  -0.266613f, -0.051366f, 0.148593f,  0.111875f,  0.077787f,  -0.371653f,
  -0.146157f, -0.229235f, 0.076203f,  0.488975f,  0.096771f,  -0.009483f,
  0.192985f,  0.246273f,  -0.192671f, -0.557890f, -0.292650f, -0.088907f,
  -0.106892f, -0.329659f, 0.012105f,  -0.359326f, 0.170723f,  -0.004357f,
  0.171593f,  -0.478768f, -0.236016f, -0.035077f, 0.133731f,  0.137962f,
  -0.397926f, -0.155164f, -0.276709f, -0.186602f, -0.258301f, 0.036965f,
  -0.649359f, 0.127605f,  0.097930f,  0.182775f,  -0.313324f, 0.053349f,
  0.204203f,  -0.222948f, -0.059008f, -0.049759f, -0.056848f, 0.087497f,
  -0.039987f, -0.055042f, -0.041623f, -0.078424f, -0.317291f, -0.191398f,
  0.632147f,  0.221825f,  0.268394f,  -0.096357f, 0.442545f,  -0.007117f,
  -0.036125f, 0.000525f,  0.088092f,  -0.203653f, 0.086925f,  0.439141f,
  0.329889f,  -0.370050f, -0.194306f, -0.207430f, 0.132779f,  -0.217614f,
  -0.039444f, -0.053019f, -0.260725f, -0.116563f, -0.271048f, 0.283737f,
  -0.007300f, 0.062257f,  -0.347865f, -0.296767f, -0.359123f, 0.230459f,
  -0.189117f, -0.087622f, -0.561091f, 0.184182f,  -0.044980f, 0.012643f,
  0.241672f,  0.050272f,  -0.204851f, -0.159285f, -0.064081f, -0.118666f,
  -0.269471f, 0.231668f,  0.135749f,  -0.131162f, 0.062760f,  0.100949f,
  0.074967f,  -0.056918f, 0.251707f,  0.034098f,  0.341290f,  -0.105027f,
  0.313246f,  -0.092679f, -0.014632f, -0.390967f, 0.136881f,  -0.241554f,
  0.097674f,  0.110832f,  -0.390245f, 0.017654f,  -0.506222f, 0.065252f,
  0.244834f,  -0.171352f, -0.331702f, 0.111043f,  0.125217f,  -0.058116f,
  -0.382595f, -0.052545f, 0.114261f,  -0.493617f, 0.243984f,  -0.171053f,
  0.165009f,  -0.063020f, 0.096502f,  0.341339f,  -0.013443f, 0.056372f,
  0.339284f,  0.398376f,  0.389409f,  0.257252f,  0.517368f,  0.078856f,
  0.087716f,  -0.171092f, 0.227461f,  0.125307f,  -0.054423f, -0.143161f,
  0.224041f,  -0.086477f, -0.092548f, 0.072392f,  -0.061608f, 0.258347f,
  0.147033f,  -0.478244f, -0.204869f, 0.038552f,  -0.144563f, 0.224087f,
  -0.296705f, 0.153889f,  -0.064624f, 0.085265f,  -0.103826f, 0.127971f,
  0.019965f,  0.111937f,  -0.074187f, -0.029518f, -0.127305f, -0.012210f,
  0.042714f,  0.070052f,  -0.202360f, 0.348144f,  -0.132097f, -0.209585f,
  -0.248286f, -0.065774f, -0.089482f, -0.133226f, 0.325430f,  -0.013468f,
  -0.406090f, -0.144936f, 0.208620f,  0.343445f,  -0.059639f, 0.114857f,
  -0.069431f, -0.218725f, 0.190575f,  -0.368101f, 0.030030f,  0.062815f,
  -0.239369f, -0.537852f, 0.022487f,  0.023038f,  0.190788f,  0.040123f,
  -0.004304f, 0.060749f,  -0.108929f, 0.136796f,  -0.542875f, -0.227074f,
  -0.182244f, 0.082559f,  0.019149f,  0.178854f,  0.120284f,  0.009070f,
  0.068268f,  -0.544822f, 0.120536f,  0.354028f,  -0.119890f, -0.122055f,
  -0.405335f, 0.122341f,  -0.304412f, 0.062405f,  -0.302568f, -0.276505f,
  -0.120915f, -0.221841f, 0.282007f,  -0.253971f, 0.059517f,  -0.144976f,
  0.149391f,  -0.047355f, -0.167742f, -0.392333f, -0.041132f, 0.342135f,
  0.017485f,  0.021038f,  -0.023728f, -0.192181f, -0.103996f, 0.092873f,
  -0.114365f, -0.397732f, -0.065421f, 0.053084f,  0.035201f,  0.053019f,
  -0.105377f, -0.039500f, 0.131904f,  -0.123911f, -0.390328f, -0.125198f,
  -0.000126f, 0.014864f,  -0.220187f, 0.084056f,  -0.492155f, -0.164979f,
  0.133592f,  0.121519f,  -0.240813f, 0.186680f,  0.118673f,  0.235006f,
  -0.239894f, -0.185759f, -0.336992f, 0.209620f,  -0.298845f, 0.127803f,
  -0.083992f, 0.194340f,  -0.245378f, 0.212308f,  0.142512f,  -0.163324f,
  0.383495f,  0.291065f,  0.286620f,  -0.239957f, 0.225127f,  -0.174424f,
  0.297231f,  -0.045434f, 0.156444f,  -0.184273f, -0.204567f, 0.202551f,
  0.370019f,  -0.073910f, 0.344897f,  0.063100f,  0.338547f,  -0.099145f,
  0.391863f,  -0.214244f, -0.241734f, -0.281851f, -0.035133f, -0.153157f,
};

static const float av1_tx_split_nn_bias_32x32_layer0[32] = {
  0.143343f,  -0.021982f, -0.314939f, 0.170867f,  -0.081248f, 0.125758f,
  -0.355762f, 0.279798f,  1.027712f,  -0.434660f, 1.072005f,  0.668893f,
  -0.031216f, -0.528650f, 0.328349f,  0.543645f,  -0.188810f, 0.221110f,
  -1.638637f, 0.058045f,  -1.731105f, -0.444284f, 0.513693f,  0.890025f,
  0.160288f,  0.393312f,  0.332856f,  -0.080767f, 0.299822f,  0.235876f,
  0.254942f,  -0.017796f,
};

static const float av1_tx_split_nn_weights_32x32_layer1[32] = {
  -0.090326f, -0.267553f, -0.026071f, 0.100912f,  0.279137f,  0.079064f,
  -0.074885f, 0.053804f,  0.736810f,  -0.031693f, -0.970514f, 0.174069f,
  0.095940f,  -0.065047f, 0.052911f,  0.176728f,  -0.058274f, 0.148364f,
  -0.162210f, 0.093875f,  -0.367663f, 0.020876f,  0.137280f,  -1.099116f,
  0.146854f,  0.075590f,  0.228534f,  0.141993f,  0.072143f,  0.101421f,
  -0.068547f, -0.154148f,
};

static const float av1_tx_split_nn_bias_32x32_layer1[1] = {
  0.316622f,
};

static const NN_CONFIG av1_tx_split_nnconfig_32x32 = {
  12,  // num_inputs
  1,   // num_outputs
  1,   // num_hidden_layers
  {
      32,
  },  // num_hidden_nodes
  {
      av1_tx_split_nn_weights_32x32_layer0,
      av1_tx_split_nn_weights_32x32_layer1,
  },
  {
      av1_tx_split_nn_bias_32x32_layer0,
      av1_tx_split_nn_bias_32x32_layer1,
  },
};
/******************************************************************************/

// Tx split model for 64x64 block.
static const float av1_tx_split_nn_weights_64x64_layer0[12 * 32] = {
  -0.006828f, 0.149944f,  -0.017614f, -0.044599f, -0.024517f, 0.507698f,
  0.001039f,  0.037164f,  0.015091f,  -0.306620f, -0.162047f, -0.369440f,
  0.396310f,  0.087121f,  0.208609f,  -0.083068f, 0.493774f,  0.217682f,
  0.377393f,  0.172879f,  0.397422f,  0.078919f,  0.741350f,  0.064169f,
  -0.099989f, -0.192983f, -0.278230f, -0.310048f, -0.439965f, -0.226698f,
  -0.436596f, -0.007551f, -0.396721f, 0.153570f,  -0.190838f, -0.071869f,
  0.048799f,  -0.301301f, -0.005015f, 0.500480f,  -0.030622f, -0.559095f,
  -0.032634f, -0.054160f, -0.056979f, -0.456545f, 0.306536f,  -0.411323f,
  -0.005366f, -0.069496f, 0.019990f,  0.327931f,  -0.002516f, 0.393190f,
  0.001759f,  0.035093f,  -0.030302f, -0.528984f, 0.174781f,  0.241462f,
  -0.415427f, -0.164502f, 0.143065f,  -0.122595f, 0.082049f,  -0.143346f,
  0.055642f,  -0.124701f, 0.004050f,  -0.216235f, -2.681730f, 0.101658f,
  0.381239f,  0.465936f,  0.331154f,  0.301708f,  -0.360171f, 0.054886f,
  -0.118658f, 0.287921f,  0.277859f,  0.203784f,  0.247809f,  0.656924f,
  -0.354628f, 0.315081f,  0.105108f,  -0.510179f, 0.059267f,  0.061386f,
  0.076423f,  0.347119f,  0.100134f,  0.028402f,  -0.118621f, -0.238689f,
  0.080141f,  -0.138863f, 0.009009f,  -0.100526f, -0.138875f, 0.066992f,
  0.005949f,  0.564336f,  0.046994f,  0.004655f,  0.366047f,  0.014695f,
  -0.146928f, -0.024665f, -0.440357f, -0.109395f, 0.527231f,  -0.020925f,
  -0.227236f, -0.068141f, 0.282009f,  0.040192f,  -0.267100f, 0.229228f,
  0.133861f,  0.338706f,  -0.030178f, -0.040919f, -0.026343f, -0.330338f,
  -0.066931f, -0.110580f, -0.072056f, 0.599457f,  -0.020738f, 0.169200f,
  0.836240f,  -0.157548f, 0.386273f,  0.002404f,  0.329410f,  -0.007020f,
  0.351705f,  -0.041259f, 0.388861f,  0.003899f,  0.582627f,  0.023572f,
  0.409912f,  -0.158472f, 0.536383f,  0.525093f,  0.604247f,  0.439159f,
  0.692832f,  0.046272f,  0.590367f,  -0.082166f, 0.262357f,  0.478671f,
  0.031935f,  0.042675f,  0.120002f,  0.398616f,  -0.078967f, 0.227986f,
  -0.044679f, 0.151061f,  -0.085564f, 0.220205f,  -0.265606f, -0.203623f,
  0.204719f,  -0.125922f, 0.038544f,  -0.269379f, 0.025866f,  0.109967f,
  0.019064f,  -0.237297f, -0.309746f, -0.329118f, -0.278368f, -0.063859f,
  0.278496f,  0.018620f,  0.209971f,  0.296250f,  0.142850f,  0.288689f,
  0.137084f,  0.130517f,  0.128171f,  -0.155396f, -0.008449f, -0.099845f,
  0.173455f,  -0.059909f, -0.147318f, 0.102851f,  -0.251389f, -0.001448f,
  0.103907f,  0.297273f,  -0.027846f, 0.028260f,  -0.382601f, 0.346695f,
  -0.601641f, 0.162366f,  -0.477495f, -0.042731f, -0.387871f, -0.051791f,
  -0.401498f, -0.048446f, -0.456270f, -0.062287f, 0.493919f,  0.003008f,
  0.099917f,  -0.358525f, -0.094903f, -0.022811f, -0.062259f, 0.019455f,
  -0.050644f, 0.020041f,  -0.132912f, -0.061578f, -3.083691f, -0.014961f,
  -0.129115f, -0.710559f, 0.157213f,  -0.844037f, -0.121991f, -0.943386f,
  -0.231269f, -0.003462f, 0.331478f,  -0.132703f, -1.285993f, -0.120957f,
  -0.373755f, -0.322609f, 0.309059f,  -0.131523f, -0.118334f, -0.063805f,
  -0.104251f, 0.012166f,  -0.094699f, -0.283753f, 0.128168f,  -0.526929f,
  -0.050331f, 0.186153f,  0.005913f,  -0.221236f, 0.036363f,  0.160909f,
  -0.001342f, -0.382749f, 0.037820f,  0.281689f,  -0.024275f, 0.028854f,
  0.318291f,  0.318526f,  0.035778f,  0.034031f,  0.189663f,  -0.293367f,
  0.082022f,  0.127923f,  0.078866f,  -0.081361f, -0.268117f, 0.246675f,
  0.248605f,  -0.215479f, -0.073084f, 0.496140f,  -0.067327f, 0.396237f,
  -0.120739f, 0.033752f,  -0.044120f, -0.218941f, -0.028078f, 0.195132f,
  -0.040400f, 0.281604f,  -0.100471f, 0.415207f,  -0.258503f, -0.429749f,
  0.150569f,  -0.010859f, 0.136448f,  0.026589f,  0.148466f,  0.110764f,
  0.380967f,  0.009177f,  0.103075f,  0.116417f,  0.226273f,  -0.327746f,
  0.169346f,  0.284553f,  -0.094986f, 0.312745f,  -0.147840f, 0.025062f,
  -0.494482f, 0.112388f,  -0.213962f, 0.107050f,  -0.433371f, -0.096276f,
  -0.244835f, -0.003518f, -0.459148f, -0.145080f, 0.017150f,  0.042846f,
  -0.237479f, 0.104746f,  0.158677f,  0.358937f,  0.099921f,  0.277109f,
  0.012410f,  -0.062897f, 0.116130f,  0.255309f,  0.341628f,  0.145002f,
  -0.429344f, -0.016433f, -0.068985f, 0.285194f,  -0.286719f, -0.018298f,
  -0.179369f, -0.194655f, -0.165380f, 0.026071f,  -0.428268f, -0.379929f,
  -0.727543f, 0.179610f,  -0.963979f, -0.042026f, -0.616202f, 0.133401f,
  -0.784966f, 0.061205f,  -0.713357f, 0.129795f,  0.120512f,  -0.339545f,
  0.353557f,  0.114906f,  -0.329813f, -0.209987f, 0.085410f,  0.214313f,
  -0.122082f, 0.335770f,  -0.020937f, 0.202456f,  0.289023f,  -0.421186f,
  0.337905f,  0.407663f,  0.132771f,  0.071734f,  0.213914f,  0.128595f,
  0.302659f,  -0.209501f, 0.217756f,  0.253079f,  -0.089505f, -0.205614f,
};

static const float av1_tx_split_nn_bias_64x64_layer0[32] = {
  0.296914f,  -1.826816f, 0.346130f,  0.969520f,  -0.528154f, 1.175862f,
  -0.075985f, -0.097323f, -0.233059f, 0.004846f,  0.401279f,  -2.272435f,
  0.086257f,  0.414162f,  -0.194786f, -0.233887f, -0.113215f, -2.453546f,
  0.861214f,  0.298361f,  0.267397f,  -0.158557f, -0.119911f, -0.098134f,
  -0.339263f, 0.385871f,  -0.678123f, 0.263218f,  0.251611f,  -1.155773f,
  -0.365437f, 0.229255f,
};

static const float av1_tx_split_nn_weights_64x64_layer1[32] = {
  0.502104f,  -0.708023f, 0.419648f,  1.583418f,  0.419355f,  -1.462981f,
  -0.439623f, 0.405691f,  0.823257f,  0.061654f,  0.750875f,  0.775031f,
  -0.387909f, 0.447385f,  0.284690f,  0.353262f,  -0.224347f, 0.832864f,
  -1.708491f, -1.042447f, -0.272829f, 0.540640f,  0.310509f,  0.723745f,
  0.245592f,  -0.218417f, -0.597987f, -0.362301f, 0.702217f,  -0.692614f,
  0.207812f,  0.513560f,
};

static const float av1_tx_split_nn_bias_64x64_layer1[1] = { -0.2307045f };

static const NN_CONFIG av1_tx_split_nnconfig_64x64 = {
  12,  // num_inputs
  1,   // num_outputs
  1,   // num_hidden_layers
  {
      32,
  },  // num_hidden_nodes
  {
      av1_tx_split_nn_weights_64x64_layer0,
      av1_tx_split_nn_weights_64x64_layer1,
  },
  {
      av1_tx_split_nn_bias_64x64_layer0,
      av1_tx_split_nn_bias_64x64_layer1,
  },
};
/******************************************************************************/

// Tx split model for 4x16 block.
static const float av1_tx_split_nn_weights_4x16_layer0[8 * 16] = {
  -1.344184f, -1.454625f, -0.703110f, -0.140570f, -0.841536f, -0.068131f,
  -2.128968f, -0.655518f, 0.432180f,  0.879752f,  -0.222211f, 0.061615f,
  -0.230969f, 0.569496f,  1.424188f,  0.598063f,  -0.436005f, -0.737606f,
  -0.137875f, -0.085730f, -0.076512f, -0.583101f, -0.937377f, -0.203556f,
  -0.215797f, -0.015361f, -0.124098f, -0.411917f, 0.340441f,  -0.331752f,
  -0.472607f, -0.097714f, -0.930572f, -1.354713f, -0.550724f, 0.176212f,
  -0.636060f, 0.183271f,  -0.610212f, 0.345895f,  -1.100906f, -1.605713f,
  0.111888f,  -0.140937f, 0.063013f,  -0.013315f, -0.273472f, -0.255870f,
  1.200328f,  0.274002f,  1.005776f,  0.322392f,  1.222373f,  0.158227f,
  0.408810f,  0.145022f,  0.139842f,  -1.249412f, 0.286672f,  -0.635699f,
  0.312562f,  -0.495606f, -1.117034f, -0.085107f, -0.097484f, -0.341521f,
  -0.132199f, -0.863055f, 0.217579f,  -1.161425f, -0.302087f, -1.357271f,
  -0.520724f, -1.211069f, -1.048729f, -0.333087f, -1.171527f, -0.280824f,
  -2.057684f, -0.228755f, 0.606278f,  0.101198f,  -0.314847f, -1.303255f,
  -0.294964f, 1.301923f,  0.041712f,  0.077593f,  -1.152746f, 0.495315f,
  -0.751566f, 0.230249f,  -0.840661f, 0.100731f,  1.346269f,  0.649898f,
  -1.432258f, -0.456710f, -1.018123f, -0.348559f, -1.225226f, -0.170717f,
  -0.354072f, 0.068292f,  -0.234168f, 0.277503f,  0.179134f,  0.907420f,
  0.354626f,  -0.627210f, 0.905779f,  0.512612f,  0.161190f,  -0.843177f,
  0.014953f,  -0.354983f, 0.011116f,  -0.429598f, -1.017138f, -0.211432f,
  0.941840f,  -0.281747f, 0.957776f,  -0.541914f, 1.041880f,  -0.433580f,
  -1.416451f, -0.166467f,
};

static const float av1_tx_split_nn_bias_4x16_layer0[16] = {
  3.086118f,  -3.235095f, 4.830956f,  -0.165706f, 0.955031f,  4.055783f,
  -0.311489f, 4.660205f,  -0.576277f, -0.248111f, -0.790519f, -1.686412f,
  -1.191704f, -3.800073f, 4.121552f,  -1.399397f,
};

static const float av1_tx_split_nn_weights_4x16_layer1[16] = {
  -0.758677f, 0.388776f,  0.439906f,  0.011390f, -0.084319f, -0.667969f,
  -0.467316f, -0.875491f, -0.160668f, 0.805292f, 0.114393f,  -0.549682f,
  0.462109f,  0.343315f,  1.092593f,  0.483152f,
};

static const float av1_tx_split_nn_bias_4x16_layer1[1] = {
  0.8205083f,
};

static const NN_CONFIG av1_tx_split_nnconfig_4x16 = {
  8,  // num_inputs
  1,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  {
      av1_tx_split_nn_weights_4x16_layer0,
      av1_tx_split_nn_weights_4x16_layer1,
  },
  {
      av1_tx_split_nn_bias_4x16_layer0,
      av1_tx_split_nn_bias_4x16_layer1,
  },
};
/******************************************************************************/

// Tx split model for 16x32 block.
static const float av1_tx_split_nn_weights_16x32_layer0[8 * 32] = {
  0.180713f,  0.033211f,  0.607561f,  0.138642f,  0.637204f,  -0.000940f,
  0.012630f,  0.358109f,  0.022238f,  0.190418f,  0.079088f,  0.065925f,
  0.038242f,  0.162380f,  -0.122728f, 0.379382f,  -0.303283f, -0.327550f,
  0.029120f,  -0.284553f, 0.269588f,  -0.309805f, -0.241036f, -0.161103f,
  -0.304887f, 0.239843f,  -0.149146f, 0.311234f,  -0.073640f, -0.132718f,
  0.178901f,  0.474712f,  0.020280f,  0.063685f,  -0.609170f, -0.013658f,
  -0.338074f, 0.250429f,  0.082978f,  -0.186315f, -0.788959f, 0.039859f,
  -0.426461f, -0.001524f, -0.447211f, 0.378102f,  0.315617f,  0.017428f,
  0.745494f,  -0.219024f, 0.512836f,  0.200522f,  0.680449f,  0.313686f,
  -0.412569f, -0.132927f, 0.631120f,  0.042735f,  0.336153f,  0.044772f,
  0.432606f,  0.175681f,  -0.634411f, -0.073509f, -0.040643f, -0.559260f,
  -0.104034f, -0.570495f, -0.247365f, 0.063256f,  -0.582021f, -0.492585f,
  -0.194955f, -0.207934f, -0.506627f, 0.021743f,  -0.416518f, 0.320876f,
  0.115889f,  0.149399f,  -0.229376f, 0.095505f,  0.115191f,  -0.471921f,
  0.113068f,  0.343684f,  -0.036831f, 0.021240f,  0.295112f,  0.031166f,
  0.448201f,  -0.132241f, 0.164032f,  0.355572f,  0.072154f,  0.017335f,
  -0.046113f, 0.178719f,  -0.026881f, -0.242590f, 0.055073f,  -0.012958f,
  0.077904f,  0.351356f,  0.107655f,  0.260568f,  -0.080052f, -0.197553f,
  0.085763f,  0.263416f,  -0.327741f, 0.158855f,  0.056899f,  -0.162121f,
  0.339518f,  -0.571204f, 0.264966f,  -0.252214f, -0.202560f, -0.134213f,
  -0.330188f, 0.009470f,  -0.468376f, -0.065240f, -0.307957f, 0.116479f,
  -0.222238f, -0.458716f, 0.186493f,  -0.391415f, 0.118649f,  -0.104653f,
  -0.259958f, -0.332081f, -0.403785f, -0.050147f, -0.573511f, 0.177117f,
  -0.598358f, 0.164947f,  -0.119694f, -0.058520f, 0.203829f,  -0.267404f,
  -0.048202f, -0.600006f, 0.181594f,  -0.731805f, 0.146417f,  -0.687148f,
  -1.210525f, -0.450101f, -0.620635f, 0.208825f,  -0.611357f, 0.112202f,
  -0.309468f, -0.323545f, 0.357770f,  0.308061f,  0.553199f,  0.049012f,
  0.530093f,  -0.208597f, 0.607882f,  -0.058120f, -0.527634f, 0.018136f,
  0.060753f,  0.118894f,  0.175649f,  0.014731f,  0.428318f,  -0.106465f,
  -0.119077f, 0.080179f,  0.524997f,  0.368286f,  0.528286f,  0.213659f,
  0.639286f,  0.195079f,  -0.049815f, -0.092008f, -0.302958f, 0.298149f,
  -0.173870f, -0.145205f, -0.233589f, -0.303368f, 0.141275f,  0.325622f,
  -0.115293f, 0.155188f,  0.047225f,  0.231050f,  -0.167447f, 0.349754f,
  0.295544f,  -0.319466f, 0.095144f,  0.174612f,  -0.194652f, 0.305915f,
  -0.239008f, -0.037453f, 0.280696f,  0.125850f,  0.749196f,  -0.101919f,
  0.791808f,  -0.236811f, 0.064157f,  0.032865f,  -0.225911f, 0.350384f,
  0.723183f,  -0.103992f, 0.483085f,  -0.123992f, 0.602138f,  0.023895f,
  -0.692601f, -0.118387f, 0.162527f,  0.145178f,  -0.184702f, -0.017753f,
  -0.159436f, 0.124105f,  -0.131067f, 0.310275f,  0.151499f,  0.138924f,
  0.537459f,  0.263212f,  0.615896f,  0.281255f,  0.021293f,  -0.473459f,
  0.210145f,  -0.056682f, 0.063658f,  0.377254f,  -0.314410f, -0.183487f,
  0.300384f,  0.328471f,  0.164694f,  -0.159272f, -0.160942f, -0.502861f,
  -0.129147f, 0.045916f,  -0.606865f, -0.101378f,
};

static const float av1_tx_split_nn_bias_16x32_layer0[32] = {
  0.051664f,  -0.212487f, -0.077596f, -0.818467f, 0.638475f,  -0.759937f,
  0.157198f,  0.989640f,  1.586035f,  0.431144f,  0.041605f,  0.543085f,
  0.498379f,  0.320504f,  0.134233f,  0.670979f,  -0.105562f, -1.574879f,
  1.261812f,  -0.287530f, -1.610592f, 0.730899f,  -0.894240f, -0.657790f,
  0.270806f,  -0.181708f, 0.298578f,  0.817240f,  -0.221508f, -0.201771f,
  -0.294389f, 1.456413f,
};

static const float av1_tx_split_nn_weights_16x32_layer1[32] = {
  1.208914f,  0.324728f,  0.383352f,  -0.874321f, 0.172565f,  -0.580927f,
  -0.432927f, 0.433698f,  -0.801935f, 0.672028f,  0.563493f,  0.260077f,
  -0.200557f, -0.121638f, 0.530735f,  -0.525196f, 0.281799f,  0.624204f,
  -0.662775f, -0.230887f, 0.980989f,  0.223437f,  -0.790591f, 0.600724f,
  -0.273445f, 0.427635f,  -0.501641f, -0.878390f, 0.234731f,  -0.172550f,
  0.418904f,  1.792187f,
};

static const float av1_tx_split_nn_bias_16x32_layer1[1] = {
  -0.29233751f,
};

static const NN_CONFIG av1_tx_split_nnconfig_16x32 = {
  8,  // num_inputs
  1,  // num_outputs
  1,  // num_hidden_layers
  {
      32,
  },  // num_hidden_nodes
  {
      av1_tx_split_nn_weights_16x32_layer0,
      av1_tx_split_nn_weights_16x32_layer1,
  },
  {
      av1_tx_split_nn_bias_16x32_layer0,
      av1_tx_split_nn_bias_16x32_layer1,
  },
};
/******************************************************************************/

// Tx split model for 32x64 block.
static const float av1_tx_split_nn_weights_32x64_layer0[8 * 32] = {
  0.031614f,  -0.110926f, 0.052418f,  -0.702506f, 0.045708f,  0.238329f,
  -0.021806f, -0.208128f, 0.509745f,  -0.293891f, 0.277788f,  0.113937f,
  0.741576f,  0.062848f,  0.351878f,  0.212532f,  0.385842f,  0.081517f,
  0.398502f,  -0.015156f, 0.242616f,  0.214619f,  -0.182678f, -0.170546f,
  0.110605f,  -0.236749f, -0.023831f, -0.285243f, 0.147156f,  -0.257639f,
  0.341355f,  -0.571641f, -0.721797f, 0.139588f,  -0.518494f, -0.206526f,
  -0.570560f, -0.184295f, 0.110271f,  0.210292f,  -0.109132f, -0.001080f,
  0.129251f,  -0.204230f, -0.396312f, -0.183024f, 0.421243f,  -0.013154f,
  0.222627f,  0.169826f,  0.226037f,  0.218153f,  -0.343528f, 0.274906f,
  -0.156632f, 0.250261f,  -0.484020f, 0.019909f,  -0.349575f, -0.286643f,
  -0.507396f, 0.202446f,  -0.154110f, -0.292644f, 0.122666f,  0.306963f,
  0.424895f,  0.005579f,  0.494094f,  -0.079551f, 0.473740f,  0.352414f,
  -0.356917f, 0.264331f,  -0.554487f, 0.119978f,  0.012291f,  -0.141641f,
  -0.254714f, -0.213723f, -0.116701f, -0.011267f, 0.190025f,  -0.118501f,
  0.305151f,  -0.316782f, -0.220801f, -0.308420f, -0.324285f, 0.421329f,
  -0.177066f, -0.055114f, 0.229698f,  -0.199523f, 0.054278f,  0.365020f,
  -0.060586f, -0.300618f, 0.157563f,  -0.064338f, -0.005711f, -0.176991f,
  -0.424502f, -0.111914f, 0.092608f,  0.126621f,  0.078547f,  0.148008f,
  0.024221f,  0.124599f,  0.001343f,  0.059402f,  0.453753f,  0.047102f,
  0.242544f,  0.055735f,  -0.067451f, -0.170061f, -0.170469f, -0.232173f,
  0.214908f,  0.248889f,  0.544348f,  -0.084566f, 0.402478f,  0.298031f,
  0.099038f,  -0.238019f, -0.475085f, -0.070042f, -0.754955f, -0.049095f,
  -0.783801f, -0.099857f, -0.582008f, -0.055194f, -0.103655f, 0.143689f,
  0.100219f,  0.293934f,  0.099271f,  -0.036320f, 0.356626f,  -0.261445f,
  0.879544f,  0.000878f,  0.532920f,  -0.093918f, 0.508867f,  -0.040215f,
  -0.789042f, -0.145380f, -0.090040f, -0.066636f, 0.015212f,  0.352989f,
  -0.058831f, -0.164588f, 0.039890f,  0.122861f,  0.222508f,  0.061217f,
  0.466487f,  0.022666f,  0.423777f,  -0.002200f, -0.656835f, -0.099760f,
  -0.520606f, 0.303204f,  -0.563620f, -0.160922f, -0.243203f, 0.313354f,
  -0.336516f, -0.206764f, -0.236040f, 0.325899f,  -0.418748f, 0.163205f,
  -0.476242f, -0.121928f, 0.139178f,  -0.157193f, -0.531766f, -0.180202f,
  -0.485254f, 0.187703f,  -0.440072f, 0.137854f,  0.029139f,  0.109530f,
  -0.078475f, -0.360618f, -0.334672f, -0.350890f, -0.403976f, 0.180336f,
  -0.304542f, 0.005123f,  0.413995f,  0.314639f,  0.342648f,  -0.293264f,
  0.358135f,  -0.180425f, -0.369530f, -0.048413f, 0.498366f,  0.121875f,
  0.270948f,  -0.187966f, 0.342503f,  0.174420f,  -0.352105f, 0.088080f,
  0.008277f,  0.020275f,  -0.002381f, 0.504389f,  -0.018832f, -0.366047f,
  -0.090947f, -0.168150f, 0.016184f,  -0.328914f, 0.089579f,  -0.017349f,
  0.005844f,  -0.005010f, -1.857514f, -0.282426f, 0.010177f,  -0.214727f,
  -0.182529f, 0.156943f,  -0.162032f, -0.472654f, 0.069432f,  0.016901f,
  -0.767905f, 0.137129f,  -0.411463f, 0.049056f,  -0.431657f, -0.037641f,
  0.785500f,  0.046225f,  0.195831f,  0.245204f,  0.368614f,  0.212261f,
  0.440626f,  -0.158048f, -0.461031f, -0.146280f,
};

static const float av1_tx_split_nn_bias_32x64_layer0[32] = {
  0.490777f,  -1.894238f, 0.621333f,  -0.076756f, 0.286298f, 0.286375f,
  -0.126431f, -0.350034f, -1.017572f, 0.620125f,  0.408128f, 0.238756f,
  -0.060728f, 0.210912f,  0.043124f,  0.445649f,  0.907025f, 0.360272f,
  1.083101f,  -0.068952f, 1.062348f,  0.396354f,  0.280075f, 0.501732f,
  0.328422f,  0.066241f,  0.474697f,  0.126313f,  0.741206f, 0.314796f,
  0.552712f,  0.299410f,
};

static const float av1_tx_split_nn_weights_32x64_layer1[32] = {
  1.033823f,  0.603439f,  0.304591f,  -0.279940f, -0.780909f, -0.132801f,
  0.154059f,  0.662014f,  -0.718368f, 0.198733f,  0.039766f,  -0.208516f,
  -0.104909f, -0.394209f, 0.081617f,  0.365041f,  -0.874960f, -0.063315f,
  -1.189897f, 0.337225f,  0.410893f,  0.307519f,  0.221323f,  0.233895f,
  0.469536f,  0.438557f,  0.280144f,  0.422423f,  -1.394513f, 0.781900f,
  0.352981f,  0.111265f,
};

static const float av1_tx_split_nn_bias_32x64_layer1[1] = {
  -0.18160765f,
};

static const NN_CONFIG av1_tx_split_nnconfig_32x64 = {
  8,  // num_inputs
  1,  // num_outputs
  1,  // num_hidden_layers
  {
      32,
  },  // num_hidden_nodes
  {
      av1_tx_split_nn_weights_32x64_layer0,
      av1_tx_split_nn_weights_32x64_layer1,
  },
  {
      av1_tx_split_nn_bias_32x64_layer0,
      av1_tx_split_nn_bias_32x64_layer1,
  },
};
/******************************************************************************/

// Tx split model for 8x32 block.
static const float av1_tx_split_nn_weights_8x32_layer0[8 * 24] = {
  -0.687846f, 0.121404f,  -0.372905f, 0.126770f,  -0.103298f, -0.101650f,
  -0.148490f, -0.271740f, 0.682915f,  -0.079765f, 0.634347f,  -0.151503f,
  0.287692f,  -0.079072f, -0.236948f, 0.065064f,  0.713383f,  0.397123f,
  0.553621f,  0.368529f,  0.767663f,  -0.046601f, -0.392402f, -0.294822f,
  -0.292325f, -0.010573f, -0.837945f, 0.050113f,  -0.811360f, 0.199162f,
  0.150832f,  0.011602f,  0.369694f,  -0.225876f, 0.234113f,  -0.269808f,
  0.303805f,  -0.190281f, -0.451136f, 0.209755f,  -0.308894f, 0.326956f,
  0.313591f,  0.089923f,  -0.095754f, 0.390981f,  0.467366f,  0.169670f,
  0.853322f,  0.054055f,  0.830319f,  -0.121918f, 0.262019f,  -0.093526f,
  0.385558f,  0.419174f,  0.040198f,  -0.347030f, -0.450492f, -0.106764f,
  0.487502f,  -0.204188f, 0.430374f,  -0.116388f, 0.236407f,  -0.157376f,
  0.732294f,  -0.651387f, 0.347446f,  0.342575f,  0.048406f,  0.187657f,
  0.434899f,  -0.447782f, 0.032728f,  -0.071168f, -0.255327f, 0.104174f,
  0.095689f,  -0.431743f, 0.725694f,  0.031797f,  0.523171f,  0.061801f,
  0.469804f,  -0.071068f, -0.059024f, -0.211937f, 0.392134f,  -0.321490f,
  0.366060f,  -0.427798f, 0.166771f,  0.299652f,  0.044660f,  0.205142f,
  0.039133f,  -0.051835f, -0.465475f, 0.216976f,  -0.341156f, 0.095358f,
  0.230807f,  0.201674f,  0.279266f,  -0.713534f, -0.091690f, -0.569708f,
  -0.119001f, 0.252160f,  -1.544578f, -0.284477f, 0.555348f,  0.226471f,
  0.347690f,  0.034365f,  0.770835f,  -0.241859f, -0.130241f, 0.292936f,
  0.396622f,  -0.417916f, 0.492224f,  0.125517f,  0.344824f,  0.232172f,
  -0.432106f, -0.278745f, 0.035069f,  -0.307247f, -0.120760f, 0.170950f,
  0.433601f,  0.044286f,  0.141463f,  -0.041382f, 0.529346f,  0.010868f,
  -0.323674f, 0.185205f,  0.623459f,  0.232842f,  -0.406693f, -0.142944f,
  0.222988f,  0.343634f,  0.065401f,  0.002621f,  0.805335f,  -0.426926f,
  0.279181f,  0.131364f,  0.192339f,  -0.402391f, 0.544120f,  -0.060618f,
  0.467780f,  0.165224f,  -0.373131f, 0.002427f,  0.688064f,  0.322317f,
  0.259713f,  0.130583f,  0.185032f,  -0.189111f, -0.067821f, 0.010875f,
  0.644724f,  -0.179291f, 0.463222f,  0.155230f,  0.721384f,  -0.046019f,
  0.438501f,  0.440027f,  -0.462090f, -0.002039f, -0.468026f, -0.008890f,
  -0.328530f, 0.370102f,  0.482531f,  0.043471f,  -0.469732f, -0.532663f,
  0.122081f,  -0.379659f, 0.037219f,  -0.519913f, -0.128975f, -0.404365f,
};

static const float av1_tx_split_nn_bias_8x32_layer0[24] = {
  -1.198965f, 0.395204f,  -0.408627f, -0.021654f, -0.658355f, 0.154525f,
  -0.288354f, 1.207574f,  0.411608f,  0.964678f,  -1.176893f, 1.059006f,
  -0.472969f, 2.087975f,  1.065536f,  0.595569f,  0.197907f,  -0.349938f,
  1.013651f,  -0.931093f, -0.973595f, -0.459094f, -1.253062f, 1.624782f,
};

static const float av1_tx_split_nn_weights_8x32_layer1[24] = {
  0.815787f,  -0.393465f, -0.483427f, -0.565592f, 0.493494f,  0.430229f,
  -0.507073f, -0.251379f, -0.353418f, -0.495445f, 0.820029f,  0.649146f,
  -0.487383f, 1.844503f,  0.480324f,  -0.982705f, -0.501446f, -0.220584f,
  0.334299f,  0.802238f,  0.805838f,  -0.487848f, 0.300772f,  -1.232857f,
};

static const float av1_tx_split_nn_bias_8x32_layer1[1] = {
  0.13435879f,
};

static const NN_CONFIG av1_tx_split_nnconfig_8x32 = {
  8,  // num_inputs
  1,  // num_outputs
  1,  // num_hidden_layers
  {
      24,
  },  // num_hidden_nodes
  {
      av1_tx_split_nn_weights_8x32_layer0,
      av1_tx_split_nn_weights_8x32_layer1,
  },
  {
      av1_tx_split_nn_bias_8x32_layer0,
      av1_tx_split_nn_bias_8x32_layer1,
  },
};
/******************************************************************************/

// Tx split model for 16x32 block.
static const float av1_tx_split_nn_weights_16x64_layer0[8 * 16] = {
  -0.378223f, -0.124216f, -0.514089f, -0.110117f, -0.585801f, -0.094838f,
  -0.455385f, -0.220254f, -0.504568f, -0.082351f, -0.476420f, -0.253993f,
  -0.454709f, -0.059461f, 0.210313f,  -0.155683f, 0.192968f,  -0.127804f,
  0.471996f,  0.253377f,  0.472625f,  0.485322f,  0.150560f,  0.164868f,
  -0.475587f, 0.447559f,  -0.455759f, -0.306665f, -0.194866f, -0.283716f,
  -0.243897f, 0.293020f,  -0.308298f, -0.191904f, -0.468568f, 0.014053f,
  -0.618848f, 0.096273f,  -0.444586f, 0.347750f,  -0.280643f, -0.062872f,
  0.118661f,  0.540099f,  0.104141f,  -0.279300f, -0.098721f, -0.173427f,
  -0.984558f, -0.424559f, -0.411928f, -0.120875f, -0.488999f, -0.050716f,
  -0.523103f, 0.093620f,  -0.930396f, -0.431997f, -1.163297f, 0.190384f,
  -0.422581f, -0.005354f, 0.450552f,  0.369210f,  0.562484f,  0.679922f,
  0.282099f,  -0.039075f, 0.404196f,  0.006371f,  0.069679f,  -0.196160f,
  -0.213675f, 0.275187f,  -0.104235f, -0.193090f, 0.003116f,  -0.252454f,
  -0.094591f, 0.210439f,  -0.137070f, 0.145043f,  0.024558f,  0.121718f,
  0.010138f,  0.301651f,  -0.377990f, 0.444414f,  0.001845f,  -0.095334f,
  0.550259f,  0.087603f,  0.792492f,  -0.044584f, 0.641706f,  -0.328458f,
  -0.447791f, 0.135376f,  0.356385f,  0.135748f,  0.310370f,  0.293757f,
  -0.062000f, -0.056368f, 0.343930f,  0.312039f,  0.370763f,  0.452381f,
  -0.023630f, -0.185909f, 0.422277f,  -0.006306f, 0.045166f,  0.423359f,
  -0.157735f, -0.084901f, 0.219527f,  -0.209510f, 0.575057f,  0.249276f,
  0.069267f,  0.233898f,  -0.229392f, 0.117197f,  -0.038551f, 0.293976f,
  0.101996f,  0.120878f,
};

static const float av1_tx_split_nn_bias_16x64_layer0[16] = {
  1.036995f,  0.160249f,  0.100264f,  0.694881f,  0.694677f,  0.128379f,
  -0.843405f, -0.405515f, 0.104139f,  0.182980f,  -0.025472f, 0.901067f,
  -0.299866f, -0.103079f, -0.190352f, -0.048121f,
};

static const float av1_tx_split_nn_weights_16x64_layer1[16] = {
  -1.778868f, 0.174690f,  0.211991f, 0.712138f,  0.589352f,  0.466652f,
  1.029146f,  -0.490044f, 0.483015f, 0.600215f,  -0.577776f, -0.755546f,
  0.348337f,  -0.205082f, 0.347129f, -0.322277f,
};

static const float av1_tx_split_nn_bias_16x64_layer1[1] = {
  0.04230947f,
};

static const NN_CONFIG av1_tx_split_nnconfig_16x64 = {
  8,  // num_inputs
  1,  // num_outputs
  1,  // num_hidden_layers
  {
      16,
  },  // num_hidden_nodes
  {
      av1_tx_split_nn_weights_16x64_layer0,
      av1_tx_split_nn_weights_16x64_layer1,
  },
  {
      av1_tx_split_nn_bias_16x64_layer0,
      av1_tx_split_nn_bias_16x64_layer1,
  },
};
/******************************************************************************/

// Map block size to its corresponding neural net model for tx split prediction.
static const NN_CONFIG *const av1_tx_split_nnconfig_map[TX_SIZES_ALL] = {
  NULL,                          // TX_4X4,
  &av1_tx_split_nnconfig_8x8,    // TX_8X8,
  &av1_tx_split_nnconfig_16x16,  // TX_16X16,
  &av1_tx_split_nnconfig_32x32,  // TX_32X32,
  &av1_tx_split_nnconfig_64x64,  // TX_64X64,
  &av1_tx_split_nnconfig_4x8,    // TX_4X8,
  &av1_tx_split_nnconfig_4x8,    // TX_8X4,
  &av1_tx_split_nnconfig_8x16,   // TX_8X16,
  &av1_tx_split_nnconfig_8x16,   // TX_16X8,
  &av1_tx_split_nnconfig_16x32,  // TX_16X32,
  &av1_tx_split_nnconfig_16x32,  // TX_32X16,
  &av1_tx_split_nnconfig_32x64,  // TX_32X64,
  &av1_tx_split_nnconfig_32x64,  // TX_64X32,
  &av1_tx_split_nnconfig_4x16,   // TX_4X16,
  &av1_tx_split_nnconfig_4x16,   // TX_16X4,
  &av1_tx_split_nnconfig_8x32,   // TX_8X32,
  &av1_tx_split_nnconfig_8x32,   // TX_32X8,
  &av1_tx_split_nnconfig_16x64,  // TX_16X64,
  &av1_tx_split_nnconfig_16x64,  // TX_64X16,
};

#if !CONFIG_REALTIME_ONLY
#define NUM_INTRA_TX_SPLIT_FEATURES 14
#define NUM_INTRA_TX_SPLIT_HIDDEN_LAYERS 1
#define NUM_INTRA_TX_SPLIT_HIDDEN_NODES 16
// Model to prune intra transform depth for intra 8x8 block.
static const float av1_intra_tx_split_8x8_mean[NUM_INTRA_TX_SPLIT_FEATURES] = {
  0.110706f,  18.901518f, 0.250436f,  13.483487f, 0.118141f,
  14.318728f, 0.028409f,  14.257664f, 0.045839f,  15.143358f,
  9.702971f,  14.300809f, 6.018646f,  3.682534f,
};

static const float av1_intra_tx_split_8x8_std[NUM_INTRA_TX_SPLIT_FEATURES] = {
  13.750575f, 13.440116f, 14.334330f, 12.236641f, 18.415247f,
  12.733355f, 18.309339f, 12.858130f, 23.465142f, 13.447014f,
  8.625048f,  10.456774f, 1.185447f,  1.810423f,
};

static const float av1_intra_tx_split_nn_weights_8x8_layer0
    [NUM_INTRA_TX_SPLIT_FEATURES * NUM_INTRA_TX_SPLIT_HIDDEN_NODES] = {
      -0.156142f, -0.753623f, 0.026883f,  0.039188f,  -0.035310f, 0.106140f,
      0.051622f,  0.077838f,  0.101632f,  0.107278f,  0.232200f,  0.269083f,
      0.048966f,  -1.553293f, -0.113983f, -0.151248f, -0.067369f, 0.787292f,
      0.076651f,  -0.802634f, 0.266414f,  1.107563f,  -0.068848f, -0.956468f,
      -0.074920f, -0.192258f, 0.006207f,  0.176196f,  -0.493442f, 0.152290f,
      -0.208874f, -0.014658f, 0.297385f,  -0.351695f, 0.246295f,  -0.178519f,
      -0.204191f, 0.049663f,  -0.330343f, -0.299754f, 0.246215f,  -0.014558f,
      -0.117611f, 0.206445f,  0.045840f,  -0.047563f, -0.049679f, 0.406892f,
      -0.052307f, -1.513404f, 0.166166f,  0.520760f,  -0.143320f, -0.593928f,
      -0.010533f, 0.250752f,  0.076738f,  0.537512f,  -0.082619f, -1.534031f,
      0.047109f,  0.634247f,  -0.089730f, 0.545534f,  -0.022742f, -0.779047f,
      -0.606358f, -0.199145f, -0.051269f, 0.248784f,  0.327545f,  -0.851751f,
      0.071739f,  0.035975f,  0.387781f,  -0.136427f, -0.284436f, 0.578449f,
      -0.198276f, 0.579950f,  0.600111f,  -0.370164f, -0.215297f, 0.517342f,
      0.200061f,  -2.507660f, -0.030851f, 0.227315f,  -0.078289f, 0.276052f,
      -0.050281f, 0.251481f,  -0.139318f, 0.281175f,  0.226524f,  0.058968f,
      0.197436f,  0.517294f,  -0.105914f, -1.599567f, 0.064985f,  0.043209f,
      -0.280038f, 0.126874f,  0.330387f,  -0.014407f, 0.031241f,  0.237801f,
      0.948959f,  -0.253791f, -0.022622f, -0.061430f, 0.265852f,  0.750823f,
      0.086606f,  0.853527f,  -0.180971f, -1.255744f, -0.152979f, -1.022198f,
      -0.044708f, 0.506424f,  -0.501968f, -0.416863f, -0.012688f, 0.193523f,
      -0.093698f, 0.430875f,  0.007379f,  0.019278f,  0.080890f,  0.462755f,
      -0.054326f, -0.157611f, -0.004851f, -1.275676f, -0.060528f, -0.508170f,
      0.195429f,  -0.023534f, 0.355211f,  0.983561f,  -0.122036f, -0.911948f,
      -0.172280f, -1.135245f, -0.043211f, 0.576456f,  -0.075247f, 0.429734f,
      -0.246309f, -0.355575f, -0.048809f, 0.217113f,  0.078385f,  0.720341f,
      0.007070f,  0.144617f,  -0.167642f, 0.303056f,  -0.031425f, 0.123448f,
      -0.320530f, 0.164070f,  -0.497849f, -0.233918f, -0.032123f, 0.084983f,
      0.312216f,  0.062609f,  -0.389815f, 0.237593f,  0.000157f,  -0.642068f,
      0.167898f,  0.495234f,  -0.083493f, -0.555971f, 0.124437f,  0.381125f,
      -0.459219f, 0.047924f,  -0.138222f, -2.232816f, 0.127585f,  -0.102420f,
      0.131598f,  0.036837f,  -0.163055f, -0.067429f, -0.078521f, -0.055666f,
      1.387057f,  0.400154f,  -0.003355f, -0.073627f, -0.305098f, -0.413383f,
      -0.008266f, -0.038329f, 0.209808f,  0.375777f,  0.037274f,  -0.050226f,
      -0.100576f, 0.237441f,  0.237854f,  0.828296f,  0.001149f,  -0.093964f,
      0.214051f,  -0.031486f, -0.561307f, 0.014540f,  0.169357f,  0.323202f,
      -0.395334f, -0.038941f, 0.476800f,  -0.213122f, -0.287521f, -0.420717f,
      -0.054142f, -0.102266f,
    };

static const float
    av1_intra_tx_split_nn_bias_8x8_layer0[NUM_INTRA_TX_SPLIT_HIDDEN_NODES] = {
      -1.150850f, -0.236404f, 0.184554f,  -0.904162f, -0.949979f, 0.427016f,
      -0.546867f, -0.611094f, -0.676570f, -0.208959f, -0.286384f, 0.562238f,
      0.434197f,  -0.746518f, 0.123085f,  -0.549836f,
    };

static const float av1_intra_tx_split_nn_weights_8x8_layer1
    [NUM_INTRA_TX_SPLIT_HIDDEN_NODES] = {
      0.749814f,  0.598172f,  0.375611f, 0.751612f,  0.947538f, -0.282228f,
      -1.457522f, -1.092290f, 0.738657f, 0.575779f,  0.514823f, -0.560616f,
      -0.491619f, -1.482014f, 0.524625f, -0.533590f,
    };

static const float av1_intra_tx_split_nn_bias_8x8_layer1[1] = {
  -0.488888f,
};

static const NN_CONFIG av1_intra_tx_split_nnconfig_8x8 = {
  NUM_INTRA_TX_SPLIT_FEATURES,       // num_inputs
  1,                                 // num_outputs
  NUM_INTRA_TX_SPLIT_HIDDEN_LAYERS,  // num_hidden_layers
  {
      NUM_INTRA_TX_SPLIT_HIDDEN_NODES,
  },  // num_hidden_nodes
  {
      av1_intra_tx_split_nn_weights_8x8_layer0,
      av1_intra_tx_split_nn_weights_8x8_layer1,
  },
  {
      av1_intra_tx_split_nn_bias_8x8_layer0,
      av1_intra_tx_split_nn_bias_8x8_layer1,
  },
};

static const float av1_intra_tx_prune_nn_thresh_8x8[2] = { -0.405465f,
                                                           0.405465f };
#endif  // !CONFIG_REALTIME_ONLY

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_TX_PRUNE_MODEL_WEIGHTS_H_
