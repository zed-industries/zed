// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2002,2008, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CDTRGTST.H
*
* Modification History:
*        Name                     Description            
*     Madhu Katragadda            Converted to C
*********************************************************************************
*/
/* REGRESSION TEST FOR DATE FORMAT */
#ifndef _CDTFRRGSTST
#define _CDTFRRGSTST

#include "unicode/utypes.h"
#include "unicode/udat.h"

#if !UCONFIG_NO_FORMATTING

#include "cintltst.h"

    /**
     * DateFormat Regression tests
     **/

    void Test4029195(void); 
    void Test4056591(void); 
    void Test4059917(void);
    void Test4060212(void); 
    void Test4061287(void); 
    void Test4073003(void); 
    void Test4162071(void); 
    void Test714(void);
    void Test_GEec(void);

    /**
     * test subroutine
     **/
    void aux917(UDateFormat *fmt, UChar* str );

    /**
     * test subroutine used by the testing functions
     **/
    UChar* myFormatit(UDateFormat* datdef, UDate d1);

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
