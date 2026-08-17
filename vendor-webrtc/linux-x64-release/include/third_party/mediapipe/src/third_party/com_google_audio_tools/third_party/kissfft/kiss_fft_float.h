/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef THIRD_PARTY_KISSFFT_KISS_FFT_FLOAT_H_
#define THIRD_PARTY_KISSFFT_KISS_FFT_FLOAT_H_

#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef FIXED_POINT
#include <sys/types.h>
#endif

#ifdef USE_SIMD
#include <xmmintrin.h>
#endif

namespace kissfft_float {
#include "kiss_fft.h"
#include "tools/kiss_fftr.h"
}  // namespace kissfft_float
#undef KISS_FFT_H
#endif  // THIRD_PARTY_KISSFFT_KISS_FFT_FLOAT_H_
