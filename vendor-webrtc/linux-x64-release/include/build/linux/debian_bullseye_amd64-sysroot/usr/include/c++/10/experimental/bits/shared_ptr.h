// Experimental shared_ptr with array support -*- C++ -*-

// Copyright (C) 2015-2020 Free Software Foundation, Inc.
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

/** @file experimental/bits/shared_ptr.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{experimental/memory}
 */

#ifndef _GLIBCXX_EXPERIMENTAL_SHARED_PTR_H
#define _GLIBCXX_EXPERIMENTAL_SHARED_PTR_H 1

#pragma GCC system_header

#if __cplusplus >= 201402L

#include <memory>
#include <experimental/type_traits>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

namespace experimental
{
inline namespace fundamentals_v2
{
  // 8.2.1

  template<typename _Tp> class shared_ptr;
  template<typename _Tp> class weak_ptr;
  template<typename _Tp> class enable_shared_from_this;

  template<typename _Yp, typename _Tp>
    constexpr bool __sp_compatible_v
      = std::__sp_compatible_with<_Yp*, _Tp*>::value;

  template<typename _Tp, typename _Yp>
    constexpr bool __sp_is_constructible_v
      = std::__sp_is_constructible<_Tp, _Yp>::value;

  template<typename _Tp>
    class shared_ptr : public __shared_ptr<_Tp>
    {
      using _Base_type = __shared_ptr<_Tp>;

    public:
      using element_type = typename _Base_type::element_type;

    private:
      // Constraint for construction from a pointer of type _Yp*:
      template<typename _Yp>
	using _SafeConv = enable_if_t<__sp_is_constructible_v<_Tp, _Yp>>;

      template<typename _Tp1, typename _Res = void>
	using _Compatible
	  = enable_if_t<__sp_compatible_v<_Tp1, _Tp>, _Res>;

      template<typename _Tp1, typename _Del,
	       typename _Ptr = typename unique_ptr<_Tp1, _Del>::pointer,
	       typename _Res = void>
	using _UniqCompatible = enable_if_t<
	  __sp_compatible_v<_Tp1, _Tp>
	  && experimental::is_convertible_v<_Ptr, element_type*>,
	  _Res>;

    public:

      // 8.2.1.1, shared_ptr constructors
      constexpr shared_ptr() noexcept = default;

      template<typename _Tp1, typename = _SafeConv<_Tp1>>
	explicit
	shared_ptr(_Tp1* __p) : _Base_type(__p)
	{ _M_enable_shared_from_this_with(__p); }

      template<typename _Tp1, typename _Deleter, typename = _SafeConv<_Tp1>>
	shared_ptr(_Tp1* __p, _Deleter __d)
	: _Base_type(__p, __d)
	{ _M_enable_shared_from_this_with(__p); }

      template<typename _Tp1, typename _Deleter, typename _Alloc,
	       typename = _SafeConv<_Tp1>>
	shared_ptr(_Tp1* __p, _Deleter __d, _Alloc __a)
	: _Base_type(__p, __d, __a)
	{ _M_enable_shared_from_this_with(__p); }

      template<typename _Deleter>
	shared_ptr(nullptr_t __p, _Deleter __d)
	: _Base_type(__p, __d) { }

      template<typename _Deleter, typename _Alloc>
	shared_ptr(nullptr_t __p, _Deleter __d, _Alloc __a)
	: _Base_type(__p, __d, __a) { }

      template<typename _Tp1>
	shared_ptr(const shared_ptr<_Tp1>& __r, element_type* __p) noexcept
	: _Base_type(__r, __p) { }

      shared_ptr(const shared_ptr& __r) noexcept
	: _Base_type(__r) { }

      template<typename _Tp1, typename = _Compatible<_Tp1>>
	shared_ptr(const shared_ptr<_Tp1>& __r) noexcept
	: _Base_type(__r) { }

      shared_ptr(shared_ptr&& __r) noexcept
      : _Base_type(std::move(__r)) { }

      template<typename _Tp1, typename = _Compatible<_Tp1>>
	shared_ptr(shared_ptr<_Tp1>&& __r) noexcept
	: _Base_type(std::move(__r)) { }

      template<typename _Tp1, typename = _Compatible<_Tp1>>
	explicit
	shared_ptr(const weak_ptr<_Tp1>& __r)
	: _Base_type(__r) { }

#if _GLIBCXX_USE_DEPRECATED
      template<typename _Tp1, typename = _Compatible<_Tp1>>
	shared_ptr(std::auto_ptr<_Tp1>&& __r)
	: _Base_type(std::move(__r))
	{ _M_enable_shared_from_this_with(static_cast<_Tp1*>(this->get())); }
#endif

      template<typename _Tp1, typename _Del,
	       typename = _UniqCompatible<_Tp1, _Del>>
	shared_ptr(unique_ptr<_Tp1, _Del>&& __r)
	: _Base_type(std::move(__r))
	{
	  // XXX assume conversion from __r.get() to this->get() to __elem_t*
	  // is a round trip, which might not be true in all cases.
	  using __elem_t = typename unique_ptr<_Tp1, _Del>::element_type;
	  _M_enable_shared_from_this_with(static_cast<__elem_t*>(this->get()));
	}

      constexpr shared_ptr(nullptr_t __p)
      : _Base_type(__p) { }

      // C++14 20.8.2.2
      ~shared_ptr() = default;

      // C++14 20.8.2.3
      shared_ptr& operator=(const shared_ptr&) noexcept = default;

      template <typename _Tp1>
	_Compatible<_Tp1, shared_ptr&>
	operator=(const shared_ptr<_Tp1>& __r) noexcept
	{
	  _Base_type::operator=(__r);
	  return *this;
	}

      shared_ptr&
      operator=(shared_ptr&& __r) noexcept
      {
	_Base_type::operator=(std::move(__r));
	return *this;
      }

      template <typename _Tp1>
	_Compatible<_Tp1, shared_ptr&>
	operator=(shared_ptr<_Tp1>&& __r) noexcept
	{
	  _Base_type::operator=(std::move(__r));
	  return *this;
	}

#if _GLIBCXX_USE_DEPRECATED
      template<typename _Tp1>
	_Compatible<_Tp1, shared_ptr&>
	operator=(std::auto_ptr<_Tp1>&& __r)
	{
	  __shared_ptr<_Tp>::operator=(std::move(__r));
	  return *this;
	}
#endif

      template <typename _Tp1, typename _Del>
	_UniqCompatible<_Tp1, _Del, shared_ptr&>
	operator=(unique_ptr<_Tp1, _Del>&& __r)
	{
	  _Base_type::operator=(std::move(__r));
	  return *this;
	}

      // C++14 20.8.2.2.4
      // swap & reset
      // 8.2.1.2 shared_ptr observers
      // in __shared_ptr

    private:
      template<typename _Alloc, typename... _Args>
	shared_ptr(_Sp_make_shared_tag __tag, const _Alloc& __a,
		   _Args&&... __args)
	: _Base_type(__tag, __a, std::forward<_Args>(__args)...)
	{ _M_enable_shared_from_this_with(this->get()); }

      template<typename _Tp1, typename _Alloc, typename... _Args>
	friend shared_ptr<_Tp1>
	allocate_shared(const _Alloc& __a, _Args&&...  __args);

      shared_ptr(const weak_ptr<_Tp>& __r, std::nothrow_t)
      : _Base_type(__r, std::nothrow) { }

      friend class weak_ptr<_Tp>;

      template<typename _Yp>
	using __esft_base_t =
	  decltype(__expt_enable_shared_from_this_base(std::declval<_Yp*>()));

      // Detect an accessible and unambiguous enable_shared_from_this base.
      template<typename _Yp, typename = void>
	struct __has_esft_base
	: false_type { };

      template<typename _Yp>
	struct __has_esft_base<_Yp, __void_t<__esft_base_t<_Yp>>>
	: __bool_constant<!is_array_v<_Tp>> { };  // ignore base for arrays

      template<typename _Yp>
	typename enable_if<__has_esft_base<_Yp>::value>::type
	_M_enable_shared_from_this_with(const _Yp* __p) noexcept
	{
	  if (auto __base = __expt_enable_shared_from_this_base(__p))
	    {
	      __base->_M_weak_this
		= shared_ptr<_Yp>(*this, const_cast<_Yp*>(__p));
	    }
	}

      template<typename _Yp>
	typename enable_if<!__has_esft_base<_Yp>::value>::type
	_M_enable_shared_from_this_with(const _Yp*) noexcept
	{ }
    };

  // C++14 20.8.2.2.7
  template<typename _Tp1, typename _Tp2>
    bool operator==(const shared_ptr<_Tp1>& __a,
		    const shared_ptr<_Tp2>& __b) noexcept
    { return __a.get() == __b.get(); }

  template<typename _Tp>
    inline bool
    operator==(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    { return !__a; }

  template<typename _Tp>
    inline bool
    operator==(nullptr_t, const shared_ptr<_Tp>& __a) noexcept
    { return !__a; }

  template<typename _Tp1, typename _Tp2>
    inline bool
    operator!=(const shared_ptr<_Tp1>& __a,
	       const shared_ptr<_Tp2>& __b) noexcept
    { return __a.get() != __b.get(); }

  template<typename _Tp>
    inline bool
    operator!=(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    { return (bool)__a; }

  template<typename _Tp>
    inline bool
    operator!=(nullptr_t, const shared_ptr<_Tp>& __a) noexcept
    { return (bool)__a; }

  template<typename _Tp1, typename _Tp2>
    inline bool
    operator<(const shared_ptr<_Tp1>& __a,
	      const shared_ptr<_Tp2>& __b) noexcept
    {
      using __elem_t1 = typename shared_ptr<_Tp1>::element_type;
      using __elem_t2 = typename shared_ptr<_Tp2>::element_type;
      using _CT = common_type_t<__elem_t1*, __elem_t2*>;
      return std::less<_CT>()(__a.get(), __b.get());
    }

  template<typename _Tp>
    inline bool
    operator<(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    {
      using __elem_t = typename shared_ptr<_Tp>::element_type;
      return std::less<__elem_t*>()(__a.get(), nullptr);
    }

  template<typename _Tp>
    inline bool
    operator<(nullptr_t, const shared_ptr<_Tp>& __a) noexcept
    {
      using __elem_t = typename shared_ptr<_Tp>::element_type;
      return std::less<__elem_t*>()(nullptr, __a.get());
    }

  template<typename _Tp1, typename _Tp2>
    inline bool
    operator<=(const shared_ptr<_Tp1>& __a,
	       const shared_ptr<_Tp2>& __b) noexcept
    { return !(__b < __a); }

  template<typename _Tp>
    inline bool
    operator<=(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    { return !(nullptr < __a); }

  template<typename _Tp>
    inline bool
    operator<=(nullptr_t, const shared_ptr<_Tp>& __a) noexcept
    { return !(__a < nullptr); }

  template<typename _Tp1, typename _Tp2>
    inline bool
    operator>(const shared_ptr<_Tp1>& __a,
	      const shared_ptr<_Tp2>& __b) noexcept
    { return (__b < __a); }

  template<typename _Tp>
    inline bool
    operator>(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    {
      using __elem_t = typename shared_ptr<_Tp>::element_type;
      return std::less<__elem_t*>()(nullptr, __a.get());
    }

  template<typename _Tp>
    inline bool
    operator>(nullptr_t, const shared_ptr<_Tp>& __a) noexcept
    {
      using __elem_t = typename shared_ptr<_Tp>::element_type;
      return std::less<__elem_t*>()(__a.get(), nullptr);
    }

  template<typename _Tp1, typename _Tp2>
    inline bool
    operator>=(const shared_ptr<_Tp1>& __a,
	       const shared_ptr<_Tp2>& __b) noexcept
    { return !(__a < __b); }

  template<typename _Tp>
    inline bool
    operator>=(const shared_ptr<_Tp>& __a, nullptr_t) noexcept
    { return !(__a < nullptr); }

  template<typename _Tp>
    inline bool
    operator>=(nullptr_t, const shared_ptr<_Tp>& __a) noexcept
    { return !(nullptr < __a); }

  // C++14 20.8.2.2.8
  template<typename _Tp>
    inline void
    swap(shared_ptr<_Tp>& __a, shared_ptr<_Tp>& __b) noexcept
    { __a.swap(__b); }

  // 8.2.1.3, shared_ptr casts
  template<typename _Tp, typename _Tp1>
    inline shared_ptr<_Tp>
    static_pointer_cast(const shared_ptr<_Tp1>& __r) noexcept
    {
      using __elem_t = typename shared_ptr<_Tp>::element_type;
      return shared_ptr<_Tp>(__r, static_cast<__elem_t*>(__r.get()));
    }

  template<typename _Tp, typename _Tp1>
    inline shared_ptr<_Tp>
    dynamic_pointer_cast(const shared_ptr<_Tp1>& __r) noexcept
    {
      using __elem_t = typename shared_ptr<_Tp>::element_type;
      if (_Tp* __p = dynamic_cast<__elem_t*>(__r.get()))
	return shared_ptr<_Tp>(__r, __p);
      return shared_ptr<_Tp>();
    }

  template<typename _Tp, typename _Tp1>
    inline shared_ptr<_Tp>
    const_pointer_cast(const shared_ptr<_Tp1>& __r) noexcept
    {
      using __elem_t = typename shared_ptr<_Tp>::element_type;
      return shared_ptr<_Tp>(__r, const_cast<__elem_t*>(__r.get()));
    }

  template<typename _Tp, typename _Tp1>
    inline shared_ptr<_Tp>
    reinterpret_pointer_cast(const shared_ptr<_Tp1>& __r) noexcept
    {
      using __elem_t = typename shared_ptr<_Tp>::element_type;
      return shared_ptr<_Tp>(__r, reinterpret_cast<__elem_t*>(__r.get()));
    }

  // C++14 20.8.2.3
  template<typename _Tp>
    class weak_ptr : public __weak_ptr<_Tp>
    {
      template<typename _Tp1, typename _Res = void>
	using _Compatible = enable_if_t<__sp_compatible_v<_Tp1, _Tp>, _Res>;

      using _Base_type = __weak_ptr<_Tp>;

     public:
       constexpr weak_ptr() noexcept = default;

       template<typename _Tp1, typename = _Compatible<_Tp1>>
	 weak_ptr(const shared_ptr<_Tp1>& __r) noexcept
	 : _Base_type(__r) { }

       weak_ptr(const weak_ptr&) noexcept = default;

       template<typename _Tp1, typename = _Compatible<_Tp1>>
	 weak_ptr(const weak_ptr<_Tp1>& __r) noexcept
	 : _Base_type(__r) { }

       weak_ptr(weak_ptr&&) noexcept = default;

       template<typename _Tp1, typename = _Compatible<_Tp1>>
	 weak_ptr(weak_ptr<_Tp1>&& __r) noexcept
	 : _Base_type(std::move(__r)) { }

       weak_ptr&
       operator=(const weak_ptr& __r) noexcept = default;

       template<typename _Tp1>
	 _Compatible<_Tp1, weak_ptr&>
	 operator=(const weak_ptr<_Tp1>& __r) noexcept
	 {
	   this->_Base_type::operator=(__r);
	   return *this;
	 }

       template<typename _Tp1>
	 _Compatible<_Tp1, weak_ptr&>
	 operator=(const shared_ptr<_Tp1>& __r) noexcept
	 {
	   this->_Base_type::operator=(__r);
	   return *this;
	 }

       weak_ptr&
       operator=(weak_ptr&& __r) noexcept = default;

       template<typename _Tp1>
	 _Compatible<_Tp1, weak_ptr&>
	 operator=(weak_ptr<_Tp1>&& __r) noexcept
	 {
	   this->_Base_type::operator=(std::move(__r));
	   return *this;
	 }

       shared_ptr<_Tp>
       lock() const noexcept
       { return shared_ptr<_Tp>(*this, std::nothrow); }

       friend class enable_shared_from_this<_Tp>;
    };

  // C++14 20.8.2.3.6
  template<typename _Tp>
    inline void
    swap(weak_ptr<_Tp>& __a, weak_ptr<_Tp>& __b) noexcept
    { __a.swap(__b); }

  /// C++14 20.8.2.2.10
  template<typename _Del, typename _Tp>
    inline _Del*
    get_deleter(const shared_ptr<_Tp>& __p) noexcept
    { return std::get_deleter<_Del>(__p); }

  // C++14 20.8.2.2.11
  template<typename _Ch, typename _Tr, typename _Tp>
    inline std::basic_ostream<_Ch, _Tr>&
    operator<<(std::basic_ostream<_Ch, _Tr>& __os, const shared_ptr<_Tp>& __p)
    {
      __os << __p.get();
      return __os;
    }

  // C++14 20.8.2.4
  template<typename _Tp = void> class owner_less;

   /// Partial specialization of owner_less for shared_ptr.
  template<typename _Tp>
    struct owner_less<shared_ptr<_Tp>>
    : public _Sp_owner_less<shared_ptr<_Tp>, weak_ptr<_Tp>>
    { };

  /// Partial specialization of owner_less for weak_ptr.
  template<typename _Tp>
    struct owner_less<weak_ptr<_Tp>>
    : public _Sp_owner_less<weak_ptr<_Tp>, shared_ptr<_Tp>>
    { };

  template<>
    class owner_less<void>
    {
      template<typename _Tp, typename _Up>
        bool
        operator()(shared_ptr<_Tp> const& __lhs,
                   shared_ptr<_Up> const& __rhs) const
        { return __lhs.owner_before(__rhs); }

      template<typename _Tp, typename _Up>
        bool
        operator()(shared_ptr<_Tp> const& __lhs,
                   weak_ptr<_Up> const& __rhs) const
        { return __lhs.owner_before(__rhs); }

      template<typename _Tp, typename _Up>
        bool
        operator()(weak_ptr<_Tp> const& __lhs,
                   shared_ptr<_Up> const& __rhs) const
        { return __lhs.owner_before(__rhs); }

      template<typename _Tp, typename _Up>
        bool
        operator()(weak_ptr<_Tp> const& __lhs,
                   weak_ptr<_Up> const& __rhs) const
        { return __lhs.owner_before(__rhs); }

      typedef void is_transparent;
    };

   // C++14 20.8.2.6
   template<typename _Tp>
     inline bool
     atomic_is_lock_free(const shared_ptr<_Tp>* __p)
     { return std::atomic_is_lock_free<_Tp, __default_lock_policy>(__p); }

   template<typename _Tp>
     shared_ptr<_Tp> atomic_load(const shared_ptr<_Tp>* __p)
     { return std::atomic_load<_Tp>(__p); }

   template<typename _Tp>
     shared_ptr<_Tp>
     atomic_load_explicit(const shared_ptr<_Tp>* __p, memory_order __mo)
     { return std::atomic_load_explicit<_Tp>(__p, __mo); }

   template<typename _Tp>
     void atomic_store(shared_ptr<_Tp>* __p, shared_ptr<_Tp> __r)
     { return std::atomic_store<_Tp>(__p, __r); }

   template<typename _Tp>
     shared_ptr<_Tp>
     atomic_store_explicit(const shared_ptr<_Tp>* __p,
			   shared_ptr<_Tp> __r,
			   memory_order __mo)
     { return std::atomic_store_explicit<_Tp>(__p, __r, __mo); }

   template<typename _Tp>
     void atomic_exchange(shared_ptr<_Tp>* __p, shared_ptr<_Tp> __r)
     { return std::atomic_exchange<_Tp>(__p, __r); }

   template<typename _Tp>
     shared_ptr<_Tp>
     atomic_exchange_explicit(const shared_ptr<_Tp>* __p,
			      shared_ptr<_Tp> __r,
			      memory_order __mo)
     { return std::atomic_exchange_explicit<_Tp>(__p, __r, __mo); }

   template<typename _Tp>
     bool atomic_compare_exchange_weak(shared_ptr<_Tp>* __p,
				       shared_ptr<_Tp>* __v,
				       shared_ptr<_Tp> __w)
     { return std::atomic_compare_exchange_weak<_Tp>(__p, __v, __w); }

   template<typename _Tp>
     bool atomic_compare_exchange_strong(shared_ptr<_Tp>* __p,
					 shared_ptr<_Tp>* __v,
					 shared_ptr<_Tp> __w)
     { return std::atomic_compare_exchange_strong<_Tp>(__p, __v, __w); }

   template<typename _Tp>
     bool atomic_compare_exchange_weak_explicit(shared_ptr<_Tp>* __p,
						shared_ptr<_Tp>* __v,
						shared_ptr<_Tp> __w,
						memory_order __success,
						memory_order __failure)
     { return std::atomic_compare_exchange_weak_explicit<_Tp>(__p, __v, __w,
							      __success,
							      __failure); }

   template<typename _Tp>
     bool atomic_compare_exchange_strong_explicit(shared_ptr<_Tp>* __p,
						  shared_ptr<_Tp>* __v,
						  shared_ptr<_Tp> __w,
						  memory_order __success,
						  memory_order __failure)
     { return std::atomic_compare_exchange_strong_explicit<_Tp>(__p, __v, __w,
								__success,
								__failure); }

  //enable_shared_from_this
  template<typename _Tp>
    class enable_shared_from_this
    {
    protected:
      constexpr enable_shared_from_this() noexcept { }

      enable_shared_from_this(const enable_shared_from_this&) noexcept { }

      enable_shared_from_this&
      operator=(const enable_shared_from_this&) noexcept
      { return *this; }

      ~enable_shared_from_this() { }

    public:
      shared_ptr<_Tp>
      shared_from_this()
      { return shared_ptr<_Tp>(this->_M_weak_this); }

      shared_ptr<const _Tp>
      shared_from_this() const
      { return shared_ptr<const _Tp>(this->_M_weak_this); }

      weak_ptr<_Tp>
      weak_from_this() noexcept
      { return _M_weak_this; }

      weak_ptr<const _Tp>
      weak_from_this() const noexcept
      { return _M_weak_this; }

    private:
      template<typename _Tp1>
	void
	_M_weak_assign(_Tp1* __p, const __shared_count<>& __n) const noexcept
	{ _M_weak_this._M_assign(__p, __n); }

      // Found by ADL when this is an associated class.
      friend const enable_shared_from_this*
      __expt_enable_shared_from_this_base(const enable_shared_from_this* __p)
      { return __p; }

      template<typename>
	friend class shared_ptr;

      mutable weak_ptr<_Tp> _M_weak_this;
    };
} // namespace fundamentals_v2
} // namespace experimental

  /// std::hash specialization for shared_ptr.
  template<typename _Tp>
    struct hash<experimental::shared_ptr<_Tp>>
    : public __hash_base<size_t, experimental::shared_ptr<_Tp>>
    {
      size_t
      operator()(const experimental::shared_ptr<_Tp>& __s) const noexcept
      { return std::hash<_Tp*>()(__s.get()); }
    };

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif // __cplusplus <= 201103L

#endif // _GLIBCXX_EXPERIMENTAL_SHARED_PTR_H
