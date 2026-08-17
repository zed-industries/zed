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

#ifndef THIRD_PARTY_FFT2D_FFT_H__
#define THIRD_PARTY_FFT2D_FFT_H__

#ifdef __cplusplus
extern "C" {
#endif

extern void rdft(int, int, double *, int *, double *);

#ifdef __cplusplus
}
#endif

#endif  // THIRD_PARTY_FFT2D_FFT_H__
