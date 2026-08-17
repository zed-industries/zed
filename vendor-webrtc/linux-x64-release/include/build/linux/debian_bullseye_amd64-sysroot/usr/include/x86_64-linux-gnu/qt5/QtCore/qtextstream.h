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

#ifndef QTEXTSTREAM_H
#define QTEXTSTREAM_H

#include <QtCore/qiodevice.h>
#include <QtCore/qstring.h>
#include <QtCore/qchar.h>
#include <QtCore/qlocale.h>
#include <QtCore/qscopedpointer.h>

#include <stdio.h>

#ifdef Status
#error qtextstream.h must be included before any header file that defines Status
#endif

QT_BEGIN_NAMESPACE


class QTextCodec;
class QTextDecoder;

class QTextStreamPrivate;
class Q_CORE_EXPORT QTextStream                                // text stream class
{
    Q_DECLARE_PRIVATE(QTextStream)

public:
    enum RealNumberNotation {
        SmartNotation,
        FixedNotation,
        ScientificNotation
    };
    enum FieldAlignment {
        AlignLeft,
        AlignRight,
        AlignCenter,
        AlignAccountingStyle
    };
    enum Status {
        Ok,
        ReadPastEnd,
        ReadCorruptData,
        WriteFailed
    };
    enum NumberFlag {
        ShowBase = 0x1,
        ForcePoint = 0x2,
        ForceSign = 0x4,
        UppercaseBase = 0x8,
        UppercaseDigits = 0x10
    };
    Q_DECLARE_FLAGS(NumberFlags, NumberFlag)

    QTextStream();
    explicit QTextStream(QIODevice *device);
    explicit QTextStream(FILE *fileHandle, QIODevice::OpenMode openMode = QIODevice::ReadWrite);
    explicit QTextStream(QString *string, QIODevice::OpenMode openMode = QIODevice::ReadWrite);
    explicit QTextStream(QByteArray *array, QIODevice::OpenMode openMode = QIODevice::ReadWrite);
    explicit QTextStream(const QByteArray &array, QIODevice::OpenMode openMode = QIODevice::ReadOnly);
    virtual ~QTextStream();

#if QT_CONFIG(textcodec)
    void setCodec(QTextCodec *codec);
    void setCodec(const char *codecName);
    QTextCodec *codec() const;
    void setAutoDetectUnicode(bool enabled);
    bool autoDetectUnicode() const;
    void setGenerateByteOrderMark(bool generate);
    bool generateByteOrderMark() const;
#endif

    void setLocale(const QLocale &locale);
    QLocale locale() const;

    void setDevice(QIODevice *device);
    QIODevice *device() const;

    void setString(QString *string, QIODevice::OpenMode openMode = QIODevice::ReadWrite);
    QString *string() const;

    Status status() const;
    void setStatus(Status status);
    void resetStatus();

    bool atEnd() const;
    void reset();
    void flush();
    bool seek(qint64 pos);
    qint64 pos() const;

    void skipWhiteSpace();

    QString readLine(qint64 maxlen = 0);
    bool readLineInto(QString *line, qint64 maxlen = 0);
    QString readAll();
    QString read(qint64 maxlen);

    void setFieldAlignment(FieldAlignment alignment);
    FieldAlignment fieldAlignment() const;

    void setPadChar(QChar ch);
    QChar padChar() const;

    void setFieldWidth(int width);
    int fieldWidth() const;

    void setNumberFlags(NumberFlags flags);
    NumberFlags numberFlags() const;

    void setIntegerBase(int base);
    int integerBase() const;

    void setRealNumberNotation(RealNumberNotation notation);
    RealNumberNotation realNumberNotation() const;

    void setRealNumberPrecision(int precision);
    int realNumberPrecision() const;

    QTextStream &operator>>(QChar &ch);
    QTextStream &operator>>(char &ch);
    QTextStream &operator>>(signed short &i);
    QTextStream &operator>>(unsigned short &i);
    QTextStream &operator>>(signed int &i);
    QTextStream &operator>>(unsigned int &i);
    QTextStream &operator>>(signed long &i);
    QTextStream &operator>>(unsigned long &i);
    QTextStream &operator>>(qlonglong &i);
    QTextStream &operator>>(qulonglong &i);
    QTextStream &operator>>(float &f);
    QTextStream &operator>>(double &f);
    QTextStream &operator>>(QString &s);
    QTextStream &operator>>(QByteArray &array);
    QTextStream &operator>>(char *c);

    QTextStream &operator<<(QChar ch);
    QTextStream &operator<<(char ch);
    QTextStream &operator<<(signed short i);
    QTextStream &operator<<(unsigned short i);
    QTextStream &operator<<(signed int i);
    QTextStream &operator<<(unsigned int i);
    QTextStream &operator<<(signed long i);
    QTextStream &operator<<(unsigned long i);
    QTextStream &operator<<(qlonglong i);
    QTextStream &operator<<(qulonglong i);
    QTextStream &operator<<(float f);
    QTextStream &operator<<(double f);
    QTextStream &operator<<(const QString &s);
    QTextStream &operator<<(QStringView s);
    QTextStream &operator<<(QLatin1String s);
    QTextStream &operator<<(const QStringRef &s);
    QTextStream &operator<<(const QByteArray &array);
    QTextStream &operator<<(const char *c);
    QTextStream &operator<<(const void *ptr);

private:
    Q_DISABLE_COPY(QTextStream)
    friend class QDebugStateSaverPrivate;
    friend class QDebug;

    QScopedPointer<QTextStreamPrivate> d_ptr;
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QTextStream::NumberFlags)

