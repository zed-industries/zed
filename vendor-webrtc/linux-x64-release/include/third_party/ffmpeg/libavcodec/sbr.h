/*
 * Spectral Band Replication definitions and structures
 * Copyright (c) 2008-2009 Robert Swain ( rob opendot cl )
 * Copyright (c) 2010      Alex Converse <alex.converse@gmail.com>
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

/**
 * @file
 * Spectral Band Replication definitions and structures
 * @author Robert Swain ( rob opendot cl )
 */

#ifndef AVCODEC_SBR_H
#define AVCODEC_SBR_H

#include <stdint.h>

#include "libavutil/mem_internal.h"
#include "libavutil/tx.h"

#include "aacps.h"
#include "sbrdsp.h"

/**
 * Spectral Band Replication header - spectrum parameters that invoke a reset if they differ from the previous header.
 */
typedef struct SpectrumParameters {
    uint8_t bs_start_freq;
    uint8_t bs_stop_freq;
    uint8_t bs_xover_band;

    /**
     * @name Variables associated with bs_header_extra_1
     * @{
     */
    uint8_t bs_freq_scale;
    uint8_t bs_alter_scale;
    uint8_t bs_noise_bands;
    /** @} */
} SpectrumParameters;

#define SBR_SYNTHESIS_BUF_SIZE ((1280-128)*2)

/**
 * Spectral Band Replication per channel data
 */
typedef struct SBRData {
    /**
     * @name Main bitstream data variables
     * @{
     */
    unsigned           bs_frame_class;
    unsigned           bs_add_harmonic_flag;
    AAC_SIGNE          bs_num_env;
    uint8_t            bs_freq_res[9];
    AAC_SIGNE          bs_num_noise;
    uint8_t            bs_df_env[9];
    uint8_t            bs_df_noise[2];
    uint8_t            bs_invf_mode[2][5];
    uint8_t            bs_add_harmonic[48];
    unsigned           bs_amp_res;
    /** @} */

    /**
     * @name State variables
     * @{
     */
    DECLARE_ALIGNED(32, INTFLOAT, synthesis_filterbank_samples)[SBR_SYNTHESIS_BUF_SIZE];
    DECLARE_ALIGNED(32, INTFLOAT, analysis_filterbank_samples) [1312];
    int                synthesis_filterbank_samples_offset;
    ///l_APrev and l_A
    int                e_a[2];
    ///Chirp factors
    INTFLOAT              bw_array[5];
    ///QMF values of the original signal
    INTFLOAT              W[2][32][32][2];
    ///QMF output of the HF adjustor
    int                Ypos;
    DECLARE_ALIGNED(16, INTFLOAT, Y)[2][38][64][2];
    DECLARE_ALIGNED(16, AAC_FLOAT, g_temp)[42][48];
    AAC_FLOAT          q_temp[42][48];
    uint8_t            s_indexmapped[9][48];
    ///Envelope scalefactors
    uint8_t            env_facs_q[9][48];
    AAC_FLOAT          env_facs[9][48];
    ///Noise scalefactors
    uint8_t            noise_facs_q[3][5];
    AAC_FLOAT          noise_facs[3][5];
    ///Envelope time borders
    uint8_t            t_env[9];
    ///Envelope time border of the last envelope of the previous frame
    uint8_t            t_env_num_env_old;
    ///Noise time borders
    uint8_t            t_q[3];
    unsigned           f_indexnoise;
    unsigned           f_indexsine;
    //inter_tes (USAC)
    uint8_t            temp_shape[6];
    uint8_t            temp_shape_mode[6];
    /** @} */
} SBRData;

typedef struct SpectralBandReplication SpectralBandReplication;

/**
 * aacsbr functions pointers
 */
typedef struct AACSBRContext {
    int (*sbr_lf_gen)(SpectralBandReplication *sbr,
                      INTFLOAT X_low[32][40][2], const INTFLOAT W[2][32][32][2],
                      int buf_idx);
    void (*sbr_hf_assemble)(INTFLOAT Y1[38][64][2],
                            const INTFLOAT X_high[64][40][2],
                            SpectralBandReplication *sbr, SBRData *ch_data,
                            const int e_a[2]);
    int (*sbr_x_gen)(SpectralBandReplication *sbr, INTFLOAT X[2][38][64],
                     const INTFLOAT Y0[38][64][2], const INTFLOAT Y1[38][64][2],
                     const INTFLOAT X_low[32][40][2], int ch);
    void (*sbr_hf_inverse_filter)(SBRDSPContext *dsp,
                                  INTFLOAT (*alpha0)[2], INTFLOAT (*alpha1)[2],
                                  const INTFLOAT X_low[32][40][2], int k0);
} AACSBRContext;

