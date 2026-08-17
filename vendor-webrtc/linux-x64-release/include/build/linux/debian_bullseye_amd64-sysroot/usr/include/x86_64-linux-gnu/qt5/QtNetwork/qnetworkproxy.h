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

#ifndef QNETWORKPROXY_H
#define QNETWORKPROXY_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtNetwork/qhostaddress.h>
#include <QtNetwork/qnetworkrequest.h>
#include <QtCore/qshareddata.h>

#ifndef QT_NO_NETWORKPROXY

QT_BEGIN_NAMESPACE


class QUrl;
class QNetworkConfiguration;

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
#if !defined(QT_NO_BEARERMANAGEMENT) && QT_DEPRECATED_SINCE(5, 10)
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
    Q_DECL_DEPRECATED_X("QNetworkConfiguration support in QNetworkProxy is deprecated")
    QNetworkProxyQuery(const QNetworkConfiguration &networkConfiguration,
                       const QUrl &requestUrl, QueryType queryType = UrlRequest);
    Q_DECL_DEPRECATED_X("QNetworkConfiguration support in QNetworkProxy is deprecated")
    QNetworkProxyQuery(const QNetworkConfiguration &networkConfiguration,
                       const QString &hostname, int port, const QString &protocolTag = QString(),
                       QueryType queryType = TcpSocket);
    Q_DECL_DEPRECATED_X("QNetworkConfiguration support in QNetworkProxy is deprecated")
    QNetworkProxyQuery(const QNetworkConfiguration &networkConfiguration,
                       quint16 bindPort, const QString &protocolTag = QString(),
                       QueryType queryType = TcpServer);
QT_WARNING_POP
#endif
    QNetworkProxyQuery(const QNetworkProxyQuery &other);
    QNetworkProxyQuery &operator=(QNetworkProxyQuery &&other) noexcept { swap(other); return *this; }
    QNetworkProxyQuery &operator=(const QNetworkProxyQuery &other);
    ~QNetworkProxyQuery();

    void swap(QNetworkProxyQuery &other) noexcept { qSwap(d, other.d); }

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

#if !defined(QT_NO_BEARERMANAGEMENT) && QT_DEPRECATED_SINCE(5, 10)
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
    Q_DECL_DEPRECATED_X("QNetworkConfiguration support in QNetworkProxy is deprecated")
    QNetworkConfiguration networkConfiguration() const;
    Q_DECL_DEPRECATED_X("QNetworkConfiguration support in QNetworkProxy is deprecated")
    void setNetworkConfiguration(const QNetworkConfiguration &networkConfiguration);
QT_WARNING_POP
#endif

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

    void swap(QNetworkProxy &other) noexcept { qSwap(d, other.d); }

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

Q_DECLARE_METATYPE(QNetworkProxy)

#endif // QT_NO_NETWORKPROXY

#endif // QHOSTINFO_H
