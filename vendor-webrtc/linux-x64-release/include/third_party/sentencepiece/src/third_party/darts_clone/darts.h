#ifndef DARTS_H_
#define DARTS_H_

#include <cstdio>
#include <exception>
#include <new>

#if DARTS_DISABLE_EXCEPTIONS
#include "absl/log/absl_check.h"
#endif

#define DARTS_VERSION "0.32"

// DARTS_THROW() throws a <Darts::Exception> whose message starts with the
// file name and the line number. For example, DARTS_THROW("error message") at
// line 123 of "darts.h" throws a <Darts::Exception> which has a pointer to
// "darts.h:123: exception: error message". The message is available by using
// what() as well as that of <std::exception>.
#define DARTS_INT_TO_STR(value) #value
#define DARTS_LINE_TO_STR(line) DARTS_INT_TO_STR(line)
#define DARTS_LINE_STR DARTS_LINE_TO_STR(__LINE__)

#if DARTS_DISABLE_EXCEPTIONS
#define DARTS_THROW(msg) ABSL_CHECK(false) << msg
#else
#define DARTS_THROW(msg)                                      \
  throw Darts::Details::Exception(__FILE__ ":" DARTS_LINE_STR \
                                           ": exception: " msg)
#endif

namespace Darts {

// The following namespace hides the internal types and classes.
namespace Details {

// This header assumes that <int> and <unsigned int> are 32-bit integer types.
//
// Darts-clone keeps values associated with keys. The type of the values is
// <value_type>. Note that the values must be positive integers because the
// most significant bit (MSB) of each value is used to represent whether the
// corresponding unit is a leaf or not. Also, the keys are represented by
// sequences of <char_type>s. <uchar_type> is the unsigned type of <char_type>.
typedef char char_type;
typedef unsigned char uchar_type;
typedef int value_type;

// The main structure of Darts-clone is an array of <DoubleArrayUnit>s, and the
// unit type is actually a wrapper of <id_type>.
typedef unsigned int id_type;

// <progress_func_type> is the type of callback functions for reporting the
// progress of building a dictionary. See also build() of <DoubleArray>.
// The 1st argument receives the progress value and the 2nd argument receives
// the maximum progress value. A usage example is to show the progress
// percentage, 100.0 * (the 1st argument) / (the 2nd argument).
typedef int (*progress_func_type)(std::size_t, std::size_t);

// <DoubleArrayUnit> is the type of double-array units and it is a wrapper of
// <id_type> in practice.
class DoubleArrayUnit {
 public:
  DoubleArrayUnit() : unit_() {}

  // has_leaf() returns whether a leaf unit is immediately derived from the
  // unit (true) or not (false).
  bool has_leaf() const { return ((unit_ >> 8) & 1) == 1; }
  // value() returns the value stored in the unit, and thus value() is
  // available when and only when the unit is a leaf unit.
  value_type value() const {
    return static_cast<value_type>(unit_ & ((1U << 31) - 1));
  }

  // label() returns the label associted with the unit. Note that a leaf unit
  // always returns an invalid label. For this feature, leaf unit's label()
  // returns an <id_type> that has the MSB of 1.
  id_type label() const { return unit_ & ((1U << 31) | 0xFF); }
  // offset() returns the offset from the unit to its derived units.
  id_type offset() const { return (unit_ >> 10) << ((unit_ & (1U << 9)) >> 6); }

 private:
  id_type unit_;

  // Copyable.
};

#if !DARTS_DISABLE_EXCEPTIONS
// Darts-clone throws an <Exception> for memory allocation failure, invalid
// arguments or a too large offset. The last case means that there are too many
// keys in the given set of keys. Note that the `msg' of <Exception> must be a
// constant or static string because an <Exception> keeps only a pointer to
// that string.
class Exception : public std::exception {
 public:
  explicit Exception(const char* msg = NULL) throw() : msg_(msg) {}
  Exception(const Exception& rhs) throw() : msg_(rhs.msg_) {}
  virtual ~Exception() throw() {}

  // <Exception> overrides what() of <std::exception>.
  virtual const char* what() const throw() {
    return (msg_ != NULL) ? msg_ : "";
  }

 private:
  const char* msg_;

  // Disallows operator=.
  Exception& operator=(const Exception&);
};
#endif  // !DARTS_DISABLE_EXCEPTIONS

}  // namespace Details

// <DoubleArrayImpl> is the interface of Darts-clone. Note that other
// classes should not be accessed from outside.
//
// <DoubleArrayImpl> has 4 template arguments but only the 3rd one is used as
// the type of values. Note that the given <T> is used only from outside, and
// the internal value type is not changed from <Darts::Details::value_type>.
// In build(), given values are casted from <T> to <Darts::Details::value_type>
// by using static_cast. On the other hand, values are casted from
// <Darts::Details::value_type> to <T> in searching dictionaries.
template <typename, typename, typename T, typename>
class DoubleArrayImpl {
 public:
  // Even if this <value_type> is changed, the internal value type is still
  // <Darts::Details::value_type>. Other types, such as 64-bit integer types
  // and floating-point number types, should not be used.
  typedef T value_type;
  // A key is reprenseted by a sequence of <key_type>s. For example,
  // exactMatchSearch() takes a <const key_type *>.
  typedef Details::char_type key_type;
  // In searching dictionaries, the values associated with the matched keys are
  // stored into or returned as <result_type>s.
  typedef value_type result_type;

  // <result_pair_type> enables applications to get the lengths of the matched
  // keys in addition to the values.
  struct result_pair_type {
    value_type value;
    std::size_t length;
  };

  // The constructor initializes member variables with 0 and NULLs.
  DoubleArrayImpl() : size_(0), array_(NULL), buf_(NULL) {}
  // The destructor frees memory allocated for units and then initializes
  // member variables with 0 and NULLs.
  virtual ~DoubleArrayImpl() { clear(); }

  // <DoubleArrayImpl> has 2 kinds of set_result()s. The 1st set_result() is to
  // set a value to a <value_type>. The 2nd set_result() is to set a value and
  // a length to a <result_pair_type>. By using set_result()s, search methods
  // can return the 2 kinds of results in the same way.
  // Why the set_result()s are non-static? It is for compatibility.
  //
  // The 1st set_result() takes a length as the 3rd argument but it is not
  // used. If a compiler does a good job, codes for getting the length may be
  // removed.
  void set_result(value_type* result, value_type value, std::size_t) const {
    *result = value;
  }
  // The 2nd set_result() uses both `value' and `length'.
  void set_result(result_pair_type* result,
                  value_type value,
                  std::size_t length) const {
    result->value = value;
    result->length = length;
  }

  // set_array() calls clear() in order to free memory allocated to the old
  // array and then sets a new array. This function is useful to set a memory-
  // mapped array. Note that the array set by set_array() is not freed in
  // clear() and the destructor of <DoubleArrayImpl>.
  // set_array() can also set the size of the new array but the size is not
  // used in search methods. So it works well even if the 2nd argument is 0 or
  // omitted. Remember that size() and total_size() returns 0 in such a case.
  void set_array(const void* ptr, std::size_t size = 0) {
    clear();
    array_ = static_cast<const unit_type*>(ptr);
    size_ = size;
  }
  // array() returns a pointer to the array of units.
  const void* array() const { return array_; }

