/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtNetwork module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

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

    void swap(QHostInfo &other) noexcept { qSwap(d_ptr, other.d_ptr); }

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
        Q_STATIC_ASSERT_X(int(SignalType::ArgumentCount) >= int(SlotType::ArgumentCount),
                          "The slot requires more arguments than the signal provides.");
        Q_STATIC_ASSERT_X((QtPrivate::CheckCompatibleArguments<typename SignalType::Arguments,
                           typename SlotType::Arguments>::value),
                          "Signal and slot arguments are not compatible.");
        Q_STATIC_ASSERT_X((QtPrivate::AreArgumentsCompatible<typename SlotType::ReturnType,
                           typename SignalType::ReturnType>::value),
                          "Return type of the slot is not compatible "
                          "with the return type of the signal.");

        auto slotObj = new QtPrivate::QSlotObject<Func, typename SlotType::Arguments, void>(slot);
        return lookupHostImpl(name, receiver, slotObj);
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

        Q_STATIC_ASSERT_X(int(SlotType::ArgumentCount) <= 1,
                          "The slot must not require more than one argument");

        auto slotObj = new QtPrivate::QFunctorSlotObject<Func1, 1,
                                                         typename QtPrivate::List<QHostInfo>,
                                                         void>(std::move(slot));
        return lookupHostImpl(name, context, slotObj);
    }
#endif // Q_QDOC

private:
    QHostInfoPrivate *d_ptr;
    Q_DECLARE_PRIVATE(QHostInfo)

    static int lookupHostImpl(const QString &name,
                              const QObject *receiver,
                              QtPrivate::QSlotObjectBase *slotObj);
};

Q_DECLARE_SHARED_NOT_MOVABLE_UNTIL_QT6(QHostInfo)

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QHostInfo)

#endif // QHOSTINFO_H
