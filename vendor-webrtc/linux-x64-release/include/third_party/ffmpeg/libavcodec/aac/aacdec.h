/*
 * AAC decoder definitions and structures
 * Copyright (c) 2005-2006 Oded Shimon ( ods15 ods15 dyndns org )
 * Copyright (c) 2006-2007 Maxim Gavrilov ( maxim.gavrilov gmail com )
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
 * AAC decoder definitions and structures
 * @author Oded Shimon  ( ods15 ods15 dyndns org )
 * @author Maxim Gavrilov ( maxim.gavrilov gmail com )
 */

#ifndef AVCODEC_AAC_AACDEC_H
#define AVCODEC_AAC_AACDEC_H

#include <stdint.h>

#include "libavutil/channel_layout.h"
#include "libavutil/float_dsp.h"
#include "libavutil/fixed_dsp.h"
#include "libavutil/mem_internal.h"
#include "libavutil/tx.h"

#include "libavcodec/aac.h"
#include "libavcodec/avcodec.h"
#include "libavcodec/mpeg4audio.h"

#include "aacdec_ac.h"

typedef struct AACDecContext AACDecContext;

/**
 * Output configuration status
 */
enum OCStatus {
    OC_NONE,        ///< Output unconfigured
    OC_TRIAL_PCE,   ///< Output configuration under trial specified by an inband PCE
    OC_TRIAL_FRAME, ///< Output configuration under trial specified by a frame header
    OC_GLOBAL_HDR,  ///< Output configuration set in a global header but not yet locked
    OC_LOCKED,      ///< Output configuration locked in place
};

enum AACOutputChannelOrder {
    CHANNEL_ORDER_DEFAULT,
    CHANNEL_ORDER_CODED,
};

/**
 * The point during decoding at which channel coupling is applied.
 */
enum CouplingPoint {
    BEFORE_TNS,
    BETWEEN_TNS_AND_IMDCT,
    AFTER_IMDCT = 3,
};

enum AACUsacElem {
    ID_USAC_SCE = 0,
    ID_USAC_CPE = 1,
    ID_USAC_LFE = 2,
    ID_USAC_EXT = 3,
};

enum ExtensionHeaderType {
    ID_CONFIG_EXT_FILL = 0,
    ID_CONFIG_EXT_LOUDNESS_INFO = 2,
    ID_CONFIG_EXT_STREAM_ID = 7,
};

enum AACUsacExtension {
    ID_EXT_ELE_FILL,
    ID_EXT_ELE_MPEGS,
    ID_EXT_ELE_SAOC,
    ID_EXT_ELE_AUDIOPREROLL,
    ID_EXT_ELE_UNI_DRC,
};

enum AACUSACLoudnessExt {
    UNIDRCLOUDEXT_TERM = 0x0,
    UNIDRCLOUDEXT_EQ = 0x1,
};

// Supposed to be equal to AAC_RENAME() in case of USE_FIXED.
#define RENAME_FIXED(name) name ## _fixed

#define INTFLOAT_UNION(name, elems)     \
    union {                             \
        int   RENAME_FIXED(name) elems; \
        float name       elems;         \
    }

#define INTFLOAT_ALIGNED_UNION(alignment, name, nb_elems)                \
    union {                                                              \
        DECLARE_ALIGNED(alignment, int,   RENAME_FIXED(name))[nb_elems]; \
        DECLARE_ALIGNED(alignment, float, name)[nb_elems];               \
    }
/**
 * Long Term Prediction
 */
typedef struct LongTermPrediction {
    int8_t present;
    int16_t lag;
    INTFLOAT_UNION(coef,);
    int8_t used[MAX_LTP_LONG_SFB];
} LongTermPrediction;

