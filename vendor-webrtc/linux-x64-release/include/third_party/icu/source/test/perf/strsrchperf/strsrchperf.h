/************************************************************************
 * © 2016 and later: Unicode, Inc. and others.
 * License & terms of use: http://www.unicode.org/copyright.html
 *
 *************************************************************************
 ********************************************************************
 * COPYRIGHT:
 * Copyright (C) 2008-2012 IBM, Inc.   All Rights Reserved.
 *
 ********************************************************************/
#ifndef _STRSRCHPERF_H
#define _STRSRCHPERF_H

#include "unicode/usearch.h"
#include "unicode/uperf.h"
#include <stdlib.h>
#include <stdio.h>

typedef void (*StrSrchFn)(UStringSearch* srch, const char16_t* src,int32_t srcLen, const char16_t* pttrn, int32_t pttrnLen, UErrorCode* status);

class StringSearchPerfFunction : public UPerfFunction {
private:
    StrSrchFn fn;
    const char16_t* src;
    int32_t srcLen;
    const char16_t* pttrn;
    int32_t pttrnLen;
    UStringSearch* srch;
    
public:
    virtual void call(UErrorCode* status) {
        (*fn)(srch, src, srcLen, pttrn, pttrnLen, status);
    }
    
    virtual long getOperationsPerIteration() {
        return (long) srcLen;
    }
    
    StringSearchPerfFunction(StrSrchFn func, UStringSearch* search, const char16_t* source,int32_t sourceLen, const char16_t* pattern, int32_t patternLen) {
        fn = func;
        src = source;
        srcLen = sourceLen;
        pttrn = pattern;
        pttrnLen = patternLen;
        srch = search;
    }
};

class StringSearchPerformanceTest : public UPerfTest {
private:
    const char16_t* src;
    int32_t srcLen;
    char16_t* pttrn;
    int32_t pttrnLen;
    UStringSearch* srch;
    
public:
    StringSearchPerformanceTest(int32_t argc, const char *argv[], UErrorCode &status);
    ~StringSearchPerformanceTest();
    virtual UPerfFunction* runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = nullptr);
    UPerfFunction* Test_ICU_Forward_Search();
    UPerfFunction* Test_ICU_Backward_Search();
};


void ICUForwardSearch(UStringSearch *srch, const char16_t* source, int32_t sourceLen, const char16_t* pattern, int32_t patternLen, UErrorCode* status) {
    int32_t match;
    
    match = usearch_first(srch, status);
    while (match != USEARCH_DONE) {
        match = usearch_next(srch, status);
    }
}

void ICUBackwardSearch(UStringSearch *srch, const char16_t* source, int32_t sourceLen, const char16_t* pattern, int32_t patternLen, UErrorCode* status) {
    int32_t match;
    
    match = usearch_last(srch, status);
    while (match != USEARCH_DONE) {
        match = usearch_previous(srch, status);
    }
}

#endif /* _STRSRCHPERF_H */