  // clear() frees memory allocated to units and then initializes member
  // variables with 0 and NULLs. Note that clear() does not free memory if the
  // array of units was set by set_array(). In such a case, `array_' is not
  // NULL and `buf_' is NULL.
  void clear() {
    size_ = 0;
    array_ = NULL;
    if (buf_ != NULL) {
      delete[] buf_;
      buf_ = NULL;
    }
  }

  // unit_size() returns the size of each unit. The size must be 4 bytes.
  std::size_t unit_size() const { return sizeof(unit_type); }
  // size() returns the number of units. It can be 0 if set_array() is used.
  std::size_t size() const { return size_; }
  // total_size() returns the number of bytes allocated to the array of units.
  // It can be 0 if set_array() is used.
  std::size_t total_size() const { return unit_size() * size(); }
  // nonzero_size() exists for compatibility. It always returns the number of
  // units because it takes long time to count the number of non-zero units.
  std::size_t nonzero_size() const { return size(); }

  // build() constructs a dictionary from given key-value pairs. If `lengths'
  // is NULL, `keys' is handled as an array of zero-terminated strings. If
  // `values' is NULL, the index in `keys' is associated with each key, i.e.
  // the ith key has (i - 1) as its value.
  // Note that the key-value pairs must be arranged in key order and the values
  // must not be negative. Also, if there are duplicate keys, only the first
  // pair will be stored in the resultant dictionary.
  // `progress_func' is a pointer to a callback function. If it is not NULL,
  // it will be called in build() so that the caller can check the progress of
  // dictionary construction. For details, please see the definition of
  // <Darts::Details::progress_func_type>.
  // The return value of build() is 0, and it indicates the success of the
  // operation. Otherwise, build() throws a <Darts::Exception>, which is a
  // derived class of <std::exception>.
  // build() uses another construction algorithm if `values' is not NULL. In
  // this case, Darts-clone uses a Directed Acyclic Word Graph (DAWG) instead
  // of a trie because a DAWG is likely to be more compact than a trie.
  int build(std::size_t num_keys,
            const key_type* const* keys,
            const std::size_t* lengths = NULL,
            const value_type* values = NULL,
            Details::progress_func_type progress_func = NULL);

  // open() reads an array of units from the specified file. And if it goes
  // well, the old array will be freed and replaced with the new array read
  // from the file. `offset' specifies the number of bytes to be skipped before
  // reading an array. `size' specifies the number of bytes to be read from the
  // file. If the `size' is 0, the whole file will be read.
  // open() returns 0 iff the operation succeeds. Otherwise, it returns a
  // non-zero value or throws a <Darts::Exception>. The exception is thrown
  // when and only when a memory allocation fails.
  int open(const char* file_name,
           const char* mode = "rb",
           std::size_t offset = 0,
           std::size_t size = 0);
  // save() writes the array of units into the specified file. `offset'
  // specifies the number of bytes to be skipped before writing the array.
  // open() returns 0 iff the operation succeeds. Otherwise, it returns a
  // non-zero value.
  int save(const char* file_name,
           const char* mode = "wb",
           std::size_t offset = 0) const;

  // The 1st exactMatchSearch() tests whether the given key exists or not, and
  // if it exists, its value and length are set to `result'. Otherwise, the
  // value and the length of `result' are set to -1 and 0 respectively.
  // Note that if `length' is 0, `key' is handled as a zero-terminated string.
  // `node_pos' specifies the start position of matching. This argument enables
  // the combination of exactMatchSearch() and traverse(). For example, if you
  // want to test "xyzA", "xyzBC", and "xyzDE", you can use traverse() to get
  // the node position corresponding to "xyz" and then you can use
  // exactMatchSearch() to test "A", "BC", and "DE" from that position.
  // Note that the length of `result' indicates the length from the `node_pos'.
  // In the above example, the lengths are { 1, 2, 2 }, not { 4, 5, 5 }.
  template <class U>
  void exactMatchSearch(const key_type* key,
                        U& result,
                        std::size_t length = 0,
                        std::size_t node_pos = 0) const {
    result = exactMatchSearch<U>(key, length, node_pos);
  }
  // The 2nd exactMatchSearch() returns a result instead of updating the 2nd
  // argument. So, the following exactMatchSearch() has only 3 arguments.
  template <class U>
  inline U exactMatchSearch(const key_type* key,
                            std::size_t length = 0,
                            std::size_t node_pos = 0) const;

  // commonPrefixSearch() searches for keys which match a prefix of the given
  // string. If `length' is 0, `key' is handled as a zero-terminated string.
  // The values and the lengths of at most `max_num_results' matched keys are
  // stored in `results'. commonPrefixSearch() returns the number of matched
  // keys. Note that the return value can be larger than `max_num_results' if
  // there are more than `max_num_results' matches. If you want to get all the
  // results, allocate more spaces and call commonPrefixSearch() again.
  // `node_pos' works as well as in exactMatchSearch().
  template <class U>
  inline std::size_t commonPrefixSearch(const key_type* key,
                                        U* results,
                                        std::size_t max_num_results,
                                        std::size_t length = 0,
                                        std::size_t node_pos = 0) const;

  // In Darts-clone, a dictionary is a deterministic finite-state automaton
  // (DFA) and traverse() tests transitions on the DFA. The initial state is
  // `node_pos' and traverse() chooses transitions labeled key[key_pos],
  // key[key_pos + 1], ... in order. If there is not a transition labeled
  // key[key_pos + i], traverse() terminates the transitions at that state and
  // returns -2. Otherwise, traverse() ends without a termination and returns
  // -1 or a nonnegative value, -1 indicates that the final state was not an
  // accept state. When a nonnegative value is returned, it is the value
  // associated with the final accept state. That is, traverse() returns the
  // value associated with the given key if it exists. Note that traverse()
  // updates `node_pos' and `key_pos' after each transition.
  inline value_type traverse(const key_type* key,
                             std::size_t& node_pos,
                             std::size_t& key_pos,
                             std::size_t length = 0) const;

 private:
  typedef Details::uchar_type uchar_type;
  typedef Details::id_type id_type;
  typedef Details::DoubleArrayUnit unit_type;

  std::size_t size_;
  const unit_type* array_;
  unit_type* buf_;

