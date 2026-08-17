//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef NASTY_CONTAINERS_H
#define NASTY_CONTAINERS_H

#include <cassert>
#include <cstddef>
#include <vector>
#include <list>
#include <type_traits>

#include "test_macros.h"

template <class T>
class nasty_vector
{
public:
    typedef typename std::vector<T>                           nested_container;
    typedef typename nested_container::value_type             value_type;
    typedef typename nested_container::reference              reference;
    typedef typename nested_container::const_reference        const_reference;
    typedef typename nested_container::iterator               iterator;
    typedef typename nested_container::const_iterator         const_iterator;

    typedef typename nested_container::size_type              size_type;
    typedef typename nested_container::difference_type        difference_type;
    typedef typename nested_container::pointer                pointer;
    typedef typename nested_container::const_pointer          const_pointer;

    typedef typename nested_container::reverse_iterator       reverse_iterator;
    typedef typename nested_container::const_reverse_iterator const_reverse_iterator;

    nasty_vector() : v_() {}
    explicit nasty_vector(size_type n) : v_(n) {}
    nasty_vector(size_type n, const value_type& value) : v_(n, value) {}
    template <class InputIterator> nasty_vector(InputIterator first, InputIterator last) : v_(first, last) {}
#if TEST_STD_VER >= 11
    nasty_vector(std::initializer_list<value_type> il) : v_(il) {}
#endif
    nasty_vector(const nasty_vector&) = default;
    nasty_vector& operator=(const nasty_vector&) = default;
    ~nasty_vector() {}

    template <class InputIterator>
        void assign(InputIterator first, InputIterator last) { v_.assign(first, last); }
    void assign(size_type n, const value_type& u) { v_.assign(n, u); }
#if TEST_STD_VER >= 11
    void assign(std::initializer_list<value_type> il)  { v_.assign(il); }
#endif

    iterator               begin() TEST_NOEXCEPT         { return v_.begin(); }
    const_iterator         begin()   const TEST_NOEXCEPT { return v_.begin(); }
    iterator               end() TEST_NOEXCEPT           { return v_.end(); }
    const_iterator         end()     const TEST_NOEXCEPT { return v_.end(); }

    reverse_iterator       rbegin() TEST_NOEXCEPT        { return v_.rbegin(); }
    const_reverse_iterator rbegin()  const TEST_NOEXCEPT { return v_.rbegin(); }
    reverse_iterator       rend() TEST_NOEXCEPT          { return v_.rend(); }
    const_reverse_iterator rend()    const TEST_NOEXCEPT { return v_.rend(); }

    const_iterator         cbegin()  const TEST_NOEXCEPT { return v_.cbegin(); }
    const_iterator         cend()    const TEST_NOEXCEPT { return v_.cend(); }
    const_reverse_iterator crbegin() const TEST_NOEXCEPT { return v_.crbegin(); }
    const_reverse_iterator crend()   const TEST_NOEXCEPT { return v_.crend(); }

    size_type size() const TEST_NOEXCEPT      { return v_.size(); }
    size_type max_size() const TEST_NOEXCEPT  { return v_.max_size(); }
    size_type capacity() const TEST_NOEXCEPT  { return v_.capacity(); }
    bool empty() const TEST_NOEXCEPT          { return v_.empty(); }
    void reserve(size_type n)             { v_.reserve(n); };
    void shrink_to_fit() TEST_NOEXCEPT        { v_.shrink_to_fit(); }

    reference       operator[](size_type n)       { return v_[n]; }
    const_reference operator[](size_type n) const { return v_[n]; }
    reference       at(size_type n)               { return v_.at(n); }
    const_reference at(size_type n) const         { return v_.at(n); }

    reference       front()       { return v_.front(); }
    const_reference front() const { return v_.front(); }
    reference       back()        { return v_.back(); }
    const_reference back() const  { return v_.back(); }

    value_type*       data() TEST_NOEXCEPT       { return v_.data(); }
    const value_type* data() const TEST_NOEXCEPT { return v_.data(); }

    void push_back(const value_type& x)     { v_.push_back(x); }
#if TEST_STD_VER >= 11
    void push_back(value_type&& x)          { v_.push_back(std::forward<value_type&&>(x)); }
    template <class... Args>
        void emplace_back(Args&&... args)   { v_.emplace_back(std::forward<Args>(args)...); }
#endif
    void pop_back()                         { v_.pop_back(); }

#if TEST_STD_VER >= 11
    template <class... Args> iterator emplace(const_iterator pos, Args&&... args)
    { return v_.emplace(pos, std::forward<Args>(args)...); }
#endif