/**
 * Spectral Band Replication
 */
struct SpectralBandReplication {
    int                sample_rate;
    int                start;
    int                ready_for_dequant;
    int                id_aac;
    int                usac;
    int                inter_tes; // USAC-only
    int                reset;
    SpectrumParameters spectrum_params;
    int                bs_amp_res_header;
    int                bs_sbr_preprocessing; // USAC-only
    /**
     * @name Variables associated with bs_header_extra_2
     * @{
     */
    unsigned           bs_limiter_bands;
    unsigned           bs_limiter_gains;
    unsigned           bs_interpol_freq;
    unsigned           bs_smoothing_mode;
    /** @} */
    unsigned           bs_coupling;
    AAC_SIGNE          k[5]; ///< k0, k1, k2
    ///kx', and kx respectively, kx is the first QMF subband where SBR is used.
    ///kx' is its value from the previous frame
    AAC_SIGNE          kx[2];
    ///M' and M respectively, M is the number of QMF subbands that use SBR.
    AAC_SIGNE          m[2];
    unsigned           kx_and_m_pushed;
    ///The number of frequency bands in f_master
    AAC_SIGNE          n_master;
    SBRData            data[2];
    PSContext          ps;
    ///N_Low and N_High respectively, the number of frequency bands for low and high resolution
    AAC_SIGNE          n[2];
    ///Number of noise floor bands
    AAC_SIGNE          n_q;
    ///Number of limiter bands
    AAC_SIGNE          n_lim;
    ///The master QMF frequency grouping
    uint16_t           f_master[49];
    ///Frequency borders for low resolution SBR
    uint16_t           f_tablelow[25];
    ///Frequency borders for high resolution SBR
    uint16_t           f_tablehigh[49];
    ///Frequency borders for noise floors
    uint16_t           f_tablenoise[6];
    ///Frequency borders for the limiter
    uint16_t           f_tablelim[30];
    AAC_SIGNE          num_patches;
    uint8_t            patch_num_subbands[6];
    uint8_t            patch_start_subband[6];
    ///QMF low frequency input to the HF generator
    DECLARE_ALIGNED(16, INTFLOAT, X_low)[32][40][2];
    ///QMF output of the HF generator
    DECLARE_ALIGNED(16, INTFLOAT, X_high)[64][40][2];
    ///QMF values of the reconstructed signal
    DECLARE_ALIGNED(16, INTFLOAT, X)[2][2][38][64];
    ///Zeroth coefficient used to filter the subband signals
    DECLARE_ALIGNED(16, INTFLOAT, alpha0)[64][2];
    ///First coefficient used to filter the subband signals
    DECLARE_ALIGNED(16, INTFLOAT, alpha1)[64][2];
    ///Dequantized envelope scalefactors, remapped
    AAC_FLOAT          e_origmapped[8][48];
    ///Dequantized noise scalefactors, remapped
    AAC_FLOAT          q_mapped[8][48];
    ///Sinusoidal presence, remapped
    uint8_t            s_mapped[8][48];
    ///Estimated envelope
    AAC_FLOAT          e_curr[8][48];
    ///Amplitude adjusted noise scalefactors
    AAC_FLOAT          q_m[8][48];
    ///Sinusoidal levels
    AAC_FLOAT          s_m[8][48];
    AAC_FLOAT          gain[8][48];
    DECLARE_ALIGNED(32, INTFLOAT, qmf_filter_scratch)[5][64];
    AVTXContext       *mdct_ana;
    av_tx_fn           mdct_ana_fn;
    AVTXContext       *mdct;
    av_tx_fn           mdct_fn;
    SBRDSPContext      dsp;
    AACSBRContext      c;
};

#endif /* AVCODEC_SBR_H */
