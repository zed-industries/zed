/*
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

#ifndef FFTOOLS_FFMPEG_H
#define FFTOOLS_FFMPEG_H

#include "config.h"

#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <signal.h>

#include "cmdutils.h"
#include "ffmpeg_sched.h"
#include "sync_queue.h"

#include "libavformat/avformat.h"
#include "libavformat/avio.h"

#include "libavcodec/avcodec.h"
#include "libavcodec/bsf.h"

#include "libavfilter/avfilter.h"

#include "libavutil/avutil.h"
#include "libavutil/dict.h"
#include "libavutil/eval.h"
#include "libavutil/fifo.h"
#include "libavutil/hwcontext.h"
#include "libavutil/pixfmt.h"
#include "libavutil/rational.h"
#include "libavutil/thread.h"
#include "libavutil/threadmessage.h"

#include "libswresample/swresample.h"

// deprecated features
#define FFMPEG_OPT_QPHIST 1
#define FFMPEG_OPT_ADRIFT_THRESHOLD 1
#define FFMPEG_OPT_ENC_TIME_BASE_NUM 1
#define FFMPEG_OPT_TOP 1
#define FFMPEG_OPT_FORCE_KF_SOURCE_NO_DROP 1
#define FFMPEG_OPT_VSYNC_DROP 1
#define FFMPEG_OPT_VSYNC 1
#define FFMPEG_OPT_FILTER_SCRIPT 1

#define FFMPEG_ERROR_RATE_EXCEEDED FFERRTAG('E', 'R', 'E', 'D')

enum VideoSyncMethod {
    VSYNC_AUTO = -1,
    VSYNC_PASSTHROUGH,
    VSYNC_CFR,
    VSYNC_VFR,
    VSYNC_VSCFR,
#if FFMPEG_OPT_VSYNC_DROP
    VSYNC_DROP,
#endif
};

enum EncTimeBase {
    ENC_TIME_BASE_DEMUX  = -1,
    ENC_TIME_BASE_FILTER = -2,
};

enum HWAccelID {
    HWACCEL_NONE = 0,
    HWACCEL_AUTO,
    HWACCEL_GENERIC,
};

enum FrameOpaque {
    FRAME_OPAQUE_SUB_HEARTBEAT = 1,
    FRAME_OPAQUE_EOF,
    FRAME_OPAQUE_SEND_COMMAND,
};

enum PacketOpaque {
    PKT_OPAQUE_SUB_HEARTBEAT = 1,
    PKT_OPAQUE_FIX_SUB_DURATION,
};

enum LatencyProbe {
    LATENCY_PROBE_DEMUX,
    LATENCY_PROBE_DEC_PRE,
    LATENCY_PROBE_DEC_POST,
    LATENCY_PROBE_FILTER_PRE,
    LATENCY_PROBE_FILTER_POST,
    LATENCY_PROBE_ENC_PRE,
    LATENCY_PROBE_ENC_POST,
    LATENCY_PROBE_NB,
};

typedef struct HWDevice {
    const char *name;
    enum AVHWDeviceType type;
    AVBufferRef *device_ref;
} HWDevice;

enum ViewSpecifierType {
    // no specifier given
    VIEW_SPECIFIER_TYPE_NONE = 0,
    // val is view index
    VIEW_SPECIFIER_TYPE_IDX,
    // val is view ID
    VIEW_SPECIFIER_TYPE_ID,
    // specify view by its position, val is AV_STEREO3D_VIEW_LEFT/RIGHT
    VIEW_SPECIFIER_TYPE_POS,
    // use all views, val is ignored
    VIEW_SPECIFIER_TYPE_ALL,
};

typedef struct ViewSpecifier {
    enum ViewSpecifierType type;
    unsigned               val;
} ViewSpecifier;

/* select an input stream for an output stream */
typedef struct StreamMap {
    int disabled;           /* 1 is this mapping is disabled by a negative map */
    int file_index;
    int stream_index;
    char *linklabel;       /* name of an output link, for mapping lavfi outputs */

    ViewSpecifier vs;
} StreamMap;

