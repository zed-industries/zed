// Copyright 2017 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_CLIENT_ANNOTATION_H_
#define CRASHPAD_CLIENT_ANNOTATION_H_

#include <stdint.h>
#include <string.h>
#include <sys/types.h>

#include <algorithm>
#include <atomic>
#include <optional>
#include <ostream>
#include <string_view>

#include "base/check.h"
#include "base/numerics/safe_conversions.h"

#include "build/build_config.h"
#include "util/synchronization/scoped_spin_guard.h"

namespace crashpad {
#if BUILDFLAG(IS_IOS)
namespace internal {
class InProcessIntermediateDumpHandler;
}  // namespace internal
#endif
class AnnotationList;

//! \brief Base class for an annotation, which records a name-value pair of
//!     arbitrary data when set.
//!
//! After an annotation is declared, its `value_ptr_` will not be captured in a
//! crash report until a call to \a SetSize() specifies how much data from the
//! value should be recorded.
//!
//! Annotations should be declared with static storage duration.
//!
//! An example declaration and usage:
//!
//! \code
//!   // foo.cc:
//!
//!   namespace {
//!   char g_buffer[1024];
//!   crashpad::Annotation g_buffer_annotation(
//!       crashpad::Annotation::Type::kString, "buffer_head", g_buffer);
//!   }  // namespace
//!
//!   void OnBufferProduced(size_t n) {
//!     // Capture the head of the buffer, in case we crash when parsing it.
//!     g_buffer_annotation.SetSize(std::min(64, n));
//!
//!     // Start parsing the header.
//!     Frobinate(g_buffer, n);
//!   }
//! \endcode
//!
//! Annotation objects are not inherently thread-safe. To manipulate them
//! from multiple threads, external synchronization must be used.
//!
//! Annotation objects should never be destroyed. Once they are Set(), they
//! are permanently referenced by a global object.
class Annotation {
 public:
  //! \brief The maximum length of an annotation’s name, in bytes.
  //!    Matches the behavior of Breakpad's SimpleStringDictionary.
  static constexpr size_t kNameMaxLength = 256;

  //! \brief The maximum size of an annotation’s value, in bytes.
  static constexpr size_t kValueMaxSize = 5 * 4096;

  //! \brief The type used for \a SetSize().
  using ValueSizeType = uint32_t;

  //! \brief The type of data stored in the annotation.
  enum class Type : uint16_t {
    //! \brief An invalid annotation. Reserved for internal use.
    kInvalid = 0,

    //! \brief A `NUL`-terminated C-string.
    kString = 1,

    //! \brief Clients may declare their own custom types by using values
    //!     greater than this.
    kUserDefinedStart = 0x8000,
  };

  //! \brief Mode used to guard concurrent reads from writes.
  enum class ConcurrentAccessGuardMode : bool {
    //! \!brief Annotation does not guard reads from concurrent
    //!     writes. Annotation values can be corrupted if the process crashes
    //!     mid-write and the handler tries to read from the Annotation while
    //!     being written to.
    kUnguarded = false,

    //! \!brief Annotation guards reads from concurrent writes using
    //!     ScopedSpinGuard. Clients must use TryCreateScopedSpinGuard()
    //!     before reading or writing the data in this Annotation.
    kScopedSpinGuard = true,
  };

  //! \brief Creates a user-defined Annotation::Type.
  //!
  //! This exists to remove the casting overhead of `enum class`.
  //!
  //! \param[in] value A value used to create a user-defined type.
  //!
  //! \returns The value added to Type::kUserDefinedStart and casted.
  constexpr static Type UserDefinedType(uint16_t value) {
    using UnderlyingType = std::underlying_type<Type>::type;
    // MSVS 2015 doesn't have full C++14 support and complains about local
    // variables defined in a constexpr function, which is valid. Avoid them
    // and the also-problematic DCHECK until all the infrastructure is updated:
    // https://crbug.com/crashpad/201.
#if !BUILDFLAG(IS_WIN) || (defined(_MSC_VER) && _MSC_VER >= 1910)
    const UnderlyingType start =
        static_cast<UnderlyingType>(Type::kUserDefinedStart);
    const UnderlyingType user_type = start + value;
    DCHECK(user_type > start) << "User-defined Type is 0 or overflows";
    return static_cast<Type>(user_type);
#else
    return static_cast<Type>(
        static_cast<UnderlyingType>(Type::kUserDefinedStart) + value);
#endif
  }

