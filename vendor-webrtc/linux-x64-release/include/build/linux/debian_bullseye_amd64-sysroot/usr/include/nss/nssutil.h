/*
 * NSS utility functions
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef __nssutil_h_
#define __nssutil_h_

#ifndef RC_INVOKED
#include "seccomon.h"
#endif

/*
 * NSS utilities's major version, minor version, patch level, build number,
 * and whether this is a beta release.
 *
 * The format of the version string should be
 *     "<major version>.<minor version>[.<patch level>[.<build number>]][ <Beta>]"
 */
#define NSSUTIL_VERSION "3.61"
#define NSSUTIL_VMAJOR 3
#define NSSUTIL_VMINOR 61
#define NSSUTIL_VPATCH 0
#define NSSUTIL_VBUILD 0
#define NSSUTIL_BETA PR_FALSE

SEC_BEGIN_PROTOS

/*
 * Returns a const string of the UTIL library version.
 */
extern const char *NSSUTIL_GetVersion(void);

extern SECStatus
NSS_InitializePRErrorTable(void);

SEC_END_PROTOS

#endif /* __nssutil_h_ */
