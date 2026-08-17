// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTESTDATA_H
#define QTESTDATA_H

#include <QtTest/qttestglobal.h>

#include <QtCore/qmetatype.h>
#include <QtCore/qstring.h>

QT_BEGIN_NAMESPACE


class QTestTable;
class QTestDataPrivate;

class Q_TESTLIB_EXPORT QTestData
{
public:
    ~QTestData();

    void append(int type, const void *data);
    void *data(int index) const;
    const char *dataTag() const;
    QTestTable *parent() const;
    int dataCount() const;

private:
    friend class QTestTable;
    QTestData(const char *tag, QTestTable *parent);

    Q_DISABLE_COPY(QTestData)

    QTestDataPrivate *d;
};

template<typename T>
QTestData &operator<<(QTestData &data, const T &value)
{
    data.append(qMetaTypeId<T>(), &value);
    return data;
}

inline QTestData &operator<<(QTestData &data, const char * value)
{
    QString str = QString::fromUtf8(value);
    data.append(QMetaType::QString, &str);
    return data;
}

#ifdef __cpp_char8_t
Q_WEAK_OVERLOAD
inline QTestData &operator<<(QTestData &data, const char8_t *value)
{
    return data << reinterpret_cast<const char *>(value);
}
#endif

#ifdef QT_USE_QSTRINGBUILDER
template<typename A, typename B>
inline QTestData &operator<<(QTestData &data, const QStringBuilder<A, B> &value)
{
    return data << typename QConcatenable<QStringBuilder<A, B> >::ConvertTo(value);
}
#endif

QT_END_NAMESPACE

#endif
