// -*- C++ -*-

// Copyright (C) 2007-2020 Free Software Foundation, Inc.
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

/** @file parallel/losertree.h
*  @brief Many generic loser tree variants.
*  This file is a GNU parallel extension to the Standard C++ Library.
*/

// Written by Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_LOSERTREE_H
#define _GLIBCXX_PARALLEL_LOSERTREE_H 1

#include <bits/stl_algobase.h>
#include <bits/stl_function.h>
#include <parallel/features.h>
#include <parallel/base.h>

namespace __gnu_parallel
{
  /**
   * @brief Guarded loser/tournament tree.
   *
   * The smallest element is at the top.
   *
   * Guarding is done explicitly through one flag _M_sup per element,
   * inf is not needed due to a better initialization routine.  This
   * is a well-performing variant.
   *
   * @param _Tp the element type
   * @param _Compare the comparator to use, defaults to std::less<_Tp>
   */
  template<typename _Tp, typename _Compare>
    class _LoserTreeBase
    {
    protected:
      /** @brief Internal representation of a _LoserTree element. */
      struct _Loser
      {
	/** @brief flag, true iff this is a "maximum" __sentinel. */
	bool _M_sup;
	/** @brief __index of the __source __sequence. */
	int _M_source;
	/** @brief _M_key of the element in the _LoserTree. */
	_Tp _M_key;
      };

      unsigned int _M_ik, _M_k, _M_offset;

      /** log_2{_M_k} */
      unsigned int _M_log_k;

      /** @brief _LoserTree __elements. */
      _Loser* _M_losers;

      /** @brief _Compare to use. */
      _Compare _M_comp;

      /**
       * @brief State flag that determines whether the _LoserTree is empty.
       *
       * Only used for building the _LoserTree.
       */
      bool _M_first_insert;

    public:
      /**
       * @brief The constructor.
       *
       * @param __k The number of sequences to merge.
       * @param __comp The comparator to use.
       */
      _LoserTreeBase(unsigned int __k, _Compare __comp)
      : _M_comp(__comp)
      {
	_M_ik = __k;

	// Compute log_2{_M_k} for the _Loser Tree
	_M_log_k = __rd_log2(_M_ik - 1) + 1;

	// Next greater power of 2.
	_M_k = 1 << _M_log_k;
	_M_offset = _M_k;

	// Avoid default-constructing _M_losers[]._M_key
	_M_losers = static_cast<_Loser*>(::operator new(2 * _M_k
							* sizeof(_Loser)));
	for (unsigned int __i = _M_ik - 1; __i < _M_k; ++__i)
	  _M_losers[__i + _M_k]._M_sup = true;

	_M_first_insert = true;
      }

      /**
       * @brief The destructor.
       */
      ~_LoserTreeBase()
      {
	for (unsigned int __i = 0; __i < (2 * _M_k); ++__i)
	  _M_losers[__i].~_Loser();
	::operator delete(_M_losers);
      }

      /**
       * @brief Initializes the sequence "_M_source" with the element "__key".
       *
       * @param __key the element to insert
       * @param __source __index of the __source __sequence
       * @param __sup flag that determines whether the value to insert is an
       *   explicit __supremum.
       */
      void
      __insert_start(const _Tp& __key, int __source, bool __sup)
      {
	unsigned int __pos = _M_k + __source;

	if (_M_first_insert)
	  {
	    // Construct all keys, so we can easily destruct them.
	    for (unsigned int __i = 0; __i < (2 * _M_k); ++__i)
	      ::new(&(_M_losers[__i]._M_key)) _Tp(__key);
	    _M_first_insert = false;
	  }
	else
	  _M_losers[__pos]._M_key = __key;

	_M_losers[__pos]._M_sup = __sup;
	_M_losers[__pos]._M_source = __source;
      }

      /**
       * @return the index of the sequence with the smallest element.
       */
      int __get_min_source()
      { return _M_losers[0]._M_source; }
    };

