/* libFLAC - Free Lossless Audio Codec library
 * Copyright (C) 2000-2009  Josh Coalson
 * Copyright (C) 2011-2016  Xiph.Org Foundation
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * - Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *
 * - Redistributions in binary form must reproduce the above copyright
 * notice, this list of conditions and the following disclaimer in the
 * documentation and/or other materials provided with the distribution.
 *
 * - Neither the name of the Xiph.org Foundation nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef FLAC__FORMAT_H
#define FLAC__FORMAT_H

#include "export.h"
#include "ordinals.h"

#ifdef __cplusplus
extern "C" {
#endif

/** \file include/FLAC/format.h
 *
 *  \brief
 *  This module contains structure definitions for the representation
 *  of FLAC format components in memory.  These are the basic
 *  structures used by the rest of the interfaces.
 *
 *  See the detailed documentation in the
 *  \link flac_format format \endlink module.
 */

/** \defgroup flac_format FLAC/format.h: format components
 *  \ingroup flac
 *
 *  \brief
 *  This module contains structure definitions for the representation
 *  of FLAC format components in memory.  These are the basic
 *  structures used by the rest of the interfaces.
 *
 *  First, you should be familiar with the
 *  <A HREF="../format.html">FLAC format</A>.  Many of the values here
 *  follow directly from the specification.  As a user of libFLAC, the
 *  interesting parts really are the structures that describe the frame
 *  header and metadata blocks.
 *
 *  The format structures here are very primitive, designed to store
 *  information in an efficient way.  Reading information from the
 *  structures is easy but creating or modifying them directly is
 *  more complex.  For the most part, as a user of a library, editing
 *  is not necessary; however, for metadata blocks it is, so there are
 *  convenience functions provided in the \link flac_metadata metadata
 *  module \endlink to simplify the manipulation of metadata blocks.
 *
 * \note
 * It's not the best convention, but symbols ending in _LEN are in bits
 * and _LENGTH are in bytes.  _LENGTH symbols are \#defines instead of
 * global variables because they are usually used when declaring byte
 * arrays and some compilers require compile-time knowledge of array
 * sizes when declared on the stack.
 *
 * \{
 */


/*
	Most of the values described in this file are defined by the FLAC
	format specification.  There is nothing to tune here.
*/

/** The largest legal metadata type code. */
#define FLAC__MAX_METADATA_TYPE_CODE (126u)

/** The minimum block size, in samples, permitted by the format. */
#define FLAC__MIN_BLOCK_SIZE (16u)

/** The maximum block size, in samples, permitted by the format. */
#define FLAC__MAX_BLOCK_SIZE (65535u)

/** The maximum block size, in samples, permitted by the FLAC subset for
 *  sample rates up to 48kHz. */
#define FLAC__SUBSET_MAX_BLOCK_SIZE_48000HZ (4608u)

/** The maximum number of channels permitted by the format. */
#define FLAC__MAX_CHANNELS (8u)

/** The minimum sample resolution permitted by the format. */
#define FLAC__MIN_BITS_PER_SAMPLE (4u)

/** The maximum sample resolution permitted by the format. */
#define FLAC__MAX_BITS_PER_SAMPLE (32u)

/** The maximum sample resolution permitted by libFLAC.
 *
 * \warning
 * FLAC__MAX_BITS_PER_SAMPLE is the limit of the FLAC format.  However,
 * the reference encoder/decoder is currently limited to 24 bits because
 * of prevalent 32-bit math, so make sure and use this value when
 * appropriate.
 */
#define FLAC__REFERENCE_CODEC_MAX_BITS_PER_SAMPLE (24u)

/** The maximum sample rate permitted by the format.  The value is
 *  ((2 ^ 16) - 1) * 10; see <A HREF="../format.html">FLAC format</A>
 *  as to why.
 */
#define FLAC__MAX_SAMPLE_RATE (655350u)

/** The maximum LPC order permitted by the format. */
#define FLAC__MAX_LPC_ORDER (32u)

/** The maximum LPC order permitted by the FLAC subset for sample rates
 *  up to 48kHz. */
#define FLAC__SUBSET_MAX_LPC_ORDER_48000HZ (12u)

/** The minimum quantized linear predictor coefficient precision
 *  permitted by the format.
 */
#define FLAC__MIN_QLP_COEFF_PRECISION (5u)

/** The maximum quantized linear predictor coefficient precision
 *  permitted by the format.
 */
#define FLAC__MAX_QLP_COEFF_PRECISION (15u)

/** The maximum order of the fixed predictors permitted by the format. */
#define FLAC__MAX_FIXED_ORDER (4u)

/** The maximum Rice partition order permitted by the format. */
#define FLAC__MAX_RICE_PARTITION_ORDER (15u)

/** The maximum Rice partition order permitted by the FLAC Subset. */
#define FLAC__SUBSET_MAX_RICE_PARTITION_ORDER (8u)

/** The version string of the release, stamped onto the libraries and binaries.
 *
 * \note
 * This does not correspond to the shared library version number, which
 * is used to determine binary compatibility.
 */
extern FLAC_API const char *FLAC__VERSION_STRING;

/** The vendor string inserted by the encoder into the VORBIS_COMMENT block.
 *  This is a NUL-terminated ASCII string; when inserted into the
 *  VORBIS_COMMENT the trailing null is stripped.
 */
extern FLAC_API const char *FLAC__VENDOR_STRING;

/** The byte string representation of the beginning of a FLAC stream. */
extern FLAC_API const FLAC__byte FLAC__STREAM_SYNC_STRING[4]; /* = "fLaC" */

