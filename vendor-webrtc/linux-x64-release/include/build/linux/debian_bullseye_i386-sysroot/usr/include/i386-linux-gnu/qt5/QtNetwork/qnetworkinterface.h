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

#ifndef QNETWORKINTERFACE_H
#define QNETWORKINTERFACE_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/qshareddata.h>
#include <QtCore/qscopedpointer.h>
#include <QtNetwork/qhostaddress.h>

#ifndef QT_NO_NETWORKINTERFACE

QT_BEGIN_NAMESPACE

class QDeadlineTimer;
template<typename T> class QList;

class QNetworkAddressEntryPrivate;
class Q_NETWORK_EXPORT QNetworkAddressEntry
{
public:
    enum DnsEligibilityStatus : qint8 {
        DnsEligibilityUnknown = -1,
        DnsIneligible = 0,
        DnsEligible = 1
    };

    QNetworkAddressEntry();
    QNetworkAddressEntry(const QNetworkAddressEntry &other);
    QNetworkAddressEntry &operator=(QNetworkAddressEntry &&other) noexcept { swap(other); return *this; }
    QNetworkAddressEntry &operator=(const QNetworkAddressEntry &other);
    ~QNetworkAddressEntry();

    void swap(QNetworkAddressEntry &other) noexcept { qSwap(d, other.d); }

    bool operator==(const QNetworkAddressEntry &other) const;
    inline bool operator!=(const QNetworkAddressEntry &other) const
    { return !(*this == other); }

    DnsEligibilityStatus dnsEligibility() const;
    void setDnsEligibility(DnsEligibilityStatus status);

    QHostAddress ip() const;
    void setIp(const QHostAddress &newIp);

    QHostAddress netmask() const;
    void setNetmask(const QHostAddress &newNetmask);
    int prefixLength() const;
    void setPrefixLength(int length);

    QHostAddress broadcast() const;
    void setBroadcast(const QHostAddress &newBroadcast);

    bool isLifetimeKnown() const;
    QDeadlineTimer preferredLifetime() const;
    QDeadlineTimer validityLifetime() const;
    void setAddressLifetime(QDeadlineTimer preferred, QDeadlineTimer validity);
    void clearAddressLifetime();
    bool isPermanent() const;
    bool isTemporary() const { return !isPermanent(); }

private:
    QScopedPointer<QNetworkAddressEntryPrivate> d;
};

Q_DECLARE_SHARED(QNetworkAddressEntry)

class QNetworkInterfacePrivate;
class Q_NETWORK_EXPORT QNetworkInterface
{
    Q_GADGET
public:
    enum InterfaceFlag {
        IsUp = 0x1,
        IsRunning = 0x2,
        CanBroadcast = 0x4,
        IsLoopBack = 0x8,
        IsPointToPoint = 0x10,
        CanMulticast = 0x20
    };
    Q_DECLARE_FLAGS(InterfaceFlags, InterfaceFlag)
    Q_FLAG(InterfaceFlags)

    enum InterfaceType {
        Loopback = 1,
        Virtual,
        Ethernet,
        Slip,
        CanBus,
        Ppp,
        Fddi,
        Wifi,
        Ieee80211 = Wifi,   // alias
        Phonet,
        Ieee802154,
        SixLoWPAN,  // 6LoWPAN, but we can't start with a digit
        Ieee80216,
        Ieee1394,

        Unknown = 0
    };
    Q_ENUM(InterfaceType)

    QNetworkInterface();
    QNetworkInterface(const QNetworkInterface &other);
    QNetworkInterface &operator=(QNetworkInterface &&other) noexcept { swap(other); return *this; }
    QNetworkInterface &operator=(const QNetworkInterface &other);
    ~QNetworkInterface();

    void swap(QNetworkInterface &other) noexcept { qSwap(d, other.d); }

    bool isValid() const;

    int index() const;
    int maximumTransmissionUnit() const;
    QString name() const;
    QString humanReadableName() const;
    InterfaceFlags flags() const;
    InterfaceType type() const;
    QString hardwareAddress() const;
    QList<QNetworkAddressEntry> addressEntries() const;

    static int interfaceIndexFromName(const QString &name);
    static QNetworkInterface interfaceFromName(const QString &name);
    static QNetworkInterface interfaceFromIndex(int index);
    static QString interfaceNameFromIndex(int index);
    static QList<QNetworkInterface> allInterfaces();
    static QList<QHostAddress> allAddresses();

private:
    friend class QNetworkInterfacePrivate;
    QSharedDataPointer<QNetworkInterfacePrivate> d;
};

Q_DECLARE_SHARED(QNetworkInterface)

Q_DECLARE_OPERATORS_FOR_FLAGS(QNetworkInterface::InterfaceFlags)

#ifndef QT_NO_DEBUG_STREAM
Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, const QNetworkInterface &networkInterface);
#endif

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QNetworkAddressEntry)
Q_DECLARE_METATYPE(QNetworkInterface)

#endif // QT_NO_NETWORKINTERFACE

#endif
