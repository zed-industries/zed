// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
 *******************************************************************************
 *
 *   Copyright (C) 2003, International Business Machines
 *   Corporation and others.  All Rights Reserved.
 *
 *******************************************************************************
 *   file name:  nfsprep.h
 *   encoding:   UTF-8
 *   tab size:   8 (not used)
 *   indentation:4
 *
 *   created on: 2003jul11
 *   created by: Ram Viswanadha
 */
#ifndef _NFSPREP_H
#define _NFSPREP_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_IDNA

#include "unicode/ustring.h"
#include "unicode/usprep.h"
#include <stdlib.h>
#include <string.h>


/* this enum must be kept in syn with NFS4DataFileNames array in nfsprep.c */
enum NFS4ProfileState{
    NFS4_CS_PREP_CS,
    NFS4_CS_PREP_CI,
    NFS4_CIS_PREP,
    NFS4_MIXED_PREP_PREFIX,
    NFS4_MIXED_PREP_SUFFIX
};

typedef enum NFS4ProfileState NFS4ProfileState;

/**
 * Prepares the source UTF-8 string for use in file names and 
 * returns UTF-8 string on output.
 * @param src
 * @param srcLen
 * @param dest
 * @param destCapacity
 * @param state
 * @param parseError
 * @param status
 */
int32_t
nfs4_prepare(const char* src, int32_t srcLength,
                  char* dest, int32_t destCapacity,
                  NFS4ProfileState state,
                  UParseError* parseError,
                  UErrorCode*  status);

/**
 * @param dest
 * @param destCapacity
 * @param src
 * @param srcLen
 * @param state
 * @param parseError
 * @param status
 */
int32_t
nfs4_mixed_prepare( const char* src, int32_t srcLength,
                    char* dest, int32_t destCapacity,
                    UParseError* parseError,
                    UErrorCode*  status);

/**
 * @param dest
 * @param destCapacity
 * @param src
 * @param srcLen
 * @param state
 * @param parseError
 * @param status
 */
int32_t
nfs4_cis_prepare(   const char* src, int32_t srcLength,
                    char* dest, int32_t destCapacity,
                    UParseError* parseError,
                    UErrorCode*  status);

/**
 * @param dest
 * @param destCapacity
 * @param src
 * @param srcLen
 * @param state
 * @param parseError
 * @param status
 */
int32_t
nfs4_cs_prepare(    const char* src, int32_t srcLength,
                    char* dest, int32_t destCapacity,
                    UBool isCaseSensitive,
                    UParseError* parseError,
                    UErrorCode*  status);
#endif

#endif
/*
 * Hey, Emacs, please set the following:
 *
 * Local Variables:
 * indent-tabs-mode: nil
 * End:
 *
 */
