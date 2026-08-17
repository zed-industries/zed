/*
** SGI FREE SOFTWARE LICENSE B (Version 2.0, Sept. 18, 2008)
** Copyright (C) [dates of first publication] Silicon Graphics, Inc.
** All Rights Reserved.
**
** Permission is hereby granted, free of charge, to any person obtaining a copy
** of this software and associated documentation files (the "Software"), to deal
** in the Software without restriction, including without limitation the rights
** to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
** of the Software, and to permit persons to whom the Software is furnished to do so,
** subject to the following conditions:
**
** The above copyright notice including the dates of first publication and either this
** permission notice or a reference to http://oss.sgi.com/projects/FreeB/ shall be
** included in all copies or substantial portions of the Software.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
** INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
** PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL SILICON GRAPHICS, INC.
** BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
** TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
** OR OTHER DEALINGS IN THE SOFTWARE.
**
** Except as contained in this notice, the name of Silicon Graphics, Inc. shall not
** be used in advertising or otherwise to promote the sale, use or other dealings in
** this Software without prior written authorization from Silicon Graphics, Inc.
*/
/*
** Author: Eric Veach, July 1994.
*/

#ifndef TESS_H
#define TESS_H

#include <setjmp.h>
#include "bucketalloc.h"
#include "mesh.h"
#include "dict.h"
#include "priorityq.h"
#include "../Include/tesselator.h"

#ifdef __cplusplus
extern "C" {
#endif

//typedef struct TESStesselator TESStesselator;

struct TESStesselator {

	/*** state needed for collecting the input data ***/
	TESSmesh	*mesh;		/* stores the input contours, and eventually
						the tessellation itself */
	int outOfMemory;

	/*** state needed for projecting onto the sweep plane ***/

	TESSreal normal[3];	/* user-specified normal (if provided) */
	TESSreal sUnit[3];	/* unit vector in s-direction (debugging) */
	TESSreal tUnit[3];	/* unit vector in t-direction (debugging) */

	TESSreal bmin[2];
	TESSreal bmax[2];

	int processCDT;	/* option to run Constrained Delayney pass. */
	int reverseContours; /* tessAddContour() will treat CCW contours as CW and vice versa */
    
	/*** state needed for the line sweep ***/
	int	windingRule;	/* rule for determining polygon interior */

	Dict *dict;		/* edge dictionary for sweep line */
	PriorityQ *pq;		/* priority queue of vertex events */
	TESSvertex *event;		/* current sweep event being processed */

	struct BucketAlloc* regionPool;

	TESSindex vertexIndexCounter;

	TESSreal *vertices;
	TESSindex *vertexIndices;
	int vertexCount;
	TESSindex *elements;
	int elementCount;

	TESSalloc alloc;

	jmp_buf env;			/* place to jump to when memAllocs fail */
};

#ifdef __cplusplus
};
#endif

#endif