  //! \brief Constructs a new annotation.
  //!
  //! Upon construction, the annotation will not be included in any crash
  //! reports until \sa SetSize() is called with a value greater than `0`.
  //!
  //! \param[in] type The data type of the value of the annotation.
  //! \param[in] name A `NUL`-terminated C-string name for the annotation. Names
  //!     do not have to be unique, though not all crash processors may handle
  //!     Annotations with the same name. Names should be constexpr data with
  //!     static storage duration.
  //! \param[in] value_ptr A pointer to the value for the annotation. The
  //!     pointer may not be changed once associated with an annotation, but
  //!     the data may be mutated.
  constexpr Annotation(Type type, const char name[], void* value_ptr)
      : Annotation(type,
                   name,
                   value_ptr,
                   ConcurrentAccessGuardMode::kUnguarded) {}

  Annotation(const Annotation&) = delete;
  Annotation& operator=(const Annotation&) = delete;

  //! \brief Specifies the number of bytes in \a value_ptr_ to include when
  //!     generating a crash report.
  //!
  //! A size of `0` indicates that no value should be recorded and is the
  //! equivalent of calling \sa Clear().
  //!
  //! This method does not mutate the data referenced by the annotation, it
  //! merely updates the annotation system's bookkeeping.
  //!
  //! Subclasses of this base class that provide additional Set methods to
  //! mutate the value of the annotation must call always call this method.
  //!
  //! \param[in] size The number of bytes.
  void SetSize(ValueSizeType size);

  //! \brief Marks the annotation as cleared, indicating the \a value_ptr_
  //!     should not be included in a crash report.
  //!
  //! This method does not mutate the data referenced by the annotation, it
  //! merely updates the annotation system's bookkeeping.
  void Clear();

  //! \brief Tests whether the annotation has been set.
  bool is_set() const { return size_ > 0; }

  Type type() const { return type_; }
  ValueSizeType size() const { return size_; }
  const char* name() const { return name_; }
  const void* value() const { return value_ptr_; }

  ConcurrentAccessGuardMode concurrent_access_guard_mode() const {
    return concurrent_access_guard_mode_;
  }

  //! \brief If this Annotation guards concurrent access using ScopedSpinGuard,
  //!     tries to obtain the spin guard and returns the result.
  //!
  //! \param[in] timeout_ns The timeout in nanoseconds after which to give up
  //!     trying to obtain the spin guard.
  //! \return std::nullopt if the spin guard could not be obtained within
  //!     timeout_ns, or the obtained spin guard otherwise.
  std::optional<ScopedSpinGuard> TryCreateScopedSpinGuard(uint64_t timeout_ns) {
    // This can't use DCHECK_EQ() because ostream doesn't support
    // operator<<(bool).
    DCHECK(concurrent_access_guard_mode_ ==
           ConcurrentAccessGuardMode::kScopedSpinGuard);
    if (concurrent_access_guard_mode_ ==
        ConcurrentAccessGuardMode::kUnguarded) {
      return std::nullopt;
    }
    return ScopedSpinGuard::TryCreateScopedSpinGuard(timeout_ns,
                                                     spin_guard_state_);
  }

 protected:
  //! \brief Constructs a new annotation.
  //!
  //! Upon construction, the annotation will not be included in any crash
  //! reports until \sa SetSize() is called with a value greater than `0`.
  //!
  //! \param[in] type The data type of the value of the annotation.
  //! \param[in] name A `NUL`-terminated C-string name for the annotation. Names
  //!     do not have to be unique, though not all crash processors may handle
  //!     Annotations with the same name. Names should be constexpr data with
  //!     static storage duration.
  //! \param[in] value_ptr A pointer to the value for the annotation. The
  //!     pointer may not be changed once associated with an annotation, but
  //!     the data may be mutated.
  //! \param[in] concurrent_access_guard_mode Mode used to guard concurrent
  //!     reads from writes.
  constexpr Annotation(Type type,
                       const char name[],
                       void* value_ptr,
                       ConcurrentAccessGuardMode concurrent_access_guard_mode)
      : link_node_(nullptr),
        name_(name),
        value_ptr_(value_ptr),
        size_(0),
        type_(type),
        concurrent_access_guard_mode_(concurrent_access_guard_mode),
        spin_guard_state_() {}

