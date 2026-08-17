// © 2017 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html

// extradata.h
// created: 2017jun04 Markus W. Scherer
// (pulled out of n2builder.cpp)

// Write mappings and compositions in compact form for Normalizer2 "extra data",
// the data that does not fit into the trie itself.

#ifndef __EXTRADATA_H__
#define __EXTRADATA_H__

#include "unicode/utypes.h"

#if !UCONFIG_NO_NORMALIZATION

#include "unicode/errorcode.h"
#include "unicode/unistr.h"
#include "unicode/utf16.h"
#include "hash.h"
#include "norms.h"
#include "toolutil.h"
#include "utrie2.h"
#include "uvectr32.h"

U_NAMESPACE_BEGIN

class ExtraData : public Norms::Enumerator {
public:
    ExtraData(Norms &n, UBool fast);

    void rangeHandler(UChar32 start, UChar32 end, Norm &norm) override;

    UnicodeString maybeYesCompositions;
    UnicodeString yesYesCompositions;
    UnicodeString yesNoMappingsAndCompositions;
    UnicodeString yesNoMappingsOnly;
    UnicodeString noNoMappingsCompYes;
    UnicodeString noNoMappingsCompBoundaryBefore;
    UnicodeString noNoMappingsCompNoMaybeCC;
    UnicodeString noNoMappingsEmpty;

private:
    /**
     * Requires norm.hasMapping().
     * Returns the offset of the "first unit" from the beginning of the extraData for c.
     * That is the same as the length of the optional data
     * for the raw mapping and the ccc/lccc word.
     */
    int32_t writeMapping(UChar32 c, const Norm &norm, UnicodeString &dataString);
    int32_t writeNoNoMapping(UChar32 c, const Norm &norm,
                             UnicodeString &dataString, Hashtable &previousMappings);
    UBool setNoNoDelta(UChar32 c, Norm &norm) const;
    /** Requires norm.compositions!=nullptr. */
    void writeCompositions(UChar32 c, const Norm &norm, UnicodeString &dataString);
    void writeExtraData(UChar32 c, Norm &norm);

    UBool optimizeFast;
    Hashtable previousNoNoMappingsCompYes;  // If constructed in runtime code, pass in UErrorCode.
    Hashtable previousNoNoMappingsCompBoundaryBefore;
    Hashtable previousNoNoMappingsCompNoMaybeCC;
    Hashtable previousNoNoMappingsEmpty;
};

U_NAMESPACE_END

#endif // #if !UCONFIG_NO_NORMALIZATION

#endif  // __EXTRADATA_H__
