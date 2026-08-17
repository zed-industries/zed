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

#ifndef AOM_AV1_ENCODER_PUSTATS_H_
#define AOM_AV1_ENCODER_PUSTATS_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "av1/encoder/ml.h"

#define NUM_FEATURES_PUSTATS 8
#define NUM_HIDDEN_LAYERS 2
#define HIDDEN_LAYERS_0_NODES 12
#define HIDDEN_LAYERS_1_NODES 10
#define LOGITS_NODES 1

static const float
    av1_pustats_rate_hiddenlayer_0_kernel[NUM_FEATURES_PUSTATS *
                                          HIDDEN_LAYERS_0_NODES] = {
      -0.1758f, -0.0499f, -10.0069f, -2.2838f,  -0.3359f,  0.3459f,  -0.3285f,
      -0.0515f, -0.5417f, 0.2357f,   -0.0575f,  -69.0782f, 0.5348f,  1.4068f,
      0.2213f,  -1.0490f, -0.0636f,  0.1654f,   1.1002f,   33.4924f, 0.4358f,
      1.2499f,  0.1143f,  0.0592f,   -1.6335f,  -0.0092f,  1.2207f,  -28.4543f,
      -0.4973f, 0.4368f,  0.2341f,   -0.1623f,  -3.8986f,  0.1311f,  -1.8789f,
      -3.9079f, -0.8158f, -0.8420f,  1.4295f,   -2.3629f,  -1.4825f, 0.6498f,
      -5.3669f, 6.4434f,  1.8393f,   -35.0678f, 3.7459f,   -2.8504f, 2.0502f,
      -0.1812f, -3.9011f, -1.0155f,  1.8375f,   -1.4517f,  1.3917f,  3.8664f,
      0.8345f,  -0.3472f, 5.7740f,   -1.1196f,  -0.3264f,  -1.2481f, -0.9284f,
      -4.9657f, 2.2831f,  0.7337f,   2.3176f,   0.6416f,   0.8804f,  1.9988f,
      -1.3426f, 1.2728f,  1.2249f,   -0.1551f,  5.6045f,   0.2046f,  -2.1464f,
      -2.4922f, -0.5334f, 12.1055f,  7.2467f,   -0.0070f,  0.0234f,  0.0021f,
      0.0215f,  -0.0098f, -0.0682f,  -6.1494f,  -0.3176f,  -1.6069f, -0.2119f,
      -1.0533f, -0.3566f, 0.5294f,   -0.4335f,  0.1626f,
    };

static const float
    av1_pustats_rate_hiddenlayer_0_bias[HIDDEN_LAYERS_0_NODES] = {
      10.5266f, 5.3268f, -1.0678f, 7.7411f,  8.7164f,  -0.3235f,
      7.3028f,  9.0874f, -6.4594f, -1.0102f, -1.1146f, 10.8419f,
    };

static const float
    av1_pustats_rate_hiddenlayer_1_kernel[HIDDEN_LAYERS_0_NODES *
                                          HIDDEN_LAYERS_1_NODES] = {
      10.5932f,  2.5192f,  -0.0015f, 5.9479f,   5.2426f,   -0.4091f, 5.3220f,
      6.0469f,   0.7200f,  3.3241f,  5.5006f,   12.8290f,  -1.6396f, 0.5743f,
      -0.8370f,  1.9956f,  -4.9270f, -1.5295f,  2.1350f,   -9.4415f, -0.7094f,
      5.1822f,   19.7287f, -3.0444f, -0.3320f,  0.0031f,   -0.2709f, -0.5249f,
      0.3281f,   -0.2240f, 0.2225f,  -0.2386f,  -0.4370f,  -0.2438f, -0.4928f,
      -0.2842f,  -2.1772f, 9.2570f,  -17.6655f, 3.5448f,   -2.8394f, -1.0167f,
      -0.5115f,  -1.9260f, -0.2111f, -0.7528f,  -1.2387f,  -0.0401f, 5.0716f,
      -3.3763f,  -0.2898f, -0.4956f, -7.9993f,  0.1526f,   -0.0242f, 0.7354f,
      6.0432f,   4.8043f,  7.4790f,  -0.6295f,  1.7565f,   3.7197f,  -2.3963f,
      6.8945f,   2.9717f,  -3.1623f, 3.4241f,   4.4676f,   -1.8154f, -2.9401f,
      -8.5657f,  -3.0240f, -1.4661f, 8.1145f,   -12.7858f, 3.3624f,  -1.0819f,
      -4.2856f,  1.1801f,  -0.5587f, -1.6062f,  -1.1813f,  -3.5882f, -0.2490f,
      -24.9566f, -0.4140f, -0.1113f, 3.5537f,   4.4112f,   0.1367f,  -1.5876f,
      1.6605f,   1.3903f,  -0.0253f, -2.1419f,  -2.2197f,  -0.7659f, -0.4249f,
      -0.0424f,  0.1486f,  0.4643f,  -0.9068f,  -0.3619f,  -0.7624f, -0.9132f,
      -0.4947f,  -0.3527f, -0.5445f, -0.4768f,  -1.7761f,  -1.0686f, 0.5462f,
      1.3371f,   4.3116f,  0.0777f,  -2.7216f,  -1.8908f,  3.4989f,  7.7269f,
      -2.7566f,
    };

