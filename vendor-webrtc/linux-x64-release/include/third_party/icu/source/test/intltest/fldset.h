// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
************************************************************************
* Copyright (c) 2007-2010, International Business Machines
* Corporation and others.  All Rights Reserved.
************************************************************************
*/
#ifndef FLDSET_H_
#define FLDSET_H_

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING
#include "unicode/calendar.h"
#include "unicode/ucal.h"
#include "unicode/udat.h"
#include "udbgutil.h"
#include "dbgutil.h"
#include "unicode/unistr.h"

#define U_FIELDS_SET_MAX  64

/**
 * This class represents a collection of integer values (fields), each designated by
 * one of a particular set of enum values.  Each integer value (int32_t) is optional and
 * may or may not be set. 
 * 
 * @internal ICU 3.8
 */
class FieldsSet {
    protected:
        /**
         * subclass interface - construct the FieldsSet to reference one of the standard
         * enumerations.
         * @param whichEnum which enumaration value goes with this set. Will be used to calculate string
         * values and also enum size.
         * @see UDebugEnumType
         */
        FieldsSet(UDebugEnumType whichEnum);
        
        /**
         * subclass interface - construct the FieldsSet without using a standard enum type.
         * @param fieldCount how many fields this object can hold.
         */
        FieldsSet(int32_t fieldsCount);

    public:
    
      /**
       * Compare two sets. In typical test usage, 'this' is the result of 
       * a tested operation, and 'other' is the predefined expected value.
       * 
       * @param other the set to compare against.
       * @param status will return U_ILLEGAL_ARGUMENT_ERROR if sets are not the same size
       * @return a formatted string listing which fields are set in 
       *   this, with the comparison made agaainst those fields in other.
       */
      U_NAMESPACE_QUALIFIER UnicodeString diffFrom(const FieldsSet& other, UErrorCode &status) const;

    public:
      /**
       * Fill-in fields from a specified string, such as "NAME1=VALUE1,NAME2=VALUE2", etc. 
       * @param str string to parse
       * @param status status of parse
       * @return the number of valid parsed fields on success, or a negative number on failure.
       */
      int32_t parseFrom(const U_NAMESPACE_QUALIFIER UnicodeString& str, UErrorCode& status) {
          return parseFrom(str,nullptr,status);
      }

      /**
       * Fill-in fields from a specified string, such as "NAME1=VALUE1,NAME2=VALUE2", etc. 
       * @param inheritFrom if a field's value is given as 0-length, such as NAME1 in "NAME1=,NAME2=VALUE2", 
       * the specified FieldsSet's value for NAME1 will be copied into this.
       * @param str string to parse
       * @param status status of parse
       * @return the number of valid parsed fields on success, or a negative number on failure.
       */
      int32_t parseFrom(const U_NAMESPACE_QUALIFIER UnicodeString& str,
                        const FieldsSet& inheritFrom,
                        UErrorCode& status) {
          return parseFrom(str, &inheritFrom, status);
      }

      /**
       * Fill-in fields from a specified string, such as "NAME1=VALUE1,NAME2=VALUE2", etc. 
       * @param inheritFrom if a field's value is given as 0-length, such as NAME1 in "NAME1=,NAME2=VALUE2", 
       * the specified FieldsSet's value for NAME1 will be copied into this.
       * @param str string to parse
       * @param status status of parse
       * @return the number of valid parsed fields on success, or a negative number on failure.
       */
      int32_t parseFrom(const U_NAMESPACE_QUALIFIER UnicodeString& str,
                        const FieldsSet* inheritFrom,
                        UErrorCode& status);

    protected:
      /**
       * Callback interface for subclass.
       * This function is called when parsing a field name, such as "MONTH"  in "MONTH=4".
       * Base implementation is to lookup the enum value using udbg_* utilities, or else as an integer if
       * enum is not available.
       * 
       * If there is a special directive, the implementer can catch it here and return -1 after special processing completes.
       * 
       * @param inheritFrom the set inheriting from - may be null.
       * @param name the field name (key side)
       * @param substr the string in question (value side)
       * @param status error status - set to error for failure.
       * @return field number, or negative if field should be skipped.
       */
      virtual int32_t handleParseName(const FieldsSet* inheritFrom,
                                      const U_NAMESPACE_QUALIFIER UnicodeString& name,
                                      const U_NAMESPACE_QUALIFIER UnicodeString& substr,
                                      UErrorCode& status);

      /**
       * Callback interface for subclass.
       * Base implementation is to call parseValueDefault(...)
       * @param inheritFrom the set inheriting from - may be null.
       * @param field which field is being parsed
       * @param substr the string in question (value side)
       * @param status error status - set to error for failure.
       * @see parseValueDefault
       */
      virtual void handleParseValue(const FieldsSet* inheritFrom,
                                    int32_t field,
                                    const U_NAMESPACE_QUALIFIER UnicodeString& substr,
                                    UErrorCode& status);

