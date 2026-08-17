// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QNETWORKPROXY_H
#define QNETWORKPROXY_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtNetwork/qhostaddress.h>
#include <QtNetwork/qnetworkrequest.h>
#include <QtCore/qshareddata.h>

#ifndef QT_NO_NETWORKPROXY

QT_BEGIN_NAMESPACE


class QUrl;

class QNetworkProxyQueryPrivate;
class Q_NETWORK_EXPORT QNetworkProxyQuery
{
    Q_GADGET

public:
    enum QueryType {
        TcpSocket,
        UdpSocket,
        SctpSocket,
        TcpServer = 100,
        UrlRequest,
        SctpServer
    };
    Q_ENUM(QueryType)

    QNetworkProxyQuery();
    explicit QNetworkProxyQuery(const QUrl &requestUrl, QueryType queryType = UrlRequest);
    QNetworkProxyQuery(const QString &hostname, int port, const QString &protocolTag = QString(),
                       QueryType queryType = TcpSocket);
    explicit QNetworkProxyQuery(quint16 bindPort, const QString &protocolTag = QString(),
                       QueryType queryType = TcpServer);
    QNetworkProxyQuery(const QNetworkProxyQuery &other);
    QNetworkProxyQuery &operator=(QNetworkProxyQuery &&other) noexcept { swap(other); return *this; }
    QNetworkProxyQuery &operator=(const QNetworkProxyQuery &other);
    ~QNetworkProxyQuery();

    void swap(QNetworkProxyQuery &other) noexcept { d.swap(other.d); }

    bool operator==(const QNetworkProxyQuery &other) const;
    inline bool operator!=(const QNetworkProxyQuery &other) const
    { return !(*this == other); }

    QueryType queryType() const;
    void setQueryType(QueryType type);

    int peerPort() const;
    void setPeerPort(int port);

    QString peerHostName() const;
    void setPeerHostName(const QString &hostname);

    int localPort() const;
    void setLocalPort(int port);

    QString protocolTag() const;
    void setProtocolTag(const QString &protocolTag);

    QUrl url() const;
    void setUrl(const QUrl &url);

private:
    QSharedDataPointer<QNetworkProxyQueryPrivate> d;
};

Q_DECLARE_SHARED(QNetworkProxyQuery)

class QNetworkProxyPrivate;

class Q_NETWORK_EXPORT QNetworkProxy
{
public:
    enum ProxyType {
        DefaultProxy,
        Socks5Proxy,
        NoProxy,
        HttpProxy,
        HttpCachingProxy,
        FtpCachingProxy
    };

    enum Capability {
        TunnelingCapability = 0x0001,
        ListeningCapability = 0x0002,
        UdpTunnelingCapability = 0x0004,
        CachingCapability = 0x0008,
        HostNameLookupCapability = 0x0010,
        SctpTunnelingCapability = 0x00020,
        SctpListeningCapability = 0x00040
    };
    Q_DECLARE_FLAGS(Capabilities, Capability)

    QNetworkProxy();
    QNetworkProxy(ProxyType type, const QString &hostName = QString(), quint16 port = 0,
                  const QString &user = QString(), const QString &password = QString());
    QNetworkProxy(const QNetworkProxy &other);
    QNetworkProxy &operator=(QNetworkProxy &&other) noexcept { swap(other); return *this; }
    QNetworkProxy &operator=(const QNetworkProxy &other);
    ~QNetworkProxy();

    void swap(QNetworkProxy &other) noexcept { d.swap(other.d); }

    bool operator==(const QNetworkProxy &other) const;
    inline bool operator!=(const QNetworkProxy &other) const
    { return !(*this == other); }

    void setType(QNetworkProxy::ProxyType type);
    QNetworkProxy::ProxyType type() const;

    void setCapabilities(Capabilities capab);
    Capabilities capabilities() const;
    bool isCachingProxy() const;
    bool isTransparentProxy() const;

    void setUser(const QString &userName);
    QString user() const;

    void setPassword(const QString &password);
    QString password() const;

    void setHostName(const QString &hostName);
    QString hostName() const;

    void setPort(quint16 port);
    quint16 port() const;

    static void setApplicationProxy(const QNetworkProxy &proxy);
    static QNetworkProxy applicationProxy();

    // "cooked" headers
    QVariant header(QNetworkRequest::KnownHeaders header) const;
    void setHeader(QNetworkRequest::KnownHeaders header, const QVariant &value);

    // raw headers:
    bool hasRawHeader(const QByteArray &headerName) const;
    QList<QByteArray> rawHeaderList() const;
    QByteArray rawHeader(const QByteArray &headerName) const;
    void setRawHeader(const QByteArray &headerName, const QByteArray &value);

private:
    QSharedDataPointer<QNetworkProxyPrivate> d;
};

Q_DECLARE_SHARED(QNetworkProxy)
Q_DECLARE_OPERATORS_FOR_FLAGS(QNetworkProxy::Capabilities)

class Q_NETWORK_EXPORT QNetworkProxyFactory
{
public:
    QNetworkProxyFactory();
    virtual ~QNetworkProxyFactory();

    virtual QList<QNetworkProxy> queryProxy(const QNetworkProxyQuery &query = QNetworkProxyQuery()) = 0;

    static bool usesSystemConfiguration();
    static void setUseSystemConfiguration(bool enable);
    static void setApplicationProxyFactory(QNetworkProxyFactory *factory);
    static QList<QNetworkProxy> proxyForQuery(const QNetworkProxyQuery &query);
    static QList<QNetworkProxy> systemProxyForQuery(const QNetworkProxyQuery &query = QNetworkProxyQuery());
};

#ifndef QT_NO_DEBUG_STREAM
Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, const QNetworkProxy &proxy);
Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, const QNetworkProxyQuery &proxyQuery);
#endif

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QNetworkProxy, Q_NETWORK_EXPORT)

#endif // QT_NO_NETWORKPROXY

#endif // QHOSTINFO_H
