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

#ifndef QCBORVALUE_H
#define QCBORVALUE_H

#include <QtCore/qbytearray.h>
#include <QtCore/qdatetime.h>
#include <QtCore/qcborcommon.h>
#if QT_CONFIG(regularexpression)
#  include <QtCore/qregularexpression.h>
#endif
#include <QtCore/qstring.h>
#include <QtCore/qstringview.h>
#include <QtCore/qurl.h>
#include <QtCore/quuid.h>
#include <QtCore/qvariant.h>
#include <QtCore/qvector.h>

// See qcborcommon.h for why we check
#if defined(QT_X11_DEFINES_FOUND)
#  undef True
#  undef False
#endif

#if 0 && __has_include(<compare>)
#  include <compare>
#endif

QT_BEGIN_NAMESPACE

class QCborArray;
class QCborMap;
class QCborStreamReader;
class QCborStreamWriter;
class QDataStream;

namespace QJsonPrivate { class Value; }

struct QCborParserError
{
    qint64 offset = 0;
    QCborError error = { QCborError::NoError };

    QString errorString() const { return error.toString(); }
};

class QCborValueRef;
class QCborContainerPrivate;
class Q_CORE_EXPORT QCborValue
{
    Q_GADGET
public:
    enum EncodingOption {
        SortKeysInMaps = 0x01,
        UseFloat = 0x02,
#ifndef QT_BOOTSTRAPPED
        UseFloat16 = UseFloat | 0x04,
#endif
        UseIntegers = 0x08,

        NoTransformation = 0
    };
    Q_DECLARE_FLAGS(EncodingOptions, EncodingOption)

    enum DiagnosticNotationOption {
        Compact         = 0x00,
        LineWrapped     = 0x01,
        ExtendedFormat  = 0x02
    };
    Q_DECLARE_FLAGS(DiagnosticNotationOptions, DiagnosticNotationOption)

    // different from QCborStreamReader::Type because we have more types
    enum Type : int {
        Integer         = 0x00,
        ByteArray       = 0x40,
        String          = 0x60,
        Array           = 0x80,
        Map             = 0xa0,
        Tag             = 0xc0,

        // range 0x100 - 0x1ff for Simple Types
        SimpleType      = 0x100,
        False           = SimpleType + int(QCborSimpleType::False),
        True            = SimpleType + int(QCborSimpleType::True),
        Null            = SimpleType + int(QCborSimpleType::Null),
        Undefined       = SimpleType + int(QCborSimpleType::Undefined),

        Double          = 0x202,

        // extended (tagged) types
        DateTime        = 0x10000,
        Url             = 0x10020,
        RegularExpression = 0x10023,
        Uuid            = 0x10025,

        Invalid         = -1
    };
    Q_ENUM(Type)

    QCborValue() {}
    QCborValue(Type t_) : t(t_) {}
    QCborValue(std::nullptr_t) : t(Null) {}
    QCborValue(bool b_) : t(b_ ? True : False) {}
#ifndef Q_QDOC
    QCborValue(int i) : QCborValue(qint64(i)) {}
    QCborValue(unsigned u) : QCborValue(qint64(u)) {}
#endif
    QCborValue(qint64 i) : n(i), t(Integer) {}
    QCborValue(double v) : t(Double) { memcpy(&n, &v, sizeof(n)); }
    QCborValue(QCborSimpleType st) : t(type_helper(st)) {}

    QCborValue(const QByteArray &ba);
#if QT_STRINGVIEW_LEVEL < 2
    QCborValue(const QString &s);
#endif
    QCborValue(QStringView s);
    QCborValue(QLatin1String s);
#ifndef QT_NO_CAST_FROM_ASCII
    QT_ASCII_CAST_WARN QCborValue(const char *s) : QCborValue(QString::fromUtf8(s)) {}
#endif
    QCborValue(const QCborArray &a);
    QCborValue(QCborArray &&a);
    QCborValue(const QCborMap &m);
    QCborValue(QCborMap &&m);
    QCborValue(QCborTag tag, const QCborValue &taggedValue = QCborValue());
    QCborValue(QCborKnownTags t_, const QCborValue &tv = QCborValue())
        : QCborValue(QCborTag(t_), tv)
    {}

    explicit QCborValue(const QDateTime &dt);
#ifndef QT_BOOTSTRAPPED
    explicit QCborValue(const QUrl &url);
#endif
#if QT_CONFIG(regularexpression)
    explicit QCborValue(const QRegularExpression &rx);
#endif
    explicit QCborValue(const QUuid &uuid);