static const float
    av1_pustats_rate_hiddenlayer_1_bias[HIDDEN_LAYERS_1_NODES] = {
      13.2435f, -8.5477f, -0.0998f, -1.5131f, -12.0187f,
      6.1715f,  0.5094f,  7.6433f,  -0.3992f, -1.3555f,
    };

static const float
    av1_pustats_rate_logits_kernel[HIDDEN_LAYERS_1_NODES * LOGITS_NODES] = {
      4.3078f, -17.3497f, 0.0195f,  34.6032f, -5.0127f,
      5.3079f, 10.0077f,  -13.129f, 0.0087f,  -8.4009f,
    };

static const float av1_pustats_rate_logits_bias[LOGITS_NODES] = {
  4.5103f,
};

static const NN_CONFIG av1_pustats_rate_nnconfig = {
  NUM_FEATURES_PUSTATS,                              // num_inputs
  LOGITS_NODES,                                      // num_outputs
  NUM_HIDDEN_LAYERS,                                 // num_hidden_layers
  { HIDDEN_LAYERS_0_NODES, HIDDEN_LAYERS_1_NODES },  // num_hidden_nodes
  {
      av1_pustats_rate_hiddenlayer_0_kernel,
      av1_pustats_rate_hiddenlayer_1_kernel,
      av1_pustats_rate_logits_kernel,
  },
  {
      av1_pustats_rate_hiddenlayer_0_bias,
      av1_pustats_rate_hiddenlayer_1_bias,
      av1_pustats_rate_logits_bias,
  },
};

static const float
    av1_pustats_dist_hiddenlayer_0_kernel[NUM_FEATURES_PUSTATS *
                                          HIDDEN_LAYERS_0_NODES] = {
      -0.2560f, 0.1105f,  -0.8434f, -0.0132f, -8.9371f, -1.1176f, -0.3655f,
      0.4885f,  1.7518f,  0.4985f,  0.5582f,  -0.3739f, 0.9403f,  0.3874f,
      0.3265f,  1.7383f,  3.1747f,  0.0285f,  3.3942f,  -0.0123f, 0.5057f,
      0.1584f,  0.2697f,  4.6151f,  3.6251f,  -0.0121f, -1.0047f, -0.0037f,
      0.0127f,  0.1935f,  -0.5277f, -2.7144f, 0.0729f,  -0.1457f, -0.0816f,
      -0.5462f, 0.4738f,  0.3599f,  -0.0564f, 0.0910f,  0.0126f,  -0.0310f,
      -2.1311f, -0.4666f, -0.0074f, -0.0765f, 0.0287f,  -0.2662f, -0.0999f,
      -0.2983f, -0.4899f, -0.2314f, 0.2873f,  -0.3614f, 0.1783f,  -0.1210f,
      0.3569f,  0.5436f,  -8.0536f, -0.0044f, -1.5255f, -0.8247f, -0.4556f,
      1.9045f,  0.5463f,  0.1102f,  -0.9293f, -0.0185f, -0.8302f, -0.4378f,
      -0.3531f, -1.3095f, 0.6099f,  0.7977f,  4.1950f,  -0.0067f, -0.2762f,
      -0.1574f, -0.2149f, 0.6104f,  -1.7053f, 0.1904f,  4.2402f,  -0.2671f,
      0.8940f,  0.6820f,  0.2241f,  -0.9459f, 1.4571f,  0.5255f,  2.3352f,
      -0.0806f, 0.5231f,  0.3928f,  0.4146f,  2.0956f,
    };

