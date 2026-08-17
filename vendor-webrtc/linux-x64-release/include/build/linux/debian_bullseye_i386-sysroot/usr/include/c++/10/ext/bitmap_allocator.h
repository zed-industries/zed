// Bitmap Allocator. -*- C++ -*-

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

/** @file ext/bitmap_allocator.h
 *  This file is a GNU extension to the Standard C++ Library.
 */

#ifndef _BITMAP_ALLOCATOR_H
#define _BITMAP_ALLOCATOR_H 1

#include <utility> // For std::pair.
#include <bits/functexcept.h> // For __throw_bad_alloc().
#include <functional> // For greater_equal, and less_equal.
#include <new> // For operator new.
#include <debug/debug.h> // _GLIBCXX_DEBUG_ASSERT
#include <ext/concurrence.h>
#include <bits/move.h>

/** @brief The constant in the expression below is the alignment
 * required in bytes.
 */
#define _BALLOC_ALIGN_BYTES 8

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  namespace __detail
  {
    /** @class  __mini_vector bitmap_allocator.h bitmap_allocator.h
     *
     *  @brief  __mini_vector<> is a stripped down version of the
     *  full-fledged std::vector<>.
     *
     *  It is to be used only for built-in types or PODs. Notable
     *  differences are:
     * 
     *  1. Not all accessor functions are present.
     *  2. Used ONLY for PODs.
     *  3. No Allocator template argument. Uses ::operator new() to get
     *  memory, and ::operator delete() to free it.
     *  Caveat: The dtor does NOT free the memory allocated, so this a
     *  memory-leaking vector!
     */
    template<typename _Tp>
      class __mini_vector
      {
	__mini_vector(const __mini_vector&);
	__mini_vector& operator=(const __mini_vector&);

      public:
	typedef _Tp value_type;
	typedef _Tp* pointer;
	typedef _Tp& reference;
	typedef const _Tp& const_reference;
	typedef std::size_t size_type;
	typedef std::ptrdiff_t difference_type;
	typedef pointer iterator;

      private:
	pointer _M_start;
	pointer _M_finish;
	pointer _M_end_of_storage;

	size_type
	_M_space_left() const throw()
	{ return _M_end_of_storage - _M_finish; }

	_GLIBCXX_NODISCARD pointer
	allocate(size_type __n)
	{ return static_cast<pointer>(::operator new(__n * sizeof(_Tp))); }

	void
	deallocate(pointer __p, size_type)
	{ ::operator delete(__p); }

      public:
	// Members used: size(), push_back(), pop_back(),
	// insert(iterator, const_reference), erase(iterator),
	// begin(), end(), back(), operator[].

	__mini_vector()
        : _M_start(0), _M_finish(0), _M_end_of_storage(0) { }

	size_type
	size() const throw()
	{ return _M_finish - _M_start; }

	iterator
	begin() const throw()
	{ return this->_M_start; }

	iterator
	end() const throw()
	{ return this->_M_finish; }

	reference
	back() const throw()
	{ return *(this->end() - 1); }

	reference
	operator[](const size_type __pos) const throw()
	{ return this->_M_start[__pos]; }

	void
	insert(iterator __pos, const_reference __x);

	void
	push_back(const_reference __x)
	{
	  if (this->_M_space_left())
	    {
	      *this->end() = __x;
	      ++this->_M_finish;
	    }
	  else
	    this->insert(this->end(), __x);
	}

	void
	pop_back() throw()
	{ --this->_M_finish; }

	void
	erase(iterator __pos) throw();

	void
	clear() throw()
	{ this->_M_finish = this->_M_start; }
      };

    // Out of line function definitions.
    template<typename _Tp>
      void __mini_vector<_Tp>::
      insert(iterator __pos, const_reference __x)
      {
	if (this->_M_space_left())
	  {
	    size_type __to_move = this->_M_finish - __pos;
	    iterator __dest = this->end();
	    iterator __src = this->end() - 1;

	    ++this->_M_finish;
	    while (__to_move)
	      {
		*__dest = *__src;
		--__dest; --__src; --__to_move;
	      }
	    *__pos = __x;
	  }
	else
	  {
	    size_type __new_size = this->size() ? this->size() * 2 : 1;
	    iterator __new_start = this->allocate(__new_size);
	    iterator __first = this->begin();
	    iterator __start = __new_start;
	    while (__first != __pos)
	      {
		*__start = *__first;
		++__start; ++__first;
	      }
	    *__start = __x;
	    ++__start;
	    while (__first != this->end())
	      {
		*__start = *__first;
		++__start; ++__first;
	      }
	    if (this->_M_start)
	      this->deallocate(this->_M_start, this->size());

	    this->_M_start = __new_start;
	    this->_M_finish = __start;
	    this->_M_end_of_storage = this->_M_start + __new_size;
	  }
      }

    template<typename _Tp>
      void __mini_vector<_Tp>::
      erase(iterator __pos) throw()
      {
	while (__pos + 1 != this->end())
	  {
	    *__pos = __pos[1];
	    ++__pos;
	  }
	--this->_M_finish;
      }


    template<typename _Tp>
      struct __mv_iter_traits
      {
	typedef typename _Tp::value_type value_type;
	typedef typename _Tp::difference_type difference_type;
      };

    template<typename _Tp>
      struct __mv_iter_traits<_Tp*>
      {
	typedef _Tp value_type;
	typedef std::ptrdiff_t difference_type;
      };

    enum 
      { 
	bits_per_byte = 8,
	bits_per_block = sizeof(std::size_t) * std::size_t(bits_per_byte)
      };

    template<typename _ForwardIterator, typename _Tp, typename _Compare>
      _ForwardIterator
      __lower_bound(_ForwardIterator __first, _ForwardIterator __last,
		    const _Tp& __val, _Compare __comp)
      {
	typedef typename __mv_iter_traits<_ForwardIterator>::difference_type
	  _DistanceType;

	_DistanceType __len = __last - __first;
	_DistanceType __half;
	_ForwardIterator __middle;

	while (__len > 0)
	  {
	    __half = __len >> 1;
	    __middle = __first;
	    __middle += __half;
	    if (__comp(*__middle, __val))
	      {
		__first = __middle;
		++__first;
		__len = __len - __half - 1;
	      }
	    else
	      __len = __half;
	  }
	return __first;
      }

    /** @brief The number of Blocks pointed to by the address pair
     *  passed to the function.
     */
    template<typename _AddrPair>
      inline std::size_t
      __num_blocks(_AddrPair __ap)
      { return (__ap.second - __ap.first) + 1; }

    /** @brief The number of Bit-maps pointed to by the address pair
     *  passed to the function.
     */
    template<typename _AddrPair>
      inline std::size_t
      __num_bitmaps(_AddrPair __ap)
      { return __num_blocks(__ap) / std::size_t(bits_per_block); }

    // _Tp should be a pointer type.
    template<typename _Tp>
      class _Inclusive_between 
      : public std::unary_function<typename std::pair<_Tp, _Tp>, bool>
      {
	typedef _Tp pointer;
	pointer _M_ptr_value;
	typedef typename std::pair<_Tp, _Tp> _Block_pair;
	
      public:
	_Inclusive_between(pointer __ptr) : _M_ptr_value(__ptr) 
	{ }
	
	bool 
	operator()(_Block_pair __bp) const throw()
	{
	  if (std::less_equal<pointer>()(_M_ptr_value, __bp.second) 
	      && std::greater_equal<pointer>()(_M_ptr_value, __bp.first))
	    return true;
	  else
	    return false;
	}
      };
  
    // Used to pass a Functor to functions by reference.
    template<typename _Functor>
      class _Functor_Ref 
      : public std::unary_function<typename _Functor::argument_type, 
				   typename _Functor::result_type>
      {
	_Functor& _M_fref;
	
      public:
	typedef typename _Functor::argument_type argument_type;
	typedef typename _Functor::result_type result_type;

	_Functor_Ref(_Functor& __fref) : _M_fref(__fref) 
	{ }

	result_type 
	operator()(argument_type __arg) 
	{ return _M_fref(__arg); }
      };

    /** @class  _Ffit_finder bitmap_allocator.h bitmap_allocator.h
     *
     *  @brief  The class which acts as a predicate for applying the
     *  first-fit memory allocation policy for the bitmap allocator.
     */
    // _Tp should be a pointer type, and _Alloc is the Allocator for
    // the vector.
    template<typename _Tp>
      class _Ffit_finder 
      : public std::unary_function<typename std::pair<_Tp, _Tp>, bool>
      {
	typedef typename std::pair<_Tp, _Tp> _Block_pair;
	typedef typename __detail::__mini_vector<_Block_pair> _BPVector;
	typedef typename _BPVector::difference_type _Counter_type;

	std::size_t* _M_pbitmap;
	_Counter_type _M_data_offset;

      public:
	_Ffit_finder() : _M_pbitmap(0), _M_data_offset(0)
	{ }

	bool 
	operator()(_Block_pair __bp) throw()
	{
	  using std::size_t;
	  // Set the _rover to the last physical location bitmap,
	  // which is the bitmap which belongs to the first free
	  // block. Thus, the bitmaps are in exact reverse order of
	  // the actual memory layout. So, we count down the bitmaps,
	  // which is the same as moving up the memory.

	  // If the used count stored at the start of the Bit Map headers
	  // is equal to the number of Objects that the current Block can
	  // store, then there is definitely no space for another single
	  // object, so just return false.
	  _Counter_type __diff = __detail::__num_bitmaps(__bp);

	  if (*(reinterpret_cast<size_t*>
		(__bp.first) - (__diff + 1)) == __detail::__num_blocks(__bp))
	    return false;

	  size_t* __rover = reinterpret_cast<size_t*>(__bp.first) - 1;

	  for (_Counter_type __i = 0; __i < __diff; ++__i)
	    {
	      _M_data_offset = __i;
	      if (*__rover)
		{
		  _M_pbitmap = __rover;
		  return true;
		}
	      --__rover;
	    }
	  return false;
	}
    
	std::size_t*
	_M_get() const throw()
	{ return _M_pbitmap; }

	_Counter_type
	_M_offset() const throw()
	{ return _M_data_offset * std::size_t(bits_per_block); }
      };

    /** @class  _Bitmap_counter bitmap_allocator.h bitmap_allocator.h
     *
     *  @brief  The bitmap counter which acts as the bitmap
     *  manipulator, and manages the bit-manipulation functions and
     *  the searching and identification functions on the bit-map.
     */
    // _Tp should be a pointer type.
    template<typename _Tp>
      class _Bitmap_counter
      {
	typedef typename
	__detail::__mini_vector<typename std::pair<_Tp, _Tp> > _BPVector;
	typedef typename _BPVector::size_type _Index_type;
	typedef _Tp pointer;

	_BPVector& _M_vbp;
	std::size_t* _M_curr_bmap;
	std::size_t* _M_last_bmap_in_block;
	_Index_type _M_curr_index;
    
      public:
	// Use the 2nd parameter with care. Make sure that such an
	// entry exists in the vector before passing that particular
	// index to this ctor.
	_Bitmap_counter(_BPVector& Rvbp, long __index = -1) : _M_vbp(Rvbp)
	{ this->_M_reset(__index); }
    
	void 
	_M_reset(long __index = -1) throw()
	{
	  if (__index == -1)
	    {
	      _M_curr_bmap = 0;
	      _M_curr_index = static_cast<_Index_type>(-1);
	      return;
	    }

	  _M_curr_index = __index;
	  _M_curr_bmap = reinterpret_cast<std::size_t*>
	    (_M_vbp[_M_curr_index].first) - 1;
	  
	  _GLIBCXX_DEBUG_ASSERT(__index <= (long)_M_vbp.size() - 1);
	
	  _M_last_bmap_in_block = _M_curr_bmap
	    - ((_M_vbp[_M_curr_index].second 
		- _M_vbp[_M_curr_index].first + 1) 
	       / std::size_t(bits_per_block) - 1);
	}
    
	// Dangerous Function! Use with extreme care. Pass to this
	// function ONLY those values that are known to be correct,
	// otherwise this will mess up big time.
	void
	_M_set_internal_bitmap(std::size_t* __new_internal_marker) throw()
	{ _M_curr_bmap = __new_internal_marker; }
    
	bool
	_M_finished() const throw()
	{ return(_M_curr_bmap == 0); }
    
	_Bitmap_counter&
	operator++() throw()
	{
	  if (_M_curr_bmap == _M_last_bmap_in_block)
	    {
	      if (++_M_curr_index == _M_vbp.size())
		_M_curr_bmap = 0;
	      else
		this->_M_reset(_M_curr_index);
	    }
	  else
	    --_M_curr_bmap;
	  return *this;
	}
    
	std::size_t*
	_M_get() const throw()
	{ return _M_curr_bmap; }
    
	pointer 
	_M_base() const throw()
	{ return _M_vbp[_M_curr_index].first; }

	_Index_type
	_M_offset() const throw()
	{
	  return std::size_t(bits_per_block)
	    * ((reinterpret_cast<std::size_t*>(this->_M_base())
		- _M_curr_bmap) - 1);
	}
    
	_Index_type
	_M_where() const throw()
	{ return _M_curr_index; }
      };

    /** @brief  Mark a memory address as allocated by re-setting the
     *  corresponding bit in the bit-map.
     */
    inline void 
    __bit_allocate(std::size_t* __pbmap, std::size_t __pos) throw()
    {
      std::size_t __mask = 1 << __pos;
      __mask = ~__mask;
      *__pbmap &= __mask;
    }
  
    /** @brief  Mark a memory address as free by setting the
     *  corresponding bit in the bit-map.
     */
    inline void 
    __bit_free(std::size_t* __pbmap, std::size_t __pos) throw()
    {
      std::size_t __mask = 1 << __pos;
      *__pbmap |= __mask;
    }
  } // namespace __detail

  /** @brief  Generic Version of the bsf instruction.
   */
  inline std::size_t
  _Bit_scan_forward(std::size_t __num)
  { return static_cast<std::size_t>(__builtin_ctzl(__num)); }

  /** @class  free_list bitmap_allocator.h bitmap_allocator.h
   *
   *  @brief  The free list class for managing chunks of memory to be
   *  given to and returned by the bitmap_allocator.
   */
  class free_list
  {
  public:
    typedef std::size_t* 			value_type;
    typedef __detail::__mini_vector<value_type> vector_type;
    typedef vector_type::iterator 		iterator;
    typedef __mutex				__mutex_type;

  private:
    struct _LT_pointer_compare
    {
      bool
      operator()(const std::size_t* __pui,
		 const std::size_t __cui) const throw()
      { return *__pui < __cui; }
    };

#if defined __GTHREADS
    __mutex_type&
    _M_get_mutex()
    {
      static __mutex_type _S_mutex;
      return _S_mutex;
    }
#endif

    vector_type&
    _M_get_free_list()
    {
      static vector_type _S_free_list;
      return _S_free_list;
    }

    /** @brief  Performs validation of memory based on their size.
     *
     *  @param  __addr The pointer to the memory block to be
     *  validated.
     *
     *  Validates the memory block passed to this function and
     *  appropriately performs the action of managing the free list of
     *  blocks by adding this block to the free list or deleting this
     *  or larger blocks from the free list.
     */
    void
    _M_validate(std::size_t* __addr) throw()
    {
      vector_type& __free_list = _M_get_free_list();
      const vector_type::size_type __max_size = 64;
      if (__free_list.size() >= __max_size)
	{
	  // Ok, the threshold value has been reached.  We determine
	  // which block to remove from the list of free blocks.
	  if (*__addr >= *__free_list.back())
	    {
	      // Ok, the new block is greater than or equal to the
	      // last block in the list of free blocks. We just free
	      // the new block.
	      ::operator delete(static_cast<void*>(__addr));
	      return;
	    }
	  else
	    {
	      // Deallocate the last block in the list of free lists,
	      // and insert the new one in its correct position.
	      ::operator delete(static_cast<void*>(__free_list.back()));
	      __free_list.pop_back();
	    }
	}
	  
      // Just add the block to the list of free lists unconditionally.
      iterator __temp = __detail::__lower_bound
	(__free_list.begin(), __free_list.end(), 
	 *__addr, _LT_pointer_compare());

      // We may insert the new free list before _temp;
      __free_list.insert(__temp, __addr);
    }

    /** @brief  Decides whether the wastage of memory is acceptable for
     *  the current memory request and returns accordingly.
     *
     *  @param __block_size The size of the block available in the free
     *  list.
     *
     *  @param __required_size The required size of the memory block.
     *
     *  @return true if the wastage incurred is acceptable, else returns
     *  false.
     */
    bool 
    _M_should_i_give(std::size_t __block_size,
		     std::size_t __required_size) throw()
    {
      const std::size_t __max_wastage_percentage = 36;
      if (__block_size >= __required_size && 
	  (((__block_size - __required_size) * 100 / __block_size)
	   < __max_wastage_percentage))
	return true;
      else
	return false;
    }

  public:
    /** @brief This function returns the block of memory to the
     *  internal free list.
     *
     *  @param  __addr The pointer to the memory block that was given
     *  by a call to the _M_get function.
     */
    inline void 
    _M_insert(std::size_t* __addr) throw()
    {
#if defined __GTHREADS
      __scoped_lock __bfl_lock(_M_get_mutex());
#endif
      // Call _M_validate to decide what should be done with
      // this particular free list.
      this->_M_validate(reinterpret_cast<std::size_t*>(__addr) - 1);
      // See discussion as to why this is 1!
    }
    
    /** @brief  This function gets a block of memory of the specified
     *  size from the free list.
     *
     *  @param  __sz The size in bytes of the memory required.
     *
     *  @return  A pointer to the new memory block of size at least
     *  equal to that requested.
     */
    std::size_t*
    _M_get(std::size_t __sz) _GLIBCXX_THROW(std::bad_alloc);

    /** @brief  This function just clears the internal Free List, and
     *  gives back all the memory to the OS.
     */
    void 
    _M_clear();
  };


  // Forward declare the class.
  template<typename _Tp> 
    class bitmap_allocator;

  // Specialize for void:
  template<>
    class bitmap_allocator<void>
    {
    public:
      typedef void*       pointer;
      typedef const void* const_pointer;

      // Reference-to-void members are impossible.
      typedef void  value_type;
      template<typename _Tp1>
        struct rebind
	{
	  typedef bitmap_allocator<_Tp1> other;
	};
    };

  /**
   * @brief Bitmap Allocator, primary template.
   * @ingroup allocators
   */
  template<typename _Tp>
    class bitmap_allocator : private free_list
    {
    public:
      typedef std::size_t    		size_type;
      typedef std::ptrdiff_t 		difference_type;
      typedef _Tp*        		pointer;
      typedef const _Tp*  		const_pointer;
      typedef _Tp&        		reference;
      typedef const _Tp&  		const_reference;
      typedef _Tp         		value_type;
      typedef free_list::__mutex_type 	__mutex_type;

      template<typename _Tp1>
        struct rebind
	{
	  typedef bitmap_allocator<_Tp1> other;
	};

#if __cplusplus >= 201103L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2103. propagate_on_container_move_assignment
      typedef std::true_type propagate_on_container_move_assignment;
#endif

    private:
      template<std::size_t _BSize, std::size_t _AlignSize>
        struct aligned_size
	{
	  enum
	    { 
	      modulus = _BSize % _AlignSize,
	      value = _BSize + (modulus ? _AlignSize - (modulus) : 0)
	    };
	};

      struct _Alloc_block
      {
	char __M_unused[aligned_size<sizeof(value_type),
			_BALLOC_ALIGN_BYTES>::value];
      };


      typedef typename std::pair<_Alloc_block*, _Alloc_block*> _Block_pair;

      typedef typename __detail::__mini_vector<_Block_pair> _BPVector;
      typedef typename _BPVector::iterator _BPiter;

      template<typename _Predicate>
        static _BPiter
        _S_find(_Predicate __p)
        {
	  _BPiter __first = _S_mem_blocks.begin();
	  while (__first != _S_mem_blocks.end() && !__p(*__first))
	    ++__first;
	  return __first;
	}

#if defined _GLIBCXX_DEBUG
      // Complexity: O(lg(N)). Where, N is the number of block of size
      // sizeof(value_type).
      void 
      _S_check_for_free_blocks() throw()
      {
	typedef typename __detail::_Ffit_finder<_Alloc_block*> _FFF;
	_BPiter __bpi = _S_find(_FFF());

	_GLIBCXX_DEBUG_ASSERT(__bpi == _S_mem_blocks.end());
      }
#endif

      /** @brief  Responsible for exponentially growing the internal
       *  memory pool.
       *
       *  @throw  std::bad_alloc. If memory cannot be allocated.
       *
       *  Complexity: O(1), but internally depends upon the
       *  complexity of the function free_list::_M_get. The part where
       *  the bitmap headers are written has complexity: O(X),where X
       *  is the number of blocks of size sizeof(value_type) within
       *  the newly acquired block. Having a tight bound.
       */
      void 
      _S_refill_pool() _GLIBCXX_THROW(std::bad_alloc)
      {
	using std::size_t;
#if defined _GLIBCXX_DEBUG
	_S_check_for_free_blocks();
#endif

	const size_t __num_bitmaps = (_S_block_size
				      / size_t(__detail::bits_per_block));
	const size_t __size_to_allocate = sizeof(size_t) 
	  + _S_block_size * sizeof(_Alloc_block) 
	  + __num_bitmaps * sizeof(size_t);

	size_t* __temp =
	  reinterpret_cast<size_t*>(this->_M_get(__size_to_allocate));
	*__temp = 0;
	++__temp;

	// The Header information goes at the Beginning of the Block.
	_Block_pair __bp = 
	  std::make_pair(reinterpret_cast<_Alloc_block*>
			 (__temp + __num_bitmaps), 
			 reinterpret_cast<_Alloc_block*>
			 (__temp + __num_bitmaps) 
			 + _S_block_size - 1);
	
	// Fill the Vector with this information.
	_S_mem_blocks.push_back(__bp);

	for (size_t __i = 0; __i < __num_bitmaps; ++__i)
	  __temp[__i] = ~static_cast<size_t>(0); // 1 Indicates all Free.

	_S_block_size *= 2;
      }

      static _BPVector _S_mem_blocks;
      static std::size_t _S_block_size;
      static __detail::_Bitmap_counter<_Alloc_block*> _S_last_request;
      static typename _BPVector::size_type _S_last_dealloc_index;
#if defined __GTHREADS
      static __mutex_type _S_mut;
#endif

    public:

      /** @brief  Allocates memory for a single object of size
       *  sizeof(_Tp).
       *
       *  @throw  std::bad_alloc. If memory cannot be allocated.
       *
       *  Complexity: Worst case complexity is O(N), but that
       *  is hardly ever hit. If and when this particular case is
       *  encountered, the next few cases are guaranteed to have a
       *  worst case complexity of O(1)!  That's why this function
       *  performs very well on average. You can consider this
       *  function to have a complexity referred to commonly as:
       *  Amortized Constant time.
       */
      pointer 
      _M_allocate_single_object() _GLIBCXX_THROW(std::bad_alloc)
      {
	using std::size_t;
#if defined __GTHREADS
	__scoped_lock __bit_lock(_S_mut);
#endif

	// The algorithm is something like this: The last_request
	// variable points to the last accessed Bit Map. When such a
	// condition occurs, we try to find a free block in the
	// current bitmap, or succeeding bitmaps until the last bitmap
	// is reached. If no free block turns up, we resort to First
	// Fit method.

	// WARNING: Do not re-order the condition in the while
	// statement below, because it relies on C++'s short-circuit
	// evaluation. The return from _S_last_request->_M_get() will
	// NOT be dereference able if _S_last_request->_M_finished()
	// returns true. This would inevitably lead to a NULL pointer
	// dereference if tinkered with.
	while (_S_last_request._M_finished() == false
	       && (*(_S_last_request._M_get()) == 0))
	  _S_last_request.operator++();

	if (__builtin_expect(_S_last_request._M_finished() == true, false))
	  {
	    // Fall Back to First Fit algorithm.
	    typedef typename __detail::_Ffit_finder<_Alloc_block*> _FFF;
	    _FFF __fff;
	    _BPiter __bpi = _S_find(__detail::_Functor_Ref<_FFF>(__fff));

	    if (__bpi != _S_mem_blocks.end())
	      {
		// Search was successful. Ok, now mark the first bit from
		// the right as 0, meaning Allocated. This bit is obtained
		// by calling _M_get() on __fff.
		size_t __nz_bit = _Bit_scan_forward(*__fff._M_get());
		__detail::__bit_allocate(__fff._M_get(), __nz_bit);

		_S_last_request._M_reset(__bpi - _S_mem_blocks.begin());

		// Now, get the address of the bit we marked as allocated.
		pointer __ret = reinterpret_cast<pointer>
		  (__bpi->first + __fff._M_offset() + __nz_bit);
		size_t* __puse_count = 
		  reinterpret_cast<size_t*>
		  (__bpi->first) - (__detail::__num_bitmaps(*__bpi) + 1);
		
		++(*__puse_count);
		return __ret;
	      }
	    else
	      {
		// Search was unsuccessful. We Add more memory to the
		// pool by calling _S_refill_pool().
		_S_refill_pool();

		// _M_Reset the _S_last_request structure to the first
		// free block's bit map.
		_S_last_request._M_reset(_S_mem_blocks.size() - 1);

		// Now, mark that bit as allocated.
	      }
	  }

	// _S_last_request holds a pointer to a valid bit map, that
	// points to a free block in memory.
	size_t __nz_bit = _Bit_scan_forward(*_S_last_request._M_get());
	__detail::__bit_allocate(_S_last_request._M_get(), __nz_bit);

	pointer __ret = reinterpret_cast<pointer>
	  (_S_last_request._M_base() + _S_last_request._M_offset() + __nz_bit);

	size_t* __puse_count = reinterpret_cast<size_t*>
	  (_S_mem_blocks[_S_last_request._M_where()].first)
	  - (__detail::
	     __num_bitmaps(_S_mem_blocks[_S_last_request._M_where()]) + 1);

	++(*__puse_count);
	return __ret;
      }

      /** @brief  Deallocates memory that belongs to a single object of
       *  size sizeof(_Tp).
       *
       *  Complexity: O(lg(N)), but the worst case is not hit
       *  often!  This is because containers usually deallocate memory
       *  close to each other and this case is handled in O(1) time by
       *  the deallocate function.
       */
      void 
      _M_deallocate_single_object(pointer __p) throw()
      {
	using std::size_t;
#if defined __GTHREADS
	__scoped_lock __bit_lock(_S_mut);
#endif
	_Alloc_block* __real_p = reinterpret_cast<_Alloc_block*>(__p);

	typedef typename _BPVector::iterator _Iterator;
	typedef typename _BPVector::difference_type _Difference_type;

	_Difference_type __diff;
	long __displacement;

	_GLIBCXX_DEBUG_ASSERT(_S_last_dealloc_index >= 0);

	__detail::_Inclusive_between<_Alloc_block*> __ibt(__real_p);
	if (__ibt(_S_mem_blocks[_S_last_dealloc_index]))
	  {
	    _GLIBCXX_DEBUG_ASSERT(_S_last_dealloc_index
				  <= _S_mem_blocks.size() - 1);

	    // Initial Assumption was correct!
	    __diff = _S_last_dealloc_index;
	    __displacement = __real_p - _S_mem_blocks[__diff].first;
	  }
	else
	  {
	    _Iterator _iter = _S_find(__ibt);

	    _GLIBCXX_DEBUG_ASSERT(_iter != _S_mem_blocks.end());

	    __diff = _iter - _S_mem_blocks.begin();
	    __displacement = __real_p - _S_mem_blocks[__diff].first;
	    _S_last_dealloc_index = __diff;
	  }

	// Get the position of the iterator that has been found.
	const size_t __rotate = (__displacement
				 % size_t(__detail::bits_per_block));
	size_t* __bitmapC = 
	  reinterpret_cast<size_t*>
	  (_S_mem_blocks[__diff].first) - 1;
	__bitmapC -= (__displacement / size_t(__detail::bits_per_block));
      
	__detail::__bit_free(__bitmapC, __rotate);
	size_t* __puse_count = reinterpret_cast<size_t*>
	  (_S_mem_blocks[__diff].first)
	  - (__detail::__num_bitmaps(_S_mem_blocks[__diff]) + 1);
	
	_GLIBCXX_DEBUG_ASSERT(*__puse_count != 0);

	--(*__puse_count);

	if (__builtin_expect(*__puse_count == 0, false))
	  {
	    _S_block_size /= 2;
	  
	    // We can safely remove this block.
	    // _Block_pair __bp = _S_mem_blocks[__diff];
	    this->_M_insert(__puse_count);
	    _S_mem_blocks.erase(_S_mem_blocks.begin() + __diff);

	    // Reset the _S_last_request variable to reflect the
	    // erased block. We do this to protect future requests
	    // after the last block has been removed from a particular
	    // memory Chunk, which in turn has been returned to the
	    // free list, and hence had been erased from the vector,
	    // so the size of the vector gets reduced by 1.
	    if ((_Difference_type)_S_last_request._M_where() >= __diff--)
	      _S_last_request._M_reset(__diff); 

	    // If the Index into the vector of the region of memory
	    // that might hold the next address that will be passed to
	    // deallocated may have been invalidated due to the above
	    // erase procedure being called on the vector, hence we
	    // try to restore this invariant too.
	    if (_S_last_dealloc_index >= _S_mem_blocks.size())
	      {
		_S_last_dealloc_index =(__diff != -1 ? __diff : 0);
		_GLIBCXX_DEBUG_ASSERT(_S_last_dealloc_index >= 0);
	      }
	  }
      }

    public:
      bitmap_allocator() _GLIBCXX_USE_NOEXCEPT
      { }

      bitmap_allocator(const bitmap_allocator&) _GLIBCXX_USE_NOEXCEPT
      { }

      template<typename _Tp1>
        bitmap_allocator(const bitmap_allocator<_Tp1>&) _GLIBCXX_USE_NOEXCEPT
        { }

      ~bitmap_allocator() _GLIBCXX_USE_NOEXCEPT
      { }

      _GLIBCXX_NODISCARD pointer 
      allocate(size_type __n)
      {
	if (__n > this->max_size())
	  std::__throw_bad_alloc();

#if __cpp_aligned_new
	if (alignof(value_type) > __STDCPP_DEFAULT_NEW_ALIGNMENT__)
	  {
	    const size_type __b = __n * sizeof(value_type);
	    std::align_val_t __al = std::align_val_t(alignof(value_type));
	    return static_cast<pointer>(::operator new(__b, __al));
	  }
#endif

	if (__builtin_expect(__n == 1, true))
	  return this->_M_allocate_single_object();
	else
	  { 
	    const size_type __b = __n * sizeof(value_type);
	    return reinterpret_cast<pointer>(::operator new(__b));
	  }
      }

      _GLIBCXX_NODISCARD pointer 
      allocate(size_type __n, typename bitmap_allocator<void>::const_pointer)
      { return allocate(__n); }

      void 
      deallocate(pointer __p, size_type __n) throw()
      {
	if (__builtin_expect(__p != 0, true))
	  {
#if __cpp_aligned_new
	    // Types with extended alignment are handled by operator delete.
	    if (alignof(value_type) > __STDCPP_DEFAULT_NEW_ALIGNMENT__)
	      {
		::operator delete(__p, std::align_val_t(alignof(value_type)));
		return;
	      }
#endif

	    if (__builtin_expect(__n == 1, true))
	      this->_M_deallocate_single_object(__p);
	    else
	      ::operator delete(__p);
	  }
      }

      pointer 
      address(reference __r) const _GLIBCXX_NOEXCEPT
      { return std::__addressof(__r); }

      const_pointer 
      address(const_reference __r) const _GLIBCXX_NOEXCEPT
      { return std::__addressof(__r); }

      size_type 
      max_size() const _GLIBCXX_USE_NOEXCEPT
      { return size_type(-1) / sizeof(value_type); }

#if __cplusplus >= 201103L
      template<typename _Up, typename... _Args>
        void
        construct(_Up* __p, _Args&&... __args)
	{ ::new((void *)__p) _Up(std::forward<_Args>(__args)...); }

      template<typename _Up>
        void 
        destroy(_Up* __p)
        { __p->~_Up(); }
#else
      void 
      construct(pointer __p, const_reference __data)
      { ::new((void *)__p) value_type(__data); }

      void 
      destroy(pointer __p)
      { __p->~value_type(); }
#endif
    };

  template<typename _Tp1, typename _Tp2>
    bool 
    operator==(const bitmap_allocator<_Tp1>&, 
	       const bitmap_allocator<_Tp2>&) throw()
    { return true; }
  
#if __cpp_impl_three_way_comparison < 201907L
  template<typename _Tp1, typename _Tp2>
    bool 
    operator!=(const bitmap_allocator<_Tp1>&, 
	       const bitmap_allocator<_Tp2>&) throw() 
    { return false; }
#endif

  // Static member definitions.
  template<typename _Tp>
    typename bitmap_allocator<_Tp>::_BPVector
    bitmap_allocator<_Tp>::_S_mem_blocks;

  template<typename _Tp>
    std::size_t bitmap_allocator<_Tp>::_S_block_size
      = 2 * std::size_t(__detail::bits_per_block);

  template<typename _Tp>
    typename bitmap_allocator<_Tp>::_BPVector::size_type 
    bitmap_allocator<_Tp>::_S_last_dealloc_index = 0;

  template<typename _Tp>
    __detail::_Bitmap_counter
      <typename bitmap_allocator<_Tp>::_Alloc_block*>
    bitmap_allocator<_Tp>::_S_last_request(_S_mem_blocks);

#if defined __GTHREADS
  template<typename _Tp>
    typename bitmap_allocator<_Tp>::__mutex_type
    bitmap_allocator<_Tp>::_S_mut;
#endif

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace __gnu_cxx

#endif 