  // Disallows copy and assignment.
  DoubleArrayImpl(const DoubleArrayImpl&);
  DoubleArrayImpl& operator=(const DoubleArrayImpl&);
};

// <DoubleArray> is the typical instance of <DoubleArrayImpl>. It uses <int>
// as the type of values and it is suitable for most cases.
typedef DoubleArrayImpl<void, void, int, void> DoubleArray;

// The interface section ends here. For using Darts-clone, there is no need
// to read the remaining section, which gives the implementation of
// Darts-clone.

//
// Member functions of DoubleArrayImpl (except build()).
//

template <typename A, typename B, typename T, typename C>
int DoubleArrayImpl<A, B, T, C>::open(const char* file_name,
                                      const char* mode,
                                      std::size_t offset,
                                      std::size_t size) {
#ifdef _MSC_VER
  std::FILE* file;
  if (::fopen_s(&file, file_name, mode) != 0) {
    return -1;
  }
#else
  std::FILE* file = std::fopen(file_name, mode);
  if (file == NULL) {
    return -1;
  }
#endif

  if (size == 0) {
    if (std::fseek(file, 0, SEEK_END) != 0) {
      std::fclose(file);
      return -1;
    }
    size = std::ftell(file) - offset;
  }

  size /= unit_size();
  if (size < 256 || (size & 0xFF) != 0) {
    std::fclose(file);
    return -1;
  }

  if (std::fseek(file, offset, SEEK_SET) != 0) {
    std::fclose(file);
    return -1;
  }

  unit_type units[256];
  if (std::fread(units, unit_size(), 256, file) != 256) {
    std::fclose(file);
    return -1;
  }

  if (units[0].label() != '\0' || units[0].has_leaf() ||
      units[0].offset() == 0 || units[0].offset() >= 512) {
    std::fclose(file);
    return -1;
  }
  for (id_type i = 1; i < 256; ++i) {
    if (units[i].label() <= 0xFF && units[i].offset() >= size) {
      std::fclose(file);
      return -1;
    }
  }

  unit_type* buf = new unit_type[size];
  for (id_type i = 0; i < 256; ++i) {
    buf[i] = units[i];
  }

  if (size > 256) {
    if (std::fread(buf + 256, unit_size(), size - 256, file) != size - 256) {
      std::fclose(file);
      delete[] buf;
      return -1;
    }
  }
  std::fclose(file);

  clear();

  size_ = size;
  array_ = buf;
  buf_ = buf;
  return 0;
}

template <typename A, typename B, typename T, typename C>
int DoubleArrayImpl<A, B, T, C>::save(const char* file_name,
                                      const char* mode,
                                      std::size_t) const {
  if (size() == 0) {
    return -1;
  }

#ifdef _MSC_VER
  std::FILE* file;
  if (::fopen_s(&file, file_name, mode) != 0) {
    return -1;
  }
#else
  std::FILE* file = std::fopen(file_name, mode);
  if (file == NULL) {
    return -1;
  }
#endif

  if (std::fwrite(array_, unit_size(), size(), file) != size()) {
    std::fclose(file);
    return -1;
  }
  std::fclose(file);
  return 0;
}

template <typename A, typename B, typename T, typename C>
template <typename U>
inline U DoubleArrayImpl<A, B, T, C>::exactMatchSearch(
    const key_type* key,
    std::size_t length,
    std::size_t node_pos) const {
  U result;
  set_result(&result, static_cast<value_type>(-1), 0);

  unit_type unit = array_[node_pos];
  if (length != 0) {
    for (std::size_t i = 0; i < length; ++i) {
      node_pos ^= unit.offset() ^ static_cast<uchar_type>(key[i]);
      unit = array_[node_pos];
      if (unit.label() != static_cast<uchar_type>(key[i])) {
        return result;
      }
    }
  } else {
    for (; key[length] != '\0'; ++length) {
      node_pos ^= unit.offset() ^ static_cast<uchar_type>(key[length]);
      unit = array_[node_pos];
      if (unit.label() != static_cast<uchar_type>(key[length])) {
        return result;
      }
    }
  }

  if (!unit.has_leaf()) {
    return result;
  }
  unit = array_[node_pos ^ unit.offset()];
  set_result(&result, static_cast<value_type>(unit.value()), length);
  return result;
}

template <typename A, typename B, typename T, typename C>
template <typename U>
inline std::size_t DoubleArrayImpl<A, B, T, C>::commonPrefixSearch(
    const key_type* key,
    U* results,
    std::size_t max_num_results,
    std::size_t length,
    std::size_t node_pos) const {
  std::size_t num_results = 0;

  unit_type unit = array_[node_pos];
  node_pos ^= unit.offset();
  if (length != 0) {
    for (std::size_t i = 0; i < length; ++i) {
      node_pos ^= static_cast<uchar_type>(key[i]);
      unit = array_[node_pos];
      if (unit.label() != static_cast<uchar_type>(key[i])) {
        return num_results;
      }

      node_pos ^= unit.offset();
      if (unit.has_leaf()) {
        if (num_results < max_num_results) {
          set_result(&results[num_results],
                     static_cast<value_type>(array_[node_pos].value()), i + 1);
        }
        ++num_results;
      }
    }
  } else {
    for (; key[length] != '\0'; ++length) {
      node_pos ^= static_cast<uchar_type>(key[length]);
      unit = array_[node_pos];
      if (unit.label() != static_cast<uchar_type>(key[length])) {
        return num_results;
      }

      node_pos ^= unit.offset();
      if (unit.has_leaf()) {
        if (num_results < max_num_results) {
          set_result(&results[num_results],
                     static_cast<value_type>(array_[node_pos].value()),
                     length + 1);
        }
        ++num_results;
      }
    }
  }

  return num_results;
}

template <typename A, typename B, typename T, typename C>
inline typename DoubleArrayImpl<A, B, T, C>::value_type
DoubleArrayImpl<A, B, T, C>::traverse(const key_type* key,
                                      std::size_t& node_pos,
                                      std::size_t& key_pos,
                                      std::size_t length) const {
  id_type id = static_cast<id_type>(node_pos);
  unit_type unit = array_[id];

  if (length != 0) {
    for (; key_pos < length; ++key_pos) {
      id ^= unit.offset() ^ static_cast<uchar_type>(key[key_pos]);
      unit = array_[id];
      if (unit.label() != static_cast<uchar_type>(key[key_pos])) {
        return static_cast<value_type>(-2);
      }
      node_pos = id;
    }
  } else {
    for (; key[key_pos] != '\0'; ++key_pos) {
      id ^= unit.offset() ^ static_cast<uchar_type>(key[key_pos]);
      unit = array_[id];
      if (unit.label() != static_cast<uchar_type>(key[key_pos])) {
        return static_cast<value_type>(-2);
      }
      node_pos = id;
    }
  }

  if (!unit.has_leaf()) {
    return static_cast<value_type>(-1);
  }
  unit = array_[id ^ unit.offset()];
  return static_cast<value_type>(unit.value());
}

namespace Details {

//
// Memory management of array.
//

template <typename T>
class AutoArray {
 public:
  explicit AutoArray(T* array = NULL) : array_(array) {}
  ~AutoArray() { clear(); }

  const T& operator[](std::size_t id) const { return array_[id]; }
  T& operator[](std::size_t id) { return array_[id]; }

  bool empty() const { return array_ == NULL; }

  void clear() {
    if (array_ != NULL) {
      delete[] array_;
      array_ = NULL;
    }
  }
  void swap(AutoArray* array) {
    T* temp = array_;
    array_ = array->array_;
    array->array_ = temp;
  }
  void reset(T* array = NULL) { AutoArray(array).swap(this); }

 private:
  T* array_;

  // Disallows copy and assignment.
  AutoArray(const AutoArray&);
  AutoArray& operator=(const AutoArray&);
};

//
// Memory management of resizable array.
//

template <typename T>
class AutoPool {
 public:
  AutoPool() : buf_(), size_(0), capacity_(0) {}
  ~AutoPool() { clear(); }

