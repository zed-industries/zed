// Debugging support implementation -*- C++ -*-

// Copyright (C) 2003-2020 Free Software Foundation, Inc.
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

/** @file debug/macros.h
 *  This file is a GNU debug extension to the Standard C++ Library.
 */

#ifndef _GLIBCXX_DEBUG_MACROS_H
#define _GLIBCXX_DEBUG_MACROS_H 1

/**
 * Macros used by the implementation to verify certain
 * properties. These macros may only be used directly by the debug
 * wrappers. Note that these are macros (instead of the more obviously
 * @a correct choice of making them functions) because we need line and
 * file information at the call site, to minimize the distance between
 * the user error and where the error is reported.
 *
 */
#if 0 /* defined _GLIBCXX_HAVE_BUILTIN_IS_CONSTANT_EVALUATED */
# define _GLIBCXX_DEBUG_VERIFY_COND_AT(_Cond,_ErrMsg,_File,_Line,_Func)	\
  if (__builtin_is_constant_evaluated())				\
    /* FIXME: Compilation error here when !_Cond. */			\
    break;								\
  if (! (_Cond))							\
    __gnu_debug::_Error_formatter::_S_at(_File, _Line, _Func)		\
      ._ErrMsg._M_error()
#else
# define _GLIBCXX_DEBUG_VERIFY_COND_AT(_Cond,_ErrMsg,_File,_Line,_Func)	\
  if (! (_Cond))							\
    __gnu_debug::_Error_formatter::_S_at(_File, _Line, _Func)		\
      ._ErrMsg._M_error()
#endif

#define _GLIBCXX_DEBUG_VERIFY_AT_F(_Cond,_ErrMsg,_File,_Line,_Func)	\
  do									\
  {									\
    _GLIBCXX_DEBUG_VERIFY_COND_AT(_Cond,_ErrMsg,_File,_Line,_Func);	\
  } while (false)

#define _GLIBCXX_DEBUG_VERIFY_AT(_Cond,_ErrMsg,_File,_Line)		\
  _GLIBCXX_DEBUG_VERIFY_AT_F(_Cond,_ErrMsg,_File,_Line,__PRETTY_FUNCTION__)

#define _GLIBCXX_DEBUG_VERIFY(_Cond,_ErrMsg)				\
  _GLIBCXX_DEBUG_VERIFY_AT_F(_Cond, _ErrMsg, __FILE__, __LINE__,	\
			     __PRETTY_FUNCTION__)

// Verify that [_First, _Last) forms a valid iterator range.
#define __glibcxx_check_valid_range(_First,_Last)			\
_GLIBCXX_DEBUG_VERIFY(__gnu_debug::__valid_range(_First, _Last),	\
		      _M_message(__gnu_debug::__msg_valid_range)	\
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last))

#define __glibcxx_check_valid_range_at(_First,_Last,_File,_Line,_Func)	\
_GLIBCXX_DEBUG_VERIFY_AT_F(__gnu_debug::__valid_range(_First, _Last),	\
			   _M_message(__gnu_debug::__msg_valid_range)	\
			   ._M_iterator(_First, #_First)		\
			   ._M_iterator(_Last, #_Last),			\
			   _File,_Line,_Func)

#define __glibcxx_check_valid_range2(_First,_Last,_Dist)		\
_GLIBCXX_DEBUG_VERIFY(__gnu_debug::__valid_range(_First, _Last, _Dist),	\
		      _M_message(__gnu_debug::__msg_valid_range)	\
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last))

#define __glibcxx_check_valid_constructor_range(_First,_Last)		\
  __gnu_debug::__check_valid_range(_First, _Last,			\
				   __FILE__, __LINE__, __PRETTY_FUNCTION__)

// Verify that [_First, _Last) forms a non-empty iterator range.
#define __glibcxx_check_non_empty_range(_First,_Last)			\
_GLIBCXX_DEBUG_VERIFY(_First != _Last,					\
		      _M_message(__gnu_debug::__msg_non_empty_range)	\
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last))