    /**
     * @brief Stable _LoserTree variant.
     *
     * Provides the stable implementations of insert_start, __init_winner,
     * __init and __delete_min_insert.
     *
     * Unstable variant is done using partial specialisation below.
     */
  template<bool __stable/* default == true */, typename _Tp,
	   typename _Compare>
    class _LoserTree
    : public _LoserTreeBase<_Tp, _Compare>
    {
      typedef _LoserTreeBase<_Tp, _Compare> _Base;
      using _Base::_M_k;
      using _Base::_M_comp;
      using _Base::_M_losers;
      using _Base::_M_first_insert;

    public:
      _LoserTree(unsigned int __k, _Compare __comp)
      : _Base::_LoserTreeBase(__k, __comp)
      { }

      unsigned int
      __init_winner(unsigned int __root)
      {
	if (__root >= _M_k)
	  return __root;
	else
	  {
	    unsigned int __left = __init_winner(2 * __root);
	    unsigned int __right = __init_winner(2 * __root + 1);
	    if (_M_losers[__right]._M_sup
		|| (!_M_losers[__left]._M_sup
		    && !_M_comp(_M_losers[__right]._M_key,
				_M_losers[__left]._M_key)))
	      {
		// Left one is less or equal.
		_M_losers[__root] = _M_losers[__right];
		return __left;
	      }
	    else
	      {
		// Right one is less.
		_M_losers[__root] = _M_losers[__left];
		return __right;
	      }
	  }
      }

      void __init()
      { _M_losers[0] = _M_losers[__init_winner(1)]; }

      /**
       * @brief Delete the smallest element and insert a new element from
       *   the previously smallest element's sequence.
       *
       * This implementation is stable.
       */
      // Do not pass a const reference since __key will be used as
      // local variable.
      void
      __delete_min_insert(_Tp __key, bool __sup)
      {
        using std::swap;
#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top!
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif

	int __source = _M_losers[0]._M_source;
	for (unsigned int __pos = (_M_k + __source) / 2; __pos > 0;
	     __pos /= 2)
	  {
	    // The smaller one gets promoted, ties are broken by _M_source.
	    if ((__sup && (!_M_losers[__pos]._M_sup
			   || _M_losers[__pos]._M_source < __source))
		|| (!__sup && !_M_losers[__pos]._M_sup
		    && ((_M_comp(_M_losers[__pos]._M_key, __key))
			|| (!_M_comp(__key, _M_losers[__pos]._M_key)
			    && _M_losers[__pos]._M_source < __source))))
	      {
		// The other one is smaller.
		std::swap(_M_losers[__pos]._M_sup, __sup);
		std::swap(_M_losers[__pos]._M_source, __source);
		swap(_M_losers[__pos]._M_key, __key);
	      }
	  }

	_M_losers[0]._M_sup = __sup;
	_M_losers[0]._M_source = __source;
	_M_losers[0]._M_key = __key;
      }
    };

    /**
     * @brief Unstable _LoserTree variant.
     *
     * Stability (non-stable here) is selected with partial specialization.
     */
  template<typename _Tp, typename _Compare>
    class _LoserTree</* __stable == */false, _Tp, _Compare>
    : public _LoserTreeBase<_Tp, _Compare>
    {
      typedef _LoserTreeBase<_Tp, _Compare> _Base;
      using _Base::_M_log_k;
      using _Base::_M_k;
      using _Base::_M_comp;
      using _Base::_M_losers;
      using _Base::_M_first_insert;

    public:
      _LoserTree(unsigned int __k, _Compare __comp)
      : _Base::_LoserTreeBase(__k, __comp)
      { }

      /**
       * Computes the winner of the competition at position "__root".
       *
       * Called recursively (starting at 0) to build the initial tree.
       *
       * @param __root __index of the "game" to start.
       */
      unsigned int
      __init_winner(unsigned int __root)
      {
	if (__root >= _M_k)
	  return __root;
	else
	  {
	    unsigned int __left = __init_winner(2 * __root);
	    unsigned int __right = __init_winner(2 * __root + 1);
	    if (_M_losers[__right]._M_sup
		|| (!_M_losers[__left]._M_sup
		    && !_M_comp(_M_losers[__right]._M_key,
				_M_losers[__left]._M_key)))
	      {
		// Left one is less or equal.
		_M_losers[__root] = _M_losers[__right];
		return __left;
	      }
	    else
	      {
		// Right one is less.
		_M_losers[__root] = _M_losers[__left];
		return __right;
	      }
	  }
      }

      void
      __init()
      { _M_losers[0] = _M_losers[__init_winner(1)]; }

      /**
       * Delete the _M_key smallest element and insert the element __key
       * instead.
       *
       * @param __key the _M_key to insert
       * @param __sup true iff __key is an explicitly marked supremum
       */
      // Do not pass a const reference since __key will be used as local
      // variable.
      void
      __delete_min_insert(_Tp __key, bool __sup)
      {
        using std::swap;
#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top!
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif

	int __source = _M_losers[0]._M_source;
	for (unsigned int __pos = (_M_k + __source) / 2; __pos > 0;
	     __pos /= 2)
	  {
	    // The smaller one gets promoted.
	    if (__sup || (!_M_losers[__pos]._M_sup
			  && _M_comp(_M_losers[__pos]._M_key, __key)))
	      {
		// The other one is smaller.
		std::swap(_M_losers[__pos]._M_sup, __sup);
		std::swap(_M_losers[__pos]._M_source, __source);
		swap(_M_losers[__pos]._M_key, __key);
	      }
	  }

	_M_losers[0]._M_sup = __sup;
	_M_losers[0]._M_source = __source;
	_M_losers[0]._M_key = __key;
      }
    };