/** The 32-bit integer big-endian representation of the beginning of
 *  a FLAC stream.
 */
extern FLAC_API const uint32_t FLAC__STREAM_SYNC; /* = 0x664C6143 */

/** The length of the FLAC signature in bits. */
extern FLAC_API const uint32_t FLAC__STREAM_SYNC_LEN; /* = 32 bits */

/** The length of the FLAC signature in bytes. */
#define FLAC__STREAM_SYNC_LENGTH (4u)


/*****************************************************************************
 *
 * Subframe structures
 *
 *****************************************************************************/

/*****************************************************************************/

/** An enumeration of the available entropy coding methods. */
typedef enum {
	FLAC__ENTROPY_CODING_METHOD_PARTITIONED_RICE = 0,
	/**< Residual is coded by partitioning into contexts, each with it's own
	 * 4-bit Rice parameter. */

	FLAC__ENTROPY_CODING_METHOD_PARTITIONED_RICE2 = 1
	/**< Residual is coded by partitioning into contexts, each with it's own
	 * 5-bit Rice parameter. */
} FLAC__EntropyCodingMethodType;

/** Maps a FLAC__EntropyCodingMethodType to a C string.
 *
 *  Using a FLAC__EntropyCodingMethodType as the index to this array will
 *  give the string equivalent.  The contents should not be modified.
 */
extern FLAC_API const char * const FLAC__EntropyCodingMethodTypeString[];


/** Contents of a Rice partitioned residual
 */
typedef struct {

	uint32_t *parameters;
	/**< The Rice parameters for each context. */

	uint32_t *raw_bits;
	/**< Widths for escape-coded partitions.  Will be non-zero for escaped
	 * partitions and zero for unescaped partitions.
	 */

	uint32_t capacity_by_order;
	/**< The capacity of the \a parameters and \a raw_bits arrays
	 * specified as an order, i.e. the number of array elements
	 * allocated is 2 ^ \a capacity_by_order.
	 */
} FLAC__EntropyCodingMethod_PartitionedRiceContents;

/** Header for a Rice partitioned residual.  (c.f. <A HREF="../format.html#partitioned_rice">format specification</A>)
 */
typedef struct {

	uint32_t order;
	/**< The partition order, i.e. # of contexts = 2 ^ \a order. */

	const FLAC__EntropyCodingMethod_PartitionedRiceContents *contents;
	/**< The context's Rice parameters and/or raw bits. */

} FLAC__EntropyCodingMethod_PartitionedRice;

extern FLAC_API const uint32_t FLAC__ENTROPY_CODING_METHOD_PARTITIONED_RICE_ORDER_LEN; /**< == 4 (bits) */
extern FLAC_API const uint32_t FLAC__ENTROPY_CODING_METHOD_PARTITIONED_RICE_PARAMETER_LEN; /**< == 4 (bits) */
extern FLAC_API const uint32_t FLAC__ENTROPY_CODING_METHOD_PARTITIONED_RICE2_PARAMETER_LEN; /**< == 5 (bits) */
extern FLAC_API const uint32_t FLAC__ENTROPY_CODING_METHOD_PARTITIONED_RICE_RAW_LEN; /**< == 5 (bits) */

extern FLAC_API const uint32_t FLAC__ENTROPY_CODING_METHOD_PARTITIONED_RICE_ESCAPE_PARAMETER;
/**< == (1<<FLAC__ENTROPY_CODING_METHOD_PARTITIONED_RICE_PARAMETER_LEN)-1 */
extern FLAC_API const uint32_t FLAC__ENTROPY_CODING_METHOD_PARTITIONED_RICE2_ESCAPE_PARAMETER;
/**< == (1<<FLAC__ENTROPY_CODING_METHOD_PARTITIONED_RICE2_PARAMETER_LEN)-1 */

/** Header for the entropy coding method.  (c.f. <A HREF="../format.html#residual">format specification</A>)
 */
typedef struct {
	FLAC__EntropyCodingMethodType type;
	union {
		FLAC__EntropyCodingMethod_PartitionedRice partitioned_rice;
	} data;
} FLAC__EntropyCodingMethod;

extern FLAC_API const uint32_t FLAC__ENTROPY_CODING_METHOD_TYPE_LEN; /**< == 2 (bits) */

/*****************************************************************************/

/** An enumeration of the available subframe types. */
typedef enum {
	FLAC__SUBFRAME_TYPE_CONSTANT = 0, /**< constant signal */
	FLAC__SUBFRAME_TYPE_VERBATIM = 1, /**< uncompressed signal */
	FLAC__SUBFRAME_TYPE_FIXED = 2, /**< fixed polynomial prediction */
	FLAC__SUBFRAME_TYPE_LPC = 3 /**< linear prediction */
} FLAC__SubframeType;

/** Maps a FLAC__SubframeType to a C string.
 *
 *  Using a FLAC__SubframeType as the index to this array will
 *  give the string equivalent.  The contents should not be modified.
 */
extern FLAC_API const char * const FLAC__SubframeTypeString[];


/** CONSTANT subframe.  (c.f. <A HREF="../format.html#subframe_constant">format specification</A>)
 */
typedef struct {
	FLAC__int32 value; /**< The constant signal value. */
} FLAC__Subframe_Constant;


/** VERBATIM subframe.  (c.f. <A HREF="../format.html#subframe_verbatim">format specification</A>)
 */
typedef struct {
	const FLAC__int32 *data; /**< A pointer to verbatim signal. */
} FLAC__Subframe_Verbatim;


