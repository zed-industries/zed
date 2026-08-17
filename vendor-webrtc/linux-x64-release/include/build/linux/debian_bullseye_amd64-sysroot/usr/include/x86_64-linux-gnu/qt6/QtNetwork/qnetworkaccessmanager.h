// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QNETWORKACCESSMANAGER_H
#define QNETWORKACCESSMANAGER_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtNetwork/qnetworkrequest.h>
#include <QtCore/QString>
#include <QtCore/QList>
#include <QtCore/QObject>
#ifndef QT_NO_SSL
#include <QtNetwork/QSslConfiguration>
#include <QtNetwork/QSslPreSharedKeyAuthenticator>
#endif
Q_MOC_INCLUDE(<QtNetwork/QSslError>)

QT_BEGIN_NAMESPACE

class QIODevice;
class QAbstractNetworkCache;
class QAuthenticator;
class QByteArray;
class QNetworkCookie;
class QNetworkCookieJar;
class QNetworkReply;
class QNetworkProxy;
class QNetworkProxyFactory;
class QSslError;
class QHstsPolicy;
class QHttpMultiPart;

class QNetworkReplyImplPrivate;
class QNetworkAccessManagerPrivate;
class Q_NETWORK_EXPORT QNetworkAccessManager: public QObject
{
    Q_OBJECT


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

    explicit QNetworkAccessManager(QObject *parent = nullptr);
    ~QNetworkAccessManager();

    virtual QStringList supportedSchemes() const;

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
    void addStrictTransportSecurityHosts(const QList<QHstsPolicy> &knownHosts);
    QList<QHstsPolicy> strictTransportSecurityHosts() const;

    QNetworkReply *head(const QNetworkRequest &request);
    QNetworkReply *get(const QNetworkRequest &request);
    QNetworkReply *post(const QNetworkRequest &request, QIODevice *data);
    QNetworkReply *post(const QNetworkRequest &request, const QByteArray &data);
    QNetworkReply *put(const QNetworkRequest &request, QIODevice *data);
    QNetworkReply *put(const QNetworkRequest &request, const QByteArray &data);
    QNetworkReply *deleteResource(const QNetworkRequest &request);
    QNetworkReply *sendCustomRequest(const QNetworkRequest &request, const QByteArray &verb, QIODevice *data = nullptr);
    QNetworkReply *sendCustomRequest(const QNetworkRequest &request, const QByteArray &verb, const QByteArray &data);

#if QT_CONFIG(http) || defined(Q_OS_WASM)
    QNetworkReply *post(const QNetworkRequest &request, QHttpMultiPart *multiPart);
    QNetworkReply *put(const QNetworkRequest &request, QHttpMultiPart *multiPart);
    QNetworkReply *sendCustomRequest(const QNetworkRequest &request, const QByteArray &verb, QHttpMultiPart *multiPart);
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
#ifndef QT_NO_SSL
    Q_PRIVATE_SLOT(d_func(), void _q_replyPreSharedKeyAuthenticationRequired(QSslPreSharedKeyAuthenticator*))
#endif
};

QT_END_NAMESPACE

#endif
