/* Copyright (c) 2023 Amazon
   Written by Jan Buethe */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#ifndef OSCE_CONFIG
#define OSCE_CONFIG

#define OSCE_FEATURES_MAX_HISTORY 350
#define OSCE_FEATURE_DIM 93
#define OSCE_MAX_FEATURE_FRAMES 4

#define OSCE_CLEAN_SPEC_NUM_BANDS 64
#define OSCE_NOISY_SPEC_NUM_BANDS 18

#define OSCE_NO_PITCH_VALUE 7

#define OSCE_PREEMPH 0.85f

#define OSCE_PITCH_HANGOVER 0

#define OSCE_CLEAN_SPEC_START 0
#define OSCE_CLEAN_SPEC_LENGTH 64

#define OSCE_NOISY_CEPSTRUM_START 64
#define OSCE_NOISY_CEPSTRUM_LENGTH 18

#define OSCE_ACORR_START 82
#define OSCE_ACORR_LENGTH 5

#define OSCE_LTP_START 87
#define OSCE_LTP_LENGTH 5

#define OSCE_LOG_GAIN_START 92
#define OSCE_LOG_GAIN_LENGTH 1


#endif
