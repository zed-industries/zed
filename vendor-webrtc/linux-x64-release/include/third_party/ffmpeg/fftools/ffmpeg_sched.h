/*
 * Inter-thread scheduling/synchronization.
 * Copyright (c) 2023 Anton Khirnov
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

#ifndef FFTOOLS_FFMPEG_SCHED_H
#define FFTOOLS_FFMPEG_SCHED_H

#include <stddef.h>
#include <stdint.h>

#include "ffmpeg_utils.h"

/*
 * This file contains the API for the transcode scheduler.
 *
 * Overall architecture of the transcoding process involves instances of the
 * following components:
 * - demuxers, each containing any number of demuxed streams; demuxed packets
 *   belonging to some stream are sent to any number of decoders (transcoding)
 *   and/or muxers (streamcopy);
 * - decoders, which receive encoded packets from some demuxed stream or
 *   encoder, decode them, and send decoded frames to any number of filtergraph
 *   inputs (audio/video) or encoders (subtitles);
 * - filtergraphs, each containing zero or more inputs (0 in case the
 *   filtergraph contains a lavfi source filter), and one or more outputs; the
 *   inputs and outputs need not have matching media types;
 *   each filtergraph input receives decoded frames from some decoder or another
 *   filtergraph output;
 *   filtered frames from each output are sent to some encoder;
 * - encoders, which receive decoded frames from some decoder (subtitles) or
 *   some filtergraph output (audio/video), encode them, and send encoded
 *   packets to any number of muxed streams or decoders;
 * - muxers, each containing any number of muxed streams; each muxed stream
 *   receives encoded packets from some demuxed stream (streamcopy) or some
 *   encoder (transcoding); those packets are interleaved and written out by the
 *   muxer.
 *
 * The structure formed by the above components is a directed acyclic graph
 * (absence of cycles is checked at startup).
 *
 * There must be at least one muxer instance, otherwise the transcode produces
 * no output and is meaningless. Otherwise, in a generic transcoding scenario
 * there may be arbitrary number of instances of any of the above components,
 * interconnected in various ways.
 *
 * The code tries to keep all the output streams across all the muxers in sync
 * (i.e. at the same DTS), which is accomplished by varying the rates at which
 * packets are read from different demuxers and lavfi sources. Note that the
 * degree of control we have over synchronization is fundamentally limited - if
 * some demuxed streams in the same input are interleaved at different rates
 * than that at which they are to be muxed (e.g. because an input file is badly
 * interleaved, or the user changed their speed by mismatching amounts), then
 * there will be increasing amounts of buffering followed by eventual
 * transcoding failure.
 *
 * N.B. 1: there are meaningful transcode scenarios with no demuxers, e.g.
 * - encoding and muxing output from filtergraph(s) that have no inputs;
 * - creating a file that contains nothing but attachments and/or metadata.
 *
 * N.B. 2: a filtergraph output could, in principle, feed multiple encoders, but
 * this is unnecessary because the (a)split filter provides the same
 * functionality.
 *
 * The scheduler, in the above model, is the master object that oversees and
 * facilitates the transcoding process. The basic idea is that all instances
 * of the abovementioned components communicate only with the scheduler and not
 * with each other. The scheduler is then the single place containing the
 * knowledge about the whole transcoding pipeline.
 */

struct AVFrame;
struct AVPacket;

typedef struct Scheduler Scheduler;

enum SchedulerNodeType {
    SCH_NODE_TYPE_NONE = 0,
    SCH_NODE_TYPE_DEMUX,
    SCH_NODE_TYPE_MUX,
    SCH_NODE_TYPE_DEC,
    SCH_NODE_TYPE_ENC,
    SCH_NODE_TYPE_FILTER_IN,
    SCH_NODE_TYPE_FILTER_OUT,
};

typedef struct SchedulerNode {
    enum SchedulerNodeType  type;
    unsigned                idx;
    unsigned                idx_stream;
} SchedulerNode;

typedef int (*SchThreadFunc)(void *arg);

#define SCH_DSTREAM(file, stream)                           \
    (SchedulerNode){ .type = SCH_NODE_TYPE_DEMUX,           \
                     .idx = file, .idx_stream = stream }