// Verify that [_First, _First + _Size) forms a valid range.
#define __glibcxx_check_can_increment(_First,_Size)			\
_GLIBCXX_DEBUG_VERIFY(__gnu_debug::__can_advance(_First, _Size),	\
		      _M_message(__gnu_debug::__msg_iter_subscript_oob)	\
		      ._M_iterator(_First, #_First)			\
		      ._M_integer(_Size, #_Size))

#define __glibcxx_check_can_increment_range(_First1,_Last1,_First2)	\
  do									\
  {									\
    typename __gnu_debug::_Distance_traits<__decltype(_First1)>::__type __dist;\
    _GLIBCXX_DEBUG_VERIFY_COND_AT(					\
			__gnu_debug::__valid_range(_First1, _Last1, __dist),\
			_M_message(__gnu_debug::__msg_valid_range)	\
			._M_iterator(_First1, #_First1)			\
			._M_iterator(_Last1, #_Last1),			\
			__FILE__,__LINE__,__PRETTY_FUNCTION__);		\
    _GLIBCXX_DEBUG_VERIFY_COND_AT(					\
			__gnu_debug::__can_advance(_First2, __dist.first),\
			_M_message(__gnu_debug::__msg_iter_subscript_oob)\
			._M_iterator(_First2, #_First2)			\
			._M_integer(__dist.first),			\
			__FILE__,__LINE__,__PRETTY_FUNCTION__);		\
  } while(false)

#define __glibcxx_check_can_decrement_range(_First1,_Last1,_First2)	\
  do									\
  {									\
    typename __gnu_debug::_Distance_traits<__decltype(_First1)>::__type __dist;\
    _GLIBCXX_DEBUG_VERIFY_COND_AT(					\
			__gnu_debug::__valid_range(_First1, _Last1, __dist),\
			_M_message(__gnu_debug::__msg_valid_range)	\
			._M_iterator(_First1, #_First1)			\
			._M_iterator(_Last1, #_Last1),			\
			__FILE__,__LINE__,__PRETTY_FUNCTION__);		\
    _GLIBCXX_DEBUG_VERIFY_COND_AT(					\
			__gnu_debug::__can_advance(_First2, -__dist.first),\
			_M_message(__gnu_debug::__msg_iter_subscript_oob)\
			._M_iterator(_First2, #_First2)			\
			._M_integer(-__dist.first),			\
			__FILE__,__LINE__,__PRETTY_FUNCTION__);		\
  } while(false)

/** Verify that we can insert into *this with the iterator _Position.
 *  Insertion into a container at a specific position requires that
 *  the iterator be nonsingular, either dereferenceable or past-the-end,
 *  and that it reference the sequence we are inserting into. Note that
 *  this macro is only valid when the container is a_Safe_sequence and
 *  the iterator is a _Safe_iterator.
*/
#define __glibcxx_check_insert(_Position)				\
_GLIBCXX_DEBUG_VERIFY(!_Position._M_singular(),				\
		      _M_message(__gnu_debug::__msg_insert_singular)	\
		      ._M_sequence(*this, "this")			\
		      ._M_iterator(_Position, #_Position));		\
_GLIBCXX_DEBUG_VERIFY(_Position._M_attached_to(this),			\
		      _M_message(__gnu_debug::__msg_insert_different)	\
		      ._M_sequence(*this, "this")			\
		      ._M_iterator(_Position, #_Position))

/** Verify that we can insert into *this after the iterator _Position.
 *  Insertion into a container after a specific position requires that
 *  the iterator be nonsingular, either dereferenceable or before-begin,
 *  and that it reference the sequence we are inserting into. Note that
 *  this macro is only valid when the container is a_Safe_sequence and
 *  the iterator is a _Safe_iterator.
*/
#define __glibcxx_check_insert_after(_Position)				\
__glibcxx_check_insert(_Position);					\
_GLIBCXX_DEBUG_VERIFY(!_Position._M_is_end(),				\
		      _M_message(__gnu_debug::__msg_insert_after_end)	\
		      ._M_sequence(*this, "this")			\
		      ._M_iterator(_Position, #_Position))

/** Verify that we can insert the values in the iterator range
 *  [_First, _Last) into *this with the iterator _Position.  Insertion
 *  into a container at a specific position requires that the iterator
 *  be nonsingular (i.e., either dereferenceable or past-the-end),
 *  that it reference the sequence we are inserting into, and that the
 *  iterator range [_First, _Last) is a valid (possibly empty)
 *  range which does not reference the sequence we are inserting into.
 *  Note that this macro is only valid when the container is a
 *  _Safe_sequence and the _Position iterator is a _Safe_iterator.
*/
#define __glibcxx_check_insert_range(_Position,_First,_Last,_Dist)	\
__glibcxx_check_valid_range2(_First,_Last,_Dist);			\
__glibcxx_check_insert(_Position);					\
_GLIBCXX_DEBUG_VERIFY(__gnu_debug::__foreign_iterator(_Position,_First,_Last),\
		      _M_message(__gnu_debug::__msg_insert_range_from_self)\
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last)			\
		      ._M_sequence(*this, "this"))

/** Verify that we can insert the values in the iterator range
 *  [_First, _Last) into *this after the iterator _Position.  Insertion
 *  into a container after a specific position requires that the iterator
 *  be nonsingular (i.e., either dereferenceable or past-the-end),
 *  that it reference the sequence we are inserting into, and that the
 *  iterator range [_First, _Last) is a valid (possibly empty)
 *  range which does not reference the sequence we are inserting into.
 *  Note that this macro is only valid when the container is a
 *  _Safe_sequence and the _Position iterator is a _Safe_iterator.
*/
#define __glibcxx_check_insert_range_after(_Position,_First,_Last,_Dist)\
__glibcxx_check_valid_range2(_First,_Last,_Dist);			\
__glibcxx_check_insert_after(_Position);				\
_GLIBCXX_DEBUG_VERIFY(__gnu_debug::__foreign_iterator(_Position,_First,_Last),\
		      _M_message(__gnu_debug::__msg_insert_range_from_self)\
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last)			\
		      ._M_sequence(*this, "this"))

/** Verify that we can erase the element referenced by the iterator
 * _Position. We can erase the element if the _Position iterator is
 * dereferenceable and references this sequence.
*/
#define __glibcxx_check_erase(_Position)				\
_GLIBCXX_DEBUG_VERIFY(_Position._M_dereferenceable(),			\
		      _M_message(__gnu_debug::__msg_erase_bad)	        \
                      ._M_sequence(*this, "this")			\
		      ._M_iterator(_Position, #_Position));		\
_GLIBCXX_DEBUG_VERIFY(_Position._M_attached_to(this),			\
		      _M_message(__gnu_debug::__msg_erase_different)    \
		      ._M_sequence(*this, "this")			\
		      ._M_iterator(_Position, #_Position))

/** Verify that we can erase the element after the iterator
 * _Position. We can erase the element if the _Position iterator is
 * before a dereferenceable one and references this sequence.
*/
#define __glibcxx_check_erase_after(_Position)				\
_GLIBCXX_DEBUG_VERIFY(_Position._M_before_dereferenceable(),		\
		      _M_message(__gnu_debug::__msg_erase_after_bad)	\
		      ._M_sequence(*this, "this")			\
		      ._M_iterator(_Position, #_Position));		\
_GLIBCXX_DEBUG_VERIFY(_Position._M_attached_to(this),			\
		      _M_message(__gnu_debug::__msg_erase_different)	\
		      ._M_sequence(*this, "this")			\
		      ._M_iterator(_Position, #_Position))

/** Verify that we can erase the elements in the iterator range
 *  [_First, _Last). We can erase the elements if [_First, _Last) is a
 *  valid iterator range within this sequence.
*/
#define __glibcxx_check_erase_range(_First,_Last)			\
__glibcxx_check_valid_range(_First,_Last);				\
_GLIBCXX_DEBUG_VERIFY(_First._M_attached_to(this),			\
		      _M_message(__gnu_debug::__msg_erase_different)    \
                      ._M_sequence(*this, "this")			\
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last))

/** Verify that we can erase the elements in the iterator range
 *  (_First, _Last). We can erase the elements if (_First, _Last) is a
 *  valid iterator range within this sequence.
*/
#define __glibcxx_check_erase_range_after(_First,_Last)			\
_GLIBCXX_DEBUG_VERIFY(_First._M_can_compare(_Last),			\
		      _M_message(__gnu_debug::__msg_erase_different)	\
		      ._M_sequence(*this, "this")			\
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last));			\
_GLIBCXX_DEBUG_VERIFY(_First._M_attached_to(this),			\
		      _M_message(__gnu_debug::__msg_erase_different)	\
		      ._M_sequence(*this, "this")			\
		      ._M_iterator(_First, #_First));			\
_GLIBCXX_DEBUG_VERIFY(_First != _Last,					\
		      _M_message(__gnu_debug::__msg_valid_range2)	\
		      ._M_sequence(*this, "this")			\
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last));			\
_GLIBCXX_DEBUG_VERIFY(_First._M_incrementable(),			\
		      _M_message(__gnu_debug::__msg_valid_range2)	\
		      ._M_sequence(*this, "this")			\
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last));			\
_GLIBCXX_DEBUG_VERIFY(!_Last._M_is_before_begin(),			\
		      _M_message(__gnu_debug::__msg_valid_range2)	\
		      ._M_sequence(*this, "this")			\
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last))			\

// Verify that the subscript _N is less than the container's size.
#define __glibcxx_check_subscript(_N)					\
_GLIBCXX_DEBUG_VERIFY(_N < this->size(),				\
		      _M_message(__gnu_debug::__msg_subscript_oob)	\
                      ._M_sequence(*this, "this")			\
		      ._M_integer(_N, #_N)				\
		      ._M_integer(this->size(), "size"))

// Verify that the bucket _N is less than the container's buckets count.
#define __glibcxx_check_bucket_index(_N)				\
_GLIBCXX_DEBUG_VERIFY(_N < this->bucket_count(),			\
		      _M_message(__gnu_debug::__msg_bucket_index_oob)	\
                      ._M_sequence(*this, "this")			\
		      ._M_integer(_N, #_N)				\
		      ._M_integer(this->bucket_count(), "size"))

// Verify that the container is nonempty
#define __glibcxx_check_nonempty()					\
_GLIBCXX_DEBUG_VERIFY(! this->empty(),					\
		      _M_message(__gnu_debug::__msg_empty)	        \
                      ._M_sequence(*this, "this"))

// Verify that a predicate is irreflexive
#define __glibcxx_check_irreflexive(_First,_Last)			\
  _GLIBCXX_DEBUG_VERIFY(_First == _Last || !(*_First < *_First),	\
			_M_message(__gnu_debug::__msg_irreflexive_ordering) \
			._M_iterator_value_type(_First, "< operator type"))

#if __cplusplus >= 201103L
# define __glibcxx_check_irreflexive2(_First,_Last)			\
  _GLIBCXX_DEBUG_VERIFY(_First == _Last					\
			|| __gnu_debug::__is_irreflexive(_First),	\
			_M_message(__gnu_debug::__msg_irreflexive_ordering) \
			._M_iterator_value_type(_First, "< operator type"))
#else
# define __glibcxx_check_irreflexive2(_First,_Last)
#endif

#define __glibcxx_check_irreflexive_pred(_First,_Last,_Pred)		\
  _GLIBCXX_DEBUG_VERIFY(_First == _Last || !_Pred(*_First, *_First),	\
			_M_message(__gnu_debug::__msg_irreflexive_ordering) \
			._M_instance(_Pred, "functor")			\
			._M_iterator_value_type(_First, "ordered type"))

#if __cplusplus >= 201103L
# define __glibcxx_check_irreflexive_pred2(_First,_Last,_Pred)		\
  _GLIBCXX_DEBUG_VERIFY(_First == _Last					\
			||__gnu_debug::__is_irreflexive_pred(_First, _Pred), \
			_M_message(__gnu_debug::__msg_irreflexive_ordering) \
			._M_instance(_Pred, "functor")			\
			._M_iterator_value_type(_First, "ordered type"))
#else
# define __glibcxx_check_irreflexive_pred2(_First,_Last,_Pred)
#endif

// Verify that the iterator range [_First, _Last) is sorted
#define __glibcxx_check_sorted(_First,_Last)				\
__glibcxx_check_valid_range(_First,_Last);				\
__glibcxx_check_irreflexive(_First,_Last);				\
 _GLIBCXX_DEBUG_VERIFY(__gnu_debug::__check_sorted(			\
			__gnu_debug::__base(_First),			\
			__gnu_debug::__base(_Last)),			\
		      _M_message(__gnu_debug::__msg_unsorted)	        \
                      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last))

/** Verify that the iterator range [_First, _Last) is sorted by the
    predicate _Pred. */
#define __glibcxx_check_sorted_pred(_First,_Last,_Pred)			\
__glibcxx_check_valid_range(_First,_Last);				\
__glibcxx_check_irreflexive_pred(_First,_Last,_Pred);			\
_GLIBCXX_DEBUG_VERIFY(__gnu_debug::__check_sorted(			\
			__gnu_debug::__base(_First),			\
			__gnu_debug::__base(_Last), _Pred),		\
		      _M_message(__gnu_debug::__msg_unsorted_pred)      \
                      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last)			\
		      ._M_string(#_Pred))

// Special variant for std::merge, std::includes, std::set_*
#define __glibcxx_check_sorted_set(_First1,_Last1,_First2)		\
__glibcxx_check_valid_range(_First1,_Last1);				\
_GLIBCXX_DEBUG_VERIFY(                                                  \
  __gnu_debug::__check_sorted_set(__gnu_debug::__base(_First1),		\
				  __gnu_debug::__base(_Last1), _First2),\
  _M_message(__gnu_debug::__msg_unsorted)				\
  ._M_iterator(_First1, #_First1)					\
  ._M_iterator(_Last1, #_Last1))

// Likewise with a _Pred.
#define __glibcxx_check_sorted_set_pred(_First1,_Last1,_First2,_Pred)	\
__glibcxx_check_valid_range(_First1,_Last1);        			\
_GLIBCXX_DEBUG_VERIFY(							\
  __gnu_debug::__check_sorted_set(__gnu_debug::__base(_First1),		\
				  __gnu_debug::__base(_Last1),		\
				  _First2, _Pred),			\
  _M_message(__gnu_debug::__msg_unsorted_pred)				\
  ._M_iterator(_First1, #_First1)					\
  ._M_iterator(_Last1, #_Last1)						\
  ._M_string(#_Pred))

/** Verify that the iterator range [_First, _Last) is partitioned
    w.r.t. the value _Value. */
#define __glibcxx_check_partitioned_lower(_First,_Last,_Value)		\
__glibcxx_check_valid_range(_First,_Last);				\
_GLIBCXX_DEBUG_VERIFY(__gnu_debug::__check_partitioned_lower(		\
			__gnu_debug::__base(_First),			\
			__gnu_debug::__base(_Last), _Value),		\
		      _M_message(__gnu_debug::__msg_unpartitioned)      \
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last)			\
		      ._M_string(#_Value))

#define __glibcxx_check_partitioned_upper(_First,_Last,_Value)		\
__glibcxx_check_valid_range(_First,_Last);				\
_GLIBCXX_DEBUG_VERIFY(__gnu_debug::__check_partitioned_upper(		\
			__gnu_debug::__base(_First),			\
			__gnu_debug::__base(_Last), _Value),		\
		      _M_message(__gnu_debug::__msg_unpartitioned)      \
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last)			\
		      ._M_string(#_Value))

/** Verify that the iterator range [_First, _Last) is partitioned
    w.r.t. the value _Value and predicate _Pred. */
#define __glibcxx_check_partitioned_lower_pred(_First,_Last,_Value,_Pred) \
__glibcxx_check_valid_range(_First,_Last);				\
_GLIBCXX_DEBUG_VERIFY(__gnu_debug::__check_partitioned_lower(		\
			__gnu_debug::__base(_First),			\
			__gnu_debug::__base(_Last), _Value, _Pred),	\
		      _M_message(__gnu_debug::__msg_unpartitioned_pred) \
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last)			\
		      ._M_string(#_Pred)				\
                      ._M_string(#_Value))

/** Verify that the iterator range [_First, _Last) is partitioned
    w.r.t. the value _Value and predicate _Pred. */
#define __glibcxx_check_partitioned_upper_pred(_First,_Last,_Value,_Pred) \
__glibcxx_check_valid_range(_First,_Last);				\
_GLIBCXX_DEBUG_VERIFY(__gnu_debug::__check_partitioned_upper(		\
			__gnu_debug::__base(_First),			\
			__gnu_debug::__base(_Last), _Value, _Pred),	\
		      _M_message(__gnu_debug::__msg_unpartitioned_pred) \
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last)			\
		      ._M_string(#_Pred)				\
                      ._M_string(#_Value))

// Verify that the iterator range [_First, _Last) is a heap
#define __glibcxx_check_heap(_First,_Last)				\
  _GLIBCXX_DEBUG_VERIFY(std::__is_heap(__gnu_debug::__base(_First),	\
				       __gnu_debug::__base(_Last)),	\
		      _M_message(__gnu_debug::__msg_not_heap)	        \
		      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last))

/** Verify that the iterator range [_First, _Last) is a heap
    w.r.t. the predicate _Pred. */
#define __glibcxx_check_heap_pred(_First,_Last,_Pred)			\
  _GLIBCXX_DEBUG_VERIFY(std::__is_heap(__gnu_debug::__base(_First),	\
				       __gnu_debug::__base(_Last),	\
				       _Pred),				\
		      _M_message(__gnu_debug::__msg_not_heap_pred)      \
                      ._M_iterator(_First, #_First)			\
		      ._M_iterator(_Last, #_Last)			\
		      ._M_string(#_Pred))

// Verify that the container is not self move assigned
#define __glibcxx_check_self_move_assign(_Other)			\
_GLIBCXX_DEBUG_VERIFY(this != &_Other,					\
		      _M_message(__gnu_debug::__msg_self_move_assign)	\
                      ._M_sequence(*this, "this"))

// Verify that load factor is positive
#define __glibcxx_check_max_load_factor(_F)				\
_GLIBCXX_DEBUG_VERIFY(_F > 0.0f,					\
		      _M_message(__gnu_debug::__msg_valid_load_factor)	\
                      ._M_sequence(*this, "this"))

#define __glibcxx_check_equal_allocs(_This, _Other)			\
_GLIBCXX_DEBUG_VERIFY(_This.get_allocator() == _Other.get_allocator(),	\
		      _M_message(__gnu_debug::__msg_equal_allocs)	\
		      ._M_sequence(_This, "this"))

#define __glibcxx_check_string(_String) _GLIBCXX_DEBUG_PEDASSERT(_String != 0)
#define __glibcxx_check_string_len(_String,_Len) \
  _GLIBCXX_DEBUG_PEDASSERT(_String != 0 || _Len == 0)

#endif
