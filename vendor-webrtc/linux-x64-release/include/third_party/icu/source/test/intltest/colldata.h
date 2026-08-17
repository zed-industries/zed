// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
 ******************************************************************************
 *   Copyright (C) 1996-2012, International Business Machines                 *
 *   Corporation and others.  All Rights Reserved.                            *
 ******************************************************************************
 */

/**
 * \file 
 * \brief Originally, added as C++ API for Collation data used to compute minLengthInChars
 * \internal
 */

/*
 * Note: This module was included in ICU 4.0.1 as @internal technology preview for supporting
 * Boyer-Moore string search API. For now, only SSearchTest depends on this module.
 * I temporarily moved the module from i18n directory to intltest, because we have no plan to
 * publish this as public API. (2012-12-18 yoshito)
 */

#ifndef COLL_DATA_H
#define COLL_DATA_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_COLLATION

#include "unicode/ucol.h"
#include "unicode/unistr.h"

 /**
  * The size of the internal CE buffer in a <code>CEList</code> object
  */
#define CELIST_BUFFER_SIZE 4

/**
 * \def INSTRUMENT_CELIST
 * Define this to enable the <code>CEList</code> objects to collect
 * statistics.
 */

 /**
  * The size of the initial list in a <code>StringList</code> object.
  */
#define STRING_LIST_BUFFER_SIZE 16

U_NAMESPACE_USE

 /**
  * This object holds a list of CEs generated from a particular
  * <code>UnicodeString</code>
  *
  */
class CEList
{
public:
    /**
     * Construct a <code>CEList</code> object.
     *
     * @param coll - the Collator used to collect the CEs.
     * @param string - the string for which to collect the CEs.
     * @param status - will be set if any errors occur. 
     *
     * Note: if on return, status is set to an error code,
     * the only safe thing to do with this object is to call
     * the destructor.
     */
    CEList(UCollator *coll, const UnicodeString &string, UErrorCode &status);

    /**
     * The destructor.
     */
    ~CEList();

    /**
     * Return the number of CEs in the list.
     *
     * @return the number of CEs in the list.
     */
    int32_t size() const;

    /**
     * Get a particular CE from the list.
     *
     * @param index - the index of the CE to return
     *
     * @return the CE, or <code>0</code> if <code>index</code> is out of range
     */
    uint32_t get(int32_t index) const;

    /**
     * Check if the CEs in another <code>CEList</code> match the
     * suffix of this list starting at a give offset.
     *
     * @param offset - the offset of the suffix
     * @param other - the other <code>CEList</code>
     *
     * @return <code>true</code> if the CEs match, <code>false</code> otherwise.
     */
    UBool matchesAt(int32_t offset, const CEList *other) const; 

    /**
     * The index operator.
     *
     * @param index - the index
     *
     * @return a reference to the given CE in the list
     */
    uint32_t &operator[](int32_t index) const;

private:
    void add(uint32_t ce, UErrorCode &status);

    uint32_t ceBuffer[CELIST_BUFFER_SIZE];
    uint32_t *ces;
    int32_t listMax;
    int32_t listSize;
};

/**
 * StringList
 *
 * This object holds a list of <code>UnicodeString</code> objects.
 */
class StringList
{
public:
    /**
     * Construct an empty <code>StringList</code>
     *
     * @param status - will be set if any errors occur. 
     *
     * Note: if on return, status is set to an error code,
     * the only safe thing to do with this object is to call
     * the destructor.
     */
    StringList(UErrorCode &status);

    /**
     * The destructor.
     */
    ~StringList();

    /**
     * Add a string to the list.
     *
     * @param string - the string to add
     * @param status - will be set if any errors occur. 
     */
    void add(const UnicodeString *string, UErrorCode &status);

    /**
     * Add an array of Unicode code points to the list.
     *
     * @param chars - the address of the array of code points
     * @param count - the number of code points in the array
     * @param status - will be set if any errors occur. 
     */
    void add(const char16_t *chars, int32_t count, UErrorCode &status);

