/*
 * Copyright © 2008 Red Hat, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Soft-
 * ware"), to deal in the Software without restriction, including without
 * limitation the rights to use, copy, modify, merge, publish, distribute,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, provided that the above copyright
 * notice(s) and this permission notice appear in all copies of the Soft-
 * ware and that both the above copyright notice(s) and this permission
 * notice appear in supporting documentation.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABIL-
 * ITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY
 * RIGHTS. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN
 * THIS NOTICE BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSE-
 * QUENTIAL DAMAGES, OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFOR-
 * MANCE OF THIS SOFTWARE.
 *
 * Except as contained in this notice, the name of a copyright holder shall
 * not be used in advertising or otherwise to promote the sale, use or
 * other dealings in this Software without prior written authorization of
 * the copyright holder.
 *
 * Authors:
 *   Kristian Høgsberg (krh@redhat.com)
 */

#ifndef _DRI2_TOKENS_H_
#define _DRI2_TOKENS_H_

#define DRI2BufferFrontLeft		0
#define DRI2BufferBackLeft		1
#define DRI2BufferFrontRight		2
#define DRI2BufferBackRight		3
#define DRI2BufferDepth			4
#define DRI2BufferStencil		5
#define DRI2BufferAccum			6
#define DRI2BufferFakeFrontLeft		7
#define DRI2BufferFakeFrontRight	8
#define DRI2BufferDepthStencil		9
#define DRI2BufferHiz			10

/* keep bits 16 and above for prime IDs */
#define DRI2DriverPrimeMask             7 /* 0 - 7 - allows for 6 devices*/
#define DRI2DriverPrimeShift           16
#define DRI2DriverPrimeId(x)         (((x) >> DRI2DriverPrimeShift) & (DRI2DriverPrimeMask))

#define DRI2DriverDRI			0
#define DRI2DriverVDPAU			1

/* Event sub-types for the swap complete event */
#define DRI2_EXCHANGE_COMPLETE		0x1
#define DRI2_BLIT_COMPLETE		0x2
#define DRI2_FLIP_COMPLETE		0x3

#endif