/** FIXED subframe.  (c.f. <A HREF="../format.html#subframe_fixed">format specification</A>)
 */
typedef struct {
	FLAC__EntropyCodingMethod entropy_coding_method;
	/**< The residual coding method. */

	uint32_t order;
	/**< The polynomial order. */

	FLAC__int32 warmup[FLAC__MAX_FIXED_ORDER];
	/**< Warmup samples to prime the predictor, length == order. */

	const FLAC__int32 *residual;
	/**< The residual signal, length == (blocksize minus order) samples. */
} FLAC__Subframe_Fixed;


/** LPC subframe.  (c.f. <A HREF="../format.html#subframe_lpc">format specification</A>)
 */
typedef struct {
	FLAC__EntropyCodingMethod entropy_coding_method;
	/**< The residual coding method. */

	uint32_t order;
	/**< The FIR order. */

	uint32_t qlp_coeff_precision;
	/**< Quantized FIR filter coefficient precision in bits. */

	int quantization_level;
	/**< The qlp coeff shift needed. */

	FLAC__int32 qlp_coeff[FLAC__MAX_LPC_ORDER];
	/**< FIR filter coefficients. */

	FLAC__int32 warmup[FLAC__MAX_LPC_ORDER];
	/**< Warmup samples to prime the predictor, length == order. */

	const FLAC__int32 *residual;
	/**< The residual signal, length == (blocksize minus order) samples. */
} FLAC__Subframe_LPC;

extern FLAC_API const uint32_t FLAC__SUBFRAME_LPC_QLP_COEFF_PRECISION_LEN; /**< == 4 (bits) */
extern FLAC_API const uint32_t FLAC__SUBFRAME_LPC_QLP_SHIFT_LEN; /**< == 5 (bits) */


/** FLAC subframe structure.  (c.f. <A HREF="../format.html#subframe">format specification</A>)
 */
typedef struct {
	FLAC__SubframeType type;
	union {
		FLAC__Subframe_Constant constant;
		FLAC__Subframe_Fixed fixed;
		FLAC__Subframe_LPC lpc;
		FLAC__Subframe_Verbatim verbatim;
	} data;
	uint32_t wasted_bits;
} FLAC__Subframe;

/** == 1 (bit)
 *
 * This used to be a zero-padding bit (hence the name
 * FLAC__SUBFRAME_ZERO_PAD_LEN) but is now a reserved bit.  It still has a
 * mandatory value of \c 0 but in the future may take on the value \c 0 or \c 1
 * to mean something else.
 */
extern FLAC_API const uint32_t FLAC__SUBFRAME_ZERO_PAD_LEN;
extern FLAC_API const uint32_t FLAC__SUBFRAME_TYPE_LEN; /**< == 6 (bits) */
extern FLAC_API const uint32_t FLAC__SUBFRAME_WASTED_BITS_FLAG_LEN; /**< == 1 (bit) */

extern FLAC_API const uint32_t FLAC__SUBFRAME_TYPE_CONSTANT_BYTE_ALIGNED_MASK; /**< = 0x00 */
extern FLAC_API const uint32_t FLAC__SUBFRAME_TYPE_VERBATIM_BYTE_ALIGNED_MASK; /**< = 0x02 */
extern FLAC_API const uint32_t FLAC__SUBFRAME_TYPE_FIXED_BYTE_ALIGNED_MASK; /**< = 0x10 */
extern FLAC_API const uint32_t FLAC__SUBFRAME_TYPE_LPC_BYTE_ALIGNED_MASK; /**< = 0x40 */

/*****************************************************************************/


/*****************************************************************************
 *
 * Frame structures
 *
 *****************************************************************************/

/** An enumeration of the available channel assignments. */
typedef enum {
	FLAC__CHANNEL_ASSIGNMENT_INDEPENDENT = 0, /**< independent channels */
	FLAC__CHANNEL_ASSIGNMENT_LEFT_SIDE = 1, /**< left+side stereo */
	FLAC__CHANNEL_ASSIGNMENT_RIGHT_SIDE = 2, /**< right+side stereo */
	FLAC__CHANNEL_ASSIGNMENT_MID_SIDE = 3 /**< mid+side stereo */
} FLAC__ChannelAssignment;

/** Maps a FLAC__ChannelAssignment to a C string.
 *
 *  Using a FLAC__ChannelAssignment as the index to this array will
 *  give the string equivalent.  The contents should not be modified.
 */
extern FLAC_API const char * const FLAC__ChannelAssignmentString[];

/** An enumeration of the possible frame numbering methods. */
typedef enum {
	FLAC__FRAME_NUMBER_TYPE_FRAME_NUMBER, /**< number contains the frame number */
	FLAC__FRAME_NUMBER_TYPE_SAMPLE_NUMBER /**< number contains the sample number of first sample in frame */
} FLAC__FrameNumberType;

/** Maps a FLAC__FrameNumberType to a C string.
 *
 *  Using a FLAC__FrameNumberType as the index to this array will
 *  give the string equivalent.  The contents should not be modified.
 */
extern FLAC_API const char * const FLAC__FrameNumberTypeString[];


/** FLAC frame header structure.  (c.f. <A HREF="../format.html#frame_header">format specification</A>)
 */