/*****************************************************************************
  QTextStream manipulators
 *****************************************************************************/

typedef QTextStream & (*QTextStreamFunction)(QTextStream &);// manipulator function
typedef void (QTextStream::*QTSMFI)(int); // manipulator w/int argument
typedef void (QTextStream::*QTSMFC)(QChar); // manipulator w/QChar argument


class Q_CORE_EXPORT QTextStreamManipulator
{
public:
    Q_DECL_CONSTEXPR QTextStreamManipulator(QTSMFI m, int a) noexcept : mf(m), mc(nullptr), arg(a), ch() {}
    Q_DECL_CONSTEXPR QTextStreamManipulator(QTSMFC m, QChar c) noexcept : mf(nullptr), mc(m), arg(-1), ch(c) {}
    void exec(QTextStream &s) { if (mf) { (s.*mf)(arg); } else { (s.*mc)(ch); } }

private:
    QTSMFI mf;                                        // QTextStream member function
    QTSMFC mc;                                        // QTextStream member function
    int arg;                                          // member function argument
    QChar ch;
};

inline QTextStream &operator>>(QTextStream &s, QTextStreamFunction f)
{ return (*f)(s); }

inline QTextStream &operator<<(QTextStream &s, QTextStreamFunction f)
{ return (*f)(s); }

inline QTextStream &operator<<(QTextStream &s, QTextStreamManipulator m)
{ m.exec(s); return s; }

namespace Qt {
Q_CORE_EXPORT QTextStream &bin(QTextStream &s);
Q_CORE_EXPORT QTextStream &oct(QTextStream &s);
Q_CORE_EXPORT QTextStream &dec(QTextStream &s);
Q_CORE_EXPORT QTextStream &hex(QTextStream &s);

Q_CORE_EXPORT QTextStream &showbase(QTextStream &s);
Q_CORE_EXPORT QTextStream &forcesign(QTextStream &s);
Q_CORE_EXPORT QTextStream &forcepoint(QTextStream &s);
Q_CORE_EXPORT QTextStream &noshowbase(QTextStream &s);
Q_CORE_EXPORT QTextStream &noforcesign(QTextStream &s);
Q_CORE_EXPORT QTextStream &noforcepoint(QTextStream &s);

Q_CORE_EXPORT QTextStream &uppercasebase(QTextStream &s);
Q_CORE_EXPORT QTextStream &uppercasedigits(QTextStream &s);
Q_CORE_EXPORT QTextStream &lowercasebase(QTextStream &s);
Q_CORE_EXPORT QTextStream &lowercasedigits(QTextStream &s);

Q_CORE_EXPORT QTextStream &fixed(QTextStream &s);
Q_CORE_EXPORT QTextStream &scientific(QTextStream &s);

Q_CORE_EXPORT QTextStream &left(QTextStream &s);
Q_CORE_EXPORT QTextStream &right(QTextStream &s);
Q_CORE_EXPORT QTextStream &center(QTextStream &s);

Q_CORE_EXPORT QTextStream &endl(QTextStream &s);
Q_CORE_EXPORT QTextStream &flush(QTextStream &s);
Q_CORE_EXPORT QTextStream &reset(QTextStream &s);

Q_CORE_EXPORT QTextStream &bom(QTextStream &s);

Q_CORE_EXPORT QTextStream &ws(QTextStream &s);

} // namespace Qt

#if QT_DEPRECATED_SINCE(5, 15)
// This namespace only exists for 'using namespace' declarations.
namespace QTextStreamFunctions {
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::bin") QTextStream &bin(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::oct") QTextStream &oct(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::dec") QTextStream &dec(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::hex") QTextStream &hex(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::showbase") QTextStream &showbase(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::forcesign") QTextStream &forcesign(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::forcepoint") QTextStream &forcepoint(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::noshowbase") QTextStream &noshowbase(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::noforcesign") QTextStream &noforcesign(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::noforcepoint") QTextStream &noforcepoint(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::uppercasebase") QTextStream &uppercasebase(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::uppercasedigits") QTextStream &uppercasedigits(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::lowercasebase") QTextStream &lowercasebase(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::lowercasedigits") QTextStream &lowercasedigits(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::fixed") QTextStream &fixed(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::scientific") QTextStream &scientific(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::left") QTextStream &left(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::right") QTextStream &right(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::center") QTextStream &center(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::endl") QTextStream &endl(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::flush") QTextStream &flush(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::reset") QTextStream &reset(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::bom") QTextStream &bom(QTextStream &s);
Q_CORE_EXPORT QT_DEPRECATED_VERSION_X(5, 15, "Use Qt::ws") QTextStream &ws(QTextStream &s);
} // namespace QTextStreamFunctions

QT_WARNING_PUSH
QT_WARNING_DISABLE_CLANG("-Wheader-hygiene")
// We use 'using namespace' as that doesn't cause
// conflicting definitions compiler errors.
using namespace QTextStreamFunctions;
QT_WARNING_POP
#endif // QT_DEPRECATED_SINCE(5, 15)

inline QTextStreamManipulator qSetFieldWidth(int width)
{
    QTSMFI func = &QTextStream::setFieldWidth;
    return QTextStreamManipulator(func,width);
}

inline QTextStreamManipulator qSetPadChar(QChar ch)
{
    QTSMFC func = &QTextStream::setPadChar;
    return QTextStreamManipulator(func, ch);
}

inline QTextStreamManipulator qSetRealNumberPrecision(int precision)
{
    QTSMFI func = &QTextStream::setRealNumberPrecision;
    return QTextStreamManipulator(func, precision);
}

QT_END_NAMESPACE

#endif // QTEXTSTREAM_H