  /**
   * @brief Base class of _Loser Tree implementation using pointers.
   */
  template<typename _Tp, typename _Compare>
    class _LoserTreePointerBase
    {
    protected:
      /** @brief Internal representation of _LoserTree __elements. */
      struct _Loser
      {
	bool _M_sup;
	int _M_source;
	const _Tp* _M_keyp;
      };

      unsigned int _M_ik, _M_k, _M_offset;
      _Loser* _M_losers;
      _Compare _M_comp;

    public:
      _LoserTreePointerBase(unsigned int __k,
			    _Compare __comp = std::less<_Tp>())
      : _M_comp(__comp)
      {
	_M_ik = __k;

	// Next greater power of 2.
	_M_k = 1 << (__rd_log2(_M_ik - 1) + 1);
	_M_offset = _M_k;
	_M_losers = new _Loser[_M_k * 2];
	for (unsigned int __i = _M_ik - 1; __i < _M_k; __i++)
	  _M_losers[__i + _M_k]._M_sup = true;
      }

      ~_LoserTreePointerBase()
      { delete[] _M_losers; }

      int __get_min_source()
      { return _M_losers[0]._M_source; }

      void __insert_start(const _Tp& __key, int __source, bool __sup)
      {
	unsigned int __pos = _M_k + __source;

	_M_losers[__pos]._M_sup = __sup;
	_M_losers[__pos]._M_source = __source;
	_M_losers[__pos]._M_keyp = &__key;
      }
    };

  /**
   * @brief Stable _LoserTree implementation.
   *
   * The unstable variant is implemented using partial instantiation below.
   */
  template<bool __stable/* default == true */, typename _Tp, typename _Compare>
    class _LoserTreePointer
    : public _LoserTreePointerBase<_Tp, _Compare>
    {
      typedef _LoserTreePointerBase<_Tp, _Compare> _Base;
      using _Base::_M_k;
      using _Base::_M_comp;
      using _Base::_M_losers;

    public:
      _LoserTreePointer(unsigned int __k, _Compare __comp = std::less<_Tp>())
      : _Base::_LoserTreePointerBase(__k, __comp)
      { }

      unsigned int
      __init_winner(unsigned int __root)
      {
	if (__root >= _M_k)
	  return __root;
	else
	  {
	    unsigned int __left = __init_winner(2 * __root);
	    unsigned int __right = __init_winner(2 * __root + 1);
	    if (_M_losers[__right]._M_sup
		|| (!_M_losers[__left]._M_sup
		    && !_M_comp(*_M_losers[__right]._M_keyp,
				*_M_losers[__left]._M_keyp)))
	      {
		// Left one is less or equal.
		_M_losers[__root] = _M_losers[__right];
		return __left;
	      }
	    else
	      {
		// Right one is less.
		_M_losers[__root] = _M_losers[__left];
		return __right;
	      }
	  }
      }

      void __init()
      { _M_losers[0] = _M_losers[__init_winner(1)]; }

      void __delete_min_insert(const _Tp& __key, bool __sup)
      {
#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top!
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif

	const _Tp* __keyp = &__key;
	int __source = _M_losers[0]._M_source;
	for (unsigned int __pos = (_M_k + __source) / 2; __pos > 0;
	     __pos /= 2)
	  {
	    // The smaller one gets promoted, ties are broken by __source.
	    if ((__sup && (!_M_losers[__pos]._M_sup
			   || _M_losers[__pos]._M_source < __source))
		|| (!__sup && !_M_losers[__pos]._M_sup &&
		    ((_M_comp(*_M_losers[__pos]._M_keyp, *__keyp))
		     || (!_M_comp(*__keyp, *_M_losers[__pos]._M_keyp)
			 && _M_losers[__pos]._M_source < __source))))
	      {
		// The other one is smaller.
		std::swap(_M_losers[__pos]._M_sup, __sup);
		std::swap(_M_losers[__pos]._M_source, __source);
		std::swap(_M_losers[__pos]._M_keyp, __keyp);
	      }
	  }

	_M_losers[0]._M_sup = __sup;
	_M_losers[0]._M_source = __source;
	_M_losers[0]._M_keyp = __keyp;
      }
    };