typedef struct OptionsContext {
    OptionGroup *g;

    /* input/output options */
    int64_t start_time;
    int64_t start_time_eof;
    int seek_timestamp;
    const char *format;

    SpecifierOptList codec_names;
    SpecifierOptList audio_ch_layouts;
    SpecifierOptList audio_channels;
    SpecifierOptList audio_sample_rate;
    SpecifierOptList frame_rates;
    SpecifierOptList max_frame_rates;
    SpecifierOptList frame_sizes;
    SpecifierOptList frame_pix_fmts;

    /* input options */
    int64_t input_ts_offset;
    int loop;
    int rate_emu;
    float readrate;
    float readrate_catchup;
    double readrate_initial_burst;
    int accurate_seek;
    int thread_queue_size;
    int input_sync_ref;
    int find_stream_info;

    SpecifierOptList ts_scale;
    SpecifierOptList dump_attachment;
    SpecifierOptList hwaccels;
    SpecifierOptList hwaccel_devices;
    SpecifierOptList hwaccel_output_formats;
    SpecifierOptList autorotate;
    SpecifierOptList apply_cropping;

    /* output options */
    StreamMap *stream_maps;
    int     nb_stream_maps;
    const char **attachments;
    int       nb_attachments;

    int chapters_input_file;

    int64_t recording_time;
    int64_t stop_time;
    int64_t limit_filesize;
    float mux_preload;
    float mux_max_delay;
    float shortest_buf_duration;
    int shortest;
    int bitexact;

    int video_disable;
    int audio_disable;
    int subtitle_disable;
    int data_disable;

    // keys are stream indices
    AVDictionary *streamid;

    SpecifierOptList metadata;
    SpecifierOptList max_frames;
    SpecifierOptList bitstream_filters;
    SpecifierOptList codec_tags;
    SpecifierOptList sample_fmts;
    SpecifierOptList qscale;
    SpecifierOptList forced_key_frames;
    SpecifierOptList fps_mode;
    SpecifierOptList force_fps;
    SpecifierOptList frame_aspect_ratios;
    SpecifierOptList display_rotations;
    SpecifierOptList display_hflips;
    SpecifierOptList display_vflips;
    SpecifierOptList rc_overrides;
    SpecifierOptList intra_matrices;
    SpecifierOptList inter_matrices;
    SpecifierOptList chroma_intra_matrices;
#if FFMPEG_OPT_TOP
    SpecifierOptList top_field_first;
#endif
    SpecifierOptList metadata_map;
    SpecifierOptList presets;
    SpecifierOptList copy_initial_nonkeyframes;
    SpecifierOptList copy_prior_start;
    SpecifierOptList filters;
#if FFMPEG_OPT_FILTER_SCRIPT
    SpecifierOptList filter_scripts;
#endif
    SpecifierOptList reinit_filters;
    SpecifierOptList fix_sub_duration;
    SpecifierOptList fix_sub_duration_heartbeat;
    SpecifierOptList canvas_sizes;
    SpecifierOptList pass;
    SpecifierOptList passlogfiles;
    SpecifierOptList max_muxing_queue_size;
    SpecifierOptList muxing_queue_data_threshold;
    SpecifierOptList guess_layout_max;
    SpecifierOptList apad;
    SpecifierOptList discard;
    SpecifierOptList disposition;
    SpecifierOptList program;
    SpecifierOptList stream_groups;
    SpecifierOptList time_bases;
    SpecifierOptList enc_time_bases;
    SpecifierOptList autoscale;
    SpecifierOptList bits_per_raw_sample;
    SpecifierOptList enc_stats_pre;
    SpecifierOptList enc_stats_post;
    SpecifierOptList mux_stats;
    SpecifierOptList enc_stats_pre_fmt;
    SpecifierOptList enc_stats_post_fmt;
    SpecifierOptList mux_stats_fmt;
} OptionsContext;

