/*
 * Blackmagic DeckLink common code
 * Copyright (c) 2013-2014 Ramiro Polla, Luca Barbato, Deti Fliegl
 * Copyright (c) 2017 Akamai Technologies, Inc.
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

#ifndef AVDEVICE_DECKLINK_COMMON_H
#define AVDEVICE_DECKLINK_COMMON_H

#include <DeckLinkAPIVersion.h>
#if BLACKMAGIC_DECKLINK_API_VERSION < 0x0b000000
#define IID_IDeckLinkProfileAttributes IID_IDeckLinkAttributes
#define IDeckLinkProfileAttributes IDeckLinkAttributes
#endif

extern "C" {
#include "libavutil/mem.h"
#include "libavcodec/packet_internal.h"
#include "libavfilter/ccfifo.h"
}
#include "libavutil/thread.h"
#include "decklink_common_c.h"
#if CONFIG_LIBKLVANC
#include "libklvanc/vanc.h"
#endif

#ifdef _WIN32
#define DECKLINK_BOOL BOOL
#else
#define DECKLINK_BOOL bool
#endif

#ifdef _WIN32
static char *dup_wchar_to_utf8(wchar_t *w)
{
    char *s = NULL;
    int l = WideCharToMultiByte(CP_UTF8, 0, w, -1, 0, 0, 0, 0);
    s = (char *) av_malloc(l);
    if (s)
        WideCharToMultiByte(CP_UTF8, 0, w, -1, s, l, 0, 0);
    return s;
}
#define DECKLINK_STR    OLECHAR *
#define DECKLINK_STRDUP dup_wchar_to_utf8
#define DECKLINK_FREE(s) SysFreeString(s)
#elif defined(__APPLE__)
static char *dup_cfstring_to_utf8(CFStringRef w)
{
    char s[256];
    CFStringGetCString(w, s, 255, kCFStringEncodingUTF8);
    return av_strdup(s);
}
#define DECKLINK_STR    const __CFString *
#define DECKLINK_STRDUP dup_cfstring_to_utf8
#define DECKLINK_FREE(s) CFRelease(s)
#else
#define DECKLINK_STR    const char *
#define DECKLINK_STRDUP av_strdup
/* free() is needed for a string returned by the DeckLink SDL. */
#define DECKLINK_FREE(s) free((void *) s)
#endif

class decklink_output_callback;
class decklink_input_callback;

typedef struct DecklinkPacketQueue {
    PacketList pkt_list;
    int nb_packets;
    unsigned long long size;
    int abort_request;
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    AVFormatContext *avctx;
    int64_t max_q_size;
} DecklinkPacketQueue;

struct decklink_ctx {
    /* DeckLink SDK interfaces */
    IDeckLink *dl;
    IDeckLinkOutput *dlo;
    IDeckLinkInput *dli;
    IDeckLinkConfiguration *cfg;
    IDeckLinkProfileAttributes *attr;
    decklink_output_callback *output_callback;

    /* DeckLink mode information */
    BMDTimeValue bmd_tb_den;
    BMDTimeValue bmd_tb_num;
    BMDDisplayMode bmd_mode;
    BMDVideoConnection video_input;
    BMDAudioConnection audio_input;
    BMDTimecodeFormat tc_format;
    int bmd_width;
    int bmd_height;
    int bmd_field_dominance;
    int supports_vanc;

    /* Capture buffer queue */
    DecklinkPacketQueue queue;

    CCFifo cc_fifo;      ///< closed captions

    /* Output VANC queue */
    DecklinkPacketQueue vanc_queue;

    /* Streams present */
    int audio;
    int video;

    /* Status */
    int playback_started;
    int64_t first_pts;
    int64_t last_pts;
    unsigned long frameCount;
    unsigned int dropped;
    AVStream *audio_st;
    AVStream *video_st;
    AVStream *klv_st;
    AVStream *teletext_st;
    uint16_t cdp_sequence_num;

    /* Options */
    int list_devices;
    int list_formats;
    int enable_klv;
    int64_t teletext_lines;
    double preroll;
    int duplex_mode;
    BMDLinkConfiguration link;
    DecklinkPtsSource audio_pts_source;
    DecklinkPtsSource video_pts_source;
    int draw_bars;
    BMDPixelFormat raw_format;
    DecklinkSignalLossAction signal_loss_action;

    int frames_preroll;
    int frames_buffer;

