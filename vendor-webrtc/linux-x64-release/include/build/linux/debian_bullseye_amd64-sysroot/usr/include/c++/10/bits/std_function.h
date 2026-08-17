// Implementation of std::function -*- C++ -*-

// Copyright (C) 2004-2020 Free Software Foundation, Inc.
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

/** @file include/bits/std_function.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{functional}
 */

#ifndef _GLIBCXX_STD_FUNCTION_H
#define _GLIBCXX_STD_FUNCTION_H 1

#pragma GCC system_header

#if __cplusplus < 201103L
# include <bits/c++0x_warning.h>
#else

#if __cpp_rtti
# include <typeinfo>
#endif
#include <bits/stl_function.h>
#include <bits/invoke.h>
#include <bits/refwrap.h>
#include <bits/functexcept.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   *  @brief Exception class thrown when class template function's
   *  operator() is called with an empty target.
   *  @ingroup exceptions
   */
  class bad_function_call : public std::exception
  {
  public:
    virtual ~bad_function_call() noexcept;

    const char* what() const noexcept;
  };

  /**
   *  Trait identifying "location-invariant" types, meaning that the
   *  address of the object (or any of its members) will not escape.
   *  Trivially copyable types are location-invariant and users can
   *  specialize this trait for other types.
   */
  template<typename _Tp>
    struct __is_location_invariant
    : is_trivially_copyable<_Tp>::type
    { };

  class _Undefined_class;

  union _Nocopy_types
  {
    void*       _M_object;
    const void* _M_const_object;
    void (*_M_function_pointer)();
    void (_Undefined_class::*_M_member_pointer)();
  };

  union [[gnu::may_alias]] _Any_data
  {
    void*       _M_access()       { return &_M_pod_data[0]; }
    const void* _M_access() const { return &_M_pod_data[0]; }

    template<typename _Tp>
      _Tp&
      _M_access()
      { return *static_cast<_Tp*>(_M_access()); }

    template<typename _Tp>
      const _Tp&
      _M_access() const
      { return *static_cast<const _Tp*>(_M_access()); }

    _Nocopy_types _M_unused;
    char _M_pod_data[sizeof(_Nocopy_types)];
  };

  enum _Manager_operation
  {
    __get_type_info,
    __get_functor_ptr,
    __clone_functor,
    __destroy_functor
  };

  template<typename _Signature>
    class function;

  /// Base class of all polymorphic function object wrappers.
  class _Function_base
  {
  public:
    static const size_t _M_max_size = sizeof(_Nocopy_types);
    static const size_t _M_max_align = __alignof__(_Nocopy_types);

    template<typename _Functor>
      class _Base_manager
      {
      protected:
	static const bool __stored_locally =
	(__is_location_invariant<_Functor>::value
	 && sizeof(_Functor) <= _M_max_size
	 && __alignof__(_Functor) <= _M_max_align
	 && (_M_max_align % __alignof__(_Functor) == 0));

	typedef integral_constant<bool, __stored_locally> _Local_storage;

	// Retrieve a pointer to the function object
	static _Functor*
	_M_get_pointer(const _Any_data& __source)
	{
	  if _GLIBCXX17_CONSTEXPR (__stored_locally)
	    {
	      const _Functor& __f = __source._M_access<_Functor>();
	      return const_cast<_Functor*>(std::__addressof(__f));
	    }
	  else // have stored a pointer
	    return __source._M_access<_Functor*>();
	}

	// Clone a location-invariant function object that fits within
	// an _Any_data structure.
	static void
	_M_clone(_Any_data& __dest, const _Any_data& __source, true_type)
	{
	  ::new (__dest._M_access()) _Functor(__source._M_access<_Functor>());
	}

	// Clone a function object that is not location-invariant or
	// that cannot fit into an _Any_data structure.
	static void
	_M_clone(_Any_data& __dest, const _Any_data& __source, false_type)
	{
	  __dest._M_access<_Functor*>() =
	    new _Functor(*__source._M_access<const _Functor*>());
	}

	// Destroying a location-invariant object may still require
	// destruction.
	static void
	_M_destroy(_Any_data& __victim, true_type)
	{
	  __victim._M_access<_Functor>().~_Functor();
	}

	// Destroying an object located on the heap.
	static void
	_M_destroy(_Any_data& __victim, false_type)
	{
	  delete __victim._M_access<_Functor*>();
	}

      public:
	static bool
	_M_manager(_Any_data& __dest, const _Any_data& __source,
		   _Manager_operation __op)
	{
	  switch (__op)
	    {
#if __cpp_rtti
	    case __get_type_info:
	      __dest._M_access<const type_info*>() = &typeid(_Functor);
	      break;
#endif
	    case __get_functor_ptr:
	      __dest._M_access<_Functor*>() = _M_get_pointer(__source);
	      break;

	    case __clone_functor:
	      _M_clone(__dest, __source, _Local_storage());
	      break;

	    case __destroy_functor:
	      _M_destroy(__dest, _Local_storage());
	      break;
	    }
	  return false;
	}

	static void
	_M_init_functor(_Any_data& __functor, _Functor&& __f)
	{ _M_init_functor(__functor, std::move(__f), _Local_storage()); }

	template<typename _Signature>
	  static bool
	  _M_not_empty_function(const function<_Signature>& __f)
	  { return static_cast<bool>(__f); }

	template<typename _Tp>
	  static bool
	  _M_not_empty_function(_Tp* __fp)
	  { return __fp != nullptr; }

	template<typename _Class, typename _Tp>
	  static bool
	  _M_not_empty_function(_Tp _Class::* __mp)
	  { return __mp != nullptr; }

	template<typename _Tp>
	  static bool
	  _M_not_empty_function(const _Tp&)
	  { return true; }

      private:
	static void
	_M_init_functor(_Any_data& __functor, _Functor&& __f, true_type)
	{ ::new (__functor._M_access()) _Functor(std::move(__f)); }

	static void
	_M_init_functor(_Any_data& __functor, _Functor&& __f, false_type)
	{ __functor._M_access<_Functor*>() = new _Functor(std::move(__f)); }
      };

    _Function_base() : _M_manager(nullptr) { }

    ~_Function_base()
    {
      if (_M_manager)
	_M_manager(_M_functor, _M_functor, __destroy_functor);
    }

    bool _M_empty() const { return !_M_manager; }

    typedef bool (*_Manager_type)(_Any_data&, const _Any_data&,
				  _Manager_operation);

    _Any_data     _M_functor;
    _Manager_type _M_manager;
  };

  template<typename _Signature, typename _Functor>
    class _Function_handler;

  template<typename _Res, typename _Functor, typename... _ArgTypes>
    class _Function_handler<_Res(_ArgTypes...), _Functor>
    : public _Function_base::_Base_manager<_Functor>
    {
      typedef _Function_base::_Base_manager<_Functor> _Base;

    public:
      static bool
      _M_manager(_Any_data& __dest, const _Any_data& __source,
		 _Manager_operation __op)
      {
	switch (__op)
	  {
#if __cpp_rtti
	  case __get_type_info:
	    __dest._M_access<const type_info*>() = &typeid(_Functor);
	    break;
#endif
	  case __get_functor_ptr:
	    __dest._M_access<_Functor*>() = _Base::_M_get_pointer(__source);
	    break;

	  default:
	    _Base::_M_manager(__dest, __source, __op);
	  }
	return false;
      }

      static _Res
      _M_invoke(const _Any_data& __functor, _ArgTypes&&... __args)
      {
	return std::__invoke_r<_Res>(*_Base::_M_get_pointer(__functor),
				     std::forward<_ArgTypes>(__args)...);
      }
    };

  /**
   *  @brief Primary class template for std::function.
   *  @ingroup functors
   *
   *  Polymorphic function wrapper.
   */
  template<typename _Res, typename... _ArgTypes>
    class function<_Res(_ArgTypes...)>
    : public _Maybe_unary_or_binary_function<_Res, _ArgTypes...>,
      private _Function_base
    {
      template<typename _Func,
	       typename _Res2 = __invoke_result<_Func&, _ArgTypes...>>
	struct _Callable
	: __is_invocable_impl<_Res2, _Res>::type
	{ };

      // Used so the return type convertibility checks aren't done when
      // performing overload resolution for copy construction/assignment.
      template<typename _Tp>
	struct _Callable<function, _Tp> : false_type { };

      template<typename _Cond, typename _Tp>
	using _Requires = typename enable_if<_Cond::value, _Tp>::type;

    public:
      typedef _Res result_type;

      // [3.7.2.1] construct/copy/destroy

      /**
       *  @brief Default construct creates an empty function call wrapper.
       *  @post @c !(bool)*this
       */
      function() noexcept
      : _Function_base() { }

      /**
       *  @brief Creates an empty function call wrapper.
       *  @post @c !(bool)*this
       */
      function(nullptr_t) noexcept
      : _Function_base() { }

      /**
       *  @brief %Function copy constructor.
       *  @param __x A %function object with identical call signature.
       *  @post @c bool(*this) == bool(__x)
       *
       *  The newly-created %function contains a copy of the target of @a
       *  __x (if it has one).
       */
      function(const function& __x);

      /**
       *  @brief %Function move constructor.
       *  @param __x A %function object rvalue with identical call signature.
       *
       *  The newly-created %function contains the target of @a __x
       *  (if it has one).
       */
      function(function&& __x) noexcept : _Function_base()
      {
	__x.swap(*this);
      }

      /**
       *  @brief Builds a %function that targets a copy of the incoming
       *  function object.
       *  @param __f A %function object that is callable with parameters of
       *  type @c T1, @c T2, ..., @c TN and returns a value convertible
       *  to @c Res.
       *
       *  The newly-created %function object will target a copy of
       *  @a __f. If @a __f is @c reference_wrapper<F>, then this function
       *  object will contain a reference to the function object @c
       *  __f.get(). If @a __f is a NULL function pointer or NULL
       *  pointer-to-member, the newly-created object will be empty.
       *
       *  If @a __f is a non-NULL function pointer or an object of type @c
       *  reference_wrapper<F>, this function will not throw.
       */
      template<typename _Functor,
	       typename = _Requires<__not_<is_same<_Functor, function>>, void>,
	       typename = _Requires<_Callable<_Functor>, void>>
	function(_Functor);

      /**
       *  @brief %Function assignment operator.
       *  @param __x A %function with identical call signature.
       *  @post @c (bool)*this == (bool)x
       *  @returns @c *this
       *
       *  The target of @a __x is copied to @c *this. If @a __x has no
       *  target, then @c *this will be empty.
       *
       *  If @a __x targets a function pointer or a reference to a function
       *  object, then this operation will not throw an %exception.
       */
      function&
      operator=(const function& __x)
      {
	function(__x).swap(*this);
	return *this;
      }

      /**
       *  @brief %Function move-assignment operator.
       *  @param __x A %function rvalue with identical call signature.
       *  @returns @c *this
       *
       *  The target of @a __x is moved to @c *this. If @a __x has no
       *  target, then @c *this will be empty.
       *
       *  If @a __x targets a function pointer or a reference to a function
       *  object, then this operation will not throw an %exception.
       */
      function&
      operator=(function&& __x) noexcept
      {
	function(std::move(__x)).swap(*this);
	return *this;
      }

      /**
       *  @brief %Function assignment to zero.
       *  @post @c !(bool)*this
       *  @returns @c *this
       *
       *  The target of @c *this is deallocated, leaving it empty.
       */
      function&
      operator=(nullptr_t) noexcept
      {
	if (_M_manager)
	  {
	    _M_manager(_M_functor, _M_functor, __destroy_functor);
	    _M_manager = nullptr;
	    _M_invoker = nullptr;
	  }
	return *this;
      }

      /**
       *  @brief %Function assignment to a new target.
       *  @param __f A %function object that is callable with parameters of
       *  type @c T1, @c T2, ..., @c TN and returns a value convertible
       *  to @c Res.
       *  @return @c *this
       *
       *  This  %function object wrapper will target a copy of @a
       *  __f. If @a __f is @c reference_wrapper<F>, then this function
       *  object will contain a reference to the function object @c
       *  __f.get(). If @a __f is a NULL function pointer or NULL
       *  pointer-to-member, @c this object will be empty.
       *
       *  If @a __f is a non-NULL function pointer or an object of type @c
       *  reference_wrapper<F>, this function will not throw.
       */
      template<typename _Functor>
	_Requires<_Callable<typename decay<_Functor>::type>, function&>
	operator=(_Functor&& __f)
	{
	  function(std::forward<_Functor>(__f)).swap(*this);
	  return *this;
	}

      /// @overload
      template<typename _Functor>
	function&
	operator=(reference_wrapper<_Functor> __f) noexcept
	{
	  function(__f).swap(*this);
	  return *this;
	}

      // [3.7.2.2] function modifiers

      /**
       *  @brief Swap the targets of two %function objects.
       *  @param __x A %function with identical call signature.
       *
       *  Swap the targets of @c this function object and @a __f. This
       *  function will not throw an %exception.
       */
      void swap(function& __x) noexcept
      {
	std::swap(_M_functor, __x._M_functor);
	std::swap(_M_manager, __x._M_manager);
	std::swap(_M_invoker, __x._M_invoker);
      }

      // [3.7.2.3] function capacity

      /**
       *  @brief Determine if the %function wrapper has a target.
       *
       *  @return @c true when this %function object contains a target,
       *  or @c false when it is empty.
       *
       *  This function will not throw an %exception.
       */
      explicit operator bool() const noexcept
      { return !_M_empty(); }

      // [3.7.2.4] function invocation

      /**
       *  @brief Invokes the function targeted by @c *this.
       *  @returns the result of the target.
       *  @throws bad_function_call when @c !(bool)*this
       *
       *  The function call operator invokes the target function object
       *  stored by @c this.
       */
      _Res operator()(_ArgTypes... __args) const;

#if __cpp_rtti
      // [3.7.2.5] function target access
      /**
       *  @brief Determine the type of the target of this function object
       *  wrapper.
       *
       *  @returns the type identifier of the target function object, or
       *  @c typeid(void) if @c !(bool)*this.
       *
       *  This function will not throw an %exception.
       */
      const type_info& target_type() const noexcept;

      /**
       *  @brief Access the stored target function object.
       *
       *  @return Returns a pointer to the stored target function object,
       *  if @c typeid(_Functor).equals(target_type()); otherwise, a NULL
       *  pointer.
       *
       * This function does not throw exceptions.
       *
       * @{
       */
      template<typename _Functor>       _Functor* target() noexcept;

      template<typename _Functor> const _Functor* target() const noexcept;
      // @}
#endif

    private:
      using _Invoker_type = _Res (*)(const _Any_data&, _ArgTypes&&...);
      _Invoker_type _M_invoker;
  };

