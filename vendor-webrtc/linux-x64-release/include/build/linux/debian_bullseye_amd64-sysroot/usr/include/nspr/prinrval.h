/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** File:        prinrval.h
** Description: API to interval timing functions of NSPR.
**
**
** NSPR provides interval times that are independent of network time
** of day values. Interval times are (in theory) accurate regardless
** of host processing requirements and also very cheap to acquire. It
** is expected that getting an interval time while in a synchronized
** function (holding one's lock).
**/

#if !defined(prinrval_h)
#define prinrval_h

#include "prtypes.h"

PR_BEGIN_EXTERN_C

/**********************************************************************/
/************************* TYPES AND CONSTANTS ************************/
/**********************************************************************/

typedef PRUint32 PRIntervalTime;

/***********************************************************************
** DEFINES:     PR_INTERVAL_MIN
**              PR_INTERVAL_MAX
** DESCRIPTION:
**  These two constants define the range (in ticks / second) of the
**  platform dependent type, PRIntervalTime. These constants bound both
**  the period and the resolution of a PRIntervalTime.
***********************************************************************/
#define PR_INTERVAL_MIN 1000UL
#define PR_INTERVAL_MAX 100000UL

/***********************************************************************
** DEFINES:     PR_INTERVAL_NO_WAIT
**              PR_INTERVAL_NO_TIMEOUT
** DESCRIPTION:
**  Two reserved constants are defined in the PRIntervalTime namespace.
**  They are used to indicate that the process should wait no time (return
**  immediately) or wait forever (never time out), respectively.
**  Note: PR_INTERVAL_NO_TIMEOUT passed as input to PR_Connect is
**  interpreted as use the OS's connect timeout.
**
***********************************************************************/
#define PR_INTERVAL_NO_WAIT 0UL
#define PR_INTERVAL_NO_TIMEOUT 0xffffffffUL

/**********************************************************************/
/****************************** FUNCTIONS *****************************/
/**********************************************************************/

/***********************************************************************
** FUNCTION:    PR_IntervalNow
** DESCRIPTION:
**  Return the value of NSPR's free running interval timer. That timer
**  can be used to establish epochs and determine intervals (be computing
**  the difference between two times).
** INPUTS:      void
** OUTPUTS:     void
** RETURN:      PRIntervalTime
**
** SIDE EFFECTS:
**  None
** RESTRICTIONS:
**  The units of PRIntervalTime are platform dependent. They are chosen
**  such that they are appropriate for the host OS, yet provide sufficient
**  resolution and period to be useful to clients.
** MEMORY:      N/A
** ALGORITHM:   Platform dependent
***********************************************************************/
NSPR_API(PRIntervalTime) PR_IntervalNow(void);

/***********************************************************************
** FUNCTION:    PR_TicksPerSecond
** DESCRIPTION:
**  Return the number of ticks per second for PR_IntervalNow's clock.
**  The value will be in the range [PR_INTERVAL_MIN..PR_INTERVAL_MAX].
** INPUTS:      void
** OUTPUTS:     void
** RETURN:      PRUint32
**
** SIDE EFFECTS:
**  None
** RESTRICTIONS:
**  None
** MEMORY:      N/A
** ALGORITHM:   N/A
***********************************************************************/
NSPR_API(PRUint32) PR_TicksPerSecond(void);

/***********************************************************************
** FUNCTION:    PR_SecondsToInterval
**              PR_MillisecondsToInterval
**              PR_MicrosecondsToInterval
** DESCRIPTION:
**  Convert standard clock units to platform dependent intervals.
** INPUTS:      PRUint32
** OUTPUTS:     void
** RETURN:      PRIntervalTime
**
** SIDE EFFECTS:
**  None
** RESTRICTIONS:
**  Conversion may cause overflow, which is not reported.
** MEMORY:      N/A
** ALGORITHM:   N/A
***********************************************************************/
NSPR_API(PRIntervalTime) PR_SecondsToInterval(PRUint32 seconds);
NSPR_API(PRIntervalTime) PR_MillisecondsToInterval(PRUint32 milli);
NSPR_API(PRIntervalTime) PR_MicrosecondsToInterval(PRUint32 micro);

/***********************************************************************
** FUNCTION:    PR_IntervalToSeconds
**              PR_IntervalToMilliseconds
**              PR_IntervalToMicroseconds
** DESCRIPTION:
**  Convert platform dependent intervals to standard clock units.
** INPUTS:      PRIntervalTime
** OUTPUTS:     void
** RETURN:      PRUint32
**
** SIDE EFFECTS:
**  None
** RESTRICTIONS:
**  Conversion may cause overflow, which is not reported.
** MEMORY:      N/A
** ALGORITHM:   N/A
***********************************************************************/
NSPR_API(PRUint32) PR_IntervalToSeconds(PRIntervalTime ticks);
NSPR_API(PRUint32) PR_IntervalToMilliseconds(PRIntervalTime ticks);
NSPR_API(PRUint32) PR_IntervalToMicroseconds(PRIntervalTime ticks);

PR_END_EXTERN_C


#endif /* !defined(prinrval_h) */

/* prinrval.h */
