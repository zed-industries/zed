// Locale support -*- C++ -*-

// Copyright (C) 2007-2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the
// Free Software Foundation; either version 3, or (at your option)
// any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/** @file bits/locale_facets_nonio.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{locale}
 */

//
// ISO C++ 14882: 22.1  Locales
//

#ifndef _LOCALE_FACETS_NONIO_H
#define _LOCALE_FACETS_NONIO_H 1

#pragma GCC system_header

#include <ctime>	// For struct tm

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   *  @brief  Time format ordering data.
   *  @ingroup locales
   *
   *  This class provides an enum representing different orderings of
   *  time: day, month, and year.
  */
  class time_base
  {
  public:
    enum dateorder { no_order, dmy, mdy, ymd, ydm };
  };

  template<typename _CharT>
    struct __timepunct_cache : public locale::facet
    {
      // List of all known timezones, with GMT first.
      static const _CharT*		_S_timezones[14];

      const _CharT*			_M_date_format;
      const _CharT*			_M_date_era_format;
      const _CharT*			_M_time_format;
      const _CharT*			_M_time_era_format;
      const _CharT*			_M_date_time_format;
      const _CharT*			_M_date_time_era_format;
      const _CharT*			_M_am;
      const _CharT*			_M_pm;
      const _CharT*			_M_am_pm_format;

      // Day names, starting with "C"'s Sunday.
      const _CharT*			_M_day1;
      const _CharT*			_M_day2;
      const _CharT*			_M_day3;
      const _CharT*			_M_day4;
      const _CharT*			_M_day5;
      const _CharT*			_M_day6;
      const _CharT*			_M_day7;

      // Abbreviated day names, starting with "C"'s Sun.
      const _CharT*			_M_aday1;
      const _CharT*			_M_aday2;
      const _CharT*			_M_aday3;
      const _CharT*			_M_aday4;
      const _CharT*			_M_aday5;
      const _CharT*			_M_aday6;
      const _CharT*			_M_aday7;

      // Month names, starting with "C"'s January.
      const _CharT*			_M_month01;
      const _CharT*			_M_month02;
      const _CharT*			_M_month03;
      const _CharT*			_M_month04;
      const _CharT*			_M_month05;
      const _CharT*			_M_month06;
      const _CharT*			_M_month07;
      const _CharT*			_M_month08;
      const _CharT*			_M_month09;
      const _CharT*			_M_month10;
      const _CharT*			_M_month11;
      const _CharT*			_M_month12;

      // Abbreviated month names, starting with "C"'s Jan.
      const _CharT*			_M_amonth01;
      const _CharT*			_M_amonth02;
      const _CharT*			_M_amonth03;
      const _CharT*			_M_amonth04;
      const _CharT*			_M_amonth05;
      const _CharT*			_M_amonth06;
      const _CharT*			_M_amonth07;
      const _CharT*			_M_amonth08;
      const _CharT*			_M_amonth09;
      const _CharT*			_M_amonth10;
      const _CharT*			_M_amonth11;
      const _CharT*			_M_amonth12;

      bool				_M_allocated;

      __timepunct_cache(size_t __refs = 0) : facet(__refs),
      _M_date_format(0), _M_date_era_format(0), _M_time_format(0),
      _M_time_era_format(0), _M_date_time_format(0),
      _M_date_time_era_format(0), _M_am(0), _M_pm(0),
      _M_am_pm_format(0), _M_day1(0), _M_day2(0), _M_day3(0),
      _M_day4(0), _M_day5(0), _M_day6(0), _M_day7(0),
      _M_aday1(0), _M_aday2(0), _M_aday3(0), _M_aday4(0),
      _M_aday5(0), _M_aday6(0), _M_aday7(0), _M_month01(0),
      _M_month02(0), _M_month03(0), _M_month04(0), _M_month05(0),
      _M_month06(0), _M_month07(0), _M_month08(0), _M_month09(0),
      _M_month10(0), _M_month11(0), _M_month12(0), _M_amonth01(0),
      _M_amonth02(0), _M_amonth03(0), _M_amonth04(0),
      _M_amonth05(0), _M_amonth06(0), _M_amonth07(0),
      _M_amonth08(0), _M_amonth09(0), _M_amonth10(0),
      _M_amonth11(0), _M_amonth12(0), _M_allocated(false)
      { }

      ~__timepunct_cache();

    private:
      __timepunct_cache&
      operator=(const __timepunct_cache&);
      
      explicit
      __timepunct_cache(const __timepunct_cache&);
    };

  template<typename _CharT>
    __timepunct_cache<_CharT>::~__timepunct_cache()
    {
      if (_M_allocated)
	{
	  // Unused.
	}
    }

  // Specializations.
  template<>
    const char*
    __timepunct_cache<char>::_S_timezones[14];

#ifdef _GLIBCXX_USE_WCHAR_T
  template<>
    const wchar_t*
    __timepunct_cache<wchar_t>::_S_timezones[14];
#endif

  // Generic.
  template<typename _CharT>
    const _CharT* __timepunct_cache<_CharT>::_S_timezones[14];

  template<typename _CharT>
    class __timepunct : public locale::facet
    {
    public:
      // Types:
      typedef _CharT			__char_type;
      typedef __timepunct_cache<_CharT>	__cache_type;

    protected:
      __cache_type*			_M_data;
      __c_locale			_M_c_locale_timepunct;
      const char*			_M_name_timepunct;

    public:
      /// Numpunct facet id.
      static locale::id			id;

      explicit
      __timepunct(size_t __refs = 0);

      explicit
      __timepunct(__cache_type* __cache, size_t __refs = 0);

      /**
       *  @brief  Internal constructor. Not for general use.
       *
       *  This is a constructor for use by the library itself to set up new
       *  locales.
       *
       *  @param __cloc  The C locale.
       *  @param __s  The name of a locale.
       *  @param refs  Passed to the base facet class.
      */
      explicit
      __timepunct(__c_locale __cloc, const char* __s, size_t __refs = 0);

      // FIXME: for error checking purposes _M_put should return the return
      // value of strftime/wcsftime.
      void
      _M_put(_CharT* __s, size_t __maxlen, const _CharT* __format,
	     const tm* __tm) const throw ();

      void
      _M_date_formats(const _CharT** __date) const
      {
	// Always have default first.
	__date[0] = _M_data->_M_date_format;
	__date[1] = _M_data->_M_date_era_format;
      }

      void
      _M_time_formats(const _CharT** __time) const
      {
	// Always have default first.
	__time[0] = _M_data->_M_time_format;
	__time[1] = _M_data->_M_time_era_format;
      }

      void
      _M_date_time_formats(const _CharT** __dt) const
      {
	// Always have default first.
	__dt[0] = _M_data->_M_date_time_format;
	__dt[1] = _M_data->_M_date_time_era_format;
      }

#if !_GLIBCXX_INLINE_VERSION
      void
      _M_am_pm_format(const _CharT*) const
      { /* Kept for ABI compatibility, see PR65927 */ }
#endif

      void
      _M_am_pm(const _CharT** __ampm) const
      {
	__ampm[0] = _M_data->_M_am;
	__ampm[1] = _M_data->_M_pm;
      }

      void
      _M_days(const _CharT** __days) const
      {
	__days[0] = _M_data->_M_day1;
	__days[1] = _M_data->_M_day2;
	__days[2] = _M_data->_M_day3;
	__days[3] = _M_data->_M_day4;
	__days[4] = _M_data->_M_day5;
	__days[5] = _M_data->_M_day6;
	__days[6] = _M_data->_M_day7;
      }

      void
      _M_days_abbreviated(const _CharT** __days) const
      {
	__days[0] = _M_data->_M_aday1;
	__days[1] = _M_data->_M_aday2;
	__days[2] = _M_data->_M_aday3;
	__days[3] = _M_data->_M_aday4;
	__days[4] = _M_data->_M_aday5;
	__days[5] = _M_data->_M_aday6;
	__days[6] = _M_data->_M_aday7;
      }

      void
      _M_months(const _CharT** __months) const
      {
	__months[0] = _M_data->_M_month01;
	__months[1] = _M_data->_M_month02;
	__months[2] = _M_data->_M_month03;
	__months[3] = _M_data->_M_month04;
	__months[4] = _M_data->_M_month05;
	__months[5] = _M_data->_M_month06;
	__months[6] = _M_data->_M_month07;
	__months[7] = _M_data->_M_month08;
	__months[8] = _M_data->_M_month09;
	__months[9] = _M_data->_M_month10;
	__months[10] = _M_data->_M_month11;
	__months[11] = _M_data->_M_month12;
      }

      void
      _M_months_abbreviated(const _CharT** __months) const
      {
	__months[0] = _M_data->_M_amonth01;
	__months[1] = _M_data->_M_amonth02;
	__months[2] = _M_data->_M_amonth03;
	__months[3] = _M_data->_M_amonth04;
	__months[4] = _M_data->_M_amonth05;
	__months[5] = _M_data->_M_amonth06;
	__months[6] = _M_data->_M_amonth07;
	__months[7] = _M_data->_M_amonth08;
	__months[8] = _M_data->_M_amonth09;
	__months[9] = _M_data->_M_amonth10;
	__months[10] = _M_data->_M_amonth11;
	__months[11] = _M_data->_M_amonth12;
      }

    protected:
      virtual
      ~__timepunct();

      // For use at construction time only.
      void
      _M_initialize_timepunct(__c_locale __cloc = 0);
    };

  template<typename _CharT>
    locale::id __timepunct<_CharT>::id;

  // Specializations.
  template<>
    void
    __timepunct<char>::_M_initialize_timepunct(__c_locale __cloc);

  template<>
    void
    __timepunct<char>::_M_put(char*, size_t, const char*, const tm*) const throw ();

