// -*- C++ -*-

// Copyright (C) 2005-2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the terms
// of the GNU General Public License as published by the Free Software
// Foundation; either version 3, or (at your option) any later
// version.

// This library is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

// Copyright (C) 2004 Ami Tavory and Vladimir Dreizin, IBM-HRL.

// Permission to use, copy, modify, sell, and distribute this software
// is hereby granted without fee, provided that the above copyright
// notice appears in all copies, and that both that copyright notice
// and this permission notice appear in supporting documentation. None
// of the above authors, nor IBM Haifa Research Laboratories, make any
// representation about the suitability of this software for any
// purpose. It is provided "as is" without express or implied
// warranty.

/** @file ext/throw_allocator.h
 *  This file is a GNU extension to the Standard C++ Library.
 *
 *  Contains two exception-generating types (throw_value, throw_allocator)
 *  intended to be used as value and allocator types while testing
 *  exception safety in templatized containers and algorithms. The
 *  allocator has additional log and debug features. The exception
 *  generated is of type forced_exception_error.
 */

#ifndef _THROW_ALLOCATOR_H
#define _THROW_ALLOCATOR_H 1

#include <cmath>
#include <ctime>
#include <map>
#include <string>
#include <ostream>
#include <stdexcept>
#include <utility>
#include <bits/functexcept.h>
#include <bits/move.h>
#if __cplusplus >= 201103L
# include <functional>
# include <random>
#else
# include <tr1/functional>
# include <tr1/random>
#endif
#include <ext/alloc_traits.h>

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   *  @brief Thown by exception safety machinery.
   *  @ingroup exceptions
   */
  struct forced_error : public std::exception
  { };

  // Substitute for forced_error object when -fno-exceptions.
  inline void
  __throw_forced_error()
  { _GLIBCXX_THROW_OR_ABORT(forced_error()); }

  /**
   *  @brief Base class for checking address and label information
   *  about allocations. Create a std::map between the allocated
   *  address (void*) and a datum for annotations, which are a pair of
   *  numbers corresponding to label and allocated size.
   */
  struct annotate_base
  {
  private:
    typedef std::pair<size_t, size_t>		data_type;
    typedef std::map<void*, data_type>		map_alloc_type;
    typedef map_alloc_type::value_type		entry_type;
    typedef map_alloc_type::const_iterator	const_iterator;
    typedef map_alloc_type::const_reference	const_reference;
#if __cplusplus >= 201103L
    typedef std::map<void*, size_t>		map_construct_type;
#endif

  public:
    annotate_base()
    {
      label();
      map_alloc();
    }

    static void
    set_label(size_t l)
    { label() = l; }

    static size_t
    get_label()
    { return label(); }

    void
    insert(void* p, size_t size)
    {
      entry_type entry = make_entry(p, size);
      if (!p)
	{
	  std::string error("annotate_base::insert null insert!\n");
	  log_to_string(error, entry);
	  std::__throw_logic_error(error.c_str());
	}

      std::pair<map_alloc_type::iterator, bool> inserted
	= map_alloc().insert(entry);
      if (!inserted.second)
	{
	  std::string error("annotate_base::insert double insert!\n");
	  log_to_string(error, entry);
	  log_to_string(error, *inserted.first);
	  std::__throw_logic_error(error.c_str());
	}
    }

    void
    erase(void* p, size_t size)
    { map_alloc().erase(check_allocated(p, size)); }

#if __cplusplus >= 201103L
    void
    insert_construct(void* p)
    {
      if (!p)
	{
	  std::string error("annotate_base::insert_construct null!\n");
	  std::__throw_logic_error(error.c_str());
	}

      auto inserted = map_construct().insert(std::make_pair(p, get_label()));
      if (!inserted.second)
	{
	  std::string error("annotate_base::insert_construct double insert!\n");
	  log_to_string(error, std::make_pair(p, get_label()));
	  log_to_string(error, *inserted.first);
	  std::__throw_logic_error(error.c_str());
	}
    }

    void
    erase_construct(void* p)
    { map_construct().erase(check_constructed(p)); }
#endif

    // See if a particular address and allocation size has been saved.
    inline map_alloc_type::iterator
    check_allocated(void* p, size_t size)
    {
      map_alloc_type::iterator found = map_alloc().find(p);
      if (found == map_alloc().end())
	{
	  std::string error("annotate_base::check_allocated by value "
			    "null erase!\n");
	  log_to_string(error, make_entry(p, size));
	  std::__throw_logic_error(error.c_str());
	}

      if (found->second.second != size)
	{
	  std::string error("annotate_base::check_allocated by value "
			    "wrong-size erase!\n");
	  log_to_string(error, make_entry(p, size));
	  log_to_string(error, *found);
	  std::__throw_logic_error(error.c_str());
	}

      return found;
    }

    // See if a given label has been allocated.
    inline void
    check(size_t label)
    {
      std::string found;
      {
	const_iterator beg = map_alloc().begin();
	const_iterator end = map_alloc().end();
	while (beg != end)
	  {
	    if (beg->second.first == label)
	      log_to_string(found, *beg);
	    ++beg;
	  }
      }

#if __cplusplus >= 201103L
      {
	auto beg = map_construct().begin();
	auto end = map_construct().end();
	while (beg != end)
	  {
	    if (beg->second == label)
	      log_to_string(found, *beg);
	    ++beg;
	  }
      }
#endif

      if (!found.empty())
	{
	  std::string error("annotate_base::check by label\n");
	  error += found;
	  std::__throw_logic_error(error.c_str());
	}
    }

    // See if there is anything left allocated or constructed.
    inline static void
    check()
    {
      std::string found;
      {
	const_iterator beg = map_alloc().begin();
	const_iterator end = map_alloc().end();
	while (beg != end)
	  {
	    log_to_string(found, *beg);
	    ++beg;
	  }
      }

#if __cplusplus >= 201103L
      {
	auto beg = map_construct().begin();
	auto end = map_construct().end();
	while (beg != end)
	  {
	    log_to_string(found, *beg);
	    ++beg;
	  }
      }
#endif

      if (!found.empty())
	{
	  std::string error("annotate_base::check \n");
	  error += found;
	  std::__throw_logic_error(error.c_str());
	}
    }

#if __cplusplus >= 201103L
    inline map_construct_type::iterator
    check_constructed(void* p)
    {
      auto found = map_construct().find(p);
      if (found == map_construct().end())
	{
	  std::string error("annotate_base::check_constructed not "
			    "constructed!\n");
	  log_to_string(error, std::make_pair(p, get_label()));
	  std::__throw_logic_error(error.c_str());
	}

      return found;
    }

    inline void
    check_constructed(size_t label)
    {
      auto beg = map_construct().begin();
      auto end = map_construct().end();
      std::string found;
      while (beg != end)
	{
	  if (beg->second == label)
	    log_to_string(found, *beg);
	  ++beg;
	}

      if (!found.empty())
	{
	  std::string error("annotate_base::check_constructed by label\n");
	  error += found;
	  std::__throw_logic_error(error.c_str());
	}
    }
#endif

  private:
    friend std::ostream&
    operator<<(std::ostream&, const annotate_base&);

    entry_type
    make_entry(void* p, size_t size)
    { return std::make_pair(p, data_type(get_label(), size)); }

    static void
    log_to_string(std::string& s, const_reference ref)
    {
      char buf[40];
      const char tab('\t');
      s += "label: ";
      unsigned long l = static_cast<unsigned long>(ref.second.first);
      __builtin_sprintf(buf, "%lu", l);
      s += buf;
      s += tab;
      s += "size: ";
      l = static_cast<unsigned long>(ref.second.second);
      __builtin_sprintf(buf, "%lu", l);
      s += buf;
      s += tab;
      s += "address: ";
      __builtin_sprintf(buf, "%p", ref.first);
      s += buf;
      s += '\n';
    }

#if __cplusplus >= 201103L
    static void
    log_to_string(std::string& s, const std::pair<const void*, size_t>& ref)
    {
      char buf[40];
      const char tab('\t');
      s += "label: ";
      unsigned long l = static_cast<unsigned long>(ref.second);
      __builtin_sprintf(buf, "%lu", l);
      s += buf;
      s += tab;
      s += "address: ";
      __builtin_sprintf(buf, "%p", ref.first);
      s += buf;
      s += '\n';
    }
#endif

    static size_t&
    label()
    {
      static size_t _S_label(std::numeric_limits<size_t>::max());
      return _S_label;
    }

    static map_alloc_type&
    map_alloc()
    {
      static map_alloc_type _S_map;
      return _S_map;
    }

#if __cplusplus >= 201103L
    static map_construct_type&
    map_construct()
    {
      static map_construct_type _S_map;
      return _S_map;
    }
#endif
  };

  inline std::ostream&
  operator<<(std::ostream& os, const annotate_base& __b)
  {
    std::string error;
    typedef annotate_base base_type;
    {
      base_type::const_iterator beg = __b.map_alloc().begin();
      base_type::const_iterator end = __b.map_alloc().end();
      for (; beg != end; ++beg)
	__b.log_to_string(error, *beg);
    }
#if __cplusplus >= 201103L
    {
      auto beg = __b.map_construct().begin();
      auto end = __b.map_construct().end();
      for (; beg != end; ++beg)
	__b.log_to_string(error, *beg);      
    }
#endif
    return os << error;
  }


  /**
   *  @brief Base struct for condition policy.
   *
   * Requires a public member function with the signature
   * void throw_conditionally()
   */
  struct condition_base
  {
#if __cplusplus >= 201103L
    condition_base() = default;
    condition_base(const condition_base&) = default;
    condition_base& operator=(const condition_base&) = default;
#endif
    virtual ~condition_base() { };
  };


  /**
   *  @brief Base class for incremental control and throw.
   */
  struct limit_condition : public condition_base
  {
    // Scope-level adjustor objects: set limit for throw at the
    // beginning of a scope block, and restores to previous limit when
    // object is destroyed on exiting the block.
    struct adjustor_base
    {
    private:
      const size_t _M_orig;

    public:
      adjustor_base() : _M_orig(limit()) { }

      virtual
      ~adjustor_base() { set_limit(_M_orig); }
    };

    /// Never enter the condition.
    struct never_adjustor : public adjustor_base
    {
      never_adjustor() { set_limit(std::numeric_limits<size_t>::max()); }
    };

    /// Always enter the condition.
    struct always_adjustor : public adjustor_base
    {
      always_adjustor() { set_limit(count()); }
    };

    /// Enter the nth condition.
    struct limit_adjustor : public adjustor_base
    {
      limit_adjustor(const size_t __l) { set_limit(__l); }
    };

    // Increment _S_count every time called.
    // If _S_count matches the limit count, throw.
    static void
    throw_conditionally()
    {
      if (count() == limit())
	__throw_forced_error();
      ++count();
    }

    static size_t&
    count()
    {
      static size_t _S_count(0);
      return _S_count;
    }

    static size_t&
    limit()
    {
      static size_t _S_limit(std::numeric_limits<size_t>::max());
      return _S_limit;
    }

    // Zero the throw counter, set limit to argument.
    static void
    set_limit(const size_t __l)
    {
      limit() = __l;
      count() = 0;
    }
  };