  friend class AnnotationList;
#if BUILDFLAG(IS_IOS)
  friend class internal::InProcessIntermediateDumpHandler;
#endif

  std::atomic<Annotation*>& link_node() { return link_node_; }

  Annotation* GetLinkNode(std::memory_order order = std::memory_order_seq_cst) {
    return link_node_.load(order);
  }
  const Annotation* GetLinkNode(
      std::memory_order order = std::memory_order_seq_cst) const {
    return link_node_.load(order);
  }

 private:
  //! \brief Linked list next-node pointer. Accessed only by \sa AnnotationList.
  //!
  //! This will be null until the first call to \sa SetSize(), after which the
  //! presence of the pointer will prevent the node from being added to the
  //! list again.
  std::atomic<Annotation*> link_node_;

  const char* const name_;
  void* const value_ptr_;
  ValueSizeType size_;
  const Type type_;

  //! \brief Mode used to guard concurrent reads from writes.
  const ConcurrentAccessGuardMode concurrent_access_guard_mode_;

  SpinGuardState spin_guard_state_;
};

//! \brief An \sa Annotation that stores a `NUL`-terminated C-string value.
//!
//! The storage for the value is allocated by the annotation and the template
//! parameter \a MaxSize controls the maxmium length for the value.
//!
//! It is expected that the string value be valid UTF-8, although this is not
//! validated.
template <Annotation::ValueSizeType MaxSize>
class StringAnnotation : public Annotation {
 public:
  //! \brief A constructor tag that enables braced initialization in C arrays.
  //!
  //! \sa StringAnnotation()
  enum class Tag { kArray };

  //! \brief Constructs a new StringAnnotation with the given \a name.
  //!
  //! \param[in] name The Annotation name.
  constexpr explicit StringAnnotation(const char name[])
      : Annotation(Type::kString, name, value_), value_() {}

  StringAnnotation(const StringAnnotation&) = delete;
  StringAnnotation& operator=(const StringAnnotation&) = delete;

  //! \brief Constructs a new StringAnnotation with the given \a name.
  //!
  //! This constructor takes the ArrayInitializerTag for use when
  //! initializing a C array of annotations. The main constructor is
  //! explicit and cannot be brace-initialized. As an example:
  //!
  //! \code
  //!   static crashpad::StringAnnotation<32> annotations[] = {
  //!     {"name-1", crashpad::StringAnnotation<32>::Tag::kArray},
  //!     {"name-2", crashpad::StringAnnotation<32>::Tag::kArray},
  //!     {"name-3", crashpad::StringAnnotation<32>::Tag::kArray},
  //!   };
  //! \endcode
  //!
  //! \param[in] name The Annotation name.
  //! \param[in] tag A constructor tag.
  constexpr StringAnnotation(const char name[], Tag tag)
      : StringAnnotation(name) {}

  //! \brief Sets the Annotation's string value.
  //!
  //! \param[in] value The `NUL`-terminated C-string value.
  void Set(const char* value) {
    strncpy(value_, value, MaxSize);
    SetSize(
        std::min(MaxSize, base::saturated_cast<ValueSizeType>(strlen(value))));
  }

  //! \brief Sets the Annotation's string value.
  //!
  //! \param[in] string The string value.
  void Set(std::string_view string) {
    Annotation::ValueSizeType size =
        std::min(MaxSize, base::saturated_cast<ValueSizeType>(string.size()));
    string = string.substr(0, size);
    std::copy(string.begin(), string.end(), value_);
    // Check for no embedded `NUL` characters.
    DCHECK(string.find('\0', /*pos=*/0) == std::string_view::npos)
        << "embedded NUL";
    SetSize(size);
  }

  const std::string_view value() const {
    return std::string_view(value_, size());
  }

 private:
  // This value is not `NUL`-terminated, since the size is stored by the base
  // annotation.
  char value_[MaxSize];
};

}  // namespace crashpad

#endif  // CRASHPAD_CLIENT_ANNOTATION_H_
