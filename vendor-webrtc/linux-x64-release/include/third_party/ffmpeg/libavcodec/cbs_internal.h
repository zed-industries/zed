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

#ifndef AVCODEC_CBS_INTERNAL_H
#define AVCODEC_CBS_INTERNAL_H

#include <stddef.h>
#include <stdint.h>

#include "libavutil/log.h"

#include "cbs.h"
#include "codec_id.h"
#include "get_bits.h"
#include "put_bits.h"
#include "libavutil/refstruct.h"


enum CBSContentType {
    // Unit content may contain some references to other structures, but all
    // managed via buffer reference counting.  The descriptor defines the
    // structure offsets of every buffer reference.
    CBS_CONTENT_TYPE_INTERNAL_REFS,
    // Unit content is something more complex.  The descriptor defines
    // special functions to manage the content.
    CBS_CONTENT_TYPE_COMPLEX,
};

enum {
      // Maximum number of unit types described by the same non-range
      // unit type descriptor.
      CBS_MAX_LIST_UNIT_TYPES = 3,
      // Maximum number of reference buffer offsets in any one unit.
      CBS_MAX_REF_OFFSETS = 2,
      // Special value used in a unit type descriptor to indicate that it
      // applies to a large range of types rather than a set of discrete
      // values.
      CBS_UNIT_TYPE_RANGE = -1,
};

typedef const struct CodedBitstreamUnitTypeDescriptor {
    // Number of entries in the unit_types array, or the special value
    // CBS_UNIT_TYPE_RANGE to indicate that the range fields should be
    // used instead.
    int nb_unit_types;

    union {
        // Array of unit types that this entry describes.
        CodedBitstreamUnitType list[CBS_MAX_LIST_UNIT_TYPES];
        // Start and end of unit type range, used if nb_unit_types is
        // CBS_UNIT_TYPE_RANGE.
        struct {
            CodedBitstreamUnitType start;
            CodedBitstreamUnitType end;
        } range;
    } unit_type;

    // The type of content described.
    enum CBSContentType content_type;
    // The size of the structure which should be allocated to contain
    // the decomposed content of this type of unit.
    size_t content_size;

    union {
        // This union's state is determined by content_type:
        // ref for CBS_CONTENT_TYPE_INTERNAL_REFS,
        // complex for CBS_CONTENT_TYPE_COMPLEX.
        struct {
            // Number of entries in the ref_offsets array.
            // May be zero, then the structure is POD-like.
            int nb_offsets;
            // The structure must contain two adjacent elements:
            //   type        *field;
            //   AVBufferRef *field_ref;
            // where field points to something in the buffer referred to by
            // field_ref.  This offset is then set to offsetof(struct, field).
            size_t offsets[CBS_MAX_REF_OFFSETS];
        } ref;

        struct {
            void (*content_free)(AVRefStructOpaque opaque, void *content);
            int  (*content_clone)(void **new_content, CodedBitstreamUnit *unit);
        } complex;
    } type;
} CodedBitstreamUnitTypeDescriptor;

typedef struct CodedBitstreamType {
    enum AVCodecID codec_id;

    // A class for the private data, used to declare private AVOptions.
    // This field is NULL for types that do not declare any options.
    // If this field is non-NULL, the first member of the filter private data
    // must be a pointer to AVClass.
    const AVClass *priv_class;

    size_t priv_data_size;

    // List of unit type descriptors for this codec.
    // Terminated by a descriptor with nb_unit_types equal to zero.
    const CodedBitstreamUnitTypeDescriptor *unit_types;

    // Split frag->data into coded bitstream units, creating the
    // frag->units array.  Fill data but not content on each unit.
    // The header argument should be set if the fragment came from
    // a header block, which may require different parsing for some
    // codecs (e.g. the AVCC header in H.264).
    int (*split_fragment)(CodedBitstreamContext *ctx,
                          CodedBitstreamFragment *frag,
                          int header);

    // Read the unit->data bitstream and decompose it, creating
    // unit->content.
    int (*read_unit)(CodedBitstreamContext *ctx,
                     CodedBitstreamUnit *unit);

    // Write the data bitstream from unit->content into pbc.
    // Return value AVERROR(ENOSPC) indicates that pbc was too small.
    int (*write_unit)(CodedBitstreamContext *ctx,
                      CodedBitstreamUnit *unit,
                      PutBitContext *pbc);

    // Return 1 when the unit should be dropped according to 'skip',
    // 0 otherwise.
    int (*discarded_unit)(CodedBitstreamContext *ctx,
                          const CodedBitstreamUnit *unit,
                          enum AVDiscard skip);

    // Read the data from all of frag->units and assemble it into
    // a bitstream for the whole fragment.
    int (*assemble_fragment)(CodedBitstreamContext *ctx,
                             CodedBitstreamFragment *frag);

    // Reset the codec internal state.
    void (*flush)(CodedBitstreamContext *ctx);

    // Free the codec internal state.
    void (*close)(CodedBitstreamContext *ctx);
} CodedBitstreamType;