  /**
   * @brief Unstable _LoserTree implementation.
   *
   * The stable variant is above.
   */
  template<typename _Tp, typename _Compare>
    class _LoserTreePointer</* __stable == */false, _Tp, _Compare>
    : public _LoserTreePointerBase<_Tp, _Compare>
    {
      typedef _LoserTreePointerBase<_Tp, _Compare> _Base;
      using _Base::_M_k;
      using _Base::_M_comp;
      using _Base::_M_losers;

    public:
      _LoserTreePointer(unsigned int __k, _Compare __comp = std::less<_Tp>())
      : _Base::_LoserTreePointerBase(__k, __comp)
      { }

      unsigned int
      __init_winner(unsigned int __root)
      {
	if (__root >= _M_k)
	  return __root;
	else
	  {
	    unsigned int __left = __init_winner(2 * __root);
	    unsigned int __right = __init_winner(2 * __root + 1);
	    if (_M_losers[__right]._M_sup
        	|| (!_M_losers[__left]._M_sup
		    && !_M_comp(*_M_losers[__right]._M_keyp,
				*_M_losers[__left]._M_keyp)))
	      {
		// Left one is less or equal.
		_M_losers[__root] = _M_losers[__right];
		return __left;
	      }
	    else
	      {
		// Right one is less.
		_M_losers[__root] = _M_losers[__left];
		return __right;
	      }
	  }
      }

      void __init()
      { _M_losers[0] = _M_losers[__init_winner(1)]; }

      void __delete_min_insert(const _Tp& __key, bool __sup)
      {
#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top!
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif

	const _Tp* __keyp = &__key;
	int __source = _M_losers[0]._M_source;
	for (unsigned int __pos = (_M_k + __source) / 2; __pos > 0;
	     __pos /= 2)
	  {
	    // The smaller one gets promoted.
	    if (__sup || (!_M_losers[__pos]._M_sup
			  && _M_comp(*_M_losers[__pos]._M_keyp, *__keyp)))
	      {
		// The other one is smaller.
		std::swap(_M_losers[__pos]._M_sup, __sup);
		std::swap(_M_losers[__pos]._M_source, __source);
		std::swap(_M_losers[__pos]._M_keyp, __keyp);
	      }
	  }

	_M_losers[0]._M_sup = __sup;
	_M_losers[0]._M_source = __source;
	_M_losers[0]._M_keyp = __keyp;
      }
    };

  /** @brief Base class for unguarded _LoserTree implementation.
   * 
   * The whole element is copied into the tree structure.
   *
   * No guarding is done, therefore not a single input sequence must
   * run empty.  Unused __sequence heads are marked with a sentinel which
   * is &gt; all elements that are to be merged.
   *
   * This is a very fast variant.
   */
  template<typename _Tp, typename _Compare>
    class _LoserTreeUnguardedBase
    {
    protected:
      struct _Loser
      {
	int _M_source;
	_Tp _M_key;
      };

      unsigned int _M_ik, _M_k, _M_offset;
      _Loser* _M_losers;
      _Compare _M_comp;

    public:
      _LoserTreeUnguardedBase(unsigned int __k, const _Tp& __sentinel,
			      _Compare __comp = std::less<_Tp>())
      : _M_comp(__comp)
      {
	_M_ik = __k;

	// Next greater power of 2.
	_M_k = 1 << (__rd_log2(_M_ik - 1) + 1);
	_M_offset = _M_k;
	// Avoid default-constructing _M_losers[]._M_key
	_M_losers = static_cast<_Loser*>(::operator new(2 * _M_k
							* sizeof(_Loser)));

        for (unsigned int __i = 0; __i < _M_k; ++__i)
          {
	    ::new(&(_M_losers[__i]._M_key)) _Tp(__sentinel);
	    _M_losers[__i]._M_source = -1;
	  }
        for (unsigned int __i = _M_k + _M_ik - 1; __i < (2 * _M_k); ++__i)
          {
	    ::new(&(_M_losers[__i]._M_key)) _Tp(__sentinel);
	    _M_losers[__i]._M_source = -1;
	  }
      }

