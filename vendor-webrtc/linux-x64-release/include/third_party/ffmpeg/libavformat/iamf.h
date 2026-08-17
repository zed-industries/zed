/*
 * Immersive Audio Model and Formats common helpers and structs
 * Copyright (c) 2023 James Almer <jamrial@gmail.com>
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

#ifndef AVFORMAT_IAMF_H
#define AVFORMAT_IAMF_H

#include <stddef.h>
#include <stdint.h>

#include "libavutil/attributes_internal.h"
#include "libavutil/channel_layout.h"
#include "libavutil/iamf.h"
#include "libavcodec/codec_id.h"
#include "libavcodec/codec_par.h"

#define MAX_IAMF_OBU_HEADER_SIZE (1 + 8 * 3)

// OBU types (section 3.2).
enum IAMF_OBU_Type {
    IAMF_OBU_IA_CODEC_CONFIG        = 0,
    IAMF_OBU_IA_AUDIO_ELEMENT       = 1,
    IAMF_OBU_IA_MIX_PRESENTATION    = 2,
    IAMF_OBU_IA_PARAMETER_BLOCK     = 3,
    IAMF_OBU_IA_TEMPORAL_DELIMITER  = 4,
    IAMF_OBU_IA_AUDIO_FRAME         = 5,
    IAMF_OBU_IA_AUDIO_FRAME_ID0     = 6,
    IAMF_OBU_IA_AUDIO_FRAME_ID1     = 7,
    IAMF_OBU_IA_AUDIO_FRAME_ID2     = 8,
    IAMF_OBU_IA_AUDIO_FRAME_ID3     = 9,
    IAMF_OBU_IA_AUDIO_FRAME_ID4     = 10,
    IAMF_OBU_IA_AUDIO_FRAME_ID5     = 11,
    IAMF_OBU_IA_AUDIO_FRAME_ID6     = 12,
    IAMF_OBU_IA_AUDIO_FRAME_ID7     = 13,
    IAMF_OBU_IA_AUDIO_FRAME_ID8     = 14,
    IAMF_OBU_IA_AUDIO_FRAME_ID9     = 15,
    IAMF_OBU_IA_AUDIO_FRAME_ID10    = 16,
    IAMF_OBU_IA_AUDIO_FRAME_ID11    = 17,
    IAMF_OBU_IA_AUDIO_FRAME_ID12    = 18,
    IAMF_OBU_IA_AUDIO_FRAME_ID13    = 19,
    IAMF_OBU_IA_AUDIO_FRAME_ID14    = 20,
    IAMF_OBU_IA_AUDIO_FRAME_ID15    = 21,
    IAMF_OBU_IA_AUDIO_FRAME_ID16    = 22,
    IAMF_OBU_IA_AUDIO_FRAME_ID17    = 23,
    // 24~30 reserved.
    IAMF_OBU_IA_SEQUENCE_HEADER     = 31,
};

typedef struct IAMFCodecConfig {
    unsigned codec_config_id;
    enum AVCodecID codec_id;
    uint32_t codec_tag;
    unsigned nb_samples;
    int audio_roll_distance;
    int sample_rate;
    int extradata_size;
    uint8_t *extradata;
} IAMFCodecConfig;

typedef struct IAMFLayer {
    unsigned int substream_count;
    unsigned int coupled_substream_count;
} IAMFLayer;

typedef struct IAMFSubStream {
    unsigned int audio_substream_id;

    // demux
    AVCodecParameters *codecpar;
} IAMFSubStream;

typedef struct IAMFAudioElement {
    const AVIAMFAudioElement *celement;
    /**
     * element backs celement iff the AVIAMFAudioElement
     * is owned by this structure.
     */
    AVIAMFAudioElement *element;
    unsigned int audio_element_id;

    IAMFSubStream *substreams;
    unsigned int nb_substreams;

    unsigned int codec_config_id;

    IAMFLayer *layers;
    unsigned int nb_layers;
} IAMFAudioElement;

typedef struct IAMFMixPresentation {
    const AVIAMFMixPresentation *cmix;
    /**
     * mix backs cmix iff the AVIAMFMixPresentation
     * is owned by this structure.
     */
    AVIAMFMixPresentation *mix;
    unsigned int mix_presentation_id;

    // demux
    unsigned int count_label;
    char **language_label;
} IAMFMixPresentation;