typedef struct {
	uint32_t blocksize;
	/**< The number of samples per subframe. */

	uint32_t sample_rate;
	/**< The sample rate in Hz. */

	uint32_t channels;
	/**< The number of channels (== number of subframes). */

	FLAC__ChannelAssignment channel_assignment;
	/**< The channel assignment for the frame. */

	uint32_t bits_per_sample;
	/**< The sample resolution. */

	FLAC__FrameNumberType number_type;
	/**< The numbering scheme used for the frame.  As a convenience, the
	 * decoder will always convert a frame number to a sample number because
	 * the rules are complex. */

	union {
		FLAC__uint32 frame_number;
		FLAC__uint64 sample_number;
	} number;
	/**< The frame number or sample number of first sample in frame;
	 * use the \a number_type value to determine which to use. */

	FLAC__uint8 crc;
	/**< CRC-8 (polynomial = x^8 + x^2 + x^1 + x^0, initialized with 0)
	 * of the raw frame header bytes, meaning everything before the CRC byte
	 * including the sync code.
	 */
} FLAC__FrameHeader;

extern FLAC_API const uint32_t FLAC__FRAME_HEADER_SYNC; /**< == 0x3ffe; the frame header sync code */
extern FLAC_API const uint32_t FLAC__FRAME_HEADER_SYNC_LEN; /**< == 14 (bits) */
extern FLAC_API const uint32_t FLAC__FRAME_HEADER_RESERVED_LEN; /**< == 1 (bits) */
extern FLAC_API const uint32_t FLAC__FRAME_HEADER_BLOCKING_STRATEGY_LEN; /**< == 1 (bits) */
extern FLAC_API const uint32_t FLAC__FRAME_HEADER_BLOCK_SIZE_LEN; /**< == 4 (bits) */
extern FLAC_API const uint32_t FLAC__FRAME_HEADER_SAMPLE_RATE_LEN; /**< == 4 (bits) */
extern FLAC_API const uint32_t FLAC__FRAME_HEADER_CHANNEL_ASSIGNMENT_LEN; /**< == 4 (bits) */
extern FLAC_API const uint32_t FLAC__FRAME_HEADER_BITS_PER_SAMPLE_LEN; /**< == 3 (bits) */
extern FLAC_API const uint32_t FLAC__FRAME_HEADER_ZERO_PAD_LEN; /**< == 1 (bit) */
extern FLAC_API const uint32_t FLAC__FRAME_HEADER_CRC_LEN; /**< == 8 (bits) */


/** FLAC frame footer structure.  (c.f. <A HREF="../format.html#frame_footer">format specification</A>)
 */
typedef struct {
	FLAC__uint16 crc;
	/**< CRC-16 (polynomial = x^16 + x^15 + x^2 + x^0, initialized with
	 * 0) of the bytes before the crc, back to and including the frame header
	 * sync code.
	 */
} FLAC__FrameFooter;

extern FLAC_API const uint32_t FLAC__FRAME_FOOTER_CRC_LEN; /**< == 16 (bits) */


/** FLAC frame structure.  (c.f. <A HREF="../format.html#frame">format specification</A>)
 */
typedef struct {
	FLAC__FrameHeader header;
	FLAC__Subframe subframes[FLAC__MAX_CHANNELS];
	FLAC__FrameFooter footer;
} FLAC__Frame;

/*****************************************************************************/


/*****************************************************************************
 *
 * Meta-data structures
 *
 *****************************************************************************/

/** An enumeration of the available metadata block types. */
typedef enum {

	FLAC__METADATA_TYPE_STREAMINFO = 0,
	/**< <A HREF="../format.html#metadata_block_streaminfo">STREAMINFO</A> block */

	FLAC__METADATA_TYPE_PADDING = 1,
	/**< <A HREF="../format.html#metadata_block_padding">PADDING</A> block */

	FLAC__METADATA_TYPE_APPLICATION = 2,
	/**< <A HREF="../format.html#metadata_block_application">APPLICATION</A> block */

	FLAC__METADATA_TYPE_SEEKTABLE = 3,
	/**< <A HREF="../format.html#metadata_block_seektable">SEEKTABLE</A> block */

	FLAC__METADATA_TYPE_VORBIS_COMMENT = 4,
	/**< <A HREF="../format.html#metadata_block_vorbis_comment">VORBISCOMMENT</A> block (a.k.a. FLAC tags) */

	FLAC__METADATA_TYPE_CUESHEET = 5,
	/**< <A HREF="../format.html#metadata_block_cuesheet">CUESHEET</A> block */

	FLAC__METADATA_TYPE_PICTURE = 6,
	/**< <A HREF="../format.html#metadata_block_picture">PICTURE</A> block */

	FLAC__METADATA_TYPE_UNDEFINED = 7,
	/**< marker to denote beginning of undefined type range; this number will increase as new metadata types are added */

	FLAC__MAX_METADATA_TYPE = FLAC__MAX_METADATA_TYPE_CODE,
	/**< No type will ever be greater than this. There is not enough room in the protocol block. */
} FLAC__MetadataType;

/** Maps a FLAC__MetadataType to a C string.
 *
 *  Using a FLAC__MetadataType as the index to this array will
 *  give the string equivalent.  The contents should not be modified.
 */
extern FLAC_API const char * const FLAC__MetadataTypeString[];


/** FLAC STREAMINFO structure.  (c.f. <A HREF="../format.html#metadata_block_streaminfo">format specification</A>)
 */
typedef struct {
	uint32_t min_blocksize, max_blocksize;
	uint32_t min_framesize, max_framesize;
	uint32_t sample_rate;
	uint32_t channels;
	uint32_t bits_per_sample;
	FLAC__uint64 total_samples;
	FLAC__byte md5sum[16];
} FLAC__StreamMetadata_StreamInfo;

