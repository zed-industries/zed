/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef NSSCKBI_H
#define NSSCKBI_H

/*
 * NSS BUILTINS Version numbers.
 *
 * These are the version numbers for the builtins module packaged with
 * this release on NSS. To determine the version numbers of the builtin
 * module you are using, use the appropriate PKCS #11 calls.
 *
 * These version numbers detail changes to the PKCS #11 interface. They map
 * to the PKCS #11 spec versions.
 */
#define NSS_BUILTINS_CRYPTOKI_VERSION_MAJOR 2
#define NSS_BUILTINS_CRYPTOKI_VERSION_MINOR 20

/* These version numbers detail the changes
 * to the list of trusted certificates.
 *
 * The NSS_BUILTINS_LIBRARY_VERSION_MINOR macro needs to be bumped
 * whenever we change the list of trusted certificates.
 *
 * Please use the following rules when increasing the version number:
 *
 * - starting with version 2.14, NSS_BUILTINS_LIBRARY_VERSION_MINOR
 *   must always be an EVEN number (e.g. 16, 18, 20 etc.)
 *
 * - whenever possible, if older branches require a modification to the
 *   list, these changes should be made on the main line of development (trunk),
 *   and the older branches should update to the most recent list.
 *
 * - ODD minor version numbers are reserved to indicate a snapshot that has
 *   deviated from the main line of development, e.g. if it was necessary
 *   to modify the list on a stable branch.
 *   Once the version has been changed to an odd number (e.g. 2.13) on a branch,
 *   it should remain unchanged on that branch, even if further changes are
 *   made on that branch.
 *
 * NSS_BUILTINS_LIBRARY_VERSION_MINOR is a CK_BYTE.  It's not clear
 * whether we may use its full range (0-255) or only 0-99 because
 * of the comment in the CK_VERSION type definition.
 * It's recommend to switch back to 0 after having reached version 98/99.
 */
#define NSS_BUILTINS_LIBRARY_VERSION_MAJOR 2
#define NSS_BUILTINS_LIBRARY_VERSION_MINOR 46
#define NSS_BUILTINS_LIBRARY_VERSION "2.46"

/* These version numbers detail the semantic changes to the ckfw engine. */
#define NSS_BUILTINS_HARDWARE_VERSION_MAJOR 1
#define NSS_BUILTINS_HARDWARE_VERSION_MINOR 0

/* These version numbers detail the semantic changes to ckbi itself
 * (new PKCS #11 objects), etc. */
#define NSS_BUILTINS_FIRMWARE_VERSION_MAJOR 1
#define NSS_BUILTINS_FIRMWARE_VERSION_MINOR 0

#endif /* NSSCKBI_H */
