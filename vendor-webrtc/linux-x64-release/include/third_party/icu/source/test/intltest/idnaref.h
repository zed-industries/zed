// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
 *******************************************************************************
 *
 *   Copyright (C) 2003-2011, International Business Machines
 *   Corporation and others.  All Rights Reserved.
 *
 *******************************************************************************
 *   file name:  idnaref.h
 *   encoding:   UTF-8
 *   tab size:   8 (not used)
 *   indentation:4
 *
 *   created on: 2003feb1
 *   created by: Ram Viswanadha
 */

#ifndef __IDNAREF_H__
#define __IDNAREF_H__

#include "unicode/utypes.h"

#if !UCONFIG_NO_IDNA

#include "unicode/parseerr.h"

#define IDNAREF_DEFAULT          0x0000
#define IDNAREF_ALLOW_UNASSIGNED 0x0001
#define IDNAREF_USE_STD3_RULES   0x0002

/**
 * This function implements the ToASCII operation as defined in the IDNA draft.
 * This operation is done on <b>single labels</b> before sending it to something that expects
 * ASCII names. A label is an individual part of a domain name. Labels are usually
 * separated by dots; for e.g." "www.example.com" is composed of 3 labels 
 * "www","example", and "com".
 *
 *
 * @param src               Input Unicode label.
 * @param srcLength         Number of UChars in src, or -1 if NUL-terminated.
 * @param dest              Output Unicode array with ACE encoded ASCII label.
 * @param destCapacity      Size of dest.
 * @param options           A bit set of options:
 *  
 *  - idnaref_UNASSIGNED        Unassigned values can be converted to ASCII for query operations
 *                          If true unassigned values are treated as normal Unicode code points.
 *                          If false the operation fails with U_UNASSIGNED_CODE_POINT_FOUND error code.
 *  - idnaref_USE_STD3_RULES    Use STD3 ASCII rules for host name syntax restrictions
 *                          If true and the input does not satisfy STD3 rules, the operation 
 *                          will fail with U_IDNA_STD3_ASCII_RULES_ERROR
 *
 * @param parseError        Pointer to UParseError struct to receive information on position 
 *                          of error if an error is encountered. Can be NULL.
 * @param status            ICU in/out error code parameter.
 *                          U_INVALID_CHAR_FOUND if src contains
 *                          unmatched single surrogates.
 *                          U_INDEX_OUTOFBOUNDS_ERROR if src contains
 *                          too many code points.
 *                          U_BUFFER_OVERFLOW_ERROR if destCapacity is not enough
 * @return                  Number of ASCII characters converted.
 */
U_CFUNC int32_t U_EXPORT2
idnaref_toASCII(const UChar* src, int32_t srcLength, 
              UChar* dest, int32_t destCapacity,
              int32_t options,
              UParseError* parseError,
              UErrorCode* status);


/**
 * This function implements the ToUnicode operation as defined in the IDNA draft.
 * This operation is done on <b>single labels</b> before sending it to something that expects
 * ASCII names. A label is an individual part of a domain name. Labels are usually
 * separated by dots; for e.g." "www.example.com" is composed of 3 labels 
 * "www","example", and "com".
 *
 * @param src               Input ASCII (ACE encoded) label.
 * @param srcLength         Number of UChars in src, or -1 if NUL-terminated.
 * @param dest Output       Converted Unicode array.
 * @param destCapacity      Size of dest.
 * @param options           A bit set of options:
 *  
 *  - idnaref_UNASSIGNED        Unassigned values can be converted to ASCII for query operations
 *                          If true unassigned values are treated as normal Unicode code points.
 *                          If false the operation fails with U_UNASSIGNED_CODE_POINT_FOUND error code.
 *  - idnaref_USE_STD3_RULES    Use STD3 ASCII rules for host name syntax restrictions
 *                          If true and the input does not satisfy STD3 rules, the operation 
 *                          will fail with U_IDNA_STD3_ASCII_RULES_ERROR
 *
 * @param parseError        Pointer to UParseError struct to receive information on position 
 *                          of error if an error is encountered. Can be NULL.
 * @param status            ICU in/out error code parameter.
 *                          U_INVALID_CHAR_FOUND if src contains
 *                          unmatched single surrogates.
 *                          U_INDEX_OUTOFBOUNDS_ERROR if src contains
 *                          too many code points.
 *                          U_BUFFER_OVERFLOW_ERROR if destCapacity is not enough
 * @return                  Number of Unicode characters converted.
 */
U_CFUNC int32_t U_EXPORT2
idnaref_toUnicode(const UChar* src, int32_t srcLength,
                UChar* dest, int32_t destCapacity,
                int32_t options,
                UParseError* parseError,
                UErrorCode* status);