#ifdef _GLIBCXX_USE_C99_STDINT_TR1
  /**
   *  @brief Base class for random probability control and throw.
   */
  struct random_condition : public condition_base
  {
    // Scope-level adjustor objects: set probability for throw at the
    // beginning of a scope block, and restores to previous
    // probability when object is destroyed on exiting the block.
    struct adjustor_base
    {
    private:
      const double _M_orig;

    public:
      adjustor_base() : _M_orig(probability()) { }

      virtual ~adjustor_base()
      { set_probability(_M_orig); }
    };

    /// Group condition.
    struct group_adjustor : public adjustor_base
    {
      group_adjustor(size_t size)
      { set_probability(1 - std::pow(double(1 - probability()),
				     double(0.5 / (size + 1))));
      }
    };

    /// Never enter the condition.
    struct never_adjustor : public adjustor_base
    {
      never_adjustor() { set_probability(0); }
    };

    /// Always enter the condition.
    struct always_adjustor : public adjustor_base
    {
      always_adjustor() { set_probability(1); }
    };

    random_condition()
    {
      probability();
      engine();
    }

    static void
    set_probability(double __p)
    { probability() = __p; }

    static void
    throw_conditionally()
    {
      if (generate() < probability())
	__throw_forced_error();
    }

    void
    seed(unsigned long __s)
    { engine().seed(__s); }

  private:
#if __cplusplus >= 201103L
    typedef std::uniform_real_distribution<double> 	distribution_type;
    typedef std::mt19937 				engine_type;
#else
    typedef std::tr1::uniform_real<double> 		distribution_type;
    typedef std::tr1::mt19937 				engine_type;
#endif

    static double
    generate()
    {
#if __cplusplus >= 201103L
      const distribution_type distribution(0, 1);
      static auto generator = std::bind(distribution, engine());
#else
      // Use variate_generator to get normalized results.
      typedef std::tr1::variate_generator<engine_type, distribution_type> gen_t;
      distribution_type distribution(0, 1);
      static gen_t generator(engine(), distribution);
#endif

      double random = generator();
      if (random < distribution.min() || random > distribution.max())
	{
	  std::string __s("random_condition::generate");
	  __s += "\n";
	  __s += "random number generated is: ";
	  char buf[40];
	  __builtin_sprintf(buf, "%f", random);
	  __s += buf;
	  std::__throw_out_of_range(__s.c_str());
	}

      return random;
    }

    static double&
    probability()
    {
      static double _S_p;
      return _S_p;
    }

    static engine_type&
    engine()
    {
      static engine_type _S_e;
      return _S_e;
    }
  };