#ifdef _GLIBCXX_USE_WCHAR_T
  template<>
    void
    __timepunct<wchar_t>::_M_initialize_timepunct(__c_locale __cloc);

  template<>
    void
    __timepunct<wchar_t>::_M_put(wchar_t*, size_t, const wchar_t*,
				 const tm*) const throw ();
#endif

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

  // Include host and configuration specific timepunct functions.
  #include <bits/time_members.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

_GLIBCXX_BEGIN_NAMESPACE_CXX11

  /**
   *  @brief  Primary class template time_get.
   *  @ingroup locales
   *
   *  This facet encapsulates the code to parse and return a date or
   *  time from a string.  It is used by the istream numeric
   *  extraction operators.
   *
   *  The time_get template uses protected virtual functions to provide the
   *  actual results.  The public accessors forward the call to the virtual
   *  functions.  These virtual functions are hooks for developers to
   *  implement the behavior they require from the time_get facet.
  */
  template<typename _CharT, typename _InIter>
    class time_get : public locale::facet, public time_base
    {
    public:
      // Types:
      //@{
      /// Public typedefs
      typedef _CharT			char_type;
      typedef _InIter			iter_type;
      //@}

      /// Numpunct facet id.
      static locale::id			id;

      /**
       *  @brief  Constructor performs initialization.
       *
       *  This is the constructor provided by the standard.
       *
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      time_get(size_t __refs = 0)
      : facet (__refs) { }

      /**
       *  @brief  Return preferred order of month, day, and year.
       *
       *  This function returns an enum from time_base::dateorder giving the
       *  preferred ordering if the format @a x given to time_put::put() only
       *  uses month, day, and year.  If the format @a x for the associated
       *  locale uses other fields, this function returns
       *  time_base::dateorder::noorder.
       *
       *  NOTE: The library always returns noorder at the moment.
       *
       *  @return  A member of time_base::dateorder.
      */
      dateorder
      date_order()  const
      { return this->do_date_order(); }

      /**
       *  @brief  Parse input time string.
       *
       *  This function parses a time according to the format @a X and puts the
       *  results into a user-supplied struct tm.  The result is returned by
       *  calling time_get::do_get_time().
       *
       *  If there is a valid time string according to format @a X, @a tm will
       *  be filled in accordingly and the returned iterator will point to the
       *  first character beyond the time string.  If an error occurs before
       *  the end, err |= ios_base::failbit.  If parsing reads all the
       *  characters, err |= ios_base::eofbit.
       *
       *  @param  __beg  Start of string to parse.
       *  @param  __end  End of string to parse.
       *  @param  __io  Source of the locale.
       *  @param  __err  Error flags to set.
       *  @param  __tm  Pointer to struct tm to fill in.
       *  @return  Iterator to first char beyond time string.
      */
      iter_type
      get_time(iter_type __beg, iter_type __end, ios_base& __io,
	       ios_base::iostate& __err, tm* __tm)  const
      { return this->do_get_time(__beg, __end, __io, __err, __tm); }

      /**
       *  @brief  Parse input date string.
       *
       *  This function parses a date according to the format @a x and puts the
       *  results into a user-supplied struct tm.  The result is returned by
       *  calling time_get::do_get_date().
       *
       *  If there is a valid date string according to format @a x, @a tm will
       *  be filled in accordingly and the returned iterator will point to the
       *  first character beyond the date string.  If an error occurs before
       *  the end, err |= ios_base::failbit.  If parsing reads all the
       *  characters, err |= ios_base::eofbit.
       *
       *  @param  __beg  Start of string to parse.
       *  @param  __end  End of string to parse.
       *  @param  __io  Source of the locale.
       *  @param  __err  Error flags to set.
       *  @param  __tm  Pointer to struct tm to fill in.
       *  @return  Iterator to first char beyond date string.
      */
      iter_type
      get_date(iter_type __beg, iter_type __end, ios_base& __io,
	       ios_base::iostate& __err, tm* __tm)  const
      { return this->do_get_date(__beg, __end, __io, __err, __tm); }

      /**
       *  @brief  Parse input weekday string.
       *
       *  This function parses a weekday name and puts the results into a
       *  user-supplied struct tm.  The result is returned by calling
       *  time_get::do_get_weekday().
       *
       *  Parsing starts by parsing an abbreviated weekday name.  If a valid
       *  abbreviation is followed by a character that would lead to the full
       *  weekday name, parsing continues until the full name is found or an
       *  error occurs.  Otherwise parsing finishes at the end of the
       *  abbreviated name.
       *
       *  If an error occurs before the end, err |= ios_base::failbit.  If
       *  parsing reads all the characters, err |= ios_base::eofbit.
       *
       *  @param  __beg  Start of string to parse.
       *  @param  __end  End of string to parse.
       *  @param  __io  Source of the locale.
       *  @param  __err  Error flags to set.
       *  @param  __tm  Pointer to struct tm to fill in.
       *  @return  Iterator to first char beyond weekday name.
      */
      iter_type
      get_weekday(iter_type __beg, iter_type __end, ios_base& __io,
		  ios_base::iostate& __err, tm* __tm) const
      { return this->do_get_weekday(__beg, __end, __io, __err, __tm); }

      /**
       *  @brief  Parse input month string.
       *
       *  This function parses a month name and puts the results into a
       *  user-supplied struct tm.  The result is returned by calling
       *  time_get::do_get_monthname().
       *
       *  Parsing starts by parsing an abbreviated month name.  If a valid
       *  abbreviation is followed by a character that would lead to the full
       *  month name, parsing continues until the full name is found or an
       *  error occurs.  Otherwise parsing finishes at the end of the
       *  abbreviated name.
       *
       *  If an error occurs before the end, err |= ios_base::failbit.  If
       *  parsing reads all the characters, err |=
       *  ios_base::eofbit.
       *
       *  @param  __beg  Start of string to parse.
       *  @param  __end  End of string to parse.
       *  @param  __io  Source of the locale.
       *  @param  __err  Error flags to set.
       *  @param  __tm  Pointer to struct tm to fill in.
       *  @return  Iterator to first char beyond month name.
      */
      iter_type
      get_monthname(iter_type __beg, iter_type __end, ios_base& __io,
		    ios_base::iostate& __err, tm* __tm) const
      { return this->do_get_monthname(__beg, __end, __io, __err, __tm); }

      /**
       *  @brief  Parse input year string.
       *
       *  This function reads up to 4 characters to parse a year string and
       *  puts the results into a user-supplied struct tm.  The result is
       *  returned by calling time_get::do_get_year().
       *
       *  4 consecutive digits are interpreted as a full year.  If there are
       *  exactly 2 consecutive digits, the library interprets this as the
       *  number of years since 1900.
       *
       *  If an error occurs before the end, err |= ios_base::failbit.  If
       *  parsing reads all the characters, err |= ios_base::eofbit.
       *
       *  @param  __beg  Start of string to parse.
       *  @param  __end  End of string to parse.
       *  @param  __io  Source of the locale.
       *  @param  __err  Error flags to set.
       *  @param  __tm  Pointer to struct tm to fill in.
       *  @return  Iterator to first char beyond year.
      */
      iter_type
      get_year(iter_type __beg, iter_type __end, ios_base& __io,
	       ios_base::iostate& __err, tm* __tm) const
      { return this->do_get_year(__beg, __end, __io, __err, __tm); }

#if __cplusplus >= 201103L
      /**
       *  @brief  Parse input string according to format.
       *
       *  This function calls time_get::do_get with the provided
       *  parameters.  @see do_get() and get().
       *
       *  @param __s        Start of string to parse.
       *  @param __end      End of string to parse.
       *  @param __io       Source of the locale.
       *  @param __err      Error flags to set.
       *  @param __tm       Pointer to struct tm to fill in.
       *  @param __format   Format specifier.
       *  @param __modifier Format modifier.
       *  @return  Iterator to first char not parsed.
       */
      inline
      iter_type get(iter_type __s, iter_type __end, ios_base& __io,
                    ios_base::iostate& __err, tm* __tm, char __format,
                    char __modifier = 0) const
      {
        return this->do_get(__s, __end, __io, __err, __tm, __format,
                            __modifier);
      }

      /**
       *  @brief  Parse input string according to format.
       *
       *  This function parses the input string according to a
       *  provided format string.  It does the inverse of
       *  time_put::put.  The format string follows the format
       *  specified for strftime(3)/strptime(3).  The actual parsing
       *  is done by time_get::do_get.
       *
       *  @param __s        Start of string to parse.
       *  @param __end      End of string to parse.
       *  @param __io       Source of the locale.
       *  @param __err      Error flags to set.
       *  @param __tm       Pointer to struct tm to fill in.
       *  @param __fmt      Start of the format string.
       *  @param __fmtend   End of the format string.
       *  @return  Iterator to first char not parsed.
       */
      iter_type get(iter_type __s, iter_type __end, ios_base& __io,
                    ios_base::iostate& __err, tm* __tm, const char_type* __fmt,
                    const char_type* __fmtend) const;
#endif // __cplusplus >= 201103L

    protected:
      /// Destructor.
      virtual
      ~time_get() { }

      /**
       *  @brief  Return preferred order of month, day, and year.
       *
       *  This function returns an enum from time_base::dateorder giving the
       *  preferred ordering if the format @a x given to time_put::put() only
       *  uses month, day, and year.  This function is a hook for derived
       *  classes to change the value returned.
       *
       *  @return  A member of time_base::dateorder.
      */
      virtual dateorder
      do_date_order() const;

      /**
       *  @brief  Parse input time string.
       *
       *  This function parses a time according to the format @a x and puts the
       *  results into a user-supplied struct tm.  This function is a hook for
       *  derived classes to change the value returned.  @see get_time() for
       *  details.
       *
       *  @param  __beg  Start of string to parse.
       *  @param  __end  End of string to parse.
       *  @param  __io  Source of the locale.
       *  @param  __err  Error flags to set.
       *  @param  __tm  Pointer to struct tm to fill in.
       *  @return  Iterator to first char beyond time string.
      */
      virtual iter_type
      do_get_time(iter_type __beg, iter_type __end, ios_base& __io,
		  ios_base::iostate& __err, tm* __tm) const;

      /**
       *  @brief  Parse input date string.
       *
       *  This function parses a date according to the format @a X and puts the
       *  results into a user-supplied struct tm.  This function is a hook for
       *  derived classes to change the value returned.  @see get_date() for
       *  details.
       *
       *  @param  __beg  Start of string to parse.
       *  @param  __end  End of string to parse.
       *  @param  __io  Source of the locale.
       *  @param  __err  Error flags to set.
       *  @param  __tm  Pointer to struct tm to fill in.
       *  @return  Iterator to first char beyond date string.
      */
      virtual iter_type
      do_get_date(iter_type __beg, iter_type __end, ios_base& __io,
		  ios_base::iostate& __err, tm* __tm) const;

      /**
       *  @brief  Parse input weekday string.
       *
       *  This function parses a weekday name and puts the results into a
       *  user-supplied struct tm.  This function is a hook for derived
       *  classes to change the value returned.  @see get_weekday() for
       *  details.
       *
       *  @param  __beg  Start of string to parse.
       *  @param  __end  End of string to parse.
       *  @param  __io  Source of the locale.
       *  @param  __err  Error flags to set.
       *  @param  __tm  Pointer to struct tm to fill in.
       *  @return  Iterator to first char beyond weekday name.
      */
      virtual iter_type
      do_get_weekday(iter_type __beg, iter_type __end, ios_base&,
		     ios_base::iostate& __err, tm* __tm) const;

      /**
       *  @brief  Parse input month string.
       *
       *  This function parses a month name and puts the results into a
       *  user-supplied struct tm.  This function is a hook for derived
       *  classes to change the value returned.  @see get_monthname() for
       *  details.
       *
       *  @param  __beg  Start of string to parse.
       *  @param  __end  End of string to parse.
       *  @param  __io  Source of the locale.
       *  @param  __err  Error flags to set.
       *  @param  __tm  Pointer to struct tm to fill in.
       *  @return  Iterator to first char beyond month name.
      */
      virtual iter_type
      do_get_monthname(iter_type __beg, iter_type __end, ios_base&,
		       ios_base::iostate& __err, tm* __tm) const;

      /**
       *  @brief  Parse input year string.
       *
       *  This function reads up to 4 characters to parse a year string and
       *  puts the results into a user-supplied struct tm.  This function is a
       *  hook for derived classes to change the value returned.  @see
       *  get_year() for details.
       *
       *  @param  __beg  Start of string to parse.
       *  @param  __end  End of string to parse.
       *  @param  __io  Source of the locale.
       *  @param  __err  Error flags to set.
       *  @param  __tm  Pointer to struct tm to fill in.
       *  @return  Iterator to first char beyond year.
      */
      virtual iter_type
      do_get_year(iter_type __beg, iter_type __end, ios_base& __io,
		  ios_base::iostate& __err, tm* __tm) const;

#if __cplusplus >= 201103L
      /**
       *  @brief  Parse input string according to format.
       *
       *  This function parses the string according to the provided
       *  format and optional modifier.  This function is a hook for
       *  derived classes to change the value returned.  @see get()
       *  for more details.
       *
       *  @param __s        Start of string to parse.
       *  @param __end      End of string to parse.
       *  @param __f        Source of the locale.
       *  @param __err      Error flags to set.
       *  @param __tm       Pointer to struct tm to fill in.
       *  @param __format   Format specifier.
       *  @param __modifier Format modifier.
       *  @return  Iterator to first char not parsed.
       */
#if _GLIBCXX_USE_CXX11_ABI
      virtual
#endif
      iter_type
      do_get(iter_type __s, iter_type __end, ios_base& __f,
             ios_base::iostate& __err, tm* __tm,
             char __format, char __modifier) const;
#endif // __cplusplus >= 201103L

      // Extract numeric component of length __len.
      iter_type
      _M_extract_num(iter_type __beg, iter_type __end, int& __member,
		     int __min, int __max, size_t __len,
		     ios_base& __io, ios_base::iostate& __err) const;

      // Extract any unique array of string literals in a const _CharT* array.
      iter_type
      _M_extract_name(iter_type __beg, iter_type __end, int& __member,
		      const _CharT** __names, size_t __indexlen,
		      ios_base& __io, ios_base::iostate& __err) const;

      // Extract day or month name in a const _CharT* array.
      iter_type
      _M_extract_wday_or_month(iter_type __beg, iter_type __end, int& __member,
			       const _CharT** __names, size_t __indexlen,
			       ios_base& __io, ios_base::iostate& __err) const;

      // Extract on a component-by-component basis, via __format argument.
      iter_type
      _M_extract_via_format(iter_type __beg, iter_type __end, ios_base& __io,
			    ios_base::iostate& __err, tm* __tm,
			    const _CharT* __format) const;
    };

  template<typename _CharT, typename _InIter>
    locale::id time_get<_CharT, _InIter>::id;

  /// class time_get_byname [22.2.5.2].
  template<typename _CharT, typename _InIter>
    class time_get_byname : public time_get<_CharT, _InIter>
    {
    public:
      // Types:
      typedef _CharT			char_type;
      typedef _InIter			iter_type;

      explicit
      time_get_byname(const char*, size_t __refs = 0)
      : time_get<_CharT, _InIter>(__refs) { }

#if __cplusplus >= 201103L
      explicit
      time_get_byname(const string& __s, size_t __refs = 0)
      : time_get_byname(__s.c_str(), __refs) { }
#endif

    protected:
      virtual
      ~time_get_byname() { }
    };

