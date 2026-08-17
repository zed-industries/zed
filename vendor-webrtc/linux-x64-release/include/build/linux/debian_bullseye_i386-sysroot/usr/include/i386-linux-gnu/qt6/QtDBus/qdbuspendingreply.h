// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDBUSPENDINGREPLY_H
#define QDBUSPENDINGREPLY_H

#include <QtDBus/qtdbusglobal.h>
#include <QtDBus/qdbusargument.h>
#include <QtDBus/qdbuspendingcall.h>

#ifndef QT_NO_DBUS

QT_BEGIN_NAMESPACE


class Q_DBUS_EXPORT QDBusPendingReplyBase : public QDBusPendingCall
{
protected:
    QDBusPendingReplyBase();
    ~QDBusPendingReplyBase();
    void assign(const QDBusPendingCall &call);
    void assign(const QDBusMessage &message);

    QVariant argumentAt(int index) const;
    void setMetaTypes(int count, const QMetaType *metaTypes);
};

namespace QDBusPendingReplyTypes {
    template<int Index, typename T, typename... Types>
    struct Select
    {
        typedef Select<Index - 1, Types...> Next;
        typedef typename Next::Type Type;
    };
    template<typename T, typename... Types>
    struct Select<0, T, Types...>
    {
        typedef T Type;
    };

    template<typename T> inline QMetaType metaTypeFor()
    { return QMetaType::fromType<T>(); }
    // specialize for QVariant, allowing it to be used in place of QDBusVariant
    template<> inline QMetaType metaTypeFor<QVariant>()
    { return QMetaType::fromType<QDBusVariant>(); }
}


template<typename... Types>
class QDBusPendingReply : public QDBusPendingReplyBase
{
    template<int Index> using Select = QDBusPendingReplyTypes::Select<Index, Types...>;
public:
    enum { Count = std::is_same_v<typename Select<0>::Type, void> ? 0 : sizeof...(Types) };

    inline constexpr int count() const { return Count; }


    inline QDBusPendingReply() = default;
    inline QDBusPendingReply(const QDBusPendingReply &other)
        : QDBusPendingReplyBase(other)
    { }
    inline Q_IMPLICIT QDBusPendingReply(const QDBusPendingCall &call) // required by qdbusxml2cpp-generated code
    { *this = call; }
    inline Q_IMPLICIT QDBusPendingReply(const QDBusMessage &message)
    { *this = message; }

    inline QDBusPendingReply &operator=(const QDBusPendingReply &other)
    { assign(other); return *this; }
    inline QDBusPendingReply &operator=(const QDBusPendingCall &call)
    { assign(call); return *this; }
    inline QDBusPendingReply &operator=(const QDBusMessage &message)
    { assign(message); return *this; }

    using QDBusPendingReplyBase::argumentAt;
    template<int Index> inline
    typename Select<Index>::Type argumentAt() const
    {
        static_assert(Index >= 0 && Index < Count, "Index out of bounds");
        typedef typename Select<Index>::Type ResultType;
        return qdbus_cast<ResultType>(argumentAt(Index));
    }

#if defined(Q_CLANG_QDOC)
    bool isFinished() const;
    void waitForFinished();
    QVariant argumentAt(int index) const;

    bool isValid() const;
    bool isError() const;
    QDBusError error() const;
    QDBusMessage reply() const;
#endif

    inline typename Select<0>::Type value() const
    {
        return argumentAt<0>();
    }

    inline operator typename Select<0>::Type() const
    {
        return argumentAt<0>();
    }

private:
    inline void calculateMetaTypes()
    {
        if (!d) return;
        if constexpr (Count == 0) {
                setMetaTypes(0, nullptr);
        } else {
            std::array<QMetaType, Count> typeIds = { QDBusPendingReplyTypes::metaTypeFor<Types>()... };
            setMetaTypes(Count, typeIds.data());
        }
    }

    inline void assign(const QDBusPendingCall &call)
    {
        QDBusPendingReplyBase::assign(call);
        calculateMetaTypes();
    }

    inline void assign(const QDBusMessage &message)
    {
        QDBusPendingReplyBase::assign(message);
        calculateMetaTypes();
    }
};

template<>
class QDBusPendingReply<> : public QDBusPendingReplyBase
{
public:
    enum { Count = 0 };
    inline int count() const { return Count; }

    inline QDBusPendingReply() = default;
    inline QDBusPendingReply(const QDBusPendingReply &other)
        : QDBusPendingReplyBase(other)
    { }
    inline Q_IMPLICIT QDBusPendingReply(const QDBusPendingCall &call) // required by qdbusxml2cpp-generated code
    { *this = call; }
    inline Q_IMPLICIT QDBusPendingReply(const QDBusMessage &message)
    { *this = message; }

    inline QDBusPendingReply &operator=(const QDBusPendingReply &other)
    { assign(other); return *this; }
    inline QDBusPendingReply &operator=(const QDBusPendingCall &call)
    { assign(call); return *this; }
    inline QDBusPendingReply &operator=(const QDBusMessage &message)
    { assign(message); return *this; }

private:
    inline void assign(const QDBusPendingCall &call)
    {
        QDBusPendingReplyBase::assign(call);
        if (d)
            setMetaTypes(0, nullptr);
    }

    inline void assign(const QDBusMessage &message)
    {
        QDBusPendingReplyBase::assign(message);
        if (d)
            setMetaTypes(0, nullptr);
    }

};

QT_END_NAMESPACE

#endif // QT_NO_DBUS
#endif
