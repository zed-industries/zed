/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Copyright (C) 2016 Intel Corporation.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtTest module of the Qt Toolkit.
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

#ifndef QTEST_H
#define QTEST_H

#include <QtTest/qttestglobal.h>
#include <QtTest/qtestcase.h>
#include <QtTest/qtestdata.h>
#include <QtTest/qbenchmark.h>

#include <QtCore/qbitarray.h>
#include <QtCore/qbytearray.h>
#include <QtCore/qstring.h>
#include <QtCore/qstringlist.h>
#include <QtCore/qcborcommon.h>
#include <QtCore/qdatetime.h>
#if QT_CONFIG(itemmodel)
#include <QtCore/qabstractitemmodel.h>
#endif
#include <QtCore/qobject.h>
#include <QtCore/qvariant.h>
#include <QtCore/qurl.h>
#include <QtCore/quuid.h>

#include <QtCore/qpoint.h>
#include <QtCore/qsize.h>
#include <QtCore/qrect.h>

#include <memory>

QT_BEGIN_NAMESPACE


namespace QTest
{

template <> inline char *toString(const QStringView &str)
{
    return QTest::toPrettyUnicode(str);
}

template<> inline char *toString(const QString &str)
{
    return toString(QStringView(str));
}

template<> inline char *toString(const QLatin1String &str)
{
    return toString(QString(str));
}

template<> inline char *toString(const QByteArray &ba)
{
    return QTest::toPrettyCString(ba.constData(), ba.length());
}

template<> inline char *toString(const QBitArray &ba)
{
    qsizetype size = ba.size();
    char *str = new char[size + 1];
    for (qsizetype i = 0; i < size; ++i)
        str[i] = "01"[ba.testBit(i)];
    str[size] = '\0';
    return str;
}

#if QT_CONFIG(datestring)
template<> inline char *toString(const QTime &time)
{
    return time.isValid()
        ? qstrdup(qPrintable(time.toString(u"hh:mm:ss.zzz")))
        : qstrdup("Invalid QTime");
}

template<> inline char *toString(const QDate &date)
{
    return date.isValid()
        ? qstrdup(qPrintable(date.toString(u"yyyy/MM/dd")))
        : qstrdup("Invalid QDate");
}

template<> inline char *toString(const QDateTime &dateTime)
{
    return dateTime.isValid()
        ? qstrdup(qPrintable(dateTime.toString(u"yyyy/MM/dd hh:mm:ss.zzz[t]")))
        : qstrdup("Invalid QDateTime");
}
#endif // datestring

template<> inline char *toString(const QCborError &c)
{
    // use the Q_ENUM formatting
    return toString(c.c);
}

template<> inline char *toString(const QChar &c)
{
    const ushort uc = c.unicode();
    if (uc < 128) {
        char msg[32] = {'\0'};
        qsnprintf(msg, sizeof(msg), "QChar: '%c' (0x%x)", char(uc), unsigned(uc));
        return qstrdup(msg);
    }
    return qstrdup(qPrintable(QString::fromLatin1("QChar: '%1' (0x%2)").arg(c).arg(QString::number(static_cast<int>(c.unicode()), 16))));
}

#if QT_CONFIG(itemmodel)
template<> inline char *toString(const QModelIndex &idx)
{
    char msg[128];
    qsnprintf(msg, sizeof(msg), "QModelIndex(%d,%d,%p,%p)", idx.row(), idx.column(), idx.internalPointer(), idx.model());
    return qstrdup(msg);
}
#endif

template<> inline char *toString(const QPoint &p)
{
    char msg[128] = {'\0'};
    qsnprintf(msg, sizeof(msg), "QPoint(%d,%d)", p.x(), p.y());
    return qstrdup(msg);
}

template<> inline char *toString(const QSize &s)
{
    char msg[128] = {'\0'};
    qsnprintf(msg, sizeof(msg), "QSize(%dx%d)", s.width(), s.height());
    return qstrdup(msg);
}

template<> inline char *toString(const QRect &s)
{
    char msg[256] = {'\0'};
    qsnprintf(msg, sizeof(msg), "QRect(%d,%d %dx%d) (bottomright %d,%d)",
              s.left(), s.top(), s.width(), s.height(), s.right(), s.bottom());
    return qstrdup(msg);
}

template<> inline char *toString(const QPointF &p)
{
    char msg[64] = {'\0'};
    qsnprintf(msg, sizeof(msg), "QPointF(%g,%g)", p.x(), p.y());
    return qstrdup(msg);
}

template<> inline char *toString(const QSizeF &s)
{
    char msg[64] = {'\0'};
    qsnprintf(msg, sizeof(msg), "QSizeF(%gx%g)", s.width(), s.height());
    return qstrdup(msg);
}

template<> inline char *toString(const QRectF &s)
{
    char msg[256] = {'\0'};
    qsnprintf(msg, sizeof(msg), "QRectF(%g,%g %gx%g) (bottomright %g,%g)",
              s.left(), s.top(), s.width(), s.height(), s.right(), s.bottom());
    return qstrdup(msg);
}

template<> inline char *toString(const QUrl &uri)
{
    if (!uri.isValid())
        return qstrdup(qPrintable(QLatin1String("Invalid URL: ") + uri.errorString()));
    return qstrdup(uri.toEncoded().constData());
}

template <> inline char *toString(const QUuid &uuid)
{
    return qstrdup(uuid.toByteArray().constData());
}

template<> inline char *toString(const QVariant &v)
{
    QByteArray vstring("QVariant(");
    if (v.isValid()) {
        QByteArray type(v.typeName());
        if (type.isEmpty()) {
            type = QByteArray::number(v.userType());
        }
        vstring.append(type);
        if (!v.isNull()) {
            vstring.append(',');
            if (v.canConvert(QMetaType::QString)) {
                vstring.append(v.toString().toLocal8Bit());
            }
            else {
                vstring.append("<value not representable as string>");
            }
        }
    }
    vstring.append(')');

    return qstrdup(vstring.constData());
}

template <typename T1, typename T2>
inline char *toString(const QPair<T1, T2> &pair)
{
    const QScopedArrayPointer<char> first(toString(pair.first));
    const QScopedArrayPointer<char> second(toString(pair.second));
    return toString(QString::asprintf("QPair(%s,%s)", first.data(), second.data()));
}

template <typename T1, typename T2>
inline char *toString(const std::pair<T1, T2> &pair)
{
    const QScopedArrayPointer<char> first(toString(pair.first));
    const QScopedArrayPointer<char> second(toString(pair.second));
    return toString(QString::asprintf("std::pair(%s,%s)", first.data(), second.data()));
}

template <typename Tuple, int... I>
inline char *toString(const Tuple & tuple, QtPrivate::IndexesList<I...>) {
    using UP = std::unique_ptr<char[]>;
    // Generate a table of N + 1 elements where N is the number of
    // elements in the tuple.
    // The last element is needed to support the empty tuple use case.
    const UP data[] = {
        UP(toString(std::get<I>(tuple)))..., UP{}
    };
    return formatString("std::tuple(", ")", sizeof...(I), data[I].get()...);
}

template <class... Types>
inline char *toString(const std::tuple<Types...> &tuple)
{
    static const std::size_t params_count = sizeof...(Types);
    return toString(tuple, typename QtPrivate::Indexes<params_count>::Value());
}

inline char *toString(std::nullptr_t)
{
    return toString(QLatin1String("nullptr"));
}

template<>
inline bool qCompare(QString const &t1, QLatin1String const &t2, const char *actual,
                    const char *expected, const char *file, int line)
{
    return qCompare(t1, QString(t2), actual, expected, file, line);
}
template<>
inline bool qCompare(QLatin1String const &t1, QString const &t2, const char *actual,
                    const char *expected, const char *file, int line)
{
    return qCompare(QString(t1), t2, actual, expected, file, line);
}

template <typename T>
inline bool qCompare(QList<T> const &t1, QList<T> const &t2, const char *actual, const char *expected,
                    const char *file, int line)
{
    char msg[1024];
    msg[0] = '\0';
    bool isOk = true;
    const int actualSize = t1.count();
    const int expectedSize = t2.count();
    if (actualSize != expectedSize) {
        qsnprintf(msg, sizeof(msg), "Compared lists have different sizes.\n"
                  "   Actual   (%s) size: %d\n"
                  "   Expected (%s) size: %d", actual, actualSize, expected, expectedSize);
        isOk = false;
    }
    for (int i = 0; isOk && i < actualSize; ++i) {
        if (!(t1.at(i) == t2.at(i))) {
            char *val1 = toString(t1.at(i));
            char *val2 = toString(t2.at(i));

            qsnprintf(msg, sizeof(msg), "Compared lists differ at index %d.\n"
                      "   Actual   (%s): %s\n"
                      "   Expected (%s): %s", i, actual, val1 ? val1 : "<null>",
                      expected, val2 ? val2 : "<null>");
            isOk = false;

            delete [] val1;
            delete [] val2;
        }
    }
    return compare_helper(isOk, msg, nullptr, nullptr, actual, expected, file, line);
}

template <>
inline bool qCompare(QStringList const &t1, QStringList const &t2, const char *actual, const char *expected,
                            const char *file, int line)
{
    return qCompare<QString>(t1, t2, actual, expected, file, line);
}

template <typename T>
inline bool qCompare(QFlags<T> const &t1, T const &t2, const char *actual, const char *expected,
                    const char *file, int line)
{
    return qCompare(int(t1), int(t2), actual, expected, file, line);
}

template <typename T>
inline bool qCompare(QFlags<T> const &t1, int const &t2, const char *actual, const char *expected,
                    const char *file, int line)
{
    return qCompare(int(t1), t2, actual, expected, file, line);
}

template<>
inline bool qCompare(qint64 const &t1, qint32 const &t2, const char *actual,
                    const char *expected, const char *file, int line)
{
    return qCompare(t1, static_cast<qint64>(t2), actual, expected, file, line);
}

template<>
inline bool qCompare(qint64 const &t1, quint32 const &t2, const char *actual,
                    const char *expected, const char *file, int line)
{
    return qCompare(t1, static_cast<qint64>(t2), actual, expected, file, line);
}

template<>
inline bool qCompare(quint64 const &t1, quint32 const &t2, const char *actual,
                    const char *expected, const char *file, int line)
{
    return qCompare(t1, static_cast<quint64>(t2), actual, expected, file, line);
}

template<>
inline bool qCompare(qint32 const &t1, qint64 const &t2, const char *actual,
                    const char *expected, const char *file, int line)
{
    return qCompare(static_cast<qint64>(t1), t2, actual, expected, file, line);
}

template<>
inline bool qCompare(quint32 const &t1, qint64 const &t2, const char *actual,
                    const char *expected, const char *file, int line)
{
    return qCompare(static_cast<qint64>(t1), t2, actual, expected, file, line);
}

template<>
inline bool qCompare(quint32 const &t1, quint64 const &t2, const char *actual,
                    const char *expected, const char *file, int line)
{
    return qCompare(static_cast<quint64>(t1), t2, actual, expected, file, line);
}
namespace Internal {

template <typename T>
class HasInitMain // SFINAE test for the presence of initMain()
{
private:
    using YesType = char[1];
    using NoType = char[2];

