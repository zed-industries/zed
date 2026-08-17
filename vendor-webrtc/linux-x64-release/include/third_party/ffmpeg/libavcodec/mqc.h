/*
 * MQ-coder: structures, common and decoder functions
 * Copyright (c) 2007 Kamil Nowosad
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_MQC_H
#define AVCODEC_MQC_H

/**
 * MQ-coder
 * @file
 * @author Kamil Nowosad
 */

#include <stdint.h>

#define MQC_CX_UNI 17
#define MQC_CX_RL  18

extern const uint16_t ff_mqc_qe[2 * 47];
extern const uint8_t  ff_mqc_nlps[2 * 47];
extern const uint8_t  ff_mqc_nmps[2 * 47];

typedef struct MqcState {
    uint8_t *bp, *bpstart;
    unsigned int a;
    unsigned int c;
    unsigned int ct;
    uint8_t cx_states[19];
    int raw;
} MqcState;

/* encoder */

/** initialize the encoder */
void ff_mqc_initenc(MqcState *mqc, uint8_t *bp);

/** code bit d with context cx */
void ff_mqc_encode(MqcState *mqc, uint8_t *cxstate, int d);

/** flush the encoder [returns number of bytes encoded] */
int ff_mqc_flush_to(MqcState *mqc, uint8_t *dst, int *dst_len);

/* decoder */

/**
 * Initialize MQ-decoder.
 * @param mqc   MQ decoder state
 * @param bp    byte pointer
 * @param raw   raw mode
 * @param reset reset states
 */
void ff_mqc_initdec(MqcState *mqc, uint8_t *bp, int raw, int reset);

/**
 * MQ decoder.
 * @param mqc       MQ decoder state
 * @param cxstate   Context
 * @return          Decision (0 to 1)
 */
int ff_mqc_decode(MqcState *mqc, uint8_t *cxstate);

/* common */

/**
 * MQ-coder context initialisations.
 * @param mqc       MQ-coder context
 */
void ff_mqc_init_contexts(MqcState *mqc);

#endif /* AVCODEC_MQC_H */
