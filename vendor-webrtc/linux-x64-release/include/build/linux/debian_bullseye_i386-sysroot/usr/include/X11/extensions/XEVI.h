/************************************************************
Copyright (c) 1997 by Silicon Graphics Computer Systems, Inc.
Permission to use, copy, modify, and distribute this
software and its documentation for any purpose and without
fee is hereby granted, provided that the above copyright
notice appear in all copies and that both that copyright
notice and this permission notice appear in supporting
documentation, and that the name of Silicon Graphics not be
used in advertising or publicity pertaining to distribution
of the software without specific prior written permission.
Silicon Graphics makes no representation about the suitability
of this software for any purpose. It is provided "as is"
without any express or implied warranty.
SILICON GRAPHICS DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS FOR A PARTICULAR PURPOSE. IN NO EVENT SHALL SILICON
GRAPHICS BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL
DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE
OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION  WITH
THE USE OR PERFORMANCE OF THIS SOFTWARE.
********************************************************/

#ifndef _XEVI_H_
#define _XEVI_H_
#include <X11/Xfuncproto.h>
#include <X11/extensions/EVI.h>

typedef struct {
    VisualID		core_visual_id;
    int			screen;
    int			level;
    unsigned int	transparency_type;
    unsigned int	transparency_value;
    unsigned int	min_hw_colormaps;
    unsigned int	max_hw_colormaps;
    unsigned int	num_colormap_conflicts;
    VisualID*		colormap_conflicts;
} ExtendedVisualInfo;

_XFUNCPROTOBEGIN

Bool XeviQueryExtension(
    Display*            /* dpy */
);
Status XeviQueryVersion(
    Display*		/* dpy */,
    int*		/* majorVersion */,
    int*		/* minorVersion */
);
Status XeviGetVisualInfo(
    Display*		 	/* dpy */,
    VisualID*			/* visual_query */,
    int				/* nVisual_query */,
    ExtendedVisualInfo**	/* extendedVisualInfo_return */,
    int*			/* nInfo_return */
);

_XFUNCPROTOEND

#endif