_GLIBCXX_END_NAMESPACE_CXX11

  /**
   *  @brief  Primary class template time_put.
   *  @ingroup locales
   *
   *  This facet encapsulates the code to format and output dates and times
   *  according to formats used by strftime().
   *
   *  The time_put template uses protected virtual functions to provide the
   *  actual results.  The public accessors forward the call to the virtual
   *  functions.  These virtual functions are hooks for developers to
   *  implement the behavior they require from the time_put facet.
  */
  template<typename _CharT, typename _OutIter>
    class time_put : public locale::facet
    {
    public:
      // Types:
      //@{
      /// Public typedefs
      typedef _CharT			char_type;
      typedef _OutIter			iter_type;
      //@}

      /// Numpunct facet id.
      static locale::id			id;

      /**
       *  @brief  Constructor performs initialization.
       *
       *  This is the constructor provided by the standard.
       *
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      time_put(size_t __refs = 0)
      : facet(__refs) { }

      /**
       *  @brief  Format and output a time or date.
       *
       *  This function formats the data in struct tm according to the
       *  provided format string.  The format string is interpreted as by
       *  strftime().
       *
       *  @param  __s  The stream to write to.
       *  @param  __io  Source of locale.
       *  @param  __fill  char_type to use for padding.
       *  @param  __tm  Struct tm with date and time info to format.
       *  @param  __beg  Start of format string.
       *  @param  __end  End of format string.
       *  @return  Iterator after writing.
       */
      iter_type
      put(iter_type __s, ios_base& __io, char_type __fill, const tm* __tm,
	  const _CharT* __beg, const _CharT* __end) const;

      /**
       *  @brief  Format and output a time or date.
       *
       *  This function formats the data in struct tm according to the
       *  provided format char and optional modifier.  The format and modifier
       *  are interpreted as by strftime().  It does so by returning
       *  time_put::do_put().
       *
       *  @param  __s  The stream to write to.
       *  @param  __io  Source of locale.
       *  @param  __fill  char_type to use for padding.
       *  @param  __tm  Struct tm with date and time info to format.
       *  @param  __format  Format char.
       *  @param  __mod  Optional modifier char.
       *  @return  Iterator after writing.
       */
      iter_type
      put(iter_type __s, ios_base& __io, char_type __fill,
	  const tm* __tm, char __format, char __mod = 0) const
      { return this->do_put(__s, __io, __fill, __tm, __format, __mod); }

    protected:
      /// Destructor.
      virtual
      ~time_put()
      { }

      /**
       *  @brief  Format and output a time or date.
       *
       *  This function formats the data in struct tm according to the
       *  provided format char and optional modifier.  This function is a hook
       *  for derived classes to change the value returned.  @see put() for
       *  more details.
       *
       *  @param  __s  The stream to write to.
       *  @param  __io  Source of locale.
       *  @param  __fill  char_type to use for padding.
       *  @param  __tm  Struct tm with date and time info to format.
       *  @param  __format  Format char.
       *  @param  __mod  Optional modifier char.
       *  @return  Iterator after writing.
       */
      virtual iter_type
      do_put(iter_type __s, ios_base& __io, char_type __fill, const tm* __tm,
	     char __format, char __mod) const;
    };

  template<typename _CharT, typename _OutIter>
    locale::id time_put<_CharT, _OutIter>::id;

  /// class time_put_byname [22.2.5.4].
  template<typename _CharT, typename _OutIter>
    class time_put_byname : public time_put<_CharT, _OutIter>
    {
    public:
      // Types:
      typedef _CharT			char_type;
      typedef _OutIter			iter_type;

      explicit
      time_put_byname(const char*, size_t __refs = 0)
      : time_put<_CharT, _OutIter>(__refs)
      { }

#if __cplusplus >= 201103L
      explicit
      time_put_byname(const string& __s, size_t __refs = 0)
      : time_put_byname(__s.c_str(), __refs) { }
#endif

    protected:
      virtual
      ~time_put_byname() { }
    };


  /**
   *  @brief  Money format ordering data.
   *  @ingroup locales
   *
   *  This class contains an ordered array of 4 fields to represent the
   *  pattern for formatting a money amount.  Each field may contain one entry
   *  from the part enum.  symbol, sign, and value must be present and the
   *  remaining field must contain either none or space.  @see
   *  moneypunct::pos_format() and moneypunct::neg_format() for details of how
   *  these fields are interpreted.
  */
  class money_base
  {
  public:
    enum part { none, space, symbol, sign, value };
    struct pattern { char field[4]; };

    static const pattern _S_default_pattern;

    enum
    {
      _S_minus,
      _S_zero,
      _S_end = 11
    };

    // String literal of acceptable (narrow) input/output, for
    // money_get/money_put. "-0123456789"
    static const char* _S_atoms;

    // Construct and return valid pattern consisting of some combination of:
    // space none symbol sign value
    _GLIBCXX_CONST static pattern
    _S_construct_pattern(char __precedes, char __space, char __posn) throw ();
  };

  template<typename _CharT, bool _Intl>
    struct __moneypunct_cache : public locale::facet
    {
      const char*			_M_grouping;
      size_t                            _M_grouping_size;
      bool				_M_use_grouping;
      _CharT				_M_decimal_point;
      _CharT				_M_thousands_sep;
      const _CharT*			_M_curr_symbol;
      size_t                            _M_curr_symbol_size;
      const _CharT*			_M_positive_sign;
      size_t                            _M_positive_sign_size;
      const _CharT*			_M_negative_sign;
      size_t                            _M_negative_sign_size;
      int				_M_frac_digits;
      money_base::pattern		_M_pos_format;
      money_base::pattern	        _M_neg_format;

      // A list of valid numeric literals for input and output: in the standard
      // "C" locale, this is "-0123456789". This array contains the chars after
      // having been passed through the current locale's ctype<_CharT>.widen().
      _CharT				_M_atoms[money_base::_S_end];

      bool				_M_allocated;

      __moneypunct_cache(size_t __refs = 0) : facet(__refs),
      _M_grouping(0), _M_grouping_size(0), _M_use_grouping(false),
      _M_decimal_point(_CharT()), _M_thousands_sep(_CharT()),
      _M_curr_symbol(0), _M_curr_symbol_size(0),
      _M_positive_sign(0), _M_positive_sign_size(0),
      _M_negative_sign(0), _M_negative_sign_size(0),
      _M_frac_digits(0),
      _M_pos_format(money_base::pattern()),
      _M_neg_format(money_base::pattern()), _M_allocated(false)
      { }

      ~__moneypunct_cache();

      void
      _M_cache(const locale& __loc);

    private:
      __moneypunct_cache&
      operator=(const __moneypunct_cache&);
      
      explicit
      __moneypunct_cache(const __moneypunct_cache&);
    };

  template<typename _CharT, bool _Intl>
    __moneypunct_cache<_CharT, _Intl>::~__moneypunct_cache()
    {
      if (_M_allocated)
	{
	  delete [] _M_grouping;
	  delete [] _M_curr_symbol;
	  delete [] _M_positive_sign;
	  delete [] _M_negative_sign;
	}
    }