// Helper functions for trace output.

void ff_cbs_trace_header(CodedBitstreamContext *ctx,
                         const char *name);


// Helper functions for read/write of common bitstream elements, including
// generation of trace output. The simple functions are equivalent to
// their non-simple counterparts except that their range is unrestricted
// (i.e. only limited by the amount of bits used) and they lack
// the ability to use subscripts.

int ff_cbs_read_unsigned(CodedBitstreamContext *ctx, GetBitContext *gbc,
                         int width, const char *name,
                         const int *subscripts, uint32_t *write_to,
                         uint32_t range_min, uint32_t range_max);

int ff_cbs_read_simple_unsigned(CodedBitstreamContext *ctx, GetBitContext *gbc,
                                int width, const char *name, uint32_t *write_to);

int ff_cbs_write_unsigned(CodedBitstreamContext *ctx, PutBitContext *pbc,
                          int width, const char *name,
                          const int *subscripts, uint32_t value,
                          uint32_t range_min, uint32_t range_max);

int ff_cbs_write_simple_unsigned(CodedBitstreamContext *ctx, PutBitContext *pbc,
                                 int width, const char *name, uint32_t value);

int ff_cbs_read_signed(CodedBitstreamContext *ctx, GetBitContext *gbc,
                       int width, const char *name,
                       const int *subscripts, int32_t *write_to,
                       int32_t range_min, int32_t range_max);

int ff_cbs_write_signed(CodedBitstreamContext *ctx, PutBitContext *pbc,
                        int width, const char *name,
                        const int *subscripts, int32_t value,
                        int32_t range_min, int32_t range_max);

// The largest unsigned value representable in N bits, suitable for use as
// range_max in the above functions.
#define MAX_UINT_BITS(length) ((UINT64_C(1) << (length)) - 1)

// The largest signed value representable in N bits, suitable for use as
// range_max in the above functions.
#define MAX_INT_BITS(length) ((INT64_C(1) << ((length) - 1)) - 1)

// The smallest signed value representable in N bits, suitable for use as
// range_min in the above functions.
#define MIN_INT_BITS(length) (-(INT64_C(1) << ((length) - 1)))


// Start of a syntax element during read tracing.
#define CBS_TRACE_READ_START() \
    GetBitContext trace_start; \
    do { \
        if (ctx->trace_enable) \
            trace_start = *gbc; \
    } while (0)

// End of a syntax element for tracing, make callback.
#define CBS_TRACE_READ_END() \
    do { \
        if (ctx->trace_enable) { \
            int start_position = get_bits_count(&trace_start); \
            int end_position   = get_bits_count(gbc); \
            av_assert0(start_position <= end_position); \
            ctx->trace_read_callback(ctx->trace_context, &trace_start, \
                                     end_position - start_position, \
                                     name, subscripts, value); \
        } \
    } while (0)

// End of a syntax element with no subscript entries.
#define CBS_TRACE_READ_END_NO_SUBSCRIPTS() \
    do { \
        const int *subscripts = NULL; \
        CBS_TRACE_READ_END(); \
    } while (0)

// End of a syntax element which is made up of subelements which
// are aleady traced, so we are only showing the value.
#define CBS_TRACE_READ_END_VALUE_ONLY() \
    do { \
        if (ctx->trace_enable) { \
            ctx->trace_read_callback(ctx->trace_context, &trace_start, 0, \
                                     name, subscripts, value); \
        } \
    } while (0)

// Start of a syntax element during write tracing.
#define CBS_TRACE_WRITE_START() \
    int start_position; \
    do { \
        if (ctx->trace_enable) \
            start_position = put_bits_count(pbc);; \
    } while (0)

// End of a syntax element for tracing, make callback.
#define CBS_TRACE_WRITE_END() \
    do { \
        if (ctx->trace_enable) { \
            int end_position   = put_bits_count(pbc); \
            av_assert0(start_position <= end_position); \
            ctx->trace_write_callback(ctx->trace_context, pbc, \
                                      end_position - start_position, \
                                      name, subscripts, value); \
        } \
    } while (0)

