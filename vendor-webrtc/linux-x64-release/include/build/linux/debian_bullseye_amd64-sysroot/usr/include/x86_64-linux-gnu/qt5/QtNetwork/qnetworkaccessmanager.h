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

#ifndef QNETWORKACCESSMANAGER_H
#define QNETWORKACCESSMANAGER_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtNetwork/qnetworkrequest.h>
#include <QtCore/QString>
#include <QtCore/QVector>
#include <QtCore/QObject>
#ifndef QT_NO_SSL
#include <QtNetwork/QSslConfiguration>
#include <QtNetwork/QSslPreSharedKeyAuthenticator>
#endif

QT_BEGIN_NAMESPACE

class QIODevice;
class QAbstractNetworkCache;
class QAuthenticator;
class QByteArray;
template<typename T> class QList;
class QNetworkCookie;
class QNetworkCookieJar;
class QNetworkReply;
class QNetworkProxy;
class QNetworkProxyFactory;
class QSslError;
class QHstsPolicy;
#ifndef QT_NO_BEARERMANAGEMENT // ### Qt6: Remove section
class QNetworkConfiguration;
#endif
class QHttpMultiPart;

class QNetworkReplyImplPrivate;
class QNetworkAccessManagerPrivate;
class Q_NETWORK_EXPORT QNetworkAccessManager: public QObject
{
    Q_OBJECT

#ifndef QT_NO_BEARERMANAGEMENT // ### Qt6: Remove section
    Q_PROPERTY(NetworkAccessibility networkAccessible READ networkAccessible WRITE setNetworkAccessible NOTIFY networkAccessibleChanged)
#endif

public:
    enum Operation {
        HeadOperation = 1,
        GetOperation,
        PutOperation,
        PostOperation,
        DeleteOperation,
        CustomOperation,

        UnknownOperation = 0
    };

#ifndef QT_NO_BEARERMANAGEMENT // ### Qt6: Remove section
    enum QT_DEPRECATED_NETWORK_API_5_15 NetworkAccessibility {
        UnknownAccessibility = -1,
        NotAccessible = 0,
        Accessible = 1
    };
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
    Q_ENUM(NetworkAccessibility)
QT_WARNING_POP
#endif

    explicit QNetworkAccessManager(QObject *parent = nullptr);
    ~QNetworkAccessManager();

    // ### Qt 6: turn into virtual
    QStringList supportedSchemes() const;

    void clearAccessCache();

    void clearConnectionCache();

#ifndef QT_NO_NETWORKPROXY
    QNetworkProxy proxy() const;
    void setProxy(const QNetworkProxy &proxy);
    QNetworkProxyFactory *proxyFactory() const;
    void setProxyFactory(QNetworkProxyFactory *factory);
#endif

    QAbstractNetworkCache *cache() const;
    void setCache(QAbstractNetworkCache *cache);

    QNetworkCookieJar *cookieJar() const;
    void setCookieJar(QNetworkCookieJar *cookieJar);

    void setStrictTransportSecurityEnabled(bool enabled);
    bool isStrictTransportSecurityEnabled() const;
    void enableStrictTransportSecurityStore(bool enabled, const QString &storeDir = QString());
    bool isStrictTransportSecurityStoreEnabled() const;
    void addStrictTransportSecurityHosts(const QVector<QHstsPolicy> &knownHosts);
    QVector<QHstsPolicy> strictTransportSecurityHosts() const;

