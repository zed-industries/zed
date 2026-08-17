// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QMETACONTAINER_H
#define QMETACONTAINER_H

#include <QtCore/qcontainerinfo.h>
#include <QtCore/qflags.h>
#include <QtCore/qglobal.h>

QT_BEGIN_NAMESPACE

class QMetaType;
namespace QtPrivate {
class QMetaTypeInterface;
template<typename T>
constexpr const QMetaTypeInterface *qMetaTypeInterfaceForType();
}

namespace QtMetaContainerPrivate {

enum IteratorCapability : quint8 {
    InputCapability         = 1 << 0,
    ForwardCapability       = 1 << 1,
    BiDirectionalCapability = 1 << 2,
    RandomAccessCapability  = 1 << 3,
};

Q_DECLARE_FLAGS(IteratorCapabilities, IteratorCapability)
Q_DECLARE_OPERATORS_FOR_FLAGS(IteratorCapabilities)

enum AddRemoveCapability : quint8 {
    CanAddAtBegin    = 1 << 0,
    CanRemoveAtBegin = 1 << 1,
    CanAddAtEnd      = 1 << 2,
    CanRemoveAtEnd   = 1 << 3
};
Q_DECLARE_FLAGS(AddRemoveCapabilities, AddRemoveCapability)
Q_DECLARE_OPERATORS_FOR_FLAGS(AddRemoveCapabilities)

class QMetaContainerInterface
{
public:
    enum Position : quint8 { AtBegin, AtEnd, Unspecified };
    ushort revision = 0;
    IteratorCapabilities iteratorCapabilities;

    using SizeFn = qsizetype(*)(const void *);
    SizeFn sizeFn;
    using ClearFn = void(*)(void *);
    ClearFn clearFn;

    using CreateIteratorFn = void *(*)(void *, Position);
    CreateIteratorFn createIteratorFn;
    using DestroyIteratorFn = void(*)(const void *);
    DestroyIteratorFn destroyIteratorFn;
    using CompareIteratorFn = bool(*)(const void *, const void *);
    CompareIteratorFn compareIteratorFn;
    using CopyIteratorFn = void(*)(void *, const void *);
    CopyIteratorFn copyIteratorFn;
    using AdvanceIteratorFn = void(*)(void *, qsizetype);
    AdvanceIteratorFn advanceIteratorFn;
    using DiffIteratorFn = qsizetype(*)(const void *, const void *);
    DiffIteratorFn diffIteratorFn;

    using CreateConstIteratorFn = void *(*)(const void *, Position);
    CreateConstIteratorFn createConstIteratorFn;
    DestroyIteratorFn destroyConstIteratorFn;
    CompareIteratorFn compareConstIteratorFn;
    CopyIteratorFn copyConstIteratorFn;
    AdvanceIteratorFn advanceConstIteratorFn;
    DiffIteratorFn diffConstIteratorFn;

    QMetaContainerInterface() = default;

    template<typename MetaContainer>
    constexpr QMetaContainerInterface(const MetaContainer &)
        : iteratorCapabilities(MetaContainer::getIteratorCapabilities())
        , sizeFn(MetaContainer::getSizeFn())
        , clearFn(MetaContainer::getClearFn())
        , createIteratorFn(MetaContainer::getCreateIteratorFn())
        , destroyIteratorFn(MetaContainer::getDestroyIteratorFn())
        , compareIteratorFn(MetaContainer::getCompareIteratorFn())
        , copyIteratorFn(MetaContainer::getCopyIteratorFn())
        , advanceIteratorFn(MetaContainer::getAdvanceIteratorFn())
        , diffIteratorFn(MetaContainer::getDiffIteratorFn())
        , createConstIteratorFn(MetaContainer::getCreateConstIteratorFn())
        , destroyConstIteratorFn(MetaContainer::getDestroyConstIteratorFn())
        , compareConstIteratorFn(MetaContainer::getCompareConstIteratorFn())
        , copyConstIteratorFn(MetaContainer::getCopyConstIteratorFn())
        , advanceConstIteratorFn(MetaContainer::getAdvanceConstIteratorFn())
        , diffConstIteratorFn(MetaContainer::getDiffConstIteratorFn())
    {}
};

class QMetaSequenceInterface : public QMetaContainerInterface
{
public:
    const QtPrivate::QMetaTypeInterface *valueMetaType;
    AddRemoveCapabilities addRemoveCapabilities;

    using ValueAtIndexFn = void(*)(const void *, qsizetype, void *);
    ValueAtIndexFn valueAtIndexFn;
    using SetValueAtIndexFn = void(*)(void *, qsizetype, const void *);
    SetValueAtIndexFn setValueAtIndexFn;

    using AddValueFn = void(*)(void *, const void *, Position);
    AddValueFn addValueFn;
    using RemoveValueFn = void(*)(void *, Position);
    RemoveValueFn removeValueFn;

    using ValueAtIteratorFn = void(*)(const void *, void *);
    ValueAtIteratorFn valueAtIteratorFn;
    using SetValueAtIteratorFn = void(*)(const void *, const void *);
    SetValueAtIteratorFn setValueAtIteratorFn;
    using InsertValueAtIteratorFn = void(*)(void *, const void *, const void *);
    InsertValueAtIteratorFn insertValueAtIteratorFn;

    ValueAtIteratorFn valueAtConstIteratorFn;

