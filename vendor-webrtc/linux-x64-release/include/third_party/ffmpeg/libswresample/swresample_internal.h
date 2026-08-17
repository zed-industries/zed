/*
 * Copyright (C) 2011-2013 Michael Niedermayer (michaelni@gmx.at)
 *
 * This file is part of libswresample
 *
 * libswresample is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * libswresample is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with libswresample; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef SWRESAMPLE_SWRESAMPLE_INTERNAL_H
#define SWRESAMPLE_SWRESAMPLE_INTERNAL_H

#include "swresample.h"
#include "libavutil/channel_layout.h"
#include "config.h"

#define SWR_CH_MAX 64

#define SQRT3_2      1.22474487139158904909  /* sqrt(3/2) */

#define NS_TAPS 20

#if ARCH_X86_64
typedef int64_t integer;
#else
typedef int integer;
#endif

typedef void (mix_1_1_func_type)(void *out, const void *in, void *coeffp, integer index, integer len);
typedef void (mix_2_1_func_type)(void *out, const void *in1, const void *in2, void *coeffp, integer index1, integer index2, integer len);

typedef void (mix_any_func_type)(uint8_t **out, const uint8_t **in1, void *coeffp, integer len);

typedef struct AudioData{
    uint8_t *ch[SWR_CH_MAX];    ///< samples buffer per channel
    uint8_t *data;              ///< samples buffer
    int ch_count;               ///< number of channels
    int bps;                    ///< bytes per sample
    int count;                  ///< number of samples
    int planar;                 ///< 1 if planar audio, 0 otherwise
    enum AVSampleFormat fmt;    ///< sample format
} AudioData;

struct DitherContext {
    int method;
    int noise_pos;
    float scale;
    float noise_scale;                              ///< Noise scale
    int ns_taps;                                    ///< Noise shaping dither taps
    float ns_scale;                                 ///< Noise shaping dither scale
    float ns_scale_1;                               ///< Noise shaping dither scale^-1
    int ns_pos;                                     ///< Noise shaping dither position
    float ns_coeffs[NS_TAPS];                       ///< Noise shaping filter coefficients
    float ns_errors[SWR_CH_MAX][2*NS_TAPS];
    AudioData noise;                                ///< noise used for dithering
    AudioData temp;                                 ///< temporary storage when writing into the input buffer isn't possible
    int output_sample_bits;                         ///< the number of used output bits, needed to scale dither correctly
};

typedef struct ResampleContext * (* resample_init_func)(struct ResampleContext *c, int out_rate, int in_rate, int filter_size, int phase_shift, int linear,
                                    double cutoff, enum AVSampleFormat format, enum SwrFilterType filter_type, double kaiser_beta, double precision, int cheby, int exact_rational);
typedef void    (* resample_free_func)(struct ResampleContext **c);
typedef int     (* multiple_resample_func)(struct ResampleContext *c, AudioData *dst, int dst_size, AudioData *src, int src_size, int *consumed);
typedef int     (* resample_flush_func)(struct SwrContext *c);
typedef int     (* set_compensation_func)(struct ResampleContext *c, int sample_delta, int compensation_distance);
typedef int64_t (* get_delay_func)(struct SwrContext *s, int64_t base);
typedef int     (* invert_initial_buffer_func)(struct ResampleContext *c, AudioData *dst, const AudioData *src, int src_size, int *dst_idx, int *dst_count);
typedef int64_t (* get_out_samples_func)(struct SwrContext *s, int in_samples);

struct Resampler {
  resample_init_func            init;
  resample_free_func            free;
  multiple_resample_func        multiple_resample;
  resample_flush_func           flush;
  set_compensation_func         set_compensation;
  get_delay_func                get_delay;
  invert_initial_buffer_func    invert_initial_buffer;
  get_out_samples_func          get_out_samples;
};

extern struct Resampler const swri_resampler;
extern struct Resampler const swri_soxr_resampler;

