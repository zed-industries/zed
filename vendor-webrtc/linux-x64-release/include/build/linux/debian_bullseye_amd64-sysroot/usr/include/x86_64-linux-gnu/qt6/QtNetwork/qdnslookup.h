// Copyright (C) 2012 Jeremy Lainé <jeremy.laine@m4x.org>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDNSLOOKUP_H
#define QDNSLOOKUP_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/qlist.h>
#include <QtCore/qobject.h>
#include <QtCore/qshareddata.h>
#include <QtCore/qsharedpointer.h>
#include <QtCore/qstring.h>
#include <QtCore/qproperty.h>

QT_REQUIRE_CONFIG(dnslookup);

QT_BEGIN_NAMESPACE

class QHostAddress;
class QDnsLookupPrivate;
class QDnsDomainNameRecordPrivate;
class QDnsHostAddressRecordPrivate;
class QDnsMailExchangeRecordPrivate;
class QDnsServiceRecordPrivate;
class QDnsTextRecordPrivate;

class Q_NETWORK_EXPORT QDnsDomainNameRecord
{
public:
    QDnsDomainNameRecord();
    QDnsDomainNameRecord(const QDnsDomainNameRecord &other);
    QDnsDomainNameRecord &operator=(QDnsDomainNameRecord &&other) noexcept { swap(other); return *this; }
    QDnsDomainNameRecord &operator=(const QDnsDomainNameRecord &other);
    ~QDnsDomainNameRecord();

    void swap(QDnsDomainNameRecord &other) noexcept { d.swap(other.d); }

    QString name() const;
    quint32 timeToLive() const;
    QString value() const;

private:
    QSharedDataPointer<QDnsDomainNameRecordPrivate> d;
    friend class QDnsLookupRunnable;
};

Q_DECLARE_SHARED(QDnsDomainNameRecord)

class Q_NETWORK_EXPORT QDnsHostAddressRecord
{
public:
    QDnsHostAddressRecord();
    QDnsHostAddressRecord(const QDnsHostAddressRecord &other);
    QDnsHostAddressRecord &operator=(QDnsHostAddressRecord &&other) noexcept { swap(other); return *this; }
    QDnsHostAddressRecord &operator=(const QDnsHostAddressRecord &other);
    ~QDnsHostAddressRecord();

    void swap(QDnsHostAddressRecord &other) noexcept { d.swap(other.d); }

    QString name() const;
    quint32 timeToLive() const;
    QHostAddress value() const;

private:
    QSharedDataPointer<QDnsHostAddressRecordPrivate> d;
    friend class QDnsLookupRunnable;
};

Q_DECLARE_SHARED(QDnsHostAddressRecord)

class Q_NETWORK_EXPORT QDnsMailExchangeRecord
{
public:
    QDnsMailExchangeRecord();
    QDnsMailExchangeRecord(const QDnsMailExchangeRecord &other);
    QDnsMailExchangeRecord &operator=(QDnsMailExchangeRecord &&other) noexcept { swap(other); return *this; }
    QDnsMailExchangeRecord &operator=(const QDnsMailExchangeRecord &other);
    ~QDnsMailExchangeRecord();

    void swap(QDnsMailExchangeRecord &other) noexcept { d.swap(other.d); }

    QString exchange() const;
    QString name() const;
    quint16 preference() const;
    quint32 timeToLive() const;

private:
    QSharedDataPointer<QDnsMailExchangeRecordPrivate> d;
    friend class QDnsLookupRunnable;
};

Q_DECLARE_SHARED(QDnsMailExchangeRecord)

class Q_NETWORK_EXPORT QDnsServiceRecord
{
public:
    QDnsServiceRecord();
    QDnsServiceRecord(const QDnsServiceRecord &other);
    QDnsServiceRecord &operator=(QDnsServiceRecord &&other) noexcept { swap(other); return *this; }
    QDnsServiceRecord &operator=(const QDnsServiceRecord &other);
    ~QDnsServiceRecord();

