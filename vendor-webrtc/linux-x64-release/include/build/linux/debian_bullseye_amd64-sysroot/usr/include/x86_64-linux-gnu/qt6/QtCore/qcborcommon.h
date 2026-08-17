// Copyright (C) 2018 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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

#if !defined(QT_NO_DATASTREAM)
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &ds, QCborSimpleType st);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &ds, QCborSimpleType &st);
#endif

inline size_t qHash(QCborSimpleType tag, size_t seed = 0)
{
    return qHash(quint8(tag), seed);
}

inline size_t qHash(QCborTag tag, size_t seed = 0)
{
    return qHash(quint64(tag), seed);
}

enum class QCborNegativeInteger : quint64 {};

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QCborTag, Q_CORE_EXPORT)

// To avoid changing namespace we need to reinstate defines, even though our .cpp
// will then have to remove them again.
#if defined(QT_X11_DEFINES_FOUND)
#  define True  1
#  define False 0
#endif

#endif // QCBORSTREAM_H
