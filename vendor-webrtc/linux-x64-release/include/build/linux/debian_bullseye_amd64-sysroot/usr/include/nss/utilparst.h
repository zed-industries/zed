/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#ifndef UTILPARS_T_H
#define UTILPARS_T_H 1
#include "pkcs11t.h"

/*
 * macros to handle parsing strings of blank sparated arguments.
 * Several NSSUTIL_HANDLE_STRING() macros should be places one after another with no intervening
 * code. The first ones have precedence over the later ones. The last Macro should be
 * NSSUTIL_HANDLE_FINAL_ARG.
 *
 *  param is the input parameters. On exit param will point to the next parameter to parse. If the
 *      last paramter has been returned, param points to a null byte (*param = '0');
 *  target is the location to store any data aquired from the parameter. Caller is responsible to free this data.
 *  value is the string value of the parameter.
 *  command is any commands you need to run to help process the parameter's data.
 */
#define NSSUTIL_HANDLE_STRING_ARG(param, target, value, command)  \
    if (PORT_Strncasecmp(param, value, sizeof(value) - 1) == 0) { \
        param += sizeof(value) - 1;                               \
        if (target)                                               \
            PORT_Free(target);                                    \
        target = NSSUTIL_ArgFetchValue(param, &next);             \
        param += next;                                            \
        command;                                                  \
    } else

#define NSSUTIL_HANDLE_FINAL_ARG(param)          \
    {                                            \
        param = NSSUTIL_ArgSkipParameter(param); \
    }                                            \
    param = NSSUTIL_ArgStrip(param);

#define NSSUTIL_PATH_SEPARATOR "/"

/* default module configuration strings */
#define NSSUTIL_DEFAULT_INTERNAL_INIT1 \
    "library= name=\"NSS Internal PKCS #11 Module\" parameters="
#define NSSUTIL_DEFAULT_INTERNAL_INIT2 \
    " NSS=\"Flags=internal,critical trustOrder=75 cipherOrder=100 slotParams=(1={"
#define NSSUTIL_DEFAULT_INTERNAL_INIT3 \
    " askpw=any timeout=30})\""
#define NSSUTIL_DEFAULT_SFTKN_FLAGS \
    "slotFlags=[ECC,RSA,DSA,DH,RC2,RC4,DES,RANDOM,SHA1,MD5,MD2,SSL,TLS,AES,Camellia,SEED,SHA256,SHA512]"

#define NSSUTIL_DEFAULT_CIPHER_ORDER 0
#define NSSUTIL_DEFAULT_TRUST_ORDER 50
#define NSSUTIL_ARG_ESCAPE '\\'

/* hold slot default flags until we initialize a slot. This structure is only
 * useful between the time we define a module (either by hand or from the
 * database) and the time the module is loaded. Not reference counted  */
struct NSSUTILPreSlotInfoStr {
    CK_SLOT_ID slotID;          /* slot these flags are for */
    unsigned long defaultFlags; /* bit mask of default implementation this slot
                                 * provides */
    int askpw;                  /* slot specific password bits */
    long timeout;               /* slot specific timeout value */
    char hasRootCerts;          /* is this the root cert PKCS #11 module? */
    char hasRootTrust;          /* is this the root cert PKCS #11 module? */
    int reserved0[2];
    void *reserved1[2];
};

/*
 * private functions for softoken.
 */
typedef enum {
    NSS_DB_TYPE_NONE = 0,
    NSS_DB_TYPE_SQL,
    NSS_DB_TYPE_EXTERN,
    NSS_DB_TYPE_LEGACY,
    NSS_DB_TYPE_MULTIACCESS
} NSSDBType;

#endif /* UTILPARS_T_H */
