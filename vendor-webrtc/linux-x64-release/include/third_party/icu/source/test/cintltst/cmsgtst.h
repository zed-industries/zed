// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2010, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************
 *
 * File CMSGTST.H
 *
 * Modification History:
 *        Name                     Description            
 *     Madhu Katragadda              Creation
 ********************************************************************/
/* C API TEST FOR MESSAGE FORMAT */
#ifndef _CMSGFRMTST
#define _CMSGFRMTST

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "cintltst.h"


/* The function used to test the Message format API*/

    /**
     * Test u_formatMessage() with various test patterns
     **/
    static void MessageFormatTest(void);
    /**
     * Test u_formatMessage() with sample test Patterns 
     **/
    static void TestSampleMessageFormat(void);
    /**
     * Test format and parse sequence and roundtrip
     **/
    static void TestSampleFormatAndParse(void);
    /**
     * Test u_formatMessage() with choice option
     **/
    static void TestMsgFormatChoice(void);
    /**
     * Test u_formatMessage() with Select option
     **/
    static void TestMsgFormatSelect(void);
    /**
     * Test u_parseMessage() with various test patterns()
     **/
    static void TestParseMessage(void);
    /**
     * function used to set up various patterns used for testing u_formatMessage()
     **/
    static void InitStrings( void );

    /**
     * Regression test for ICU4C Jitterbug 904
     */
    static void TestJ904(void);

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
