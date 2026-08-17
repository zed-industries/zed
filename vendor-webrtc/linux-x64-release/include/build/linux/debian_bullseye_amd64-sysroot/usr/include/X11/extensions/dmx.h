/*
 * Copyright 2002-2004 Red Hat Inc., Durham, North Carolina.
 *
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation on the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial
 * portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NON-INFRINGEMENT.  IN NO EVENT SHALL RED HAT AND/OR THEIR SUPPLIERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

/*
 * Authors:
 *   Rickard E. (Rik) Faith <faith@redhat.com>
 *
 */

/** \file
 * This file describes the interface to the client-side libdmx.a
 * library.  All DMX-aware client-side applications should include this
 * file. */

#ifndef _DMX_H_
#define _DMX_H_

/* These values must be larger than LastExtensionError.
   The values in dmxext.h and dmxproto.h *MUST* match. */
#define DmxBadXinerama         1001
#define DmxBadValue            1002
#define DmxBadReply            1003

#define DMXScreenWindowWidth   (1L<<0)
#define DMXScreenWindowHeight  (1L<<1)
#define DMXScreenWindowXoffset (1L<<2)
#define DMXScreenWindowYoffset (1L<<3)
#define DMXRootWindowWidth     (1L<<4)
#define DMXRootWindowHeight    (1L<<5)
#define DMXRootWindowXoffset   (1L<<6)
#define DMXRootWindowYoffset   (1L<<7)
#define DMXRootWindowXorigin   (1L<<8)
#define DMXRootWindowYorigin   (1L<<9)

#define DMXDesktopWidth        (1L<<0)
#define DMXDesktopHeight       (1L<<1)
#define DMXDesktopShiftX       (1L<<2)
#define DMXDesktopShiftY       (1L<<3)

#define DMXInputType           (1L<<0)
#define DMXInputPhysicalScreen (1L<<1)
#define DMXInputSendsCore      (1L<<2)

#endif
