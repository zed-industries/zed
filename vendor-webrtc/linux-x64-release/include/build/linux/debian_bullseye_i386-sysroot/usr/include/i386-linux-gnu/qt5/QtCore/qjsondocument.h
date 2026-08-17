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

    int        offset;
    ParseError error;
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

    enum DataValidation {
        Validate,
        BypassValidation
    };

#if QT_CONFIG(binaryjson) && QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_X("Use CBOR format instead")
    static QJsonDocument fromRawData(const char *data, int size, DataValidation validation = Validate);

    QT_DEPRECATED_X("Use CBOR format instead")
    const char *rawData(int *size) const;

    QT_DEPRECATED_X("Use CBOR format instead")
    static QJsonDocument fromBinaryData(const QByteArray &data, DataValidation validation  = Validate);

    QT_DEPRECATED_X("Use CBOR format instead")
    QByteArray toBinaryData() const;
#endif // QT_CONFIG(binaryjson) && QT_DEPRECATED_SINCE(5, 15)

    static QJsonDocument fromVariant(const QVariant &variant);
    QVariant toVariant() const;

    enum JsonFormat {
        Indented,
        Compact
    };

    static QJsonDocument fromJson(const QByteArray &json, QJsonParseError *error = nullptr);

#if !defined(QT_JSON_READONLY) || defined(Q_CLANG_QDOC)
    QByteArray toJson() const; //### Merge in Qt6
    QByteArray toJson(JsonFormat format) const;
#endif

    bool isEmpty() const;
    bool isArray() const;
    bool isObject() const;

    QJsonObject object() const;
    QJsonArray array() const;

    void setObject(const QJsonObject &object);
    void setArray(const QJsonArray &array);

#if QT_STRINGVIEW_LEVEL < 2
    const QJsonValue operator[](const QString &key) const;
#endif
    const QJsonValue operator[](QStringView key) const;
    const QJsonValue operator[](QLatin1String key) const;
    const QJsonValue operator[](int i) const;

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

Q_DECLARE_SHARED_NOT_MOVABLE_UNTIL_QT6(QJsonDocument)

#if !defined(QT_NO_DEBUG_STREAM) && !defined(QT_JSON_READONLY)
Q_CORE_EXPORT QDebug operator<<(QDebug, const QJsonDocument &);
#endif

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QJsonDocument &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QJsonDocument &);
#endif

QT_END_NAMESPACE

#endif // QJSONDOCUMENT_H