#endif // _GLIBCXX_USE_C99_STDINT_TR1

  /**
   *  @brief Class with exception generation control. Intended to be
   *  used as a value_type in templatized code.
   *
   *  Note: Destructor not allowed to throw.
   */
  template<typename _Cond>
    struct throw_value_base : public _Cond
    {
      typedef _Cond  				condition_type;

      using condition_type::throw_conditionally;

      std::size_t			       	_M_i;

#ifndef _GLIBCXX_IS_AGGREGATE
      throw_value_base() : _M_i(0)
      { throw_conditionally(); }

      throw_value_base(const throw_value_base& __v) : _M_i(__v._M_i)
      { throw_conditionally(); }

#if __cplusplus >= 201103L
      // Shall not throw.
      throw_value_base(throw_value_base&&) = default;
#endif

      explicit throw_value_base(const std::size_t __i) : _M_i(__i)
      { throw_conditionally(); }
#endif

      throw_value_base&
      operator=(const throw_value_base& __v)
      {
	throw_conditionally();
	_M_i = __v._M_i;
	return *this;
      }

#if __cplusplus >= 201103L
      // Shall not throw.
      throw_value_base&
      operator=(throw_value_base&&) = default;
#endif

      throw_value_base&
      operator++()
      {
	throw_conditionally();
	++_M_i;
	return *this;
      }
    };

  template<typename _Cond>
    inline void
    swap(throw_value_base<_Cond>& __a, throw_value_base<_Cond>& __b)
    {
      typedef throw_value_base<_Cond> throw_value;
      throw_value::throw_conditionally();
      throw_value orig(__a);
      __a = __b;
      __b = orig;
    }

  // General instantiable types requirements.
  template<typename _Cond>
    inline bool
    operator==(const throw_value_base<_Cond>& __a,
	       const throw_value_base<_Cond>& __b)
    {
      typedef throw_value_base<_Cond> throw_value;
      throw_value::throw_conditionally();
      bool __ret = __a._M_i == __b._M_i;
      return __ret;
    }

  template<typename _Cond>
    inline bool
    operator<(const throw_value_base<_Cond>& __a,
	      const throw_value_base<_Cond>& __b)
    {
      typedef throw_value_base<_Cond> throw_value;
      throw_value::throw_conditionally();
      bool __ret = __a._M_i < __b._M_i;
      return __ret;
    }

  // Numeric algorithms instantiable types requirements.
  template<typename _Cond>
    inline throw_value_base<_Cond>
    operator+(const throw_value_base<_Cond>& __a,
	      const throw_value_base<_Cond>& __b)
    {
      typedef throw_value_base<_Cond> throw_value;
      throw_value::throw_conditionally();
      throw_value __ret(__a._M_i + __b._M_i);
      return __ret;
    }

  template<typename _Cond>
    inline throw_value_base<_Cond>
    operator-(const throw_value_base<_Cond>& __a,
	      const throw_value_base<_Cond>& __b)
    {
      typedef throw_value_base<_Cond> throw_value;
      throw_value::throw_conditionally();
      throw_value __ret(__a._M_i - __b._M_i);
      return __ret;
    }

  template<typename _Cond>
    inline throw_value_base<_Cond>
    operator*(const throw_value_base<_Cond>& __a,
	      const throw_value_base<_Cond>& __b)
    {
      typedef throw_value_base<_Cond> throw_value;
      throw_value::throw_conditionally();
      throw_value __ret(__a._M_i * __b._M_i);
      return __ret;
    }


  /// Type throwing via limit condition.
  struct throw_value_limit : public throw_value_base<limit_condition>
  {
    typedef throw_value_base<limit_condition> base_type;

#ifndef _GLIBCXX_IS_AGGREGATE
    throw_value_limit() { }

    throw_value_limit(const throw_value_limit& __other)
    : base_type(__other._M_i) { }

#if __cplusplus >= 201103L
    throw_value_limit(throw_value_limit&&) = default;
#endif

    explicit throw_value_limit(const std::size_t __i) : base_type(__i) { }
#endif

    throw_value_limit&
    operator=(const throw_value_limit& __other)
    {
      base_type::operator=(__other);
      return *this;
    }

#if __cplusplus >= 201103L
    throw_value_limit&
    operator=(throw_value_limit&&) = default;
#endif
  };