  const T& operator[](std::size_t id) const {
    return *(reinterpret_cast<const T*>(&buf_[0]) + id);
  }
  T& operator[](std::size_t id) {
    return *(reinterpret_cast<T*>(&buf_[0]) + id);
  }

  bool empty() const { return size_ == 0; }
  std::size_t size() const { return size_; }

  void clear() {
    resize(0);
    buf_.clear();
    size_ = 0;
    capacity_ = 0;
  }

  void push_back(const T& value) { append(value); }
  void pop_back() { (*this)[--size_].~T(); }

  void append() {
    if (size_ == capacity_) {
      resize_buf(size_ + 1);
    }
    new (&(*this)[size_++]) T;
  }
  void append(const T& value) {
    if (size_ == capacity_) {
      resize_buf(size_ + 1);
    }
    new (&(*this)[size_++]) T(value);
  }

  void resize(std::size_t size) {
    while (size_ > size) {
      (*this)[--size_].~T();
    }
    if (size > capacity_) {
      resize_buf(size);
    }
    while (size_ < size) {
      new (&(*this)[size_++]) T;
    }
  }
  void resize(std::size_t size, const T& value) {
    while (size_ > size) {
      (*this)[--size_].~T();
    }
    if (size > capacity_) {
      resize_buf(size);
    }
    while (size_ < size) {
      new (&(*this)[size_++]) T(value);
    }
  }

  void reserve(std::size_t size) {
    if (size > capacity_) {
      resize_buf(size);
    }
  }

 private:
  AutoArray<char> buf_;
  std::size_t size_;
  std::size_t capacity_;

  // Disallows copy and assignment.
  AutoPool(const AutoPool&);
  AutoPool& operator=(const AutoPool&);

  void resize_buf(std::size_t size);
};

template <typename T>
void AutoPool<T>::resize_buf(std::size_t size) {
  std::size_t capacity;
  if (size >= capacity_ * 2) {
    capacity = size;
  } else {
    capacity = 1;
    while (capacity < size) {
      capacity <<= 1;
    }
  }

  AutoArray<char> buf;
  buf.reset(new char[sizeof(T) * capacity]);

  if (size_ > 0) {
    T* src = reinterpret_cast<T*>(&buf_[0]);
    T* dest = reinterpret_cast<T*>(&buf[0]);
    for (std::size_t i = 0; i < size_; ++i) {
      new (&dest[i]) T(src[i]);
      src[i].~T();
    }
  }

  buf_.swap(&buf);
  capacity_ = capacity;
}

//
// Memory management of stack.
//

template <typename T>
class AutoStack {
 public:
  AutoStack() : pool_() {}
  ~AutoStack() { clear(); }

  const T& top() const { return pool_[size() - 1]; }
  T& top() { return pool_[size() - 1]; }

  bool empty() const { return pool_.empty(); }
  std::size_t size() const { return pool_.size(); }

  void push(const T& value) { pool_.push_back(value); }
  void pop() { pool_.pop_back(); }

  void clear() { pool_.clear(); }

 private:
  AutoPool<T> pool_;

  // Disallows copy and assignment.
  AutoStack(const AutoStack&);
  AutoStack& operator=(const AutoStack&);
};

//
// Succinct bit vector.
//

class BitVector {
 public:
  BitVector() : units_(), ranks_(), num_ones_(0), size_(0) {}
  ~BitVector() { clear(); }

  bool operator[](std::size_t id) const {
    return (units_[id / UNIT_SIZE] >> (id % UNIT_SIZE) & 1) == 1;
  }

  id_type rank(std::size_t id) const {
    std::size_t unit_id = id / UNIT_SIZE;
    return ranks_[unit_id] +
           pop_count(units_[unit_id] &
                     (~0U >> (UNIT_SIZE - (id % UNIT_SIZE) - 1)));
  }

  void set(std::size_t id, bool bit) {
    if (bit) {
      units_[id / UNIT_SIZE] |= 1U << (id % UNIT_SIZE);
    } else {
      units_[id / UNIT_SIZE] &= ~(1U << (id % UNIT_SIZE));
    }
  }

  bool empty() const { return units_.empty(); }
  std::size_t num_ones() const { return num_ones_; }
  std::size_t size() const { return size_; }

  void append() {
    if ((size_ % UNIT_SIZE) == 0) {
      units_.append(0);
    }
    ++size_;
  }
  void build();

  void clear() {
    units_.clear();
    ranks_.clear();
  }

 private:
  enum { UNIT_SIZE = sizeof(id_type) * 8 };

  AutoPool<id_type> units_;
  AutoArray<id_type> ranks_;
  std::size_t num_ones_;
  std::size_t size_;

  // Disallows copy and assignment.
  BitVector(const BitVector&);
  BitVector& operator=(const BitVector&);

  static id_type pop_count(id_type unit) {
    unit = ((unit & 0xAAAAAAAA) >> 1) + (unit & 0x55555555);
    unit = ((unit & 0xCCCCCCCC) >> 2) + (unit & 0x33333333);
    unit = ((unit >> 4) + unit) & 0x0F0F0F0F;
    unit += unit >> 8;
    unit += unit >> 16;
    return unit & 0xFF;
  }
};

inline void BitVector::build() {
  ranks_.reset(new id_type[units_.size()]);

  num_ones_ = 0;
  for (std::size_t i = 0; i < units_.size(); ++i) {
    ranks_[i] = num_ones_;
    num_ones_ += pop_count(units_[i]);
  }
}

//
// Keyset.
//

template <typename T>
class Keyset {
 public:
  Keyset(std::size_t num_keys,
         const char_type* const* keys,
         const std::size_t* lengths,
         const T* values)
      : num_keys_(num_keys), keys_(keys), lengths_(lengths), values_(values) {}

  std::size_t num_keys() const { return num_keys_; }
  const char_type* keys(std::size_t id) const { return keys_[id]; }
  uchar_type keys(std::size_t key_id, std::size_t char_id) const {
    if (has_lengths() && char_id >= lengths_[key_id]) {
      return '\0';
    }
    return keys_[key_id][char_id];
  }

  bool has_lengths() const { return lengths_ != NULL; }
  std::size_t lengths(std::size_t id) const {
    if (has_lengths()) {
      return lengths_[id];
    }
    std::size_t length = 0;
    while (keys_[id][length] != '\0') {
      ++length;
    }
    return length;
  }

  bool has_values() const { return values_ != NULL; }
  const value_type values(std::size_t id) const {
    if (has_values()) {
      return static_cast<value_type>(values_[id]);
    }
    return static_cast<value_type>(id);
  }

 private:
  std::size_t num_keys_;
  const char_type* const* keys_;
  const std::size_t* lengths_;
  const T* values_;

  // Disallows copy and assignment.
  Keyset(const Keyset&);
  Keyset& operator=(const Keyset&);
};

//
// Node of Directed Acyclic Word Graph (DAWG).
//

class DawgNode {
 public:
  DawgNode()
      : child_(0),
        sibling_(0),
        label_('\0'),
        is_state_(false),
        has_sibling_(false) {}

