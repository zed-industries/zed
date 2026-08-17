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

/*
 *
 * Copyright (c) Sandflow Consulting LLC
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * * Redistributions of source code must retain the above copyright notice, this
 *   list of conditions and the following disclaimer.
 * * Redistributions in binary form must reproduce the above copyright notice,
 *   this list of conditions and the following disclaimer in the documentation
 *   and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

/**
 * Public header file for the processing of Interoperable Master Format (IMF)
 * packages.
 *
 * @author Pierre-Anthony Lemieux
 * @author Valentin Noel
 * @file
 * @ingroup lavu_imf
 */

#ifndef AVFORMAT_IMF_H
#define AVFORMAT_IMF_H

#include "avformat.h"
#include "libavformat/avio.h"
#include "libavutil/rational.h"
#include "libavutil/uuid.h"
#include "libavutil/timecode.h"
#include <libxml/tree.h>

/**
 * IMF Composition Playlist Base Resource
 */
typedef struct FFIMFBaseResource {
    AVRational edit_rate;  /**< BaseResourceType/EditRate */
    uint32_t entry_point;  /**< BaseResourceType/EntryPoint */
    uint32_t duration;     /**< BaseResourceType/Duration */
    uint32_t repeat_count; /**< BaseResourceType/RepeatCount */
} FFIMFBaseResource;

/**
 * IMF Composition Playlist Track File Resource
 */
typedef struct FFIMFTrackFileResource {
    FFIMFBaseResource base;
    AVUUID track_file_uuid; /**< TrackFileResourceType/TrackFileId */
} FFIMFTrackFileResource;

/**
 * IMF Marker
 */
typedef struct FFIMFMarker {
    xmlChar *label_utf8; /**< Marker/Label */
    xmlChar *scope_utf8; /**< Marker/Label/\@scope */
    uint32_t offset;     /**< Marker/Offset */
} FFIMFMarker;

/**
 * IMF Composition Playlist Marker Resource
 */
typedef struct FFIMFMarkerResource {
    FFIMFBaseResource base;
    uint32_t marker_count; /**< Number of Marker elements */
    FFIMFMarker *markers;  /**< Marker elements */
} FFIMFMarkerResource;

/**
 * IMF Composition Playlist Virtual Track
 */
typedef struct FFIMFBaseVirtualTrack {
    AVUUID id_uuid; /**< TrackId associated with the Virtual Track */
} FFIMFBaseVirtualTrack;

/**
 * IMF Composition Playlist Virtual Track that consists of Track File Resources
 */
typedef struct FFIMFTrackFileVirtualTrack {
    FFIMFBaseVirtualTrack base;
    uint32_t resource_count;           /**< Number of Resource elements present in the Virtual Track */
    FFIMFTrackFileResource *resources; /**< Resource elements of the Virtual Track */
    unsigned int resources_alloc_sz;   /**< Size of the resources buffer */
} FFIMFTrackFileVirtualTrack;

/**
 * IMF Composition Playlist Virtual Track that consists of Marker Resources
 */
typedef struct FFIMFMarkerVirtualTrack {
    FFIMFBaseVirtualTrack base;
    uint32_t resource_count;        /**< Number of Resource elements present in the Virtual Track */
    FFIMFMarkerResource *resources; /**< Resource elements of the Virtual Track */
} FFIMFMarkerVirtualTrack;

/**
 * IMF Composition Playlist
 */
typedef struct FFIMFCPL {
    AVUUID id_uuid;                               /**< CompositionPlaylist/Id element */
    xmlChar *content_title_utf8;                     /**< CompositionPlaylist/ContentTitle element */
    AVRational edit_rate;                            /**< CompositionPlaylist/EditRate element */
    AVTimecode *tc;                                  /**< CompositionPlaylist/CompositionTimecode element */
    FFIMFMarkerVirtualTrack *main_markers_track;     /**< Main Marker Virtual Track */
    FFIMFTrackFileVirtualTrack *main_image_2d_track; /**< Main Image Virtual Track */
    uint32_t main_audio_track_count;                 /**< Number of Main Audio Virtual Tracks */
    FFIMFTrackFileVirtualTrack *main_audio_tracks;   /**< Main Audio Virtual Tracks */
} FFIMFCPL;

/**
 * Parse an IMF CompositionPlaylist element into the FFIMFCPL data structure.
 * @param[in] log_ctx Logging context (points to an instance of AVClass). May be NULL.
 * @param[in] doc An XML document from which the CPL is read.
 * @param[out] cpl Pointer to a memory area (allocated by the client), where the
 *  function writes a pointer to the newly constructed FFIMFCPL structure (or
 *  NULL if the CPL could not be parsed). The client is responsible for freeing
 *  the FFIMFCPL structure using ff_imf_cpl_free().
 * @return A non-zero value in case of an error.
 */
int ff_imf_parse_cpl_from_xml_dom(void *log_ctx, xmlDocPtr doc, FFIMFCPL **cpl);

/**
 * Parse an IMF Composition Playlist document into the FFIMFCPL data structure.
 * @param[in] log_ctx Logging context (points to an instance of AVClass). May be NULL.
 * @param[in] in The context from which the CPL is read.
 * @param[out] cpl Pointer to a memory area (allocated by the client), where the
 * function writes a pointer to the newly constructed FFIMFCPL structure (or
 * NULL if the CPL could not be parsed). The client is responsible for freeing
 * the FFIMFCPL structure using ff_imf_cpl_free().
 * @return A non-zero value in case of an error.
 */
int ff_imf_parse_cpl(void *log_ctx, AVIOContext *in, FFIMFCPL **cpl);

/**
 * Allocates and initializes an FFIMFCPL data structure.
 * @return A pointer to the newly constructed FFIMFCPL structure (or NULL if the
 * structure could not be constructed). The client is responsible for freeing
 * the FFIMFCPL structure using ff_imf_cpl_free().
 */
FFIMFCPL *ff_imf_cpl_alloc(void);

/**
 * Deletes an FFIMFCPL data structure previously instantiated with ff_imf_cpl_alloc().
 * @param[in] cpl The FFIMFCPL structure to delete.
 */
void ff_imf_cpl_free(FFIMFCPL *cpl);

/**
 * Reads an unsigned 32-bit integer from an XML element
 * @return 0 on success, < 0 AVERROR code on error.
 */
int ff_imf_xml_read_uint32(xmlNodePtr element, uint32_t *number);

/**
 * Reads an AVRational from an XML element
 * @return 0 on success, < 0 AVERROR code on error.
 */
int ff_imf_xml_read_rational(xmlNodePtr element, AVRational *rational);

/**
 * Reads a UUID from an XML element
 * @return 0 on success, < 0 AVERROR code on error.
 */
int ff_imf_xml_read_uuid(xmlNodePtr element, AVUUID uuid);

/**
 * Returns the first child element with the specified local name
 * @return A pointer to the child element, or NULL if no such child element exists.
 */
xmlNodePtr ff_imf_xml_get_child_element_by_name(xmlNodePtr parent, const char *name_utf8);

#endif