    ~QCborValue() { if (container) dispose(); }

    // make sure const char* doesn't go call the bool constructor
    QCborValue(const void *) = delete;

    QCborValue(const QCborValue &other);
    QCborValue(QCborValue &&other) noexcept
        : n(other.n), container(other.container), t(other.t)
    {
        other.t = Undefined;
        other.container = nullptr;
    }
    QCborValue &operator=(const QCborValue &other);
    QCborValue &operator=(QCborValue &&other) noexcept
    {
        QCborValue tmp(std::move(other));
        swap(tmp);
        return *this;
    }

    void swap(QCborValue &other) noexcept
    {
        qSwap(n, other.n);
        qSwap(container, other.container);
        qSwap(t, other.t);
    }

    Type type() const           { return t; }
    bool isInteger() const      { return type() == Integer; }
    bool isByteArray() const    { return type() == ByteArray; }
    bool isString() const       { return type() == String; }
    bool isArray() const        { return type() == Array; }
    bool isMap() const          { return type() == Map; }
    bool isTag() const          { return isTag_helper(type()); }
    bool isFalse() const        { return type() == False; }
    bool isTrue() const         { return type() == True; }
    bool isBool() const         { return isFalse() || isTrue(); }
    bool isNull() const         { return type() == Null; }
    bool isUndefined() const    { return type() == Undefined; }
    bool isDouble() const       { return type() == Double; }
    bool isDateTime() const     { return type() == DateTime; }
    bool isUrl() const          { return type() == Url; }
    bool isRegularExpression() const { return type() == RegularExpression; }
    bool isUuid() const         { return type() == Uuid; }
    bool isInvalid() const      { return type() == Invalid; }
    bool isContainer() const    { return isMap() || isArray(); }

    bool isSimpleType() const
    {
        return int(type()) >> 8 == int(SimpleType) >> 8;
    }
    bool isSimpleType(QCborSimpleType st) const
    {
        return type() == type_helper(st);
    }
    QCborSimpleType toSimpleType(QCborSimpleType defaultValue = QCborSimpleType::Undefined) const
    {
        return isSimpleType() ? QCborSimpleType(type() & 0xff) : defaultValue;
    }

    qint64 toInteger(qint64 defaultValue = 0) const
    { return isInteger() ? value_helper() : isDouble() ? qint64(fp_helper()) : defaultValue; }
    bool toBool(bool defaultValue = false) const
    { return isBool() ? isTrue() : defaultValue; }
    double toDouble(double defaultValue = 0) const
    { return isDouble() ? fp_helper() : isInteger() ? double(value_helper()) : defaultValue; }

    QCborTag tag(QCborTag defaultValue = QCborTag(-1)) const;
    QCborValue taggedValue(const QCborValue &defaultValue = QCborValue()) const;

    QByteArray toByteArray(const QByteArray &defaultValue = {}) const;
    QString toString(const QString &defaultValue = {}) const;
    QDateTime toDateTime(const QDateTime &defaultValue = {}) const;
    QUrl toUrl(const QUrl &defaultValue = {}) const;
#if QT_CONFIG(regularexpression)
    QRegularExpression toRegularExpression(const QRegularExpression &defaultValue = {}) const;
#endif
    QUuid toUuid(const QUuid &defaultValue = {}) const;

    // only forward-declared, need split functions
    QCborArray toArray() const;
    QCborArray toArray(const QCborArray &defaultValue) const;
    QCborMap toMap() const;
    QCborMap toMap(const QCborMap &defaultValue) const;

    const QCborValue operator[](const QString &key) const;
    const QCborValue operator[](QLatin1String key) const;
    const QCborValue operator[](qint64 key) const;
    QCborValueRef operator[](qint64 key);
    QCborValueRef operator[](QLatin1String key);
    QCborValueRef operator[](const QString & key);

    int compare(const QCborValue &other) const;
#if 0 && __has_include(<compare>)
    std::strong_ordering operator<=>(const QCborValue &other) const
    {
        int c = compare(other);
        if (c > 0) return std::partial_ordering::greater;
        if (c == 0) return std::partial_ordering::equivalent;
        return std::partial_ordering::less;
    }
#else
    bool operator==(const QCborValue &other) const noexcept
    { return compare(other) == 0; }
    bool operator!=(const QCborValue &other) const noexcept
    { return !(*this == other); }
    bool operator<(const QCborValue &other) const
    { return compare(other) < 0; }
#endif