_GLIBCXX_BEGIN_NAMESPACE_CXX11

  /**
   *  @brief  Primary class template moneypunct.
   *  @ingroup locales
   *
   *  This facet encapsulates the punctuation, grouping and other formatting
   *  features of money amount string representations.
  */
  template<typename _CharT, bool _Intl>
    class moneypunct : public locale::facet, public money_base
    {
    public:
      // Types:
      //@{
      /// Public typedefs
      typedef _CharT			char_type;
      typedef basic_string<_CharT>	string_type;
      //@}
      typedef __moneypunct_cache<_CharT, _Intl>     __cache_type;

    private:
      __cache_type*			_M_data;

    public:
      /// This value is provided by the standard, but no reason for its
      /// existence.
      static const bool			intl = _Intl;
      /// Numpunct facet id.
      static locale::id			id;

      /**
       *  @brief  Constructor performs initialization.
       *
       *  This is the constructor provided by the standard.
       *
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      moneypunct(size_t __refs = 0)
      : facet(__refs), _M_data(0)
      { _M_initialize_moneypunct(); }

      /**
       *  @brief  Constructor performs initialization.
       *
       *  This is an internal constructor.
       *
       *  @param __cache  Cache for optimization.
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      moneypunct(__cache_type* __cache, size_t __refs = 0)
      : facet(__refs), _M_data(__cache)
      { _M_initialize_moneypunct(); }

      /**
       *  @brief  Internal constructor. Not for general use.
       *
       *  This is a constructor for use by the library itself to set up new
       *  locales.
       *
       *  @param __cloc  The C locale.
       *  @param __s  The name of a locale.
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      moneypunct(__c_locale __cloc, const char* __s, size_t __refs = 0)
      : facet(__refs), _M_data(0)
      { _M_initialize_moneypunct(__cloc, __s); }

      /**
       *  @brief  Return decimal point character.
       *
       *  This function returns a char_type to use as a decimal point.  It
       *  does so by returning returning
       *  moneypunct<char_type>::do_decimal_point().
       *
       *  @return  @a char_type representing a decimal point.
      */
      char_type
      decimal_point() const
      { return this->do_decimal_point(); }

      /**
       *  @brief  Return thousands separator character.
       *
       *  This function returns a char_type to use as a thousands
       *  separator.  It does so by returning returning
       *  moneypunct<char_type>::do_thousands_sep().
       *
       *  @return  char_type representing a thousands separator.
      */
      char_type
      thousands_sep() const
      { return this->do_thousands_sep(); }

      /**
       *  @brief  Return grouping specification.
       *
       *  This function returns a string representing groupings for the
       *  integer part of an amount.  Groupings indicate where thousands
       *  separators should be inserted.
       *
       *  Each char in the return string is interpret as an integer rather
       *  than a character.  These numbers represent the number of digits in a
       *  group.  The first char in the string represents the number of digits
       *  in the least significant group.  If a char is negative, it indicates
       *  an unlimited number of digits for the group.  If more chars from the
       *  string are required to group a number, the last char is used
       *  repeatedly.
       *
       *  For example, if the grouping() returns <code>\003\002</code>
       *  and is applied to the number 123456789, this corresponds to
       *  12,34,56,789.  Note that if the string was <code>32</code>, this would
       *  put more than 50 digits into the least significant group if
       *  the character set is ASCII.
       *
       *  The string is returned by calling
       *  moneypunct<char_type>::do_grouping().
       *
       *  @return  string representing grouping specification.
      */
      string
      grouping() const
      { return this->do_grouping(); }

      /**
       *  @brief  Return currency symbol string.
       *
       *  This function returns a string_type to use as a currency symbol.  It
       *  does so by returning returning
       *  moneypunct<char_type>::do_curr_symbol().
       *
       *  @return  @a string_type representing a currency symbol.
      */
      string_type
      curr_symbol() const
      { return this->do_curr_symbol(); }

      /**
       *  @brief  Return positive sign string.
       *
       *  This function returns a string_type to use as a sign for positive
       *  amounts.  It does so by returning returning
       *  moneypunct<char_type>::do_positive_sign().
       *
       *  If the return value contains more than one character, the first
       *  character appears in the position indicated by pos_format() and the
       *  remainder appear at the end of the formatted string.
       *
       *  @return  @a string_type representing a positive sign.
      */
      string_type
      positive_sign() const
      { return this->do_positive_sign(); }

      /**
       *  @brief  Return negative sign string.
       *
       *  This function returns a string_type to use as a sign for negative
       *  amounts.  It does so by returning returning
       *  moneypunct<char_type>::do_negative_sign().
       *
       *  If the return value contains more than one character, the first
       *  character appears in the position indicated by neg_format() and the
       *  remainder appear at the end of the formatted string.
       *
       *  @return  @a string_type representing a negative sign.
      */
      string_type
      negative_sign() const
      { return this->do_negative_sign(); }

      /**
       *  @brief  Return number of digits in fraction.
       *
       *  This function returns the exact number of digits that make up the
       *  fractional part of a money amount.  It does so by returning
       *  returning moneypunct<char_type>::do_frac_digits().
       *
       *  The fractional part of a money amount is optional.  But if it is
       *  present, there must be frac_digits() digits.
       *
       *  @return  Number of digits in amount fraction.
      */
      int
      frac_digits() const
      { return this->do_frac_digits(); }

      //@{
      /**
       *  @brief  Return pattern for money values.
       *
       *  This function returns a pattern describing the formatting of a
       *  positive or negative valued money amount.  It does so by returning
       *  returning moneypunct<char_type>::do_pos_format() or
       *  moneypunct<char_type>::do_neg_format().
       *
       *  The pattern has 4 fields describing the ordering of symbol, sign,
       *  value, and none or space.  There must be one of each in the pattern.
       *  The none and space enums may not appear in the first field and space
       *  may not appear in the final field.
       *
       *  The parts of a money string must appear in the order indicated by
       *  the fields of the pattern.  The symbol field indicates that the
       *  value of curr_symbol() may be present.  The sign field indicates
       *  that the value of positive_sign() or negative_sign() must be
       *  present.  The value field indicates that the absolute value of the
       *  money amount is present.  none indicates 0 or more whitespace
       *  characters, except at the end, where it permits no whitespace.
       *  space indicates that 1 or more whitespace characters must be
       *  present.
       *
       *  For example, for the US locale and pos_format() pattern
       *  {symbol,sign,value,none}, curr_symbol() == &apos;$&apos;
       *  positive_sign() == &apos;+&apos;, and value 10.01, and
       *  options set to force the symbol, the corresponding string is
       *  <code>$+10.01</code>.
       *
       *  @return  Pattern for money values.
      */
      pattern
      pos_format() const
      { return this->do_pos_format(); }

      pattern
      neg_format() const
      { return this->do_neg_format(); }
      //@}

    protected:
      /// Destructor.
      virtual
      ~moneypunct();

      /**
       *  @brief  Return decimal point character.
       *
       *  Returns a char_type to use as a decimal point.  This function is a
       *  hook for derived classes to change the value returned.
       *
       *  @return  @a char_type representing a decimal point.
      */
      virtual char_type
      do_decimal_point() const
      { return _M_data->_M_decimal_point; }

      /**
       *  @brief  Return thousands separator character.
       *
       *  Returns a char_type to use as a thousands separator.  This function
       *  is a hook for derived classes to change the value returned.
       *
       *  @return  @a char_type representing a thousands separator.
      */
      virtual char_type
      do_thousands_sep() const
      { return _M_data->_M_thousands_sep; }

      /**
       *  @brief  Return grouping specification.
       *
       *  Returns a string representing groupings for the integer part of a
       *  number.  This function is a hook for derived classes to change the
       *  value returned.  @see grouping() for details.
       *
       *  @return  String representing grouping specification.
      */
      virtual string
      do_grouping() const
      { return _M_data->_M_grouping; }

      /**
       *  @brief  Return currency symbol string.
       *
       *  This function returns a string_type to use as a currency symbol.
       *  This function is a hook for derived classes to change the value
       *  returned.  @see curr_symbol() for details.
       *
       *  @return  @a string_type representing a currency symbol.
      */
      virtual string_type
      do_curr_symbol()   const
      { return _M_data->_M_curr_symbol; }

      /**
       *  @brief  Return positive sign string.
       *
       *  This function returns a string_type to use as a sign for positive
       *  amounts.  This function is a hook for derived classes to change the
       *  value returned.  @see positive_sign() for details.
       *
       *  @return  @a string_type representing a positive sign.
      */
      virtual string_type
      do_positive_sign() const
      { return _M_data->_M_positive_sign; }

      /**
       *  @brief  Return negative sign string.
       *
       *  This function returns a string_type to use as a sign for negative
       *  amounts.  This function is a hook for derived classes to change the
       *  value returned.  @see negative_sign() for details.
       *
       *  @return  @a string_type representing a negative sign.
      */
      virtual string_type
      do_negative_sign() const
      { return _M_data->_M_negative_sign; }

      /**
       *  @brief  Return number of digits in fraction.
       *
       *  This function returns the exact number of digits that make up the
       *  fractional part of a money amount.  This function is a hook for
       *  derived classes to change the value returned.  @see frac_digits()
       *  for details.
       *
       *  @return  Number of digits in amount fraction.
      */
      virtual int
      do_frac_digits() const
      { return _M_data->_M_frac_digits; }

      /**
       *  @brief  Return pattern for money values.
       *
       *  This function returns a pattern describing the formatting of a
       *  positive valued money amount.  This function is a hook for derived
       *  classes to change the value returned.  @see pos_format() for
       *  details.
       *
       *  @return  Pattern for money values.
      */
      virtual pattern
      do_pos_format() const
      { return _M_data->_M_pos_format; }

      /**
       *  @brief  Return pattern for money values.
       *
       *  This function returns a pattern describing the formatting of a
       *  negative valued money amount.  This function is a hook for derived
       *  classes to change the value returned.  @see neg_format() for
       *  details.
       *
       *  @return  Pattern for money values.
      */
      virtual pattern
      do_neg_format() const
      { return _M_data->_M_neg_format; }

      // For use at construction time only.
       void
       _M_initialize_moneypunct(__c_locale __cloc = 0,
				const char* __name = 0);
    };

  template<typename _CharT, bool _Intl>
    locale::id moneypunct<_CharT, _Intl>::id;

  template<typename _CharT, bool _Intl>
    const bool moneypunct<_CharT, _Intl>::intl;

  template<>
    moneypunct<char, true>::~moneypunct();

  template<>
    moneypunct<char, false>::~moneypunct();

  template<>
    void
    moneypunct<char, true>::_M_initialize_moneypunct(__c_locale, const char*);

  template<>
    void
    moneypunct<char, false>::_M_initialize_moneypunct(__c_locale, const char*);

