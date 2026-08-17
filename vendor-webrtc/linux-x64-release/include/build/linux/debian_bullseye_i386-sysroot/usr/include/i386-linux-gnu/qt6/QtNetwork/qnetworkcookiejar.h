// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QNETWORKCOOKIEJAR_H
#define QNETWORKCOOKIEJAR_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/QObject>
#include <QtCore/QUrl>

QT_BEGIN_NAMESPACE


class QNetworkCookie;

class QNetworkCookieJarPrivate;
class Q_NETWORK_EXPORT QNetworkCookieJar: public QObject
{
    Q_OBJECT
public:
    explicit QNetworkCookieJar(QObject *parent = nullptr);
    virtual ~QNetworkCookieJar();

    virtual QList<QNetworkCookie> cookiesForUrl(const QUrl &url) const;
    virtual bool setCookiesFromUrl(const QList<QNetworkCookie> &cookieList, const QUrl &url);

    virtual bool insertCookie(const QNetworkCookie &cookie);
    virtual bool updateCookie(const QNetworkCookie &cookie);
    virtual bool deleteCookie(const QNetworkCookie &cookie);

protected:
    QList<QNetworkCookie> allCookies() const;
    void setAllCookies(const QList<QNetworkCookie> &cookieList);
    virtual bool validateCookie(const QNetworkCookie &cookie, const QUrl &url) const;

private:
    Q_DECLARE_PRIVATE(QNetworkCookieJar)
    Q_DISABLE_COPY(QNetworkCookieJar)
};

QT_END_NAMESPACE

#endif