    using EraseValueAtIteratorFn = void(*)(void *, const void *);
    EraseValueAtIteratorFn eraseValueAtIteratorFn;

    using EraseRangeAtIteratorFn = void(*)(void *, const void *, const void *);
    EraseRangeAtIteratorFn eraseRangeAtIteratorFn;

    QMetaSequenceInterface() = default;

    template<typename MetaSequence>
    constexpr QMetaSequenceInterface(const MetaSequence &m)
        : QMetaContainerInterface(m)
        , valueMetaType(MetaSequence::getValueMetaType())
        , addRemoveCapabilities(MetaSequence::getAddRemoveCapabilities())
        , valueAtIndexFn(MetaSequence::getValueAtIndexFn())
        , setValueAtIndexFn(MetaSequence::getSetValueAtIndexFn())
        , addValueFn(MetaSequence::getAddValueFn())
        , removeValueFn(MetaSequence::getRemoveValueFn())
        , valueAtIteratorFn(MetaSequence::getValueAtIteratorFn())
        , setValueAtIteratorFn(MetaSequence::getSetValueAtIteratorFn())
        , insertValueAtIteratorFn(MetaSequence::getInsertValueAtIteratorFn())
        , valueAtConstIteratorFn(MetaSequence::getValueAtConstIteratorFn())
        , eraseValueAtIteratorFn(MetaSequence::getEraseValueAtIteratorFn())
        , eraseRangeAtIteratorFn(MetaSequence::getEraseRangeAtIteratorFn())
    {}
};

class QMetaAssociationInterface : public QMetaContainerInterface
{
public:
    const QtPrivate::QMetaTypeInterface *keyMetaType;
    const QtPrivate::QMetaTypeInterface *mappedMetaType;

    using InsertKeyFn = void(*)(void *, const void *);
    InsertKeyFn insertKeyFn;
    using RemoveKeyFn = void(*)(void *, const void *);
    RemoveKeyFn removeKeyFn;
    using ContainsKeyFn = bool(*)(const void *, const void *);
    ContainsKeyFn containsKeyFn;

    using MappedAtKeyFn = void(*)(const void *, const void *, void *);
    MappedAtKeyFn mappedAtKeyFn;
    using SetMappedAtKeyFn = void(*)(void *, const void *, const void *);
    SetMappedAtKeyFn setMappedAtKeyFn;

    using CreateIteratorAtKeyFn = void *(*)(void *, const void *);
    CreateIteratorAtKeyFn createIteratorAtKeyFn;
    using CreateConstIteratorAtKeyFn = void *(*)(const void *, const void *);
    CreateConstIteratorAtKeyFn createConstIteratorAtKeyFn;

    using KeyAtIteratorFn = void(*)(const void *, void *);
    KeyAtIteratorFn keyAtIteratorFn;
    KeyAtIteratorFn keyAtConstIteratorFn;

    using MappedAtIteratorFn = void(*)(const void *, void *);
    MappedAtIteratorFn mappedAtIteratorFn;
    MappedAtIteratorFn mappedAtConstIteratorFn;

    using SetMappedAtIteratorFn = void(*)(const void *, const void *);
    SetMappedAtIteratorFn setMappedAtIteratorFn;

    using EraseKeyAtIteratorFn = void(*)(void *, const void *);
    EraseKeyAtIteratorFn eraseKeyAtIteratorFn;

    QMetaAssociationInterface() = default;

    template<typename MetaAssociation>
    constexpr QMetaAssociationInterface(const MetaAssociation &m)
        : QMetaContainerInterface(m)
        , keyMetaType(MetaAssociation::getKeyMetaType())
        , mappedMetaType(MetaAssociation::getMappedMetaType())
        , insertKeyFn(MetaAssociation::getInsertKeyFn())
        , removeKeyFn(MetaAssociation::getRemoveKeyFn())
        , containsKeyFn(MetaAssociation::getContainsKeyFn())
        , mappedAtKeyFn(MetaAssociation::getMappedAtKeyFn())
        , setMappedAtKeyFn(MetaAssociation::getSetMappedAtKeyFn())
        , createIteratorAtKeyFn(MetaAssociation::createIteratorAtKeyFn())
        , createConstIteratorAtKeyFn(MetaAssociation::createConstIteratorAtKeyFn())
        , keyAtIteratorFn(MetaAssociation::getKeyAtIteratorFn())
        , keyAtConstIteratorFn(MetaAssociation::getKeyAtConstIteratorFn())
        , mappedAtIteratorFn(MetaAssociation::getMappedAtIteratorFn())
        , mappedAtConstIteratorFn(MetaAssociation::getMappedAtConstIteratorFn())
        , setMappedAtIteratorFn(MetaAssociation::getSetMappedAtIteratorFn())
        , eraseKeyAtIteratorFn(MetaAssociation::getEraseKeyAtIteratorFn())
    {}
};

template<typename C>
class QMetaContainerForContainer
{
    friend QMetaContainerInterface;

    template <typename Iterator>
    static constexpr IteratorCapabilities capabilitiesForIterator()
    {
       using Tag = typename std::iterator_traits<Iterator>::iterator_category;
       IteratorCapabilities caps {};
       if constexpr (std::is_base_of_v<std::input_iterator_tag, Tag>)
           caps |= InputCapability;
       if constexpr (std::is_base_of_v<std::forward_iterator_tag, Tag>)
           caps |= ForwardCapability;
       if constexpr (std::is_base_of_v<std::bidirectional_iterator_tag, Tag>)
           caps |= BiDirectionalCapability;
       if constexpr (std::is_base_of_v<std::random_access_iterator_tag, Tag>)
           caps |= RandomAccessCapability;
       return caps;
    }

