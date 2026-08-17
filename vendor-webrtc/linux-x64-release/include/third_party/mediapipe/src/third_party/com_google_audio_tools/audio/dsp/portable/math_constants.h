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

/* Definitions of math constants.
 *
 * math.h implementations often define M_PI, M_SQRT2, etc., but the C/C++
 * standards do not require it. This header ensures the basic ones are defined.
 */

#ifndef AUDIO_DSP_PORTABLE_MATH_CONSTANTS_H_
#define AUDIO_DSP_PORTABLE_MATH_CONSTANTS_H_

/* Let constant defined in math.h take precedence over ours. */
#include <math.h>

#ifdef __cplusplus
extern "C" {
#endif

/* e, Euler's number. */
#ifndef M_E
#define M_E 2.71828182845904524
#endif

/* log(2), natural log of 2. */
#ifndef M_LN2
#define M_LN2 0.69314718055994531
#endif

/* log(10), natural log of 10. */
#ifndef M_LN10
#define M_LN10 2.30258509299404568
#endif

/* The constant pi. */
#ifndef M_PI
#define M_PI 3.14159265358979324
#endif

/* sqrt(2). */
#ifndef M_SQRT2
#define M_SQRT2 1.41421356237309505
#endif

#ifdef __cplusplus
} /* extern "C" */
#endif
#endif  /* AUDIO_DSP_PORTABLE_MATH_CONSTANTS_H_ */