#if __cpp_deduction_guides >= 201606
  template<typename>
    struct __function_guide_helper
    { };

  template<typename _Res, typename _Tp, bool _Nx, typename... _Args>
    struct __function_guide_helper<
      _Res (_Tp::*) (_Args...) noexcept(_Nx)
    >
    { using type = _Res(_Args...); };

  template<typename _Res, typename _Tp, bool _Nx, typename... _Args>
    struct __function_guide_helper<
      _Res (_Tp::*) (_Args...) & noexcept(_Nx)
    >
    { using type = _Res(_Args...); };

  template<typename _Res, typename _Tp, bool _Nx, typename... _Args>
    struct __function_guide_helper<
      _Res (_Tp::*) (_Args...) const noexcept(_Nx)
    >
    { using type = _Res(_Args...); };

  template<typename _Res, typename _Tp, bool _Nx, typename... _Args>
    struct __function_guide_helper<
      _Res (_Tp::*) (_Args...) const & noexcept(_Nx)
    >
    { using type = _Res(_Args...); };

  template<typename _Res, typename... _ArgTypes>
    function(_Res(*)(_ArgTypes...)) -> function<_Res(_ArgTypes...)>;

  template<typename _Functor, typename _Signature = typename
	   __function_guide_helper<decltype(&_Functor::operator())>::type>
    function(_Functor) -> function<_Signature>;
