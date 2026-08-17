// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
**********************************************************************
* Copyright (c) 2004-2011, International Business Machines
* Corporation and others.  All Rights Reserved.
**********************************************************************
* Author: Alan Liu
* Created: March 19 2004
* Since: ICU 3.0
**********************************************************************
*/
#ifndef __ICU_INTLTEST_TEXTFILE__
#define __ICU_INTLTEST_TEXTFILE__

#include "intltest.h"
#include "filestrm.h"

/**
 * This class implements access to a text data file located in the
 * icu/source/test/testdata/ directory.
 */
class TextFile {
 public:
    /**
     * Open a file with the given name, in the given encoding, in the
     * ICU testdata directory. See textfile.cpp to determine if the
     * 'name' and 'encoding' parameters are aliased or copied.
     */
    TextFile(const char* name, const char* encoding, UErrorCode& ec);

    virtual ~TextFile();

    /**
     * Read a line terminated by ^J or ^M or ^M^J, and convert it from
     * this file's encoding to Unicode. The EOL character(s) are not
     * included in 'line'.
     * @return true if a line was read, or false if the EOF
     * was reached or an error occurred
     */
    UBool readLine(UnicodeString& line, UErrorCode& ec);

    /**
     * Read a line, ignoring blank lines and lines that start with
     * '#'.  Trim leading white space.
     * @param trim if true then remove leading Pattern_White_Space
     * @return true if a line was read, or false if the EOF
     * was reached or an error occurred
     */
    UBool readLineSkippingComments(UnicodeString& line, UErrorCode& ec,
                                   UBool trim = false);

    /**
     * Return the line number of the last line returned by readLine().
     */
    inline int32_t getLineNumber() const;

 private:
    UBool ensureCapacity(int32_t capacity);
    UBool setBuffer(int32_t index, char c, UErrorCode& ec);

    FileStream* file;
    char* name;
    char* encoding;
    char* buffer;
    int32_t capacity;
    int32_t lineNo;
};

inline int32_t TextFile::getLineNumber() const {
    return lineNo;
}

#endif