  void set_child(id_type child) { child_ = child; }
  void set_sibling(id_type sibling) { sibling_ = sibling; }
  void set_value(value_type value) { child_ = value; }
  void set_label(uchar_type label) { label_ = label; }
  void set_is_state(bool is_state) { is_state_ = is_state; }
  void set_has_sibling(bool has_sibling) { has_sibling_ = has_sibling; }

  id_type child() const { return child_; }
  id_type sibling() const { return sibling_; }
  value_type value() const { return static_cast<value_type>(child_); }
  uchar_type label() const { return label_; }
  bool is_state() const { return is_state_; }
  bool has_sibling() const { return has_sibling_; }

  id_type unit() const {
    if (label_ == '\0') {
      return (child_ << 1) | (has_sibling_ ? 1 : 0);
    }
    return (child_ << 2) | (is_state_ ? 2 : 0) | (has_sibling_ ? 1 : 0);
  }

 private:
  id_type child_;
  id_type sibling_;
  uchar_type label_;
  bool is_state_;
  bool has_sibling_;

  // Copyable.
};

//
// Fixed unit of Directed Acyclic Word Graph (DAWG).
//

class DawgUnit {
 public:
  explicit DawgUnit(id_type unit = 0) : unit_(unit) {}
  DawgUnit(const DawgUnit& unit) : unit_(unit.unit_) {}

  DawgUnit& operator=(id_type unit) {
    unit_ = unit;
    return *this;
  }

  id_type unit() const { return unit_; }

  id_type child() const { return unit_ >> 2; }
  bool has_sibling() const { return (unit_ & 1) == 1; }
  value_type value() const { return static_cast<value_type>(unit_ >> 1); }
  bool is_state() const { return (unit_ & 2) == 2; }

 private:
  id_type unit_;

  // Copyable.
};

//
// Directed Acyclic Word Graph (DAWG) builder.
//

class DawgBuilder {
 public:
  DawgBuilder()
      : nodes_(),
        units_(),
        labels_(),
        is_intersections_(),
        table_(),
        node_stack_(),
        recycle_bin_(),
        num_states_(0) {}
  ~DawgBuilder() { clear(); }

  id_type root() const { return 0; }

  id_type child(id_type id) const { return units_[id].child(); }
  id_type sibling(id_type id) const {
    return units_[id].has_sibling() ? (id + 1) : 0;
  }
  int value(id_type id) const { return units_[id].value(); }

  bool is_leaf(id_type id) const { return label(id) == '\0'; }
  uchar_type label(id_type id) const { return labels_[id]; }

  bool is_intersection(id_type id) const { return is_intersections_[id]; }
  id_type intersection_id(id_type id) const {
    return is_intersections_.rank(id) - 1;
  }

  std::size_t num_intersections() const { return is_intersections_.num_ones(); }

  std::size_t size() const { return units_.size(); }

  void init();
  void finish();

  void insert(const char* key, std::size_t length, value_type value);

  void clear();

 private:
  enum { INITIAL_TABLE_SIZE = 1 << 10 };

  AutoPool<DawgNode> nodes_;
  AutoPool<DawgUnit> units_;
  AutoPool<uchar_type> labels_;
  BitVector is_intersections_;
  AutoPool<id_type> table_;
  AutoStack<id_type> node_stack_;
  AutoStack<id_type> recycle_bin_;
  std::size_t num_states_;

  // Disallows copy and assignment.
  DawgBuilder(const DawgBuilder&);
  DawgBuilder& operator=(const DawgBuilder&);

  void flush(id_type id);

  void expand_table();

  id_type find_unit(id_type id, id_type* hash_id) const;
  id_type find_node(id_type node_id, id_type* hash_id) const;

  bool are_equal(id_type node_id, id_type unit_id) const;

  id_type hash_unit(id_type id) const;
  id_type hash_node(id_type id) const;

  id_type append_node();
  id_type append_unit();

  void free_node(id_type id) { recycle_bin_.push(id); }

