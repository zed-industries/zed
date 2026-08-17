// Copyright (C) 2021 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Marc Mutz <marc.mutz@kdab.com>
// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2013 Richard J. Moore <rich@kde.org>.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QCRYPTOGRAPHICHASH_H
#define QCRYPTOGRAPHICHASH_H

#include <QtCore/qbytearray.h>
#include <QtCore/qobjectdefs.h>

QT_BEGIN_NAMESPACE


class QCryptographicHashPrivate;
class QIODevice;

class Q_CORE_EXPORT QCryptographicHash
{
    Q_GADGET
public:
    enum Algorithm {
#ifndef QT_CRYPTOGRAPHICHASH_ONLY_SHA1
        Md4,
        Md5,
#endif
        Sha1 = 2,
#ifndef QT_CRYPTOGRAPHICHASH_ONLY_SHA1
        Sha224,
        Sha256,
        Sha384,
        Sha512,

        Keccak_224 = 7,
        Keccak_256,
        Keccak_384,
        Keccak_512,
        RealSha3_224 = 11,
        RealSha3_256,
        RealSha3_384,
        RealSha3_512,
#  ifndef QT_SHA3_KECCAK_COMPAT
        Sha3_224 = RealSha3_224,
        Sha3_256 = RealSha3_256,
        Sha3_384 = RealSha3_384,
        Sha3_512 = RealSha3_512,
#  else
        Sha3_224 = Keccak_224,
        Sha3_256 = Keccak_256,
        Sha3_384 = Keccak_384,
        Sha3_512 = Keccak_512,
#  endif

        Blake2b_160 = 15,
        Blake2b_256,
        Blake2b_384,
        Blake2b_512,
        Blake2s_128,
        Blake2s_160,
        Blake2s_224,
        Blake2s_256,
#endif
    };
    Q_ENUM(Algorithm)

    explicit QCryptographicHash(Algorithm method);
    ~QCryptographicHash();

    void reset() noexcept;

#if QT_DEPRECATED_SINCE(6, 4)
    QT_DEPRECATED_VERSION_X_6_4("Use the QByteArrayView overload instead")
    void addData(const char *data, qsizetype length);
#endif
#if QT_CORE_REMOVED_SINCE(6, 3)
    void addData(const QByteArray &data);
#endif
    void addData(QByteArrayView data) noexcept;
    bool addData(QIODevice *device);

    QByteArray result() const;
    QByteArrayView resultView() const noexcept;

#if QT_CORE_REMOVED_SINCE(6, 3)
    static QByteArray hash(const QByteArray &data, Algorithm method);
#endif
    static QByteArray hash(QByteArrayView data, Algorithm method);
    static int hashLength(Algorithm method);
private:
    Q_DISABLE_COPY(QCryptographicHash)
    QCryptographicHashPrivate *d;
};

QT_END_NAMESPACE

#endif
