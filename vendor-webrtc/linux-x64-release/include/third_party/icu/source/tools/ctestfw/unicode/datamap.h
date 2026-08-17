// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 2002-2006, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

/* Created by weiv 05/09/2002 */

#ifndef U_TESTFW_DATAMAP
#define U_TESTFW_DATAMAP

#include "unicode/resbund.h"
#include "unicode/testtype.h"



U_NAMESPACE_BEGIN
class Hashtable;
U_NAMESPACE_END

/** Holder of test data and settings. Allows addressing of items by name.
 *  For test cases, names are defined in the "Headers" section. For settings
 *  and info data, names are keys in data. Currently, we return scalar strings
 *  and integers and arrays of strings and integers. Arrays should be deposited
 *  of by the user. 
 */
class T_CTEST_EXPORT_API DataMap {
public:
  virtual ~DataMap();

protected:
  DataMap();
  int32_t utoi(const UnicodeString &s) const;


public:
  /** get the string from the DataMap. Addressed by name
   *  @param key name of the data field.
   *  @return a string containing the data
   */
  virtual const UnicodeString getString(const char* key, UErrorCode &status) const = 0;

  /** get the string from the DataMap. Addressed by name
   *  parses a bundle string into an integer
   *  @param key name of the data field.
   *  @return an integer containing the data
   */
  virtual int32_t getInt(const char* key, UErrorCode &status) const = 0;

  /**
   * Get a signed integer without runtime parsing.
   * @param key name of the data field.
   * @param status UErrorCode in/out parameter
   * @return the integer
   */
  virtual int32_t getInt28(const char* key, UErrorCode &status) const = 0;

  /**
   * Get an unsigned integer without runtime parsing.
   * @param key name of the data field.
   * @param status UErrorCode in/out parameter
   * @return the integer
   */
  virtual uint32_t getUInt28(const char* key, UErrorCode &status) const = 0;

  /**
   * Get a vector of integers without runtime parsing.
   * @param length output parameter for the length of the vector
   * @param key name of the data field.
   * @param status UErrorCode in/out parameter
   * @return the integer vector, do not delete
   */
  virtual const int32_t *getIntVector(int32_t &length, const char *key, UErrorCode &status) const = 0;

  /**
   * Get binary data without runtime parsing.
   * @param length output parameter for the length of the data
   * @param key name of the data field.
   * @param status UErrorCode in/out parameter
   * @return the binary data, do not delete
   */
  virtual const uint8_t *getBinary(int32_t &length, const char *key, UErrorCode &status) const = 0;

  /** get an array of strings from the DataMap. Addressed by name.
   *  The user must dispose of it after usage, using delete.
   *  @param key name of the data field.
   *  @return a string array containing the data
   */
  virtual const UnicodeString* getStringArray(int32_t& count, const char* key, UErrorCode &status) const = 0;

  /** get an array of integers from the DataMap. Addressed by name.
   *  The user must dispose of it after usage, using delete.
   *  @param key name of the data field.
   *  @return an integer array containing the data
   */
  virtual const int32_t* getIntArray(int32_t& count, const char* key, UErrorCode &status) const = 0;

  // ... etc ...
};

// This one is already concrete - it is going to be instantiated from 
// concrete data by TestData children...
class T_CTEST_EXPORT_API RBDataMap : public DataMap{
private:
  Hashtable *fData;

public:
  virtual ~RBDataMap();

public:
  RBDataMap();

  RBDataMap(UResourceBundle *data, UErrorCode &status);
  RBDataMap(UResourceBundle *headers, UResourceBundle *data, UErrorCode &status);

public:
  void init(UResourceBundle *data, UErrorCode &status);
  void init(UResourceBundle *headers, UResourceBundle *data, UErrorCode &status);

  virtual const ResourceBundle *getItem(const char* key, UErrorCode &status) const;

  virtual const UnicodeString getString(const char* key, UErrorCode &status) const override;
  virtual int32_t getInt28(const char* key, UErrorCode &status) const override;
  virtual uint32_t getUInt28(const char* key, UErrorCode &status) const override;
  virtual const int32_t *getIntVector(int32_t &length, const char *key, UErrorCode &status) const override;
  virtual const uint8_t *getBinary(int32_t &length, const char *key, UErrorCode &status) const override;

  virtual int32_t getInt(const char* key, UErrorCode &status) const override;
  
  virtual const UnicodeString* getStringArray(int32_t& count, const char* key, UErrorCode &status) const override;
  virtual const int32_t* getIntArray(int32_t& count, const char* key, UErrorCode &status) const override;

  // ... etc ...
};


#endif