struct SwrContext {
    const AVClass *av_class;                        ///< AVClass used for AVOption and av_log()
    int log_level_offset;                           ///< logging level offset
    void *log_ctx;                                  ///< parent logging context
    enum AVSampleFormat  in_sample_fmt;             ///< input sample format
    enum AVSampleFormat int_sample_fmt;             ///< internal sample format (AV_SAMPLE_FMT_FLTP or AV_SAMPLE_FMT_S16P)
    enum AVSampleFormat out_sample_fmt;             ///< output sample format
    AVChannelLayout used_ch_layout;                 ///< number of used input channels (mapped channel count if channel_map, otherwise in.ch_count)
    AVChannelLayout  in_ch_layout;                  ///< input channel layout
    AVChannelLayout out_ch_layout;                  ///< output channel layout
    int      in_sample_rate;                        ///< input sample rate
    int     out_sample_rate;                        ///< output sample rate
    int flags;                                      ///< miscellaneous flags such as SWR_FLAG_RESAMPLE
    float slev;                                     ///< surround mixing level
    float clev;                                     ///< center mixing level
    float lfe_mix_level;                            ///< LFE mixing level
    float rematrix_volume;                          ///< rematrixing volume coefficient
    float rematrix_maxval;                          ///< maximum value for rematrixing output
    int matrix_encoding;                            /**< matrixed stereo encoding */
    const int *channel_map;                         ///< channel index (or -1 if muted channel) map
    int engine;

    AVChannelLayout user_used_chlayout;             ///< User set used channel layout
    AVChannelLayout user_in_chlayout;               ///< User set input channel layout
    AVChannelLayout user_out_chlayout;              ///< User set output channel layout
    enum AVSampleFormat user_int_sample_fmt;        ///< User set internal sample format
    int user_dither_method;                         ///< User set dither method

    struct DitherContext dither;

    int filter_size;                                /**< length of each FIR filter in the resampling filterbank relative to the cutoff frequency */
    int phase_shift;                                /**< log2 of the number of entries in the resampling polyphase filterbank */
    int linear_interp;                              /**< if 1 then the resampling FIR filter will be linearly interpolated */
    int exact_rational;                             /**< if 1 then enable non power of 2 phase_count */
    double cutoff;                                  /**< resampling cutoff frequency (swr: 6dB point; soxr: 0dB point). 1.0 corresponds to half the output sample rate */
    int filter_type;                                /**< swr resampling filter type */
    double kaiser_beta;                                /**< swr beta value for Kaiser window (only applicable if filter_type == AV_FILTER_TYPE_KAISER) */
    double precision;                               /**< soxr resampling precision (in bits) */
    int cheby;                                      /**< soxr: if 1 then passband rolloff will be none (Chebyshev) & irrational ratio approximation precision will be higher */

    float min_compensation;                         ///< swr minimum below which no compensation will happen
    float min_hard_compensation;                    ///< swr minimum below which no silence inject / sample drop will happen
    float soft_compensation_duration;               ///< swr duration over which soft compensation is applied
    float max_soft_compensation;                    ///< swr maximum soft compensation in seconds over soft_compensation_duration
    float async;                                    ///< swr simple 1 parameter async, similar to ffmpegs -async
    int64_t firstpts_in_samples;                    ///< swr first pts in samples

    int resample_first;                             ///< 1 if resampling must come first, 0 if rematrixing
    int rematrix;                                   ///< flag to indicate if rematrixing is needed (basically if input and output layouts mismatch)
    int rematrix_custom;                            ///< flag to indicate that a custom matrix has been defined

