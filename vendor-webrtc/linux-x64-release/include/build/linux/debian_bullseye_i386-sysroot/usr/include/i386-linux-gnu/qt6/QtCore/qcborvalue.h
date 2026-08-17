// Copyright (C) 2022 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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

/* X11 headers use these values too, but as defines */
#if defined(False) && defined(True)
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
    QCborValue(const QString &s);
    QCborValue(QStringView s);
    QCborValue(QLatin1StringView s);
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
#  if QT_CONFIG(regularexpression)
    explicit QCborValue(const QRegularExpression &rx);
#  endif
    explicit QCborValue(const QUuid &uuid);
#endif

    ~QCborValue() { if (container) dispose(); }

    // make sure const char* doesn't go call the bool constructor
    QCborValue(const void *) = delete;

    QCborValue(const QCborValue &other);
    QCborValue(QCborValue &&other) noexcept
        : n(other.n), container(qExchange(other.container, nullptr)), t(qExchange(other.t, Undefined))
    {
    }
    QCborValue &operator=(const QCborValue &other);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QCborValue)

    void swap(QCborValue &other) noexcept
    {
        std::swap(n, other.n);
        qt_ptr_swap(container, other.container);
        std::swap(t, other.t);
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
#ifndef QT_BOOTSTRAPPED
    QUrl toUrl(const QUrl &defaultValue = {}) const;
#  if QT_CONFIG(regularexpression)
    QRegularExpression toRegularExpression(const QRegularExpression &defaultValue = {}) const;
#  endif
    QUuid toUuid(const QUuid &defaultValue = {}) const;
#endif

    // only forward-declared, need split functions
    QCborArray toArray() const;
    QCborArray toArray(const QCborArray &defaultValue) const;
    QCborMap toMap() const;
    QCborMap toMap(const QCborMap &defaultValue) const;

    const QCborValue operator[](const QString &key) const;
    const QCborValue operator[](QLatin1StringView key) const;
    const QCborValue operator[](qint64 key) const;
    QCborValueRef operator[](qint64 key);
    QCborValueRef operator[](QLatin1StringView key);
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
    QByteArray toCbor(EncodingOptions opt = NoTransformation) const;
    void toCbor(QCborStreamWriter &writer, EncodingOptions opt = NoTransformation) const;
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
        static_assert(sizeof(double) == sizeof(n));
        double d;
        memcpy(&d, &n, sizeof(d));
        return d;
    }

    constexpr static Type type_helper(QCborSimpleType st)
    {
        return Type(quint8(st) | SimpleType);
    }

    constexpr static bool isTag_helper(Type tt)
    {
        return tt == Tag || tt >= 0x10000;
    }
};
Q_DECLARE_SHARED(QCborValue)

class QCborValueConstRef
{
public:
    QCborValueConstRef(const QCborValueConstRef &) = default;
    QCborValueConstRef &operator=(const QCborValueConstRef &) = delete;
    operator QCborValue() const     { return concrete(); }

    QCborValue::Type type() const   { return concreteType(*this); }
    bool isInteger() const          { return type() == QCborValue::Integer; }
    bool isByteArray() const        { return type() == QCborValue::ByteArray; }
    bool isString() const           { return type() == QCborValue::String; }
    bool isArray() const            { return type() == QCborValue::Array; }
    bool isMap() const              { return type() == QCborValue::Map; }
    bool isTag() const              { return concrete().isTag(); }
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
    bool isSimpleType() const       { return concrete().isSimpleType(); }
    bool isSimpleType(QCborSimpleType st) const { return concrete().isSimpleType(st); }