#ifdef _GLIBCXX_USE_C99_STDINT_TR1
  /// Type throwing via random condition.
  struct throw_value_random : public throw_value_base<random_condition>
  {
    typedef throw_value_base<random_condition> base_type;

#ifndef _GLIBCXX_IS_AGGREGATE
    throw_value_random() { }

    throw_value_random(const throw_value_random& __other)
    : base_type(__other._M_i) { }

#if __cplusplus >= 201103L
    throw_value_random(throw_value_random&&) = default;
#endif

    explicit throw_value_random(const std::size_t __i) : base_type(__i) { }
#endif

    throw_value_random&
    operator=(const throw_value_random& __other)
    {
      base_type::operator=(__other);
      return *this;
    }

#if __cplusplus >= 201103L
    throw_value_random&
    operator=(throw_value_random&&) = default;
#endif
  };
#endif // _GLIBCXX_USE_C99_STDINT_TR1

  /**
   *  @brief Allocator class with logging and exception generation control.
   * Intended to be used as an allocator_type in templatized code.
   *  @ingroup allocators
   *
   *  Note: Deallocate not allowed to throw.
   */
  template<typename _Tp, typename _Cond>
    class throw_allocator_base
    : public annotate_base, public _Cond
    {
    public:
      typedef std::size_t 			size_type;
      typedef std::ptrdiff_t 			difference_type;
      typedef _Tp 				value_type;
      typedef value_type* 			pointer;
      typedef const value_type* 		const_pointer;
      typedef value_type& 			reference;
      typedef const value_type& 		const_reference;

#if __cplusplus >= 201103L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2103. std::allocator propagate_on_container_move_assignment
      typedef std::true_type propagate_on_container_move_assignment;
#endif

    private:
      typedef _Cond				condition_type;

      std::allocator<value_type> 		_M_allocator;

      typedef __gnu_cxx::__alloc_traits<std::allocator<value_type> > traits;

      using condition_type::throw_conditionally;

    public:
      size_type
      max_size() const _GLIBCXX_USE_NOEXCEPT
      { return traits::max_size(_M_allocator); }

      pointer
      address(reference __x) const _GLIBCXX_NOEXCEPT
      { return std::__addressof(__x); }

      const_pointer
      address(const_reference __x) const _GLIBCXX_NOEXCEPT
      { return std::__addressof(__x); }

      _GLIBCXX_NODISCARD pointer
      allocate(size_type __n, const void* hint = 0)
      {
	if (__n > this->max_size())
	  std::__throw_bad_alloc();

	throw_conditionally();
	pointer const a = traits::allocate(_M_allocator, __n, hint);
	insert(a, sizeof(value_type) * __n);
	return a;
      }

#if __cplusplus >= 201103L
      template<typename _Up, typename... _Args>
        void
        construct(_Up* __p, _Args&&... __args)
	{
	  traits::construct(_M_allocator, __p, std::forward<_Args>(__args)...);
	  insert_construct(__p);
	}

      template<typename _Up>
        void 
        destroy(_Up* __p)
        {
	  erase_construct(__p);
	  traits::destroy(_M_allocator, __p);
	}
#else
      void
      construct(pointer __p, const value_type& val)
      { return _M_allocator.construct(__p, val); }

      void
      destroy(pointer __p)
      { _M_allocator.destroy(__p); }
#endif

      void
      deallocate(pointer __p, size_type __n)
      {
	erase(__p, sizeof(value_type) * __n);
	_M_allocator.deallocate(__p, __n);
      }

      void
      check_allocated(pointer __p, size_type __n)
      {
	size_type __t = sizeof(value_type) * __n;
	annotate_base::check_allocated(__p, __t);
      }

      void
      check(size_type __n)
      { annotate_base::check(__n); }
  };

  template<typename _Tp, typename _Cond>
    inline bool
    operator==(const throw_allocator_base<_Tp, _Cond>&,
	       const throw_allocator_base<_Tp, _Cond>&)
    { return true; }

