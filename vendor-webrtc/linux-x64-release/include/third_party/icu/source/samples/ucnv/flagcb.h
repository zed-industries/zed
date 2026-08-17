/* © 2016 and later: Unicode, Inc. and others.
   License & terms of use: http://www.unicode.org/copyright.html

   Copyright (c) 2000 IBM, Inc. and Others. 
   FLAGCB.H - interface to 'flagging' callback which 
   simply marks the fact that the callback was called. 
*/

#ifndef _FLAGCB
#define _FLAGCB

#include "unicode/utypes.h"
#include "unicode/ucnv.h"

/* The structure of a FromU Flag context. 
   (conceivably there could be a ToU Flag Context) */

typedef struct
{
  UConverterFromUCallback  subCallback;
  const void               *subContext;
  UBool                    flag;
} FromUFLAGContext;

/**
 * open the context 
 */

U_CAPI FromUFLAGContext* U_EXPORT2  flagCB_fromU_openContext();

/**
 * the actual callback 
 */
U_CAPI void U_EXPORT2 flagCB_fromU(
                  const void *context,
                  UConverterFromUnicodeArgs *fromUArgs,
                  const UChar* codeUnits,
                  int32_t length,
                  UChar32 codePoint,
                  UConverterCallbackReason reason,
				  UErrorCode * err);



typedef struct
{
    UConverterFromUCallback  subCallback;
    const void               *subContext;
    uint32_t       magic;      /* 0xC0FFEE to identify that the object is OK */
    uint32_t       serial;     /* minted from nextSerial */
} debugCBContext;

U_CAPI void debugCB_fromU(const void *context,
                   UConverterFromUnicodeArgs *fromUArgs,
                   const UChar* codeUnits,
                   int32_t length,
                   UChar32 codePoint,
                   UConverterCallbackReason reason,
                   UErrorCode * err);

U_CAPI debugCBContext *debugCB_openContext();

#endif