    iterator insert(const_iterator pos, const value_type& x) { return v_.insert(pos, x); }
#if TEST_STD_VER >= 11
    iterator insert(const_iterator pos, value_type&& x)      { return v_.insert(pos, std::forward<value_type>(x)); }
#endif
    iterator insert(const_iterator pos, size_type n, const value_type& x) { return v_.insert(pos, n, x); }
    template <class InputIterator>
        iterator insert(const_iterator pos, InputIterator first, InputIterator last)
    { return v_.insert(pos, first, last); }

#if TEST_STD_VER >= 11
    iterator insert(const_iterator pos, std::initializer_list<value_type> il) { return v_.insert(pos, il); }
#endif

    iterator erase(const_iterator pos)                        { return v_.erase(pos); }
    iterator erase(const_iterator first, const_iterator last) { return v_.erase(first, last); }

    void clear() TEST_NOEXCEPT { v_.clear(); }

    void resize(size_type sz)                      { v_.resize(sz); }
    void resize(size_type sz, const value_type& c) { v_.resize(sz, c); }

    void swap(nasty_vector& nv)
#if TEST_STD_VER > 14
        noexcept(std::is_nothrow_swappable<nested_container>::value)
#elif defined(_LIBCPP_VERSION)
        TEST_NOEXCEPT_COND(std::__is_nothrow_swappable_v<nested_container>)
#endif
    {
      v_.swap(nv.v_);
    }

    nasty_vector *operator &()             { assert(false); return nullptr; }  // nasty
    const nasty_vector *operator &() const { assert(false); return nullptr; }  // nasty

    nested_container v_;
};

template <class T>
bool operator==(const nasty_vector<T>& x, const nasty_vector<T>& y) { return x.v_ == y.v_; }


#if TEST_STD_VER >= 20

template <class T>
auto operator<=>(const nasty_vector<T>& x, const nasty_vector<T>& y) { return x.v_ <=> y.v_; }

#endif

template <class T>
class nasty_list
{
public:

    typedef typename std::list<T>                             nested_container;
    typedef typename nested_container::value_type             value_type;
    typedef typename nested_container::reference              reference;
    typedef typename nested_container::const_reference        const_reference;
    typedef typename nested_container::iterator               iterator;
    typedef typename nested_container::const_iterator         const_iterator;

    typedef typename nested_container::size_type              size_type;
    typedef typename nested_container::difference_type        difference_type;
    typedef typename nested_container::pointer                pointer;
    typedef typename nested_container::const_pointer          const_pointer;

    typedef typename nested_container::reverse_iterator       reverse_iterator;
    typedef typename nested_container::const_reverse_iterator const_reverse_iterator;

    nasty_list() : l_() {}
    explicit nasty_list(size_type n)  : l_(n) {}
    nasty_list(size_type n, const value_type& value)  : l_(n,value) {}
    template <class Iter>
        nasty_list(Iter first, Iter last)  : l_(first, last) {}
#if TEST_STD_VER >= 11
    nasty_list(std::initializer_list<value_type> il) : l_(il) {}
#endif
    nasty_list(const nasty_list&) = default;
    nasty_list& operator=(const nasty_list&) = default;
    ~nasty_list() {}

#if TEST_STD_VER >= 11
    nasty_list& operator=(std::initializer_list<value_type> il) { l_ = il; return *this; }
#endif
    template <class Iter>
        void assign(Iter first, Iter last) { l_.assign(first, last); }
    void assign(size_type n, const value_type& t) { l_.assign(n, t); }
#if TEST_STD_VER >= 11
    void assign(std::initializer_list<value_type> il) { l_.assign(il); }
#endif


    iterator               begin() TEST_NOEXCEPT         { return l_.begin(); }
    const_iterator         begin()   const TEST_NOEXCEPT { return l_.begin(); }
    iterator               end() TEST_NOEXCEPT           { return l_.end(); }
    const_iterator         end()     const TEST_NOEXCEPT { return l_.end(); }

    reverse_iterator       rbegin() TEST_NOEXCEPT        { return l_.rbegin(); }
    const_reverse_iterator rbegin()  const TEST_NOEXCEPT { return l_.rbegin(); }
    reverse_iterator       rend() TEST_NOEXCEPT          { return l_.rend(); }
    const_reverse_iterator rend()    const TEST_NOEXCEPT { return l_.rend(); }

    const_iterator         cbegin()  const TEST_NOEXCEPT { return l_.cbegin(); }
    const_iterator         cend()    const TEST_NOEXCEPT { return l_.cend(); }
    const_reverse_iterator crbegin() const TEST_NOEXCEPT { return l_.crbegin(); }
    const_reverse_iterator crend()   const TEST_NOEXCEPT { return l_.crend(); }

    reference       front()       { return l_.front(); }
    const_reference front() const { return l_.front(); }
    reference       back()        { return l_.back(); }
    const_reference back() const  { return l_.back(); }

