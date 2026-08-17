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

#ifndef AVFILTER_FORMATS_H
#define AVFILTER_FORMATS_H

#include "avfilter.h"

/**
 * A list of supported formats for one end of a filter link. This is used
 * during the format negotiation process to try to pick the best format to
 * use to minimize the number of necessary conversions. Each filter gives a
 * list of the formats supported by each input and output pad. The list
 * given for each pad need not be distinct - they may be references to the
 * same list of formats, as is often the case when a filter supports multiple
 * formats, but will always output the same format as it is given in input.
 *
 * In this way, a list of possible input formats and a list of possible
 * output formats are associated with each link. When a set of formats is
 * negotiated over a link, the input and output lists are merged to form a
 * new list containing only the common elements of each list. In the case
 * that there were no common elements, a format conversion is necessary.
 * Otherwise, the lists are merged, and all other links which reference
 * either of the format lists involved in the merge are also affected.
 *
 * For example, consider the filter chain:
 * filter (a) --> (b) filter (b) --> (c) filter
 *
 * where the letters in parenthesis indicate a list of formats supported on
 * the input or output of the link. Suppose the lists are as follows:
 * (a) = {A, B}
 * (b) = {A, B, C}
 * (c) = {B, C}
 *
 * First, the first link's lists are merged, yielding:
 * filter (a) --> (a) filter (a) --> (c) filter
 *
 * Notice that format list (b) now refers to the same list as filter list (a).
 * Next, the lists for the second link are merged, yielding:
 * filter (a) --> (a) filter (a) --> (a) filter
 *
 * where (a) = {B}.
 *
 * Unfortunately, when the format lists at the two ends of a link are merged,
 * we must ensure that all links which reference either pre-merge format list
 * get updated as well. Therefore, we have the format list structure store a
 * pointer to each of the pointers to itself.
 */
struct AVFilterFormats {
    unsigned nb_formats;        ///< number of formats
    int *formats;               ///< list of media formats

    unsigned refcount;          ///< number of references to this list
    struct AVFilterFormats ***refs; ///< references to this list
};

/**
 * A list of supported channel layouts.
 *
 * The list works the same as AVFilterFormats, except for the following
 * differences:
 * - A list with all_layouts = 1 means all channel layouts with a known
 *   disposition; nb_channel_layouts must then be 0.
 * - A list with all_counts = 1 means all channel counts, with a known or
 *   unknown disposition; nb_channel_layouts must then be 0 and all_layouts 1.
 * - The list must not contain a layout with a known disposition and a
 *   channel count with unknown disposition with the same number of channels
 *   (e.g. AV_CH_LAYOUT_STEREO and FF_COUNT2LAYOUT(2).
 */
struct AVFilterChannelLayouts {
    AVChannelLayout *channel_layouts; ///< list of channel layouts
    int    nb_channel_layouts;  ///< number of channel layouts
    char all_layouts;           ///< accept any known channel layout
    char all_counts;            ///< accept any channel layout or count

    unsigned refcount;          ///< number of references to this list
    struct AVFilterChannelLayouts ***refs; ///< references to this list
};

/**
 * Encode a channel count as a channel layout.
 * FF_COUNT2LAYOUT(c) means any channel layout with c channels, with a known
 * or unknown disposition.
 * The result is only valid inside AVFilterChannelLayouts and immediately
 * related functions.
 */
#define FF_COUNT2LAYOUT(c) ((AVChannelLayout) { .order = AV_CHANNEL_ORDER_UNSPEC, .nb_channels = c })

/**
 * Decode a channel count encoded as a channel layout.
 * Return 0 if the channel layout was a real one.
 */
#define FF_LAYOUT2COUNT(l) (((l)->order == AV_CHANNEL_ORDER_UNSPEC) ? \
                            (l)->nb_channels : 0)

#define KNOWN(l) (!FF_LAYOUT2COUNT(l)) /* for readability */

/**
 * Construct an empty AVFilterChannelLayouts/AVFilterFormats struct --
 * representing any channel layout (with known disposition)/sample rate.
 */
av_warn_unused_result
AVFilterChannelLayouts *ff_all_channel_layouts(void);

av_warn_unused_result
AVFilterFormats *ff_all_samplerates(void);

