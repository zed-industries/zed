/*
 * Copyright (c) 2013 Vittorio Giovara <vittorio.giovara@gmail.com>
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
 * Stereoscopic video
 */

#ifndef AVUTIL_STEREO3D_H
#define AVUTIL_STEREO3D_H

#include <stdint.h>

#include "frame.h"

/**
 * @addtogroup lavu_video
 * @{
 *
 * @defgroup lavu_video_stereo3d Stereo3D types and functions
 * @{
 */

/**
 * @addtogroup lavu_video_stereo3d
 * A stereoscopic video file consists in multiple views embedded in a single
 * frame, usually describing two views of a scene. This file describes all
 * possible codec-independent view arrangements.
 * */

/**
 * List of possible 3D Types
 */
enum AVStereo3DType {
    /**
     * Video is not stereoscopic (and metadata has to be there).
     */
    AV_STEREO3D_2D,

    /**
     * Views are next to each other.
     *
     * @code{.unparsed}
     *    LLLLRRRR
     *    LLLLRRRR
     *    LLLLRRRR
     *    ...
     * @endcode
     */
    AV_STEREO3D_SIDEBYSIDE,

    /**
     * Views are on top of each other.
     *
     * @code{.unparsed}
     *    LLLLLLLL
     *    LLLLLLLL
     *    RRRRRRRR
     *    RRRRRRRR
     * @endcode
     */
    AV_STEREO3D_TOPBOTTOM,

    /**
     * Views are alternated temporally.
     *
     * @code{.unparsed}
     *     frame0   frame1   frame2   ...
     *    LLLLLLLL RRRRRRRR LLLLLLLL
     *    LLLLLLLL RRRRRRRR LLLLLLLL
     *    LLLLLLLL RRRRRRRR LLLLLLLL
     *    ...      ...      ...
     * @endcode
     */
    AV_STEREO3D_FRAMESEQUENCE,

    /**
     * Views are packed in a checkerboard-like structure per pixel.
     *
     * @code{.unparsed}
     *    LRLRLRLR
     *    RLRLRLRL
     *    LRLRLRLR
     *    ...
     * @endcode
     */
    AV_STEREO3D_CHECKERBOARD,

    /**
     * Views are next to each other, but when upscaling
     * apply a checkerboard pattern.
     *
     * @code{.unparsed}
     *     LLLLRRRR          L L L L    R R R R
     *     LLLLRRRR    =>     L L L L  R R R R
     *     LLLLRRRR          L L L L    R R R R
     *     LLLLRRRR           L L L L  R R R R
     * @endcode
     */
    AV_STEREO3D_SIDEBYSIDE_QUINCUNX,

    /**
     * Views are packed per line, as if interlaced.
     *
     * @code{.unparsed}
     *    LLLLLLLL
     *    RRRRRRRR
     *    LLLLLLLL
     *    ...
     * @endcode
     */
    AV_STEREO3D_LINES,

    /**
     * Views are packed per column.
     *
     * @code{.unparsed}
     *    LRLRLRLR
     *    LRLRLRLR
     *    LRLRLRLR
     *    ...
     * @endcode
     */
    AV_STEREO3D_COLUMNS,
};

/**
 * List of possible view types.
 */
enum AVStereo3DView {
    /**
     * Frame contains two packed views.
     */
    AV_STEREO3D_VIEW_PACKED,

    /**
     * Frame contains only the left view.
     */
    AV_STEREO3D_VIEW_LEFT,

    /**
     * Frame contains only the right view.
     */
    AV_STEREO3D_VIEW_RIGHT,
};

/**
 * Inverted views, Right/Bottom represents the left view.
 */
#define AV_STEREO3D_FLAG_INVERT     (1 << 0)

/**
 * Stereo 3D type: this structure describes how two videos are packed
 * within a single video surface, with additional information as needed.
 *
 * @note The struct must be allocated with av_stereo3d_alloc() and
 *       its size is not a part of the public ABI.
 */
typedef struct AVStereo3D {
    /**
     * How views are packed within the video.
     */
    enum AVStereo3DType type;

    /**
     * Additional information about the frame packing.
     */
    int flags;

    /**
     * Determines which views are packed.
     */
    enum AVStereo3DView view;
} AVStereo3D;

/**
 * Allocate an AVStereo3D structure and set its fields to default values.
 * The resulting struct can be freed using av_freep().
 *
 * @return An AVStereo3D filled with default values or NULL on failure.
 */
AVStereo3D *av_stereo3d_alloc(void);

/**
 * Allocate a complete AVFrameSideData and add it to the frame.
 *
 * @param frame The frame which side data is added to.
 *
 * @return The AVStereo3D structure to be filled by caller.
 */
AVStereo3D *av_stereo3d_create_side_data(AVFrame *frame);

/**
 * Provide a human-readable name of a given stereo3d type.
 *
 * @param type The input stereo3d type value.
 *
 * @return The name of the stereo3d value, or "unknown".
 */
const char *av_stereo3d_type_name(unsigned int type);

/**
 * Get the AVStereo3DType form a human-readable name.
 *
 * @param name The input string.
 *
 * @return The AVStereo3DType value, or -1 if not found.
 */
int av_stereo3d_from_name(const char *name);

/**
 * @}
 * @}
 */

#endif /* AVUTIL_STEREO3D_H */