typedef struct IAMFParamDefinition {
    const IAMFAudioElement *audio_element;
    AVIAMFParamDefinition *param;
    int mode;
    size_t param_size;
} IAMFParamDefinition;

typedef struct IAMFContext {
    IAMFCodecConfig **codec_configs;
    int nb_codec_configs;
    IAMFAudioElement **audio_elements;
    int nb_audio_elements;
    IAMFMixPresentation **mix_presentations;
    int nb_mix_presentations;
    IAMFParamDefinition **param_definitions;
    int nb_param_definitions;
} IAMFContext;

enum IAMF_Anchor_Element {
    IAMF_ANCHOR_ELEMENT_UNKNWONW,
    IAMF_ANCHOR_ELEMENT_DIALOGUE,
    IAMF_ANCHOR_ELEMENT_ALBUM,
};

enum IAMF_Sound_System {
    SOUND_SYSTEM_A_0_2_0  = 0,  // "Loudspeaker configuration for Sound System A"
    SOUND_SYSTEM_B_0_5_0  = 1,  // "Loudspeaker configuration for Sound System B"
    SOUND_SYSTEM_C_2_5_0  = 2,  // "Loudspeaker configuration for Sound System C"
    SOUND_SYSTEM_D_4_5_0  = 3,  // "Loudspeaker configuration for Sound System D"
    SOUND_SYSTEM_E_4_5_1  = 4,  // "Loudspeaker configuration for Sound System E"
    SOUND_SYSTEM_F_3_7_0  = 5,  // "Loudspeaker configuration for Sound System F"
    SOUND_SYSTEM_G_4_9_0  = 6,  // "Loudspeaker configuration for Sound System G"
    SOUND_SYSTEM_H_9_10_3 = 7,  // "Loudspeaker configuration for Sound System H"
    SOUND_SYSTEM_I_0_7_0  = 8,  // "Loudspeaker configuration for Sound System I"
    SOUND_SYSTEM_J_4_7_0  = 9, // "Loudspeaker configuration for Sound System J"
    SOUND_SYSTEM_10_2_7_0 = 10, // "Loudspeaker configuration for Sound System I" + Ltf + Rtf
    SOUND_SYSTEM_11_2_3_0 = 11, // Front subset of "Loudspeaker configuration for Sound System J"
    SOUND_SYSTEM_12_0_1_0 = 12, // Mono
    SOUND_SYSTEM_13_9_1_6 = 13, // Subset of "Loudspeaker configuration for Sound System H"
};

struct IAMFSoundSystemMap {
    enum IAMF_Sound_System id;
    AVChannelLayout layout;
};

FF_VISIBILITY_PUSH_HIDDEN
extern const AVChannelLayout ff_iamf_scalable_ch_layouts[10];
extern const AVChannelLayout ff_iamf_expanded_scalable_ch_layouts[13];
extern const struct IAMFSoundSystemMap ff_iamf_sound_system_map[14];

static inline IAMFCodecConfig *ff_iamf_get_codec_config(const IAMFContext *c,
                                                        unsigned int codec_config_id)
{
    IAMFCodecConfig *codec_config = NULL;

    for (int i = 0; i < c->nb_codec_configs; i++) {
        if (c->codec_configs[i]->codec_config_id == codec_config_id)
            codec_config = c->codec_configs[i];
    }

    return codec_config;
}

static inline IAMFParamDefinition *ff_iamf_get_param_definition(const IAMFContext *iamf,
                                                                unsigned int parameter_id)
{
    IAMFParamDefinition *param_definition = NULL;

    for (int i = 0; i < iamf->nb_param_definitions; i++)
        if (iamf->param_definitions[i]->param->parameter_id == parameter_id) {
            param_definition = iamf->param_definitions[i];
            break;
        }

    return param_definition;
}

void ff_iamf_free_audio_element(IAMFAudioElement **paudio_element);
void ff_iamf_free_mix_presentation(IAMFMixPresentation **pmix_presentation);
void ff_iamf_uninit_context(IAMFContext *c);
FF_VISIBILITY_POP_HIDDEN

#endif /* AVFORMAT_IAMF_H */