#define SCH_MSTREAM(file, stream)                           \
    (SchedulerNode){ .type = SCH_NODE_TYPE_MUX,             \
                     .idx = file, .idx_stream = stream }
#define SCH_DEC_IN(decoder)                                 \
    (SchedulerNode){ .type = SCH_NODE_TYPE_DEC,             \
                     .idx = decoder }
#define SCH_DEC_OUT(decoder, out_idx)                       \
    (SchedulerNode){ .type = SCH_NODE_TYPE_DEC,             \
                     .idx = decoder, .idx_stream = out_idx }
#define SCH_ENC(encoder)                                    \
    (SchedulerNode){ .type = SCH_NODE_TYPE_ENC,             \
                    .idx = encoder }
#define SCH_FILTER_IN(filter, input)                        \
    (SchedulerNode){ .type = SCH_NODE_TYPE_FILTER_IN,       \
                     .idx = filter, .idx_stream = input }
#define SCH_FILTER_OUT(filter, output)                      \
    (SchedulerNode){ .type = SCH_NODE_TYPE_FILTER_OUT,      \
                     .idx = filter, .idx_stream = output }

Scheduler *sch_alloc(void);
void sch_free(Scheduler **sch);

int sch_start(Scheduler *sch);
int sch_stop(Scheduler *sch, int64_t *finish_ts);

/**
 * Wait until transcoding terminates or the specified timeout elapses.
 *
 * @param timeout_us Amount of time in microseconds after which this function
 *                   will timeout.
 * @param transcode_ts Current transcode timestamp in AV_TIME_BASE_Q, for
 *                     informational purposes only.
 *
 * @retval 0 waiting timed out, transcoding is not finished
 * @retval 1 transcoding is finished
 */
int sch_wait(Scheduler *sch, uint64_t timeout_us, int64_t *transcode_ts);

/**
 * Add a demuxer to the scheduler.
 *
 * @param func Function executed as the demuxer task.
 * @param ctx Demuxer state; will be passed to func and used for logging.
 *
 * @retval ">=0" Index of the newly-created demuxer.
 * @retval "<0"  Error code.
 */
int sch_add_demux(Scheduler *sch, SchThreadFunc func, void *ctx);
/**
 * Add a demuxed stream for a previously added demuxer.
 *
 * @param demux_idx index previously returned by sch_add_demux()
 *
 * @retval ">=0" Index of the newly-created demuxed stream.
 * @retval "<0"  Error code.
 */
int sch_add_demux_stream(Scheduler *sch, unsigned demux_idx);

/**
 * Add a decoder to the scheduler.
 *
 * @param func Function executed as the decoder task.
 * @param ctx Decoder state; will be passed to func and used for logging.
 * @param send_end_ts The decoder will return an end timestamp after flush packets
 *                    are delivered to it. See documentation for
 *                    sch_dec_receive() for more details.
 *
 * @retval ">=0" Index of the newly-created decoder.
 * @retval "<0"  Error code.
 */
int sch_add_dec(Scheduler *sch, SchThreadFunc func, void *ctx, int send_end_ts);

/**
 * Add another output to decoder (e.g. for multiview video).
 *
 * @retval ">=0" Index of the newly-added decoder output.
 * @retval "<0"  Error code.
 */
int sch_add_dec_output(Scheduler *sch, unsigned dec_idx);

/**
 * Add a filtergraph to the scheduler.
 *
 * @param nb_inputs Number of filtergraph inputs.
 * @param nb_outputs number of filtergraph outputs
 * @param func Function executed as the filtering task.
 * @param ctx Filter state; will be passed to func and used for logging.
 *
 * @retval ">=0" Index of the newly-created filtergraph.
 * @retval "<0"  Error code.
 */
int sch_add_filtergraph(Scheduler *sch, unsigned nb_inputs, unsigned nb_outputs,
                        SchThreadFunc func, void *ctx);