/**
 * Construct an AVFilterChannelLayouts coding for any channel layout, with
 * known or unknown disposition.
 */
av_warn_unused_result
AVFilterChannelLayouts *ff_all_channel_counts(void);

av_warn_unused_result
AVFilterChannelLayouts *ff_make_channel_layout_list(const AVChannelLayout *fmts);

/**
 * Construct an AVFilterFormats representing all possible color spaces.
 *
 * Note: This list does not include AVCOL_SPC_RESERVED.
 */
av_warn_unused_result
AVFilterFormats *ff_all_color_spaces(void);

/**
 * Construct an AVFilterFormats representing all possible color ranges.
 */
av_warn_unused_result
AVFilterFormats *ff_all_color_ranges(void);

/**
 * Helpers for query_formats() which set all free audio links to the same list
 * of channel layouts/sample rates. If there are no links hooked to this list,
 * the list is freed.
 */
av_warn_unused_result
int ff_set_common_channel_layouts(AVFilterContext *ctx,
                                  AVFilterChannelLayouts *layouts);
/**
 * Equivalent to ff_set_common_channel_layouts(ctx, ff_make_channel_layout_list(fmts))
 */
av_warn_unused_result
int ff_set_common_channel_layouts_from_list(AVFilterContext *ctx,
                                            const AVChannelLayout *fmts);
/**
 * Equivalent to ff_set_common_channel_layouts(ctx, ff_all_channel_counts())
 */
av_warn_unused_result
int ff_set_common_all_channel_counts(AVFilterContext *ctx);

av_warn_unused_result
int ff_set_common_samplerates(AVFilterContext *ctx,
                              AVFilterFormats *samplerates);
/**
 * Equivalent to ff_set_common_samplerates(ctx, ff_make_format_list(samplerates))
 */
av_warn_unused_result
int ff_set_common_samplerates_from_list(AVFilterContext *ctx,
                                        const int *samplerates);
/**
 * Equivalent to ff_set_common_samplerates(ctx, ff_all_samplerates())
 */
av_warn_unused_result
int ff_set_common_all_samplerates(AVFilterContext *ctx);

av_warn_unused_result
int ff_set_common_color_spaces(AVFilterContext *ctx,
                               AVFilterFormats *color_spaces);
/**
 * Equivalent to ff_set_common_color_spaces(ctx, ff_make_format_list(color_spaces))
 */
av_warn_unused_result
int ff_set_common_color_spaces_from_list(AVFilterContext *ctx,
                                         const int *color_spaces);

/**
 * Equivalent to ff_set_common_color_spaces(ctx, ff_all_color_spaces())
 */
av_warn_unused_result
int ff_set_common_all_color_spaces(AVFilterContext *ctx);

av_warn_unused_result
int ff_set_common_color_ranges(AVFilterContext *ctx,
                               AVFilterFormats *color_ranges);
/**
 * Equivalent to ff_set_common_color_ranges(ctx, ff_make_format_list(color_ranges))
 */
av_warn_unused_result
int ff_set_common_color_ranges_from_list(AVFilterContext *ctx,
                                         const int *color_ranges);

/**
 * Equivalent to ff_set_common_color_ranges(ctx, ff_all_color_ranges())
 */
av_warn_unused_result
int ff_set_common_all_color_ranges(AVFilterContext *ctx);

/**
 * A helper for query_formats() which sets all links to the same list of
 * formats. If there are no links hooked to this filter, the list of formats is
 * freed.
 */
av_warn_unused_result
int ff_set_common_formats(AVFilterContext *ctx, AVFilterFormats *formats);

/**
 * Equivalent to ff_set_common_formats(ctx, ff_make_format_list(fmts))
 */
av_warn_unused_result
int ff_set_common_formats_from_list(AVFilterContext *ctx, const int *fmts);

/**
 * Helpers for query_formats2() which set all free audio links to the same list
 * of channel layouts/sample rates. If there are no links hooked to this list,
 * the list is freed.
 */
av_warn_unused_result
int ff_set_common_channel_layouts2(const AVFilterContext *ctx,
                                   AVFilterFormatsConfig **cfg_in,
                                   AVFilterFormatsConfig **cfg_out,
                                   AVFilterChannelLayouts *channel_layouts);

