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

#ifndef GEOM_H
#define GEOM_H

#include "mesh.h"

#ifdef NO_BRANCH_CONDITIONS
/* MIPS architecture has special instructions to evaluate boolean
* conditions -- more efficient than branching, IF you can get the
* compiler to generate the right instructions (SGI compiler doesn't)
*/
#define VertEq(u,v)	(((u)->s == (v)->s) & ((u)->t == (v)->t))
#define VertLeq(u,v)	(((u)->s < (v)->s) | \
	((u)->s == (v)->s & (u)->t <= (v)->t))
#else
#define VertEq(u,v) ((u)->s == (v)->s && (u)->t == (v)->t)
#define VertLeq(u,v) (((u)->s < (v)->s) || ((u)->s == (v)->s && (u)->t <= (v)->t))
#endif

#define EdgeEval(u,v,w)	tesedgeEval(u,v,w)
#define EdgeSign(u,v,w)	tesedgeSign(u,v,w)

/* Versions of VertLeq, EdgeSign, EdgeEval with s and t transposed. */

#define TransLeq(u,v) (((u)->t < (v)->t) || ((u)->t == (v)->t && (u)->s <= (v)->s))
#define TransEval(u,v,w) testransEval(u,v,w)
#define TransSign(u,v,w) testransSign(u,v,w)


#define EdgeGoesLeft(e) VertLeq( (e)->Dst, (e)->Org )
#define EdgeGoesRight(e) VertLeq( (e)->Org, (e)->Dst )
#define EdgeIsInternal(e) e->Rface && e->Rface->inside

#define ABS(x) ((x) < 0 ? -(x) : (x))
#define VertL1dist(u,v) (ABS(u->s - v->s) + ABS(u->t - v->t))

#define VertCCW(u,v,w) tesvertCCW(u,v,w)

int tesvertLeq( TESSvertex *u, TESSvertex *v );
TESSreal	tesedgeEval( TESSvertex *u, TESSvertex *v, TESSvertex *w );
TESSreal	tesedgeSign( TESSvertex *u, TESSvertex *v, TESSvertex *w );
TESSreal	testransEval( TESSvertex *u, TESSvertex *v, TESSvertex *w );
TESSreal	testransSign( TESSvertex *u, TESSvertex *v, TESSvertex *w );
int tesvertCCW( TESSvertex *u, TESSvertex *v, TESSvertex *w );
void tesedgeIntersect( TESSvertex *o1, TESSvertex *d1, TESSvertex *o2, TESSvertex *d2, TESSvertex *v );
int tesedgeIsLocallyDelaunay( TESShalfEdge *e );

#endif
