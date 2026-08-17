/*
 * Copyright © 2013 Keith Packard
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that copyright
 * notice and this permission notice appear in supporting documentation, and
 * that the name of the copyright holders not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  The copyright holders make no representations
 * about the suitability of this software for any purpose.  It is provided "as
 * is" without express or implied warranty.
 *
 * THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

#ifndef _PRESENT_TOKENS_H_
#define _PRESENT_TOKENS_H_

#define PRESENT_NAME			"Present"
#define PRESENT_MAJOR			1
#define PRESENT_MINOR			2

#define PresentNumberErrors		0
#define PresentNumberEvents		0

/* Requests */
#define X_PresentQueryVersion		0
#define X_PresentPixmap			1
#define X_PresentNotifyMSC		2
#define X_PresentSelectInput		3
#define X_PresentQueryCapabilities      4

#define PresentNumberRequests		5

/* Present operation options */
#define PresentOptionNone               0
#define PresentOptionAsync              (1 << 0)
#define PresentOptionCopy               (1 << 1)
#define PresentOptionUST                (1 << 2)
#define PresentOptionSuboptimal         (1 << 3)

#define PresentAllOptions       (PresentOptionAsync | \
                                 PresentOptionCopy | \
                                 PresentOptionUST | \
                                 PresentOptionSuboptimal)

/* Present capabilities */

#define PresentCapabilityNone           0
#define PresentCapabilityAsync          1
#define PresentCapabilityFence          2
#define PresentCapabilityUST            4

#define PresentAllCapabilities  (PresentCapabilityAsync | \
                                 PresentCapabilityFence | \
                                 PresentCapabilityUST)

/* Events */
#define PresentConfigureNotify	0
#define PresentCompleteNotify	1
#define PresentIdleNotify       2
#if PRESENT_FUTURE_VERSION
#define PresentRedirectNotify	3
#endif

/* Event Masks */
#define PresentConfigureNotifyMask      1
#define PresentCompleteNotifyMask       2
#define PresentIdleNotifyMask           4
#if PRESENT_FUTURE_VERSION
#define PresentRedirectNotifyMask       8
#endif

#if PRESENT_FUTURE_VERSION
#define PRESENT_REDIRECT_NOTIFY_MASK    PresentRedirectNotifyMask
#else
#define PRESENT_REDIRECT_NOTIFY_MASK    0
#endif

#define PresentAllEvents   (PresentConfigureNotifyMask |        \
                            PresentCompleteNotifyMask |         \
                            PresentIdleNotifyMask |             \
                            PRESENT_REDIRECT_NOTIFY_MASK)

/* Complete Kinds */

#define PresentCompleteKindPixmap       0
#define PresentCompleteKindNotifyMSC    1

/* Complete Modes */

#define PresentCompleteModeCopy           0
#define PresentCompleteModeFlip           1
#define PresentCompleteModeSkip           2
#define PresentCompleteModeSuboptimalCopy 3

#endif