static const float
    av1_pustats_dist_hiddenlayer_0_bias[HIDDEN_LAYERS_0_NODES] = {
      1.1597f, 0.0836f, -0.7471f, -0.2439f, -0.0438f, 2.4626f,
      0.f,     1.1485f, 2.7085f,  -4.7897f, 1.4093f,  -1.657f,
    };

static const float
    av1_pustats_dist_hiddenlayer_1_kernel[HIDDEN_LAYERS_0_NODES *
                                          HIDDEN_LAYERS_1_NODES] = {
      -0.5203f, -1.3468f, 0.3865f,  -0.6859f, 0.0058f,  4.0682f,  0.4807f,
      -0.1380f, 0.6050f,  0.8958f,  0.7748f,  -0.1311f, 1.7317f,  1.1265f,
      0.0827f,  0.1407f,  -0.3605f, 0.5429f,  0.1880f,  -0.1439f, 0.2837f,
      1.6477f,  0.0832f,  0.0593f,  -1.8464f, -0.7241f, -1.0672f, -0.3546f,
      -0.3842f, -2.3637f, 0.2514f,  0.8263f,  -0.1872f, 0.5774f,  -0.3610f,
      -0.0205f, 1.3977f,  -0.1083f, 0.6923f,  1.3039f,  -0.2870f, 1.0622f,
      -0.0566f, 0.2697f,  -0.5429f, -0.6193f, 1.7559f,  0.3246f,  1.9159f,
      0.3744f,  0.0686f,  1.0191f,  -0.4212f, 1.9591f,  -0.0691f, -0.1085f,
      -1.2034f, 0.0606f,  1.0116f,  0.5565f,  -0.1874f, -0.7898f, 0.4796f,
      0.2290f,  0.4334f,  -0.5817f, -0.2949f, 0.1367f,  -0.2932f, -1.1265f,
      0.0133f,  -0.5309f, -3.3191f, 0.0939f,  0.3895f,  -2.5812f, -0.0066f,
      -3.0063f, -0.2982f, 0.7309f,  -0.2422f, -0.2770f, -0.7152f, 0.1700f,
      1.9630f,  0.1988f,  0.4194f,  0.8762f,  0.3402f,  0.1051f,  -0.1598f,
      0.2405f,  0.0392f,  1.1256f,  1.5245f,  0.0950f,  0.2160f,  -0.5023f,
      0.2584f,  0.2074f,  0.2218f,  0.3966f,  -0.0921f, -0.2435f, -0.4560f,
      -1.1923f, -0.3716f, -0.3286f, -1.3225f, 0.1896f,  -0.3342f, -0.7888f,
      -0.4488f, -1.7168f, 0.3341f,  0.1146f,  0.5226f,  0.2610f,  -0.4574f,
      -0.4164f,
    };

static const float
    av1_pustats_dist_hiddenlayer_1_bias[HIDDEN_LAYERS_1_NODES] = {
      -2.3014f, -2.4292f, 1.3317f, -3.2361f, -1.918f,
      2.7149f,  -2.5649f, 2.7765f, 2.9617f,  2.7684f,
    };

static const float
    av1_pustats_dist_logits_kernel[HIDDEN_LAYERS_1_NODES * LOGITS_NODES] = {
      -0.6868f, -0.6715f, 0.449f,  -1.293f, 0.6214f,
      0.9894f,  -0.4342f, 0.7002f, 1.4363f, 0.6951f,
    };

static const float av1_pustats_dist_logits_bias[LOGITS_NODES] = {
  2.3371f,
};

static const NN_CONFIG av1_pustats_dist_nnconfig = {
  NUM_FEATURES_PUSTATS,                              // num_inputs
  LOGITS_NODES,                                      // num_outputs
  NUM_HIDDEN_LAYERS,                                 // num_hidden_layers
  { HIDDEN_LAYERS_0_NODES, HIDDEN_LAYERS_1_NODES },  // num_hidden_nodes
  {
      av1_pustats_dist_hiddenlayer_0_kernel,
      av1_pustats_dist_hiddenlayer_1_kernel,
      av1_pustats_dist_logits_kernel,
  },
  {
      av1_pustats_dist_hiddenlayer_0_bias,
      av1_pustats_dist_hiddenlayer_1_bias,
      av1_pustats_dist_logits_bias,
  },
};

#undef NUM_HIDDEN_LAYERS
#undef HIDDEN_LAYERS_0_NODES
#undef HIDDEN_LAYERS_1_NODES
#undef LOGITS_NODES

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_PUSTATS_H_