#if __cpp_impl_three_way_comparison < 201907L
  template<typename _Tp, typename _Cond>
    inline bool
    operator!=(const throw_allocator_base<_Tp, _Cond>&,
	       const throw_allocator_base<_Tp, _Cond>&)
    { return false; }
#endif

  /// Allocator throwing via limit condition.
  template<typename _Tp>
    struct throw_allocator_limit
    : public throw_allocator_base<_Tp, limit_condition>
    {
      template<typename _Tp1>
	struct rebind
	{ typedef throw_allocator_limit<_Tp1> other; };

      throw_allocator_limit() _GLIBCXX_USE_NOEXCEPT { }

      throw_allocator_limit(const throw_allocator_limit&)
      _GLIBCXX_USE_NOEXCEPT { }

      template<typename _Tp1>
	throw_allocator_limit(const throw_allocator_limit<_Tp1>&)
	_GLIBCXX_USE_NOEXCEPT { }

      ~throw_allocator_limit() _GLIBCXX_USE_NOEXCEPT { }
    };

#ifdef _GLIBCXX_USE_C99_STDINT_TR1
  /// Allocator throwing via random condition.
  template<typename _Tp>
    struct throw_allocator_random
    : public throw_allocator_base<_Tp, random_condition>
    {
      template<typename _Tp1>
	struct rebind
	{ typedef throw_allocator_random<_Tp1> other; };

      throw_allocator_random() _GLIBCXX_USE_NOEXCEPT { }

      throw_allocator_random(const throw_allocator_random&)
      _GLIBCXX_USE_NOEXCEPT { }

      template<typename _Tp1>
	throw_allocator_random(const throw_allocator_random<_Tp1>&)
	_GLIBCXX_USE_NOEXCEPT { }

      ~throw_allocator_random() _GLIBCXX_USE_NOEXCEPT { }
    };