// End of a syntax element with no subscript entries.
#define CBS_TRACE_WRITE_END_NO_SUBSCRIPTS() \
    do { \
        const int *subscripts = NULL; \
        CBS_TRACE_WRITE_END(); \
    } while (0)

// End of a syntax element which is made up of subelements which are
// aleady traced, so we are only showing the value.  This forges a
// PutBitContext to point to the position of the start of the syntax
// element, but the other state doesn't matter because length is zero.
#define CBS_TRACE_WRITE_END_VALUE_ONLY() \
    do { \
        if (ctx->trace_enable) { \
            PutBitContext tmp; \
            init_put_bits(&tmp, pbc->buf, start_position); \
            skip_put_bits(&tmp, start_position); \
            ctx->trace_write_callback(ctx->trace_context, &tmp, 0, \
                                      name, subscripts, value); \
        } \
    } while (0)

#define TYPE_LIST(...) { __VA_ARGS__ }
#define CBS_UNIT_TYPE_POD(type_, structure) { \
        .nb_unit_types  = 1, \
        .unit_type.list = { type_ }, \
        .content_type   = CBS_CONTENT_TYPE_INTERNAL_REFS, \
        .content_size   = sizeof(structure), \
        .type.ref       = { .nb_offsets = 0 }, \
    }
#define CBS_UNIT_RANGE_POD(range_start, range_end, structure) { \
        .nb_unit_types         = CBS_UNIT_TYPE_RANGE, \
        .unit_type.range.start = range_start, \
        .unit_type.range.end   = range_end, \
        .content_type          = CBS_CONTENT_TYPE_INTERNAL_REFS, \
        .content_size          = sizeof(structure), \
        .type.ref              = { .nb_offsets = 0 }, \
    }

#define CBS_UNIT_TYPES_INTERNAL_REF(types, structure, ref_field) { \
        .nb_unit_types  = FF_ARRAY_ELEMS((CodedBitstreamUnitType[])TYPE_LIST types), \
        .unit_type.list = TYPE_LIST types, \
        .content_type   = CBS_CONTENT_TYPE_INTERNAL_REFS, \
        .content_size   = sizeof(structure), \
        .type.ref       = { .nb_offsets = 1, \
                            .offsets    = { offsetof(structure, ref_field) } }, \
    }
#define CBS_UNIT_TYPE_INTERNAL_REF(type, structure, ref_field) \
    CBS_UNIT_TYPES_INTERNAL_REF((type), structure, ref_field)

#define CBS_UNIT_RANGE_INTERNAL_REF(range_start, range_end, structure, ref_field) { \
        .nb_unit_types         = CBS_UNIT_TYPE_RANGE, \
        .unit_type.range.start = range_start, \
        .unit_type.range.end   = range_end, \
        .content_type          = CBS_CONTENT_TYPE_INTERNAL_REFS, \
        .content_size          = sizeof(structure), \
        .type.ref = { .nb_offsets = 1, \
                      .offsets    = { offsetof(structure, ref_field) } }, \
    }

#define CBS_UNIT_TYPES_COMPLEX(types, structure, free_func) { \
        .nb_unit_types  = FF_ARRAY_ELEMS((CodedBitstreamUnitType[])TYPE_LIST types), \
        .unit_type.list = TYPE_LIST types, \
        .content_type   = CBS_CONTENT_TYPE_COMPLEX, \
        .content_size   = sizeof(structure), \
        .type.complex   = { .content_free = free_func }, \
    }
#define CBS_UNIT_TYPE_COMPLEX(type, structure, free_func) \
    CBS_UNIT_TYPES_COMPLEX((type), structure, free_func)

#define CBS_UNIT_TYPE_END_OF_LIST { .nb_unit_types = 0 }


extern const CodedBitstreamType ff_cbs_type_av1;
extern const CodedBitstreamType ff_cbs_type_h264;
extern const CodedBitstreamType ff_cbs_type_h265;
extern const CodedBitstreamType ff_cbs_type_h266;
extern const CodedBitstreamType ff_cbs_type_jpeg;
extern const CodedBitstreamType ff_cbs_type_mpeg2;
extern const CodedBitstreamType ff_cbs_type_vp8;
extern const CodedBitstreamType ff_cbs_type_vp9;


#endif /* AVCODEC_CBS_INTERNAL_H */