    pthread_mutex_t mutex;
    pthread_cond_t cond;
    int frames_buffer_available_spots;
    int autodetect;

#if CONFIG_LIBKLVANC
    struct klvanc_context_s *vanc_ctx;
#endif

    int channels;
    int audio_depth;
    unsigned long tc_seen;    // used with option wait_for_tc
};

typedef enum { DIRECTION_IN, DIRECTION_OUT} decklink_direction_t;

static const BMDPixelFormat decklink_raw_format_map[] = {
    (BMDPixelFormat)0,
    bmdFormat8BitYUV,
    bmdFormat10BitYUV,
    bmdFormat8BitARGB,
    bmdFormat8BitBGRA,
    bmdFormat10BitRGB,
};

static const BMDAudioConnection decklink_audio_connection_map[] = {
    (BMDAudioConnection)0,
    bmdAudioConnectionEmbedded,
    bmdAudioConnectionAESEBU,
    bmdAudioConnectionAnalog,
    bmdAudioConnectionAnalogXLR,
    bmdAudioConnectionAnalogRCA,
    bmdAudioConnectionMicrophone,
};

static const BMDVideoConnection decklink_video_connection_map[] = {
    (BMDVideoConnection)0,
    bmdVideoConnectionSDI,
    bmdVideoConnectionHDMI,
    bmdVideoConnectionOpticalSDI,
    bmdVideoConnectionComponent,
    bmdVideoConnectionComposite,
    bmdVideoConnectionSVideo,
};

static const BMDTimecodeFormat decklink_timecode_format_map[] = {
    (BMDTimecodeFormat)0,
    bmdTimecodeRP188VITC1,
    bmdTimecodeRP188VITC2,
    bmdTimecodeRP188LTC,
    bmdTimecodeRP188Any,
    bmdTimecodeVITC,
    bmdTimecodeVITCField2,
    bmdTimecodeSerial,
#if BLACKMAGIC_DECKLINK_API_VERSION >= 0x0b000000
    bmdTimecodeRP188HighFrameRate,
#else
    (BMDTimecodeFormat)0,
#endif
};

static const BMDLinkConfiguration decklink_link_conf_map[] = {
    (BMDLinkConfiguration)0,
    bmdLinkConfigurationSingleLink,
    bmdLinkConfigurationDualLink,
    bmdLinkConfigurationQuadLink
};

#if BLACKMAGIC_DECKLINK_API_VERSION >= 0x0b000000
static const BMDProfileID decklink_profile_id_map[] = {
    (BMDProfileID)0,
    bmdProfileTwoSubDevicesHalfDuplex,
    bmdProfileOneSubDeviceFullDuplex,
    bmdProfileOneSubDeviceHalfDuplex,
    bmdProfileTwoSubDevicesFullDuplex,
    bmdProfileFourSubDevicesHalfDuplex,
};
#endif

int ff_decklink_set_configs(AVFormatContext *avctx, decklink_direction_t direction);
int ff_decklink_set_format(AVFormatContext *avctx, int width, int height, int tb_num, int tb_den, enum AVFieldOrder field_order, decklink_direction_t direction = DIRECTION_OUT);
int ff_decklink_set_format(AVFormatContext *avctx, decklink_direction_t direction);
int ff_decklink_list_devices(AVFormatContext *avctx, struct AVDeviceInfoList *device_list, int show_inputs, int show_outputs);
void ff_decklink_list_devices_legacy(AVFormatContext *avctx, int show_inputs, int show_outputs);
int ff_decklink_list_formats(AVFormatContext *avctx, decklink_direction_t direction = DIRECTION_OUT);
void ff_decklink_cleanup(AVFormatContext *avctx);
int ff_decklink_init_device(AVFormatContext *avctx, const char* name);

void ff_decklink_packet_queue_init(AVFormatContext *avctx, DecklinkPacketQueue *q, int64_t queue_size);
void ff_decklink_packet_queue_flush(DecklinkPacketQueue *q);
void ff_decklink_packet_queue_end(DecklinkPacketQueue *q);
unsigned long long ff_decklink_packet_queue_size(DecklinkPacketQueue *q);
int ff_decklink_packet_queue_put(DecklinkPacketQueue *q, AVPacket *pkt);
int ff_decklink_packet_queue_get(DecklinkPacketQueue *q, AVPacket *pkt, int block);
int64_t ff_decklink_packet_queue_peekpts(DecklinkPacketQueue *q);

#endif /* AVDEVICE_DECKLINK_COMMON_H */
