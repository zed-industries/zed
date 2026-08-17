/*
 * Copyright 1992 Network Computing Devices
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of NCD. not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  NCD. makes no representations about the
 * suitability of this software for any purpose.  It is provided "as is"
 * without express or implied warranty.
 *
 * NCD. DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING ALL
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL NCD.
 * BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
 * OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
 * CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 */

#ifndef _LBX_H_
#define _LBX_H_

#define LBXNAME "LBX"

#define LBX_MAJOR_VERSION	1
#define LBX_MINOR_VERSION	0

#define LbxNumberReqs			44
#define LbxEvent			0
#define LbxQuickMotionDeltaEvent	1
#define LbxNumberEvents			2

/* This is always the master client */
#define LbxMasterClientIndex		0

/* LbxEvent lbxType sub-fields */
#define LbxSwitchEvent			0
#define LbxCloseEvent			1
#define LbxDeltaEvent			2
#define LbxInvalidateTagEvent		3
#define LbxSendTagDataEvent		4
#define LbxListenToOne			5
#define LbxListenToAll			6
#define LbxMotionDeltaEvent		7
#define LbxReleaseCmapEvent		8
#define LbxFreeCellsEvent		9

/*
 * Lbx image compression methods
 *
 * No compression is always assigned the value of 0.
 *
 * The rest of the compression method opcodes are assigned dynamically
 * at option negotiation time.
 */

#define LbxImageCompressNone		0


#define BadLbxClient			0
#define LbxNumberErrors			(BadLbxClient + 1)

/* tagged data types */
#define	LbxTagTypeModmap		1
#define	LbxTagTypeKeymap		2
#define	LbxTagTypeProperty		3
#define	LbxTagTypeFont			4
#define	LbxTagTypeConnInfo		5

#endif