      ~_LoserTreeUnguardedBase()
      {
	for (unsigned int __i = 0; __i < (2 * _M_k); ++__i)
	  _M_losers[__i].~_Loser();
	::operator delete(_M_losers);
      }

      int
      __get_min_source()
      {
#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top!
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif
	return _M_losers[0]._M_source;
      }

      void
      __insert_start(const _Tp& __key, int __source, bool)
      {
	unsigned int __pos = _M_k + __source;

	::new(&(_M_losers[__pos]._M_key)) _Tp(__key);
	_M_losers[__pos]._M_source = __source;
      }
    };

  /**
   * @brief Stable implementation of unguarded _LoserTree.
   *
   * Unstable variant is selected below with partial specialization.
   */
  template<bool __stable/* default == true */, typename _Tp, typename _Compare>
    class _LoserTreeUnguarded
    : public _LoserTreeUnguardedBase<_Tp, _Compare>
    {
      typedef _LoserTreeUnguardedBase<_Tp, _Compare> _Base;
      using _Base::_M_k;
      using _Base::_M_comp;
      using _Base::_M_losers;

  public:
      _LoserTreeUnguarded(unsigned int __k, const _Tp& __sentinel,
			  _Compare __comp = std::less<_Tp>())
      : _Base::_LoserTreeUnguardedBase(__k, __sentinel, __comp)
      { }

      unsigned int
      __init_winner(unsigned int __root)
      {
	if (__root >= _M_k)
	  return __root;
	else
	  {
	    unsigned int __left = __init_winner(2 * __root);
	    unsigned int __right = __init_winner(2 * __root + 1);
	    if (!_M_comp(_M_losers[__right]._M_key,
			 _M_losers[__left]._M_key))
	      {
		// Left one is less or equal.
		_M_losers[__root] = _M_losers[__right];
		return __left;
	      }
	    else
	      {
		// Right one is less.
		_M_losers[__root] = _M_losers[__left];
		return __right;
	      }
	  }
      }

      void
      __init()
      {
	_M_losers[0] = _M_losers[__init_winner(1)];

#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top at the beginning
	// (0 sequences!)
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif
      }

      // Do not pass a const reference since __key will be used as
      // local variable.
      void
      __delete_min_insert(_Tp __key, bool)
      {
        using std::swap;
#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top!
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif

	int __source = _M_losers[0]._M_source;
	for (unsigned int __pos = (_M_k + __source) / 2; __pos > 0;
	     __pos /= 2)
	  {
	    // The smaller one gets promoted, ties are broken by _M_source.
	    if (_M_comp(_M_losers[__pos]._M_key, __key)
        	|| (!_M_comp(__key, _M_losers[__pos]._M_key)
                    && _M_losers[__pos]._M_source < __source))
	      {
		// The other one is smaller.
		std::swap(_M_losers[__pos]._M_source, __source);
		swap(_M_losers[__pos]._M_key, __key);
	      }
	  }

	_M_losers[0]._M_source = __source;
	_M_losers[0]._M_key = __key;
      }
    };