#ifdef _GLIBCXX_USE_WCHAR_T
  template<>
    moneypunct<wchar_t, true>::~moneypunct();

  template<>
    moneypunct<wchar_t, false>::~moneypunct();

  template<>
    void
    moneypunct<wchar_t, true>::_M_initialize_moneypunct(__c_locale,
							const char*);

  template<>
    void
    moneypunct<wchar_t, false>::_M_initialize_moneypunct(__c_locale,
							 const char*);
#endif

  /// class moneypunct_byname [22.2.6.4].
  template<typename _CharT, bool _Intl>
    class moneypunct_byname : public moneypunct<_CharT, _Intl>
    {
    public:
      typedef _CharT			char_type;
      typedef basic_string<_CharT>	string_type;

      static const bool intl = _Intl;

      explicit
      moneypunct_byname(const char* __s, size_t __refs = 0)
      : moneypunct<_CharT, _Intl>(__refs)
      {
	if (__builtin_strcmp(__s, "C") != 0
	    && __builtin_strcmp(__s, "POSIX") != 0)
	  {
	    __c_locale __tmp;
	    this->_S_create_c_locale(__tmp, __s);
	    this->_M_initialize_moneypunct(__tmp);
	    this->_S_destroy_c_locale(__tmp);
	  }
      }

#if __cplusplus >= 201103L
      explicit
      moneypunct_byname(const string& __s, size_t __refs = 0)
      : moneypunct_byname(__s.c_str(), __refs) { }
#endif

    protected:
      virtual
      ~moneypunct_byname() { }
    };

  template<typename _CharT, bool _Intl>
    const bool moneypunct_byname<_CharT, _Intl>::intl;

