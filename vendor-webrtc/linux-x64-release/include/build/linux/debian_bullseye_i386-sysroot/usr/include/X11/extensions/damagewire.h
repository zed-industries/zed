/*
 * Copyright © 2003 Keith Packard
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of Keith Packard not be used in
 * advertising or publicity pertaining to distribution of the software without
 * specific, written prior permission.  Keith Packard makes no
 * representations about the suitability of this software for any purpose.  It
 * is provided "as is" without express or implied warranty.
 *
 * KEITH PACKARD DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL KEITH PACKARD BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */

#ifndef _DAMAGEWIRE_H_
#define _DAMAGEWIRE_H_

#define	DAMAGE_NAME	"DAMAGE"
#define DAMAGE_MAJOR	1
#define DAMAGE_MINOR	1

/************* Version 1 ****************/

/* Constants */
#define XDamageReportRawRectangles	0
#define XDamageReportDeltaRectangles	1
#define XDamageReportBoundingBox	2
#define XDamageReportNonEmpty		3

/* Requests */
#define X_DamageQueryVersion		0
#define X_DamageCreate			1
#define X_DamageDestroy			2
#define X_DamageSubtract		3
#define X_DamageAdd			4

#define XDamageNumberRequests		(X_DamageAdd + 1)

/* Events */
#define XDamageNotify			0

#define XDamageNumberEvents		(XDamageNotify + 1)

/* Errors */
#define BadDamage			0
#define XDamageNumberErrors		(BadDamage + 1)

#endif /* _DAMAGEWIRE_H_ */
