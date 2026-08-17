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

#ifndef AVCODEC_CBS_BSF_H
#define AVCODEC_CBS_BSF_H

#include "libavutil/log.h"
#include "libavutil/opt.h"

#include "bsf.h"
#include "codec_id.h"
#include "cbs.h"
#include "packet.h"


typedef struct CBSBSFType {
    enum AVCodecID codec_id;

    // Name of a frame fragment in this codec (e.g. "access unit",
    // "temporal unit").
    const char *fragment_name;

    // Name of a unit for this BSF, for use in error messages (e.g.
    // "NAL unit", "OBU").
    const char *unit_name;

    // Update the content of a fragment with whatever metadata changes
    // are desired.  The associated AVPacket is provided so that any side
    // data associated with the fragment can be inspected or edited.  If
    // pkt is NULL, then an extradata header fragment is being updated.
    int (*update_fragment)(AVBSFContext *bsf, AVPacket *pkt,
                           CodedBitstreamFragment *frag);
} CBSBSFType;

// Common structure for all generic CBS BSF users.  An instance of this
// structure must be the first member of the BSF private context (to be
// pointed to by AVBSFContext.priv_data).
typedef struct CBSBSFContext {
    const AVClass         *class;
    const CBSBSFType      *type;

    CodedBitstreamContext *input;
    CodedBitstreamContext *output;
    CodedBitstreamFragment fragment;
} CBSBSFContext;

/**
 * Initialise generic CBS BSF setup.
 *
 * Creates the input and output CBS instances, and applies the filter to
 * the extradata on the input codecpar if any is present.
 *
 * Since it calls the update_fragment() function immediately to deal with
 * extradata, this should be called after any codec-specific setup is done
 * (probably at the end of the FFBitStreamFilter.init function).
 */
int ff_cbs_bsf_generic_init(AVBSFContext *bsf, const CBSBSFType *type);

/**
 * Close a generic CBS BSF instance.
 *
 * If no other deinitialisation is required then this function can be used
 * directly as FFBitStreamFilter.close.
 */
void ff_cbs_bsf_generic_close(AVBSFContext *bsf);

/**
 * Filter operation for CBS BSF.
 *
 * Reads the input packet into a CBS fragment, calls update_fragment() on
 * it, then writes the result to an output packet.  If the input packet
 * has AV_PKT_DATA_NEW_EXTRADATA side-data associated with it then it does
 * the same thing to that new extradata to form the output side-data first.
 *
 * If the BSF does not do anything else then this function can be used
 * directly as FFBitStreamFilter.filter.
 */
int ff_cbs_bsf_generic_filter(AVBSFContext *bsf, AVPacket *pkt);


// Options for element manipulation.
enum {
    // Pass this element through unchanged.
    BSF_ELEMENT_PASS,
    // Insert this element, replacing any existing instances of it.
    // Associated values may be provided explicitly (as additional options)
    // or implicitly (either as side data or deduced from other parts of
    // the stream).
    BSF_ELEMENT_INSERT,
    // Remove this element if it appears in the stream.
    BSF_ELEMENT_REMOVE,
    // Extract this element to side data, so that further manipulation
    // can happen elsewhere.
    BSF_ELEMENT_EXTRACT,
};

#define BSF_ELEMENT_OPTIONS_PIR(name, help, field, opt_flags) \
    { name, help, OFFSET(field), AV_OPT_TYPE_INT, \
        { .i64 = BSF_ELEMENT_PASS }, \
        BSF_ELEMENT_PASS, BSF_ELEMENT_REMOVE, opt_flags, .unit = name }, \
    { "pass",   NULL, 0, AV_OPT_TYPE_CONST, \
        { .i64 = BSF_ELEMENT_PASS   }, .flags = opt_flags, .unit = name }, \
    { "insert", NULL, 0, AV_OPT_TYPE_CONST, \
        { .i64 = BSF_ELEMENT_INSERT }, .flags = opt_flags, .unit = name }, \
    { "remove", NULL, 0, AV_OPT_TYPE_CONST, \
        { .i64 = BSF_ELEMENT_REMOVE }, .flags = opt_flags, .unit = name }

#define BSF_ELEMENT_OPTIONS_PIRE(name, help, field, opt_flags) \
    { name, help, OFFSET(field), AV_OPT_TYPE_INT, \
        { .i64 = BSF_ELEMENT_PASS }, \
        BSF_ELEMENT_PASS, BSF_ELEMENT_EXTRACT, opt_flags, .unit = name }, \
    { "pass",   NULL, 0, AV_OPT_TYPE_CONST, \
        { .i64 = BSF_ELEMENT_PASS    }, .flags = opt_flags, .unit = name }, \
    { "insert", NULL, 0, AV_OPT_TYPE_CONST, \
        { .i64 = BSF_ELEMENT_INSERT  }, .flags = opt_flags, .unit = name }, \
    { "remove", NULL, 0, AV_OPT_TYPE_CONST, \
        { .i64 = BSF_ELEMENT_REMOVE  }, .flags = opt_flags, .unit = name }, \
    { "extract", NULL, 0, AV_OPT_TYPE_CONST, \
        { .i64 = BSF_ELEMENT_EXTRACT }, .flags = opt_flags, .unit = name } \


#endif /* AVCODEC_CBS_BSF_H */
