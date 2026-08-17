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

#ifndef AVCODEC_CBS_SEI_H
#define AVCODEC_CBS_SEI_H

#include <stddef.h>
#include <stdint.h>

#include "cbs.h"
#include "sei.h"


typedef struct SEIRawFillerPayload {
    uint32_t payload_size;
} SEIRawFillerPayload;

typedef struct SEIRawUserDataRegistered {
    uint8_t      itu_t_t35_country_code;
    uint8_t      itu_t_t35_country_code_extension_byte;
    uint8_t     *data; ///< RefStruct reference
    size_t       data_length;
} SEIRawUserDataRegistered;

typedef struct SEIRawUserDataUnregistered {
    uint8_t      uuid_iso_iec_11578[16];
    uint8_t     *data; ///< RefStruct reference
    size_t       data_length;
} SEIRawUserDataUnregistered;

typedef struct SEIRawFramePackingArrangement {
    uint32_t fp_arrangement_id;
    uint8_t fp_arrangement_cancel_flag;
    uint8_t fp_arrangement_type;
    uint8_t fp_quincunx_sampling_flag;
    uint8_t fp_content_interpretation_type;
    uint8_t fp_spatial_flipping_flag;
    uint8_t fp_frame0_flipped_flag;
    uint8_t fp_field_views_flag;
    uint8_t fp_current_frame_is_frame0_flag;
    uint8_t fp_frame0_self_contained_flag;
    uint8_t fp_frame1_self_contained_flag;
    uint8_t fp_frame0_grid_position_x;
    uint8_t fp_frame0_grid_position_y;
    uint8_t fp_frame1_grid_position_x;
    uint8_t fp_frame1_grid_position_y;
    uint8_t fp_arrangement_persistence_flag;
    uint8_t fp_upsampled_aspect_ratio_flag;
} SEIRawFramePackingArrangement;

typedef struct SEIRawDecodedPictureHash {
    uint8_t  dph_sei_hash_type;
    uint8_t  dph_sei_single_component_flag;
    uint8_t  dph_sei_picture_md5[3][16];
    uint16_t dph_sei_picture_crc[3];
    uint32_t dph_sei_picture_checksum[3];

    uint8_t  dph_sei_reserved_zero_7bits;
} SEIRawDecodedPictureHash;

typedef struct SEIRawMasteringDisplayColourVolume {
    uint16_t display_primaries_x[3];
    uint16_t display_primaries_y[3];
    uint16_t white_point_x;
    uint16_t white_point_y;
    uint32_t max_display_mastering_luminance;
    uint32_t min_display_mastering_luminance;
} SEIRawMasteringDisplayColourVolume;

typedef struct SEIRawContentLightLevelInfo {
    uint16_t max_content_light_level;
    uint16_t max_pic_average_light_level;
} SEIRawContentLightLevelInfo;

typedef struct SEIRawAlternativeTransferCharacteristics {
    uint8_t preferred_transfer_characteristics;
} SEIRawAlternativeTransferCharacteristics;

typedef struct SEIRawAmbientViewingEnvironment {
    uint32_t ambient_illuminance;
    uint16_t ambient_light_x;
    uint16_t ambient_light_y;
} SEIRawAmbientViewingEnvironment;

typedef struct SEIRawMessage {
    uint32_t     payload_type;
    uint32_t     payload_size;
    void        *payload;
    void        *payload_ref;    ///< RefStruct reference
    uint8_t     *extension_data; ///< RefStruct reference
    size_t       extension_bit_length;
} SEIRawMessage;

typedef struct SEIRawMessageList {
    SEIRawMessage *messages;
    int         nb_messages;
    int         nb_messages_allocated;
} SEIRawMessageList;


typedef struct SEIMessageState {
    // The type of the payload being written.
    uint32_t payload_type;
    // When reading, contains the size of the payload to allow finding the
    // end of variable-length fields (such as user_data_payload_byte[]).
    // (When writing, the size will be derived from the total number of
    // bytes actually written.)
    uint32_t payload_size;
    // When writing, indicates that payload extension data is present so
    // all extended fields must be written.  May be updated by the writer
    // to indicate that extended fields have been written, so the extension
    // end bits must be written too.
    uint8_t  extension_present;
} SEIMessageState;

struct GetBitContext;
struct PutBitContext;

typedef int (*SEIMessageReadFunction)(CodedBitstreamContext *ctx,
                                      struct GetBitContext *rw,
                                      void *current,
                                      SEIMessageState *sei);

typedef int (*SEIMessageWriteFunction)(CodedBitstreamContext *ctx,
                                       struct PutBitContext *rw,
                                       void *current,
                                       SEIMessageState *sei);

typedef struct SEIMessageTypeDescriptor {
    // Payload type for the message.  (-1 in this field ends a list.)
    int     type;
    // Valid in a prefix SEI NAL unit (always for H.264).
    uint8_t prefix;
    // Valid in a suffix SEI NAL unit (never for H.264).
    uint8_t suffix;
    // Size of the decomposed structure.
    size_t  size;
    // Read bitstream into SEI message.
    SEIMessageReadFunction  read;
    // Write bitstream from SEI message.
    SEIMessageWriteFunction write;
} SEIMessageTypeDescriptor;

// End-of-list sentinel element.
#define SEI_MESSAGE_TYPE_END { .type = -1 }


/**
 * Find the type descriptor for the given payload type.
 *
 * Returns NULL if the payload type is not known.
 */
const SEIMessageTypeDescriptor *ff_cbs_sei_find_type(CodedBitstreamContext *ctx,
                                                     int payload_type);

/**
 * Allocate a new payload for the given SEI message.
 */
int ff_cbs_sei_alloc_message_payload(SEIRawMessage *message,
                                     const SEIMessageTypeDescriptor *desc);

/**
 * Allocate a new empty SEI message in a message list.
 *
 * The new message is in place nb_messages - 1.
 */
int ff_cbs_sei_list_add(SEIRawMessageList *list);

/**
 * Free all SEI messages in a message list.
 */
void ff_cbs_sei_free_message_list(SEIRawMessageList *list);

/**
 * Add an SEI message to an access unit.
 *
 * Will add to an existing SEI NAL unit, or create a new one for the
 * message if there is no suitable existing one.
 *
 * If set, payload_ref must be a RefStruct reference backing payload_data.
 * This function creates a new reference to payload_ref in this case.
 * If payload_ref is NULL, the new message will not be reference counted.
 */
int ff_cbs_sei_add_message(CodedBitstreamContext *ctx,
                           CodedBitstreamFragment *au,
                           int prefix,
                           uint32_t     payload_type,
                           void        *payload_data,
                           void        *payload_ref);

/**
 * Iterate over messages with the given payload type in an access unit.
 *
 * Set message to NULL in the first call.  Returns 0 while more messages
 * are available, AVERROR(ENOENT) when all messages have been found.
 */
int ff_cbs_sei_find_message(CodedBitstreamContext *ctx,
                            CodedBitstreamFragment *au,
                            uint32_t payload_type,
                            SEIRawMessage **message);

/**
 * Delete all messages with the given payload type from an access unit.
 */
void ff_cbs_sei_delete_message_type(CodedBitstreamContext *ctx,
                                    CodedBitstreamFragment *au,
                                    uint32_t payload_type);

#endif /* AVCODEC_CBS_SEI_H */
