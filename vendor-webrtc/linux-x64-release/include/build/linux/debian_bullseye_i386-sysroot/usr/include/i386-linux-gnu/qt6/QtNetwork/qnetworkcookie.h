// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QNETWORKCOOKIE_H
#define QNETWORKCOOKIE_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/QSharedDataPointer>
#include <QtCore/QList>
#include <QtCore/QMetaType>
#include <QtCore/QObject>

QT_BEGIN_NAMESPACE


class QByteArray;
class QDateTime;
class QString;
class QUrl;

class QNetworkCookiePrivate;
class Q_NETWORK_EXPORT QNetworkCookie
{
    Q_GADGET
public:
    enum RawForm {
        NameAndValueOnly,
        Full
    };
    enum class SameSite {
        Default,
        None,
        Lax,
        Strict
    };
    Q_ENUM(SameSite)

    explicit QNetworkCookie(const QByteArray &name = QByteArray(), const QByteArray &value = QByteArray());
    QNetworkCookie(const QNetworkCookie &other);
    ~QNetworkCookie();
    QNetworkCookie &operator=(QNetworkCookie &&other) noexcept { swap(other); return *this; }
    QNetworkCookie &operator=(const QNetworkCookie &other);

    void swap(QNetworkCookie &other) noexcept { d.swap(other.d); }

    bool operator==(const QNetworkCookie &other) const;
    inline bool operator!=(const QNetworkCookie &other) const
    { return !(*this == other); }

    bool isSecure() const;
    void setSecure(bool enable);
    bool isHttpOnly() const;
    void setHttpOnly(bool enable);
    SameSite sameSitePolicy() const;
    void setSameSitePolicy(SameSite sameSite);

    bool isSessionCookie() const;
    QDateTime expirationDate() const;
    void setExpirationDate(const QDateTime &date);

    QString domain() const;
    void setDomain(const QString &domain);

    QString path() const;
    void setPath(const QString &path);

    QByteArray name() const;
    void setName(const QByteArray &cookieName);

    QByteArray value() const;
    void setValue(const QByteArray &value);

    QByteArray toRawForm(RawForm form = Full) const;

    bool hasSameIdentifier(const QNetworkCookie &other) const;
    void normalize(const QUrl &url);

    static QList<QNetworkCookie> parseCookies(const QByteArray &cookieString);

private:
    QSharedDataPointer<QNetworkCookiePrivate> d;
    friend class QNetworkCookiePrivate;
};

Q_DECLARE_SHARED(QNetworkCookie)

#ifndef QT_NO_DEBUG_STREAM
class QDebug;
Q_NETWORK_EXPORT QDebug operator<<(QDebug, const QNetworkCookie &);
#endif

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QNetworkCookie, Q_NETWORK_EXPORT)

#endif
