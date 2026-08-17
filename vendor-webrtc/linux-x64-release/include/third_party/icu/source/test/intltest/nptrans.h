// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
 *******************************************************************************
 *
 *   Copyright (C) 2003-2011, International Business Machines
 *   Corporation and others.  All Rights Reserved.
 *
 *******************************************************************************
 *   file name:  nptrans.h
 *   encoding:   UTF-8
 *   tab size:   8 (not used)
 *   indentation:4
 *
 *   created on: 2003feb1
 *   created by: Ram Viswanadha
 */

#ifndef NPTRANS_H
#define NPTRANS_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_IDNA
#if !UCONFIG_NO_TRANSLITERATION

#include "unicode/uniset.h"
#include "unicode/ures.h"
#include "unicode/translit.h"

#include "intltest.h"


#define ASCII_SPACE 0x0020

class NamePrepTransform {

private :
    Transliterator *mapping;
    UnicodeSet unassigned;
    UnicodeSet prohibited;
    UnicodeSet labelSeparatorSet;
    UResourceBundle *bundle;
    NamePrepTransform(UParseError& parseError, UErrorCode& status);


public :

    static NamePrepTransform* createInstance(UParseError& parseError, UErrorCode& status);

    virtual ~NamePrepTransform();


    inline UBool isProhibited(UChar32 ch);

    /**
     * ICU "poor man's RTTI", returns a UClassID for the actual class.
     */
    inline UClassID getDynamicClassID() const { return getStaticClassID(); }

    /**
     * ICU "poor man's RTTI", returns a UClassID for this class.
     */
    static inline UClassID getStaticClassID() { return (UClassID)&fgClassID; }

    /**
     * Map every character in input stream with mapping character 
     * in the mapping table and populate the output stream.
     * For any individual character the mapping table may specify 
     * that that a character be mapped to nothing, mapped to one 
     * other character or to a string of other characters.
     *
     * @param src           Pointer to char16_t buffer containing a single label
     * @param srcLength     Number of characters in the source label
     * @param dest          Pointer to the destination buffer to receive the output
     * @param destCapacity  The capacity of destination array
     * @param allowUnassigned   Unassigned values can be converted to ASCII for query operations
     *                          If true unassigned values are treated as normal Unicode code point.
     *                          If false the operation fails with U_UNASSIGNED_CODE_POINT_FOUND error code.
     * @param status        ICU error code in/out parameter.
     *                      Must fulfill U_SUCCESS before the function call.
     * @return The number of UChars in the destination buffer
     */
    int32_t map(const char16_t* src, int32_t srcLength,
                        char16_t* dest, int32_t destCapacity,
                        UBool allowUnassigned,
                        UParseError* parseError,
                        UErrorCode& status );

    /**
     * Prepare the input stream with for use. This operation maps, normalizes(NFKC),
     * checks for prohited and BiDi characters in the order defined by RFC 3454
     * 
     * @param src           Pointer to char16_t buffer containing a single label
     * @param srcLength     Number of characters in the source label
     * @param dest          Pointer to the destination buffer to receive the output
     * @param destCapacity  The capacity of destination array
     * @param allowUnassigned   Unassigned values can be converted to ASCII for query operations
     *                          If true unassigned values are treated as normal Unicode code point.
     *                          If false the operation fails with U_UNASSIGNED_CODE_POINT error code.
     * @param status        ICU error code in/out parameter.
     *                      Must fulfill U_SUCCESS before the function call.
     * @return The number of UChars in the destination buffer
     */
    int32_t process(const char16_t* src, int32_t srcLength,
                            char16_t* dest, int32_t destCapacity,
                            UBool allowUnassigned,
                            UParseError* parseError,
                            UErrorCode& status );

    /**
     * Ascertain if the given code point is a label separator as specified by IDNA
     *
     * @return true is the code point is a label separator
     */
    UBool isLabelSeparator(UChar32 ch, UErrorCode& status);

    inline UBool isLDHChar(UChar32 ch);

private:
    /**
     * The address of this static class variable serves as this class's ID
     * for ICU "poor man's RTTI".
     */
    static const char fgClassID;
};

inline UBool NamePrepTransform::isLDHChar(UChar32 ch){
    // high runner case
    if(ch>0x007A){
        return false;
    }
    //[\\u002D \\u0030-\\u0039 \\u0041-\\u005A \\u0061-\\u007A]
    if( (ch==0x002D) || 
        (0x0030 <= ch && ch <= 0x0039) ||
        (0x0041 <= ch && ch <= 0x005A) ||
        (0x0061 <= ch && ch <= 0x007A)
      ){
        return true;
    }
    return false;
}

#endif /* #if !UCONFIG_NO_TRANSLITERATION */
#else
class NamePrepTransform {
};
#endif /* #if !UCONFIG_NO_IDNA */

#endif

/*
 * Hey, Emacs, please set the following:
 *
 * Local Variables:
 * indent-tabs-mode: nil
 * End:
 *
 */