    static QCborValue fromVariant(const QVariant &variant);
    QVariant toVariant() const;
    static QCborValue fromJsonValue(const QJsonValue &v);
    QJsonValue toJsonValue() const;

#if QT_CONFIG(cborstreamreader)
    static QCborValue fromCbor(QCborStreamReader &reader);
    static QCborValue fromCbor(const QByteArray &ba, QCborParserError *error = nullptr);
    static QCborValue fromCbor(const char *data, qsizetype len, QCborParserError *error = nullptr)
    { return fromCbor(QByteArray(data, int(len)), error); }
    static QCborValue fromCbor(const quint8 *data, qsizetype len, QCborParserError *error = nullptr)
    { return fromCbor(QByteArray(reinterpret_cast<const char *>(data), int(len)), error); }
#endif // QT_CONFIG(cborstreamreader)
#if QT_CONFIG(cborstreamwriter)
    QByteArray toCbor(EncodingOptions opt = NoTransformation);
    void toCbor(QCborStreamWriter &writer, EncodingOptions opt = NoTransformation);
#endif

    QString toDiagnosticNotation(DiagnosticNotationOptions opts = Compact) const;

private:
    friend class QCborValueRef;
    friend class QCborContainerPrivate;
    friend class QJsonPrivate::Value;

    qint64 n = 0;
    QCborContainerPrivate *container = nullptr;
    Type t = Undefined;

    void dispose();
    qint64 value_helper() const
    {
        return n;
    }

    double fp_helper() const
    {
        Q_STATIC_ASSERT(sizeof(double) == sizeof(n));
        double d;
        memcpy(&d, &n, sizeof(d));
        return d;
    }

    Q_DECL_CONSTEXPR static Type type_helper(QCborSimpleType st)
    {
        return Type(quint8(st) | SimpleType);
    }

    Q_DECL_CONSTEXPR static bool isTag_helper(Type tt)
    {
        return tt == Tag || tt >= 0x10000;
    }
};
Q_DECLARE_SHARED(QCborValue)

class Q_CORE_EXPORT QCborValueRef
{
public:
    operator QCborValue() const     { return concrete(); }

    QCborValueRef(const QCborValueRef &) noexcept = default;
    QCborValueRef(QCborValueRef &&) noexcept = default;
    QCborValueRef &operator=(const QCborValue &other)
    { assign(*this, other); return *this; }
    QCborValueRef &operator=(QCborValue &&other)
    { assign(*this, std::move(other)); other.container = nullptr; return *this; }
    QCborValueRef &operator=(const QCborValueRef &other)
    { assign(*this, other); return *this; }

    QCborValue::Type type() const   { return concreteType(); }
    bool isInteger() const          { return type() == QCborValue::Integer; }
    bool isByteArray() const        { return type() == QCborValue::ByteArray; }
    bool isString() const           { return type() == QCborValue::String; }
    bool isArray() const            { return type() == QCborValue::Array; }
    bool isMap() const              { return type() == QCborValue::Map; }
    bool isTag() const              { return QCborValue::isTag_helper(type()); }
    bool isFalse() const            { return type() == QCborValue::False; }
    bool isTrue() const             { return type() == QCborValue::True; }
    bool isBool() const             { return isFalse() || isTrue(); }
    bool isNull() const             { return type() == QCborValue::Null; }
    bool isUndefined() const        { return type() == QCborValue::Undefined; }
    bool isDouble() const           { return type() == QCborValue::Double; }
    bool isDateTime() const         { return type() == QCborValue::DateTime; }
    bool isUrl() const              { return type() == QCborValue::Url; }
    bool isRegularExpression() const { return type() == QCborValue::RegularExpression; }
    bool isUuid() const             { return type() == QCborValue::Uuid; }
    bool isInvalid() const          { return type() == QCborValue::Invalid; }
    bool isContainer() const        { return isMap() || isArray(); }
    bool isSimpleType() const
    {
        return type() >= QCborValue::SimpleType && type() < QCborValue::SimpleType + 0x100;
    }
    bool isSimpleType(QCborSimpleType st) const
    {
        return type() == QCborValue::type_helper(st);
    }

    QCborTag tag(QCborTag defaultValue = QCborTag(-1)) const
    { return concrete().tag(defaultValue); }
    QCborValue taggedValue(const QCborValue &defaultValue = QCborValue()) const
    { return concrete().taggedValue(defaultValue); }