extern FLAC_API const uint32_t FLAC__STREAM_METADATA_STREAMINFO_MIN_BLOCK_SIZE_LEN; /**< == 16 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_STREAMINFO_MAX_BLOCK_SIZE_LEN; /**< == 16 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_STREAMINFO_MIN_FRAME_SIZE_LEN; /**< == 24 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_STREAMINFO_MAX_FRAME_SIZE_LEN; /**< == 24 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_STREAMINFO_SAMPLE_RATE_LEN; /**< == 20 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_STREAMINFO_CHANNELS_LEN; /**< == 3 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_STREAMINFO_BITS_PER_SAMPLE_LEN; /**< == 5 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_STREAMINFO_TOTAL_SAMPLES_LEN; /**< == 36 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_STREAMINFO_MD5SUM_LEN; /**< == 128 (bits) */

/** The total stream length of the STREAMINFO block in bytes. */
#define FLAC__STREAM_METADATA_STREAMINFO_LENGTH (34u)

/** FLAC PADDING structure.  (c.f. <A HREF="../format.html#metadata_block_padding">format specification</A>)
 */
typedef struct {
	int dummy;
	/**< Conceptually this is an empty struct since we don't store the
	 * padding bytes.  Empty structs are not allowed by some C compilers,
	 * hence the dummy.
	 */
} FLAC__StreamMetadata_Padding;


/** FLAC APPLICATION structure.  (c.f. <A HREF="../format.html#metadata_block_application">format specification</A>)
 */
typedef struct {
	FLAC__byte id[4];
	FLAC__byte *data;
} FLAC__StreamMetadata_Application;

extern FLAC_API const uint32_t FLAC__STREAM_METADATA_APPLICATION_ID_LEN; /**< == 32 (bits) */

/** SeekPoint structure used in SEEKTABLE blocks.  (c.f. <A HREF="../format.html#seekpoint">format specification</A>)
 */
typedef struct {
	FLAC__uint64 sample_number;
	/**<  The sample number of the target frame. */

	FLAC__uint64 stream_offset;
	/**< The offset, in bytes, of the target frame with respect to
	 * beginning of the first frame. */

	uint32_t frame_samples;
	/**< The number of samples in the target frame. */
} FLAC__StreamMetadata_SeekPoint;

extern FLAC_API const uint32_t FLAC__STREAM_METADATA_SEEKPOINT_SAMPLE_NUMBER_LEN; /**< == 64 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_SEEKPOINT_STREAM_OFFSET_LEN; /**< == 64 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_SEEKPOINT_FRAME_SAMPLES_LEN; /**< == 16 (bits) */

/** The total stream length of a seek point in bytes. */
#define FLAC__STREAM_METADATA_SEEKPOINT_LENGTH (18u)

/** The value used in the \a sample_number field of
 *  FLAC__StreamMetadataSeekPoint used to indicate a placeholder
 *  point (== 0xffffffffffffffff).
 */
extern FLAC_API const FLAC__uint64 FLAC__STREAM_METADATA_SEEKPOINT_PLACEHOLDER;


/** FLAC SEEKTABLE structure.  (c.f. <A HREF="../format.html#metadata_block_seektable">format specification</A>)
 *
 * \note From the format specification:
 * - The seek points must be sorted by ascending sample number.
 * - Each seek point's sample number must be the first sample of the
 *   target frame.
 * - Each seek point's sample number must be unique within the table.
 * - Existence of a SEEKTABLE block implies a correct setting of
 *   total_samples in the stream_info block.
 * - Behavior is undefined when more than one SEEKTABLE block is
 *   present in a stream.
 */
typedef struct {
	uint32_t num_points;
	FLAC__StreamMetadata_SeekPoint *points;
} FLAC__StreamMetadata_SeekTable;


/** Vorbis comment entry structure used in VORBIS_COMMENT blocks.  (c.f. <A HREF="../format.html#metadata_block_vorbis_comment">format specification</A>)
 *
 *  For convenience, the APIs maintain a trailing NUL character at the end of
 *  \a entry which is not counted toward \a length, i.e.
 *  \code strlen(entry) == length \endcode
 */
typedef struct {
	FLAC__uint32 length;
	FLAC__byte *entry;
} FLAC__StreamMetadata_VorbisComment_Entry;

extern FLAC_API const uint32_t FLAC__STREAM_METADATA_VORBIS_COMMENT_ENTRY_LENGTH_LEN; /**< == 32 (bits) */


/** FLAC VORBIS_COMMENT structure.  (c.f. <A HREF="../format.html#metadata_block_vorbis_comment">format specification</A>)
 */
typedef struct {
	FLAC__StreamMetadata_VorbisComment_Entry vendor_string;
	FLAC__uint32 num_comments;
	FLAC__StreamMetadata_VorbisComment_Entry *comments;
} FLAC__StreamMetadata_VorbisComment;

extern FLAC_API const uint32_t FLAC__STREAM_METADATA_VORBIS_COMMENT_NUM_COMMENTS_LEN; /**< == 32 (bits) */


/** FLAC CUESHEET track index structure.  (See the
 * <A HREF="../format.html#cuesheet_track_index">format specification</A> for
 * the full description of each field.)
 */
typedef struct {
	FLAC__uint64 offset;
	/**< Offset in samples, relative to the track offset, of the index
	 * point.
	 */

	FLAC__byte number;
	/**< The index point number. */
} FLAC__StreamMetadata_CueSheet_Index;

extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_INDEX_OFFSET_LEN; /**< == 64 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_INDEX_NUMBER_LEN; /**< == 8 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_INDEX_RESERVED_LEN; /**< == 3*8 (bits) */


