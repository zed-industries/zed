// Safe container/iterator base implementation  -*- C++ -*-

// Copyright (C) 2011-2020 Free Software Foundation, Inc.
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

/** @file debug/safe_unordered_base.h
 *  This file is a GNU debug extension to the Standard C++ Library.
 */

#ifndef _GLIBCXX_DEBUG_SAFE_UNORDERED_BASE_H
#define _GLIBCXX_DEBUG_SAFE_UNORDERED_BASE_H 1

#include <debug/safe_base.h>

namespace __gnu_debug
{
  class _Safe_unordered_container_base;

  /** \brief Basic functionality for a @a safe iterator.
   *
   *  The %_Safe_local_iterator_base base class implements the functionality
   *  of a safe local iterator that is not specific to a particular iterator
   *  type. It contains a pointer back to the container it references
   *  along with iterator version information and pointers to form a
   *  doubly-linked list of local iterators referenced by the container.
   *
   *  This class must not perform any operations that can throw an
   *  exception, or the exception guarantees of derived iterators will
   *  be broken.
   */
  class _Safe_local_iterator_base : public _Safe_iterator_base
  {
  protected:
    /** Initializes the iterator and makes it singular. */
    _Safe_local_iterator_base()
    { }

    /** Initialize the iterator to reference the container pointed to
     *  by @p __seq. @p __constant is true when we are initializing a
     *  constant local iterator, and false if it is a mutable local iterator.
     *  Note that @p __seq may be NULL, in which case the iterator will be
     *  singular. Otherwise, the iterator will reference @p __seq and
     *  be nonsingular.
     */
    _Safe_local_iterator_base(const _Safe_sequence_base* __seq, bool __constant)
    { this->_M_attach(const_cast<_Safe_sequence_base*>(__seq), __constant); }

    /** Initializes the iterator to reference the same container that
	@p __x does. @p __constant is true if this is a constant
	iterator, and false if it is mutable. */
    _Safe_local_iterator_base(const _Safe_local_iterator_base& __x,
			      bool __constant)
    { this->_M_attach(__x._M_sequence, __constant); }

    ~_Safe_local_iterator_base() { this->_M_detach(); }

    _Safe_unordered_container_base*
    _M_get_container() const noexcept;

    /** Attaches this iterator to the given container, detaching it
     *	from whatever container it was attached to originally. If the
     *	new container is the NULL pointer, the iterator is left
     *	unattached.
     */
    void
    _M_attach(_Safe_sequence_base* __seq, bool __constant);

    /** Likewise, but not thread-safe. */
    void
    _M_attach_single(_Safe_sequence_base* __seq, bool __constant) throw ();

    /** Detach the iterator for whatever container it is attached to,
     *	if any.
    */
    void
    _M_detach();

    /** Likewise, but not thread-safe. */
    void
    _M_detach_single() throw ();
  };

  /**
   * @brief Base class that supports tracking of local iterators that
   * reference an unordered container.
   *
   * The %_Safe_unordered_container_base class provides basic support for
   * tracking iterators into an unordered container. Containers that track
   * iterators must derived from %_Safe_unordered_container_base publicly, so
   * that safe iterators (which inherit _Safe_iterator_base) can
   * attach to them. This class contains four linked lists of
   * iterators, one for constant iterators, one for mutable
   * iterators, one for constant local iterators, one for mutable local
   * iterators and a version number that allows very fast
   * invalidation of all iterators that reference the container.
   *
   * This class must ensure that no operation on it may throw an
   * exception, otherwise @a safe containers may fail to provide the
   * exception-safety guarantees required by the C++ standard.
   */
  class _Safe_unordered_container_base : public _Safe_sequence_base
  {
    friend class _Safe_local_iterator_base;
    typedef _Safe_sequence_base _Base;

  public:
    /// The list of mutable local iterators that reference this container
    _Safe_iterator_base* _M_local_iterators;

    /// The list of constant local iterators that reference this container
    _Safe_iterator_base* _M_const_local_iterators;

  protected:
    // Initialize with a version number of 1 and no iterators
    _Safe_unordered_container_base() noexcept
    : _M_local_iterators(nullptr), _M_const_local_iterators(nullptr)
    { }

    // Copy constructor does not copy iterators.
    _Safe_unordered_container_base(const _Safe_unordered_container_base&)
    noexcept
    : _Safe_unordered_container_base() { }

    // When moved unordered containers iterators are swapped.
    _Safe_unordered_container_base(_Safe_unordered_container_base&& __x)
    noexcept
    : _Safe_unordered_container_base()
    { this->_M_swap(__x); }

    /** Notify all iterators that reference this container that the
	container is being destroyed. */
    ~_Safe_unordered_container_base() noexcept
    { this->_M_detach_all(); }

    /** Detach all iterators, leaving them singular. */
    void
    _M_detach_all();

    /** Swap this container with the given container. This operation
     *  also swaps ownership of the iterators, so that when the
     *  operation is complete all iterators that originally referenced
     *  one container now reference the other container.
     */
    void
    _M_swap(_Safe_unordered_container_base& __x) noexcept;

  private:
    /** Attach an iterator to this container. */
    void
    _M_attach_local(_Safe_iterator_base* __it, bool __constant);

    /** Likewise but not thread safe. */
    void
    _M_attach_local_single(_Safe_iterator_base* __it, bool __constant) throw ();

    /** Detach an iterator from this container */
    void
    _M_detach_local(_Safe_iterator_base* __it);

    /** Likewise but not thread safe. */
    void
    _M_detach_local_single(_Safe_iterator_base* __it) throw ();
  };
} // namespace __gnu_debug

#endif