/**
 * Convenience function that implements the IDNToASCII operation as defined in the IDNA draft.
 * This operation is done on complete domain names, e.g: "www.example.com". 
 * It is important to note that this operation can fail. If it fails, then the input 
 * domain name cannot be used as an Internationalized Domain Name and the application
 * should have methods defined to deal with the failure.
 * 
 * <b>Note:</b> IDNA draft specifies that a conformant application should divide a domain name
 * into separate labels, decide whether to apply allowUnassigned and useSTD3ASCIIRules on each, 
 * and then convert. This function does not offer that level of granularity. The options once  
 * set will apply to all labels in the domain name
 *
 * @param src               Input ASCII IDN.
 * @param srcLength         Number of UChars in src, or -1 if NUL-terminated.
 * @param dest Output       Unicode array.
 * @param destCapacity      Size of dest.
 * @param options           A bit set of options:
 *  
 *  - idnaref_UNASSIGNED        Unassigned values can be converted to ASCII for query operations
 *                          If true unassigned values are treated as normal Unicode code points.
 *                          If false the operation fails with U_UNASSIGNED_CODE_POINT_FOUND error code.
 *  - idnaref_USE_STD3_RULES    Use STD3 ASCII rules for host name syntax restrictions
 *                          If true and the input does not satisfy STD3 rules, the operation 
 *                          will fail with U_IDNA_STD3_ASCII_RULES_ERROR
 * 
 * @param parseError        Pointer to UParseError struct to receive information on position 
 *                          of error if an error is encountered. Can be NULL.
 * @param status            ICU in/out error code parameter.
 *                          U_INVALID_CHAR_FOUND if src contains
 *                          unmatched single surrogates.
 *                          U_INDEX_OUTOFBOUNDS_ERROR if src contains
 *                          too many code points.
 *                          U_BUFFER_OVERFLOW_ERROR if destCapacity is not enough
 * @return                  Number of ASCII characters converted.
 */
U_CFUNC int32_t U_EXPORT2
idnaref_IDNToASCII(  const UChar* src, int32_t srcLength,
                   UChar* dest, int32_t destCapacity,
                   int32_t options,
                   UParseError* parseError,
                   UErrorCode* status);

/**
 * Convenience function that implements the IDNToUnicode operation as defined in the IDNA draft.
 * This operation is done on complete domain names, e.g: "www.example.com". 
 *
 * <b>Note:</b> IDNA draft specifies that a conformant application should divide a domain name
 * into separate labels, decide whether to apply allowUnassigned and useSTD3ASCIIRules on each, 
 * and then convert. This function does not offer that level of granularity. The options once  
 * set will apply to all labels in the domain name
 *
 * @param src               Input Unicode IDN.
 * @param srcLength         Number of UChars in src, or -1 if NUL-terminated.
 * @param dest Output       ASCII array.
 * @param destCapacity      Size of dest.
 * @param options           A bit set of options:
 *  
 *  - idnaref_UNASSIGNED        Unassigned values can be converted to ASCII for query operations
 *                          If true unassigned values are treated as normal Unicode code points.
 *                          If false the operation fails with U_UNASSIGNED_CODE_POINT_FOUND error code.
 *  - idnaref_USE_STD3_RULES    Use STD3 ASCII rules for host name syntax restrictions
 *                          If true and the input does not satisfy STD3 rules, the operation 
 *                          will fail with U_IDNA_STD3_ASCII_RULES_ERROR
 *
 * @param parseError        Pointer to UParseError struct to receive information on position 
 *                          of error if an error is encountered. Can be NULL.
 * @param status            ICU in/out error code parameter.
 *                          U_INVALID_CHAR_FOUND if src contains
 *                          unmatched single surrogates.
 *                          U_INDEX_OUTOFBOUNDS_ERROR if src contains
 *                          too many code points.
 *                          U_BUFFER_OVERFLOW_ERROR if destCapacity is not enough
 * @return                  Number of ASCII characters converted.
 */
U_CFUNC int32_t U_EXPORT2
idnaref_IDNToUnicode(  const UChar* src, int32_t srcLength,
                     UChar* dest, int32_t destCapacity,
                     int32_t options,
                     UParseError* parseError,
                     UErrorCode* status);

/**
 * Compare two strings for IDNs for equivalence.
 * This function splits the domain names into labels and compares them.
 * According to IDN draft, whenever two labels are compared, they are 
 * considered equal if and only if their ASCII forms (obtained by 
 * applying toASCII) match using an case-insensitive ASCII comparison.
 * Two domain names are considered a match if and only if all labels 
 * match regardless of whether label separators match.
 *
 * @param s1                First source string.
 * @param length1           Length of first source string, or -1 if NUL-terminated.
 *
 * @param s2                Second source string.
 * @param length2           Length of second source string, or -1 if NUL-terminated.
 * @param options           A bit set of options:
 *  
 *  - idnaref_UNASSIGNED        Unassigned values can be converted to ASCII for query operations
 *                          If true unassigned values are treated as normal Unicode code points.
 *                          If false the operation fails with U_UNASSIGNED_CODE_POINT_FOUND error code.
 *  - idnaref_USE_STD3_RULES    Use STD3 ASCII rules for host name syntax restrictions
 *                          If true and the input does not satisfy STD3 rules, the operation 
 *                          will fail with U_IDNA_STD3_ASCII_RULES_ERROR
 *
 * @param status            ICU error code in/out parameter.
 *                          Must fulfill U_SUCCESS before the function call.
 * @return <0 or 0 or >0 as usual for string comparisons
 */
U_CFUNC int32_t U_EXPORT2
idnaref_compare(  const UChar *s1, int32_t length1,
                const UChar *s2, int32_t length2,
                int32_t options,
                UErrorCode* status);

#endif /* #if !UCONFIG_NO_IDNA */

#endif