av_warn_unused_result
int ff_set_common_channel_layouts_from_list2(const AVFilterContext *ctx,
                                             AVFilterFormatsConfig **cfg_in,
                                             AVFilterFormatsConfig **cfg_out,
                                             const AVChannelLayout *fmts);
av_warn_unused_result
int ff_set_common_all_channel_counts2(const AVFilterContext *ctx,
                                      AVFilterFormatsConfig **cfg_in,
                                      AVFilterFormatsConfig **cfg_out);

av_warn_unused_result
int ff_set_common_samplerates2(const AVFilterContext *ctx,
                               AVFilterFormatsConfig **cfg_in,
                               AVFilterFormatsConfig **cfg_out,
                               AVFilterFormats *samplerates);

av_warn_unused_result
int ff_set_common_samplerates_from_list2(const AVFilterContext *ctx,
                                         AVFilterFormatsConfig **cfg_in,
                                         AVFilterFormatsConfig **cfg_out,
                                         const int *samplerates);

av_warn_unused_result
int ff_set_common_all_samplerates2(const AVFilterContext *ctx,
                                   AVFilterFormatsConfig **cfg_in,
                                   AVFilterFormatsConfig **cfg_out);

av_warn_unused_result
int ff_set_common_color_spaces2(const AVFilterContext *ctx,
                                AVFilterFormatsConfig **cfg_in,
                                AVFilterFormatsConfig **cfg_out,
                                AVFilterFormats *color_spaces);

av_warn_unused_result
int ff_set_common_color_spaces_from_list2(const AVFilterContext *ctx,
                                          AVFilterFormatsConfig **cfg_in,
                                          AVFilterFormatsConfig **cfg_out,
                                          const int *color_ranges);

av_warn_unused_result
int ff_set_common_all_color_spaces2(const AVFilterContext *ctx,
                                    AVFilterFormatsConfig **cfg_in,
                                    AVFilterFormatsConfig **cfg_out);

av_warn_unused_result
int ff_set_common_color_ranges2(const AVFilterContext *ctx,
                                AVFilterFormatsConfig **cfg_in,
                                AVFilterFormatsConfig **cfg_out,
                                AVFilterFormats *color_ranges);

av_warn_unused_result
int ff_set_common_color_ranges_from_list2(const AVFilterContext *ctx,
                                          AVFilterFormatsConfig **cfg_in,
                                          AVFilterFormatsConfig **cfg_out,
                                          const int *color_ranges);

av_warn_unused_result
int ff_set_common_all_color_ranges2(const AVFilterContext *ctx,
                                    AVFilterFormatsConfig **cfg_in,
                                    AVFilterFormatsConfig **cfg_out);

av_warn_unused_result
int ff_set_common_formats2(const AVFilterContext *ctx,
                           AVFilterFormatsConfig **cfg_in,
                           AVFilterFormatsConfig **cfg_out,
                           AVFilterFormats *formats);

av_warn_unused_result
int ff_set_common_formats_from_list2(const AVFilterContext *ctx,
                                     AVFilterFormatsConfig **cfg_in,
                                     AVFilterFormatsConfig **cfg_out,
                                     const int *fmts);

av_warn_unused_result
int ff_add_channel_layout(AVFilterChannelLayouts **l,
                          const AVChannelLayout *channel_layout);

/**
 * Add *ref as a new reference to f.
 */
av_warn_unused_result
int ff_channel_layouts_ref(AVFilterChannelLayouts *f,
                           AVFilterChannelLayouts **ref);

/**
 * Remove a reference to a channel layouts list.
 */
void ff_channel_layouts_unref(AVFilterChannelLayouts **ref);

void ff_channel_layouts_changeref(AVFilterChannelLayouts **oldref,
                                  AVFilterChannelLayouts **newref);

/**
 * Sets all remaining unset filter lists for all inputs/outputs to their
 * corresponding `ff_all_*()` lists.
 */
av_warn_unused_result
int ff_default_query_formats(AVFilterContext *ctx);

/**
 * Create a list of supported formats. This is intended for use in
 * AVFilter->query_formats().
 *
 * @param fmts list of media formats, terminated by -1
 * @return the format list, with no existing references
 */
