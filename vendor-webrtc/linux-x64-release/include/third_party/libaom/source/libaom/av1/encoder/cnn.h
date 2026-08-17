/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_CNN_H_
#define AOM_AV1_ENCODER_CNN_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <math.h>
#include <stdbool.h>

#include "aom_util/aom_thread.h"
#include "config/av1_rtcd.h"

struct AV1Common;

#define CNN_MAX_HIDDEN_LAYERS 64
#define CNN_MAX_LAYERS (CNN_MAX_HIDDEN_LAYERS + 1)
#define CNN_MAX_CHANNELS 256
#define CNN_MAX_BRANCHES 4
#define CNN_MAX_THREADS 32

#define NO_BRANCH_CONFIG \
  { 0, 0, 0 }
#define NO_BN_PARAMS \
  { NULL, NULL, NULL, NULL }

enum {
  PADDING_SAME_ZERO,       // tensorflow's SAME padding with pixels outside
                           // the image area assumed to be 0 (default)
  PADDING_SAME_REPLICATE,  // tensorflow's SAME padding with pixels outside
                           // the image area replicated from closest edge
  PADDING_VALID            // tensorflow's VALID padding
} UENUM1BYTE(PADDING_TYPE);

// enum { NONE, RELU, SOFTSIGN } UENUM1BYTE(ACTIVATION);

// Times when input tensor may be copied to branches given in input_to_branches.
// BRANCH_NO_COPY: doesn't copy any tensor.
// BRANCH_INPUT: copies the input tensor to branches.
// BRANCH_OUTPUT: copies the convolved tensor to branches.
// BRANCH_COMBINED: copies the combined (after convolving and branch combining)
//   tensor. If no combinations happen at this layer, then this option
//   has the same effect as COPY_OUTPUT.
enum {
  BRANCH_NO_COPY,
  BRANCH_INPUT,
  BRANCH_OUTPUT,
  BRANCH_COMBINED
} UENUM1BYTE(BRANCH_COPY);

// Types of combining branches with output of current layer:
// BRANCH_NOC: no branch combining
// BRANCH_ADD: Add previously stored branch tensor to output of layer
// BRANCH_CAT: Concatenate branch tensor to output of layer
enum { BRANCH_NOC, BRANCH_ADD, BRANCH_CAT } UENUM1BYTE(BRANCH_COMBINE);

// The parameters used to scale each channel in batch
// normalization. The processing in done on a per-channel basis.
// e.g. bn_mean[c] is the mean for all pixels in channel c. This
// is always applied after activation. The output is given by
// out[c,i,j] = norm[c,i,j] * bn_gamma[c] + bn_beta[c] where
// norm[c,i,j] = (in[c,i,j] - bn_mean[c]) / bn_std[c]
// here we assume that the effect of variance_epsilon is already
// taken into account when bn_std is calculated. The pointers
// needs to be either all zero or all valid. If all zero, then
// batchnorm is disabled, else batchnorm is applied.
struct CNN_BATCHNORM_PARAMS {
  const float *bn_gamma;
  const float *bn_beta;
  const float *bn_mean;
  const float *bn_std;
};

struct CNN_BRANCH_CONFIG {
  int input_to_branches;  // If nonzero, copy the active tensor to the current
  // layer and store for future use in branches
  // specified in the field as a binary mask. For
  // example, if input_to_branch = 0x06, it means the
  // input tensor to the current branch is copied to
  // branches 1 and 2 (where 0 represents the primary
  // branch). One restriction is that the mask
  // cannot indicate copying to the current branch.
  // If greater than 0, only copies the channels up
  // to the given index.
  int channels_to_copy;  // Within the layer, input a copy of active
  // tensor to branches given in input_to_branches.
  int branches_to_combine;  // mask of branches to combine with output of
  // current layer, if
  // branch_combine_type != BRANCH_NOC
  // For example, if branches_to_combine = 0x0A,
  // it means that braches 1 and 3 are combined
  // with the current branch.
};

struct CNN_LAYER_CONFIG {
  int in_channels;
  int filter_width;
  int filter_height;
  int out_channels;
  int skip_width;
  int skip_height;
  int maxpool;            // whether to use maxpool or not (only effective when
                          // skip width or skip_height are > 1)
  const float *weights;   // array of length filter_height x filter_width x
                          // in_channels x out_channels where the inner-most
                          // scan is out_channels and the outer most scan is
                          // filter_height.
  const float *bias;      // array of length out_channels
  PADDING_TYPE pad;       // padding type
  ACTIVATION activation;  // the activation function to use after convolution
  int deconvolve;         // whether this is a deconvolution layer.
                          // 0: If skip_width or skip_height are > 1, then we
                          // reduce resolution
                          // 1: If skip_width or skip_height are > 1, then we
                          // increase resolution
  int branch;             // branch index in [0, CNN_MAX_BRANCHES - 1], where
                          // 0 refers to the primary branch.
  BRANCH_COPY branch_copy_type;
  BRANCH_COMBINE branch_combine_type;
  struct CNN_BRANCH_CONFIG branch_config;
  struct CNN_BATCHNORM_PARAMS
      bn_params;   // A struct that contains the parameters
                   // used for batch normalization.
  int output_num;  // The output buffer idx to which the layer output is
                   // written. Set to -1 to disable writing it to the output. In
                   // the case that branch_combine_type is BRANCH_CAT, all
                   // concatenated channels will be written to output. In the
                   // case of BRANCH_ADD, the output will be the result of
                   // summation.
};

struct CNN_CONFIG {
  int num_layers;  // number of CNN layers ( = number of hidden layers + 1)
  int is_residue;  // whether the output activation is a residue
  int ext_width, ext_height;  // extension horizontally and vertically
  int strict_bounds;          // whether the input bounds are strict or not.
                              // If strict, the extension area is filled by
                              // replication; if not strict, image data is
                              // assumed available beyond the bounds.
  CNN_LAYER_CONFIG layer_config[CNN_MAX_LAYERS];
};

struct CNN_THREAD_DATA {
  int num_workers;
  AVxWorker *workers;
};

struct CNN_MULTI_OUT {
  int num_outputs;
  const int *output_channels;
  const int *output_strides;
  float **output_buffer;
};

// Function to return size of output
void av1_find_cnn_output_size(int in_width, int in_height,
                              const CNN_CONFIG *cnn_config, int *out_width,
                              int *out_height, int *out_channels);

// Function to return output width and output height of given layer.
void av1_find_cnn_layer_output_size(int in_width, int in_height,
                                    const CNN_LAYER_CONFIG *layer_config,
                                    int *out_width, int *out_height);

// Prediction functions from set of input image buffers. This function supports
// CNN with multiple outputs.
bool av1_cnn_predict_img_multi_out(uint8_t **dgd, int width, int height,
                                   int stride, const CNN_CONFIG *cnn_config,
                                   const CNN_THREAD_DATA *thread_data,
                                   struct CNN_MULTI_OUT *output);
bool av1_cnn_predict_img_multi_out_highbd(uint16_t **dgd, int width, int height,
                                          int stride,
                                          const CNN_CONFIG *cnn_config,
                                          const CNN_THREAD_DATA *thread_data,
                                          int bit_depth, CNN_MULTI_OUT *output);
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_CNN_H_