    /**
     * Get a particular string from the list.
     *
     * @param index - the index of the string
     *
     * @return a pointer to the <code>UnicodeString</code> or <code>nullptr</code>
     *         if <code>index</code> is out of bounds.
     */
    const UnicodeString *get(int32_t index) const;

    /**
     * Get the number of strings in the list.
     *
     * @return the number of strings in the list.
     */
    int32_t size() const;

private:
    UnicodeString *strings;
    int32_t listMax;
    int32_t listSize;
};


/*
 * Forward references to internal classes.
 */
class StringToCEsMap;
class CEToStringsMap;

/**
 * CollData
 *
 * This class holds the Collator-specific data needed to
 * compute the length of the shortest string that can
 * generate a particular list of CEs.
 *
 * <code>CollData</code> objects are quite expensive to compute. Because
 * of this, they are cached. When you call <code>CollData::open</code> it
 * returns a reference counted cached object. When you call <code>CollData::close</code>
 * the reference count on the object is decremented but the object is not deleted.
 *
 * If you do not need to reuse any unreferenced objects in the cache, you can call
 * <code>CollData::flushCollDataCache</code>. If you no longer need any <code>CollData</code>
 * objects, you can call <code>CollData::freeCollDataCache</code>
 */
class CollData
{
public:
    /**
     * Construct a <code>CollData</code> object.
     *
     * @param collator - the collator
     * @param status - will be set if any errors occur. 
     */
    CollData(UCollator *collator, UErrorCode &status);

    /**
     * The destructor.
     */
    ~CollData();

    /**
     * Get the <code>UCollator</code> object used to create this object.
     * The object returned may not be the exact object that was used to
     * create this object, but it will have the same behavior.
     */
    UCollator *getCollator() const;

    /**
     * Get a list of all the strings which generate a list
     * of CEs starting with a given CE.
     *
     * @param ce - the CE
     *
     * return a <code>StringList</code> object containing all
     *        the strings, or <code>nullptr</code> if there are
     *        no such strings.
     */
    const StringList *getStringList(int32_t ce) const;

    /**
     * Get a list of the CEs generated by a particular string.
     *
     * @param string - the string
     *
     * @return a <code>CEList</code> object containing the CEs. You
     *         must call <code>freeCEList</code> when you are finished
     *         using the <code>CEList</code>/
     */
    const CEList *getCEList(const UnicodeString *string) const;

    /**
     * Release a <code>CEList</code> returned by <code>getCEList</code>.
     *
     * @param list - the <code>CEList</code> to free.
     */
    void freeCEList(const CEList *list);

    /**
     * Return the length of the shortest string that will generate
     * the given list of CEs.
     *
     * @param ces - the CEs
     * @param offset - the offset of the first CE in the list to use.
     *
     * @return the length of the shortest string.
     */
    int32_t minLengthInChars(const CEList *ces, int32_t offset) const;

 
    /**
     * Return the length of the shortest string that will generate
     * the given list of CEs.
     *
     * Note: the algorithm used to do this computation is recursive. To
     * limit the amount of recursion, a "history" list is used to record
     * the best answer starting at a particular offset in the list of CEs.
     * If the same offset is visited again during the recursion, the answer
     * in the history list is used.
     *
     * @param ces - the CEs
     * @param offset - the offset of the first CE in the list to use.
     * @param history - the history list. Must be at least as long as
     *                 the number of cEs in the <code>CEList</code>
     *
     * @return the length of the shortest string.
     */
   int32_t minLengthInChars(const CEList *ces, int32_t offset, int32_t *history) const;

private:
    UCollator      *coll;
    CEToStringsMap *ceToCharsStartingWith;

    uint32_t minHan;
    uint32_t maxHan;

    uint32_t jamoLimits[4];
};

#endif // #if !UCONFIG_NO_COLLATION
#endif // #ifndef COLL_DATA_H
