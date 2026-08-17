/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_VAD_GMM_H_
#define MODULES_AUDIO_PROCESSING_VAD_GMM_H_

namespace webrtc {

// A structure that specifies a GMM.
// A GMM is formulated as
//  f(x) = w[0] * mixture[0] + w[1] * mixture[1] + ... +
//         w[num_mixtures - 1] * mixture[num_mixtures - 1];
// Where a 'mixture' is a Gaussian density.

struct GmmParameters {
  // weight[n] = log(w[n]) - `dimension`/2 * log(2*pi) - 1/2 * log(det(cov[n]));
  // where cov[n] is the covariance matrix of mixture n;
  const double* weight;
  // pointer to the first element of a `num_mixtures`x`dimension` matrix
  // where kth row is the mean of the kth mixture.
  const double* mean;
  // pointer to the first element of a `num_mixtures`x`dimension`x`dimension`
  // 3D-matrix, where the kth 2D-matrix is the inverse of the covariance
  // matrix of the kth mixture.
  const double* covar_inverse;
  // Dimensionality of the mixtures.
  int dimension;
  // number of the mixtures.
  int num_mixtures;
};

// Evaluate the given GMM, according to `gmm_parameters`, at the given point
// `x`. If the dimensionality of the given GMM is larger that the maximum
// acceptable dimension by the following function -1 is returned.
double EvaluateGmm(const double* x, const GmmParameters& gmm_parameters);

}  // namespace webrtc
#endif  // MODULES_AUDIO_PROCESSING_VAD_GMM_H_