/** FLAC CUESHEET track structure.  (See the
 * <A HREF="../format.html#cuesheet_track">format specification</A> for
 * the full description of each field.)
 */
typedef struct {
	FLAC__uint64 offset;
	/**< Track offset in samples, relative to the beginning of the FLAC audio stream. */

	FLAC__byte number;
	/**< The track number. */

	char isrc[13];
	/**< Track ISRC.  This is a 12-digit alphanumeric code plus a trailing \c NUL byte */

	uint32_t type:1;
	/**< The track type: 0 for audio, 1 for non-audio. */

	uint32_t pre_emphasis:1;
	/**< The pre-emphasis flag: 0 for no pre-emphasis, 1 for pre-emphasis. */

	FLAC__byte num_indices;
	/**< The number of track index points. */

	FLAC__StreamMetadata_CueSheet_Index *indices;
	/**< NULL if num_indices == 0, else pointer to array of index points. */

} FLAC__StreamMetadata_CueSheet_Track;

extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_TRACK_OFFSET_LEN; /**< == 64 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_TRACK_NUMBER_LEN; /**< == 8 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_TRACK_ISRC_LEN; /**< == 12*8 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_TRACK_TYPE_LEN; /**< == 1 (bit) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_TRACK_PRE_EMPHASIS_LEN; /**< == 1 (bit) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_TRACK_RESERVED_LEN; /**< == 6+13*8 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_TRACK_NUM_INDICES_LEN; /**< == 8 (bits) */


/** FLAC CUESHEET structure.  (See the
 * <A HREF="../format.html#metadata_block_cuesheet">format specification</A>
 * for the full description of each field.)
 */
typedef struct {
	char media_catalog_number[129];
	/**< Media catalog number, in ASCII printable characters 0x20-0x7e.  In
	 * general, the media catalog number may be 0 to 128 bytes long; any
	 * unused characters should be right-padded with NUL characters.
	 */

	FLAC__uint64 lead_in;
	/**< The number of lead-in samples. */

	FLAC__bool is_cd;
	/**< \c true if CUESHEET corresponds to a Compact Disc, else \c false. */

	uint32_t num_tracks;
	/**< The number of tracks. */

	FLAC__StreamMetadata_CueSheet_Track *tracks;
	/**< NULL if num_tracks == 0, else pointer to array of tracks. */

} FLAC__StreamMetadata_CueSheet;

extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_MEDIA_CATALOG_NUMBER_LEN; /**< == 128*8 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_LEAD_IN_LEN; /**< == 64 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_IS_CD_LEN; /**< == 1 (bit) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_RESERVED_LEN; /**< == 7+258*8 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_CUESHEET_NUM_TRACKS_LEN; /**< == 8 (bits) */


/** An enumeration of the PICTURE types (see FLAC__StreamMetadataPicture and id3 v2.4 APIC tag). */
typedef enum {
	FLAC__STREAM_METADATA_PICTURE_TYPE_OTHER = 0, /**< Other */
	FLAC__STREAM_METADATA_PICTURE_TYPE_FILE_ICON_STANDARD = 1, /**< 32x32 pixels 'file icon' (PNG only) */
	FLAC__STREAM_METADATA_PICTURE_TYPE_FILE_ICON = 2, /**< Other file icon */
	FLAC__STREAM_METADATA_PICTURE_TYPE_FRONT_COVER = 3, /**< Cover (front) */
	FLAC__STREAM_METADATA_PICTURE_TYPE_BACK_COVER = 4, /**< Cover (back) */
	FLAC__STREAM_METADATA_PICTURE_TYPE_LEAFLET_PAGE = 5, /**< Leaflet page */
	FLAC__STREAM_METADATA_PICTURE_TYPE_MEDIA = 6, /**< Media (e.g. label side of CD) */
	FLAC__STREAM_METADATA_PICTURE_TYPE_LEAD_ARTIST = 7, /**< Lead artist/lead performer/soloist */
	FLAC__STREAM_METADATA_PICTURE_TYPE_ARTIST = 8, /**< Artist/performer */
	FLAC__STREAM_METADATA_PICTURE_TYPE_CONDUCTOR = 9, /**< Conductor */
	FLAC__STREAM_METADATA_PICTURE_TYPE_BAND = 10, /**< Band/Orchestra */
	FLAC__STREAM_METADATA_PICTURE_TYPE_COMPOSER = 11, /**< Composer */
	FLAC__STREAM_METADATA_PICTURE_TYPE_LYRICIST = 12, /**< Lyricist/text writer */
	FLAC__STREAM_METADATA_PICTURE_TYPE_RECORDING_LOCATION = 13, /**< Recording Location */
	FLAC__STREAM_METADATA_PICTURE_TYPE_DURING_RECORDING = 14, /**< During recording */
	FLAC__STREAM_METADATA_PICTURE_TYPE_DURING_PERFORMANCE = 15, /**< During performance */
	FLAC__STREAM_METADATA_PICTURE_TYPE_VIDEO_SCREEN_CAPTURE = 16, /**< Movie/video screen capture */
	FLAC__STREAM_METADATA_PICTURE_TYPE_FISH = 17, /**< A bright coloured fish */
	FLAC__STREAM_METADATA_PICTURE_TYPE_ILLUSTRATION = 18, /**< Illustration */
	FLAC__STREAM_METADATA_PICTURE_TYPE_BAND_LOGOTYPE = 19, /**< Band/artist logotype */
	FLAC__STREAM_METADATA_PICTURE_TYPE_PUBLISHER_LOGOTYPE = 20, /**< Publisher/Studio logotype */
	FLAC__STREAM_METADATA_PICTURE_TYPE_UNDEFINED
} FLAC__StreamMetadata_Picture_Type;

