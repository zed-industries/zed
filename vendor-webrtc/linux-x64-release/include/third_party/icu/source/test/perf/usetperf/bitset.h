/*
***********************************************************************
* © 2016 and later: Unicode, Inc. and others.
* License & terms of use: http://www.unicode.org/copyright.html
***********************************************************************
***********************************************************************
* Copyright (c) 2002-2005, International Business Machines
* Corporation and others.  All Rights Reserved.
***********************************************************************
* 2002-09-20 aliu Created.
*/
#ifndef __BITSET_H__
#define __BITSET_H__

#include "unicode/utypes.h"

/**
 * A simple, limited clone of the java.util.BitSet.
 */
class BitSet {

    uint32_t len;
    int32_t* data;

    void ensureCapacity(uint32_t minLen);

public:

    BitSet();
    ~BitSet();

    UBool get(int32_t bitIndex) const;

    void set(int32_t bitIndex);

    // Non-java
    void clearAll();

    // TODO add other methods as needed.
};

#endif