_GLIBCXX_END_NAMESPACE_CXX11

_GLIBCXX_BEGIN_NAMESPACE_LDBL_OR_CXX11

  /**
   *  @brief  Primary class template money_get.
   *  @ingroup locales
   *
   *  This facet encapsulates the code to parse and return a monetary
   *  amount from a string.
   *
   *  The money_get template uses protected virtual functions to
   *  provide the actual results.  The public accessors forward the
   *  call to the virtual functions.  These virtual functions are
   *  hooks for developers to implement the behavior they require from
   *  the money_get facet.
  */
  template<typename _CharT, typename _InIter>
    class money_get : public locale::facet
    {
    public:
      // Types:
      //@{
      /// Public typedefs
      typedef _CharT			char_type;
      typedef _InIter			iter_type;
      typedef basic_string<_CharT>	string_type;
      //@}

      /// Numpunct facet id.
      static locale::id			id;

      /**
       *  @brief  Constructor performs initialization.
       *
       *  This is the constructor provided by the standard.
       *
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      money_get(size_t __refs = 0) : facet(__refs) { }

      /**
       *  @brief  Read and parse a monetary value.
       *
       *  This function reads characters from @a __s, interprets them as a
       *  monetary value according to moneypunct and ctype facets retrieved
       *  from io.getloc(), and returns the result in @a units as an integral
       *  value moneypunct::frac_digits() * the actual amount.  For example,
       *  the string $10.01 in a US locale would store 1001 in @a units.
       *
       *  Any characters not part of a valid money amount are not consumed.
       *
       *  If a money value cannot be parsed from the input stream, sets
       *  err=(err|io.failbit).  If the stream is consumed before finishing
       *  parsing,  sets err=(err|io.failbit|io.eofbit).  @a units is
       *  unchanged if parsing fails.
       *
       *  This function works by returning the result of do_get().
       *
       *  @param  __s  Start of characters to parse.
       *  @param  __end  End of characters to parse.
       *  @param  __intl  Parameter to use_facet<moneypunct<CharT,intl> >.
       *  @param  __io  Source of facets and io state.
       *  @param  __err  Error field to set if parsing fails.
       *  @param  __units  Place to store result of parsing.
       *  @return  Iterator referencing first character beyond valid money
       *	   amount.
       */
      iter_type
      get(iter_type __s, iter_type __end, bool __intl, ios_base& __io,
	  ios_base::iostate& __err, long double& __units) const
      { return this->do_get(__s, __end, __intl, __io, __err, __units); }

      /**
       *  @brief  Read and parse a monetary value.
       *
       *  This function reads characters from @a __s, interprets them as
       *  a monetary value according to moneypunct and ctype facets
       *  retrieved from io.getloc(), and returns the result in @a
       *  digits.  For example, the string $10.01 in a US locale would
       *  store <code>1001</code> in @a digits.
       *
       *  Any characters not part of a valid money amount are not consumed.
       *
       *  If a money value cannot be parsed from the input stream, sets
       *  err=(err|io.failbit).  If the stream is consumed before finishing
       *  parsing,  sets err=(err|io.failbit|io.eofbit).
       *
       *  This function works by returning the result of do_get().
       *
       *  @param  __s  Start of characters to parse.
       *  @param  __end  End of characters to parse.
       *  @param  __intl  Parameter to use_facet<moneypunct<CharT,intl> >.
       *  @param  __io  Source of facets and io state.
       *  @param  __err  Error field to set if parsing fails.
       *  @param  __digits  Place to store result of parsing.
       *  @return  Iterator referencing first character beyond valid money
       *	   amount.
       */
      iter_type
      get(iter_type __s, iter_type __end, bool __intl, ios_base& __io,
	  ios_base::iostate& __err, string_type& __digits) const
      { return this->do_get(__s, __end, __intl, __io, __err, __digits); }

    protected:
      /// Destructor.
      virtual
      ~money_get() { }

      /**
       *  @brief  Read and parse a monetary value.
       *
       *  This function reads and parses characters representing a monetary
       *  value.  This function is a hook for derived classes to change the
       *  value returned.  @see get() for details.
       */
      // XXX GLIBCXX_ABI Deprecated
#if defined _GLIBCXX_LONG_DOUBLE_COMPAT && defined __LONG_DOUBLE_128__ \
      && _GLIBCXX_USE_CXX11_ABI == 0
      virtual iter_type
      __do_get(iter_type __s, iter_type __end, bool __intl, ios_base& __io,
	       ios_base::iostate& __err, double& __units) const;
#else
      virtual iter_type
      do_get(iter_type __s, iter_type __end, bool __intl, ios_base& __io,
	     ios_base::iostate& __err, long double& __units) const;
#endif

      /**
       *  @brief  Read and parse a monetary value.
       *
       *  This function reads and parses characters representing a monetary
       *  value.  This function is a hook for derived classes to change the
       *  value returned.  @see get() for details.
       */
      virtual iter_type
      do_get(iter_type __s, iter_type __end, bool __intl, ios_base& __io,
	     ios_base::iostate& __err, string_type& __digits) const;

      // XXX GLIBCXX_ABI Deprecated
#if defined _GLIBCXX_LONG_DOUBLE_COMPAT && defined __LONG_DOUBLE_128__ \
      && _GLIBCXX_USE_CXX11_ABI == 0
      virtual iter_type
      do_get(iter_type __s, iter_type __end, bool __intl, ios_base& __io,
	     ios_base::iostate& __err, long double& __units) const;
#endif

      template<bool _Intl>
        iter_type
        _M_extract(iter_type __s, iter_type __end, ios_base& __io,
		   ios_base::iostate& __err, string& __digits) const;     
    };

  template<typename _CharT, typename _InIter>
    locale::id money_get<_CharT, _InIter>::id;

  /**
   *  @brief  Primary class template money_put.
   *  @ingroup locales
   *
   *  This facet encapsulates the code to format and output a monetary
   *  amount.
   *
   *  The money_put template uses protected virtual functions to
   *  provide the actual results.  The public accessors forward the
   *  call to the virtual functions.  These virtual functions are
   *  hooks for developers to implement the behavior they require from
   *  the money_put facet.
  */
  template<typename _CharT, typename _OutIter>
    class money_put : public locale::facet
    {
    public:
      //@{
      /// Public typedefs
      typedef _CharT			char_type;
      typedef _OutIter			iter_type;
      typedef basic_string<_CharT>	string_type;
      //@}

      /// Numpunct facet id.
      static locale::id			id;

      /**
       *  @brief  Constructor performs initialization.
       *
       *  This is the constructor provided by the standard.
       *
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      money_put(size_t __refs = 0) : facet(__refs) { }

      /**
       *  @brief  Format and output a monetary value.
       *
       *  This function formats @a units as a monetary value according to
       *  moneypunct and ctype facets retrieved from io.getloc(), and writes
       *  the resulting characters to @a __s.  For example, the value 1001 in a
       *  US locale would write <code>$10.01</code> to @a __s.
       *
       *  This function works by returning the result of do_put().
       *
       *  @param  __s  The stream to write to.
       *  @param  __intl  Parameter to use_facet<moneypunct<CharT,intl> >.
       *  @param  __io  Source of facets and io state.
       *  @param  __fill  char_type to use for padding.
       *  @param  __units  Place to store result of parsing.
       *  @return  Iterator after writing.
       */
      iter_type
      put(iter_type __s, bool __intl, ios_base& __io,
	  char_type __fill, long double __units) const
      { return this->do_put(__s, __intl, __io, __fill, __units); }

      /**
       *  @brief  Format and output a monetary value.
       *
       *  This function formats @a digits as a monetary value
       *  according to moneypunct and ctype facets retrieved from
       *  io.getloc(), and writes the resulting characters to @a __s.
       *  For example, the string <code>1001</code> in a US locale
       *  would write <code>$10.01</code> to @a __s.
       *
       *  This function works by returning the result of do_put().
       *
       *  @param  __s  The stream to write to.
       *  @param  __intl  Parameter to use_facet<moneypunct<CharT,intl> >.
       *  @param  __io  Source of facets and io state.
       *  @param  __fill  char_type to use for padding.
       *  @param  __digits  Place to store result of parsing.
       *  @return  Iterator after writing.
       */
      iter_type
      put(iter_type __s, bool __intl, ios_base& __io,
	  char_type __fill, const string_type& __digits) const
      { return this->do_put(__s, __intl, __io, __fill, __digits); }

    protected:
      /// Destructor.
      virtual
      ~money_put() { }

      /**
       *  @brief  Format and output a monetary value.
       *
       *  This function formats @a units as a monetary value according to
       *  moneypunct and ctype facets retrieved from io.getloc(), and writes
       *  the resulting characters to @a __s.  For example, the value 1001 in a
       *  US locale would write <code>$10.01</code> to @a __s.
       *
       *  This function is a hook for derived classes to change the value
       *  returned.  @see put().
       *
       *  @param  __s  The stream to write to.
       *  @param  __intl  Parameter to use_facet<moneypunct<CharT,intl> >.
       *  @param  __io  Source of facets and io state.
       *  @param  __fill  char_type to use for padding.
       *  @param  __units  Place to store result of parsing.
       *  @return  Iterator after writing.
       */
      // XXX GLIBCXX_ABI Deprecated
#if defined _GLIBCXX_LONG_DOUBLE_COMPAT && defined __LONG_DOUBLE_128__ \
      && _GLIBCXX_USE_CXX11_ABI == 0
      virtual iter_type
      __do_put(iter_type __s, bool __intl, ios_base& __io, char_type __fill,
	       double __units) const;
#else
      virtual iter_type
      do_put(iter_type __s, bool __intl, ios_base& __io, char_type __fill,
	     long double __units) const;
#endif

      /**
       *  @brief  Format and output a monetary value.
       *
       *  This function formats @a digits as a monetary value
       *  according to moneypunct and ctype facets retrieved from
       *  io.getloc(), and writes the resulting characters to @a __s.
       *  For example, the string <code>1001</code> in a US locale
       *  would write <code>$10.01</code> to @a __s.
       *
       *  This function is a hook for derived classes to change the value
       *  returned.  @see put().
       *
       *  @param  __s  The stream to write to.
       *  @param  __intl  Parameter to use_facet<moneypunct<CharT,intl> >.
       *  @param  __io  Source of facets and io state.
       *  @param  __fill  char_type to use for padding.
       *  @param  __digits  Place to store result of parsing.
       *  @return  Iterator after writing.
       */
      virtual iter_type
      do_put(iter_type __s, bool __intl, ios_base& __io, char_type __fill,
	     const string_type& __digits) const;

      // XXX GLIBCXX_ABI Deprecated
#if defined _GLIBCXX_LONG_DOUBLE_COMPAT && defined __LONG_DOUBLE_128__ \
      && _GLIBCXX_USE_CXX11_ABI == 0
      virtual iter_type
      do_put(iter_type __s, bool __intl, ios_base& __io, char_type __fill,
	     long double __units) const;
#endif

      template<bool _Intl>
        iter_type
        _M_insert(iter_type __s, ios_base& __io, char_type __fill,
		  const string_type& __digits) const;
    };

  template<typename _CharT, typename _OutIter>
    locale::id money_put<_CharT, _OutIter>::id;

