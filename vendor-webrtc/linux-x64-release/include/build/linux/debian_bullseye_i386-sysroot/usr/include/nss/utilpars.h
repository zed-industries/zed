/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _UTILPARS_H_
#define _UTILPARS_H_ 1

#include "utilparst.h"
#include "plarena.h"

/* handle a module db request */
char **NSSUTIL_DoModuleDBFunction(unsigned long function, char *parameters, void *args);

/* parsing functions */
char *NSSUTIL_ArgFetchValue(const char *string, int *pcount);
const char *NSSUTIL_ArgStrip(const char *c);
char *NSSUTIL_ArgGetParamValue(const char *paramName, const char *parameters);
const char *NSSUTIL_ArgSkipParameter(const char *string);
char *NSSUTIL_ArgGetLabel(const char *inString, int *next);
long NSSUTIL_ArgDecodeNumber(const char *num);
PRBool NSSUTIL_ArgIsBlank(char c);
PRBool NSSUTIL_ArgHasFlag(const char *label, const char *flag,
                          const char *parameters);
long NSSUTIL_ArgReadLong(const char *label, const char *params, long defValue,
                         PRBool *isdefault);

/* quoting functions */
int NSSUTIL_EscapeSize(const char *string, char quote);
char *NSSUTIL_Escape(const char *string, char quote);
int NSSUTIL_QuoteSize(const char *string, char quote);
char *NSSUTIL_Quote(const char *string, char quote);
int NSSUTIL_DoubleEscapeSize(const char *string, char quote1, char quote2);
char *NSSUTIL_DoubleEscape(const char *string, char quote1, char quote2);

unsigned long NSSUTIL_ArgParseSlotFlags(const char *label, const char *params);
struct NSSUTILPreSlotInfoStr *NSSUTIL_ArgParseSlotInfo(PLArenaPool *arena,
                                                       const char *slotParams, int *retCount);
char *NSSUTIL_MkSlotString(unsigned long slotID, unsigned long defaultFlags,
                           unsigned long timeout, unsigned char askpw_in,
                           PRBool hasRootCerts, PRBool hasRootTrust);
SECStatus NSSUTIL_ArgParseModuleSpec(const char *modulespec, char **lib,
                                     char **mod, char **parameters, char **nss);
SECStatus NSSUTIL_ArgParseModuleSpecEx(const char *modulespec, char **lib,
                                       char **mod, char **parameters, char **nss, char **config);
char *NSSUTIL_MkModuleSpec(char *dllName, char *commonName,
                           char *parameters, char *NSS);
char *NSSUTIL_MkModuleSpecEx(char *dllName, char *commonName,
                             char *parameters, char *NSS, char *config);
char *NSSUTIL_AddNSSFlagToModuleSpec(char *spec, char *addFlag);
void NSSUTIL_ArgParseCipherFlags(unsigned long *newCiphers,
                                 const char *cipherList);
char *NSSUTIL_MkNSSString(char **slotStrings, int slotCount, PRBool internal,
                          PRBool isFIPS, PRBool isModuleDB, PRBool isModuleDBOnly,
                          PRBool isCritical, unsigned long trustOrder,
                          unsigned long cipherOrder, unsigned long ssl0, unsigned long ssl1);

/*
 * private functions for softoken.
 */
char *_NSSUTIL_GetSecmodName(const char *param, NSSDBType *dbType,
                             char **appName, char **filename, PRBool *rw);
const char *_NSSUTIL_EvaluateConfigDir(const char *configdir, NSSDBType *dbType, char **app);
#if defined(_WIN32)
wchar_t *_NSSUTIL_UTF8ToWide(const char *buf);
PRStatus _NSSUTIL_Access(const char *path, PRAccessHow how);
#else
#define _NSSUTIL_Access(path, how) PR_Access((path), (how))
#endif

#endif /* _UTILPARS_H_ */