#endif // _GLIBCXX_USE_C99_STDINT_TR1

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#if __cplusplus >= 201103L

# include <bits/functional_hash.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
  /// Explicit specialization of std::hash for __gnu_cxx::throw_value_limit.
  template<>
    struct hash<__gnu_cxx::throw_value_limit>
    : public std::unary_function<__gnu_cxx::throw_value_limit, size_t>
    {
      size_t
      operator()(const __gnu_cxx::throw_value_limit& __val) const
      {
	__gnu_cxx::throw_value_limit::throw_conditionally();
	std::hash<std::size_t> __h;
	size_t __result = __h(__val._M_i);
	return __result;
      }
    };

#ifdef _GLIBCXX_USE_C99_STDINT_TR1
  /// Explicit specialization of std::hash for __gnu_cxx::throw_value_random.
  template<>
    struct hash<__gnu_cxx::throw_value_random>
    : public std::unary_function<__gnu_cxx::throw_value_random, size_t>
    {
      size_t
      operator()(const __gnu_cxx::throw_value_random& __val) const
      {
	__gnu_cxx::throw_value_random::throw_conditionally();
	std::hash<std::size_t> __h;
	size_t __result = __h(__val._M_i);
	return __result;
      }
    };
#endif
} // end namespace std
#endif

#endif