/* Per channel core mode */
typedef struct AACUsacElemData {
    uint8_t core_mode;
    uint8_t scale_factor_grouping;
    uint8_t tns_data_present;

    /* Timewarping ratio */
#define NUM_TW_NODES 16
    uint8_t tw_ratio[NUM_TW_NODES];

    struct {
        uint8_t acelp_core_mode : 3;
        uint8_t lpd_mode : 5;

        uint8_t bpf_control_info : 1;
        uint8_t core_mode_last : 1;
        uint8_t fac_data_present : 1;

        int last_lpd_mode;
    } ldp;

    struct {
        unsigned int seed;
        uint8_t level : 3;
        uint8_t offset : 5;
    } noise;

    struct {
        uint8_t gain;
        uint32_t kv[8 /* (1024 / 16) / 8 */][8];
    } fac;

    AACArithState ac;
} AACUsacElemData;

/**
 * Individual Channel Stream
 */
typedef struct IndividualChannelStream {
    uint8_t max_sfb;            ///< number of scalefactor bands per group
    enum WindowSequence window_sequence[2];
    uint8_t use_kb_window[2];   ///< If set, use Kaiser-Bessel window, otherwise use a sine window.
    int num_window_groups;
    int prev_num_window_groups; ///< Previous frame's number of window groups
    uint8_t group_len[8];
    LongTermPrediction ltp;
    const uint16_t *swb_offset; ///< table of offsets to the lowest spectral coefficient of a scalefactor band, sfb, for a particular window
    int num_swb;                ///< number of scalefactor window bands
    int num_windows;
    int tns_max_bands;
    int predictor_present;
    int predictor_initialized;
    int predictor_reset_group;
    uint8_t prediction_used[41];
    uint8_t window_clipping[8]; ///< set if a certain window is near clipping
} IndividualChannelStream;

/**
 * Temporal Noise Shaping
 */
typedef struct TemporalNoiseShaping {
    int present;
    int n_filt[8];
    int length[8][4];
    int direction[8][4];
    int order[8][4];
    INTFLOAT_UNION(coef, [8][4][TNS_MAX_ORDER]);
} TemporalNoiseShaping;

/**
 * coupling parameters
 */
typedef struct ChannelCoupling {
    enum CouplingPoint coupling_point;  ///< The point during decoding at which coupling is applied.
    int num_coupled;       ///< number of target elements
    enum RawDataBlockType type[8];   ///< Type of channel element to be coupled - SCE or CPE.
    int id_select[8];      ///< element id
    int ch_select[8];      /**< [0] shared list of gains; [1] list of gains for right channel;
                            *   [2] list of gains for left channel; [3] lists of gains for both channels
                            */
    INTFLOAT_UNION(gain, [16][120]);
} ChannelCoupling;

/**
 * Single Channel Element - used for both SCE and LFE elements.
 */
typedef struct SingleChannelElement {
    IndividualChannelStream ics;
    AACUsacElemData ue;                             ///< USAC element data
    TemporalNoiseShaping tns;
    enum BandType band_type[128];                   ///< band types
    int sfo[128];                                   ///< scalefactor offsets
    INTFLOAT_UNION(sf, [128]);                      ///< scalefactors (8 windows * 16 sfb max)
    INTFLOAT_ALIGNED_UNION(32, coeffs,    1024);    ///< coefficients for IMDCT, maybe processed
    INTFLOAT_ALIGNED_UNION(32, prev_coeffs, 1024);  ///< unscaled previous contents of coeffs[] for USAC
    INTFLOAT_ALIGNED_UNION(32, saved,     1536);    ///< overlap
    INTFLOAT_ALIGNED_UNION(32, ret_buf,   2048);    ///< PCM output buffer
    INTFLOAT_ALIGNED_UNION(16, ltp_state, 3072);    ///< time signal for LTP
    union {
        struct PredictorStateFixed *RENAME_FIXED(predictor_state);
        struct PredictorState      *predictor_state;
    };
    union {
        float *output;                              ///< PCM output
        int   *RENAME_FIXED(output);                ///< PCM output
    };
} SingleChannelElement;

