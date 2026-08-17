/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
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

#ifndef QUUID_H
#define QUUID_H

#include <QtCore/qstring.h>

#if defined(Q_OS_WIN) || defined(Q_CLANG_QDOC)
#ifndef GUID_DEFINED
#define GUID_DEFINED
typedef struct _GUID
{
    ulong   Data1;
    ushort  Data2;
    ushort  Data3;
    uchar   Data4[8];
} GUID, *REFGUID, *LPGUID;
#endif
#endif

#if defined(Q_OS_DARWIN) || defined(Q_CLANG_QDOC)
Q_FORWARD_DECLARE_CF_TYPE(CFUUID);
Q_FORWARD_DECLARE_OBJC_CLASS(NSUUID);
#endif

QT_BEGIN_NAMESPACE


class Q_CORE_EXPORT QUuid
{
    QUuid(Qt::Initialization) {}
public:
    enum Variant {
        VarUnknown        =-1,
        NCS                = 0, // 0 - -
        DCE                = 2, // 1 0 -
        Microsoft        = 6, // 1 1 0
        Reserved        = 7  // 1 1 1
    };

    enum Version {
        VerUnknown        =-1,
        Time                = 1, // 0 0 0 1
        EmbeddedPOSIX        = 2, // 0 0 1 0
        Md5                 = 3, // 0 0 1 1
        Name = Md5,
        Random                = 4,  // 0 1 0 0
        Sha1                 = 5 // 0 1 0 1
    };

    enum StringFormat {
        WithBraces      = 0,
        WithoutBraces   = 1,
        Id128           = 3
    };

#if defined(Q_COMPILER_UNIFORM_INIT) && !defined(Q_CLANG_QDOC)

    Q_DECL_CONSTEXPR QUuid() noexcept : data1(0), data2(0), data3(0), data4{0,0,0,0,0,0,0,0} {}

    Q_DECL_CONSTEXPR QUuid(uint l, ushort w1, ushort w2, uchar b1, uchar b2, uchar b3,
                           uchar b4, uchar b5, uchar b6, uchar b7, uchar b8) noexcept
        : data1(l), data2(w1), data3(w2), data4{b1, b2, b3, b4, b5, b6, b7, b8} {}
#else
    QUuid() noexcept
    {
        data1 = 0;
        data2 = 0;
        data3 = 0;
        for(int i = 0; i < 8; i++)
            data4[i] = 0;
    }
    QUuid(uint l, ushort w1, ushort w2, uchar b1, uchar b2, uchar b3, uchar b4, uchar b5, uchar b6, uchar b7, uchar b8) noexcept
    {
        data1 = l;
        data2 = w1;
        data3 = w2;
        data4[0] = b1;
        data4[1] = b2;
        data4[2] = b3;
        data4[3] = b4;
        data4[4] = b5;
        data4[5] = b6;
        data4[6] = b7;
        data4[7] = b8;
    }
#endif

    QUuid(const QString &);
    static QUuid fromString(QStringView string) noexcept;
    static QUuid fromString(QLatin1String string) noexcept;
    QUuid(const char *);
    QString toString() const;
    QString toString(StringFormat mode) const; // ### Qt6: merge with previous
    QUuid(const QByteArray &);
    QByteArray toByteArray() const;
    QByteArray toByteArray(StringFormat mode) const; // ### Qt6: merge with previous
    QByteArray toRfc4122() const;
    static QUuid fromRfc4122(const QByteArray &);
    bool isNull() const noexcept;

    Q_DECL_RELAXED_CONSTEXPR bool operator==(const QUuid &orig) const noexcept
    {
        if (data1 != orig.data1 || data2 != orig.data2 ||
             data3 != orig.data3)
            return false;

        for (uint i = 0; i < 8; i++)
            if (data4[i] != orig.data4[i])
                return false;

        return true;
    }

    Q_DECL_RELAXED_CONSTEXPR bool operator!=(const QUuid &orig) const noexcept
    {
        return !(*this == orig);
    }

    bool operator<(const QUuid &other) const noexcept;
    bool operator>(const QUuid &other) const noexcept;

#if defined(Q_OS_WIN) || defined(Q_CLANG_QDOC)
    // On Windows we have a type GUID that is used by the platform API, so we
    // provide convenience operators to cast from and to this type.
#if defined(Q_COMPILER_UNIFORM_INIT) && !defined(Q_CLANG_QDOC)
    Q_DECL_CONSTEXPR QUuid(const GUID &guid) noexcept
        : data1(guid.Data1), data2(guid.Data2), data3(guid.Data3),
          data4{guid.Data4[0], guid.Data4[1], guid.Data4[2], guid.Data4[3],
                guid.Data4[4], guid.Data4[5], guid.Data4[6], guid.Data4[7]} {}
#else
    QUuid(const GUID &guid) noexcept
    {
        data1 = guid.Data1;
        data2 = guid.Data2;
        data3 = guid.Data3;
        for(int i = 0; i < 8; i++)
            data4[i] = guid.Data4[i];
    }
#endif

    Q_DECL_RELAXED_CONSTEXPR QUuid &operator=(const GUID &guid) noexcept
    {
        *this = QUuid(guid);
        return *this;
    }

    Q_DECL_RELAXED_CONSTEXPR operator GUID() const noexcept
    {
        GUID guid = { data1, data2, data3, { data4[0], data4[1], data4[2], data4[3], data4[4], data4[5], data4[6], data4[7] } };
        return guid;
    }

    Q_DECL_RELAXED_CONSTEXPR bool operator==(const GUID &guid) const noexcept
    {
        return *this == QUuid(guid);
    }

    Q_DECL_RELAXED_CONSTEXPR bool operator!=(const GUID &guid) const noexcept
    {
        return !(*this == guid);
    }
#endif
    static QUuid createUuid();
#ifndef QT_BOOTSTRAPPED
    static QUuid createUuidV3(const QUuid &ns, const QByteArray &baseData);
#endif
    static QUuid createUuidV5(const QUuid &ns, const QByteArray &baseData);
#ifndef QT_BOOTSTRAPPED
    static inline QUuid createUuidV3(const QUuid &ns, const QString &baseData)
    {
        return QUuid::createUuidV3(ns, baseData.toUtf8());
    }
#endif

    static inline QUuid createUuidV5(const QUuid &ns, const QString &baseData)
    {
        return QUuid::createUuidV5(ns, baseData.toUtf8());
    }


    QUuid::Variant variant() const noexcept;
    QUuid::Version version() const noexcept;

#if defined(Q_OS_DARWIN) || defined(Q_CLANG_QDOC)
    static QUuid fromCFUUID(CFUUIDRef uuid);
    CFUUIDRef toCFUUID() const Q_DECL_CF_RETURNS_RETAINED;
    static QUuid fromNSUUID(const NSUUID *uuid);
    NSUUID *toNSUUID() const Q_DECL_NS_RETURNS_AUTORELEASED;
#endif

    uint    data1;
    ushort  data2;
    ushort  data3;
    uchar   data4[8];
};

Q_DECLARE_TYPEINFO(QUuid, Q_PRIMITIVE_TYPE);

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QUuid &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QUuid &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QUuid &);
#endif

Q_CORE_EXPORT uint qHash(const QUuid &uuid, uint seed = 0) noexcept;

inline bool operator<=(const QUuid &lhs, const QUuid &rhs) noexcept
{ return !(rhs < lhs); }
inline bool operator>=(const QUuid &lhs, const QUuid &rhs) noexcept
{ return !(lhs < rhs); }

QT_END_NAMESPACE

#endif // QUUID_H
