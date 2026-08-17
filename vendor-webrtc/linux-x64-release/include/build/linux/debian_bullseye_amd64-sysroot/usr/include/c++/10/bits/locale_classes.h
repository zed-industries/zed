// Locale support -*- C++ -*-

// Copyright (C) 1997-2020 Free Software Foundation, Inc.
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

/** @file bits/locale_classes.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{locale}
 */

//
// ISO C++ 14882: 22.1  Locales
//

#ifndef _LOCALE_CLASSES_H
#define _LOCALE_CLASSES_H 1

#pragma GCC system_header

#include <bits/localefwd.h>
#include <string>
#include <ext/atomicity.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  // 22.1.1 Class locale
  /**
   *  @brief  Container class for localization functionality.
   *  @ingroup locales
   *
   *  The locale class is first a class wrapper for C library locales.  It is
   *  also an extensible container for user-defined localization.  A locale is
   *  a collection of facets that implement various localization features such
   *  as money, time, and number printing.
   *
   *  Constructing C++ locales does not change the C library locale.
   *
   *  This library supports efficient construction and copying of locales
   *  through a reference counting implementation of the locale class.
  */
  class locale
  {
  public:
    // Types:
    /// Definition of locale::category.
    typedef int	category;

    // Forward decls and friends:
    class facet;
    class id;
    class _Impl;

    friend class facet;
    friend class _Impl;

    template<typename _Facet>
      friend bool
      has_facet(const locale&) throw();

    template<typename _Facet>
      friend const _Facet&
      use_facet(const locale&);

    template<typename _Cache>
      friend struct __use_cache;

    //@{
    /**
     *  @brief  Category values.
     *
     *  The standard category values are none, ctype, numeric, collate, time,
     *  monetary, and messages.  They form a bitmask that supports union and
     *  intersection.  The category all is the union of these values.
     *
     *  NB: Order must match _S_facet_categories definition in locale.cc
    */
    static const category none		= 0;
    static const category ctype		= 1L << 0;
    static const category numeric	= 1L << 1;
    static const category collate	= 1L << 2;
    static const category time		= 1L << 3;
    static const category monetary	= 1L << 4;
    static const category messages	= 1L << 5;
    static const category all		= (ctype | numeric | collate |
					   time  | monetary | messages);
    //@}

    // Construct/copy/destroy:

    /**
     *  @brief  Default constructor.
     *
     *  Constructs a copy of the global locale.  If no locale has been
     *  explicitly set, this is the C locale.
    */
    locale() throw();

    /**
     *  @brief  Copy constructor.
     *
     *  Constructs a copy of @a other.
     *
     *  @param  __other  The locale to copy.
    */
    locale(const locale& __other) throw();

    /**
     *  @brief  Named locale constructor.
     *
     *  Constructs a copy of the named C library locale.
     *
     *  @param  __s  Name of the locale to construct.
     *  @throw  std::runtime_error if __s is null or an undefined locale.
    */
    explicit
    locale(const char* __s);

    /**
     *  @brief  Construct locale with facets from another locale.
     *
     *  Constructs a copy of the locale @a base.  The facets specified by @a
     *  cat are replaced with those from the locale named by @a s.  If base is
     *  named, this locale instance will also be named.
     *
     *  @param  __base  The locale to copy.
     *  @param  __s  Name of the locale to use facets from.
     *  @param  __cat  Set of categories defining the facets to use from __s.
     *  @throw  std::runtime_error if __s is null or an undefined locale.
    */
    locale(const locale& __base, const char* __s, category __cat);

#if __cplusplus >= 201103L
    /**
     *  @brief  Named locale constructor.
     *
     *  Constructs a copy of the named C library locale.
     *
     *  @param  __s  Name of the locale to construct.
     *  @throw  std::runtime_error if __s is an undefined locale.
    */
    explicit
    locale(const std::string& __s) : locale(__s.c_str()) { }

    /**
     *  @brief  Construct locale with facets from another locale.
     *
     *  Constructs a copy of the locale @a base.  The facets specified by @a
     *  cat are replaced with those from the locale named by @a s.  If base is
     *  named, this locale instance will also be named.
     *
     *  @param  __base  The locale to copy.
     *  @param  __s  Name of the locale to use facets from.
     *  @param  __cat  Set of categories defining the facets to use from __s.
     *  @throw  std::runtime_error if __s is an undefined locale.
    */
    locale(const locale& __base, const std::string& __s, category __cat)
    : locale(__base, __s.c_str(), __cat) { }
#endif

    /**
     *  @brief  Construct locale with facets from another locale.
     *
     *  Constructs a copy of the locale @a base.  The facets specified by @a
     *  cat are replaced with those from the locale @a add.  If @a base and @a
     *  add are named, this locale instance will also be named.
     *
     *  @param  __base  The locale to copy.
     *  @param  __add  The locale to use facets from.
     *  @param  __cat  Set of categories defining the facets to use from add.
    */
    locale(const locale& __base, const locale& __add, category __cat);

    /**
     *  @brief  Construct locale with another facet.
     *
     *  Constructs a copy of the locale @a __other.  The facet @a __f
     *  is added to @a __other, replacing an existing facet of type
     *  Facet if there is one.  If @a __f is null, this locale is a
     *  copy of @a __other.
     *
     *  @param  __other  The locale to copy.
     *  @param  __f  The facet to add in.
    */
    template<typename _Facet>
      locale(const locale& __other, _Facet* __f);

    /// Locale destructor.
    ~locale() throw();

    /**
     *  @brief  Assignment operator.
     *
     *  Set this locale to be a copy of @a other.
     *
     *  @param  __other  The locale to copy.
     *  @return  A reference to this locale.
    */
    const locale&
    operator=(const locale& __other) throw();

    /**
     *  @brief  Construct locale with another facet.
     *
     *  Constructs and returns a new copy of this locale.  Adds or replaces an
     *  existing facet of type Facet from the locale @a other into the new
     *  locale.
     *
     *  @tparam  _Facet  The facet type to copy from other
     *  @param  __other  The locale to copy from.
     *  @return  Newly constructed locale.
     *  @throw  std::runtime_error if __other has no facet of type _Facet.
    */
    template<typename _Facet>
      locale
      combine(const locale& __other) const;

    // Locale operations:
    /**
     *  @brief  Return locale name.
     *  @return  Locale name or "*" if unnamed.
    */
    _GLIBCXX_DEFAULT_ABI_TAG
    string
    name() const;

    /**
     *  @brief  Locale equality.
     *
     *  @param  __other  The locale to compare against.
     *  @return  True if other and this refer to the same locale instance, are
     *		 copies, or have the same name.  False otherwise.
    */
    bool
    operator==(const locale& __other) const throw();

#if __cpp_impl_three_way_comparison < 201907L
    /**
     *  @brief  Locale inequality.
     *
     *  @param  __other  The locale to compare against.
     *  @return  ! (*this == __other)
    */
    bool
    operator!=(const locale& __other) const throw()
    { return !(this->operator==(__other)); }
#endif

    /**
     *  @brief  Compare two strings according to collate.
     *
     *  Template operator to compare two strings using the compare function of
     *  the collate facet in this locale.  One use is to provide the locale to
     *  the sort function.  For example, a vector v of strings could be sorted
     *  according to locale loc by doing:
     *  @code
     *  std::sort(v.begin(), v.end(), loc);
     *  @endcode
     *
     *  @param  __s1  First string to compare.
     *  @param  __s2  Second string to compare.
     *  @return  True if collate<_Char> facet compares __s1 < __s2, else false.
    */
    template<typename _Char, typename _Traits, typename _Alloc>
      bool
      operator()(const basic_string<_Char, _Traits, _Alloc>& __s1,
		 const basic_string<_Char, _Traits, _Alloc>& __s2) const;

    // Global locale objects:
    /**
     *  @brief  Set global locale
     *
     *  This function sets the global locale to the argument and returns a
     *  copy of the previous global locale.  If the argument has a name, it
     *  will also call std::setlocale(LC_ALL, loc.name()).
     *
     *  @param  __loc  The new locale to make global.
     *  @return  Copy of the old global locale.
    */
    static locale
    global(const locale& __loc);

    /**
     *  @brief  Return reference to the C locale.
    */
    static const locale&
    classic();

  private:
    // The (shared) implementation
    _Impl*		_M_impl;

    // The "C" reference locale
    static _Impl*       _S_classic;

    // Current global locale
    static _Impl*	_S_global;

    // Names of underlying locale categories.
    // NB: locale::global() has to know how to modify all the
    // underlying categories, not just the ones required by the C++
    // standard.
    static const char* const* const _S_categories;

    // Number of standard categories. For C++, these categories are
    // collate, ctype, monetary, numeric, time, and messages. These
    // directly correspond to ISO C99 macros LC_COLLATE, LC_CTYPE,
    // LC_MONETARY, LC_NUMERIC, and LC_TIME. In addition, POSIX (IEEE
    // 1003.1-2001) specifies LC_MESSAGES.
    // In addition to the standard categories, the underlying
    // operating system is allowed to define extra LC_*
    // macros. For GNU systems, the following are also valid:
    // LC_PAPER, LC_NAME, LC_ADDRESS, LC_TELEPHONE, LC_MEASUREMENT,
    // and LC_IDENTIFICATION.
    enum { _S_categories_size = 6 + _GLIBCXX_NUM_CATEGORIES };

#ifdef __GTHREADS
    static __gthread_once_t _S_once;
#endif

    explicit
    locale(_Impl*) throw();

    static void
    _S_initialize();

    static void
    _S_initialize_once() throw();

    static category
    _S_normalize_category(category);

    void
    _M_coalesce(const locale& __base, const locale& __add, category __cat);

#if _GLIBCXX_USE_CXX11_ABI
    static const id* const _S_twinned_facets[];
#endif
  };


  // 22.1.1.1.2  Class locale::facet
  /**
   *  @brief  Localization functionality base class.
   *  @ingroup locales
   *
   *  The facet class is the base class for a localization feature, such as
   *  money, time, and number printing.  It provides common support for facets
   *  and reference management.
   *
   *  Facets may not be copied or assigned.
  */
  class locale::facet
  {
  private:
    friend class locale;
    friend class locale::_Impl;

    mutable _Atomic_word		_M_refcount;

    // Contains data from the underlying "C" library for the classic locale.
    static __c_locale                   _S_c_locale;

    // String literal for the name of the classic locale.
    static const char			_S_c_name[2];

#ifdef __GTHREADS
    static __gthread_once_t		_S_once;
#endif

    static void
    _S_initialize_once();

  protected:
    /**
     *  @brief  Facet constructor.
     *
     *  This is the constructor provided by the standard.  If refs is 0, the
     *  facet is destroyed when the last referencing locale is destroyed.
     *  Otherwise the facet will never be destroyed.
     *
     *  @param __refs  The initial value for reference count.
    */
    explicit
    facet(size_t __refs = 0) throw() : _M_refcount(__refs ? 1 : 0)
    { }

    /// Facet destructor.
    virtual
    ~facet();

    static void
    _S_create_c_locale(__c_locale& __cloc, const char* __s,
		       __c_locale __old = 0);

    static __c_locale
    _S_clone_c_locale(__c_locale& __cloc) throw();

    static void
    _S_destroy_c_locale(__c_locale& __cloc);

    static __c_locale
    _S_lc_ctype_c_locale(__c_locale __cloc, const char* __s);

    // Returns data from the underlying "C" library data for the
    // classic locale.
    static __c_locale
    _S_get_c_locale();

    _GLIBCXX_CONST static const char*
    _S_get_c_name() throw();

#if __cplusplus < 201103L
  private:
    facet(const facet&);  // Not defined.

    facet&
    operator=(const facet&);  // Not defined.
#else
    facet(const facet&) = delete;

    facet&
    operator=(const facet&) = delete;
#endif

  private:
    void
    _M_add_reference() const throw()
    { __gnu_cxx::__atomic_add_dispatch(&_M_refcount, 1); }

    void
    _M_remove_reference() const throw()
    {
      // Be race-detector-friendly.  For more info see bits/c++config.
      _GLIBCXX_SYNCHRONIZATION_HAPPENS_BEFORE(&_M_refcount);
      if (__gnu_cxx::__exchange_and_add_dispatch(&_M_refcount, -1) == 1)
	{
          _GLIBCXX_SYNCHRONIZATION_HAPPENS_AFTER(&_M_refcount);
	  __try
	    { delete this; }
	  __catch(...)
	    { }
	}
    }

    const facet* _M_sso_shim(const id*) const;
    const facet* _M_cow_shim(const id*) const;

  protected:
    class __shim; // For internal use only.
  };


  // 22.1.1.1.3 Class locale::id
  /**
   *  @brief  Facet ID class.
   *  @ingroup locales
   *
   *  The ID class provides facets with an index used to identify them.
   *  Every facet class must define a public static member locale::id, or be
   *  derived from a facet that provides this member, otherwise the facet
   *  cannot be used in a locale.  The locale::id ensures that each class
   *  type gets a unique identifier.
  */
  class locale::id
  {
  private:
    friend class locale;
    friend class locale::_Impl;

    template<typename _Facet>
      friend const _Facet&
      use_facet(const locale&);

    template<typename _Facet>
      friend bool
      has_facet(const locale&) throw();

    // NB: There is no accessor for _M_index because it may be used
    // before the constructor is run; the effect of calling a member
    // function (even an inline) would be undefined.
    mutable size_t		_M_index;

    // Last id number assigned.
    static _Atomic_word		_S_refcount;

    void
    operator=(const id&);  // Not defined.

    id(const id&);  // Not defined.

  public:
    // NB: This class is always a static data member, and thus can be
    // counted on to be zero-initialized.
    /// Constructor.
    id() { }

    size_t
    _M_id() const throw();
  };


  // Implementation object for locale.
  class locale::_Impl
  {
  public:
    // Friends.
    friend class locale;
    friend class locale::facet;

    template<typename _Facet>
      friend bool
      has_facet(const locale&) throw();

    template<typename _Facet>
      friend const _Facet&
      use_facet(const locale&);

    template<typename _Cache>
      friend struct __use_cache;

  private:
    // Data Members.
    _Atomic_word			_M_refcount;
    const facet**			_M_facets;
    size_t				_M_facets_size;
    const facet**			_M_caches;
    char**				_M_names;
    static const locale::id* const	_S_id_ctype[];
    static const locale::id* const	_S_id_numeric[];
    static const locale::id* const	_S_id_collate[];
    static const locale::id* const	_S_id_time[];
    static const locale::id* const	_S_id_monetary[];
    static const locale::id* const	_S_id_messages[];
    static const locale::id* const* const _S_facet_categories[];

    void
    _M_add_reference() throw()
    { __gnu_cxx::__atomic_add_dispatch(&_M_refcount, 1); }

    void
    _M_remove_reference() throw()
    {
      // Be race-detector-friendly.  For more info see bits/c++config.
      _GLIBCXX_SYNCHRONIZATION_HAPPENS_BEFORE(&_M_refcount);
      if (__gnu_cxx::__exchange_and_add_dispatch(&_M_refcount, -1) == 1)
	{
          _GLIBCXX_SYNCHRONIZATION_HAPPENS_AFTER(&_M_refcount);
	  __try
	    { delete this; }
	  __catch(...)
	    { }
	}
    }

    _Impl(const _Impl&, size_t);
    _Impl(const char*, size_t);
    _Impl(size_t) throw();

   ~_Impl() throw();

    _Impl(const _Impl&);  // Not defined.

    void
    operator=(const _Impl&);  // Not defined.

    bool
    _M_check_same_name()
    {
      bool __ret = true;
      if (_M_names[1])
	// We must actually compare all the _M_names: can be all equal!
	for (size_t __i = 0; __ret && __i < _S_categories_size - 1; ++__i)
	  __ret = __builtin_strcmp(_M_names[__i], _M_names[__i + 1]) == 0;
      return __ret;
    }

    void
    _M_replace_categories(const _Impl*, category);

    void
    _M_replace_category(const _Impl*, const locale::id* const*);

    void
    _M_replace_facet(const _Impl*, const locale::id*);

    void
    _M_install_facet(const locale::id*, const facet*);

    template<typename _Facet>
      void
      _M_init_facet(_Facet* __facet)
      { _M_install_facet(&_Facet::id, __facet); }

    template<typename _Facet>
      void
      _M_init_facet_unchecked(_Facet* __facet)
      {
	__facet->_M_add_reference();
	_M_facets[_Facet::id._M_id()] = __facet;
      }

    void
    _M_install_cache(const facet*, size_t);

    void _M_init_extra(facet**);
    void _M_init_extra(void*, void*, const char*, const char*);
  };


  /**
   *  @brief  Facet for localized string comparison.
   *
   *  This facet encapsulates the code to compare strings in a localized
   *  manner.
   *
   *  The collate template uses protected virtual functions to provide
   *  the actual results.  The public accessors forward the call to
   *  the virtual functions.  These virtual functions are hooks for
   *  developers to implement the behavior they require from the
   *  collate facet.
  */
  template<typename _CharT>
    class _GLIBCXX_NAMESPACE_CXX11 collate : public locale::facet
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
      // initialization, needed by collate_byname as well.
      __c_locale			_M_c_locale_collate;

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
      collate(size_t __refs = 0)
      : facet(__refs), _M_c_locale_collate(_S_get_c_locale())
      { }

      /**
       *  @brief  Internal constructor. Not for general use.
       *
       *  This is a constructor for use by the library itself to set up new
       *  locales.
       *
       *  @param __cloc  The C locale.
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      collate(__c_locale __cloc, size_t __refs = 0)
      : facet(__refs), _M_c_locale_collate(_S_clone_c_locale(__cloc))
      { }

      /**
       *  @brief  Compare two strings.
       *
       *  This function compares two strings and returns the result by calling
       *  collate::do_compare().
       *
       *  @param __lo1  Start of string 1.
       *  @param __hi1  End of string 1.
       *  @param __lo2  Start of string 2.
       *  @param __hi2  End of string 2.
       *  @return  1 if string1 > string2, -1 if string1 < string2, else 0.
      */
      int
      compare(const _CharT* __lo1, const _CharT* __hi1,
	      const _CharT* __lo2, const _CharT* __hi2) const
      { return this->do_compare(__lo1, __hi1, __lo2, __hi2); }

      /**
       *  @brief  Transform string to comparable form.
       *
       *  This function is a wrapper for strxfrm functionality.  It takes the
       *  input string and returns a modified string that can be directly
       *  compared to other transformed strings.  In the C locale, this
       *  function just returns a copy of the input string.  In some other
       *  locales, it may replace two chars with one, change a char for
       *  another, etc.  It does so by returning collate::do_transform().
       *
       *  @param __lo  Start of string.
       *  @param __hi  End of string.
       *  @return  Transformed string_type.
      */
      string_type
      transform(const _CharT* __lo, const _CharT* __hi) const
      { return this->do_transform(__lo, __hi); }

      /**
       *  @brief  Return hash of a string.
       *
       *  This function computes and returns a hash on the input string.  It
       *  does so by returning collate::do_hash().
       *
       *  @param __lo  Start of string.
       *  @param __hi  End of string.
       *  @return  Hash value.
      */
      long
      hash(const _CharT* __lo, const _CharT* __hi) const
      { return this->do_hash(__lo, __hi); }

      // Used to abstract out _CharT bits in virtual member functions, below.
      int
      _M_compare(const _CharT*, const _CharT*) const throw();

      size_t
      _M_transform(_CharT*, const _CharT*, size_t) const throw();

  protected:
      /// Destructor.
      virtual
      ~collate()
      { _S_destroy_c_locale(_M_c_locale_collate); }

      /**
       *  @brief  Compare two strings.
       *
       *  This function is a hook for derived classes to change the value
       *  returned.  @see compare().
       *
       *  @param __lo1  Start of string 1.
       *  @param __hi1  End of string 1.
       *  @param __lo2  Start of string 2.
       *  @param __hi2  End of string 2.
       *  @return  1 if string1 > string2, -1 if string1 < string2, else 0.
      */
      virtual int
      do_compare(const _CharT* __lo1, const _CharT* __hi1,
		 const _CharT* __lo2, const _CharT* __hi2) const;

      /**
       *  @brief  Transform string to comparable form.
       *
       *  This function is a hook for derived classes to change the value
       *  returned.
       *
       *  @param __lo  Start.
       *  @param __hi  End.
       *  @return  transformed string.
      */
      virtual string_type
      do_transform(const _CharT* __lo, const _CharT* __hi) const;

      /**
       *  @brief  Return hash of a string.
       *
       *  This function computes and returns a hash on the input string.  This
       *  function is a hook for derived classes to change the value returned.
       *
       *  @param __lo  Start of string.
       *  @param __hi  End of string.
       *  @return  Hash value.
      */
      virtual long
      do_hash(const _CharT* __lo, const _CharT* __hi) const;
    };

  template<typename _CharT>
    locale::id collate<_CharT>::id;

  // Specializations.
  template<>
    int
    collate<char>::_M_compare(const char*, const char*) const throw();

  template<>
    size_t
    collate<char>::_M_transform(char*, const char*, size_t) const throw();