enum IFilterFlags {
    IFILTER_FLAG_AUTOROTATE     = (1 << 0),
    IFILTER_FLAG_REINIT         = (1 << 1),
    IFILTER_FLAG_CFR            = (1 << 2),
    IFILTER_FLAG_CROP           = (1 << 3),
};

typedef struct InputFilterOptions {
    int64_t             trim_start_us;
    int64_t             trim_end_us;

    uint8_t            *name;

    /* When IFILTER_FLAG_CFR is set, the stream is guaranteed to be CFR with
     * this framerate.
     *
     * Otherwise, this is an estimate that should not be relied upon to be
     * accurate */
    AVRational          framerate;

    unsigned            crop_top;
    unsigned            crop_bottom;
    unsigned            crop_left;
    unsigned            crop_right;

    int                 sub2video_width;
    int                 sub2video_height;

    // a combination of IFILTER_FLAG_*
    unsigned            flags;

    AVFrame            *fallback;
} InputFilterOptions;

enum OFilterFlags {
    OFILTER_FLAG_DISABLE_CONVERT    = (1 << 0),
    // produce 24-bit audio
    OFILTER_FLAG_AUDIO_24BIT        = (1 << 1),
    OFILTER_FLAG_AUTOSCALE          = (1 << 2),
};

typedef struct OutputFilterOptions {
    // Caller-provided name for this output
    char               *name;

    // Codec used for encoding, may be NULL
    const AVCodec      *enc;

    int64_t             trim_start_us;
    int64_t             trim_duration_us;
    int64_t             ts_offset;

    /* Desired output timebase.
     * Numerator can be one of EncTimeBase values, or 0 when no preference.
     */
    AVRational          output_tb;

    AVDictionary       *sws_opts;
    AVDictionary       *swr_opts;

    int64_t             nb_threads;

    // A combination of OFilterFlags.
    unsigned            flags;

    int                 format;
    int                 width;
    int                 height;
    enum AVColorSpace   color_space;
    enum AVColorRange   color_range;

    enum VideoSyncMethod vsync_method;
    AVRational           frame_rate;
    AVRational           max_frame_rate;

    int                 sample_rate;
    AVChannelLayout     ch_layout;

    const int                *formats;
    const int                *sample_rates;
    const AVChannelLayout    *ch_layouts;
    const AVRational         *frame_rates;
    const enum AVColorSpace  *color_spaces;
    const enum AVColorRange  *color_ranges;

    // for simple filtergraphs only, view specifier passed
    // along to the decoder
    const ViewSpecifier *vs;
} OutputFilterOptions;

typedef struct InputFilter {
    struct FilterGraph *graph;
    uint8_t            *name;
} InputFilter;

typedef struct OutputFilter {
    const AVClass       *class;

    struct FilterGraph  *graph;
    uint8_t             *name;

    /* for filters that are not yet bound to an output stream,
     * this stores the output linklabel, if any */
    int                  bound;
    uint8_t             *linklabel;

    char                *apad;

    enum AVMediaType     type;

    atomic_uint_least64_t nb_frames_dup;
    atomic_uint_least64_t nb_frames_drop;
} OutputFilter;

typedef struct FilterGraph {
    const AVClass *class;
    int            index;

    InputFilter   **inputs;
    int          nb_inputs;
    OutputFilter **outputs;
    int         nb_outputs;
} FilterGraph;

