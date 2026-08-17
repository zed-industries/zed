// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QJSONDOCUMENT_H
#define QJSONDOCUMENT_H

#include <QtCore/qjsonvalue.h>
#include <QtCore/qscopedpointer.h>

#include <memory>

QT_BEGIN_NAMESPACE

class QDebug;
class QCborValue;

namespace QJsonPrivate { class Parser; }

struct Q_CORE_EXPORT QJsonParseError
{
    enum ParseError {
        NoError = 0,
        UnterminatedObject,
        MissingNameSeparator,
        UnterminatedArray,
        MissingValueSeparator,
        IllegalValue,
        TerminationByNumber,
        IllegalNumber,
        IllegalEscapeSequence,
        IllegalUTF8String,
        UnterminatedString,
        MissingObject,
        DeepNesting,
        DocumentTooLarge,
        GarbageAtEnd
    };

    QString    errorString() const;

    int        offset = -1;
    ParseError error = NoError;
};

class QJsonDocumentPrivate;
class Q_CORE_EXPORT QJsonDocument
{
public:
#ifdef Q_LITTLE_ENDIAN
    static const uint BinaryFormatTag = ('q') | ('b' << 8) | ('j' << 16) | ('s' << 24);
#else
    static const uint BinaryFormatTag = ('q' << 24) | ('b' << 16) | ('j' << 8) | ('s');
#endif

    QJsonDocument();
    explicit QJsonDocument(const QJsonObject &object);
    explicit QJsonDocument(const QJsonArray &array);
    ~QJsonDocument();

    QJsonDocument(const QJsonDocument &other);
    QJsonDocument &operator =(const QJsonDocument &other);

    QJsonDocument(QJsonDocument &&other) noexcept;

    QJsonDocument &operator =(QJsonDocument &&other) noexcept
    {
        swap(other);
        return *this;
    }

    void swap(QJsonDocument &other) noexcept;

    static QJsonDocument fromVariant(const QVariant &variant);
    QVariant toVariant() const;

    enum JsonFormat {
        Indented,
        Compact
    };

    static QJsonDocument fromJson(const QByteArray &json, QJsonParseError *error = nullptr);

#if !defined(QT_JSON_READONLY) || defined(Q_CLANG_QDOC)
    QByteArray toJson(JsonFormat format = Indented) const;
#endif

    bool isEmpty() const;
    bool isArray() const;
    bool isObject() const;

    QJsonObject object() const;
    QJsonArray array() const;

    void setObject(const QJsonObject &object);
    void setArray(const QJsonArray &array);

    const QJsonValue operator[](const QString &key) const;
    const QJsonValue operator[](QStringView key) const;
    const QJsonValue operator[](QLatin1StringView key) const;
    const QJsonValue operator[](qsizetype i) const;

    bool operator==(const QJsonDocument &other) const;
    bool operator!=(const QJsonDocument &other) const { return !(*this == other); }

    bool isNull() const;

private:
    friend class QJsonValue;
    friend class QJsonPrivate::Parser;
    friend Q_CORE_EXPORT QDebug operator<<(QDebug, const QJsonDocument &);

    QJsonDocument(const QCborValue &data);

    std::unique_ptr<QJsonDocumentPrivate> d;
};

Q_DECLARE_SHARED(QJsonDocument)

#if !defined(QT_NO_DEBUG_STREAM) && !defined(QT_JSON_READONLY)
Q_CORE_EXPORT QDebug operator<<(QDebug, const QJsonDocument &);
#endif

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QJsonDocument &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QJsonDocument &);
#endif

QT_END_NAMESPACE

#endif // QJSONDOCUMENT_H