  /**
   * @brief Non-Stable implementation of unguarded _LoserTree.
   *
   * Stable implementation is above.
   */
  template<typename _Tp, typename _Compare>
    class _LoserTreeUnguarded</* __stable == */false, _Tp, _Compare>
    : public _LoserTreeUnguardedBase<_Tp, _Compare>
    {
      typedef _LoserTreeUnguardedBase<_Tp, _Compare> _Base;
      using _Base::_M_k;
      using _Base::_M_comp;
      using _Base::_M_losers;

    public:
      _LoserTreeUnguarded(unsigned int __k, const _Tp& __sentinel,
			  _Compare __comp = std::less<_Tp>())
      : _Base::_LoserTreeUnguardedBase(__k, __sentinel, __comp)
      { }

      unsigned int
      __init_winner(unsigned int __root)
      {
	if (__root >= _M_k)
	  return __root;
	else
	  {
	    unsigned int __left = __init_winner(2 * __root);
	    unsigned int __right = __init_winner(2 * __root + 1);

#if _GLIBCXX_PARALLEL_ASSERTIONS
	    // If __left one is sentinel then __right one must be, too.
	    if (_M_losers[__left]._M_source == -1)
	      _GLIBCXX_PARALLEL_ASSERT(_M_losers[__right]._M_source == -1);
#endif

	    if (!_M_comp(_M_losers[__right]._M_key,
			 _M_losers[__left]._M_key))
	      {
		// Left one is less or equal.
		_M_losers[__root] = _M_losers[__right];
		return __left;
	      }
	    else
	      {
		// Right one is less.
		_M_losers[__root] = _M_losers[__left];
		return __right;
	      }
	  }
      }

      void
      __init()
      {
	_M_losers[0] = _M_losers[__init_winner(1)];

#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top at the beginning
	// (0 sequences!)
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif
      }

      // Do not pass a const reference since __key will be used as
      // local variable.
      void
      __delete_min_insert(_Tp __key, bool)
      {
        using std::swap;
#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top!
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif

	int __source = _M_losers[0]._M_source;
	for (unsigned int __pos = (_M_k + __source) / 2; __pos > 0;
	     __pos /= 2)
	  {
	    // The smaller one gets promoted.
	    if (_M_comp(_M_losers[__pos]._M_key, __key))
	      {
		// The other one is smaller.
		std::swap(_M_losers[__pos]._M_source, __source);
		swap(_M_losers[__pos]._M_key, __key);
	      }
	  }

	_M_losers[0]._M_source = __source;
	_M_losers[0]._M_key = __key;
      }
    };

  /** @brief Unguarded loser tree, keeping only pointers to the
  * elements in the tree structure.
  *
  *  No guarding is done, therefore not a single input sequence must
  *  run empty.  This is a very fast variant.
  */
  template<typename _Tp, typename _Compare>
    class _LoserTreePointerUnguardedBase
    {
    protected:
      struct _Loser
      {
	int _M_source;
	const _Tp* _M_keyp;
      };

      unsigned int _M_ik, _M_k, _M_offset;
      _Loser* _M_losers;
      _Compare _M_comp;

    public:

      _LoserTreePointerUnguardedBase(unsigned int __k, const _Tp& __sentinel,
				     _Compare __comp = std::less<_Tp>())
      : _M_comp(__comp)
      {
	_M_ik = __k;

	// Next greater power of 2.
	_M_k = 1 << (__rd_log2(_M_ik - 1) + 1);
	_M_offset = _M_k;
	// Avoid default-constructing _M_losers[]._M_key
	_M_losers = new _Loser[2 * _M_k];

	for (unsigned int __i = _M_k + _M_ik - 1; __i < (2 * _M_k); ++__i)
	  {
	    _M_losers[__i]._M_keyp = &__sentinel;
	    _M_losers[__i]._M_source = -1;
	  }
      }

      ~_LoserTreePointerUnguardedBase()
      { delete[] _M_losers; }

      int
      __get_min_source()
      {
#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top!
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif
	return _M_losers[0]._M_source;
      }

      void
      __insert_start(const _Tp& __key, int __source, bool)
      {
	unsigned int __pos = _M_k + __source;

	_M_losers[__pos]._M_keyp = &__key;
	_M_losers[__pos]._M_source = __source;
      }
    };

