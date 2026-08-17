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

#ifndef DRED_RDOVAE_DEC_H
#define DRED_RDOVAE_DEC_H

#include "dred_rdovae.h"
#include "dred_rdovae_dec_data.h"
#include "dred_rdovae_stats_data.h"

struct RDOVAEDecStruct {
  int initialized;
  float gru1_state[DEC_GRU1_STATE_SIZE];
  float gru2_state[DEC_GRU2_STATE_SIZE];
  float gru3_state[DEC_GRU3_STATE_SIZE];
  float gru4_state[DEC_GRU4_STATE_SIZE];
  float gru5_state[DEC_GRU5_STATE_SIZE];
  float conv1_state[DEC_CONV1_STATE_SIZE];
  float conv2_state[DEC_CONV2_STATE_SIZE];
  float conv3_state[DEC_CONV3_STATE_SIZE];
  float conv4_state[DEC_CONV4_STATE_SIZE];
  float conv5_state[DEC_CONV5_STATE_SIZE];
};

void dred_rdovae_dec_init_states(RDOVAEDecState *h, const RDOVAEDec *model, const float * initial_state, int arch);
void dred_rdovae_decode_qframe(RDOVAEDecState *h, const RDOVAEDec *model, float *qframe, const float * z, int arch);
void DRED_rdovae_decode_all(const RDOVAEDec *model, float *features, const float *state, const float *latents, int nb_latents, int arch);

#endif