#endif

  // Out-of-line member definitions.
  template<typename _Res, typename... _ArgTypes>
    function<_Res(_ArgTypes...)>::
    function(const function& __x)
    : _Function_base()
    {
      if (static_cast<bool>(__x))
	{
	  __x._M_manager(_M_functor, __x._M_functor, __clone_functor);
	  _M_invoker = __x._M_invoker;
	  _M_manager = __x._M_manager;
	}
    }

  template<typename _Res, typename... _ArgTypes>
    template<typename _Functor, typename, typename>
      function<_Res(_ArgTypes...)>::
      function(_Functor __f)
      : _Function_base()
      {
	typedef _Function_handler<_Res(_ArgTypes...), _Functor> _My_handler;

	if (_My_handler::_M_not_empty_function(__f))
	  {
	    _My_handler::_M_init_functor(_M_functor, std::move(__f));
	    _M_invoker = &_My_handler::_M_invoke;
	    _M_manager = &_My_handler::_M_manager;
	  }
      }

  template<typename _Res, typename... _ArgTypes>
    _Res
    function<_Res(_ArgTypes...)>::
    operator()(_ArgTypes... __args) const
    {
      if (_M_empty())
	__throw_bad_function_call();
      return _M_invoker(_M_functor, std::forward<_ArgTypes>(__args)...);
    }