    template <typename C> static YesType& test( decltype(&C::initMain) ) ;
    template <typename C> static NoType& test(...);

public:
    enum { value = sizeof(test<T>(nullptr)) == sizeof(YesType) };
};

template<typename T>
typename std::enable_if<HasInitMain<T>::value, void>::type callInitMain()
{
    T::initMain();
}

template<typename T>
typename std::enable_if<!HasInitMain<T>::value, void>::type callInitMain()
{
}

} // namespace Internal

} // namespace QTest
QT_END_NAMESPACE

#ifdef QT_TESTCASE_BUILDDIR
#  define QTEST_SET_MAIN_SOURCE_PATH  QTest::setMainSourcePath(__FILE__, QT_TESTCASE_BUILDDIR);
#else
#  define QTEST_SET_MAIN_SOURCE_PATH  QTest::setMainSourcePath(__FILE__);
#endif

// Hooks for coverage-testing of QTestLib itself:
#if QT_CONFIG(testlib_selfcover) && defined(__COVERAGESCANNER__)
struct QtCoverageScanner
{
    QtCoverageScanner(const char *name)
    {
        __coveragescanner_clear();
        __coveragescanner_testname(name);
    }
    ~QtCoverageScanner()
    {
        __coveragescanner_save();
        __coveragescanner_testname("");
    }
};
#define TESTLIB_SELFCOVERAGE_START(name) QtCoverageScanner _qtCoverageScanner(name);
#else
#define TESTLIB_SELFCOVERAGE_START(name)
#endif

