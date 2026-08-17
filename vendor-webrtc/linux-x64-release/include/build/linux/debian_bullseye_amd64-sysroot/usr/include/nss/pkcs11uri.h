/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _PKCS11URI_H_
#define _PKCS11URI_H_ 1

#include "seccomon.h"

/* Path attributes defined in RFC7512. */
#define PK11URI_PATTR_TOKEN "token"
#define PK11URI_PATTR_MANUFACTURER "manufacturer"
#define PK11URI_PATTR_SERIAL "serial"
#define PK11URI_PATTR_MODEL "model"
#define PK11URI_PATTR_LIBRARY_MANUFACTURER "library-manufacturer"
#define PK11URI_PATTR_LIBRARY_DESCRIPTION "library-description"
#define PK11URI_PATTR_LIBRARY_VERSION "library-version"
#define PK11URI_PATTR_OBJECT "object"
#define PK11URI_PATTR_TYPE "type"
#define PK11URI_PATTR_ID "id"
#define PK11URI_PATTR_SLOT_MANUFACTURER "slot-manufacturer"
#define PK11URI_PATTR_SLOT_DESCRIPTION "slot-description"
#define PK11URI_PATTR_SLOT_ID "slot-id"

/* Query attributes defined in RFC7512. */
#define PK11URI_QATTR_PIN_SOURCE "pin-source"
#define PK11URI_QATTR_PIN_VALUE "pin-value"
#define PK11URI_QATTR_MODULE_NAME "module-name"
#define PK11URI_QATTR_MODULE_PATH "module-path"

SEC_BEGIN_PROTOS

/* A PK11URI object is an immutable structure that holds path and
 * query attributes of a PKCS#11 URI. */
struct PK11URIStr;
typedef struct PK11URIStr PK11URI;

struct PK11URIAttributeStr {
    const char *name;
    const char *value;
};
typedef struct PK11URIAttributeStr PK11URIAttribute;

/* Create a new PK11URI object from a set of attributes. */
extern PK11URI *PK11URI_CreateURI(const PK11URIAttribute *pattrs,
                                  size_t num_pattrs,
                                  const PK11URIAttribute *qattrs,
                                  size_t num_qattrs);

/* Parse PKCS#11 URI and return a new PK11URI object. */
extern PK11URI *PK11URI_ParseURI(const char *string);

/* Format a PK11URI object to a string. */
extern char *PK11URI_FormatURI(PLArenaPool *arena, PK11URI *uri);

/* Destroy a PK11URI object. */
extern void PK11URI_DestroyURI(PK11URI *uri);

/* Retrieve a path attribute with the given name. */
extern const char *PK11URI_GetPathAttribute(PK11URI *uri, const char *name);

/* Retrieve a query attribute with the given name. */
extern const char *PK11URI_GetQueryAttribute(PK11URI *uri, const char *name);

SEC_END_PROTOS

#endif /* _PKCS11URI_H_ */
