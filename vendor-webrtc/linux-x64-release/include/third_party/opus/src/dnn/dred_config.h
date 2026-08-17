/* Copyright (c) 2022 Amazon
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

#ifndef DRED_CONFIG_H
#define DRED_CONFIG_H

/* Change this once DRED gets an extension number assigned. */
#define DRED_EXTENSION_ID 126

/* Remove these two completely once DRED gets an extension number assigned. */
#define DRED_EXPERIMENTAL_VERSION 10
#define DRED_EXPERIMENTAL_BYTES 2


#define DRED_MIN_BYTES 8

/* these are inpart duplicates to the values defined in dred_rdovae_constants.h */
#define DRED_SILK_ENCODER_DELAY (79+12-80)
#define DRED_FRAME_SIZE 160
#define DRED_DFRAME_SIZE (2 * (DRED_FRAME_SIZE))
#define DRED_MAX_DATA_SIZE 1000
#define DRED_ENC_Q0 6
#define DRED_ENC_Q1 15

/* Covers 1.04 second so we can cover one second, after the lookahead. */
#define DRED_MAX_LATENTS 26
#define DRED_NUM_REDUNDANCY_FRAMES (2*DRED_MAX_LATENTS)
#define DRED_MAX_FRAMES (4*DRED_MAX_LATENTS)

#endif