  /**
   * @brief Stable unguarded _LoserTree variant storing pointers.
   *
   * Unstable variant is implemented below using partial specialization.
   */
  template<bool __stable/* default == true */, typename _Tp, typename _Compare>
    class _LoserTreePointerUnguarded
    : public _LoserTreePointerUnguardedBase<_Tp, _Compare>
    {
      typedef _LoserTreePointerUnguardedBase<_Tp, _Compare> _Base;
      using _Base::_M_k;
      using _Base::_M_comp;
      using _Base::_M_losers;

    public:
      _LoserTreePointerUnguarded(unsigned int __k, const _Tp& __sentinel,
				 _Compare __comp = std::less<_Tp>())
      : _Base::_LoserTreePointerUnguardedBase(__k, __sentinel, __comp)
      { }

      unsigned int
      __init_winner(unsigned int __root)
      {
	if (__root >= _M_k)
	  return __root;
	else
	  {
	    unsigned int __left = __init_winner(2 * __root);
	    unsigned int __right = __init_winner(2 * __root + 1);
	    if (!_M_comp(*_M_losers[__right]._M_keyp,
			 *_M_losers[__left]._M_keyp))
	      {
		// Left one is less or equal.
		_M_losers[__root] = _M_losers[__right];
		return __left;
	      }
	    else
	      {
		// Right one is less.
		_M_losers[__root] = _M_losers[__left];
		return __right;
	      }
	  }
      }

      void
      __init()
      {
	_M_losers[0] = _M_losers[__init_winner(1)];

#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top at the beginning
	// (0 sequences!)
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif
      }

      void
      __delete_min_insert(const _Tp& __key, bool __sup)
      {
#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top!
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif

	const _Tp* __keyp = &__key;
	int __source = _M_losers[0]._M_source;
	for (unsigned int __pos = (_M_k + __source) / 2; __pos > 0;
	     __pos /= 2)
	  {
	    // The smaller one gets promoted, ties are broken by _M_source.
	    if (_M_comp(*_M_losers[__pos]._M_keyp, *__keyp)
		|| (!_M_comp(*__keyp, *_M_losers[__pos]._M_keyp)
		    && _M_losers[__pos]._M_source < __source))
	      {
		// The other one is smaller.
		std::swap(_M_losers[__pos]._M_source, __source);
		std::swap(_M_losers[__pos]._M_keyp, __keyp);
	      }
	  }

	_M_losers[0]._M_source = __source;
	_M_losers[0]._M_keyp = __keyp;
      }
    };

  /**
   * @brief Unstable unguarded _LoserTree variant storing pointers.
   *
   * Stable variant is above.
   */
  template<typename _Tp, typename _Compare>
    class _LoserTreePointerUnguarded</* __stable == */false, _Tp, _Compare>
    : public _LoserTreePointerUnguardedBase<_Tp, _Compare>
    {
      typedef _LoserTreePointerUnguardedBase<_Tp, _Compare> _Base;
      using _Base::_M_k;
      using _Base::_M_comp;
      using _Base::_M_losers;

  public:
      _LoserTreePointerUnguarded(unsigned int __k, const _Tp& __sentinel,
				 _Compare __comp = std::less<_Tp>())
      : _Base::_LoserTreePointerUnguardedBase(__k, __sentinel, __comp)
      { }

      unsigned int
      __init_winner(unsigned int __root)
      {
	if (__root >= _M_k)
	  return __root;
	else
	  {
	    unsigned int __left = __init_winner(2 * __root);
	    unsigned int __right = __init_winner(2 * __root + 1);

#if _GLIBCXX_PARALLEL_ASSERTIONS
	    // If __left one is sentinel then __right one must be, too.
	    if (_M_losers[__left]._M_source == -1)
	      _GLIBCXX_PARALLEL_ASSERT(_M_losers[__right]._M_source == -1);
#endif

	    if (!_M_comp(*_M_losers[__right]._M_keyp,
			 *_M_losers[__left]._M_keyp))
	      {
		// Left one is less or equal.
		_M_losers[__root] = _M_losers[__right];
		return __left;
	      }
	    else
	      {
		// Right one is less.
		_M_losers[__root] = _M_losers[__left];
		return __right;
	      }
	  }
      }

      void
      __init()
      {
	_M_losers[0] = _M_losers[__init_winner(1)];

#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top at the beginning
	// (0 sequences!)
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif
      }

      void
      __delete_min_insert(const _Tp& __key, bool __sup)
      {
#if _GLIBCXX_PARALLEL_ASSERTIONS
	// no dummy sequence can ever be at the top!
	_GLIBCXX_PARALLEL_ASSERT(_M_losers[0]._M_source != -1);
#endif

	const _Tp* __keyp = &__key;
	int __source = _M_losers[0]._M_source;
	for (unsigned int __pos = (_M_k + __source) / 2; __pos > 0;
	     __pos /= 2)
	  {
	    // The smaller one gets promoted.
	    if (_M_comp(*(_M_losers[__pos]._M_keyp), *__keyp))
	      {
		// The other one is smaller.
		std::swap(_M_losers[__pos]._M_source, __source);
		std::swap(_M_losers[__pos]._M_keyp, __keyp);
	      }
	  }

	_M_losers[0]._M_source = __source;
	_M_losers[0]._M_keyp = __keyp;
      }
    };
} // namespace __gnu_parallel

#endif /* _GLIBCXX_PARALLEL_LOSERTREE_H */