/**
 * Add a muxer to the scheduler.
 *
 * Note that muxer thread startup is more complicated than for other components,
 * because
 * - muxer streams fed by audio/video encoders become initialized dynamically at
 *   runtime, after those encoders receive their first frame and initialize
 *   themselves, followed by calling sch_mux_stream_ready()
 * - the header can be written after all the streams for a muxer are initialized
 * - we may need to write an SDP, which must happen
 *      - AFTER all the headers are written
 *      - BEFORE any packets are written by any muxer
 *      - with all the muxers quiescent
 * To avoid complicated muxer-thread synchronization dances, we postpone
 * starting the muxer threads until after the SDP is written. The sequence of
 * events is then as follows:
 * - After sch_mux_stream_ready() is called for all the streams in a given muxer,
 *   the header for that muxer is written (care is taken that headers for
 *   different muxers are not written concurrently, since they write file
 *   information to stderr). If SDP is not wanted, the muxer thread then starts
 *   and muxing begins.
 * - When SDP _is_ wanted, no muxer threads start until the header for the last
 *   muxer is written. After that, the SDP is written, after which all the muxer
 *   threads are started at once.
 *
 * In order for the above to work, the scheduler needs to be able to invoke
 * just writing the header, which is the reason the init parameter exists.
 *
 * @param func Function executed as the muxing task.
 * @param init Callback that is called to initialize the muxer and write the
 *             header. Called after sch_mux_stream_ready() is called for all the
 *             streams in the muxer.
 * @param ctx Muxer state; will be passed to func/init and used for logging.
 * @param sdp_auto Determines automatic SDP writing - see sch_sdp_filename().
 * @param thread_queue_size number of packets that can be buffered before
 *                          sending to the muxer blocks
 *
 * @retval ">=0" Index of the newly-created muxer.
 * @retval "<0"  Error code.
 */
int sch_add_mux(Scheduler *sch, SchThreadFunc func, int (*init)(void *),
                void *ctx, int sdp_auto, unsigned thread_queue_size);

/**
 * Default size of a packet thread queue.  For muxing this can be overridden by
 * the thread_queue_size option as passed to a call to sch_add_mux().
 */
#define DEFAULT_PACKET_THREAD_QUEUE_SIZE 8

/**
 * Default size of a frame thread queue.
 */
#define DEFAULT_FRAME_THREAD_QUEUE_SIZE 8

/**
 * Add a muxed stream for a previously added muxer.
 *
 * @param mux_idx index previously returned by sch_add_mux()
 *
 * @retval ">=0" Index of the newly-created muxed stream.
 * @retval "<0"  Error code.
 */
int sch_add_mux_stream(Scheduler *sch, unsigned mux_idx);

/**
 * Configure limits on packet buffering performed before the muxer task is
 * started.
 *
 * @param mux_idx index previously returned by sch_add_mux()
 * @param stream_idx_idx index previously returned by sch_add_mux_stream()
 * @param data_threshold Total size of the buffered packets' data after which
 *                       max_packets applies.
 * @param max_packets maximum Maximum number of buffered packets after
 *                            data_threshold is reached.
 */
void sch_mux_stream_buffering(Scheduler *sch, unsigned mux_idx, unsigned stream_idx,
                              size_t data_threshold, int max_packets);

/**
 * Signal to the scheduler that the specified muxed stream is initialized and
 * ready. Muxing is started once all the streams are ready.
 */
int sch_mux_stream_ready(Scheduler *sch, unsigned mux_idx, unsigned stream_idx);

/**
 * Set the file path for the SDP.
 *
 * The SDP is written when either of the following is true:
 * - this function is called at least once
 * - sdp_auto=1 is passed to EVERY call of sch_add_mux()
 */
int sch_sdp_filename(Scheduler *sch, const char *sdp_filename);

/**
 * Add an encoder to the scheduler.
 *
 * @param func Function executed as the encoding task.
 * @param ctx Encoder state; will be passed to func and used for logging.
 * @param open_cb This callback, if specified, will be called when the first
 *                frame is obtained for this encoder. For audio encoders with a
 *                fixed frame size (which use a sync queue in the scheduler to
 *                rechunk frames), it must return that frame size on success.
 *                Otherwise (non-audio, variable frame size) it should return 0.
 *
 * @retval ">=0" Index of the newly-created encoder.
 * @retval "<0"  Error code.
 */
