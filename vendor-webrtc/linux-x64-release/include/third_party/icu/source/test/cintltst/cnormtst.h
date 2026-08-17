// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2001, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CNORMTST.H
*
* Modification History:
*        Name                     Description            
*     Madhu Katragadda            Converted to C
*     synwee                      added test for quick check
*     synwee                      added test for checkFCD
*********************************************************************************
*/
#ifndef _NORMTST
#define _NORMTST
/**
 *  tests for u_normalization
 */

#include "cintltst.h"
    
    void TestDecomp(void);
    void TestCompatDecomp(void);
    void TestCanonDecompCompose(void);
    void TestCompatDecompCompose(void);
    void TestNull(void);
    void TestQuickCheck(void);
    void TestCheckFCD(void);

    /*internal functions*/
    
/*    static void assertEqual(const UChar* result,  const UChar* expected, int32_t index);
*/
    static void assertEqual(const UChar* result,  const char* expected, int32_t index);


    


#endif
