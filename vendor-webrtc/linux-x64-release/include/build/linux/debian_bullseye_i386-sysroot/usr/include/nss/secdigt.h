/*
 * secdigt.h - public data structures for digest-info objects
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _SECDIGT_H_
#define _SECDIGT_H_

#include "utilrename.h"
#include "plarena.h"
#include "secoidt.h"
#include "secitem.h"

/*
** A PKCS#1 digest-info object
*/
struct SGNDigestInfoStr {
    PLArenaPool* arena;
    SECAlgorithmID digestAlgorithm;
    SECItem digest;
};
typedef struct SGNDigestInfoStr SGNDigestInfo;

#endif /* _SECDIGT_H_ */