    QCborSimpleType toSimpleType(QCborSimpleType defaultValue = QCborSimpleType::Undefined) const
    {
        return concrete().toSimpleType(defaultValue);
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
#  if QT_CONFIG(regularexpression)
    QRegularExpression toRegularExpression(const QRegularExpression &defaultValue = {}) const
    { return concrete().toRegularExpression(defaultValue); }
#  endif
    QUuid toUuid(const QUuid &defaultValue = {}) const
    { return concrete().toUuid(defaultValue); }
#endif

    // only forward-declared, need split functions. Implemented in qcbor{array,map}.h
    inline QCborArray toArray() const;
    inline QCborArray toArray(const QCborArray &a) const;
    inline QCborMap toMap() const;
    inline QCborMap toMap(const QCborMap &m) const;

    Q_CORE_EXPORT const QCborValue operator[](const QString &key) const;
    Q_CORE_EXPORT const QCborValue operator[](QLatin1StringView key) const;
    Q_CORE_EXPORT const QCborValue operator[](qint64 key) const;

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
    inline QJsonValue toJsonValue() const;      // in qjsonvalue.h

#if QT_CONFIG(cborstreamwriter)
    QByteArray toCbor(QCborValue::EncodingOptions opt = QCborValue::NoTransformation) const
    { return concrete().toCbor(opt); }
    void toCbor(QCborStreamWriter &writer, QCborValue::EncodingOptions opt = QCborValue::NoTransformation) const
    { return concrete().toCbor(writer, opt); }
#endif

    QString toDiagnosticNotation(QCborValue::DiagnosticNotationOptions opt = QCborValue::Compact) const
    { return concrete().toDiagnosticNotation(opt); }

protected:
    friend class QCborValue;
    friend class QCborArray;
    friend class QCborMap;
    friend class QCborContainerPrivate;

    QCborValue concrete() const noexcept  { return concrete(*this); }

    static Q_CORE_EXPORT QCborValue concrete(QCborValueConstRef that) noexcept;
    static Q_CORE_EXPORT QCborValue::Type concreteType(QCborValueConstRef that) noexcept Q_DECL_PURE_FUNCTION;
    static Q_CORE_EXPORT bool
    concreteBoolean(QCborValueConstRef that, bool defaultValue) noexcept Q_DECL_PURE_FUNCTION;
    static Q_CORE_EXPORT double
    concreteDouble(QCborValueConstRef that, double defaultValue) noexcept Q_DECL_PURE_FUNCTION;
    static Q_CORE_EXPORT qint64
    concreteIntegral(QCborValueConstRef that, qint64 defaultValue) noexcept Q_DECL_PURE_FUNCTION;
    static Q_CORE_EXPORT QByteArray
    concreteByteArray(QCborValueConstRef that, const QByteArray &defaultValue);
    static Q_CORE_EXPORT QString
    concreteString(QCborValueConstRef that, const QString &defaultValue);

    constexpr QCborValueConstRef() : d(nullptr), i(0) {} // this will actually be invalid
    constexpr QCborValueConstRef(QCborContainerPrivate *dd, qsizetype ii)
        : d(dd), i(ii)
    {}
    QCborContainerPrivate *d;
    qsizetype i;
};

class QT6_ONLY(Q_CORE_EXPORT) QCborValueRef : public QCborValueConstRef
{
public:
    QCborValueRef(const QCborValueRef &) noexcept = default;
    QCborValueRef(QCborValueRef &&) noexcept = default;
    QCborValueRef &operator=(const QCborValue &other)
    { assign(*this, other); return *this; }
    QCborValueRef &operator=(QCborValue &&other)
    { assign(*this, std::move(other)); other.container = nullptr; return *this; }
    QCborValueRef &operator=(const QCborValueRef &other)
    { assign(*this, other); return *this; }

    QT7_ONLY(Q_CORE_EXPORT) QCborValueRef operator[](qint64 key);
    QT7_ONLY(Q_CORE_EXPORT) QCborValueRef operator[](QLatin1StringView key);
    QT7_ONLY(Q_CORE_EXPORT) QCborValueRef operator[](const QString & key);

#if QT_VERSION < QT_VERSION_CHECK(7, 0, 0) && !defined(QT_BOOTSTRAPPED)
    // retained for binary compatibility (due to the Q_CORE_EXPORT) because at
    // least one compiler emits and exports all inlines in an exported class

    operator QCborValue() const     { return concrete(); }
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
    QCborSimpleType toSimpleType(QCborSimpleType defaultValue = QCborSimpleType::Undefined) const
    {
        return isSimpleType() ? QCborSimpleType(type() & 0xff) : defaultValue;
    }