    size_type size() const TEST_NOEXCEPT      { return l_.size(); }
    size_type max_size() const TEST_NOEXCEPT  { return l_.max_size(); }
    bool empty() const TEST_NOEXCEPT          { return l_.empty(); }

    void push_front(const value_type& x)    { l_.push_front(x); }
    void push_back(const value_type& x)     { l_.push_back(x); }
#if TEST_STD_VER >= 11
    void push_back(value_type&& x)          { l_.push_back(std::forward<value_type&&>(x)); }
    void push_front(value_type&& x)         { l_.push_front(std::forward<value_type&&>(x)); }
    template <class... Args>
        void emplace_back(Args&&... args)   { l_.emplace_back(std::forward<Args>(args)...); }
    template <class... Args>
        void emplace_front(Args&&... args)  { l_.emplace_front(std::forward<Args>(args)...); }
#endif
    void pop_front()                        { l_.pop_front(); }
    void pop_back()                         { l_.pop_back(); }

#if TEST_STD_VER >= 11
    template <class... Args> iterator emplace(const_iterator pos, Args&&... args)
    { return l_.emplace(pos, std::forward<Args>(args)...); }
#endif

    iterator insert(const_iterator pos, const value_type& x) { return l_.insert(pos, x); }
#if TEST_STD_VER >= 11
    iterator insert(const_iterator pos, value_type&& x)      { return l_.insert(pos, std::forward<value_type>(x)); }
#endif
    iterator insert(const_iterator pos, size_type n, const value_type& x) { return l_.insert(pos, n, x); }
    template <class InputIterator>
        iterator insert(const_iterator pos, InputIterator first, InputIterator last)
    { return l_.insert(pos, first, last); }

#if TEST_STD_VER >= 11
    iterator insert(const_iterator pos, std::initializer_list<value_type> il) { return l_.insert(pos, il); }
#endif

    iterator erase(const_iterator pos)                      { return l_.erase(pos); }
    iterator erase(const_iterator first, const_iterator last) { return l_.erase(first, last); }

    void resize(size_type n)                      { l_.resize(n); }
    void resize(size_type n, const value_type& c) { l_.resize(n, c); }

    void swap(nasty_list& nl)
#if TEST_STD_VER > 14
        noexcept(std::is_nothrow_swappable<nested_container>::value)
#elif defined(_LIBCPP_VERSION)
        TEST_NOEXCEPT_COND(std::__is_nothrow_swappable_v<nested_container>)
#endif
    {
      l_.swap(nl.l_);
    }

    void clear() TEST_NOEXCEPT { l_.clear(); }

//     void splice(const_iterator position, list& x);
//     void splice(const_iterator position, list&& x);
//     void splice(const_iterator position, list& x, const_iterator i);
//     void splice(const_iterator position, list&& x, const_iterator i);
//     void splice(const_iterator position, list& x, const_iterator first,
//                                                   const_iterator last);
//     void splice(const_iterator position, list&& x, const_iterator first,
//                                                   const_iterator last);
//
//     void remove(const value_type& value);
//     template <class Pred> void remove_if(Pred pred);
//     void unique();
//     template <class BinaryPredicate>
//         void unique(BinaryPredicate binary_pred);
//     void merge(list& x);
//     void merge(list&& x);
//     template <class Compare>
//         void merge(list& x, Compare comp);
//     template <class Compare>
//         void merge(list&& x, Compare comp);
//     void sort();
//     template <class Compare>
//         void sort(Compare comp);
//     void reverse() noexcept;

    nasty_list *operator &()             { assert(false); return nullptr; }  // nasty
    const nasty_list *operator &() const { assert(false); return nullptr; }  // nasty

    nested_container l_;
};

template <class T>
bool operator==(const nasty_list<T>& x, const nasty_list<T>& y) { return x.l_ == y.l_; }

#if TEST_STD_VER >= 20

template <class T>
auto operator<=>(const nasty_list<T>& x, const nasty_list<T>& y) { return x.l_ <=> y.l_; }

#endif

// Not really a mutex, but can play one in tests
class nasty_mutex
{
public:
     nasty_mutex() TEST_NOEXCEPT {}
     ~nasty_mutex() {}

    nasty_mutex *operator& ()   { assert(false); return nullptr; }
    template <typename T>
    void operator, (const T &) { assert(false); }

private:
    nasty_mutex(const nasty_mutex&)            { assert(false); }
    nasty_mutex& operator=(const nasty_mutex&) { assert(false); return *this; }

public:
    void lock()               {}
    bool try_lock() TEST_NOEXCEPT { return true; }
    void unlock() TEST_NOEXCEPT   {}

    // Shared ownership
    void lock_shared()     {}
    bool try_lock_shared() { return true; }
    void unlock_shared()   {}
};

#endif