/** Maps a FLAC__StreamMetadata_Picture_Type to a C string.
 *
 *  Using a FLAC__StreamMetadata_Picture_Type as the index to this array
 *  will give the string equivalent.  The contents should not be
 *  modified.
 */
extern FLAC_API const char * const FLAC__StreamMetadata_Picture_TypeString[];

/** FLAC PICTURE structure.  (See the
 * <A HREF="../format.html#metadata_block_picture">format specification</A>
 * for the full description of each field.)
 */
typedef struct {
	FLAC__StreamMetadata_Picture_Type type;
	/**< The kind of picture stored. */

	char *mime_type;
	/**< Picture data's MIME type, in ASCII printable characters
	 * 0x20-0x7e, NUL terminated.  For best compatibility with players,
	 * use picture data of MIME type \c image/jpeg or \c image/png.  A
	 * MIME type of '-->' is also allowed, in which case the picture
	 * data should be a complete URL.  In file storage, the MIME type is
	 * stored as a 32-bit length followed by the ASCII string with no NUL
	 * terminator, but is converted to a plain C string in this structure
	 * for convenience.
	 */

	FLAC__byte *description;
	/**< Picture's description in UTF-8, NUL terminated.  In file storage,
	 * the description is stored as a 32-bit length followed by the UTF-8
	 * string with no NUL terminator, but is converted to a plain C string
	 * in this structure for convenience.
	 */

	FLAC__uint32 width;
	/**< Picture's width in pixels. */

	FLAC__uint32 height;
	/**< Picture's height in pixels. */

	FLAC__uint32 depth;
	/**< Picture's color depth in bits-per-pixel. */

	FLAC__uint32 colors;
	/**< For indexed palettes (like GIF), picture's number of colors (the
	 * number of palette entries), or \c 0 for non-indexed (i.e. 2^depth).
	 */

	FLAC__uint32 data_length;
	/**< Length of binary picture data in bytes. */

	FLAC__byte *data;
	/**< Binary picture data. */

} FLAC__StreamMetadata_Picture;

extern FLAC_API const uint32_t FLAC__STREAM_METADATA_PICTURE_TYPE_LEN; /**< == 32 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_PICTURE_MIME_TYPE_LENGTH_LEN; /**< == 32 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_PICTURE_DESCRIPTION_LENGTH_LEN; /**< == 32 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_PICTURE_WIDTH_LEN; /**< == 32 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_PICTURE_HEIGHT_LEN; /**< == 32 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_PICTURE_DEPTH_LEN; /**< == 32 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_PICTURE_COLORS_LEN; /**< == 32 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_PICTURE_DATA_LENGTH_LEN; /**< == 32 (bits) */


/** Structure that is used when a metadata block of unknown type is loaded.
 *  The contents are opaque.  The structure is used only internally to
 *  correctly handle unknown metadata.
 */
typedef struct {
	FLAC__byte *data;
} FLAC__StreamMetadata_Unknown;


/** FLAC metadata block structure.  (c.f. <A HREF="../format.html#metadata_block">format specification</A>)
 */
typedef struct {
	FLAC__MetadataType type;
	/**< The type of the metadata block; used determine which member of the
	 * \a data union to dereference.  If type >= FLAC__METADATA_TYPE_UNDEFINED
	 * then \a data.unknown must be used. */

	FLAC__bool is_last;
	/**< \c true if this metadata block is the last, else \a false */

	uint32_t length;
	/**< Length, in bytes, of the block data as it appears in the stream. */

	union {
		FLAC__StreamMetadata_StreamInfo stream_info;
		FLAC__StreamMetadata_Padding padding;
		FLAC__StreamMetadata_Application application;
		FLAC__StreamMetadata_SeekTable seek_table;
		FLAC__StreamMetadata_VorbisComment vorbis_comment;
		FLAC__StreamMetadata_CueSheet cue_sheet;
		FLAC__StreamMetadata_Picture picture;
		FLAC__StreamMetadata_Unknown unknown;
	} data;
	/**< Polymorphic block data; use the \a type value to determine which
	 * to use. */
} FLAC__StreamMetadata;

extern FLAC_API const uint32_t FLAC__STREAM_METADATA_IS_LAST_LEN; /**< == 1 (bit) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_TYPE_LEN; /**< == 7 (bits) */
extern FLAC_API const uint32_t FLAC__STREAM_METADATA_LENGTH_LEN; /**< == 24 (bits) */

/** The total stream length of a metadata block header in bytes. */
#define FLAC__STREAM_METADATA_HEADER_LENGTH (4u)

/*****************************************************************************/


/*****************************************************************************
 *
 * Utility functions
 *
 *****************************************************************************/

/** Tests that a sample rate is valid for FLAC.
 *
 * \param sample_rate  The sample rate to test for compliance.
 * \retval FLAC__bool
 *    \c true if the given sample rate conforms to the specification, else
 *    \c false.
 */
FLAC_API FLAC__bool FLAC__format_sample_rate_is_valid(uint32_t sample_rate);

/** Tests that a blocksize at the given sample rate is valid for the FLAC
 *  subset.
 *
 * \param blocksize    The blocksize to test for compliance.
 * \param sample_rate  The sample rate is needed, since the valid subset
 *                     blocksize depends on the sample rate.
 * \retval FLAC__bool
 *    \c true if the given blocksize conforms to the specification for the
 *    subset at the given sample rate, else \c false.
 */
