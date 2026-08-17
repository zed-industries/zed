// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QHOSTINFO_H
#define QHOSTINFO_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/qlist.h>
#include <QtCore/qscopedpointer.h>
#include <QtNetwork/qhostaddress.h>

QT_BEGIN_NAMESPACE


class QObject;
class QHostInfoPrivate;

class Q_NETWORK_EXPORT QHostInfo
{
public:
    enum HostInfoError {
        NoError,
        HostNotFound,
        UnknownError
    };

    explicit QHostInfo(int lookupId = -1);
    QHostInfo(const QHostInfo &d);
    QHostInfo(QHostInfo &&other) noexcept : d_ptr(qExchange(other.d_ptr, nullptr)) {}
    QHostInfo &operator=(const QHostInfo &d);
    QHostInfo &operator=(QHostInfo &&other) noexcept { swap(other); return *this; }
    ~QHostInfo();

    void swap(QHostInfo &other) noexcept { qt_ptr_swap(d_ptr, other.d_ptr); }

    QString hostName() const;
    void setHostName(const QString &name);

    QList<QHostAddress> addresses() const;
    void setAddresses(const QList<QHostAddress> &addresses);

    HostInfoError error() const;
    void setError(HostInfoError error);

    QString errorString() const;
    void setErrorString(const QString &errorString);

    void setLookupId(int id);
    int lookupId() const;

    static int lookupHost(const QString &name, QObject *receiver, const char *member);
    static void abortHostLookup(int lookupId);

    static QHostInfo fromName(const QString &name);
    static QString localHostName();
    static QString localDomainName();

#ifdef Q_CLANG_QDOC
    template<typename Functor>
    static int lookupHost(const QString &name, Functor functor);
    template<typename Functor>
    static int lookupHost(const QString &name, const QObject *context, Functor functor);
#else
    // lookupHost to a QObject slot
    template <typename Func>
    static inline int lookupHost(const QString &name,
                                 const typename QtPrivate::FunctionPointer<Func>::Object *receiver,
                                 Func slot)
    {
        typedef QtPrivate::FunctionPointer<Func> SlotType;

        typedef QtPrivate::FunctionPointer<void (*)(QHostInfo)> SignalType;
        static_assert(int(SignalType::ArgumentCount) >= int(SlotType::ArgumentCount),
                          "The slot requires more arguments than the signal provides.");
        static_assert((QtPrivate::CheckCompatibleArguments<typename SignalType::Arguments,
                           typename SlotType::Arguments>::value),
                          "Signal and slot arguments are not compatible.");
        static_assert((QtPrivate::AreArgumentsCompatible<typename SlotType::ReturnType,
                           typename SignalType::ReturnType>::value),
                          "Return type of the slot is not compatible "
                          "with the return type of the signal.");

        auto slotObj = new QtPrivate::QSlotObject<Func, typename SlotType::Arguments, void>(slot);
        return lookupHostImpl(name, receiver, slotObj, nullptr);
    }

    // lookupHost to a callable (without context)
    template <typename Func>
    static inline typename std::enable_if<!QtPrivate::FunctionPointer<Func>::IsPointerToMemberFunction &&
                                          !std::is_same<const char *, Func>::value, int>::type
        lookupHost(const QString &name, Func slot)
    {
        return lookupHost(name, nullptr, std::move(slot));
    }

    // lookupHost to a functor or function pointer (with context)
    template <typename Func1>
    static inline typename std::enable_if<!QtPrivate::FunctionPointer<Func1>::IsPointerToMemberFunction &&
                                          !std::is_same<const char*, Func1>::value, int>::type
        lookupHost(const QString &name, QObject *context, Func1 slot)
    {
        typedef QtPrivate::FunctionPointer<Func1> SlotType;

        static_assert(int(SlotType::ArgumentCount) <= 1,
                          "The slot must not require more than one argument");

        auto slotObj = new QtPrivate::QFunctorSlotObject<Func1, 1,
                                                         typename QtPrivate::List<QHostInfo>,
                                                         void>(std::move(slot));
        return lookupHostImpl(name, context, slotObj, nullptr);
    }
#endif // Q_QDOC

private:
    QHostInfoPrivate *d_ptr;
    Q_DECLARE_PRIVATE(QHostInfo)

    static int lookupHostImpl(const QString &name,
                              const QObject *receiver,
                              QtPrivate::QSlotObjectBase *slotObj,
                              const char *member);

    friend QHostInfo Q_NETWORK_EXPORT qt_qhostinfo_lookup(const QString &name, QObject *receiver,
                                                          const char *member, bool *valid, int *id);
};

Q_DECLARE_SHARED(QHostInfo)

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QHostInfo, Q_NETWORK_EXPORT)

#endif // QHOSTINFO_H