enum DecoderFlags {
    DECODER_FLAG_FIX_SUB_DURATION = (1 << 0),
    // input timestamps are unreliable (guessed by demuxer)
    DECODER_FLAG_TS_UNRELIABLE    = (1 << 1),
    // decoder should override timestamps by fixed framerate
    // from DecoderOpts.framerate
    DECODER_FLAG_FRAMERATE_FORCED = (1 << 2),
#if FFMPEG_OPT_TOP
    DECODER_FLAG_TOP_FIELD_FIRST  = (1 << 3),
#endif
    DECODER_FLAG_SEND_END_TS      = (1 << 4),
    // force bitexact decoding
    DECODER_FLAG_BITEXACT         = (1 << 5),
};

typedef struct DecoderOpts {
    int                         flags;

    char                       *name;
    void                       *log_parent;

    const AVCodec              *codec;
    const AVCodecParameters    *par;

    /* hwaccel options */
    enum HWAccelID              hwaccel_id;
    enum AVHWDeviceType         hwaccel_device_type;
    char                       *hwaccel_device;
    enum AVPixelFormat          hwaccel_output_format;

    AVRational                  time_base;

    // Either forced (when DECODER_FLAG_FRAMERATE_FORCED is set) or
    // estimated (otherwise) video framerate.
    AVRational                  framerate;
} DecoderOpts;

typedef struct Decoder {
    const AVClass   *class;

    enum AVMediaType type;

    const uint8_t   *subtitle_header;
    int              subtitle_header_size;

    // number of frames/samples retrieved from the decoder
    uint64_t         frames_decoded;
    uint64_t         samples_decoded;
    uint64_t         decode_errors;
} Decoder;

typedef struct InputStream {
    const AVClass        *class;

    /* parent source */
    struct InputFile     *file;

    int                   index;

    AVStream             *st;
    int                   user_set_discard;

    /**
     * Codec parameters - to be used by the decoding/streamcopy code.
     * st->codecpar should not be accessed, because it may be modified
     * concurrently by the demuxing thread.
     */
    AVCodecParameters    *par;
    Decoder              *decoder;
    const AVCodec        *dec;

    /* framerate forced with -r */
    AVRational            framerate;
#if FFMPEG_OPT_TOP
    int                   top_field_first;
#endif

    int                   fix_sub_duration;

    /* decoded data from this stream goes into all those filters
     * currently video and audio only */
    InputFilter         **filters;
    int                nb_filters;
} InputStream;

typedef struct InputFile {
    const AVClass   *class;

    int              index;

    AVFormatContext *ctx;
    int64_t          input_ts_offset;
    int              input_sync_ref;
    /**
     * Effective format start time based on enabled streams.
     */
    int64_t          start_time_effective;
    int64_t          ts_offset;
    /* user-specified start time in AV_TIME_BASE or AV_NOPTS_VALUE */
    int64_t          start_time;

    /* streams that ffmpeg is aware of;
     * there may be extra streams in ctx that are not mapped to an InputStream
     * if new streams appear dynamically during demuxing */
    InputStream    **streams;
    int           nb_streams;
} InputFile;

enum forced_keyframes_const {
    FKF_N,
    FKF_N_FORCED,
    FKF_PREV_FORCED_N,
    FKF_PREV_FORCED_T,
    FKF_T,
    FKF_NB
};

#define ABORT_ON_FLAG_EMPTY_OUTPUT        (1 <<  0)
#define ABORT_ON_FLAG_EMPTY_OUTPUT_STREAM (1 <<  1)

enum EncStatsType {
    ENC_STATS_LITERAL = 0,
    ENC_STATS_FILE_IDX,
    ENC_STATS_STREAM_IDX,
    ENC_STATS_FRAME_NUM,
    ENC_STATS_FRAME_NUM_IN,
    ENC_STATS_TIMEBASE,
    ENC_STATS_TIMEBASE_IN,
    ENC_STATS_PTS,
    ENC_STATS_PTS_TIME,
    ENC_STATS_PTS_IN,
    ENC_STATS_PTS_TIME_IN,
    ENC_STATS_DTS,
    ENC_STATS_DTS_TIME,
    ENC_STATS_SAMPLE_NUM,
    ENC_STATS_NB_SAMPLES,
    ENC_STATS_PKT_SIZE,
    ENC_STATS_BITRATE,
    ENC_STATS_AVG_BITRATE,
    ENC_STATS_KEYFRAME,
};