_GLIBCXX_END_NAMESPACE_LDBL_OR_CXX11

  /**
   *  @brief  Messages facet base class providing catalog typedef.
   *  @ingroup locales
   */
  struct messages_base
  {
    typedef int catalog;
  };

_GLIBCXX_BEGIN_NAMESPACE_CXX11

  /**
   *  @brief  Primary class template messages.
   *  @ingroup locales
   *
   *  This facet encapsulates the code to retrieve messages from
   *  message catalogs.  The only thing defined by the standard for this facet
   *  is the interface.  All underlying functionality is
   *  implementation-defined.
   *
   *  This library currently implements 3 versions of the message facet.  The
   *  first version (gnu) is a wrapper around gettext, provided by libintl.
   *  The second version (ieee) is a wrapper around catgets.  The final
   *  version (default) does no actual translation.  These implementations are
   *  only provided for char and wchar_t instantiations.
   *
   *  The messages template uses protected virtual functions to
   *  provide the actual results.  The public accessors forward the
   *  call to the virtual functions.  These virtual functions are
   *  hooks for developers to implement the behavior they require from
   *  the messages facet.
  */
  template<typename _CharT>
    class messages : public locale::facet, public messages_base
    {
    public:
      // Types:
      //@{
      /// Public typedefs
      typedef _CharT			char_type;
      typedef basic_string<_CharT>	string_type;
      //@}

    protected:
      // Underlying "C" library locale information saved from
      // initialization, needed by messages_byname as well.
      __c_locale			_M_c_locale_messages;
      const char*			_M_name_messages;

    public:
      /// Numpunct facet id.
      static locale::id			id;

      /**
       *  @brief  Constructor performs initialization.
       *
       *  This is the constructor provided by the standard.
       *
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      messages(size_t __refs = 0);

      // Non-standard.
      /**
       *  @brief  Internal constructor.  Not for general use.
       *
       *  This is a constructor for use by the library itself to set up new
       *  locales.
       *
       *  @param  __cloc  The C locale.
       *  @param  __s  The name of a locale.
       *  @param  __refs  Refcount to pass to the base class.
       */
      explicit
      messages(__c_locale __cloc, const char* __s, size_t __refs = 0);

      /*
       *  @brief  Open a message catalog.
       *
       *  This function opens and returns a handle to a message catalog by
       *  returning do_open(__s, __loc).
       *
       *  @param  __s  The catalog to open.
       *  @param  __loc  Locale to use for character set conversions.
       *  @return  Handle to the catalog or value < 0 if open fails.
      */
      catalog
      open(const basic_string<char>& __s, const locale& __loc) const
      { return this->do_open(__s, __loc); }

      // Non-standard and unorthodox, yet effective.
      /*
       *  @brief  Open a message catalog.
       *
       *  This non-standard function opens and returns a handle to a message
       *  catalog by returning do_open(s, loc).  The third argument provides a
       *  message catalog root directory for gnu gettext and is ignored
       *  otherwise.
       *
       *  @param  __s  The catalog to open.
       *  @param  __loc  Locale to use for character set conversions.
       *  @param  __dir  Message catalog root directory.
       *  @return  Handle to the catalog or value < 0 if open fails.
      */
      catalog
      open(const basic_string<char>&, const locale&, const char*) const;

      /*
       *  @brief  Look up a string in a message catalog.
       *
       *  This function retrieves and returns a message from a catalog by
       *  returning do_get(c, set, msgid, s).
       *
       *  For gnu, @a __set and @a msgid are ignored.  Returns gettext(s).
       *  For default, returns s. For ieee, returns catgets(c,set,msgid,s).
       *
       *  @param  __c  The catalog to access.
       *  @param  __set  Implementation-defined.
       *  @param  __msgid  Implementation-defined.
       *  @param  __s  Default return value if retrieval fails.
       *  @return  Retrieved message or @a __s if get fails.
      */
      string_type
      get(catalog __c, int __set, int __msgid, const string_type& __s) const
      { return this->do_get(__c, __set, __msgid, __s); }

      /*
       *  @brief  Close a message catalog.
       *
       *  Closes catalog @a c by calling do_close(c).
       *
       *  @param  __c  The catalog to close.
      */
      void
      close(catalog __c) const
      { return this->do_close(__c); }

    protected:
      /// Destructor.
      virtual
      ~messages();

      /*
       *  @brief  Open a message catalog.
       *
       *  This function opens and returns a handle to a message catalog in an
       *  implementation-defined manner.  This function is a hook for derived
       *  classes to change the value returned.
       *
       *  @param  __s  The catalog to open.
       *  @param  __loc  Locale to use for character set conversions.
       *  @return  Handle to the opened catalog, value < 0 if open failed.
      */
      virtual catalog
      do_open(const basic_string<char>&, const locale&) const;

      /*
       *  @brief  Look up a string in a message catalog.
       *
       *  This function retrieves and returns a message from a catalog in an
       *  implementation-defined manner.  This function is a hook for derived
       *  classes to change the value returned.
       *
       *  For gnu, @a __set and @a __msgid are ignored.  Returns gettext(s).
       *  For default, returns s. For ieee, returns catgets(c,set,msgid,s).
       *
       *  @param  __c  The catalog to access.
       *  @param  __set  Implementation-defined.
       *  @param  __msgid  Implementation-defined.
       *  @param  __s  Default return value if retrieval fails.
       *  @return  Retrieved message or @a __s if get fails.
      */
      virtual string_type
      do_get(catalog, int, int, const string_type& __dfault) const;

      /*
       *  @brief  Close a message catalog.
       *
       *  @param  __c  The catalog to close.
      */
      virtual void
      do_close(catalog) const;

      // Returns a locale and codeset-converted string, given a char* message.
      char*
      _M_convert_to_char(const string_type& __msg) const
      {
	// XXX
	return reinterpret_cast<char*>(const_cast<_CharT*>(__msg.c_str()));
      }

      // Returns a locale and codeset-converted string, given a char* message.
      string_type
      _M_convert_from_char(char*) const
      {
	// XXX
	return string_type();
      }
     };

  template<typename _CharT>
    locale::id messages<_CharT>::id;

  /// Specializations for required instantiations.
  template<>
    string
    messages<char>::do_get(catalog, int, int, const string&) const;

#ifdef _GLIBCXX_USE_WCHAR_T
  template<>
    wstring
    messages<wchar_t>::do_get(catalog, int, int, const wstring&) const;
#endif

   /// class messages_byname [22.2.7.2].
   template<typename _CharT>
    class messages_byname : public messages<_CharT>
    {
    public:
      typedef _CharT			char_type;
      typedef basic_string<_CharT>	string_type;

      explicit
      messages_byname(const char* __s, size_t __refs = 0);

#if __cplusplus >= 201103L
      explicit
      messages_byname(const string& __s, size_t __refs = 0)
      : messages_byname(__s.c_str(), __refs) { }
#endif

    protected:
      virtual
      ~messages_byname()
      { }
    };

_GLIBCXX_END_NAMESPACE_CXX11

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

// Include host and configuration specific messages functions.
#include <bits/messages_members.h>

// 22.2.1.5  Template class codecvt
#include <bits/codecvt.h>

#include <bits/locale_facets_nonio.tcc>

#endif