typedef struct AACUsacStereo {
    uint8_t common_window;
    uint8_t common_tw;
    uint8_t tns_on_lr; ///< Apply TNS before M/S and stereo prediction

    uint8_t ms_mask_mode;
    uint8_t config_idx;

    /* Complex prediction */
    uint8_t use_prev_frame;
    uint8_t pred_dir;
    uint8_t complex_coef;

    uint8_t pred_used[128];

    INTFLOAT_ALIGNED_UNION(32, alpha_q_re, 1024);
    INTFLOAT_ALIGNED_UNION(32, alpha_q_im, 1024);
    INTFLOAT_ALIGNED_UNION(32, prev_alpha_q_re, 1024);
    INTFLOAT_ALIGNED_UNION(32, prev_alpha_q_im, 1024);

    INTFLOAT_ALIGNED_UNION(32, dmix_re, 1024);
    INTFLOAT_ALIGNED_UNION(32, prev_dmix_re, 1024); /* Recalculated on every frame */
    INTFLOAT_ALIGNED_UNION(32, dmix_im, 1024); /* Final prediction data */
} AACUsacStereo;

/**
 * channel element - generic struct for SCE/CPE/CCE/LFE
 */
typedef struct ChannelElement {
    int present;
    // CPE specific
    uint8_t max_sfb_ste;      ///< (USAC) Maximum of both max_sfb values
    uint8_t ms_mask[128];     ///< Set if mid/side stereo is used for each scalefactor window band
    // shared
    SingleChannelElement ch[2];
    // CCE specific
    ChannelCoupling coup;
    // USAC stereo coupling data
    AACUsacStereo us;
} ChannelElement;

typedef struct AACUSACLoudnessInfo {
    uint8_t drc_set_id : 6;
    uint8_t downmix_id : 7;
    struct {
        uint16_t lvl : 12;
        uint8_t present : 1;
    } sample_peak;

    struct {
        uint16_t lvl : 12;
        uint8_t measurement : 4;
        uint8_t reliability : 2;
        uint8_t present : 1;
    } true_peak;

    uint8_t nb_measurements : 4;
    struct {
        uint8_t method_def : 4;
        uint8_t method_val;
        uint8_t measurement : 4;
        uint8_t reliability : 2;
    } measurements[16];
} AACUSACLoudnessInfo;

typedef struct AACUsacElemConfig {
    enum AACUsacElem type;

    uint8_t tw_mdct : 1;
    uint8_t noise_fill : 1;

    uint8_t stereo_config_index;

    struct {
        int ratio;

        uint8_t harmonic_sbr : 1; /* harmonicSBR */
        uint8_t bs_intertes : 1; /* bs_interTes */
        uint8_t bs_pvc : 1; /* bs_pvc */

        struct {
            uint8_t start_freq; /* dflt_start_freq */
            uint8_t stop_freq; /* dflt_stop_freq */

            uint8_t freq_scale; /* dflt_freq_scale */
            uint8_t alter_scale : 1; /* dflt_alter_scale */
            uint8_t noise_bands; /* dflt_noise_bands */

            uint8_t limiter_bands; /* dflt_limiter_bands */
            uint8_t limiter_gains; /* dflt_limiter_gains */
            uint8_t interpol_freq : 1; /* dflt_interpol_freq */
            uint8_t smoothing_mode : 1; /* dflt_smoothing_mode */
        } dflt;
    } sbr;

    struct {
        uint8_t freq_res; /* bsFreqRes */
        uint8_t fixed_gain; /* bsFixedGainDMX */
        uint8_t temp_shape_config; /* bsTempShapeConfig */
        uint8_t decorr_config; /* bsDecorrConfig */
        uint8_t high_rate_mode : 1; /* bsHighRateMode */
        uint8_t phase_coding : 1; /* bsPhaseCoding */

        uint8_t otts_bands_phase; /* bsOttBandsPhase */
        uint8_t residual_coding; /* bsResidualCoding */
        uint8_t residual_bands; /* bsResidualBands */
        uint8_t pseudo_lr : 1; /* bsPseudoLr */
        uint8_t env_quant_mode : 1; /* bsEnvQuantMode */
    } mps;

    struct {
        enum AACUsacExtension type;
        uint8_t payload_frag;
        uint32_t default_len;
        uint32_t pl_data_offset;
        uint8_t *pl_data;
    } ext;
} AACUsacElemConfig;