typedef struct EncStatsComponent {
    enum EncStatsType type;

    uint8_t *str;
    size_t   str_len;
} EncStatsComponent;

typedef struct EncStats {
    EncStatsComponent  *components;
    int              nb_components;

    AVIOContext        *io;

    pthread_mutex_t     lock;
    int                 lock_initialized;
} EncStats;

enum {
    KF_FORCE_SOURCE         = 1,
#if FFMPEG_OPT_FORCE_KF_SOURCE_NO_DROP
    KF_FORCE_SOURCE_NO_DROP = 2,
#endif
};

typedef struct KeyframeForceCtx {
    int          type;

    int64_t      ref_pts;

    // timestamps of the forced keyframes, in AV_TIME_BASE_Q
    int64_t     *pts;
    int       nb_pts;
    int          index;

    AVExpr      *pexpr;
    double       expr_const_values[FKF_NB];

    int          dropped_keyframe;
} KeyframeForceCtx;

typedef struct Encoder {
    const AVClass          *class;

    AVCodecContext         *enc_ctx;

    // number of frames/samples sent to the encoder
    uint64_t                frames_encoded;
    uint64_t                samples_encoded;
} Encoder;

enum CroppingType {
    CROP_DISABLED = 0,
    CROP_ALL,
    CROP_CODEC,
    CROP_CONTAINER,
};

typedef struct OutputStream {
    const AVClass *class;

    enum AVMediaType type;

    /* parent muxer */
    struct OutputFile *file;

    int index;               /* stream index in the output file */

    /* input stream that is the source for this output stream;
     * may be NULL for streams with no well-defined source, e.g.
     * attachments or outputs from complex filtergraphs */
    InputStream *ist;

    AVStream *st;            /* stream in the output file */

    Encoder *enc;

    /* video only */
#if FFMPEG_OPT_TOP
    int top_field_first;
#endif
    int bitexact;
    int bits_per_raw_sample;

    AVRational frame_aspect_ratio;

    KeyframeForceCtx kf;

    const char *logfile_prefix;
    FILE *logfile;

    // simple filtergraph feeding this stream, if any
    FilterGraph  *fg_simple;
    OutputFilter *filter;

    char *attachment_filename;

    /* stats */
    // number of packets send to the muxer
    atomic_uint_least64_t packets_written;

    /* packet quality factor */
    atomic_int quality;

    EncStats enc_stats_pre;
    EncStats enc_stats_post;

    /*
     * bool on whether this stream should be utilized for splitting
     * subtitles utilizing fix_sub_duration at random access points.
     */
    unsigned int fix_sub_duration_heartbeat;
} OutputStream;

typedef struct OutputFile {
    const AVClass *class;

    int index;

    const char           *url;

    OutputStream **streams;
    int         nb_streams;

    int64_t recording_time;  ///< desired length of the resulting file in microseconds == AV_TIME_BASE units
    int64_t start_time;      ///< start time in microseconds == AV_TIME_BASE units

    int bitexact;
} OutputFile;

// optionally attached as opaque_ref to decoded AVFrames
typedef struct FrameData {
    // demuxer-estimated dts in AV_TIME_BASE_Q,
    // to be used when real dts is missing
    int64_t dts_est;

    // properties that come from the decoder
    struct {
        uint64_t   frame_num;

        int64_t    pts;
        AVRational tb;
    } dec;

    AVRational frame_rate_filter;

    int        bits_per_raw_sample;

    int64_t wallclock[LATENCY_PROBE_NB];

    AVCodecParameters *par_enc;
} FrameData;

extern InputFile   **input_files;
extern int        nb_input_files;