av_warn_unused_result
AVFilterFormats *ff_make_format_list(const int *fmts);

/**
 * Equivalent to ff_make_format_list({const int[]}{ fmt, -1 })
 */
av_warn_unused_result
AVFilterFormats *ff_make_formats_list_singleton(int fmt);

/**
 * Add fmt to the list of media formats contained in *avff.
 * If *avff is NULL the function allocates the filter formats struct
 * and puts its pointer in *avff.
 *
 * @return a non negative value in case of success, or a negative
 * value corresponding to an AVERROR code in case of error
 */
av_warn_unused_result
int ff_add_format(AVFilterFormats **avff, int64_t fmt);

/**
 * Return a list of all formats supported by FFmpeg for the given media type.
 */
av_warn_unused_result
AVFilterFormats *ff_all_formats(enum AVMediaType type);

/**
 * Construct a formats list containing all pixel formats with certain
 * properties
 */
av_warn_unused_result
AVFilterFormats *ff_formats_pixdesc_filter(unsigned want, unsigned rej);

//* format is software, non-planar with sub-sampling
#define FF_PIX_FMT_FLAG_SW_FLAT_SUB (1 << 24)

/**
 * Construct a formats list containing all planar sample formats.
 */
av_warn_unused_result
AVFilterFormats *ff_planar_sample_fmts(void);

/**
 * Add *ref as a new reference to formats.
 * That is the pointers will point like in the ascii art below:
 *   ________
 *  |formats |<--------.
 *  |  ____  |     ____|___________________
 *  | |refs| |    |  __|_
 *  | |* * | |    | |  | |  AVFilterLink
 *  | |* *--------->|*ref|
 *  | |____| |    | |____|
 *  |________|    |________________________
 */
av_warn_unused_result
int ff_formats_ref(AVFilterFormats *formats, AVFilterFormats **ref);

/**
 * If *ref is non-NULL, remove *ref as a reference to the format list
 * it currently points to, deallocates that list if this was the last
 * reference, and sets *ref to NULL.
 *
 *         Before                                 After
 *   ________                               ________         NULL
 *  |formats |<--------.                   |formats |         ^
 *  |  ____  |     ____|________________   |  ____  |     ____|________________
 *  | |refs| |    |  __|_                  | |refs| |    |  __|_
 *  | |* * | |    | |  | |  AVFilterLink   | |* * | |    | |  | |  AVFilterLink
 *  | |* *--------->|*ref|                 | |*   | |    | |*ref|
 *  | |____| |    | |____|                 | |____| |    | |____|
 *  |________|    |_____________________   |________|    |_____________________
 */
void ff_formats_unref(AVFilterFormats **ref);

/**
 *         Before                                 After
 *   ________                         ________
 *  |formats |<---------.            |formats |<---------.
 *  |  ____  |       ___|___         |  ____  |       ___|___
 *  | |refs| |      |   |   |        | |refs| |      |   |   |   NULL
 *  | |* *--------->|*oldref|        | |* *--------->|*newref|     ^
 *  | |* * | |      |_______|        | |* * | |      |_______|  ___|___
 *  | |____| |                       | |____| |                |   |   |
 *  |________|                       |________|                |*oldref|
 *                                                             |_______|
 */
void ff_formats_changeref(AVFilterFormats **oldref, AVFilterFormats **newref);

/**
 * Check that fmts is a valid pixel formats list.
 *
 * In particular, check for duplicates.
 */
int ff_formats_check_pixel_formats(void *log, const AVFilterFormats *fmts);

/**
 * Check that fmts is a valid sample formats list.
 *
 * In particular, check for duplicates.
 */
int ff_formats_check_sample_formats(void *log, const AVFilterFormats *fmts);

/**
 * Check that fmts is a valid sample rates list.
 *
 * In particular, check for duplicates.
 */
int ff_formats_check_sample_rates(void *log, const AVFilterFormats *fmts);

/**
 * Check that fmts is a valid channel layouts list.
 *
 * In particular, check for duplicates.
 */
int ff_formats_check_channel_layouts(void *log, const AVFilterChannelLayouts *fmts);

/**
 * Check that fmts is a valid formats list for YUV colorspace metadata.
 *
 * In particular, check for duplicates.
 */
