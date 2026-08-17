/****************************************************************************
**
** Copyright (C) 2018 Intel Corporation.
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

#ifndef QCBORCOMMON_H
#define QCBORCOMMON_H

#include <QtCore/qobjectdefs.h>
#include <QtCore/qmetatype.h>
#include <QtCore/qdebug.h>

#if 0
#pragma qt_class(QtCborCommon)
#endif

/* X11 headers use these values too, but as defines */
#if defined(False) && defined(True)
#  define QT_X11_DEFINES_FOUND 1
#  undef True
#  undef False
#endif

QT_BEGIN_NAMESPACE

enum class QCborSimpleType : quint8 {
    False = 20,
    True = 21,
    Null = 22,
    Undefined = 23
};

enum class QCborTag : quint64 {};
enum class QCborKnownTags {
    DateTimeString          = 0,
    UnixTime_t              = 1,
    PositiveBignum          = 2,
    NegativeBignum          = 3,
    Decimal                 = 4,
    Bigfloat                = 5,
    COSE_Encrypt0           = 16,
    COSE_Mac0               = 17,
    COSE_Sign1              = 18,
    ExpectedBase64url       = 21,
    ExpectedBase64          = 22,
    ExpectedBase16          = 23,
    EncodedCbor             = 24,
    Url                     = 32,
    Base64url               = 33,
    Base64                  = 34,
    RegularExpression       = 35,
    MimeMessage             = 36,
    Uuid                    = 37,
    COSE_Encrypt            = 96,
    COSE_Mac                = 97,
    COSE_Sign               = 98,
    Signature               = 55799
};

inline bool operator==(QCborTag t, QCborKnownTags kt)   { return quint64(t) == quint64(kt); }
inline bool operator==(QCborKnownTags kt, QCborTag t)   { return quint64(t) == quint64(kt); }
inline bool operator!=(QCborTag t, QCborKnownTags kt)   { return quint64(t) != quint64(kt); }
inline bool operator!=(QCborKnownTags kt, QCborTag t)   { return quint64(t) != quint64(kt); }

struct Q_CORE_EXPORT QCborError
{
    Q_GADGET
public:
    enum Code : int {
        UnknownError = 1,
        AdvancePastEnd = 3,
        InputOutputError = 4,
        GarbageAtEnd = 256,
        EndOfFile,
        UnexpectedBreak,
        UnknownType,
        IllegalType,
        IllegalNumber,
        IllegalSimpleType,

        InvalidUtf8String = 516,

        DataTooLarge = 1024,
        NestingTooDeep,
        UnsupportedType,

        NoError = 0
    };
    Q_ENUM(Code)

    Code c;
    operator Code() const { return c; }
    QString toString() const;
};

#if !defined(QT_NO_DEBUG_STREAM)
Q_CORE_EXPORT QDebug operator<<(QDebug, QCborSimpleType st);
Q_CORE_EXPORT QDebug operator<<(QDebug, QCborKnownTags tg);
Q_CORE_EXPORT QDebug operator<<(QDebug, QCborTag tg);
#endif

#if !defined(QT_NO_DEBUG_STREAM)
QDataStream &operator<<(QDataStream &ds, QCborSimpleType st);
QDataStream &operator>>(QDataStream &ds, QCborSimpleType &st);
#endif

inline uint qHash(QCborSimpleType tag, uint seed = 0)
{
    return qHash(quint8(tag), seed);
}

inline uint qHash(QCborTag tag, uint seed = 0)
{
    return qHash(quint64(tag), seed);
}

enum class QCborNegativeInteger : quint64 {};

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QCborTag)

// To avoid changing namespace we need to reinstate defines, even though our .cpp
// will then have to remove them again.
#if defined(QT_X11_DEFINES_FOUND)
#  define True  1
#  define False 0
#endif

#endif // QCBORSTREAM_H
