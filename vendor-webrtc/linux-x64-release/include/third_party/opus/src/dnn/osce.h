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

#ifndef OSCE_H
#define OSCE_H


#include "opus_types.h"
/*#include "osce_config.h"*/
#ifndef DISABLE_LACE
#include "lace_data.h"
#endif
#ifndef DISABLE_NOLACE
#include "nolace_data.h"
#endif
#include "nndsp.h"
#include "nnet.h"
#include "osce_structs.h"
#include "structs.h"

#define OSCE_METHOD_NONE 0
#ifndef DISABLE_LACE
#define OSCE_METHOD_LACE 1
#endif
#ifndef DISABLE_NOLACE
#define OSCE_METHOD_NOLACE 2
#endif

#if !defined(DISABLE_NOLACE)
#define OSCE_DEFAULT_METHOD OSCE_METHOD_NOLACE
#define OSCE_MAX_RNN_NEURONS NOLACE_FNET_GRU_STATE_SIZE
#elif !defined(DISABLE_LACE)
#define OSCE_DEFAULT_METHOD OSCE_METHOD_LACE
#define OSCE_MAX_RNN_NEURONS LACE_FNET_GRU_STATE_SIZE
#else
#define OSCE_DEFAULT_METHOD OSCE_METHOD_NONE
#define OSCE_MAX_RNN_NEURONS 0
#endif




/* API */


void osce_enhance_frame(
    OSCEModel                   *model,                         /* I    OSCE model struct                           */
    silk_decoder_state          *psDec,                         /* I/O  Decoder state                               */
    silk_decoder_control        *psDecCtrl,                     /* I    Decoder control                             */
    opus_int16                  xq[],                           /* I/O  Decoded speech                              */
    opus_int32                  num_bits,                       /* I    Size of SILK payload in bits                */
    int                         arch                            /* I    Run-time architecture                       */
);


int osce_load_models(OSCEModel *hModel, const void *data, int len);
void osce_reset(silk_OSCE_struct *hOSCE, int method);


#endif