int ff_formats_check_color_spaces(void *log, const AVFilterFormats *fmts);
int ff_formats_check_color_ranges(void *log, const AVFilterFormats *fmts);

typedef struct AVFilterFormatMerger {
    unsigned offset;
    int (*merge)(void *a, void *b);
    int (*can_merge)(const void *a, const void *b);
} AVFilterFormatsMerger;

/**
 * Callbacks and properties to describe the steps of a format negotiation.
 *
 * The steps are:
 *
 * 1. query_formats(): call the callbacks on all filter to set lists of
 *                     supported formats.
 *                     When links on a filter must eventually have the same
 *                     format, the lists of supported formats are the same
 *                     object in memory.
 *                     See:
 *                     http://www.normalesup.org/~george/articles/format_negotiation_in_libavfilter/#12
 *
 *
 * 2. query_formats(): merge lists of supported formats or insert automatic
 *                     conversion filters.
 *                     Compute the intersection of the lists of supported
 *                     formats on the ends of links. If it succeeds, replace
 *                     both objects with the intersection everywhere they
 *                     are referenced.
 *                     If the intersection is empty, insert an automatic
 *                     conversion filter.
 *                     If several formats are negotiated at once (format,
 *                     rate, layout), only merge if all three can be, since
 *                     the conversion filter can convert all three at once.
 *                     This process goes on as long as progress is made.
 *                     See:
 *                     http://www.normalesup.org/~george/articles/format_negotiation_in_libavfilter/#14
 *                     http://www.normalesup.org/~george/articles/format_negotiation_in_libavfilter/#29
 *
 * 3. reduce_formats(): try to reduce format conversion within filters.
 *                      For each link where there is only one supported
 *                      formats on output, for each output of the connected
 *                      filter, if the media type is the same and said
 *                      format is supported, keep only this one.
 *                      This process goes on as long as progress is made.
 *                      Rationale: conversion filters will set a large list
 *                      of supported formats on outputs but users will
 *                      expect the output to be as close as possible as the
 *                      input (examples: scale without changing the pixel
 *                      format, resample without changint the layout).
 *                      FIXME: this can probably be done by merging the
 *                      input and output lists instead of re-implementing
 *                      the logic.
 *
 * 4. swap_sample_fmts():
 *    swap_samplerates():
 *    swap_channel_layouts(): For each filter with an input with only one
 *                            supported format, when outputs have several
 *                            supported formats, put the best one with
 *                            reference to the input at the beginning of the
 *                            list, to prepare it for being picked up by
 *                            pick_formats().
 *                            The best format is the one that is most
 *                            similar to the input while not losing too much
 *                            information.
 *                            This process need to run only once.
 *                            FIXME: reduce_formats() operates on all inputs
 *                            with a single format, swap_*() operates on the
 *                            first one only: check if the difference makes
 *                            sense.
 *                            TODO: the swapping done for one filter can
 *                            override the swapping done for another filter
 *                            connected to the same list of formats, maybe
 *                            it would be better to compute a total score
 *                            for all connected filters and use the score to
 *                            pick the format instead of just swapping.
 *                            TODO: make the similarity logic available as
 *                            public functions in libavutil.
 *
 * 5. pick_formats(): Choose one format from the lists of supported formats,
 *                    use it for the link and reduce the list to a single
 *                    element to force other filters connected to the same
 *                    list to use it.
 *                    First process all links where there is a single format
 *                    and the output links of all filters with an input,
 *                    trying to preserve similarity between input and
 *                    outputs.
 *                    Repeat as long as process is made.
 *                    Then do a final run for the remaining filters.
 *                    FIXME: the similarity logic (the ref argument to
 *                    pick_format()) added in FFmpeg duplicates and
 *                    overrides the swapping logic added in libav. Better
 *                    merge them into a score system.
 */
typedef struct AVFilterNegotiation {
    unsigned nb_mergers;
    const AVFilterFormatsMerger *mergers;
    const char *conversion_filter;
    unsigned conversion_opts_offset;
} AVFilterNegotiation;

const AVFilterNegotiation *ff_filter_get_negotiation(AVFilterLink *link);

#endif /* AVFILTER_FORMATS_H */
