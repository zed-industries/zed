/*
 * Copyright (C) 2005, 2006, 2009 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_QUALIFIED_NAME_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_QUALIFIED_NAME_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_table_deleted_value_type.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

struct QualifiedNameComponents {
  DISALLOW_NEW();
  StringImpl* prefix_;
  StringImpl* local_name_;
  StringImpl* namespace_;
};

// This struct is used to pass data between QualifiedName and the
// QNameTranslator.  For hashing and equality only the QualifiedNameComponents
// fields are used.
struct QualifiedNameData {
  DISALLOW_NEW();
  QualifiedNameComponents components_;
  bool is_static_;
};

class CORE_EXPORT QualifiedName;

}  // namespace blink

// `QualifiedName`'s only field is an interned pointer, so it's safe to hash;
// allow conversion to a byte span to facilitate this.
namespace base {
template <>
inline constexpr bool kCanSafelyConvertToByteSpan<::blink::QualifiedName> =
    true;
}

namespace blink {

CORE_EXPORT extern const QualifiedName& g_any_name;
CORE_EXPORT extern const QualifiedName& g_null_name;

class CORE_EXPORT QualifiedName {
  USING_FAST_MALLOC(QualifiedName);

 public:
  class CORE_EXPORT QualifiedNameImpl : public RefCounted<QualifiedNameImpl> {
   public:
    static scoped_refptr<QualifiedNameImpl> Create(StringImpl* prefix,
                                                   StringImpl* local_name,
                                                   StringImpl* namespace_uri,
                                                   bool is_static) {
      return base::AdoptRef(
          new QualifiedNameImpl(prefix, local_name, namespace_uri, is_static));
    }

    ~QualifiedNameImpl();

    unsigned ComputeHash() const;

    bool IsStatic() const {
      return is_static_and_html_attribute_triggers_index_ != kNotStatic;
    }

    void AddRef() {
      if (IsStatic()) {
        return;
      }
      RefCounted<QualifiedNameImpl>::AddRef();
    }

    void Release() {
      if (IsStatic()) {
        return;
      }
      RefCounted<QualifiedNameImpl>::Release();
    }

    enum StaticAndAttributeTriggersConstants {
      kLargestAllowedIndex = 253,
      kNotStatic = 254,
      kStaticWithNoIndex = 255
    };

    // We rely on HashComponents() clearing out the top 8 bits when
    // doing hashing and use one of the bits for the is_static_ value.
    mutable unsigned existing_hash_ : 24;
    mutable unsigned is_static_and_html_attribute_triggers_index_ : 8;
    const AtomicString prefix_;
    const AtomicString local_name_;
    const AtomicString namespace_;
    mutable AtomicString local_name_upper_;

   private:
    QualifiedNameImpl(StringImpl* prefix,
                      StringImpl* local_name,
                      StringImpl* namespace_uri,
                      bool is_static)
        : existing_hash_(0),
          is_static_and_html_attribute_triggers_index_(
              is_static ? kStaticWithNoIndex : kNotStatic),
          prefix_(prefix),
          local_name_(local_name),
          namespace_(namespace_uri)

    {
      DCHECK(!namespace_.empty() || namespace_.IsNull());
    }
  };

  QualifiedName(const AtomicString& prefix,
                const AtomicString& local_name,
                const AtomicString& namespace_uri);
  // Creates a QualifiedName instance with null prefix, the specified local
  // name, and null namespace.
  explicit QualifiedName(const AtomicString& local_name);
  ~QualifiedName();

  QualifiedName(const QualifiedName& other) = default;
  const QualifiedName& operator=(const QualifiedName& other) {
    impl_ = other.impl_;
    return *this;
  }
  QualifiedName(QualifiedName&& other) = default;
  QualifiedName& operator=(QualifiedName&& other) = default;

  bool operator==(const QualifiedName& other) const {
    return impl_ == other.impl_;
  }
  bool operator!=(const QualifiedName& other) const {
    return !(*this == other);
  }

  bool Matches(const QualifiedName& other) const {
    return impl_ == other.impl_ || (LocalName() == other.LocalName() &&
                                    NamespaceURI() == other.NamespaceURI());
  }

  bool HasPrefix() const { return impl_->prefix_ != g_null_atom; }
  void SetPrefix(const AtomicString& prefix) {
    *this = QualifiedName(prefix, LocalName(), NamespaceURI());
  }

  const AtomicString& Prefix() const { return impl_->prefix_; }
  const AtomicString& LocalName() const { return impl_->local_name_; }
  const AtomicString& NamespaceURI() const { return impl_->namespace_; }

  // Uppercased localName, cached for efficiency
  const AtomicString& LocalNameUpper() const {
    if (impl_->local_name_upper_)
      return impl_->local_name_upper_;
    return LocalNameUpperSlow();
  }

  const AtomicString& LocalNameUpperSlow() const;

  void RegisterHTMLAttributeTriggersIndex(unsigned index) const {
    using enum QualifiedNameImpl::StaticAndAttributeTriggersConstants;
    CHECK_EQ(impl_->is_static_and_html_attribute_triggers_index_,
             kStaticWithNoIndex);
    CHECK_LE(index, kLargestAllowedIndex);
    impl_->is_static_and_html_attribute_triggers_index_ = index;
    CHECK_EQ(*HTMLAttributeTriggersIndex(), index);
  }

  std::optional<unsigned> HTMLAttributeTriggersIndex() const {
    using enum QualifiedNameImpl::StaticAndAttributeTriggersConstants;
    if (impl_->is_static_and_html_attribute_triggers_index_ >
        kLargestAllowedIndex) {
      return std::nullopt;
    }
    return unsigned{impl_->is_static_and_html_attribute_triggers_index_};
  }

  // Returns true if this is a built-in name. That is, one of the names defined
  // at build time (such as <img>).
  bool IsDefinedName() const { return impl_ && impl_->IsStatic(); }

  String ToString() const;

  QualifiedNameImpl* Impl() const { return impl_.get(); }

  // Init routine for globals
  static void InitAndReserveCapacityForSize(unsigned size);

  static const QualifiedName& Null() { return g_null_name; }

  // The below methods are only for creating static global QNames that need no
  // ref counting.
  static void CreateStatic(void* target_address, StringImpl* name);
  static void CreateStatic(void* target_address,
                           StringImpl* name,
                           const AtomicString& name_namespace);

 private:
  friend struct WTF::HashTraits<blink::QualifiedName>;

  // This constructor is used only to create global/static QNames that don't
  // require any ref counting.
  QualifiedName(const AtomicString& prefix,
                const AtomicString& local_name,
                const AtomicString& namespace_uri,
                bool is_static);

  scoped_refptr<QualifiedNameImpl> impl_;
};

inline const QualifiedName& AnyQName() {
  return g_any_name;
}

inline bool operator==(const AtomicString& a, const QualifiedName& q) {
  return a == q.LocalName();
}
inline bool operator!=(const AtomicString& a, const QualifiedName& q) {
  return a != q.LocalName();
}
inline bool operator==(const QualifiedName& q, const AtomicString& a) {
  return a == q.LocalName();
}
inline bool operator!=(const QualifiedName& q, const AtomicString& a) {
  return a != q.LocalName();
}

inline unsigned HashComponents(const QualifiedNameComponents& buf) {
  return StringHasher::HashMemory(base::byte_span_from_ref(buf)) & 0xFFFFFF;
}

CORE_EXPORT std::ostream& operator<<(std::ostream&, const QualifiedName&);

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::QualifiedName)

namespace WTF {

template <>
struct HashTraits<blink::QualifiedName::QualifiedNameImpl*>
    : GenericHashTraits<blink::QualifiedName::QualifiedNameImpl*> {
  static unsigned GetHash(const blink::QualifiedName::QualifiedNameImpl* name) {
    if (!name->existing_hash_) {
      name->existing_hash_ = name->ComputeHash();
    }
    return name->existing_hash_;
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

template <>
struct HashTraits<blink::QualifiedName>
    : GenericHashTraits<blink::QualifiedName> {
  using QualifiedNameImpl = blink::QualifiedName::QualifiedNameImpl;
  static unsigned GetHash(const blink::QualifiedName& name) {
    return WTF::GetHash(name.Impl());
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;

  static constexpr bool kEmptyValueIsZero = false;
  static const blink::QualifiedName& EmptyValue() {
    return blink::QualifiedName::Null();
  }

  static bool IsDeletedValue(const blink::QualifiedName& value) {
    return HashTraits<scoped_refptr<QualifiedNameImpl>>::IsDeletedValue(
        value.impl_);
  }
  static void ConstructDeletedValue(blink::QualifiedName& slot) {
    HashTraits<scoped_refptr<QualifiedNameImpl>>::ConstructDeletedValue(
        slot.impl_);
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_QUALIFIED_NAME_H_
