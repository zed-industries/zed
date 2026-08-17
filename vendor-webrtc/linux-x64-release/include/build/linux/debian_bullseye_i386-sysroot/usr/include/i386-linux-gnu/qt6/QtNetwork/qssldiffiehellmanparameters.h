// Copyright (C) 2015 Mikkel Krautz <mikkel@krautz.dk>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only


#ifndef QSSLDIFFIEHELLMANPARAMETERS_H
#define QSSLDIFFIEHELLMANPARAMETERS_H

#include <QtNetwork/qssl.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qbytearray.h>
#include <QtCore/qshareddata.h>

QT_BEGIN_NAMESPACE

#ifndef QT_NO_SSL

class QIODevice;
class QSslContext;
class QSslDiffieHellmanParametersPrivate;

class QSslDiffieHellmanParameters;
// qHash is a friend, but we can't use default arguments for friends (§8.3.6.4)
Q_NETWORK_EXPORT size_t qHash(const QSslDiffieHellmanParameters &dhparam, size_t seed = 0) noexcept;

#ifndef QT_NO_DEBUG_STREAM
class QDebug;
Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, const QSslDiffieHellmanParameters &dhparams);
#endif

class Q_NETWORK_EXPORT QSslDiffieHellmanParameters
{
public:
    enum Error {
        NoError,
        InvalidInputDataError,
        UnsafeParametersError
    };

    static QSslDiffieHellmanParameters defaultParameters();

    QSslDiffieHellmanParameters();
    QSslDiffieHellmanParameters(const QSslDiffieHellmanParameters &other);
    QSslDiffieHellmanParameters(QSslDiffieHellmanParameters &&other) noexcept : d(other.d) { other.d = nullptr; }
    ~QSslDiffieHellmanParameters();

    QSslDiffieHellmanParameters &operator=(const QSslDiffieHellmanParameters &other);
    QSslDiffieHellmanParameters &operator=(QSslDiffieHellmanParameters &&other) noexcept { swap(other); return *this; }

    void swap(QSslDiffieHellmanParameters &other) noexcept { qt_ptr_swap(d, other.d); }

    static QSslDiffieHellmanParameters fromEncoded(const QByteArray &encoded, QSsl::EncodingFormat format = QSsl::Pem);
    static QSslDiffieHellmanParameters fromEncoded(QIODevice *device, QSsl::EncodingFormat format = QSsl::Pem);

    bool isEmpty() const noexcept;
    bool isValid() const noexcept;
    Error error() const noexcept;
    QString errorString() const noexcept;

private:
    QSslDiffieHellmanParametersPrivate *d;
    friend class QSslContext;

    bool isEqual(const QSslDiffieHellmanParameters &other) const noexcept;
    friend bool operator==(const QSslDiffieHellmanParameters &lhs, const QSslDiffieHellmanParameters &rhs) noexcept
    { return lhs.isEqual(rhs); }
    friend bool operator!=(const QSslDiffieHellmanParameters &lhs, const QSslDiffieHellmanParameters &rhs) noexcept
    { return !lhs.isEqual(rhs); }

#ifndef QT_NO_DEBUG_STREAM
    friend Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, const QSslDiffieHellmanParameters &dhparam);
#endif
    friend Q_NETWORK_EXPORT size_t qHash(const QSslDiffieHellmanParameters &dhparam, size_t seed) noexcept;
};

Q_DECLARE_SHARED(QSslDiffieHellmanParameters)

#endif // QT_NO_SSL

QT_END_NAMESPACE

#endif
