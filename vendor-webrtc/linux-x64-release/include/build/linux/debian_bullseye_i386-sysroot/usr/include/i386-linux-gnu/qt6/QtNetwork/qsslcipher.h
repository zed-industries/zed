// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only


#ifndef QSSLCIPHER_H
#define QSSLCIPHER_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/qstring.h>
#include <QtCore/qscopedpointer.h>
#include <QtNetwork/qssl.h>

#include <memory>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_SSL

class QSslCipherPrivate;
class Q_NETWORK_EXPORT QSslCipher
{
public:
    QSslCipher();
    explicit QSslCipher(const QString &name);
    QSslCipher(const QString &name, QSsl::SslProtocol protocol);
    QSslCipher(const QSslCipher &other);
    QSslCipher &operator=(QSslCipher &&other) noexcept { swap(other); return *this; }
    QSslCipher &operator=(const QSslCipher &other);
    ~QSslCipher();

    void swap(QSslCipher &other) noexcept
    { d.swap(other.d); }

    bool operator==(const QSslCipher &other) const;
    inline bool operator!=(const QSslCipher &other) const { return !operator==(other); }

    bool isNull() const;
    QString name() const;
    int supportedBits() const;
    int usedBits() const;

    QString keyExchangeMethod() const;
    QString authenticationMethod() const;
    QString encryptionMethod() const;
    QString protocolString() const;
    QSsl::SslProtocol protocol() const;

private:
    // ### Qt 7: make implicitly shared
    std::unique_ptr<QSslCipherPrivate> d;
    friend class QTlsBackend;
};

Q_DECLARE_SHARED(QSslCipher)

#ifndef QT_NO_DEBUG_STREAM
class QDebug;
Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, const QSslCipher &cipher);
#endif

#endif // QT_NO_SSL

QT_END_NAMESPACE

#endif