#define QTEST_APPLESS_MAIN(TestObject) \
int main(int argc, char *argv[]) \
{ \
    TESTLIB_SELFCOVERAGE_START(TestObject) \
    TestObject tc; \
    QTEST_SET_MAIN_SOURCE_PATH \
    return QTest::qExec(&tc, argc, argv); \
}

#include <QtTest/qtestsystem.h>

// Two backwards-compatibility defines for an obsolete feature:
#define QTEST_ADD_GPU_BLACKLIST_SUPPORT_DEFS
#define QTEST_ADD_GPU_BLACKLIST_SUPPORT
// ### Qt 6: fully remove these.

#if defined(QT_NETWORK_LIB)
#  include <QtTest/qtest_network.h>
#endif

#if defined(QT_WIDGETS_LIB)

#include <QtTest/qtest_widgets.h>

#ifdef QT_KEYPAD_NAVIGATION
#  define QTEST_DISABLE_KEYPAD_NAVIGATION QApplication::setNavigationMode(Qt::NavigationModeNone);
#else
#  define QTEST_DISABLE_KEYPAD_NAVIGATION
#endif

#define QTEST_MAIN_IMPL(TestObject) \
    TESTLIB_SELFCOVERAGE_START(#TestObject) \
    QT_PREPEND_NAMESPACE(QTest::Internal::callInitMain)<TestObject>(); \
    QApplication app(argc, argv); \
    app.setAttribute(Qt::AA_Use96Dpi, true); \
    QTEST_DISABLE_KEYPAD_NAVIGATION \
    TestObject tc; \
    QTEST_SET_MAIN_SOURCE_PATH \
    return QTest::qExec(&tc, argc, argv);

#elif defined(QT_GUI_LIB)

#include <QtTest/qtest_gui.h>

#define QTEST_MAIN_IMPL(TestObject) \
    TESTLIB_SELFCOVERAGE_START(#TestObject) \
    QT_PREPEND_NAMESPACE(QTest::Internal::callInitMain)<TestObject>(); \
    QGuiApplication app(argc, argv); \
    app.setAttribute(Qt::AA_Use96Dpi, true); \
    TestObject tc; \
    QTEST_SET_MAIN_SOURCE_PATH \
    return QTest::qExec(&tc, argc, argv);

#else

#define QTEST_MAIN_IMPL(TestObject) \
    TESTLIB_SELFCOVERAGE_START(#TestObject) \
    QT_PREPEND_NAMESPACE(QTest::Internal::callInitMain)<TestObject>(); \
    QCoreApplication app(argc, argv); \
    app.setAttribute(Qt::AA_Use96Dpi, true); \
    TestObject tc; \
    QTEST_SET_MAIN_SOURCE_PATH \
    return QTest::qExec(&tc, argc, argv);

#endif // QT_GUI_LIB

#define QTEST_MAIN(TestObject) \
int main(int argc, char *argv[]) \
{ \
    QTEST_MAIN_IMPL(TestObject) \
}

#define QTEST_GUILESS_MAIN(TestObject) \
int main(int argc, char *argv[]) \
{ \
    TESTLIB_SELFCOVERAGE_START(#TestObject) \
    QT_PREPEND_NAMESPACE(QTest::Internal::callInitMain)<TestObject>(); \
    QCoreApplication app(argc, argv); \
    app.setAttribute(Qt::AA_Use96Dpi, true); \
    TestObject tc; \
    QTEST_SET_MAIN_SOURCE_PATH \
    return QTest::qExec(&tc, argc, argv); \
}

#endif