    QNetworkReply *head(const QNetworkRequest &request);
    QNetworkReply *get(const QNetworkRequest &request);
    QNetworkReply *post(const QNetworkRequest &request, QIODevice *data);
    QNetworkReply *post(const QNetworkRequest &request, const QByteArray &data);
    QNetworkReply *put(const QNetworkRequest &request, QIODevice *data);
    QNetworkReply *put(const QNetworkRequest &request, const QByteArray &data);
    QNetworkReply *deleteResource(const QNetworkRequest &request);
    QNetworkReply *sendCustomRequest(const QNetworkRequest &request, const QByteArray &verb, QIODevice *data = nullptr);
    QNetworkReply *sendCustomRequest(const QNetworkRequest &request, const QByteArray &verb, const QByteArray &data);

#if QT_CONFIG(http)
    QNetworkReply *post(const QNetworkRequest &request, QHttpMultiPart *multiPart);
    QNetworkReply *put(const QNetworkRequest &request, QHttpMultiPart *multiPart);
    QNetworkReply *sendCustomRequest(const QNetworkRequest &request, const QByteArray &verb, QHttpMultiPart *multiPart);
#endif

#if !defined(QT_NO_BEARERMANAGEMENT) // ### Qt6: Remove section
    QT_DEPRECATED_VERSION_5_15 void setConfiguration(const QNetworkConfiguration &config);
    QT_DEPRECATED_VERSION_5_15 QNetworkConfiguration configuration() const;
    QT_DEPRECATED_VERSION_5_15 QNetworkConfiguration activeConfiguration() const;

QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
    QT_DEPRECATED_VERSION_5_15 void setNetworkAccessible(NetworkAccessibility accessible);
    QT_DEPRECATED_VERSION_5_15 NetworkAccessibility networkAccessible() const;
QT_WARNING_POP
#endif

#ifndef QT_NO_SSL
    void connectToHostEncrypted(const QString &hostName, quint16 port = 443,
                                const QSslConfiguration &sslConfiguration = QSslConfiguration::defaultConfiguration());
    void connectToHostEncrypted(const QString &hostName, quint16 port,
                                const QSslConfiguration &sslConfiguration,
                                const QString &peerName);
#endif
    void connectToHost(const QString &hostName, quint16 port = 80);

    void setRedirectPolicy(QNetworkRequest::RedirectPolicy policy);
    QNetworkRequest::RedirectPolicy redirectPolicy() const;

    bool autoDeleteReplies() const;
    void setAutoDeleteReplies(bool autoDelete);

    int transferTimeout() const;
    void setTransferTimeout(int timeout = QNetworkRequest::DefaultTransferTimeoutConstant);

Q_SIGNALS:
#ifndef QT_NO_NETWORKPROXY
    void proxyAuthenticationRequired(const QNetworkProxy &proxy, QAuthenticator *authenticator);
#endif
    void authenticationRequired(QNetworkReply *reply, QAuthenticator *authenticator);
    void finished(QNetworkReply *reply);
#ifndef QT_NO_SSL
    void encrypted(QNetworkReply *reply);
    void sslErrors(QNetworkReply *reply, const QList<QSslError> &errors);
    void preSharedKeyAuthenticationRequired(QNetworkReply *reply, QSslPreSharedKeyAuthenticator *authenticator);
#endif

#ifndef QT_NO_BEARERMANAGEMENT // ### Qt6: Remove section
    QT_DEPRECATED_VERSION_5_15 void networkSessionConnected();

#ifndef Q_MOC_RUN // moc has trouble with the expansion of these macros
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
#endif
    QT_DEPRECATED_VERSION_5_15 void networkAccessibleChanged(QNetworkAccessManager::NetworkAccessibility accessible);
#ifndef Q_MOC_RUN // moc has trouble with the expansion of these macros
QT_WARNING_POP
#endif
#endif

protected:
    virtual QNetworkReply *createRequest(Operation op, const QNetworkRequest &request,
                                         QIODevice *outgoingData = nullptr);

protected Q_SLOTS:
    QStringList supportedSchemesImplementation() const;

private:
    friend class QNetworkReplyImplPrivate;
    friend class QNetworkReplyHttpImpl;
    friend class QNetworkReplyHttpImplPrivate;
    friend class QNetworkReplyFileImpl;

#ifdef Q_OS_WASM
    friend class QNetworkReplyWasmImpl;
#endif
    Q_DECLARE_PRIVATE(QNetworkAccessManager)
    Q_PRIVATE_SLOT(d_func(), void _q_replySslErrors(QList<QSslError>))
    Q_PRIVATE_SLOT(d_func(), void _q_replyPreSharedKeyAuthenticationRequired(QSslPreSharedKeyAuthenticator*))
#ifndef QT_NO_BEARERMANAGEMENT // ### Qt6: Remove section
    Q_PRIVATE_SLOT(d_func(), void _q_networkSessionClosed())
    Q_PRIVATE_SLOT(d_func(), void _q_networkSessionStateChanged(QNetworkSession::State))
    Q_PRIVATE_SLOT(d_func(), void _q_configurationChanged(const QNetworkConfiguration &))
    Q_PRIVATE_SLOT(d_func(), void _q_networkSessionFailed(QNetworkSession::SessionError))
#endif
    Q_PRIVATE_SLOT(d_func(), void _q_onlineStateChanged(bool))
};

QT_END_NAMESPACE

#endif