extern OutputFile   **output_files;
extern int         nb_output_files;

// complex filtergraphs
extern FilterGraph **filtergraphs;
extern int        nb_filtergraphs;

// standalone decoders (not tied to demuxed streams)
extern Decoder     **decoders;
extern int        nb_decoders;

extern char *vstats_filename;

extern float dts_delta_threshold;
extern float dts_error_threshold;

extern enum VideoSyncMethod video_sync_method;
extern float frame_drop_threshold;
extern int do_benchmark;
extern int do_benchmark_all;
extern int do_hex_dump;
extern int do_pkt_dump;
extern int copy_ts;
extern int start_at_zero;
extern int copy_tb;
extern int debug_ts;
extern int exit_on_error;
extern int abort_on_flags;
extern int print_stats;
extern int64_t stats_period;
extern int stdin_interaction;
extern AVIOContext *progress_avio;
extern float max_error_rate;

extern char *filter_nbthreads;
extern int filter_complex_nbthreads;
extern int vstats_version;
extern int auto_conversion_filters;

extern const AVIOInterruptCB int_cb;

extern const OptionDef options[];
extern HWDevice *filter_hw_device;

extern atomic_uint nb_output_dumped;

extern int ignore_unknown_streams;
extern int copy_unknown_streams;

extern int recast_media;

extern FILE *vstats_file;

void term_init(void);
void term_exit(void);

void show_usage(void);

int check_avoptions_used(const AVDictionary *opts, const AVDictionary *opts_used,
                         void *logctx, int decode);

int assert_file_overwrite(const char *filename);
int find_codec(void *logctx, const char *name,
               enum AVMediaType type, int encoder, const AVCodec **codec);
int parse_and_set_vsync(const char *arg, int *vsync_var, int file_idx, int st_idx, int is_global);

int filtergraph_is_simple(const FilterGraph *fg);
int fg_create_simple(FilterGraph **pfg,
                     InputStream *ist,
                     char *graph_desc,
                     Scheduler *sch, unsigned sched_idx_enc,
                     const OutputFilterOptions *opts);
int fg_finalise_bindings(void);

/**
 * Get our axiliary frame data attached to the frame, allocating it
 * if needed.
 */
FrameData *frame_data(AVFrame *frame);

const FrameData *frame_data_c(AVFrame *frame);

FrameData       *packet_data  (AVPacket *pkt);
const FrameData *packet_data_c(AVPacket *pkt);

int ofilter_bind_enc(OutputFilter *ofilter,
                     unsigned sched_idx_enc,
                     const OutputFilterOptions *opts);

/**
 * Create a new filtergraph in the global filtergraph list.
 *
 * @param graph_desc Graph description; an av_malloc()ed string, filtergraph
 *                   takes ownership of it.
 */
int fg_create(FilterGraph **pfg, char *graph_desc, Scheduler *sch);

void fg_free(FilterGraph **pfg);

void fg_send_command(FilterGraph *fg, double time, const char *target,
                     const char *command, const char *arg, int all_filters);

int ffmpeg_parse_options(int argc, char **argv, Scheduler *sch);

void enc_stats_write(OutputStream *ost, EncStats *es,
                     const AVFrame *frame, const AVPacket *pkt,
                     uint64_t frame_num);

HWDevice *hw_device_get_by_name(const char *name);
HWDevice *hw_device_get_by_type(enum AVHWDeviceType type);
int hw_device_init_from_string(const char *arg, HWDevice **dev);
int hw_device_init_from_type(enum AVHWDeviceType type,
                             const char *device,
                             HWDevice **dev_out);
void hw_device_free_all(void);

/**
 * Get a hardware device to be used with this filtergraph.
 * The returned reference is owned by the callee, the caller
 * must ref it explicitly for long-term use.
 */
AVBufferRef *hw_device_for_filter(void);

/**
 * Create a standalone decoder.
 */