  static id_type hash(id_type key) {
    key = ~key + (key << 15);  // key = (key << 15) - key - 1;
    key = key ^ (key >> 12);
    key = key + (key << 2);
    key = key ^ (key >> 4);
    key = key * 2057;  // key = (key + (key << 3)) + (key << 11);
    key = key ^ (key >> 16);
    return key;
  }
};

inline void DawgBuilder::init() {
  table_.resize(INITIAL_TABLE_SIZE, 0);

  append_node();
  append_unit();

  num_states_ = 1;

  nodes_[0].set_label(0xFF);
  node_stack_.push(0);
}

inline void DawgBuilder::finish() {
  flush(0);

  units_[0] = nodes_[0].unit();
  labels_[0] = nodes_[0].label();

  nodes_.clear();
  table_.clear();
  node_stack_.clear();
  recycle_bin_.clear();

  is_intersections_.build();
}

inline void DawgBuilder::insert(const char* key,
                                std::size_t length,
                                value_type value) {
  if (value < 0) {
    DARTS_THROW("failed to insert key: negative value");
  } else if (length == 0) {
    DARTS_THROW("failed to insert key: zero-length key");
  }

  id_type id = 0;
  std::size_t key_pos = 0;

  for (; key_pos <= length; ++key_pos) {
    id_type child_id = nodes_[id].child();
    if (child_id == 0) {
      break;
    }

    uchar_type key_label = static_cast<uchar_type>(key[key_pos]);
    if (key_pos < length && key_label == '\0') {
      DARTS_THROW("failed to insert key: invalid null character");
    }

    uchar_type unit_label = nodes_[child_id].label();
    if (key_label < unit_label) {
      DARTS_THROW("failed to insert key: wrong key order");
    } else if (key_label > unit_label) {
      nodes_[child_id].set_has_sibling(true);
      flush(child_id);
      break;
    }
    id = child_id;
  }

  if (key_pos > length) {
    return;
  }

  for (; key_pos <= length; ++key_pos) {
    uchar_type key_label =
        static_cast<uchar_type>((key_pos < length) ? key[key_pos] : '\0');
    id_type child_id = append_node();

    if (nodes_[id].child() == 0) {
      nodes_[child_id].set_is_state(true);
    }
    nodes_[child_id].set_sibling(nodes_[id].child());
    nodes_[child_id].set_label(key_label);
    nodes_[id].set_child(child_id);
    node_stack_.push(child_id);

    id = child_id;
  }
  nodes_[id].set_value(value);
}

inline void DawgBuilder::clear() {
  nodes_.clear();
  units_.clear();
  labels_.clear();
  is_intersections_.clear();
  table_.clear();
  node_stack_.clear();
  recycle_bin_.clear();
  num_states_ = 0;
}

inline void DawgBuilder::flush(id_type id) {
  while (node_stack_.top() != id) {
    id_type node_id = node_stack_.top();
    node_stack_.pop();

    if (num_states_ >= table_.size() - (table_.size() >> 2)) {
      expand_table();
    }

    id_type num_siblings = 0;
    for (id_type i = node_id; i != 0; i = nodes_[i].sibling()) {
      ++num_siblings;
    }

    id_type hash_id;
    id_type match_id = find_node(node_id, &hash_id);
    if (match_id != 0) {
      is_intersections_.set(match_id, true);
    } else {
      id_type unit_id = 0;
      for (id_type i = 0; i < num_siblings; ++i) {
        unit_id = append_unit();
      }
      for (id_type i = node_id; i != 0; i = nodes_[i].sibling()) {
        units_[unit_id] = nodes_[i].unit();
        labels_[unit_id] = nodes_[i].label();
        --unit_id;
      }
      match_id = unit_id + 1;
      table_[hash_id] = match_id;
      ++num_states_;
    }

    for (id_type i = node_id, next; i != 0; i = next) {
      next = nodes_[i].sibling();
      free_node(i);
    }

    nodes_[node_stack_.top()].set_child(match_id);
  }
  node_stack_.pop();
}

inline void DawgBuilder::expand_table() {
  std::size_t table_size = table_.size() << 1;
  table_.clear();
  table_.resize(table_size, 0);

  for (std::size_t i = 1; i < units_.size(); ++i) {
    id_type id = static_cast<id_type>(i);
    if (labels_[id] == '\0' || units_[id].is_state()) {
      id_type hash_id;
      find_unit(id, &hash_id);
      table_[hash_id] = id;
    }
  }
}

inline id_type DawgBuilder::find_unit(id_type id, id_type* hash_id) const {
  *hash_id = hash_unit(id) % table_.size();
  for (;; *hash_id = (*hash_id + 1) % table_.size()) {
    id_type unit_id = table_[*hash_id];
    if (unit_id == 0) {
      break;
    }

    // There must not be the same unit.
  }
  return 0;
}

inline id_type DawgBuilder::find_node(id_type node_id, id_type* hash_id) const {
  *hash_id = hash_node(node_id) % table_.size();
  for (;; *hash_id = (*hash_id + 1) % table_.size()) {
    id_type unit_id = table_[*hash_id];
    if (unit_id == 0) {
      break;
    }

    if (are_equal(node_id, unit_id)) {
      return unit_id;
    }
  }
  return 0;
}

inline bool DawgBuilder::are_equal(id_type node_id, id_type unit_id) const {
  for (id_type i = nodes_[node_id].sibling(); i != 0; i = nodes_[i].sibling()) {
    if (units_[unit_id].has_sibling() == false) {
      return false;
    }
    ++unit_id;
  }
  if (units_[unit_id].has_sibling() == true) {
    return false;
  }

  for (id_type i = node_id; i != 0; i = nodes_[i].sibling(), --unit_id) {
    if (nodes_[i].unit() != units_[unit_id].unit() ||
        nodes_[i].label() != labels_[unit_id]) {
      return false;
    }
  }
  return true;
}

inline id_type DawgBuilder::hash_unit(id_type id) const {
  id_type hash_value = 0;
  for (; id != 0; ++id) {
    id_type unit = units_[id].unit();
    uchar_type label = labels_[id];
    hash_value ^= hash((label << 24) ^ unit);

    if (units_[id].has_sibling() == false) {
      break;
    }
  }
  return hash_value;
}

inline id_type DawgBuilder::hash_node(id_type id) const {
  id_type hash_value = 0;
  for (; id != 0; id = nodes_[id].sibling()) {
    id_type unit = nodes_[id].unit();
    uchar_type label = nodes_[id].label();
    hash_value ^= hash((label << 24) ^ unit);
  }
  return hash_value;
}

inline id_type DawgBuilder::append_unit() {
  is_intersections_.append();
  units_.append();
  labels_.append();

  return static_cast<id_type>(is_intersections_.size() - 1);
}

inline id_type DawgBuilder::append_node() {
  id_type id;
  if (recycle_bin_.empty()) {
    id = static_cast<id_type>(nodes_.size());
    nodes_.append();
  } else {
    id = recycle_bin_.top();
    nodes_[id] = DawgNode();
    recycle_bin_.pop();
  }
  return id;
}

//
// Unit of double-array builder.
//

class DoubleArrayBuilderUnit {
 public:
  DoubleArrayBuilderUnit() : unit_(0) {}

  void set_has_leaf(bool has_leaf) {
    if (has_leaf) {
      unit_ |= 1U << 8;
    } else {
      unit_ &= ~(1U << 8);
    }
  }
  void set_value(value_type value) { unit_ = value | (1U << 31); }
  void set_label(uchar_type label) { unit_ = (unit_ & ~0xFFU) | label; }
  void set_offset(id_type offset) {
    if (offset >= 1U << 29) {
      DARTS_THROW("failed to modify unit: too large offset");
    }
    unit_ &= (1U << 31) | (1U << 8) | 0xFF;
    if (offset < 1U << 21) {
      unit_ |= (offset << 10);
    } else {
      unit_ |= (offset << 2) | (1U << 9);
    }
  }

 private:
  id_type unit_;

  // Copyable.
};

//
// Extra unit of double-array builder.
//

class DoubleArrayBuilderExtraUnit {
 public:
  DoubleArrayBuilderExtraUnit()
      : prev_(0), next_(0), is_fixed_(false), is_used_(false) {}

  void set_prev(id_type prev) { prev_ = prev; }
  void set_next(id_type next) { next_ = next; }
  void set_is_fixed(bool is_fixed) { is_fixed_ = is_fixed; }
  void set_is_used(bool is_used) { is_used_ = is_used; }

  id_type prev() const { return prev_; }
  id_type next() const { return next_; }
  bool is_fixed() const { return is_fixed_; }
  bool is_used() const { return is_used_; }

 private:
  id_type prev_;
  id_type next_;
  bool is_fixed_;
  bool is_used_;

  // Copyable.
};

//
// DAWG -> double-array converter.
//

class DoubleArrayBuilder {
 public:
  explicit DoubleArrayBuilder(progress_func_type progress_func)
      : progress_func_(progress_func),
        units_(),
        extras_(),
        labels_(),
        table_(),
        extras_head_(0) {}
  ~DoubleArrayBuilder() { clear(); }

  template <typename T>
  void build(const Keyset<T>& keyset);
  void copy(std::size_t* size_ptr, DoubleArrayUnit** buf_ptr) const;

  void clear();

 private:
  enum { BLOCK_SIZE = 256 };
  enum { NUM_EXTRA_BLOCKS = 16 };
  enum { NUM_EXTRAS = BLOCK_SIZE * NUM_EXTRA_BLOCKS };

  enum { UPPER_MASK = 0xFF << 21 };
  enum { LOWER_MASK = 0xFF };

  typedef DoubleArrayBuilderUnit unit_type;
  typedef DoubleArrayBuilderExtraUnit extra_type;

  progress_func_type progress_func_;
  AutoPool<unit_type> units_;
  AutoArray<extra_type> extras_;
  AutoPool<uchar_type> labels_;
  AutoArray<id_type> table_;
  id_type extras_head_;

  // Disallows copy and assignment.
  DoubleArrayBuilder(const DoubleArrayBuilder&);
  DoubleArrayBuilder& operator=(const DoubleArrayBuilder&);

  std::size_t num_blocks() const { return units_.size() / BLOCK_SIZE; }