typedef struct AACUSACConfig {
    uint8_t core_sbr_frame_len_idx; /* coreSbrFrameLengthIndex */
    uint16_t core_frame_len;
    uint16_t stream_identifier;

    AACUsacElemConfig elems[64];
    int nb_elems;

    struct {
        uint8_t nb_album;
        AACUSACLoudnessInfo album_info[64];
        uint8_t nb_info;
        AACUSACLoudnessInfo info[64];
    } loudness;
} AACUSACConfig;

typedef struct OutputConfiguration {
    MPEG4AudioConfig m4ac;
    uint8_t layout_map[MAX_ELEM_ID*4][3];
    int layout_map_tags;
    AVChannelLayout ch_layout;
    enum OCStatus status;
    AACUSACConfig usac;
} OutputConfiguration;

/**
 * Dynamic Range Control - decoded from the bitstream but not processed further.
 */
typedef struct DynamicRangeControl {
    int pce_instance_tag;                           ///< Indicates with which program the DRC info is associated.
    int dyn_rng_sgn[17];                            ///< DRC sign information; 0 - positive, 1 - negative
    int dyn_rng_ctl[17];                            ///< DRC magnitude information
    int exclude_mask[MAX_CHANNELS];                 ///< Channels to be excluded from DRC processing.
    int band_incr;                                  ///< Number of DRC bands greater than 1 having DRC info.
    int interpolation_scheme;                       ///< Indicates the interpolation scheme used in the SBR QMF domain.
    int band_top[17];                               ///< Indicates the top of the i-th DRC band in units of 4 spectral lines.
    int prog_ref_level;                             /**< A reference level for the long-term program audio level for all
                                                     *   channels combined.
                                                     */
} DynamicRangeControl;

/**
 * Decode-specific primitives
 */
typedef struct AACDecProc {
    int (*decode_spectrum_and_dequant)(AACDecContext *ac,
                                       GetBitContext *gb,
                                       const Pulse *pulse,
                                       SingleChannelElement *sce);

    int (*decode_cce)(AACDecContext *ac, GetBitContext *gb, ChannelElement *che);

    int (*sbr_ctx_alloc_init)(AACDecContext *ac, ChannelElement **che, int id_aac);
    int (*sbr_decode_extension)(AACDecContext *ac, ChannelElement *che,
                                GetBitContext *gb, int crc, int cnt, int id_aac);
    void (*sbr_apply)(AACDecContext *ac, ChannelElement *che,
                      int id_aac, void /* INTFLOAT */ *L, void /* INTFLOAT */ *R);
    void (*sbr_ctx_close)(ChannelElement *che);
} AACDecProc;

/**
 * DSP-specific primitives
 */
typedef struct AACDecDSP {
    void (*dequant_scalefactors)(SingleChannelElement *sce);

    void (*apply_mid_side_stereo)(AACDecContext *ac, ChannelElement *cpe);
    void (*apply_intensity_stereo)(AACDecContext *ac, ChannelElement *cpe,
                                   int ms_present);

    void (*apply_tns)(void *_coef_param, TemporalNoiseShaping *tns,
                      IndividualChannelStream *ics, int decode);

    void (*apply_ltp)(AACDecContext *ac, SingleChannelElement *sce);
    void (*update_ltp)(AACDecContext *ac, SingleChannelElement *sce);

    void (*apply_prediction)(AACDecContext *ac, SingleChannelElement *sce);

    void (*apply_dependent_coupling)(AACDecContext *ac,
                                     SingleChannelElement *target,
                                     ChannelElement *cce, int index);
    void (*apply_independent_coupling)(AACDecContext *ac,
                                       SingleChannelElement *target,
                                       ChannelElement *cce, int index);

    void (*imdct_and_windowing)(AACDecContext *ac, SingleChannelElement *sce);
    void (*imdct_and_windowing_768)(AACDecContext *ac, SingleChannelElement *sce);
    void (*imdct_and_windowing_960)(AACDecContext *ac, SingleChannelElement *sce);
    void (*imdct_and_windowing_ld)(AACDecContext *ac, SingleChannelElement *sce);
    void (*imdct_and_windowing_eld)(AACDecContext *ac, SingleChannelElement *sce);

    void (*clip_output)(AACDecContext *ac, ChannelElement *che, int type, int samples);
} AACDecDSP;