      /**
       * the default implementation for handleParseValue.
       * Base implementation is to parse a decimal integer value, or inherit from inheritFrom if the string is 0-length.
       * Implementations of this function should call set(field,...) on successful parse.
       * @see handleParseValue
       */
      void parseValueDefault(const FieldsSet* inheritFrom,
                             int32_t field,
                             const U_NAMESPACE_QUALIFIER UnicodeString& substr,
                             UErrorCode& status);      


      /**
       * convenience implementation for handleParseValue
       * attempt to load a value from an enum value using udbg_enumByString()
       * if fails, will call parseValueDefault()
       * @see handleParseValue
       */
      void parseValueEnum(UDebugEnumType type,
                          const FieldsSet* inheritFrom,
                          int32_t field,
                          const U_NAMESPACE_QUALIFIER UnicodeString& substr,
                          UErrorCode& status);

    private:
      /** 
       * Not callable - construct a default FieldsSet
       * @internal
       */
      FieldsSet();
      
      /**
       * construct the object.
       * @internal
       */
      void construct(UDebugEnumType whichEnum, int32_t fieldCount);

    public:
    /**
     * destructor
     */
     virtual ~FieldsSet();

    /**
     * Mark all fields as unset
     */
    void clear();
    
    /**
     * Mark a specific field as unset
     * @param field the field to unset
     */
    void clear(int32_t field);
    
    /**
     * Set a specific field
     * @param field the field to set (i.e. enum value)
     * @param value the field's value
     */
    void set(int32_t field, int32_t value);
    
    UBool isSet(int32_t field) const;

    /**
     * Return the field's value
     * @param field which field
     * @return field's value, or -1 if unset.
     */
    int32_t get(int32_t field) const;
    
    /**
     * Return true if both FieldsSet objects either are based on the same enum, or have the same number of fields.
     */
    UBool isSameType(const FieldsSet& other) const;

    /**
     * @return the number of fields 
     */
    int32_t fieldCount() const;
    
    protected:
       int32_t fValue[U_FIELDS_SET_MAX];
       UBool fIsSet[U_FIELDS_SET_MAX];
    protected:
       int32_t fFieldCount;
       UDebugEnumType fEnum;
};

/**
 * A subclass of FieldsSet representing the fields in a Calendar
 * @see Calendar
 */
class CalendarFieldsSet : public FieldsSet {
public:
    CalendarFieldsSet();
    virtual ~CalendarFieldsSet();

//        void clear(UCalendarDateFields field) { clear((int32_t)field); }
//        void set(UCalendarDateFields field, int32_t amount) { set ((int32_t)field, amount); }

//        UBool isSet(UCalendarDateFields field) const { return isSet((int32_t)field); }
//        int32_t get(UCalendarDateFields field) const { return get((int32_t)field); }

    /**
     * @param matches fillin to hold any fields different. Will have the calendar's value set on them.
     * @return true if the calendar matches in these fields.
     */
    UBool matches(U_NAMESPACE_QUALIFIER Calendar *cal,
                  CalendarFieldsSet &diffSet,
                  UErrorCode& status) const;

    /**
     * For each set field, set the same field on this Calendar.
     * Doesn't clear the Calendar first.
     * @param cal Calendar to modify
     * @param status Contains any errors propagated by the Calendar.
     */
    void setOnCalendar(U_NAMESPACE_QUALIFIER Calendar *cal, UErrorCode& status) const;
        
protected:
    /**
     * subclass override 
     */
    void handleParseValue(const FieldsSet* inheritFrom,
                          int32_t field,
                          const U_NAMESPACE_QUALIFIER UnicodeString& substr,
                          UErrorCode& status) override;
};

/**
 * This class simply implements a set of date and time styles
 * such as DATE=SHORT  or TIME=SHORT,DATE=LONG, such as would be passed
 * to DateFormat::createInstance()
 * @see DateFormat
 */
class DateTimeStyleSet : public FieldsSet {
    public:
        DateTimeStyleSet();
        virtual ~DateTimeStyleSet();

        /** 
         * @return the date style, or UDAT_NONE if not set
         */
        UDateFormatStyle getDateStyle() const;
        
        /** 
         * @return the time style, or UDAT_NONE if not set
         */
        UDateFormatStyle getTimeStyle() const;
    protected:
        void handleParseValue(const FieldsSet* inheritFrom,
                              int32_t field,
                              const U_NAMESPACE_QUALIFIER UnicodeString& substr,
                              UErrorCode& status) override;
        int32_t handleParseName(const FieldsSet* inheritFrom,
                                const U_NAMESPACE_QUALIFIER UnicodeString& name,
                                const U_NAMESPACE_QUALIFIER UnicodeString& substr,
                                UErrorCode& status) override;
};


#endif /*!UCONFIG_NO_FORMAT*/
#endif /*FLDSET_H_*/