  const extra_type& extras(id_type id) const {
    return extras_[id % NUM_EXTRAS];
  }
  extra_type& extras(id_type id) { return extras_[id % NUM_EXTRAS]; }

  template <typename T>
  void build_dawg(const Keyset<T>& keyset, DawgBuilder* dawg_builder);
  void build_from_dawg(const DawgBuilder& dawg);
  void build_from_dawg(const DawgBuilder& dawg,
                       id_type dawg_id,
                       id_type dic_id);
  id_type arrange_from_dawg(const DawgBuilder& dawg,
                            id_type dawg_id,
                            id_type dic_id);

  template <typename T>
  void build_from_keyset(const Keyset<T>& keyset);
  template <typename T>
  void build_from_keyset(const Keyset<T>& keyset,
                         std::size_t begin,
                         std::size_t end,
                         std::size_t depth,
                         id_type dic_id);
  template <typename T>
  id_type arrange_from_keyset(const Keyset<T>& keyset,
                              std::size_t begin,
                              std::size_t end,
                              std::size_t depth,
                              id_type dic_id);

  id_type find_valid_offset(id_type id) const;
  bool is_valid_offset(id_type id, id_type offset) const;

  void reserve_id(id_type id);
  void expand_units();

  void fix_all_blocks();
  void fix_block(id_type block_id);
};

template <typename T>
void DoubleArrayBuilder::build(const Keyset<T>& keyset) {
  if (keyset.has_values()) {
    Details::DawgBuilder dawg_builder;
    build_dawg(keyset, &dawg_builder);
    build_from_dawg(dawg_builder);
    dawg_builder.clear();
  } else {
    build_from_keyset(keyset);
  }
}

inline void DoubleArrayBuilder::copy(std::size_t* size_ptr,
                                     DoubleArrayUnit** buf_ptr) const {
  if (size_ptr != NULL) {
    *size_ptr = units_.size();
  }
  if (buf_ptr != NULL) {
    *buf_ptr = new DoubleArrayUnit[units_.size()];
    unit_type* units = reinterpret_cast<unit_type*>(*buf_ptr);
    for (std::size_t i = 0; i < units_.size(); ++i) {
      units[i] = units_[i];
    }
  }
}

inline void DoubleArrayBuilder::clear() {
  units_.clear();
  extras_.clear();
  labels_.clear();
  table_.clear();
  extras_head_ = 0;
}

template <typename T>
void DoubleArrayBuilder::build_dawg(const Keyset<T>& keyset,
                                    DawgBuilder* dawg_builder) {
  dawg_builder->init();
  for (std::size_t i = 0; i < keyset.num_keys(); ++i) {
    dawg_builder->insert(keyset.keys(i), keyset.lengths(i), keyset.values(i));
    if (progress_func_ != NULL) {
      progress_func_(i + 1, keyset.num_keys() + 1);
    }
  }
  dawg_builder->finish();
}

inline void DoubleArrayBuilder::build_from_dawg(const DawgBuilder& dawg) {
  std::size_t num_units = 1;
  while (num_units < dawg.size()) {
    num_units <<= 1;
  }
  units_.reserve(num_units);

  table_.reset(new id_type[dawg.num_intersections()]);
  for (std::size_t i = 0; i < dawg.num_intersections(); ++i) {
    table_[i] = 0;
  }

  extras_.reset(new extra_type[NUM_EXTRAS]);

  reserve_id(0);
  extras(0).set_is_used(true);
  units_[0].set_offset(1);
  units_[0].set_label('\0');

  if (dawg.child(dawg.root()) != 0) {
    build_from_dawg(dawg, dawg.root(), 0);
  }

  fix_all_blocks();

  extras_.clear();
  labels_.clear();
  table_.clear();
}

inline void DoubleArrayBuilder::build_from_dawg(const DawgBuilder& dawg,
                                                id_type dawg_id,
                                                id_type dic_id) {
  id_type dawg_child_id = dawg.child(dawg_id);
  if (dawg.is_intersection(dawg_child_id)) {
    id_type intersection_id = dawg.intersection_id(dawg_child_id);
    id_type offset = table_[intersection_id];
    if (offset != 0) {
      offset ^= dic_id;
      if (!(offset & UPPER_MASK) || !(offset & LOWER_MASK)) {
        if (dawg.is_leaf(dawg_child_id)) {
          units_[dic_id].set_has_leaf(true);
        }
        units_[dic_id].set_offset(offset);
        return;
      }
    }
  }

  id_type offset = arrange_from_dawg(dawg, dawg_id, dic_id);
  if (dawg.is_intersection(dawg_child_id)) {
    table_[dawg.intersection_id(dawg_child_id)] = offset;
  }

  do {
    uchar_type child_label = dawg.label(dawg_child_id);
    id_type dic_child_id = offset ^ child_label;
    if (child_label != '\0') {
      build_from_dawg(dawg, dawg_child_id, dic_child_id);
    }
    dawg_child_id = dawg.sibling(dawg_child_id);
  } while (dawg_child_id != 0);
}

inline id_type DoubleArrayBuilder::arrange_from_dawg(const DawgBuilder& dawg,
                                                     id_type dawg_id,
                                                     id_type dic_id) {
  labels_.resize(0);

  id_type dawg_child_id = dawg.child(dawg_id);
  while (dawg_child_id != 0) {
    labels_.append(dawg.label(dawg_child_id));
    dawg_child_id = dawg.sibling(dawg_child_id);
  }

  id_type offset = find_valid_offset(dic_id);
  units_[dic_id].set_offset(dic_id ^ offset);

  dawg_child_id = dawg.child(dawg_id);
  for (std::size_t i = 0; i < labels_.size(); ++i) {
    id_type dic_child_id = offset ^ labels_[i];
    reserve_id(dic_child_id);

    if (dawg.is_leaf(dawg_child_id)) {
      units_[dic_id].set_has_leaf(true);
      units_[dic_child_id].set_value(dawg.value(dawg_child_id));
    } else {
      units_[dic_child_id].set_label(labels_[i]);
    }

    dawg_child_id = dawg.sibling(dawg_child_id);
  }
  extras(offset).set_is_used(true);

  return offset;
}

template <typename T>
void DoubleArrayBuilder::build_from_keyset(const Keyset<T>& keyset) {
  std::size_t num_units = 1;
  while (num_units < keyset.num_keys()) {
    num_units <<= 1;
  }
  units_.reserve(num_units);

  extras_.reset(new extra_type[NUM_EXTRAS]);

  reserve_id(0);
  extras(0).set_is_used(true);
  units_[0].set_offset(1);
  units_[0].set_label('\0');

  if (keyset.num_keys() > 0) {
    build_from_keyset(keyset, 0, keyset.num_keys(), 0, 0);
  }

  fix_all_blocks();

  extras_.clear();
  labels_.clear();
}

template <typename T>
void DoubleArrayBuilder::build_from_keyset(const Keyset<T>& keyset,
                                           std::size_t begin,
                                           std::size_t end,
                                           std::size_t depth,
                                           id_type dic_id) {
  id_type offset = arrange_from_keyset(keyset, begin, end, depth, dic_id);

  while (begin < end) {
    if (keyset.keys(begin, depth) != '\0') {
      break;
    }
    ++begin;
  }
  if (begin == end) {
    return;
  }

  std::size_t last_begin = begin;
  uchar_type last_label = keyset.keys(begin, depth);
  while (++begin < end) {
    uchar_type label = keyset.keys(begin, depth);
    if (label != last_label) {
      build_from_keyset(keyset, last_begin, begin, depth + 1,
                        offset ^ last_label);
      last_begin = begin;
      last_label = keyset.keys(begin, depth);
    }
  }
  build_from_keyset(keyset, last_begin, end, depth + 1, offset ^ last_label);
}

template <typename T>
id_type DoubleArrayBuilder::arrange_from_keyset(const Keyset<T>& keyset,
                                                std::size_t begin,
                                                std::size_t end,
                                                std::size_t depth,
                                                id_type dic_id) {
  labels_.resize(0);

  value_type value = -1;
  for (std::size_t i = begin; i < end; ++i) {
    uchar_type label = keyset.keys(i, depth);
    if (label == '\0') {
      if (keyset.has_lengths() && depth < keyset.lengths(i)) {
        DARTS_THROW(
            "failed to build double-array: "
            "invalid null character");
      } else if (keyset.values(i) < 0) {
        DARTS_THROW("failed to build double-array: negative value");
      }

      if (value == -1) {
        value = keyset.values(i);
      }
      if (progress_func_ != NULL) {
        progress_func_(i + 1, keyset.num_keys() + 1);
      }
    }

    if (labels_.empty()) {
      labels_.append(label);
    } else if (label != labels_[labels_.size() - 1]) {
      if (label < labels_[labels_.size() - 1]) {
        DARTS_THROW("failed to build double-array: wrong key order");
      }
      labels_.append(label);
    }
  }

  id_type offset = find_valid_offset(dic_id);
  units_[dic_id].set_offset(dic_id ^ offset);

  for (std::size_t i = 0; i < labels_.size(); ++i) {
    id_type dic_child_id = offset ^ labels_[i];
    reserve_id(dic_child_id);
    if (labels_[i] == '\0') {
      units_[dic_id].set_has_leaf(true);
      units_[dic_child_id].set_value(value);
    } else {
      units_[dic_child_id].set_label(labels_[i]);
    }
  }
  extras(offset).set_is_used(true);

  return offset;
}

inline id_type DoubleArrayBuilder::find_valid_offset(id_type id) const {
  if (extras_head_ >= units_.size()) {
    return units_.size() | (id & LOWER_MASK);
  }

  id_type unfixed_id = extras_head_;
  do {
    id_type offset = unfixed_id ^ labels_[0];
    if (is_valid_offset(id, offset)) {
      return offset;
    }
    unfixed_id = extras(unfixed_id).next();
  } while (unfixed_id != extras_head_);

  return units_.size() | (id & LOWER_MASK);
}

inline bool DoubleArrayBuilder::is_valid_offset(id_type id,
                                                id_type offset) const {
  if (extras(offset).is_used()) {
    return false;
  }

  id_type rel_offset = id ^ offset;
  if ((rel_offset & LOWER_MASK) && (rel_offset & UPPER_MASK)) {
    return false;
  }

  for (std::size_t i = 1; i < labels_.size(); ++i) {
    if (extras(offset ^ labels_[i]).is_fixed()) {
      return false;
    }
  }

  return true;
}

inline void DoubleArrayBuilder::reserve_id(id_type id) {
  if (id >= units_.size()) {
    expand_units();
  }

  if (id == extras_head_) {
    extras_head_ = extras(id).next();
    if (extras_head_ == id) {
      extras_head_ = units_.size();
    }
  }
  extras(extras(id).prev()).set_next(extras(id).next());
  extras(extras(id).next()).set_prev(extras(id).prev());
  extras(id).set_is_fixed(true);
}

inline void DoubleArrayBuilder::expand_units() {
  id_type src_num_units = units_.size();
  id_type src_num_blocks = num_blocks();

  id_type dest_num_units = src_num_units + BLOCK_SIZE;
  id_type dest_num_blocks = src_num_blocks + 1;

  if (dest_num_blocks > NUM_EXTRA_BLOCKS) {
    fix_block(src_num_blocks - NUM_EXTRA_BLOCKS);
  }

  units_.resize(dest_num_units);

  if (dest_num_blocks > NUM_EXTRA_BLOCKS) {
    for (std::size_t id = src_num_units; id < dest_num_units; ++id) {
      extras(id).set_is_used(false);
      extras(id).set_is_fixed(false);
    }
  }

  for (id_type i = src_num_units + 1; i < dest_num_units; ++i) {
    extras(i - 1).set_next(i);
    extras(i).set_prev(i - 1);
  }

  extras(src_num_units).set_prev(dest_num_units - 1);
  extras(dest_num_units - 1).set_next(src_num_units);

  extras(src_num_units).set_prev(extras(extras_head_).prev());
  extras(dest_num_units - 1).set_next(extras_head_);

  extras(extras(extras_head_).prev()).set_next(src_num_units);
  extras(extras_head_).set_prev(dest_num_units - 1);
}

inline void DoubleArrayBuilder::fix_all_blocks() {
  id_type begin = 0;
  if (num_blocks() > NUM_EXTRA_BLOCKS) {
    begin = num_blocks() - NUM_EXTRA_BLOCKS;
  }
  id_type end = num_blocks();

  for (id_type block_id = begin; block_id != end; ++block_id) {
    fix_block(block_id);
  }
}

inline void DoubleArrayBuilder::fix_block(id_type block_id) {
  id_type begin = block_id * BLOCK_SIZE;
  id_type end = begin + BLOCK_SIZE;

  id_type unused_offset = 0;
  for (id_type offset = begin; offset != end; ++offset) {
    if (!extras(offset).is_used()) {
      unused_offset = offset;
      break;
    }
  }

  for (id_type id = begin; id != end; ++id) {
    if (!extras(id).is_fixed()) {
      reserve_id(id);
      units_[id].set_label(static_cast<uchar_type>(id ^ unused_offset));
    }
  }
}

}  // namespace Details

//
// Member function build() of DoubleArrayImpl.
//

template <typename A, typename B, typename T, typename C>
int DoubleArrayImpl<A, B, T, C>::build(
    std::size_t num_keys,
    const key_type* const* keys,
    const std::size_t* lengths,
    const value_type* values,
    Details::progress_func_type progress_func) {
  Details::Keyset<value_type> keyset(num_keys, keys, lengths, values);

  Details::DoubleArrayBuilder builder(progress_func);
  builder.build(keyset);

  std::size_t size = 0;
  unit_type* buf = NULL;
  builder.copy(&size, &buf);

  clear();

  size_ = size;
  array_ = buf;
  buf_ = buf;

  if (progress_func != NULL) {
    progress_func(num_keys + 1, num_keys + 1);
  }

  return 0;
}

}  // namespace Darts

#undef DARTS_INT_TO_STR
#undef DARTS_LINE_TO_STR
#undef DARTS_LINE_STR
#undef DARTS_THROW

#endif  // DARTS_H_