/**
 * main AAC decoding context
 */
struct AACDecContext {
    const struct AVClass  *class;
    struct AVCodecContext *avctx;

    AACDecDSP dsp;
    AACDecProc proc;

    struct AVFrame *frame;

    int is_saved;                 ///< Set if elements have stored overlap from previous frame.
    DynamicRangeControl che_drc;

    /**
     * @name Channel element related data
     * @{
     */
    ChannelElement          *che[4][MAX_ELEM_ID];
    ChannelElement  *tag_che_map[4][MAX_ELEM_ID];
    int tags_mapped;
    int warned_remapping_once;
    /** @} */

    /**
     * @name temporary aligned temporary buffers
     * (We do not want to have these on the stack.)
     * @{
     */
    INTFLOAT_ALIGNED_UNION(32, buf_mdct, 1024);
    INTFLOAT_ALIGNED_UNION(32, temp, 128);
    /** @} */

    /**
     * @name Computed / set up during initialization
     * @{
     */
    AVTXContext *mdct96;
    AVTXContext *mdct120;
    AVTXContext *mdct128;
    AVTXContext *mdct480;
    AVTXContext *mdct512;
    AVTXContext *mdct768;
    AVTXContext *mdct960;
    AVTXContext *mdct1024;
    AVTXContext *mdct_ltp;

    av_tx_fn mdct96_fn;
    av_tx_fn mdct120_fn;
    av_tx_fn mdct128_fn;
    av_tx_fn mdct480_fn;
    av_tx_fn mdct512_fn;
    av_tx_fn mdct768_fn;
    av_tx_fn mdct960_fn;
    av_tx_fn mdct1024_fn;
    av_tx_fn mdct_ltp_fn;
    union {
        AVFixedDSPContext *RENAME_FIXED(fdsp);
        AVFloatDSPContext *fdsp;
    };
    int random_state;
    /** @} */

    /**
     * @name Members used for output
     * @{
     */
    SingleChannelElement *output_element[MAX_CHANNELS]; ///< Points to each SingleChannelElement
    /** @} */


    /**
     * @name Japanese DTV specific extension
     * @{
     */
    int force_dmono_mode;///< 0->not dmono, 1->use first channel, 2->use second channel
    int dmono_mode;      ///< 0->not dmono, 1->use first channel, 2->use second channel
    /** @} */

    enum AACOutputChannelOrder output_channel_order;

    OutputConfiguration oc[2];
    int warned_num_aac_frames;
    int warned_960_sbr;
    unsigned warned_71_wide;
    int warned_gain_control;
    int warned_he_aac_mono;

    int is_fixed;
};

#if defined(USE_FIXED) && USE_FIXED
#define fdsp          RENAME_FIXED(fdsp)
#endif

int ff_aac_decode_init(AVCodecContext *avctx);
int ff_aac_decode_init_float(AVCodecContext *avctx);
int ff_aac_decode_init_fixed(AVCodecContext *avctx);

int ff_aac_decode_ics(AACDecContext *ac, SingleChannelElement *sce,
                      GetBitContext *gb, int common_window, int scale_flag);

int ff_aac_decode_tns(AACDecContext *ac, TemporalNoiseShaping *tns,
                      GetBitContext *gb, const IndividualChannelStream *ics);

int ff_aac_set_default_channel_config(AACDecContext *ac, AVCodecContext *avctx,
                                      uint8_t (*layout_map)[3],
                                      int *tags,
                                      int channel_config);

int ff_aac_output_configure(AACDecContext *ac,
                            uint8_t layout_map[MAX_ELEM_ID * 4][3], int tags,
                            enum OCStatus oc_type, int get_new_frame);

ChannelElement *ff_aac_get_che(AACDecContext *ac, int type, int elem_id);

#endif /* AVCODEC_AAC_AACDEC_H */