    void swap(QDnsServiceRecord &other) noexcept { d.swap(other.d); }

    QString name() const;
    quint16 port() const;
    quint16 priority() const;
    QString target() const;
    quint32 timeToLive() const;
    quint16 weight() const;

private:
    QSharedDataPointer<QDnsServiceRecordPrivate> d;
    friend class QDnsLookupRunnable;
};

Q_DECLARE_SHARED(QDnsServiceRecord)

class Q_NETWORK_EXPORT QDnsTextRecord
{
public:
    QDnsTextRecord();
    QDnsTextRecord(const QDnsTextRecord &other);
    QDnsTextRecord &operator=(QDnsTextRecord &&other) noexcept { swap(other); return *this; }
    QDnsTextRecord &operator=(const QDnsTextRecord &other);
    ~QDnsTextRecord();

    void swap(QDnsTextRecord &other) noexcept { d.swap(other.d); }

    QString name() const;
    quint32 timeToLive() const;
    QList<QByteArray> values() const;

private:
    QSharedDataPointer<QDnsTextRecordPrivate> d;
    friend class QDnsLookupRunnable;
};

Q_DECLARE_SHARED(QDnsTextRecord)

class Q_NETWORK_EXPORT QDnsLookup : public QObject
{
    Q_OBJECT
    Q_PROPERTY(Error error READ error NOTIFY finished)
    Q_PROPERTY(QString errorString READ errorString NOTIFY finished)
    Q_PROPERTY(QString name READ name WRITE setName NOTIFY nameChanged BINDABLE bindableName)
    Q_PROPERTY(Type type READ type WRITE setType NOTIFY typeChanged BINDABLE bindableType)
    Q_PROPERTY(QHostAddress nameserver READ nameserver WRITE setNameserver NOTIFY nameserverChanged
               BINDABLE bindableNameserver)

public:
    enum Error
    {
        NoError = 0,
        ResolverError,
        OperationCancelledError,
        InvalidRequestError,
        InvalidReplyError,
        ServerFailureError,
        ServerRefusedError,
        NotFoundError
    };
    Q_ENUM(Error)

    enum Type
    {
        A = 1,
        AAAA = 28,
        ANY = 255,
        CNAME = 5,
        MX = 15,
        NS = 2,
        PTR = 12,
        SRV = 33,
        TXT = 16
    };
    Q_ENUM(Type)

    explicit QDnsLookup(QObject *parent = nullptr);
    QDnsLookup(Type type, const QString &name, QObject *parent = nullptr);
    QDnsLookup(Type type, const QString &name, const QHostAddress &nameserver, QObject *parent = nullptr);
    ~QDnsLookup();

    Error error() const;
    QString errorString() const;
    bool isFinished() const;

    QString name() const;
    void setName(const QString &name);
    QBindable<QString> bindableName();

    Type type() const;
    void setType(QDnsLookup::Type);
    QBindable<Type> bindableType();

    QHostAddress nameserver() const;
    void setNameserver(const QHostAddress &nameserver);
    QBindable<QHostAddress> bindableNameserver();

    QList<QDnsDomainNameRecord> canonicalNameRecords() const;
    QList<QDnsHostAddressRecord> hostAddressRecords() const;
    QList<QDnsMailExchangeRecord> mailExchangeRecords() const;
    QList<QDnsDomainNameRecord> nameServerRecords() const;
    QList<QDnsDomainNameRecord> pointerRecords() const;
    QList<QDnsServiceRecord> serviceRecords() const;
    QList<QDnsTextRecord> textRecords() const;


public Q_SLOTS:
    void abort();
    void lookup();

Q_SIGNALS:
    void finished();
    void nameChanged(const QString &name);
    void typeChanged(Type type);
    void nameserverChanged(const QHostAddress &nameserver);

private:
    Q_DECLARE_PRIVATE(QDnsLookup)
    Q_PRIVATE_SLOT(d_func(), void _q_lookupFinished(const QDnsLookupReply &reply))
};

QT_END_NAMESPACE

#endif // QDNSLOOKUP_H