    QCborTag tag(QCborTag defaultValue = QCborTag(-1)) const
    { return concrete().tag(defaultValue); }
    QCborValue taggedValue(const QCborValue &defaultValue = QCborValue()) const
    { return concrete().taggedValue(defaultValue); }

    qint64 toInteger(qint64 defaultValue = 0) const
    { return concreteIntegral(*this, defaultValue); }
    bool toBool(bool defaultValue = false) const
    { return concreteBoolean(*this, defaultValue); }
    double toDouble(double defaultValue = 0) const
    { return concreteDouble(*this, defaultValue); }

    QByteArray toByteArray(const QByteArray &defaultValue = {}) const
    { return concreteByteArray(*this, defaultValue); }
    QString toString(const QString &defaultValue = {}) const
    { return concreteString(*this, defaultValue); }
    QDateTime toDateTime(const QDateTime &defaultValue = {}) const
    { return concrete().toDateTime(defaultValue); }
#ifndef QT_BOOTSTRAPPED
    QUrl toUrl(const QUrl &defaultValue = {}) const
    { return concrete().toUrl(defaultValue); }
#  if QT_CONFIG(regularexpression)
    QRegularExpression toRegularExpression(const QRegularExpression &defaultValue = {}) const
    { return concrete().toRegularExpression(defaultValue); }
#  endif
    QUuid toUuid(const QUuid &defaultValue = {}) const
    { return concrete().toUuid(defaultValue); }
#endif

    // only forward-declared, need split functions. Implemented in qcbor{array,map}.h
    QCborArray toArray() const;
    QCborArray toArray(const QCborArray &a) const;
    QCborMap toMap() const;
    QCborMap toMap(const QCborMap &m) const;

    const QCborValue operator[](const QString &key) const;
    const QCborValue operator[](QLatin1StringView key) const;
    const QCborValue operator[](qint64 key) const;

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
    using QCborValueConstRef::toCbor;
    QByteArray toCbor(QCborValue::EncodingOptions opt = QCborValue::NoTransformation)
    { return std::as_const(*this).toCbor(opt); }
    void toCbor(QCborStreamWriter &writer, QCborValue::EncodingOptions opt = QCborValue::NoTransformation);
#endif

    using QCborValueConstRef::toDiagnosticNotation;
    QString toDiagnosticNotation(QCborValue::DiagnosticNotationOptions opt = QCborValue::Compact)
    { return std::as_const(*this).toDiagnosticNotation(opt); }

private:
    static QCborValue concrete(QCborValueRef that) noexcept;
    QCborValue concrete() const noexcept  { return concrete(*this); }

    static QCborValue::Type concreteType(QCborValueRef self) noexcept Q_DECL_PURE_FUNCTION;
    QCborValue::Type concreteType() const noexcept { return concreteType(*this); }

    // this will actually be invalid...
    constexpr QCborValueRef() : QCborValueConstRef(nullptr, 0) {}

    QCborValueRef(QCborContainerPrivate *dd, qsizetype ii)
        : QCborValueConstRef(dd, ii)
    {}
#else
private:
    using QCborValueConstRef::QCborValueConstRef;
#endif // < Qt 7

    friend class QCborValue;
    friend class QCborArray;
    friend class QCborMap;
    friend class QCborContainerPrivate;
    friend class QCborValueConstRef;

    // static so we can pass this by value
    QT7_ONLY(Q_CORE_EXPORT) static void assign(QCborValueRef that, const QCborValue &other);
    QT7_ONLY(Q_CORE_EXPORT) static void assign(QCborValueRef that, QCborValue &&other);
    QT7_ONLY(Q_CORE_EXPORT) static void assign(QCborValueRef that, const QCborValueRef other);
};

Q_CORE_EXPORT size_t qHash(const QCborValue &value, size_t seed = 0);

#if !defined(QT_NO_DEBUG_STREAM)
Q_CORE_EXPORT QDebug operator<<(QDebug, const QCborValue &v);
#endif

#ifndef QT_NO_DATASTREAM
#if QT_CONFIG(cborstreamwriter)
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QCborValue &);
#endif
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QCborValue &);
#endif

QT_END_NAMESPACE

#if defined(QT_X11_DEFINES_FOUND)
#  define True  1
#  define False 0
#endif

#endif // QCBORVALUE_H
