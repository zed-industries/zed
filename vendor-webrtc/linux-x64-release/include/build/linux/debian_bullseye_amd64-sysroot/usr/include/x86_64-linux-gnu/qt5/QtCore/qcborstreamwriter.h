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

#ifndef QCBORSTREAMWRITER_H
#define QCBORSTREAMWRITER_H

#include <QtCore/qbytearray.h>
#include <QtCore/qcborcommon.h>
#include <QtCore/qscopedpointer.h>
#include <QtCore/qstring.h>
#include <QtCore/qstringview.h>
#ifndef QT_BOOTSTRAPPED
#include <QtCore/qfloat16.h>
#endif

QT_REQUIRE_CONFIG(cborstreamwriter);

// See qcborcommon.h for why we check
#if defined(QT_X11_DEFINES_FOUND)
#  undef True
#  undef False
#endif

QT_BEGIN_NAMESPACE

class QIODevice;

class QCborStreamWriterPrivate;
class Q_CORE_EXPORT QCborStreamWriter
{
public:
    explicit QCborStreamWriter(QIODevice *device);
    explicit QCborStreamWriter(QByteArray *data);
    ~QCborStreamWriter();
    Q_DISABLE_COPY(QCborStreamWriter)

    void setDevice(QIODevice *device);
    QIODevice *device() const;

    void append(quint64 u);
    void append(qint64 i);
    void append(QCborNegativeInteger n);
    void append(const QByteArray &ba)       { appendByteString(ba.constData(), ba.size()); }
    void append(QLatin1String str);
    void append(QStringView str);
    void append(QCborTag tag);
    void append(QCborKnownTags tag)         { append(QCborTag(tag)); }
    void append(QCborSimpleType st);
    void append(std::nullptr_t)             { append(QCborSimpleType::Null); }
#ifndef QT_BOOTSTRAPPED
    void append(qfloat16 f);
#endif
    void append(float f);
    void append(double d);

    void appendByteString(const char *data, qsizetype len);
    void appendTextString(const char *utf8, qsizetype len);

    // convenience
    void append(bool b)     { append(b ? QCborSimpleType::True : QCborSimpleType::False); }
    void appendNull()       { append(QCborSimpleType::Null); }
    void appendUndefined()  { append(QCborSimpleType::Undefined); }

#ifndef Q_QDOC
    // overloads to make normal code not complain
    void append(int i)      { append(qint64(i)); }
    void append(uint u)     { append(quint64(u)); }
#endif
#ifndef QT_NO_CAST_FROM_ASCII
    void append(const char *str, qsizetype size = -1)
    { appendTextString(str, (str && size == -1)  ? int(strlen(str)) : size); }
#endif

    void startArray();
    void startArray(quint64 count);
    bool endArray();
    void startMap();
    void startMap(quint64 count);
    bool endMap();

    // no API for encoding chunked strings

private:
    QScopedPointer<QCborStreamWriterPrivate> d;
};

QT_END_NAMESPACE

#if defined(QT_X11_DEFINES_FOUND)
#  define True  1
#  define False 0
#endif

#endif // QCBORSTREAMWRITER_H