    AudioData in;                                   ///< input audio data
    AudioData postin;                               ///< post-input audio data: used for rematrix/resample
    AudioData midbuf;                               ///< intermediate audio data (postin/preout)
    AudioData preout;                               ///< pre-output audio data: used for rematrix/resample
    AudioData out;                                  ///< converted output audio data
    AudioData in_buffer;                            ///< cached audio data (convert and resample purpose)
    AudioData silence;                              ///< temporary with silence
    AudioData drop_temp;                            ///< temporary used to discard output
    int in_buffer_index;                            ///< cached buffer position
    int in_buffer_count;                            ///< cached buffer length
    int resample_in_constraint;                     ///< 1 if the input end was reach before the output end, 0 otherwise
    int flushed;                                    ///< 1 if data is to be flushed and no further input is expected
    int64_t outpts;                                 ///< output PTS
    int64_t firstpts;                               ///< first PTS
    int drop_output;                                ///< number of output samples to drop
    double delayed_samples_fixup;                   ///< soxr 0.1.1: needed to fixup delayed_samples after flush has been called.

    struct AudioConvert *in_convert;                ///< input conversion context
    struct AudioConvert *out_convert;               ///< output conversion context
    struct AudioConvert *full_convert;              ///< full conversion context (single conversion for input and output)
    struct ResampleContext *resample;               ///< resampling context
    struct Resampler const *resampler;              ///< resampler virtual function table

    double matrix[SWR_CH_MAX][SWR_CH_MAX];          ///< floating point rematrixing coefficients
    float matrix_flt[SWR_CH_MAX][SWR_CH_MAX];       ///< single precision floating point rematrixing coefficients
    uint8_t *native_matrix;
    uint8_t *native_one;
    uint8_t *native_simd_one;
    uint8_t *native_simd_matrix;
    int32_t matrix32[SWR_CH_MAX][SWR_CH_MAX];       ///< 17.15 fixed point rematrixing coefficients
    uint8_t matrix_ch[SWR_CH_MAX][SWR_CH_MAX+1];    ///< Lists of input channels per output channel that have non zero rematrixing coefficients
    mix_1_1_func_type *mix_1_1_f;
    mix_1_1_func_type *mix_1_1_simd;

    mix_2_1_func_type *mix_2_1_f;
    mix_2_1_func_type *mix_2_1_simd;

    mix_any_func_type *mix_any_f;

    /* TODO: callbacks for ASM optimizations */
};

av_warn_unused_result
int swri_realloc_audio(AudioData *a, int count);

void swri_noise_shaping_int16 (SwrContext *s, AudioData *dsts, const AudioData *srcs, const AudioData *noises, int count);
void swri_noise_shaping_int32 (SwrContext *s, AudioData *dsts, const AudioData *srcs, const AudioData *noises, int count);
void swri_noise_shaping_float (SwrContext *s, AudioData *dsts, const AudioData *srcs, const AudioData *noises, int count);
void swri_noise_shaping_double(SwrContext *s, AudioData *dsts, const AudioData *srcs, const AudioData *noises, int count);

av_warn_unused_result
int swri_rematrix_init(SwrContext *s);
void swri_rematrix_free(SwrContext *s);
int swri_rematrix(SwrContext *s, AudioData *out, AudioData *in, int len, int mustcopy);
int swri_rematrix_init_x86(struct SwrContext *s);

av_warn_unused_result
int swri_get_dither(SwrContext *s, void *dst, int len, unsigned seed, enum AVSampleFormat noise_fmt);
av_warn_unused_result
int swri_dither_init(SwrContext *s, enum AVSampleFormat out_fmt, enum AVSampleFormat in_fmt);

void swri_audio_convert_init_aarch64(struct AudioConvert *ac,
                                 enum AVSampleFormat out_fmt,
                                 enum AVSampleFormat in_fmt,
                                 int channels);
void swri_audio_convert_init_arm(struct AudioConvert *ac,
                                 enum AVSampleFormat out_fmt,
                                 enum AVSampleFormat in_fmt,
                                 int channels);
void swri_audio_convert_init_x86(struct AudioConvert *ac,
                                 enum AVSampleFormat out_fmt,
                                 enum AVSampleFormat in_fmt,
                                 int channels);

#endif