int dec_create(const OptionsContext *o, const char *arg, Scheduler *sch);

/**
 * @param dec_opts Dictionary filled with decoder options. Its ownership
 *                 is transferred to the decoder.
 * @param param_out If non-NULL, media properties after opening the decoder
 *                  are written here.
 *
 * @retval ">=0" non-negative scheduler index on success
 * @retval "<0"  an error code on failure
 */
int dec_init(Decoder **pdec, Scheduler *sch,
             AVDictionary **dec_opts, const DecoderOpts *o,
             AVFrame *param_out);
void dec_free(Decoder **pdec);

/*
 * Called by filters to connect decoder's output to given filtergraph input.
 *
 * @param opts filtergraph input options, to be filled by this function
 */
int dec_filter_add(Decoder *dec, InputFilter *ifilter, InputFilterOptions *opts,
                   const ViewSpecifier *vs, SchedulerNode *src);

/*
 * For multiview video, request output of the view(s) determined by vs.
 * May be called multiple times.
 *
 * If this function is never called, only the base view is output. If it is
 * called at least once, only the views requested are output.
 *
 * @param src scheduler node from which the frames corresponding vs
 *            will originate
 */
int dec_request_view(Decoder *dec, const ViewSpecifier *vs,
                     SchedulerNode *src);

int enc_alloc(Encoder **penc, const AVCodec *codec,
              Scheduler *sch, unsigned sch_idx, void *log_parent);
void enc_free(Encoder **penc);

int enc_open(void *opaque, const AVFrame *frame);

int enc_loopback(Encoder *enc);

/*
 * Initialize muxing state for the given stream, should be called
 * after the codec/streamcopy setup has been done.
 *
 * Open the muxer once all the streams have been initialized.
 */
int of_stream_init(OutputFile *of, OutputStream *ost,
                   const AVCodecContext *enc_ctx);
int of_write_trailer(OutputFile *of);
int of_open(const OptionsContext *o, const char *filename, Scheduler *sch);
void of_free(OutputFile **pof);

void of_enc_stats_close(void);

int64_t of_filesize(OutputFile *of);

int ifile_open(const OptionsContext *o, const char *filename, Scheduler *sch);
void ifile_close(InputFile **f);

int ist_use(InputStream *ist, int decoding_needed,
            const ViewSpecifier *vs, SchedulerNode *src);
int ist_filter_add(InputStream *ist, InputFilter *ifilter, int is_simple,
                   const ViewSpecifier *vs, InputFilterOptions *opts,
                   SchedulerNode *src);

/**
 * Find an unused input stream of given type.
 */
InputStream *ist_find_unused(enum AVMediaType type);

/* iterate over all input streams in all input files;
 * pass NULL to start iteration */
InputStream *ist_iter(InputStream *prev);

/* iterate over all output streams in all output files;
 * pass NULL to start iteration */
OutputStream *ost_iter(OutputStream *prev);

void update_benchmark(const char *fmt, ...);

const char *opt_match_per_type_str(const SpecifierOptList *sol,
                                   char mediatype);
void opt_match_per_stream_str(void *logctx, const SpecifierOptList *sol,
                              AVFormatContext *fc, AVStream *st, const char **out);
void opt_match_per_stream_int(void *logctx, const SpecifierOptList *sol,
                              AVFormatContext *fc, AVStream *st, int *out);
void opt_match_per_stream_int64(void *logctx, const SpecifierOptList *sol,
                                AVFormatContext *fc, AVStream *st, int64_t *out);
void opt_match_per_stream_dbl(void *logctx, const SpecifierOptList *sol,
                              AVFormatContext *fc, AVStream *st, double *out);

int view_specifier_parse(const char **pspec, ViewSpecifier *vs);

int muxer_thread(void *arg);
int encoder_thread(void *arg);

#endif /* FFTOOLS_FFMPEG_H */