#ifdef _GLIBCXX_USE_WCHAR_T
  template<>
    int
    collate<wchar_t>::_M_compare(const wchar_t*, const wchar_t*) const throw();

  template<>
    size_t
    collate<wchar_t>::_M_transform(wchar_t*, const wchar_t*, size_t) const throw();
#endif

  /// class collate_byname [22.2.4.2].
  template<typename _CharT>
    class _GLIBCXX_NAMESPACE_CXX11 collate_byname : public collate<_CharT>
    {
    public:
      //@{
      /// Public typedefs
      typedef _CharT               char_type;
      typedef basic_string<_CharT> string_type;
      //@}

      explicit
      collate_byname(const char* __s, size_t __refs = 0)
      : collate<_CharT>(__refs)
      {
	if (__builtin_strcmp(__s, "C") != 0
	    && __builtin_strcmp(__s, "POSIX") != 0)
	  {
	    this->_S_destroy_c_locale(this->_M_c_locale_collate);
	    this->_S_create_c_locale(this->_M_c_locale_collate, __s);
	  }
      }

#if __cplusplus >= 201103L
      explicit
      collate_byname(const string& __s, size_t __refs = 0)
      : collate_byname(__s.c_str(), __refs) { }
#endif

    protected:
      virtual
      ~collate_byname() { }
    };

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

# include <bits/locale_classes.tcc>

#endif
