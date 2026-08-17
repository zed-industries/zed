// Class filesystem::path -*- C++ -*-

// Copyright (C) 2014-2020 Free Software Foundation, Inc.
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

/** @file experimental/bits/fs_path.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{experimental/filesystem}
 */

#ifndef _GLIBCXX_EXPERIMENTAL_FS_PATH_H
#define _GLIBCXX_EXPERIMENTAL_FS_PATH_H 1

#if __cplusplus < 201103L
# include <bits/c++0x_warning.h>
#else

#include <utility>
#include <type_traits>
#include <vector>
#include <locale>
#include <iosfwd>
#include <codecvt>
#include <system_error>
#include <bits/stl_algobase.h>
#include <bits/quoted_string.h>
#include <bits/locale_conv.h>
#if __cplusplus == 201402L
# include <experimental/string_view>
#endif

#if defined(_WIN32) && !defined(__CYGWIN__)
# define _GLIBCXX_FILESYSTEM_IS_WINDOWS 1
# include <algorithm>
#endif

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

namespace experimental
{
namespace filesystem
{
inline namespace v1
{
_GLIBCXX_BEGIN_NAMESPACE_CXX11

#if __cplusplus == 201402L
  using std::experimental::basic_string_view;
#elif __cplusplus > 201402L
  using std::basic_string_view;
#endif

  /** @addtogroup filesystem-ts
   *  @{
   */

  /// @cond undocumented
namespace __detail
{
  template<typename _CharT,
	   typename _Ch = typename remove_const<_CharT>::type>
    using __is_encoded_char
      = __or_<is_same<_Ch, char>,
	      is_same<_Ch, wchar_t>,
#ifdef _GLIBCXX_USE_CHAR8_T
	      is_same<_Ch, char8_t>,
#endif
	      is_same<_Ch, char16_t>,
	      is_same<_Ch, char32_t>>;

  template<typename _Iter,
	   typename _Iter_traits = std::iterator_traits<_Iter>>
    using __is_path_iter_src
      = __and_<__is_encoded_char<typename _Iter_traits::value_type>,
	       std::is_base_of<std::input_iterator_tag,
			       typename _Iter_traits::iterator_category>>;

  template<typename _Iter>
    static __is_path_iter_src<_Iter>
    __is_path_src(_Iter, int);

  template<typename _CharT, typename _Traits, typename _Alloc>
    static __is_encoded_char<_CharT>
    __is_path_src(const basic_string<_CharT, _Traits, _Alloc>&, int);

#if __cplusplus >= 201402L
  template<typename _CharT, typename _Traits>
    static __is_encoded_char<_CharT>
    __is_path_src(const basic_string_view<_CharT, _Traits>&, int);
#endif

  template<typename _Unknown>
    static std::false_type
    __is_path_src(const _Unknown&, ...);

  template<typename _Tp1, typename _Tp2>
    struct __constructible_from;

  template<typename _Iter>
    struct __constructible_from<_Iter, _Iter>
    : __is_path_iter_src<_Iter>
    { };

  template<typename _Source>
    struct __constructible_from<_Source, void>
    : decltype(__is_path_src(std::declval<_Source>(), 0))
    { };

  template<typename _Tp1, typename _Tp2 = void,
	   typename _Tp1_nocv = typename remove_cv<_Tp1>::type,
	   typename _Tp1_noptr = typename remove_pointer<_Tp1>::type>
    using _Path = typename
      std::enable_if<__and_<__not_<is_same<_Tp1_nocv, path>>,
			    __not_<is_void<_Tp1_noptr>>,
			    __constructible_from<_Tp1, _Tp2>>::value,
		     path>::type;

  template<typename _Source>
    inline _Source
    _S_range_begin(_Source __begin) { return __begin; }

  struct __null_terminated { };

  template<typename _Source>
    inline __null_terminated
    _S_range_end(_Source) { return {}; }

  template<typename _CharT, typename _Traits, typename _Alloc>
    inline const _CharT*
    _S_range_begin(const basic_string<_CharT, _Traits, _Alloc>& __str)
    { return __str.data(); }

  template<typename _CharT, typename _Traits, typename _Alloc>
    inline const _CharT*
    _S_range_end(const basic_string<_CharT, _Traits, _Alloc>& __str)
    { return __str.data() + __str.size(); }

#if __cplusplus >= 201402L
  template<typename _CharT, typename _Traits>
    inline const _CharT*
    _S_range_begin(const basic_string_view<_CharT, _Traits>& __str)
    { return __str.data(); }

  template<typename _CharT, typename _Traits>
    inline const _CharT*
    _S_range_end(const basic_string_view<_CharT, _Traits>& __str)
    { return __str.data() + __str.size(); }
#endif

  template<typename _Tp,
	   typename _Iter = decltype(_S_range_begin(std::declval<_Tp>())),
	   typename _Val = typename std::iterator_traits<_Iter>::value_type,
	   typename _UnqualVal = typename std::remove_const<_Val>::type>
    using __value_type_is_char = typename std::enable_if<
      std::is_same<_UnqualVal, char>::value,
      _UnqualVal>::type;

  template<typename _Tp,
	   typename _Iter = decltype(_S_range_begin(std::declval<_Tp>())),
	   typename _Val = typename std::iterator_traits<_Iter>::value_type,
	   typename _UnqualVal = typename std::remove_const<_Val>::type>
    using __value_type_is_char_or_char8_t = typename std::enable_if<
      __or_<
	std::is_same<_UnqualVal, char>
#ifdef _GLIBCXX_USE_CHAR8_T
	,std::is_same<_UnqualVal, char8_t>
#endif
      >::value, _UnqualVal>::type;

} // namespace __detail
  /// @endcond

  /// A filesystem path.
  class path
  {
  public:
#ifdef _GLIBCXX_FILESYSTEM_IS_WINDOWS
    typedef wchar_t				value_type;
    static constexpr value_type			preferred_separator = L'\\';
#else
    typedef char				value_type;
    static constexpr value_type			preferred_separator = '/';
#endif
    typedef std::basic_string<value_type>	string_type;

    // constructors and destructor

    path() noexcept { }

    path(const path& __p) = default;

    path(path&& __p) noexcept
    : _M_pathname(std::move(__p._M_pathname)), _M_type(__p._M_type)
    {
      if (_M_type == _Type::_Multi)
	_M_split_cmpts();
      __p.clear();
    }

    path(string_type&& __source)
    : _M_pathname(std::move(__source))
    { _M_split_cmpts(); }

    template<typename _Source,
	     typename _Require = __detail::_Path<_Source>>
      path(_Source const& __source)
      : _M_pathname(_S_convert(__detail::_S_range_begin(__source),
			       __detail::_S_range_end(__source)))
      { _M_split_cmpts(); }

    template<typename _InputIterator,
	     typename _Require = __detail::_Path<_InputIterator, _InputIterator>>
      path(_InputIterator __first, _InputIterator __last)
      : _M_pathname(_S_convert(__first, __last))
      { _M_split_cmpts(); }

    template<typename _Source,
	     typename _Require = __detail::_Path<_Source>,
	     typename _Require2 = __detail::__value_type_is_char<_Source>>
      path(_Source const& __source, const locale& __loc)
      : _M_pathname(_S_convert_loc(__detail::_S_range_begin(__source),
				   __detail::_S_range_end(__source), __loc))
      { _M_split_cmpts(); }

    template<typename _InputIterator,
	     typename _Require = __detail::_Path<_InputIterator, _InputIterator>,
	     typename _Require2 = __detail::__value_type_is_char<_InputIterator>>
      path(_InputIterator __first, _InputIterator __last, const locale& __loc)
      : _M_pathname(_S_convert_loc(__first, __last, __loc))
      { _M_split_cmpts(); }

    ~path() = default;

    // assignments

    path& operator=(const path& __p) = default;
    path& operator=(path&& __p) noexcept;
    path& operator=(string_type&& __source);
    path& assign(string_type&& __source);

    template<typename _Source>
      __detail::_Path<_Source>&
      operator=(_Source const& __source)
      { return *this = path(__source); }

    template<typename _Source>
      __detail::_Path<_Source>&
      assign(_Source const& __source)
      { return *this = path(__source); }

    template<typename _InputIterator>
      __detail::_Path<_InputIterator, _InputIterator>&
      assign(_InputIterator __first, _InputIterator __last)
      { return *this = path(__first, __last); }

    // appends

    path& operator/=(const path& __p) { return _M_append(__p._M_pathname); }

    template<typename _Source>
      __detail::_Path<_Source>&
      operator/=(_Source const& __source)
      { return append(__source); }

    template<typename _Source>
      __detail::_Path<_Source>&
      append(_Source const& __source)
      {
	return _M_append(_S_convert(__detail::_S_range_begin(__source),
				    __detail::_S_range_end(__source)));
      }

    template<typename _InputIterator>
      __detail::_Path<_InputIterator, _InputIterator>&
      append(_InputIterator __first, _InputIterator __last)
      { return _M_append(_S_convert(__first, __last)); }

    // concatenation

    path& operator+=(const path& __x);
    path& operator+=(const string_type& __x);
    path& operator+=(const value_type* __x);
    path& operator+=(value_type __x);
#if __cplusplus >= 201402L
    path& operator+=(basic_string_view<value_type> __x);
#endif

    template<typename _Source>
      __detail::_Path<_Source>&
      operator+=(_Source const& __x) { return concat(__x); }

    template<typename _CharT>
      __detail::_Path<_CharT*, _CharT*>&
      operator+=(_CharT __x);

    template<typename _Source>
      __detail::_Path<_Source>&
      concat(_Source const& __x)
      {
	return *this += _S_convert(__detail::_S_range_begin(__x),
				   __detail::_S_range_end(__x));
      }

    template<typename _InputIterator>
      __detail::_Path<_InputIterator, _InputIterator>&
      concat(_InputIterator __first, _InputIterator __last)
      { return *this += _S_convert(__first, __last); }

    // modifiers

    void clear() noexcept { _M_pathname.clear(); _M_split_cmpts(); }

    path& make_preferred();
    path& remove_filename();
    path& replace_filename(const path& __replacement);
    path& replace_extension(const path& __replacement = path());

    void swap(path& __rhs) noexcept;

    // native format observers

    const string_type&  native() const noexcept { return _M_pathname; }
    const value_type*   c_str() const noexcept { return _M_pathname.c_str(); }
    operator string_type() const { return _M_pathname; }

    template<typename _CharT, typename _Traits = std::char_traits<_CharT>,
	     typename _Allocator = std::allocator<_CharT>>
      std::basic_string<_CharT, _Traits, _Allocator>
      string(const _Allocator& __a = _Allocator()) const;

    std::string    string() const;
#if _GLIBCXX_USE_WCHAR_T
    std::wstring   wstring() const;
#endif
#ifdef _GLIBCXX_USE_CHAR8_T
    __attribute__((__abi_tag__("__u8")))
    std::u8string  u8string() const;
#else
    std::string    u8string() const;
#endif // _GLIBCXX_USE_CHAR8_T
    std::u16string u16string() const;
    std::u32string u32string() const;

    // generic format observers
    template<typename _CharT, typename _Traits = std::char_traits<_CharT>,
	     typename _Allocator = std::allocator<_CharT>>
      std::basic_string<_CharT, _Traits, _Allocator>
      generic_string(const _Allocator& __a = _Allocator()) const;

    std::string    generic_string() const;
#if _GLIBCXX_USE_WCHAR_T
    std::wstring   generic_wstring() const;
#endif
#ifdef _GLIBCXX_USE_CHAR8_T
    __attribute__((__abi_tag__("__u8")))
    std::u8string  generic_u8string() const;
#else
    std::string    generic_u8string() const;
#endif // _GLIBCXX_USE_CHAR8_T
    std::u16string generic_u16string() const;
    std::u32string generic_u32string() const;

    // compare

    int compare(const path& __p) const noexcept;
    int compare(const string_type& __s) const;
    int compare(const value_type* __s) const;
#if __cplusplus >= 201402L
    int compare(const basic_string_view<value_type> __s) const;
#endif

    // decomposition

    path root_name() const;
    path root_directory() const;
    path root_path() const;
    path relative_path() const;
    path parent_path() const;
    path filename() const;
    path stem() const;
    path extension() const;

    // query

    _GLIBCXX_NODISCARD bool empty() const noexcept { return _M_pathname.empty(); }
    bool has_root_name() const;
    bool has_root_directory() const;
    bool has_root_path() const;
    bool has_relative_path() const;
    bool has_parent_path() const;
    bool has_filename() const;
    bool has_stem() const;
    bool has_extension() const;
    bool is_absolute() const;
    bool is_relative() const { return !is_absolute(); }

    // iterators
    class iterator;
    typedef iterator const_iterator;

    iterator begin() const;
    iterator end() const;

    /// @cond undocumented
    // Create a basic_string by reading until a null character.
    template<typename _InputIterator,
	     typename _Traits = std::iterator_traits<_InputIterator>,
	     typename _CharT
	       = typename std::remove_cv<typename _Traits::value_type>::type>
      static std::basic_string<_CharT>
      _S_string_from_iter(_InputIterator __source)
      {
	std::basic_string<_CharT> __str;
	for (_CharT __ch = *__source; __ch != _CharT(); __ch = *++__source)
	  __str.push_back(__ch);
	return __str;
      }
    /// @endcond

  private:
    enum class _Type : unsigned char {
	_Multi, _Root_name, _Root_dir, _Filename
    };

    path(string_type __str, _Type __type) : _M_pathname(__str), _M_type(__type)
    {
      __glibcxx_assert(!empty());
      __glibcxx_assert(_M_type != _Type::_Multi);
    }

    enum class _Split { _Stem, _Extension };

    path& _M_append(const string_type& __str)
    {
      if (!_M_pathname.empty() && !_S_is_dir_sep(_M_pathname.back())
	  && !__str.empty() && !_S_is_dir_sep(__str.front()))
	_M_pathname += preferred_separator;
      _M_pathname += __str;
      _M_split_cmpts();
      return *this;
    }

    pair<const string_type*, size_t> _M_find_extension() const;

    template<typename _CharT>
      struct _Cvt;

    static string_type
    _S_convert(value_type* __src, __detail::__null_terminated)
    { return string_type(__src); }

    static string_type
    _S_convert(const value_type* __src, __detail::__null_terminated)
    { return string_type(__src); }

    template<typename _Iter>
      static string_type
      _S_convert(_Iter __first, _Iter __last)
      {
	using __value_type = typename std::iterator_traits<_Iter>::value_type;
	return _Cvt<typename remove_cv<__value_type>::type>::
	  _S_convert(__first, __last);
      }

    template<typename _InputIterator>
      static string_type
      _S_convert(_InputIterator __src, __detail::__null_terminated)
      {
	auto __s = _S_string_from_iter(__src);
	return _S_convert(__s.c_str(), __s.c_str() + __s.size());
      }

    static string_type
    _S_convert_loc(const char* __first, const char* __last,
		   const std::locale& __loc);

    template<typename _Iter>
      static string_type
      _S_convert_loc(_Iter __first, _Iter __last, const std::locale& __loc)
      {
	const std::string __str(__first, __last);
	return _S_convert_loc(__str.data(), __str.data()+__str.size(), __loc);
      }

    template<typename _InputIterator>
      static string_type
      _S_convert_loc(_InputIterator __src, __detail::__null_terminated,
		     const std::locale& __loc)
      {
	const std::string __s = _S_string_from_iter(__src);
	return _S_convert_loc(__s.data(), __s.data() + __s.size(), __loc);
      }

    static bool _S_is_dir_sep(value_type __ch)
    {
#ifdef _GLIBCXX_FILESYSTEM_IS_WINDOWS
      return __ch == L'/' || __ch == preferred_separator;
#else
      return __ch == '/';
#endif
    }

    void _M_split_cmpts();
    void _M_trim();
    void _M_add_root_name(size_t __n);
    void _M_add_root_dir(size_t __pos);
    void _M_add_filename(size_t __pos, size_t __n);

    string_type _M_pathname;

    struct _Cmpt;
    using _List = _GLIBCXX_STD_C::vector<_Cmpt>;
    _List _M_cmpts; // empty unless _M_type == _Type::_Multi
    _Type _M_type = _Type::_Multi;
  };

  /// @relates std::experimental::filesystem::path @{

  /// Swap overload for paths
  inline void swap(path& __lhs, path& __rhs) noexcept { __lhs.swap(__rhs); }

  /// Compute a hash value for a path
  size_t hash_value(const path& __p) noexcept;

  /// Compare paths
  inline bool operator<(const path& __lhs, const path& __rhs) noexcept
  { return __lhs.compare(__rhs) < 0; }

  /// Compare paths
  inline bool operator<=(const path& __lhs, const path& __rhs) noexcept
  { return !(__rhs < __lhs); }

  /// Compare paths
  inline bool operator>(const path& __lhs, const path& __rhs) noexcept
  { return __rhs < __lhs; }

  /// Compare paths
  inline bool operator>=(const path& __lhs, const path& __rhs) noexcept
  { return !(__lhs < __rhs); }

  /// Compare paths
  inline bool operator==(const path& __lhs, const path& __rhs) noexcept
  { return __lhs.compare(__rhs) == 0; }

  /// Compare paths
  inline bool operator!=(const path& __lhs, const path& __rhs) noexcept
  { return !(__lhs == __rhs); }

  /// Append one path to another
  inline path operator/(const path& __lhs, const path& __rhs)
  {
    path __result(__lhs);
    __result /= __rhs;
    return __result;
  }

  /// Write a path to a stream
  template<typename _CharT, typename _Traits>
    basic_ostream<_CharT, _Traits>&
    operator<<(basic_ostream<_CharT, _Traits>& __os, const path& __p)
    {
      auto __tmp = __p.string<_CharT, _Traits>();
      using __quoted_string
	= std::__detail::_Quoted_string<decltype(__tmp)&, _CharT>;
      __os << __quoted_string{__tmp, _CharT('"'), _CharT('\\')};
      return __os;
    }

  /// Read a path from a stream
  template<typename _CharT, typename _Traits>
    basic_istream<_CharT, _Traits>&
    operator>>(basic_istream<_CharT, _Traits>& __is, path& __p)
    {
      basic_string<_CharT, _Traits> __tmp;
      using __quoted_string
	= std::__detail::_Quoted_string<decltype(__tmp)&, _CharT>;
      if (__is >> __quoted_string{ __tmp, _CharT('"'), _CharT('\\') })
	__p = std::move(__tmp);
      return __is;
    }

  /// Create a path from a UTF-8-encoded sequence of char
#ifdef _GLIBCXX_FILESYSTEM_IS_WINDOWS
  template<typename _InputIterator>
    inline path
    __u8path(_InputIterator __first, _InputIterator __last, char)
    {
      // XXX This assumes native wide encoding is UTF-16.
      std::codecvt_utf8_utf16<path::value_type> __cvt;
      path::string_type __tmp;
      const std::string __u8str{__first, __last};
      const char* const __ptr = __u8str.data();
      if (__str_codecvt_in_all(__ptr, __ptr + __u8str.size(), __tmp, __cvt))
	return path{ __tmp };
      _GLIBCXX_THROW_OR_ABORT(filesystem_error(
	    "Cannot convert character sequence",
	    std::make_error_code(errc::illegal_byte_sequence)));
    }

#ifdef _GLIBCXX_USE_CHAR8_T
  template<typename _InputIterator>
    inline path
    __u8path(_InputIterator __first, _InputIterator __last, char8_t)
    {
      return path{ __first, __last };
    }
#endif // _GLIBCXX_USE_CHAR8_T
#endif // _GLIBCXX_FILESYSTEM_IS_WINDOWS

  template<typename _InputIterator,
	   typename _Require = __detail::_Path<_InputIterator, _InputIterator>,
	   typename _CharT =
	     __detail::__value_type_is_char_or_char8_t<_InputIterator>>
    inline path
    u8path(_InputIterator __first, _InputIterator __last)
    {
#ifdef _GLIBCXX_FILESYSTEM_IS_WINDOWS
      return __u8path(__first, __last, _CharT{});
#else
      return path{ __first, __last };
#endif
    }

  /// Create a path from a UTF-8-encoded sequence of char
#ifdef _GLIBCXX_FILESYSTEM_IS_WINDOWS
  inline path
  __u8path(const string& __s, char)
  {
    return filesystem::u8path(__s.data(), __s.data() + __s.size());
  }

  template<typename _Source>
    inline __enable_if_t<is_convertible<const _Source&, string>::value, path>
    __u8path(const _Source& __source, char)
    {
      std::string __s = __source;
      return filesystem::u8path(__s.data(), __s.data() + __s.size());
    }

  template<typename _Source>
    inline __enable_if_t<!is_convertible<const _Source&, string>::value, path>
    __u8path(const _Source& __source, char)
    {
      std::string __s = path::_S_string_from_iter(__source);
      return filesystem::u8path(__s.data(), __s.data() + __s.size());
    }

#ifdef _GLIBCXX_USE_CHAR8_T
  template<typename _Source>
    inline path
    __u8path(const _Source& __source, char8_t)
    {
      return path{ __source };
    }
#endif // _GLIBCXX_USE_CHAR8_T
#endif // _GLIBCXX_FILESYSTEM_IS_WINDOWS

  template<typename _Source,
	   typename _Require = __detail::_Path<_Source>,
	   typename _CharT =
	     __detail::__value_type_is_char_or_char8_t<_Source>>
    inline path
    u8path(const _Source& __source)
    {
#ifdef _GLIBCXX_FILESYSTEM_IS_WINDOWS
      return __u8path(__source, _CharT{});
#else
      return path{ __source };
#endif
    }

  /// @}

  /// Exception type thrown by the Filesystem TS library
  class filesystem_error : public std::system_error
  {
  public:
    filesystem_error(const string& __what_arg, error_code __ec)
    : system_error(__ec, __what_arg) { }

    filesystem_error(const string& __what_arg, const path& __p1,
		     error_code __ec)
    : system_error(__ec, __what_arg), _M_path1(__p1) { }

    filesystem_error(const string& __what_arg, const path& __p1,
		     const path& __p2, error_code __ec)
    : system_error(__ec, __what_arg), _M_path1(__p1), _M_path2(__p2)
    { }

    ~filesystem_error();

    const path& path1() const noexcept { return _M_path1; }
    const path& path2() const noexcept { return _M_path2; }
    const char* what() const noexcept { return _M_what.c_str(); }

  private:
    std::string _M_gen_what();

    path _M_path1;
    path _M_path2;
    std::string _M_what = _M_gen_what();
  };

  /// @cond undocumented
  struct path::_Cmpt : path
  {
    _Cmpt(string_type __s, _Type __t, size_t __pos)
      : path(std::move(__s), __t), _M_pos(__pos) { }

    _Cmpt() : _M_pos(-1) { }

    size_t _M_pos;
  };

  // specialize _Cvt for degenerate 'noconv' case
  template<>
    struct path::_Cvt<path::value_type>
    {
      template<typename _Iter>
	static string_type
	_S_convert(_Iter __first, _Iter __last)
	{ return string_type{__first, __last}; }
    };

  template<typename _CharT>
    struct path::_Cvt
    {
#ifdef _GLIBCXX_FILESYSTEM_IS_WINDOWS
#ifdef _GLIBCXX_USE_CHAR8_T
      static string_type
      _S_wconvert(const char8_t* __f, const char8_t* __l, const char8_t*)
      {
	const char* __f2 = (const char*)__f;
	const char* __l2 = (const char*)__l;
	std::wstring __wstr;
	std::codecvt_utf8_utf16<wchar_t> __wcvt;
	if (__str_codecvt_in_all(__f2, __l2, __wstr, __wcvt))
	  return __wstr;
      }
#endif

      static string_type
      _S_wconvert(const char* __f, const char* __l, const char*)
      {
	using _Cvt = std::codecvt<wchar_t, char, mbstate_t>;
	const auto& __cvt = std::use_facet<_Cvt>(std::locale{});
	std::wstring __wstr;
	if (__str_codecvt_in_all(__f, __l, __wstr, __cvt))
	    return __wstr;
	_GLIBCXX_THROW_OR_ABORT(filesystem_error(
	      "Cannot convert character sequence",
	      std::make_error_code(errc::illegal_byte_sequence)));
      }

      static string_type
      _S_wconvert(const _CharT* __f, const _CharT* __l, const void*)
      {
	struct _UCvt : std::codecvt<_CharT, char, std::mbstate_t>
	{ } __cvt;
	std::string __str;
	if (__str_codecvt_out_all(__f, __l, __str, __cvt))
	  {
	    const char* __f2 = __str.data();
	    const char* __l2 = __f2 + __str.size();
	    std::codecvt_utf8_utf16<wchar_t> __wcvt;
	    std::wstring __wstr;
	    if (__str_codecvt_in_all(__f2, __l2, __wstr, __wcvt))
	      return __wstr;
	  }
	_GLIBCXX_THROW_OR_ABORT(filesystem_error(
	      "Cannot convert character sequence",
	      std::make_error_code(errc::illegal_byte_sequence)));
      }

      static string_type
      _S_convert(const _CharT* __f, const _CharT* __l)
      {
	return _S_wconvert(__f, __l, (const _CharT*)nullptr);
      }
#else
      static string_type
      _S_convert(const _CharT* __f, const _CharT* __l)
      {
#ifdef _GLIBCXX_USE_CHAR8_T
	if constexpr (is_same<_CharT, char8_t>::value)
	  return string_type(__f, __l);
	else
#endif
	  {
	    struct _UCvt : std::codecvt<_CharT, char, std::mbstate_t>
	    { } __cvt;
	    std::string __str;
	    if (__str_codecvt_out_all(__f, __l, __str, __cvt))
	      return __str;
	    _GLIBCXX_THROW_OR_ABORT(filesystem_error(
		  "Cannot convert character sequence",
		  std::make_error_code(errc::illegal_byte_sequence)));
	  }
      }
#endif

      static string_type
      _S_convert(_CharT* __f, _CharT* __l)
      {
	return _S_convert(const_cast<const _CharT*>(__f),
			  const_cast<const _CharT*>(__l));
      }

      template<typename _Iter>
	static string_type
	_S_convert(_Iter __first, _Iter __last)
	{
	  const std::basic_string<_CharT> __str(__first, __last);
	  return _S_convert(__str.data(), __str.data() + __str.size());
	}

      template<typename _Iter, typename _Cont>
	static string_type
	_S_convert(__gnu_cxx::__normal_iterator<_Iter, _Cont> __first,
		  __gnu_cxx::__normal_iterator<_Iter, _Cont> __last)
	{ return _S_convert(__first.base(), __last.base()); }
    };
  /// @endcond

  /// An iterator for the components of a path
  class path::iterator
  {
  public:
    using difference_type	= std::ptrdiff_t;
    using value_type		= path;
    using reference		= const path&;
    using pointer		= const path*;
    using iterator_category	= std::bidirectional_iterator_tag;

    iterator() : _M_path(nullptr), _M_cur(), _M_at_end() { }

    iterator(const iterator&) = default;
    iterator& operator=(const iterator&) = default;

    reference operator*() const;
    pointer   operator->() const { return std::__addressof(**this); }

    iterator& operator++();
    iterator  operator++(int) { auto __tmp = *this; ++*this; return __tmp; }

    iterator& operator--();
    iterator  operator--(int) { auto __tmp = *this; --*this; return __tmp; }

    friend bool operator==(const iterator& __lhs, const iterator& __rhs)
    { return __lhs._M_equals(__rhs); }

    friend bool operator!=(const iterator& __lhs, const iterator& __rhs)
    { return !__lhs._M_equals(__rhs); }

  private:
    friend class path;

    iterator(const path* __path, path::_List::const_iterator __iter)
    : _M_path(__path), _M_cur(__iter), _M_at_end()
    { }

    iterator(const path* __path, bool __at_end)
    : _M_path(__path), _M_cur(), _M_at_end(__at_end)
    { }

    bool _M_equals(iterator) const;

    const path* 		_M_path;
    path::_List::const_iterator _M_cur;
    bool			_M_at_end;  // only used when type != _Multi
  };


  inline path&
  path::operator=(path&& __p) noexcept
  {
    _M_pathname = std::move(__p._M_pathname);
    _M_cmpts = std::move(__p._M_cmpts);
    _M_type = __p._M_type;
    __p.clear();
    return *this;
  }

  inline path&
  path::operator=(string_type&& __source)
  { return *this = path(std::move(__source)); }

  inline path&
  path::assign(string_type&& __source)
  { return *this = path(std::move(__source)); }

  inline path&
  path::operator+=(const path& __p)
  {
    return operator+=(__p.native());
  }

  inline path&
  path::operator+=(const string_type& __x)
  {
    _M_pathname += __x;
    _M_split_cmpts();
    return *this;
  }

  inline path&
  path::operator+=(const value_type* __x)
  {
    _M_pathname += __x;
    _M_split_cmpts();
    return *this;
  }

  inline path&
  path::operator+=(value_type __x)
  {
    _M_pathname += __x;
    _M_split_cmpts();
    return *this;
  }

#if __cplusplus >= 201402L
  inline path&
  path::operator+=(basic_string_view<value_type> __x)
  {
    _M_pathname.append(__x.data(), __x.size());
    _M_split_cmpts();
    return *this;
  }
#endif

  template<typename _CharT>
    inline __detail::_Path<_CharT*, _CharT*>&
    path::operator+=(_CharT __x)
    {
      auto* __addr = std::__addressof(__x);
      return concat(__addr, __addr + 1);
    }

  inline path&
  path::make_preferred()
  {
#ifdef _GLIBCXX_FILESYSTEM_IS_WINDOWS
    std::replace(_M_pathname.begin(), _M_pathname.end(), L'/',
		 preferred_separator);
#endif
    return *this;
  }

  inline void path::swap(path& __rhs) noexcept
  {
    _M_pathname.swap(__rhs._M_pathname);
    _M_cmpts.swap(__rhs._M_cmpts);
    std::swap(_M_type, __rhs._M_type);
  }

  template<typename _CharT, typename _Traits, typename _Allocator>
    inline std::basic_string<_CharT, _Traits, _Allocator>
    path::string(const _Allocator& __a) const
    {
      if (is_same<_CharT, value_type>::value)
	return { _M_pathname.begin(), _M_pathname.end(), __a };

      using _WString = basic_string<_CharT, _Traits, _Allocator>;

      const value_type* __first = _M_pathname.data();
      const value_type* __last = __first + _M_pathname.size();

#ifdef _GLIBCXX_FILESYSTEM_IS_WINDOWS
      using _CharAlloc = __alloc_rebind<_Allocator, char>;
      using _String = basic_string<char, char_traits<char>, _CharAlloc>;

      // First convert native string from UTF-16 to to UTF-8.
      // XXX This assumes that the execution wide-character set is UTF-16.
      codecvt_utf8_utf16<value_type> __cvt;
      _String __u8str{_CharAlloc{__a}};
      if (__str_codecvt_out_all(__first, __last, __u8str, __cvt))
	{
	  struct
	  {
	    const _String*
	    operator()(const _String& __from, _String&, true_type)
	    { return std::__addressof(__from); }

	    _WString*
	    operator()(const _String& __from, _WString& __to, false_type)
	    {
#ifdef _GLIBCXX_USE_CHAR8_T
	      if constexpr (is_same<_CharT, char8_t>::value)
	        {
	          __to.assign(__from.begin(), __from.end());
	          return std::__addressof(__to);
	        }
	      else
#endif
	        {
	          // Convert UTF-8 to wide string.
	          struct _UCvt : std::codecvt<_CharT, char, std::mbstate_t>
		  { } __cvt;
	          const char* __f = __from.data();
	          const char* __l = __f + __from.size();
	          if (__str_codecvt_in_all(__f, __l, __to, __cvt))
		    return std::__addressof(__to);
	        }
	      return nullptr;
	    }
	  } __dispatch;
	  _WString __wstr(__a);
	  if (auto* __p = __dispatch(__u8str, __wstr, is_same<_CharT, char>{}))
	    return *__p;
	}
#else
#ifdef _GLIBCXX_USE_CHAR8_T
      if constexpr (is_same<_CharT, char8_t>::value)
          return _WString(__first, __last, __a);
      else
#endif
        {
          struct _UCvt : std::codecvt<_CharT, char, std::mbstate_t> { } __cvt;
          _WString __wstr(__a);
          if (__str_codecvt_in_all(__first, __last, __wstr, __cvt))
	    return __wstr;
        }
#endif
      _GLIBCXX_THROW_OR_ABORT(filesystem_error(
	    "Cannot convert character sequence",
	    std::make_error_code(errc::illegal_byte_sequence)));
    }

  inline std::string
  path::string() const { return string<char>(); }

#if _GLIBCXX_USE_WCHAR_T
  inline std::wstring
  path::wstring() const { return string<wchar_t>(); }
#endif

#ifdef _GLIBCXX_USE_CHAR8_T
  inline std::u8string
  path::u8string() const { return string<char8_t>(); }
#else
  inline std::string
  path::u8string() const
  {
#ifdef _GLIBCXX_FILESYSTEM_IS_WINDOWS
    std::string __str;
    // convert from native wide encoding (assumed to be UTF-16) to UTF-8
    std::codecvt_utf8_utf16<value_type> __cvt;
    const value_type* __first = _M_pathname.data();
    const value_type* __last = __first + _M_pathname.size();
    if (__str_codecvt_out_all(__first, __last, __str, __cvt))
      return __str;
    _GLIBCXX_THROW_OR_ABORT(filesystem_error(
	  "Cannot convert character sequence",
	  std::make_error_code(errc::illegal_byte_sequence)));
#else
    return _M_pathname;
#endif
  }
#endif // _GLIBCXX_USE_CHAR8_T

  inline std::u16string
  path::u16string() const { return string<char16_t>(); }

  inline std::u32string
  path::u32string() const { return string<char32_t>(); }

  template<typename _CharT, typename _Traits, typename _Allocator>
    inline std::basic_string<_CharT, _Traits, _Allocator>
    path::generic_string(const _Allocator& __a) const
    {
#ifdef _GLIBCXX_FILESYSTEM_IS_WINDOWS
      const _CharT __slash = is_same<_CharT, wchar_t>::value
	? _CharT(L'/')
	: _CharT('/'); // Assume value is correct for the encoding.
#else
      const _CharT __slash = _CharT('/');
#endif
      basic_string<_CharT, _Traits, _Allocator> __str(__a);
      __str.reserve(_M_pathname.size());
      bool __add_slash = false;
      for (auto& __elem : *this)
	{
	  if (__elem._M_type == _Type::_Root_dir)
	    {
	      __str += __slash;
	      continue;
	    }
	  if (__add_slash)
	    __str += __slash;
	  __str += __elem.string<_CharT, _Traits, _Allocator>(__a);
	  __add_slash = __elem._M_type == _Type::_Filename;
	}
      return __str;
    }

  inline std::string
  path::generic_string() const { return generic_string<char>(); }

#if _GLIBCXX_USE_WCHAR_T
  inline std::wstring
  path::generic_wstring() const { return generic_string<wchar_t>(); }
#endif

#ifdef _GLIBCXX_USE_CHAR8_T
  inline std::u8string
  path::generic_u8string() const { return generic_string<char8_t>(); }
#else
  inline std::string
  path::generic_u8string() const { return generic_string<char>(); }
#endif

  inline std::u16string
  path::generic_u16string() const { return generic_string<char16_t>(); }

  inline std::u32string
  path::generic_u32string() const { return generic_string<char32_t>(); }

  inline int
  path::compare(const string_type& __s) const { return compare(path(__s)); }

  inline int
  path::compare(const value_type* __s) const { return compare(path(__s)); }

#if __cplusplus >= 201402L
  inline int
  path::compare(basic_string_view<value_type> __s) const
  { return compare(path(__s)); }
#endif

  inline path
  path::filename() const { return empty() ? path() : *--end(); }

  inline path
  path::stem() const
  {
    auto ext = _M_find_extension();
    if (ext.first && ext.second != 0)
      return path{ext.first->substr(0, ext.second)};
    return {};
  }

  inline path
  path::extension() const
  {
    auto ext = _M_find_extension();
    if (ext.first && ext.second != string_type::npos)
      return path{ext.first->substr(ext.second)};
    return {};
  }

  inline bool
  path::has_stem() const
  {
    auto ext = _M_find_extension();
    return ext.first && ext.second != 0;
  }

  inline bool
  path::has_extension() const
  {
    auto ext = _M_find_extension();
    return ext.first && ext.second != string_type::npos;
  }

  inline bool
  path::is_absolute() const
  {
#ifdef _GLIBCXX_FILESYSTEM_IS_WINDOWS
    return has_root_name() && has_root_directory();
#else
    return has_root_directory();
#endif
  }

  inline path::iterator
  path::begin() const
  {
    if (_M_type == _Type::_Multi)
      return iterator(this, _M_cmpts.begin());
    return iterator(this, false);
  }

  inline path::iterator
  path::end() const
  {
    if (_M_type == _Type::_Multi)
      return iterator(this, _M_cmpts.end());
    return iterator(this, true);
  }

  inline path::iterator&
  path::iterator::operator++()
  {
    __glibcxx_assert(_M_path != nullptr);
    if (_M_path->_M_type == _Type::_Multi)
      {
	__glibcxx_assert(_M_cur != _M_path->_M_cmpts.end());
	++_M_cur;
      }
    else
      {
	__glibcxx_assert(!_M_at_end);
	_M_at_end = true;
      }
    return *this;
  }

  inline path::iterator&
  path::iterator::operator--()
  {
    __glibcxx_assert(_M_path != nullptr);
    if (_M_path->_M_type == _Type::_Multi)
      {
	__glibcxx_assert(_M_cur != _M_path->_M_cmpts.begin());
	--_M_cur;
      }
    else
      {
	__glibcxx_assert(_M_at_end);
	_M_at_end = false;
      }
    return *this;
  }

  inline path::iterator::reference
  path::iterator::operator*() const
  {
    __glibcxx_assert(_M_path != nullptr);
    if (_M_path->_M_type == _Type::_Multi)
      {
	__glibcxx_assert(_M_cur != _M_path->_M_cmpts.end());
	return *_M_cur;
      }
    return *_M_path;
  }

  inline bool
  path::iterator::_M_equals(iterator __rhs) const
  {
    if (_M_path != __rhs._M_path)
      return false;
    if (_M_path == nullptr)
      return true;
    if (_M_path->_M_type == path::_Type::_Multi)
      return _M_cur == __rhs._M_cur;
    return _M_at_end == __rhs._M_at_end;
  }

  // @} group filesystem-ts
_GLIBCXX_END_NAMESPACE_CXX11
} // namespace v1
} // namespace filesystem
} // namespace experimental

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif // C++11

#endif // _GLIBCXX_EXPERIMENTAL_FS_PATH_H
