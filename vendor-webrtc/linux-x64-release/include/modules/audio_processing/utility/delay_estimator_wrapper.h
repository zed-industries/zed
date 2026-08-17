/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Performs delay estimation on block by block basis.
// The return value is  0 - OK and -1 - Error, unless otherwise stated.

#ifndef MODULES_AUDIO_PROCESSING_UTILITY_DELAY_ESTIMATOR_WRAPPER_H_
#define MODULES_AUDIO_PROCESSING_UTILITY_DELAY_ESTIMATOR_WRAPPER_H_

#include <stdint.h>

namespace webrtc {

// Releases the memory allocated by WebRtc_CreateDelayEstimatorFarend(...)
void WebRtc_FreeDelayEstimatorFarend(void* handle);

// Allocates the memory needed by the far-end part of the delay estimation. The
// memory needs to be initialized separately through
// WebRtc_InitDelayEstimatorFarend(...).
//
// Inputs:
//  - spectrum_size     : Size of the spectrum used both in far-end and
//                        near-end. Used to allocate memory for spectrum
//                        specific buffers.
//  - history_size      : The far-end history buffer size. A change in buffer
//                        size can be forced with WebRtc_set_history_size().
//                        Note that the maximum delay which can be estimated is
//                        determined together with WebRtc_set_lookahead().
//
// Return value:
//  - void*             : Created `handle`. If the memory can't be allocated or
//                        if any of the input parameters are invalid NULL is
//                        returned.
void* WebRtc_CreateDelayEstimatorFarend(int spectrum_size, int history_size);

// Initializes the far-end part of the delay estimation instance returned by
// WebRtc_CreateDelayEstimatorFarend(...)
int WebRtc_InitDelayEstimatorFarend(void* handle);

// Soft resets the far-end part of the delay estimation instance returned by
// WebRtc_CreateDelayEstimatorFarend(...).
// Input:
//      - delay_shift   : The amount of blocks to shift history buffers.
void WebRtc_SoftResetDelayEstimatorFarend(void* handle, int delay_shift);

// Adds the far-end spectrum to the far-end history buffer. This spectrum is
// used as reference when calculating the delay using
// WebRtc_ProcessSpectrum().
//
// Inputs:
//    - far_spectrum    : Far-end spectrum.
//    - spectrum_size   : The size of the data arrays (same for both far- and
//                        near-end).
//    - far_q           : The Q-domain of the far-end data.
//
// Output:
//    - handle          : Updated far-end instance.
//
int WebRtc_AddFarSpectrumFix(void* handle,
                             const uint16_t* far_spectrum,
                             int spectrum_size,
                             int far_q);

// See WebRtc_AddFarSpectrumFix() for description.
int WebRtc_AddFarSpectrumFloat(void* handle,
                               const float* far_spectrum,
                               int spectrum_size);

// Releases the memory allocated by WebRtc_CreateDelayEstimator(...)
void WebRtc_FreeDelayEstimator(void* handle);

// Allocates the memory needed by the delay estimation. The memory needs to be
// initialized separately through WebRtc_InitDelayEstimator(...).
//
// Inputs:
//      - farend_handle : Pointer to the far-end part of the delay estimation
//                        instance created prior to this call using
//                        WebRtc_CreateDelayEstimatorFarend().
//
//                        Note that WebRtc_CreateDelayEstimator does not take
//                        ownership of `farend_handle`, which has to be torn
//                        down properly after this instance.
//
//      - max_lookahead : Maximum amount of non-causal lookahead allowed. The
//                        actual amount of lookahead used can be controlled by
//                        WebRtc_set_lookahead(...). The default `lookahead` is
//                        set to `max_lookahead` at create time. Use
//                        WebRtc_set_lookahead(...) before start if a different
//                        value is desired.
//
//                        Using lookahead can detect cases in which a near-end
//                        signal occurs before the corresponding far-end signal.
//                        It will delay the estimate for the current block by an
//                        equal amount, and the returned values will be offset
//                        by it.
//
//                        A value of zero is the typical no-lookahead case.
//                        This also represents the minimum delay which can be
//                        estimated.
//
//                        Note that the effective range of delay estimates is
//                        [-`lookahead`,... ,`history_size`-`lookahead`)
//                        where `history_size` is set through
//                        WebRtc_set_history_size().
//
// Return value:
//      - void*         : Created `handle`. If the memory can't be allocated or
//                        if any of the input parameters are invalid NULL is
//                        returned.
void* WebRtc_CreateDelayEstimator(void* farend_handle, int max_lookahead);

// Initializes the delay estimation instance returned by
// WebRtc_CreateDelayEstimator(...)
int WebRtc_InitDelayEstimator(void* handle);

// Soft resets the delay estimation instance returned by
// WebRtc_CreateDelayEstimator(...)
// Input:
//      - delay_shift   : The amount of blocks to shift history buffers.
//
// Return value:
//      - actual_shifts : The actual number of shifts performed.
int WebRtc_SoftResetDelayEstimator(void* handle, int delay_shift);

// Sets the effective `history_size` used. Valid values from 2. We simply need
// at least two delays to compare to perform an estimate. If `history_size` is
// changed, buffers are reallocated filling in with zeros if necessary.
// Note that changing the `history_size` affects both buffers in far-end and
// near-end. Hence it is important to change all DelayEstimators that use the
// same reference far-end, to the same `history_size` value.
// Inputs:
//  - handle            : Pointer to the delay estimation instance.
//  - history_size      : Effective history size to be used.
// Return value:
//  - new_history_size  : The new history size used. If the memory was not able
//                        to be allocated 0 is returned.
int WebRtc_set_history_size(void* handle, int history_size);

// Returns the history_size currently used.
// Input:
//      - handle        : Pointer to the delay estimation instance.
int WebRtc_history_size(const void* handle);

// Sets the amount of `lookahead` to use. Valid values are [0, max_lookahead]
// where `max_lookahead` was set at create time through
// WebRtc_CreateDelayEstimator(...).
//
// Input:
//      - handle        : Pointer to the delay estimation instance.
//      - lookahead     : The amount of lookahead to be used.
//
// Return value:
//      - new_lookahead : The actual amount of lookahead set, unless `handle` is
//                        a NULL pointer or `lookahead` is invalid, for which an
//                        error is returned.
int WebRtc_set_lookahead(void* handle, int lookahead);

// Returns the amount of lookahead we currently use.
// Input:
//      - handle        : Pointer to the delay estimation instance.
int WebRtc_lookahead(void* handle);

// Sets the `allowed_offset` used in the robust validation scheme.  If the
// delay estimator is used in an echo control component, this parameter is
// related to the filter length.  In principle `allowed_offset` should be set to
// the echo control filter length minus the expected echo duration, i.e., the
// delay offset the echo control can handle without quality regression.  The
// default value, used if not set manually, is zero.  Note that `allowed_offset`
// has to be non-negative.
// Inputs:
//  - handle            : Pointer to the delay estimation instance.
//  - allowed_offset    : The amount of delay offset, measured in partitions,
//                        the echo control filter can handle.
int WebRtc_set_allowed_offset(void* handle, int allowed_offset);

// Returns the `allowed_offset` in number of partitions.
int WebRtc_get_allowed_offset(const void* handle);

// Enables/Disables a robust validation functionality in the delay estimation.
// This is by default set to disabled at create time.  The state is preserved
// over a reset.
// Inputs:
//      - handle        : Pointer to the delay estimation instance.
//      - enable        : Enable (1) or disable (0) this feature.
int WebRtc_enable_robust_validation(void* handle, int enable);

// Returns 1 if robust validation is enabled and 0 if disabled.
int WebRtc_is_robust_validation_enabled(const void* handle);

// Estimates and returns the delay between the far-end and near-end blocks. The
// value will be offset by the lookahead (i.e. the lookahead should be
// subtracted from the returned value).
// Inputs:
//      - handle        : Pointer to the delay estimation instance.
//      - near_spectrum : Pointer to the near-end spectrum data of the current
//                        block.
//      - spectrum_size : The size of the data arrays (same for both far- and
//                        near-end).
//      - near_q        : The Q-domain of the near-end data.
//
// Output:
//      - handle        : Updated instance.
//
// Return value:
//      - delay         :  >= 0 - Calculated delay value.
//                        -1    - Error.
//                        -2    - Insufficient data for estimation.
int WebRtc_DelayEstimatorProcessFix(void* handle,
                                    const uint16_t* near_spectrum,
                                    int spectrum_size,
                                    int near_q);

// See WebRtc_DelayEstimatorProcessFix() for description.
int WebRtc_DelayEstimatorProcessFloat(void* handle,
                                      const float* near_spectrum,
                                      int spectrum_size);

// Returns the last calculated delay updated by the function
// WebRtc_DelayEstimatorProcess(...).
//
// Input:
//      - handle        : Pointer to the delay estimation instance.
//
// Return value:
//      - delay         : >= 0  - Last calculated delay value.
//                        -1    - Error.
//                        -2    - Insufficient data for estimation.
int WebRtc_last_delay(void* handle);

// Returns the estimation quality/probability of the last calculated delay
// updated by the function WebRtc_DelayEstimatorProcess(...). The estimation
// quality is a value in the interval [0, 1]. The higher the value, the better
// the quality.
//
// Return value:
//      - delay_quality : >= 0  - Estimation quality of last calculated delay.
float WebRtc_last_delay_quality(void* handle);

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_UTILITY_DELAY_ESTIMATOR_WRAPPER_H_
