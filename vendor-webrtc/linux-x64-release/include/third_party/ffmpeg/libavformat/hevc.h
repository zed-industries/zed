/*
 * Copyright (c) 2014 Tim Walker <tdskywalker@gmail.com>
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
 * internal header for HEVC (de)muxer utilities
 */

#ifndef AVFORMAT_HEVC_H
#define AVFORMAT_HEVC_H

#include <stdint.h>
#include "avio.h"

/**
 * Writes Annex B formatted HEVC NAL units to the provided AVIOContext.
 *
 * The NAL units are converted to an MP4-compatible format (start code prefixes
 * are replaced by 4-byte size fields, as per ISO/IEC 14496-15).
 *
 * If filter_ps is non-zero, any HEVC parameter sets found in the input will be
 * discarded, and *ps_count will be set to the number of discarded PS NAL units.
 *
 * @param pb address of the AVIOContext where the data shall be written
 * @param buf_in address of the buffer holding the input data
 * @param size size (in bytes) of the input buffer
 * @param filter_ps whether to write parameter set NAL units to the output (0)
 *        or to discard them (non-zero)
 * @param ps_count address of the variable where the number of discarded
 *        parameter set NAL units shall be written, may be NULL
 * @return the amount (in bytes) of data written in case of success, a negative
 *         value corresponding to an AVERROR code in case of failure
 */
int ff_hevc_annexb2mp4(AVIOContext *pb, const uint8_t *buf_in,
                       int size, int filter_ps, int *ps_count);

/**
 * Writes Annex B formatted HEVC NAL units to a data buffer.
 *
 * The NAL units are converted to an MP4-compatible format (start code prefixes
 * are replaced by 4-byte size fields, as per ISO/IEC 14496-15).
 *
 * If filter_ps is non-zero, any HEVC parameter sets found in the input will be
 * discarded, and *ps_count will be set to the number of discarded PS NAL units.
 *
 * On success, *size holds the size (in bytes) of the output data buffer.
 *
 * @param buf_in address of the buffer holding the input data
 * @param size address of the variable holding the size (in bytes) of the input
 *        buffer (on input) and of the output buffer (on success)
 * @param buf_out on success, address of the variable holding the address of
 *        the output buffer
 * @param filter_ps whether to write parameter set NAL units to the output (0)
 *        or to discard them (non-zero)
 * @param ps_count address of the variable where the number of discarded
 *        parameter set NAL units shall be written, may be NULL
 * @return 0 in case of success, a negative value corresponding to an AVERROR
 *         code in case of failure
 * @note *buf_out will be treated as uninitialized on input and won't be freed.
 */
int ff_hevc_annexb2mp4_buf(const uint8_t *buf_in, uint8_t **buf_out,
                           int *size, int filter_ps, int *ps_count);

/**
 * Writes HEVC extradata (parameter sets and declarative SEI NAL units with
 * nuh_layer_id == 0, as a HEVCDecoderConfigurationRecord) to the
 * provided AVIOContext.
 *
 * If the extradata is Annex B format, it gets converted to hvcC format before
 * writing.
 *
 * @param pb address of the AVIOContext where the hvcC shall be written
 * @param data address of the buffer holding the data needed to write the hvcC
 * @param size size (in bytes) of the data buffer
 * @param ps_array_completeness whether all parameter sets are in the hvcC (1)
 *        or there may be additional parameter sets in the bitstream (0)
 * @param logctx opaque struct starting with an AVClass element, used for logging
 * @return >=0 in case of success, a negative value corresponding to an AVERROR
 *         code in case of failure
 */
int ff_isom_write_hvcc(AVIOContext *pb, const uint8_t *data,
                       int size, int ps_array_completeness, void *logctx);

/**
 * Writes L-HEVC extradata (parameter sets with nuh_layer_id > 0, as a
 * LHEVCDecoderConfigurationRecord) to the provided AVIOContext.
 *
 * If the extradata is Annex B format, it gets converted to lhvC format before
 * writing. Otherwise, hvcC formated extradata is expected, not lhvC.
 *
 * @param pb address of the AVIOContext where the lhvC shall be written
 * @param data address of the buffer holding the data needed to write the lhvC
 * @param size size (in bytes) of the data buffer
 * @param ps_array_completeness whether all parameter sets are in the lhvC (1)
 *        or there may be additional parameter sets in the bitstream (0)
 * @param logctx opaque struct starting with an AVClass element, used for logging
 * @return >=0 in case of success, a negative value corresponding to an AVERROR
 *         code in case of failure
 */
int ff_isom_write_lhvc(AVIOContext *pb, const uint8_t *data,
                       int size, int ps_array_completeness, void *logctx);
#endif /* AVFORMAT_HEVC_H */
