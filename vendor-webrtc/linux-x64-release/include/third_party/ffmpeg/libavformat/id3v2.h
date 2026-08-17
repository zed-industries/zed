/*
 * ID3v2 header parser
 * Copyright (c) 2003 Fabrice Bellard
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

#ifndef AVFORMAT_ID3V2_H
#define AVFORMAT_ID3V2_H

#include <stdint.h>
#include "avformat.h"
#include "internal.h"
#include "metadata.h"

#define ID3v2_HEADER_SIZE 10

/**
 * Default magic bytes for ID3v2 header: "ID3"
 */
#define ID3v2_DEFAULT_MAGIC "ID3"

#define ID3v2_FLAG_DATALEN     0x0001
#define ID3v2_FLAG_UNSYNCH     0x0002
#define ID3v2_FLAG_ENCRYPTION  0x0004
#define ID3v2_FLAG_COMPRESSION 0x0008

#define ID3v2_PRIV_METADATA_PREFIX "id3v2_priv."

enum ID3v2Encoding {
    ID3v2_ENCODING_ISO8859  = 0,
    ID3v2_ENCODING_UTF16BOM = 1,
    ID3v2_ENCODING_UTF16BE  = 2,
    ID3v2_ENCODING_UTF8     = 3,
};

typedef struct ID3v2EncContext {
    int      version;       ///< ID3v2 minor version, either 3 or 4
    int64_t size_pos;       ///< offset of the tag total size
    int          len;       ///< size of the tag written so far
} ID3v2EncContext;

typedef struct ID3v2ExtraMetaGEOB {
    uint32_t datasize;
    uint8_t *mime_type;
    uint8_t *file_name;
    uint8_t *description;
    uint8_t *data;
} ID3v2ExtraMetaGEOB;

typedef struct ID3v2ExtraMetaAPIC {
    AVBufferRef *buf;
    const char  *type;
    uint8_t     *description;
    enum AVCodecID id;
} ID3v2ExtraMetaAPIC;

typedef struct ID3v2ExtraMetaPRIV {
    uint8_t *owner;
    uint8_t *data;
    uint32_t datasize;
} ID3v2ExtraMetaPRIV;

typedef struct ID3v2ExtraMetaCHAP {
    uint8_t *element_id;
    uint32_t start, end;
    AVDictionary *meta;
} ID3v2ExtraMetaCHAP;

typedef struct ID3v2ExtraMeta {
    const char *tag;
    struct ID3v2ExtraMeta *next;
    union {
        ID3v2ExtraMetaAPIC apic;
        ID3v2ExtraMetaCHAP chap;
        ID3v2ExtraMetaGEOB geob;
        ID3v2ExtraMetaPRIV priv;
    } data;
} ID3v2ExtraMeta;

/**
 * Detect ID3v2 Header.
 * @param buf   must be ID3v2_HEADER_SIZE byte long
 * @param magic magic bytes to identify the header.
 * If in doubt, use ID3v2_DEFAULT_MAGIC.
 */
int ff_id3v2_match(const uint8_t *buf, const char *magic);

/**
 * Get the length of an ID3v2 tag.
 * @param buf must be ID3v2_HEADER_SIZE bytes long and point to the start of an
 * already detected ID3v2 tag
 */
int ff_id3v2_tag_len(const uint8_t *buf);

/**
 * Read an ID3v2 tag into specified dictionary and retrieve supported extra metadata.
 *
 * @param metadata Parsed metadata is stored here
 * @param[out] extra_meta If not NULL, extra metadata is parsed into a list of
 * ID3v2ExtraMeta structs and *extra_meta points to the head of the list
 */
void ff_id3v2_read_dict(AVIOContext *pb, AVDictionary **metadata, const char *magic, ID3v2ExtraMeta **extra_meta);

/**
 * Read an ID3v2 tag, including supported extra metadata.
 *
 * Data is read from and stored to AVFormatContext.
 *
 * @param[out] extra_meta If not NULL, extra metadata is parsed into a list of
 * ID3v2ExtraMeta structs and *extra_meta points to the head of the list
 * @param[opt] max_search_search restrict ID3 magic number search (bytes from start)
 */
void ff_id3v2_read(AVFormatContext *s, const char *magic, ID3v2ExtraMeta **extra_meta,
                   unsigned int max_search_size);

/**
 * Initialize an ID3v2 tag.
 */
void ff_id3v2_start(ID3v2EncContext *id3, AVIOContext *pb, int id3v2_version,
                    const char *magic);

/**
 * Convert and write all global metadata from s into an ID3v2 tag.
 */
int ff_id3v2_write_metadata(AVFormatContext *s, ID3v2EncContext *id3);

/**
 * Write an attached picture from pkt into an ID3v2 tag.
 */
int ff_id3v2_write_apic(AVFormatContext *s, ID3v2EncContext *id3, AVPacket *pkt);

/**
 * Finalize an opened ID3v2 tag.
 */
void ff_id3v2_finish(ID3v2EncContext *id3, AVIOContext *pb, int padding_bytes);

/**
 * Write an ID3v2 tag containing all global metadata from s.
 * @param id3v2_version Subversion of ID3v2; supported values are 3 and 4
 * @param magic magic bytes to identify the header
 * If in doubt, use ID3v2_DEFAULT_MAGIC.
 */
int ff_id3v2_write_simple(struct AVFormatContext *s, int id3v2_version, const char *magic);

/**
 * Free memory allocated parsing special (non-text) metadata.
 * @param extra_meta Pointer to a pointer to the head of a ID3v2ExtraMeta list, *extra_meta is set to NULL.
 */
void ff_id3v2_free_extra_meta(ID3v2ExtraMeta **extra_meta);

/**
 * Create a stream for each APIC (attached picture) extracted from the
 * ID3v2 header.
 */
int ff_id3v2_parse_apic(AVFormatContext *s, ID3v2ExtraMeta *extra_meta);

/**
 * Create chapters for all CHAP tags found in the ID3v2 header.
 */
int ff_id3v2_parse_chapters(AVFormatContext *s, ID3v2ExtraMeta *extra_meta);

/**
 * Parse PRIV tags into a dictionary. The PRIV owner is the metadata key. The
 * PRIV data is the value, with non-printable characters escaped.
 */
int ff_id3v2_parse_priv_dict(AVDictionary **d, ID3v2ExtraMeta *extra_meta);

/**
 * Add metadata for all PRIV tags in the ID3v2 header. The PRIV owner is the
 * metadata key. The PRIV data is the value, with non-printable characters
 * escaped.
 */
int ff_id3v2_parse_priv(AVFormatContext *s, ID3v2ExtraMeta *extra_meta);

extern const AVMetadataConv ff_id3v2_34_metadata_conv[];
extern const AVMetadataConv ff_id3v2_4_metadata_conv[];

/**
 * A list of text information frames allowed in both ID3 v2.3 and v2.4
 * http://www.id3.org/id3v2.4.0-frames
 * http://www.id3.org/id3v2.4.0-changes
 */
extern const char ff_id3v2_tags[][4];

/**
 * ID3v2.4-only text information frames.
 */
extern const char ff_id3v2_4_tags[][4];

/**
 * ID3v2.3-only text information frames.
 */
extern const char ff_id3v2_3_tags[][4];

extern const CodecMime ff_id3v2_mime_tags[];

extern const char * const ff_id3v2_picture_types[21];

#endif /* AVFORMAT_ID3V2_H */