    qint64 toInteger(qint64 defaultValue = 0) const
    { return concrete().toInteger(defaultValue); }
    bool toBool(bool defaultValue = false) const
    { return concrete().toBool(defaultValue); }
    double toDouble(double defaultValue = 0) const
    { return concrete().toDouble(defaultValue); }

    QByteArray toByteArray(const QByteArray &defaultValue = {}) const
    { return concrete().toByteArray(defaultValue); }
    QString toString(const QString &defaultValue = {}) const
    { return concrete().toString(defaultValue); }
    QDateTime toDateTime(const QDateTime &defaultValue = {}) const
    { return concrete().toDateTime(defaultValue); }
#ifndef QT_BOOTSTRAPPED
    QUrl toUrl(const QUrl &defaultValue = {}) const
    { return concrete().toUrl(defaultValue); }
#endif
#if QT_CONFIG(regularexpression)
    QRegularExpression toRegularExpression(const QRegularExpression &defaultValue = {}) const
    { return concrete().toRegularExpression(defaultValue); }
#endif
    QUuid toUuid(const QUuid &defaultValue = {}) const
    { return concrete().toUuid(defaultValue); }

    // only forward-declared, need split functions. Implemented in qcbor{array,map}.h
    QCborArray toArray() const;
    QCborArray toArray(const QCborArray &a) const;
    QCborMap toMap() const;
    QCborMap toMap(const QCborMap &m) const;

    const QCborValue operator[](const QString &key) const;
    const QCborValue operator[](QLatin1String key) const;
    const QCborValue operator[](qint64 key) const;
    QCborValueRef operator[](qint64 key);
    QCborValueRef operator[](QLatin1String key);
    QCborValueRef operator[](const QString & key);

    int compare(const QCborValue &other) const
    { return concrete().compare(other); }
#if 0 && __has_include(<compare>)
    std::strong_ordering operator<=>(const QCborValue &other) const
    {
        int c = compare(other);
        if (c > 0) return std::strong_ordering::greater;
        if (c == 0) return std::strong_ordering::equivalent;
        return std::strong_ordering::less;
    }
#else
    bool operator==(const QCborValue &other) const
    { return compare(other) == 0; }
    bool operator!=(const QCborValue &other) const
    { return !(*this == other); }
    bool operator<(const QCborValue &other) const
    { return compare(other) < 0; }
#endif

    QVariant toVariant() const                  { return concrete().toVariant(); }
    QJsonValue toJsonValue() const;

#if QT_CONFIG(cborstreamwriter)
    QByteArray toCbor(QCborValue::EncodingOptions opt = QCborValue::NoTransformation)
    { return concrete().toCbor(opt); }
    void toCbor(QCborStreamWriter &writer, QCborValue::EncodingOptions opt = QCborValue::NoTransformation);
#endif

    QString toDiagnosticNotation(QCborValue::DiagnosticNotationOptions opt = QCborValue::Compact)
    { return concrete().toDiagnosticNotation(opt); }

private:
    friend class QCborValue;
    friend class QCborArray;
    friend class QCborMap;
    friend class QCborContainerPrivate;
    friend class QCborValueRefPtr;

    // static so we can pass this by value
    static void assign(QCborValueRef that, const QCborValue &other);
    static void assign(QCborValueRef that, QCborValue &&other);
    static void assign(QCborValueRef that, const QCborValueRef other);
    static QCborValue concrete(QCborValueRef that) noexcept;
    QCborValue concrete() const noexcept  { return concrete(*this); }

    static QCborValue::Type concreteType(QCborValueRef self) noexcept Q_DECL_PURE_FUNCTION;
    QCborValue::Type concreteType() const noexcept { return concreteType(*this); }

    // this will actually be invalid...
    Q_DECL_CONSTEXPR QCborValueRef() : d(nullptr), i(0) {}

    QCborValueRef(QCborContainerPrivate *dd, qsizetype ii)
        : d(dd), i(ii)
    {}
    QCborContainerPrivate *d;
    qsizetype i;
};

Q_CORE_EXPORT uint qHash(const QCborValue &value, uint seed = 0);

#if !defined(QT_NO_DEBUG_STREAM)
Q_CORE_EXPORT QDebug operator<<(QDebug, const QCborValue &v);
#endif

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QCborValue &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QCborValue &);
#endif

QT_END_NAMESPACE

#if defined(QT_X11_DEFINES_FOUND)
#  define True  1
#  define False 0
#endif

#endif // QCBORVALUE_H