    static constexpr IteratorCapabilities getIteratorCapabilities()
    {
        if constexpr (QContainerInfo::has_iterator_v<C> && !std::is_const_v<C>)
            return capabilitiesForIterator<QContainerInfo::iterator<C>>();
        else if constexpr (QContainerInfo::has_const_iterator_v<C>)
            return capabilitiesForIterator<QContainerInfo::const_iterator<C>>();
        else
            return {};
    }

    static constexpr QMetaContainerInterface::SizeFn getSizeFn()
    {
        if constexpr (QContainerInfo::has_size_v<C>) {
            return [](const void *c) -> qsizetype { return static_cast<const C *>(c)->size(); };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::ClearFn getClearFn()
    {
        if constexpr (QContainerInfo::has_clear_v<C>) {
            return [](void *c) { return static_cast<C *>(c)->clear(); };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::CreateIteratorFn getCreateIteratorFn()
    {
        if constexpr (QContainerInfo::has_iterator_v<C> && !std::is_const_v<C>) {
            return [](void *c, QMetaContainerInterface::Position p) -> void* {
                using Iterator = QContainerInfo::iterator<C>;
                switch (p) {
                case QMetaContainerInterface::Unspecified:
                    return new Iterator;
                case QMetaContainerInterface::AtBegin:
                    return new Iterator(static_cast<C *>(c)->begin());
                case QMetaContainerInterface::AtEnd:
                    return new Iterator(static_cast<C *>(c)->end());
                }
                return nullptr;
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::DestroyIteratorFn getDestroyIteratorFn()
    {
        if constexpr (QContainerInfo::has_iterator_v<C> && !std::is_const_v<C>) {
            return [](const void *i) {
                using Iterator = QContainerInfo::iterator<C>;
                delete static_cast<const Iterator *>(i);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::CompareIteratorFn getCompareIteratorFn()
    {
        if constexpr (QContainerInfo::has_iterator_v<C> && !std::is_const_v<C>) {
            return [](const void *i, const void *j) {
                using Iterator = QContainerInfo::iterator<C>;
                return *static_cast<const Iterator *>(i) == *static_cast<const Iterator *>(j);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::CopyIteratorFn getCopyIteratorFn()
    {
        if constexpr (QContainerInfo::has_iterator_v<C> && !std::is_const_v<C>) {
            return [](void *i, const void *j) {
                using Iterator = QContainerInfo::iterator<C>;
                *static_cast<Iterator *>(i) = *static_cast<const Iterator *>(j);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::AdvanceIteratorFn getAdvanceIteratorFn()
    {
        if constexpr (QContainerInfo::has_iterator_v<C> && !std::is_const_v<C>) {
            return [](void *i, qsizetype step) {
                std::advance(*static_cast<QContainerInfo::iterator<C> *>(i), step);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::DiffIteratorFn getDiffIteratorFn()
    {
        if constexpr (QContainerInfo::has_iterator_v<C> && !std::is_const_v<C>) {
            return [](const void *i, const void *j) -> qsizetype {
                return std::distance(*static_cast<const QContainerInfo::iterator<C> *>(j),
                                     *static_cast<const QContainerInfo::iterator<C> *>(i));
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::CreateConstIteratorFn getCreateConstIteratorFn()
    {
        if constexpr (QContainerInfo::has_const_iterator_v<C>) {
            return [](const void *c, QMetaContainerInterface::Position p) -> void* {
                using Iterator = QContainerInfo::const_iterator<C>;
                switch (p) {
                case QMetaContainerInterface::Unspecified:
                    return new Iterator;
                case QMetaContainerInterface::AtBegin:
                    return new Iterator(static_cast<const C *>(c)->begin());
                case QMetaContainerInterface::AtEnd:
                    return new Iterator(static_cast<const C *>(c)->end());
                }
                return nullptr;
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::DestroyIteratorFn getDestroyConstIteratorFn()
    {
        if constexpr (QContainerInfo::has_const_iterator_v<C>) {
            return [](const void *i) {
                using Iterator = QContainerInfo::const_iterator<C>;
                delete static_cast<const Iterator *>(i);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::CompareIteratorFn getCompareConstIteratorFn()
    {
        if constexpr (QContainerInfo::has_const_iterator_v<C>) {
            return [](const void *i, const void *j) {
                using Iterator = QContainerInfo::const_iterator<C>;
                return *static_cast<const Iterator *>(i) == *static_cast<const Iterator *>(j);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::CopyIteratorFn getCopyConstIteratorFn()
    {
        if constexpr (QContainerInfo::has_const_iterator_v<C>) {
            return [](void *i, const void *j) {
                using Iterator = QContainerInfo::const_iterator<C>;
                *static_cast<Iterator *>(i) = *static_cast<const Iterator *>(j);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::AdvanceIteratorFn getAdvanceConstIteratorFn()
    {
        if constexpr (QContainerInfo::has_const_iterator_v<C>) {
            return [](void *i, qsizetype step) {
                std::advance(*static_cast<QContainerInfo::const_iterator<C> *>(i), step);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaContainerInterface::DiffIteratorFn getDiffConstIteratorFn()
    {
        if constexpr (QContainerInfo::has_const_iterator_v<C>) {
            return [](const void *i, const void *j) -> qsizetype {
                return std::distance(*static_cast<const QContainerInfo::const_iterator<C> *>(j),
                                     *static_cast<const QContainerInfo::const_iterator<C> *>(i));
            };
        } else {
            return nullptr;
        }
    }

protected:

    template<typename EraseFn>
    static constexpr EraseFn getEraseAtIteratorFn()
    {
        if constexpr (QContainerInfo::has_iterator_v<C>
                && QContainerInfo::can_erase_at_iterator_v<C> && !std::is_const_v<C>) {
            return [](void *c, const void *i) {
                static_cast<C *>(c)->erase(*static_cast<const QContainerInfo::iterator<C> *>(i));
            };
        } else {
            return nullptr;
        }
    }
};

template<typename C>
class QMetaSequenceForContainer : public QMetaContainerForContainer<C>
{
    friend QMetaSequenceInterface;

    static constexpr const QtPrivate::QMetaTypeInterface *getValueMetaType()
    {
        if constexpr (QContainerInfo::has_value_type_v<C>)
            return QtPrivate::qMetaTypeInterfaceForType<typename C::value_type>();
        else
            return nullptr;
    }

    static constexpr AddRemoveCapabilities getAddRemoveCapabilities()
    {
        AddRemoveCapabilities caps;
        if constexpr (QContainerInfo::has_push_back_v<C>)
            caps |= CanAddAtEnd;
        if constexpr (QContainerInfo::has_pop_back_v<C>)
            caps |= CanRemoveAtEnd;
        if constexpr (QContainerInfo::has_push_front_v<C>)
            caps |= CanAddAtBegin;
        if constexpr (QContainerInfo::has_pop_front_v<C>)
            caps |= CanRemoveAtBegin;
        return caps;
    }

    static constexpr QMetaSequenceInterface::ValueAtIndexFn getValueAtIndexFn()
    {
        if constexpr (QContainerInfo::has_at_index_v<C>) {
            return [](const void *c, qsizetype i, void *r) {
                *static_cast<QContainerInfo::value_type<C> *>(r)
                        = static_cast<const C *>(c)->at(i);
            };
        } else if constexpr (QContainerInfo::can_get_at_index_v<C>) {
            return [](const void *c, qsizetype i, void *r) {
                *static_cast<QContainerInfo::value_type<C> *>(r)
                        = (*static_cast<const C *>(c))[i];
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaSequenceInterface::SetValueAtIndexFn getSetValueAtIndexFn()
    {
        if constexpr (QContainerInfo::can_set_at_index_v<C>) {
            return [](void *c, qsizetype i, const void *e) {
                (*static_cast<C *>(c))[i]
                        = *static_cast<const QContainerInfo::value_type<C> *>(e);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaSequenceInterface::AddValueFn getAddValueFn()
    {
        if constexpr (QContainerInfo::has_push_back_v<C>) {
            if constexpr (QContainerInfo::has_push_front_v<C>) {
                return [](void *c, const void *v, QMetaSequenceInterface::Position position) {
                    const auto &value = *static_cast<const QContainerInfo::value_type<C> *>(v);
                    switch (position) {
                    case QMetaSequenceInterface::AtBegin:
                        static_cast<C *>(c)->push_front(value);
                        break;
                    case QMetaSequenceInterface::AtEnd:
                    case QMetaSequenceInterface::Unspecified:
                        static_cast<C *>(c)->push_back(value);
                        break;
                    }
                };
            } else {
                return [](void *c, const void *v, QMetaSequenceInterface::Position position) {
                    const auto &value = *static_cast<const QContainerInfo::value_type<C> *>(v);
                    switch (position) {
                    case QMetaSequenceInterface::AtBegin:
                        break;
                    case QMetaSequenceInterface::AtEnd:
                    case QMetaSequenceInterface::Unspecified:
                        static_cast<C *>(c)->push_back(value);
                        break;
                    }
                };
            }
        } else if constexpr (QContainerInfo::has_push_front_v<C>) {
            return [](void *c, const void *v, QMetaSequenceInterface::Position position) {
                const auto &value = *static_cast<const QContainerInfo::value_type<C> *>(v);
                switch (position) {
                case QMetaSequenceInterface::Unspecified:
                case QMetaSequenceInterface::AtBegin:
                    static_cast<C *>(c)->push_front(value);
                case QMetaSequenceInterface::AtEnd:
                    break;
                }
            };
        } else if constexpr (QContainerInfo::has_insert_v<C>) {
            return [](void *c, const void *v, QMetaSequenceInterface::Position position) {
                if (position == QMetaSequenceInterface::Unspecified) {
                    static_cast<C *>(c)->insert(
                                *static_cast<const QContainerInfo::value_type<C> *>(v));
                }
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaSequenceInterface::RemoveValueFn getRemoveValueFn()
    {
        if constexpr (QContainerInfo::has_pop_back_v<C>) {
            if constexpr (QContainerInfo::has_pop_front_v<C>) {
                return [](void *c, QMetaSequenceInterface::Position position) {
                    switch (position) {
                    case QMetaSequenceInterface::AtBegin:
                        static_cast<C *>(c)->pop_front();
                        break;
                    case QMetaSequenceInterface::AtEnd:
                    case QMetaSequenceInterface::Unspecified:
                        static_cast<C *>(c)->pop_back();
                        break;
                    }
                };
            } else {
                return [](void *c, QMetaSequenceInterface::Position position) {
                    switch (position) {
                    case QMetaSequenceInterface::AtBegin:
                        break;
                    case QMetaSequenceInterface::Unspecified:
                    case QMetaSequenceInterface::AtEnd:
                        static_cast<C *>(c)->pop_back();
                        break;
                    }
                };
            }
        } else if constexpr (QContainerInfo::has_pop_front_v<C>) {
            return [](void *c, QMetaSequenceInterface::Position position) {
                switch (position) {
                case QMetaSequenceInterface::Unspecified:
                case QMetaSequenceInterface::AtBegin:
                    static_cast<C *>(c)->pop_front();
                    break;
                case QMetaSequenceInterface::AtEnd:
                    break;
                }
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaSequenceInterface::ValueAtIteratorFn getValueAtIteratorFn()
    {
        if constexpr (QContainerInfo::has_iterator_v<C>
                && QContainerInfo::iterator_dereferences_to_value_v<C> && !std::is_const_v<C>) {
            return [](const void *i, void *r) {
                *static_cast<QContainerInfo::value_type<C> *>(r) =
                        *(*static_cast<const QContainerInfo::iterator<C> *>(i));
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaSequenceInterface::SetValueAtIteratorFn getSetValueAtIteratorFn()
    {
        if constexpr (QContainerInfo::has_iterator_v<C>
                && QContainerInfo::can_set_value_at_iterator_v<C> && !std::is_const_v<C>) {
            return [](const void *i, const void *e) {
                *(*static_cast<const QContainerInfo::iterator<C> *>(i))
                        = *static_cast<const QContainerInfo::value_type<C> *>(e);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaSequenceInterface::InsertValueAtIteratorFn getInsertValueAtIteratorFn()
    {
        if constexpr (QContainerInfo::has_iterator_v<C>
                && QContainerInfo::can_insert_value_at_iterator_v<C> && !std::is_const_v<C>) {
            return [](void *c, const void *i, const void *e) {
                static_cast<C *>(c)->insert(
                            *static_cast<const QContainerInfo::iterator<C> *>(i),
                            *static_cast<const QContainerInfo::value_type<C> *>(e));
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaSequenceInterface::ValueAtIteratorFn getValueAtConstIteratorFn()
    {
        if constexpr (QContainerInfo::has_const_iterator_v<C>
                && QContainerInfo::iterator_dereferences_to_value_v<C>) {
            return [](const void *i, void *r) {
                *static_cast<QContainerInfo::value_type<C> *>(r) =
                        *(*static_cast<const QContainerInfo::const_iterator<C> *>(i));
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaSequenceInterface::EraseValueAtIteratorFn getEraseValueAtIteratorFn()
    {
        return QMetaContainerForContainer<C>::template getEraseAtIteratorFn<
                QMetaSequenceInterface::EraseValueAtIteratorFn>();
    }

    static constexpr QMetaSequenceInterface::EraseRangeAtIteratorFn getEraseRangeAtIteratorFn()
    {
        if constexpr (QContainerInfo::has_iterator_v<C>
                && QContainerInfo::can_erase_range_at_iterator_v<C> && !std::is_const_v<C>) {
            return [](void *c, const void *i, const void *j) {
                static_cast<C *>(c)->erase(*static_cast<const QContainerInfo::iterator<C> *>(i),
                                           *static_cast<const QContainerInfo::iterator<C> *>(j));
            };
        } else {
            return nullptr;
        }
    }
};

template<typename C>
class QMetaAssociationForContainer : public QMetaContainerForContainer<C>
{
    friend QMetaAssociationInterface;

    static constexpr const QtPrivate::QMetaTypeInterface *getKeyMetaType()
    {
        if constexpr (QContainerInfo::has_key_type_v<C>)
            return QtPrivate::qMetaTypeInterfaceForType<typename C::key_type>();
        else
            return nullptr;
    }

    static constexpr const QtPrivate::QMetaTypeInterface *getMappedMetaType()
    {
        if constexpr (QContainerInfo::has_mapped_type_v<C>)
            return QtPrivate::qMetaTypeInterfaceForType<typename C::mapped_type>();
        else
            return nullptr;
    }

    static constexpr QMetaAssociationInterface::InsertKeyFn getInsertKeyFn()
    {
        if constexpr (QContainerInfo::can_insert_key_v<C>) {
            return [](void *c, const void *k) {
                static_cast<C *>(c)->insert(
                            *static_cast<const QContainerInfo::key_type<C> *>(k));
            };
        } else if constexpr (QContainerInfo::can_insert_pair_v<C>) {
            return [](void *c, const void *k) {
                static_cast<C *>(c)->insert(
                            {*static_cast<const QContainerInfo::key_type<C> *>(k), {}});
            };
        } else if constexpr (QContainerInfo::can_insert_key_mapped_v<C>) {
            return [](void *c, const void *k) {
                static_cast<C *>(c)->insert(
                            *static_cast<const QContainerInfo::key_type<C> *>(k), {});
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaAssociationInterface::RemoveKeyFn getRemoveKeyFn()
    {
        if constexpr (QContainerInfo::can_erase_at_key_v<C>) {
            return [](void *c, const void *k) {
                static_cast<C *>(c)->erase(*static_cast<const QContainerInfo::key_type<C> *>(k));
            };
        } else if constexpr (QContainerInfo::can_remove_at_key_v<C>) {
            return [](void *c, const void *k) {
                static_cast<C *>(c)->remove(*static_cast<const QContainerInfo::key_type<C> *>(k));
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaAssociationInterface::ContainsKeyFn getContainsKeyFn()
    {
        if constexpr (QContainerInfo::has_contains_v<C>) {
            return [](const void *c, const void *k) {
                return static_cast<const C *>(c)->contains(
                            *static_cast<const QContainerInfo::key_type<C> *>(k));
            };
        } else if (QContainerInfo::has_find_v<C>) {
            return [](const void *c, const void *k) {
                const C *container = static_cast<const C *>(c);
                return container->find(
                            *static_cast<const QContainerInfo::key_type<C> *>(k))
                        != container->end();
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaAssociationInterface::MappedAtKeyFn getMappedAtKeyFn()
    {
        if constexpr (QContainerInfo::has_at_key_v<C>) {
            return [](const void *c, const void *k, void *r) {
                *static_cast<QContainerInfo::mapped_type<C> *>(r)
                        = static_cast<const C *>(c)->at(
                                *static_cast<const QContainerInfo::key_type<C> *>(k));
            };
        } else if constexpr (QContainerInfo::can_get_at_key_v<C>) {
            return [](const void *c, const void *k, void *r) {
                *static_cast<QContainerInfo::mapped_type<C> *>(r)
                        = (*static_cast<const C *>(c))[
                                *static_cast<const QContainerInfo::key_type<C> *>(k)];
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaAssociationInterface::SetMappedAtKeyFn getSetMappedAtKeyFn()
    {
        if constexpr (QContainerInfo::can_set_at_key_v<C>) {
            return [](void *c, const void *k, const void *m) {
                (*static_cast<C *>(c))[*static_cast<const QContainerInfo::key_type<C> *>(k)] =
                        *static_cast<const QContainerInfo::mapped_type<C> *>(m);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaAssociationInterface::CreateIteratorAtKeyFn createIteratorAtKeyFn()
    {
        if constexpr (QContainerInfo::has_find_v<C>) {
            return [](void *c, const void *k) -> void* {
                using Iterator = QContainerInfo::iterator<C>;
                return new Iterator(static_cast<C *>(c)->find(
                            *static_cast<const QContainerInfo::key_type<C> *>(k)));
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaAssociationInterface::CreateConstIteratorAtKeyFn createConstIteratorAtKeyFn()
    {
        if constexpr (QContainerInfo::has_find_v<C>) {
            return [](const void *c, const void *k) -> void* {
                using Iterator = QContainerInfo::const_iterator<C>;
                return new Iterator(static_cast<const C *>(c)->find(
                            *static_cast<const QContainerInfo::key_type<C> *>(k)));
            };
        } else {
            return nullptr;
        }
    }

    template<typename Iterator>
    static constexpr QMetaAssociationInterface::KeyAtIteratorFn keyAtIteratorFn()
    {
        if constexpr (QContainerInfo::iterator_has_key_v<C>) {
            return [](const void *i, void *k) {
                *static_cast<QContainerInfo::key_type<C> *>(k)
                        = static_cast<const Iterator *>(i)->key();
            };
        } else if constexpr (QContainerInfo::iterator_dereferences_to_value_v<C>
                && QContainerInfo::value_type_has_first_v<C>) {
            return [](const void *i, void *k) {
                *static_cast<QContainerInfo::key_type<C> *>(k)
                        = (*static_cast<const Iterator *>(i))->first;
            };
        } else if constexpr (QContainerInfo::iterator_dereferences_to_key_v<C>) {
            return [](const void *i, void *k) {
                *static_cast<QContainerInfo::key_type<C> *>(k)
                        = *(*static_cast<const Iterator *>(i));
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaAssociationInterface::KeyAtIteratorFn getKeyAtIteratorFn()
    {
        return keyAtIteratorFn<QContainerInfo::iterator<C>>();
    }

    static constexpr QMetaAssociationInterface::KeyAtIteratorFn getKeyAtConstIteratorFn()
    {
        return keyAtIteratorFn<QContainerInfo::const_iterator<C>>();
    }

    template<typename Iterator>
    static constexpr QMetaAssociationInterface::MappedAtIteratorFn mappedAtIteratorFn()
    {
        if constexpr (QContainerInfo::iterator_has_value_v<C>) {
            return [](const void *i, void *k) {
                *static_cast<QContainerInfo::mapped_type<C> *>(k)
                        = static_cast<const Iterator *>(i)->value();
            };
        } else if constexpr (QContainerInfo::iterator_dereferences_to_value_v<C>
                && QContainerInfo::value_type_has_second_v<C>) {
            return [](const void *i, void *k) {
                *static_cast<QContainerInfo::mapped_type<C> *>(k)
                        = (*static_cast<const Iterator *>(i))->second;
            };
        } else if constexpr (QContainerInfo::iterator_dereferences_to_mapped_v<C>) {
            return [](const void *i, void *k) {
                *static_cast<QContainerInfo::mapped_type<C> *>(k)
                        = *static_cast<const Iterator *>(i);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaAssociationInterface::MappedAtIteratorFn getMappedAtIteratorFn()
    {
        return mappedAtIteratorFn<QContainerInfo::iterator<C>>();
    }

    static constexpr QMetaAssociationInterface::MappedAtIteratorFn getMappedAtConstIteratorFn()
    {
        return mappedAtIteratorFn<QContainerInfo::const_iterator<C>>();
    }

    static constexpr QMetaAssociationInterface::SetMappedAtIteratorFn getSetMappedAtIteratorFn()
    {
        if constexpr (QContainerInfo::can_set_mapped_at_iterator_v<C> && !std::is_const_v<C>) {
            return [](const void *i, const void *m) {
                *(*static_cast<const QContainerInfo::iterator<C> *>(i))
                        = *static_cast<const QContainerInfo::mapped_type<C> *>(m);
            };
        } else if constexpr (QContainerInfo::iterator_dereferences_to_value_v<C>
                && QContainerInfo::value_type_has_second_v<C>) {
            return [](const void *i, const void *m) {
                (*static_cast<const QContainerInfo::iterator<C> *>(i))->second
                        = *static_cast<const QContainerInfo::mapped_type<C> *>(m);
            };
        } else {
            return nullptr;
        }
    }

    static constexpr QMetaAssociationInterface::EraseKeyAtIteratorFn getEraseKeyAtIteratorFn()
    {
        return QMetaContainerForContainer<C>::template getEraseAtIteratorFn<
                QMetaAssociationInterface::EraseKeyAtIteratorFn>();
    }
};

} // namespace QtMetaContainerPrivate

class Q_CORE_EXPORT QMetaContainer
{
public:
    QMetaContainer() = default;
    explicit QMetaContainer(const QtMetaContainerPrivate::QMetaContainerInterface *d) : d_ptr(d) {}

    bool hasInputIterator() const;
    bool hasForwardIterator() const;
    bool hasBidirectionalIterator() const;
    bool hasRandomAccessIterator() const;

    bool hasSize() const;
    qsizetype size(const void *container) const;

    bool canClear() const;
    void clear(void *container) const;

    bool hasIterator() const;
    void *begin(void *container) const;
    void *end(void *container) const;
    void destroyIterator(const void *iterator) const;
    bool compareIterator(const void *i, const void *j) const;
    void copyIterator(void *target, const void *source) const;
    void advanceIterator(void *iterator, qsizetype step) const;
    qsizetype diffIterator(const void *i, const void *j) const;

    bool hasConstIterator() const;
    void *constBegin(const void *container) const;
    void *constEnd(const void *container) const;
    void destroyConstIterator(const void *iterator) const;
    bool compareConstIterator(const void *i, const void *j) const;
    void copyConstIterator(void *target, const void *source) const;
    void advanceConstIterator(void *iterator, qsizetype step) const;
    qsizetype diffConstIterator(const void *i, const void *j) const;

protected:
    const QtMetaContainerPrivate::QMetaContainerInterface *d_ptr = nullptr;
};

class Q_CORE_EXPORT QMetaSequence : public QMetaContainer
{
public:
    QMetaSequence() = default;
    explicit QMetaSequence(const QtMetaContainerPrivate::QMetaSequenceInterface *d) : QMetaContainer(d) {}

    template<typename T>
    static constexpr QMetaSequence fromContainer()
    {
        return QMetaSequence(&MetaSequence<T>::value);
    }

    QMetaType valueMetaType() const;

    bool isSortable() const;
    bool canAddValueAtBegin() const;
    void addValueAtBegin(void *container, const void *value) const;
    bool canAddValueAtEnd() const;
    void addValueAtEnd(void *container, const void *value) const;
    bool canRemoveValueAtBegin() const;
    void removeValueAtBegin(void *container) const;
    bool canRemoveValueAtEnd() const;
    void removeValueAtEnd(void *container) const;

    bool canGetValueAtIndex() const;
    void valueAtIndex(const void *container, qsizetype index, void *result) const;

    bool canSetValueAtIndex() const;
    void setValueAtIndex(void *container, qsizetype index, const void *value) const;

    bool canAddValue() const;
    void addValue(void *container, const void *value) const;

    bool canRemoveValue() const;
    void removeValue(void *container) const;

    bool canGetValueAtIterator() const;
    void valueAtIterator(const void *iterator, void *result) const;

    bool canSetValueAtIterator() const;
    void setValueAtIterator(const void *iterator, const void *value) const;

    bool canInsertValueAtIterator() const;
    void insertValueAtIterator(void *container, const void *iterator, const void *value) const;

    bool canEraseValueAtIterator() const;
    void eraseValueAtIterator(void *container, const void *iterator) const;

    bool canEraseRangeAtIterator() const;
    void eraseRangeAtIterator(void *container, const void *iterator1, const void *iterator2) const;

    bool canGetValueAtConstIterator() const;
    void valueAtConstIterator(const void *iterator, void *result) const;

    friend bool operator==(const QMetaSequence &a, const QMetaSequence &b)
    {
        return a.d() == b.d();
    }
    friend bool operator!=(const QMetaSequence &a, const QMetaSequence &b)
    {
        return a.d() != b.d();
    }

private:
    template<typename T>
    struct MetaSequence
    {
        static constexpr const QtMetaContainerPrivate::QMetaSequenceInterface value
            = QtMetaContainerPrivate::QMetaSequenceInterface(
                    QtMetaContainerPrivate::QMetaSequenceForContainer<T>());
    };

    const QtMetaContainerPrivate::QMetaSequenceInterface *d() const
    {
        return static_cast<const QtMetaContainerPrivate::QMetaSequenceInterface *>(d_ptr);
    }
};

class Q_CORE_EXPORT QMetaAssociation : public QMetaContainer
{
public:
    QMetaAssociation() = default;
    explicit QMetaAssociation(const QtMetaContainerPrivate::QMetaAssociationInterface *d) : QMetaContainer(d) {}

    template<typename T>
    static constexpr QMetaAssociation fromContainer()
    {
        return QMetaAssociation(&MetaAssociation<T>::value);
    }

    QMetaType keyMetaType() const;
    QMetaType mappedMetaType() const;

    bool canInsertKey() const
    {
        if (auto iface = d())
            return iface->insertKeyFn;
        return false;
    }
    void insertKey(void *container, const void *key) const
    {
        if (canInsertKey())
            d()->insertKeyFn(container, key);
    }

    bool canRemoveKey() const
    {
        if (auto iface = d())
            return iface->removeKeyFn;
        return false;
    }
    void removeKey(void *container, const void *key) const
    {
        if (canRemoveKey())
            d()->removeKeyFn(container, key);
    }

    bool canContainsKey() const
    {
        if (auto iface = d())
            return iface->containsKeyFn;
        return false;
    }
    bool containsKey(const void *container, const void *key) const
    {
        if (canContainsKey())
            return d()->containsKeyFn(container, key);
        return false;
    }


    bool canGetMappedAtKey() const
    {
        if (auto iface = d())
            return iface->mappedAtKeyFn;
        return false;
    }
    void mappedAtKey(const void *container, const void *key, void *mapped) const
    {
        if (canGetMappedAtKey())
            d()->mappedAtKeyFn(container, key, mapped);
    }

    bool canSetMappedAtKey() const
    {
        if (auto iface = d())
            return iface->setMappedAtKeyFn;
        return false;
    }
    void setMappedAtKey(void *container, const void *key, const void *mapped) const
    {
        if (canSetMappedAtKey())
            d()->setMappedAtKeyFn(container, key, mapped);
    }

    bool canGetKeyAtIterator() const
    {
        if (auto iface = d())
            return iface->keyAtIteratorFn;
        return false;
    }

    void keyAtIterator(const void *iterator, void *key) const
    {
        if (canGetKeyAtIterator())
            d()->keyAtIteratorFn(iterator, key);
    }

    bool canGetKeyAtConstIterator() const
    {
        if (auto iface = d())
            return iface->keyAtConstIteratorFn;
        return false;
    }

    void keyAtConstIterator(const void *iterator, void *key) const
    {
        if (canGetKeyAtConstIterator())
            d()->keyAtConstIteratorFn(iterator, key);
    }

    bool canGetMappedAtIterator() const
    {
        if (auto iface = d())
            return iface->mappedAtIteratorFn;
        return false;
    }

    void mappedAtIterator(const void *iterator, void *mapped) const
    {
        if (canGetMappedAtIterator())
            d()->mappedAtIteratorFn(iterator, mapped);
    }

    bool canGetMappedAtConstIterator() const
    {
        if (auto iface = d())
            return iface->mappedAtConstIteratorFn;
        return false;
    }

    void mappedAtConstIterator(const void *iterator, void *mapped) const
    {
        if (canGetMappedAtConstIterator())
            d()->mappedAtConstIteratorFn(iterator, mapped);
    }

    bool canSetMappedAtIterator() const
    {
        if (auto iface = d())
            return iface->setMappedAtIteratorFn;
        return false;
    }

    void setMappedAtIterator(const void *iterator, const void *mapped) const
    {
        if (canSetMappedAtIterator())
            d()->setMappedAtIteratorFn(iterator, mapped);
    }

    bool canCreateIteratorAtKey() const
    {
        if (auto iface = d())
            return iface->createIteratorAtKeyFn;
        return false;
    }

    void *createIteratorAtKey(void *container, const void *key) const
    {
        if (canCreateIteratorAtKey())
            return d()->createIteratorAtKeyFn(container, key);
        return nullptr;
    }

    bool canCreateConstIteratorAtKey() const
    {
        if (auto iface = d())
            return iface->createConstIteratorAtKeyFn;
        return false;
    }

    void *createConstIteratorAtKey(const void *container, const void *key) const
    {
        if (canCreateConstIteratorAtKey())
            return d()->createConstIteratorAtKeyFn(container, key);
        return nullptr;
    }

    friend bool operator==(const QMetaAssociation &a, const QMetaAssociation &b)
    {
        return a.d() == b.d();
    }
    friend bool operator!=(const QMetaAssociation &a, const QMetaAssociation &b)
    {
        return a.d() != b.d();
    }

private:
    template<typename T>
    struct MetaAssociation
    {
        static constexpr const QtMetaContainerPrivate::QMetaAssociationInterface value
                = QtMetaContainerPrivate::QMetaAssociationInterface(
                        QtMetaContainerPrivate::QMetaAssociationForContainer<T>());
    };

    const QtMetaContainerPrivate::QMetaAssociationInterface *d() const
    {
        return static_cast<const QtMetaContainerPrivate::QMetaAssociationInterface *>(d_ptr);
    }
};

QT_END_NAMESPACE

#endif // QMETACONTAINER_H
