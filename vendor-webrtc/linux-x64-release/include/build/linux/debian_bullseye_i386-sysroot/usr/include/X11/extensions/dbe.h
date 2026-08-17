/******************************************************************************
 *
 * Copyright (c) 1994, 1995  Hewlett-Packard Company
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL HEWLETT-PACKARD COMPANY BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR
 * THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * Except as contained in this notice, the name of the Hewlett-Packard
 * Company shall not be used in advertising or otherwise to promote the
 * sale, use or other dealings in this Software without prior written
 * authorization from the Hewlett-Packard Company.
 *
 *     Header file for Xlib-related DBE
 *
 *****************************************************************************/

#ifndef DBE_H
#define DBE_H

/* Values for swap_action field of XdbeSwapInfo structure */
#define XdbeUndefined    0
#define XdbeBackground   1
#define XdbeUntouched    2
#define XdbeCopied       3

/* Errors */
#define XdbeBadBuffer    0

#define DBE_PROTOCOL_NAME "DOUBLE-BUFFER"

/* Current version numbers */
#define DBE_MAJOR_VERSION       1
#define DBE_MINOR_VERSION       0

/* Used when adding extension; also used in Xdbe macros */
#define DbeNumberEvents			0
#define DbeBadBuffer			0
#define DbeNumberErrors			(DbeBadBuffer + 1)

#endif /* DBE_H */