int sch_add_enc(Scheduler *sch, SchThreadFunc func, void *ctx,
                int (*open_cb)(void *func_arg, const struct AVFrame *frame));

/**
 * Add an pre-encoding sync queue to the scheduler.
 *
 * @param buf_size_us Sync queue buffering size, passed to sq_alloc().
 * @param logctx Logging context for the sync queue. passed to sq_alloc().
 *
 * @retval ">=0" Index of the newly-created sync queue.
 * @retval "<0"  Error code.
 */
int sch_add_sq_enc(Scheduler *sch, uint64_t buf_size_us, void *logctx);
int sch_sq_add_enc(Scheduler *sch, unsigned sq_idx, unsigned enc_idx,
                   int limiting, uint64_t max_frames);

int sch_connect(Scheduler *sch, SchedulerNode src, SchedulerNode dst);

enum DemuxSendFlags {
    /**
     * Treat the packet as an EOF for SCH_NODE_TYPE_MUX destinations
     * send normally to other types.
     */
    DEMUX_SEND_STREAMCOPY_EOF = (1 << 0),
};

/**
 * Called by demuxer tasks to communicate with their downstreams. The following
 * may be sent:
 * - a demuxed packet for the stream identified by pkt->stream_index;
 * - demuxer discontinuity/reset (e.g. after a seek) - this is signalled by an
 *   empty packet with stream_index=-1.
 *
 * @param demux_idx demuxer index
 * @param pkt A demuxed packet to send.
 *            When flushing (i.e. pkt->stream_index=-1 on entry to this
 *            function), on successful return pkt->pts/pkt->time_base will be
 *            set to the maximum end timestamp of any decoded audio stream, or
 *            AV_NOPTS_VALUE if no decoded audio streams are present.
 *
 * @retval "non-negative value" success
 * @retval AVERROR_EOF all consumers for the stream are done
 * @retval AVERROR_EXIT all consumers are done, should terminate demuxing
 * @retval "anoter negative error code" other failure
 */
int sch_demux_send(Scheduler *sch, unsigned demux_idx, struct AVPacket *pkt,
                   unsigned flags);

/**
 * Called by decoder tasks to receive a packet for decoding.
 *
 * @param dec_idx decoder index
 * @param pkt Input packet will be written here on success.
 *
 *            An empty packet signals that the decoder should be flushed, but
 *            more packets will follow (e.g. after seeking). When a decoder
 *            created with send_end_ts=1 receives a flush packet, it must write
 *            the end timestamp of the stream after flushing to
 *            pkt->pts/time_base on the next call to this function (if any).
 *
 * @retval "non-negative value" success
 * @retval AVERROR_EOF no more packets will arrive, should terminate decoding
 * @retval "another negative error code" other failure
 */
int sch_dec_receive(Scheduler *sch, unsigned dec_idx, struct AVPacket *pkt);

/**
 * Called by decoder tasks to send a decoded frame downstream.
 *
 * @param dec_idx Decoder index previously returned by sch_add_dec().
 * @param frame Decoded frame; on success it is consumed and cleared by this
 *              function
 *
 * @retval ">=0" success
 * @retval AVERROR_EOF all consumers are done, should terminate decoding
 * @retval "another negative error code" other failure
 */
int sch_dec_send(Scheduler *sch, unsigned dec_idx,
                 unsigned out_idx, struct AVFrame *frame);

/**
 * Called by filtergraph tasks to obtain frames for filtering. Will wait for a
 * frame to become available and return it in frame.
 *
 * Filtergraphs that contain lavfi sources and do not currently require new
 * input frames should call this function as a means of rate control - then
 * in_idx should be set equal to nb_inputs on entry to this function.
 *
 * @param fg_idx Filtergraph index previously returned by sch_add_filtergraph().
 * @param[in,out] in_idx On input contains the index of the input on which a frame
 *                       is most desired. May be set to nb_inputs to signal that
 *                       the filtergraph does not need more input currently.
 *
 *                       On success, will be replaced with the input index of
 *                       the actually returned frame or EOF timestamp.
 *
 * @retval ">=0" Frame data or EOF timestamp was delivered into frame, in_idx
 *               contains the index of the input it belongs to.
 * @retval AVERROR(EAGAIN) No frame was returned, the filtergraph should
 *                         resume filtering. May only be returned when
 *                         in_idx=nb_inputs on entry to this function.
 * @retval AVERROR_EOF No more frames will arrive, should terminate filtering.
 */