FLAC_API FLAC__bool FLAC__format_blocksize_is_subset(uint32_t blocksize, uint32_t sample_rate);

/** Tests that a sample rate is valid for the FLAC subset.  The subset rules
 *  for valid sample rates are slightly more complex since the rate has to
 *  be expressible completely in the frame header.
 *
 * \param sample_rate  The sample rate to test for compliance.
 * \retval FLAC__bool
 *    \c true if the given sample rate conforms to the specification for the
 *    subset, else \c false.
 */
FLAC_API FLAC__bool FLAC__format_sample_rate_is_subset(uint32_t sample_rate);

/** Check a Vorbis comment entry name to see if it conforms to the Vorbis
 *  comment specification.
 *
 *  Vorbis comment names must be composed only of characters from
 *  [0x20-0x3C,0x3E-0x7D].
 *
 * \param name       A NUL-terminated string to be checked.
 * \assert
 *    \code name != NULL \endcode
 * \retval FLAC__bool
 *    \c false if entry name is illegal, else \c true.
 */
FLAC_API FLAC__bool FLAC__format_vorbiscomment_entry_name_is_legal(const char *name);

/** Check a Vorbis comment entry value to see if it conforms to the Vorbis
 *  comment specification.
 *
 *  Vorbis comment values must be valid UTF-8 sequences.
 *
 * \param value      A string to be checked.
 * \param length     A the length of \a value in bytes.  May be
 *                   \c (uint32_t)(-1) to indicate that \a value is a plain
 *                   UTF-8 NUL-terminated string.
 * \assert
 *    \code value != NULL \endcode
 * \retval FLAC__bool
 *    \c false if entry name is illegal, else \c true.
 */
FLAC_API FLAC__bool FLAC__format_vorbiscomment_entry_value_is_legal(const FLAC__byte *value, uint32_t length);

/** Check a Vorbis comment entry to see if it conforms to the Vorbis
 *  comment specification.
 *
 *  Vorbis comment entries must be of the form 'name=value', and 'name' and
 *  'value' must be legal according to
 *  FLAC__format_vorbiscomment_entry_name_is_legal() and
 *  FLAC__format_vorbiscomment_entry_value_is_legal() respectively.
 *
 * \param entry      An entry to be checked.
 * \param length     The length of \a entry in bytes.
 * \assert
 *    \code value != NULL \endcode
 * \retval FLAC__bool
 *    \c false if entry name is illegal, else \c true.
 */
FLAC_API FLAC__bool FLAC__format_vorbiscomment_entry_is_legal(const FLAC__byte *entry, uint32_t length);

/** Check a seek table to see if it conforms to the FLAC specification.
 *  See the format specification for limits on the contents of the
 *  seek table.
 *
 * \param seek_table  A pointer to a seek table to be checked.
 * \assert
 *    \code seek_table != NULL \endcode
 * \retval FLAC__bool
 *    \c false if seek table is illegal, else \c true.
 */
FLAC_API FLAC__bool FLAC__format_seektable_is_legal(const FLAC__StreamMetadata_SeekTable *seek_table);

/** Sort a seek table's seek points according to the format specification.
 *  This includes a "unique-ification" step to remove duplicates, i.e.
 *  seek points with identical \a sample_number values.  Duplicate seek
 *  points are converted into placeholder points and sorted to the end of
 *  the table.
 *
 * \param seek_table  A pointer to a seek table to be sorted.
 * \assert
 *    \code seek_table != NULL \endcode
 * \retval uint32_t
 *    The number of duplicate seek points converted into placeholders.
 */
FLAC_API uint32_t FLAC__format_seektable_sort(FLAC__StreamMetadata_SeekTable *seek_table);

/** Check a cue sheet to see if it conforms to the FLAC specification.
 *  See the format specification for limits on the contents of the
 *  cue sheet.
 *
 * \param cue_sheet  A pointer to an existing cue sheet to be checked.
 * \param check_cd_da_subset  If \c true, check CUESHEET against more
 *                   stringent requirements for a CD-DA (audio) disc.
 * \param violation  Address of a pointer to a string.  If there is a
 *                   violation, a pointer to a string explanation of the
 *                   violation will be returned here. \a violation may be
 *                   \c NULL if you don't need the returned string.  Do not
 *                   free the returned string; it will always point to static
 *                   data.
 * \assert
 *    \code cue_sheet != NULL \endcode
 * \retval FLAC__bool
 *    \c false if cue sheet is illegal, else \c true.
 */
FLAC_API FLAC__bool FLAC__format_cuesheet_is_legal(const FLAC__StreamMetadata_CueSheet *cue_sheet, FLAC__bool check_cd_da_subset, const char **violation);

/** Check picture data to see if it conforms to the FLAC specification.
 *  See the format specification for limits on the contents of the
 *  PICTURE block.
 *
 * \param picture    A pointer to existing picture data to be checked.
 * \param violation  Address of a pointer to a string.  If there is a
 *                   violation, a pointer to a string explanation of the
 *                   violation will be returned here. \a violation may be
 *                   \c NULL if you don't need the returned string.  Do not
 *                   free the returned string; it will always point to static
 *                   data.
 * \assert
 *    \code picture != NULL \endcode
 * \retval FLAC__bool
 *    \c false if picture data is illegal, else \c true.
 */
FLAC_API FLAC__bool FLAC__format_picture_is_legal(const FLAC__StreamMetadata_Picture *picture, const char **violation);

/* \} */

#ifdef __cplusplus
}
#endif

#endif
