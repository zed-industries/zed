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


#ifndef QSSLCERTIFICATE_H
#define QSSLCERTIFICATE_H

#ifdef verify
#undef verify
#endif

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qbytearray.h>
#include <QtCore/qcryptographichash.h>
#include <QtCore/qdatetime.h>
#include <QtCore/qregexp.h>
#include <QtCore/qsharedpointer.h>
#include <QtCore/qmap.h>
#include <QtNetwork/qssl.h>

QT_BEGIN_NAMESPACE

class QDateTime;
class QIODevice;
class QSslError;
class QSslKey;
class QSslCertificateExtension;
class QStringList;

class QSslCertificate;
// qHash is a friend, but we can't use default arguments for friends (§8.3.6.4)
Q_NETWORK_EXPORT uint qHash(const QSslCertificate &key, uint seed = 0) noexcept;

class QSslCertificatePrivate;
class Q_NETWORK_EXPORT QSslCertificate
{
public:
    enum SubjectInfo {
        Organization,
        CommonName,
        LocalityName,
        OrganizationalUnitName,
        CountryName,
        StateOrProvinceName,
        DistinguishedNameQualifier,
        SerialNumber,
        EmailAddress
    };

    enum class PatternSyntax {
        RegularExpression,
        Wildcard,
        FixedString
    };


    explicit QSslCertificate(QIODevice *device, QSsl::EncodingFormat format = QSsl::Pem);
    explicit QSslCertificate(const QByteArray &data = QByteArray(), QSsl::EncodingFormat format = QSsl::Pem);
    QSslCertificate(const QSslCertificate &other);
    ~QSslCertificate();
    QSslCertificate &operator=(QSslCertificate &&other) noexcept { swap(other); return *this; }
    QSslCertificate &operator=(const QSslCertificate &other);

    void swap(QSslCertificate &other) noexcept
    { qSwap(d, other.d); }

    bool operator==(const QSslCertificate &other) const;
    inline bool operator!=(const QSslCertificate &other) const { return !operator==(other); }

    bool isNull() const;
#if QT_DEPRECATED_SINCE(5,0)
    QT_DEPRECATED inline bool isValid() const {
        const QDateTime currentTime = QDateTime::currentDateTimeUtc();
        return currentTime >= effectiveDate() &&
               currentTime <= expiryDate() &&
               !isBlacklisted();
    }
#endif
    bool isBlacklisted() const;
    bool isSelfSigned() const;
    void clear();

    // Certificate info
    QByteArray version() const;
    QByteArray serialNumber() const;
    QByteArray digest(QCryptographicHash::Algorithm algorithm = QCryptographicHash::Md5) const;
    QStringList issuerInfo(SubjectInfo info) const;
    QStringList issuerInfo(const QByteArray &attribute) const;
    QStringList subjectInfo(SubjectInfo info) const;
    QStringList subjectInfo(const QByteArray &attribute) const;
    QString issuerDisplayName() const;
    QString subjectDisplayName() const;

    QList<QByteArray> subjectInfoAttributes() const;
    QList<QByteArray> issuerInfoAttributes() const;
#if QT_DEPRECATED_SINCE(5,0)
    QT_DEPRECATED inline QMultiMap<QSsl::AlternateNameEntryType, QString>
                  alternateSubjectNames() const { return subjectAlternativeNames(); }
#endif
    QMultiMap<QSsl::AlternativeNameEntryType, QString> subjectAlternativeNames() const;
    QDateTime effectiveDate() const;
    QDateTime expiryDate() const;
#ifndef QT_NO_SSL
    QSslKey publicKey() const;
#endif
    QList<QSslCertificateExtension> extensions() const;

    QByteArray toPem() const;
    QByteArray toDer() const;
    QString toText() const;

#if QT_DEPRECATED_SINCE(5,15)
    QT_DEPRECATED_X("Use the overload not using QRegExp")
    static QList<QSslCertificate> fromPath(const QString &path, QSsl::EncodingFormat format,
                                           QRegExp::PatternSyntax syntax);
#endif
    static QList<QSslCertificate> fromPath(const QString &path,
                                           QSsl::EncodingFormat format = QSsl::Pem,
                                           PatternSyntax syntax = PatternSyntax::FixedString);

    static QList<QSslCertificate> fromDevice(
        QIODevice *device, QSsl::EncodingFormat format = QSsl::Pem);
    static QList<QSslCertificate> fromData(
        const QByteArray &data, QSsl::EncodingFormat format = QSsl::Pem);

#ifndef QT_NO_SSL
#if QT_VERSION >= QT_VERSION_CHECK(6,0,0)
    static QList<QSslError> verify(const QList<QSslCertificate> &certificateChain, const QString &hostName = QString());
#else
    static QList<QSslError> verify(QList<QSslCertificate> certificateChain, const QString &hostName = QString());
#endif

    static bool importPkcs12(QIODevice *device,
                             QSslKey *key, QSslCertificate *cert,
                             QList<QSslCertificate> *caCertificates = nullptr,
                             const QByteArray &passPhrase=QByteArray());
#endif

    Qt::HANDLE handle() const;

private:
    QExplicitlySharedDataPointer<QSslCertificatePrivate> d;
    friend class QSslCertificatePrivate;
    friend class QSslSocketBackendPrivate;

    friend Q_NETWORK_EXPORT uint qHash(const QSslCertificate &key, uint seed) noexcept;
};
Q_DECLARE_SHARED(QSslCertificate)

#ifndef QT_NO_DEBUG_STREAM
class QDebug;
Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, const QSslCertificate &certificate);
Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, QSslCertificate::SubjectInfo info);
#endif

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QSslCertificate)

#endif