int sch_filter_receive(Scheduler *sch, unsigned fg_idx,
                       unsigned *in_idx, struct AVFrame *frame);
/**
 * Called by filter tasks to signal that a filter input will no longer accept input.
 *
 * @param fg_idx Filtergraph index previously returned from sch_add_filtergraph().
 * @param in_idx Index of the input to finish.
 */
void sch_filter_receive_finish(Scheduler *sch, unsigned fg_idx, unsigned in_idx);

/**
 * Called by filtergraph tasks to send a filtered frame or EOF to consumers.
 *
 * @param fg_idx Filtergraph index previously returned by sch_add_filtergraph().
 * @param out_idx Index of the output which produced the frame.
 * @param frame The frame to send to consumers. When NULL, signals that no more
 *              frames will be produced for the specified output. When non-NULL,
 *              the frame is consumed and cleared by this function on success.
 *
 * @retval "non-negative value" success
 * @retval AVERROR_EOF all consumers are done
 * @retval "anoter negative error code" other failure
 */
int sch_filter_send(Scheduler *sch, unsigned fg_idx, unsigned out_idx,
                    struct AVFrame *frame);

int sch_filter_command(Scheduler *sch, unsigned fg_idx, struct AVFrame *frame);

/**
 * Called by encoder tasks to obtain frames for encoding. Will wait for a frame
 * to become available and return it in frame.
 *
 * @param enc_idx Encoder index previously returned by sch_add_enc().
 * @param frame   Newly-received frame will be stored here on success. Must be
 *                clean on entrance to this function.
 *
 * @retval 0 A frame was successfully delivered into frame.
 * @retval AVERROR_EOF No more frames will be delivered, the encoder should
 *                     flush everything and terminate.
 *
 */
int sch_enc_receive(Scheduler *sch, unsigned enc_idx, struct AVFrame *frame);

/**
 * Called by encoder tasks to send encoded packets downstream.
 *
 * @param enc_idx Encoder index previously returned by sch_add_enc().
 * @param pkt     An encoded packet; it will be consumed and cleared by this
 *                function on success.
 *
 * @retval 0     success
 * @retval "<0"  Error code.
 */
int sch_enc_send   (Scheduler *sch, unsigned enc_idx, struct AVPacket *pkt);

/**
 * Called by muxer tasks to obtain packets for muxing. Will wait for a packet
 * for any muxed stream to become available and return it in pkt.
 *
 * @param mux_idx Muxer index previously returned by sch_add_mux().
 * @param pkt     Newly-received packet will be stored here on success. Must be
 *                clean on entrance to this function.
 *
 * @retval 0 A packet was successfully delivered into pkt. Its stream_index
 *           corresponds to a stream index previously returned from
 *           sch_add_mux_stream().
 * @retval AVERROR_EOF When pkt->stream_index is non-negative, this signals that
 *                     no more packets will be delivered for this stream index.
 *                     Otherwise this indicates that no more packets will be
 *                     delivered for any stream and the muxer should therefore
 *                     flush everything and terminate.
 */
int sch_mux_receive(Scheduler *sch, unsigned mux_idx, struct AVPacket *pkt);

/**
 * Called by muxer tasks to signal that a stream will no longer accept input.
 *
 * @param stream_idx Stream index previously returned from sch_add_mux_stream().
 */
void sch_mux_receive_finish(Scheduler *sch, unsigned mux_idx, unsigned stream_idx);

int sch_mux_sub_heartbeat_add(Scheduler *sch, unsigned mux_idx, unsigned stream_idx,
                              unsigned dec_idx);
int sch_mux_sub_heartbeat(Scheduler *sch, unsigned mux_idx, unsigned stream_idx,
                          const AVPacket *pkt);

#endif /* FFTOOLS_FFMPEG_SCHED_H */