#if __cpp_rtti
  template<typename _Res, typename... _ArgTypes>
    const type_info&
    function<_Res(_ArgTypes...)>::
    target_type() const noexcept
    {
      if (_M_manager)
	{
	  _Any_data __typeinfo_result;
	  _M_manager(__typeinfo_result, _M_functor, __get_type_info);
	  return *__typeinfo_result._M_access<const type_info*>();
	}
      else
	return typeid(void);
    }

  template<typename _Res, typename... _ArgTypes>
    template<typename _Functor>
      _Functor*
      function<_Res(_ArgTypes...)>::
      target() noexcept
      {
	const function* __const_this = this;
	const _Functor* __func = __const_this->template target<_Functor>();
	return const_cast<_Functor*>(__func);
      }

  template<typename _Res, typename... _ArgTypes>
    template<typename _Functor>
      const _Functor*
      function<_Res(_ArgTypes...)>::
      target() const noexcept
      {
	if (typeid(_Functor) == target_type() && _M_manager)
	  {
	    _Any_data __ptr;
	    _M_manager(__ptr, _M_functor, __get_functor_ptr);
	    return __ptr._M_access<const _Functor*>();
	  }
	else
	  return nullptr;
      }
#endif

  // [20.7.15.2.6] null pointer comparisons

  /**
   *  @brief Compares a polymorphic function object wrapper against 0
   *  (the NULL pointer).
   *  @returns @c true if the wrapper has no target, @c false otherwise
   *
   *  This function will not throw an %exception.
   */
  template<typename _Res, typename... _Args>
    inline bool
    operator==(const function<_Res(_Args...)>& __f, nullptr_t) noexcept
    { return !static_cast<bool>(__f); }

