// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QNETWORKINTERFACE_H
#define QNETWORKINTERFACE_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/qshareddata.h>
#include <QtCore/qscopedpointer.h>
#include <QtNetwork/qhostaddress.h>

#include <memory>

#ifndef QT_NO_NETWORKINTERFACE

QT_BEGIN_NAMESPACE

class QDeadlineTimer;

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

    void swap(QNetworkAddressEntry &other) noexcept { d.swap(other.d); }

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
    // ### Qt 7: make implicitly shared
    std::unique_ptr<QNetworkAddressEntryPrivate> d;
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

    void swap(QNetworkInterface &other) noexcept { d.swap(other.d); }

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
Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, const QNetworkAddressEntry &entry);
Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, const QNetworkInterface &networkInterface);
#endif

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QNetworkAddressEntry, Q_NETWORK_EXPORT)
QT_DECL_METATYPE_EXTERN(QNetworkInterface, Q_NETWORK_EXPORT)

#endif // QT_NO_NETWORKINTERFACE

#endif