#if __cpp_impl_three_way_comparison < 201907L
  /// @overload
  template<typename _Res, typename... _Args>
    inline bool
    operator==(nullptr_t, const function<_Res(_Args...)>& __f) noexcept
    { return !static_cast<bool>(__f); }

  /**
   *  @brief Compares a polymorphic function object wrapper against 0
   *  (the NULL pointer).
   *  @returns @c false if the wrapper has no target, @c true otherwise
   *
   *  This function will not throw an %exception.
   */
  template<typename _Res, typename... _Args>
    inline bool
    operator!=(const function<_Res(_Args...)>& __f, nullptr_t) noexcept
    { return static_cast<bool>(__f); }

  /// @overload
  template<typename _Res, typename... _Args>
    inline bool
    operator!=(nullptr_t, const function<_Res(_Args...)>& __f) noexcept
    { return static_cast<bool>(__f); }
#endif

  // [20.7.15.2.7] specialized algorithms

  /**
   *  @brief Swap the targets of two polymorphic function object wrappers.
   *
   *  This function will not throw an %exception.
   */
  // _GLIBCXX_RESOLVE_LIB_DEFECTS
  // 2062. Effect contradictions w/o no-throw guarantee of std::function swaps
  template<typename _Res, typename... _Args>
    inline void
    swap(function<_Res(_Args...)>& __x, function<_Res(_Args...)>& __y) noexcept
    { __x.swap(__y); }

#if __cplusplus >= 201703L
  namespace __detail::__variant
  {
    template<typename> struct _Never_valueless_alt; // see <variant>

    // Provide the strong exception-safety guarantee when emplacing a
    // function into a variant.
    template<typename _Signature>
      struct _Never_valueless_alt<std::function<_Signature>>
      : std::true_type
      { };
  }  // namespace __detail::__variant
#endif // C++17

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif // C++11
#endif // _GLIBCXX_STD_FUNCTION_H
