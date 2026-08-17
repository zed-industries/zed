// Copyright (C) 2021 The Qt Company Ltd.
// Copyright (C) 2020 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTEST_H
#define QTEST_H

#include <QtTest/qttestglobal.h>
#include <QtTest/qtestcase.h>
#include <QtTest/qtestdata.h>
#include <QtTest/qbenchmark.h>

#include <QtCore/qbitarray.h>
#include <QtCore/qbytearray.h>
#include <QtCore/qcborarray.h>
#include <QtCore/qcborcommon.h>
#include <QtCore/qcbormap.h>
#include <QtCore/qcborvalue.h>
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

#if defined(TESTCASE_LOWDPI)
#include <QtCore/qcoreapplication.h>
#endif

#include <QtCore/qpoint.h>
#include <QtCore/qsize.h>
#include <QtCore/qrect.h>

#include <initializer_list>
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

template<> inline char *toString(const QLatin1StringView &str)
{
    return toString(QString(str));
}

template<> inline char *toString(const QByteArray &ba)
{
    return QTest::toPrettyCString(ba.constData(), ba.size());
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
        return qstrdup(qPrintable(QLatin1StringView("Invalid URL: ") + uri.errorString()));
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
            if (v.canConvert<QString>()) {
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

namespace Internal {
struct QCborValueFormatter
{
    enum { BufferLen = 256 };
    static char *formatSimpleType(QCborSimpleType st)
    {
        char *buf = new char[BufferLen];
        qsnprintf(buf, BufferLen, "QCborValue(QCborSimpleType(%d))", int(st));
        return buf;
    }

    static char *formatTag(QCborTag tag, const QCborValue &taggedValue)
    {
        QScopedArrayPointer<char> hold(format(taggedValue));
        char *buf = new char[BufferLen];
        qsnprintf(buf, BufferLen, "QCborValue(QCborTag(%llu), %s)", tag, hold.get());
        return buf;
    }

    static char *innerFormat(QCborValue::Type t, const char *str)
    {
        static const QMetaEnum typeEnum = []() {
            int idx = QCborValue::staticMetaObject.indexOfEnumerator("Type");
            return QCborValue::staticMetaObject.enumerator(idx);
        }();

        char *buf = new char[BufferLen];
        const char *typeName = typeEnum.valueToKey(t);
        if (typeName)
            qsnprintf(buf, BufferLen, "QCborValue(%s, %s)", typeName, str);
        else
            qsnprintf(buf, BufferLen, "QCborValue(<unknown type 0x%02x>)", t);
        return buf;
    }

    template<typename T> static char *format(QCborValue::Type type, const T &t)
    {
        QScopedArrayPointer<char> hold(QTest::toString(t));
        return innerFormat(type, hold.get());
    }

    static char *format(const QCborValue &v)
    {
        switch (v.type()) {
        case QCborValue::Integer:
            return format(v.type(), v.toInteger());
        case QCborValue::ByteArray:
            return format(v.type(), v.toByteArray());
        case QCborValue::String:
            return format(v.type(), v.toString());
        case QCborValue::Array:
            return innerFormat(v.type(), QScopedArrayPointer<char>(format(v.toArray())).get());
        case QCborValue::Map:
            return innerFormat(v.type(), QScopedArrayPointer<char>(format(v.toMap())).get());
        case QCborValue::Tag:
            return formatTag(v.tag(), v.taggedValue());
        case QCborValue::SimpleType:
            break;
        case QCborValue::True:
            return qstrdup("QCborValue(true)");
        case QCborValue::False:
            return qstrdup("QCborValue(false)");
        case QCborValue::Null:
            return qstrdup("QCborValue(nullptr)");
        case QCborValue::Undefined:
            return qstrdup("QCborValue()");
        case QCborValue::Double:
            return format(v.type(), v.toDouble());
        case QCborValue::DateTime:
        case QCborValue::Url:
        case QCborValue::RegularExpression:
            return format(v.type(), v.taggedValue().toString());
        case QCborValue::Uuid:
            return format(v.type(), v.toUuid());
        case QCborValue::Invalid:
            return qstrdup("QCborValue(<invalid>)");
        }

        if (v.isSimpleType())
            return formatSimpleType(v.toSimpleType());
        return innerFormat(v.type(), "");
    }

    static char *format(const QCborArray &a)
    {
        QByteArray out(1, '[');
        const char *comma = "";
        for (QCborValueConstRef v : a) {
            QScopedArrayPointer<char> s(format(v));
            out += comma;
            out += s.get();
            comma = ", ";
        }
        out += ']';
        return qstrdup(out.constData());
    }

    static char *format(const QCborMap &m)
    {
        QByteArray out(1, '{');
        const char *comma = "";
        for (auto pair : m) {
            QScopedArrayPointer<char> key(format(pair.first));
            QScopedArrayPointer<char> value(format(pair.second));
            out += comma;
            out += key.get();
            out += ": ";
            out += value.get();
            comma = ", ";
        }
        out += '}';
        return qstrdup(out.constData());
    }
};
}

template<> inline char *toString(const QCborValue &v)
{
    return Internal::QCborValueFormatter::format(v);
}

template<> inline char *toString(const QCborValueRef &v)
{
    return toString(QCborValue(v));
}

template<> inline char *toString(const QCborArray &a)
{
    return Internal::QCborValueFormatter::format(a);
}

template<> inline char *toString(const QCborMap &m)
{
    return Internal::QCborValueFormatter::format(m);
}

template <typename T1, typename T2>
inline char *toString(const std::pair<T1, T2> &pair)
{
    const QScopedArrayPointer<char> first(toString(pair.first));
    const QScopedArrayPointer<char> second(toString(pair.second));
    return formatString("std::pair(", ")", 2, first.data(), second.data());
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
    return toString(QStringLiteral("nullptr"));
}

template<>
inline bool qCompare(QString const &t1, QLatin1StringView const &t2, const char *actual,
                     const char *expected, const char *file, int line)
{
    return qCompare(t1, QString(t2), actual, expected, file, line);
}
template<>
inline bool qCompare(QLatin1StringView const &t1, QString const &t2, const char *actual,
                     const char *expected, const char *file, int line)
{
    return qCompare(QString(t1), t2, actual, expected, file, line);
}

// Compare sequences of equal size
template <typename ActualIterator, typename ExpectedIterator>
bool _q_compareSequence(ActualIterator actualIt, ActualIterator actualEnd,
                        ExpectedIterator expectedBegin, ExpectedIterator expectedEnd,
                        const char *actual, const char *expected,
                        const char *file, int line)
{
    char msg[1024];
    msg[0] = '\0';

    const qsizetype actualSize = actualEnd - actualIt;
    const qsizetype expectedSize = expectedEnd - expectedBegin;
    bool isOk = actualSize == expectedSize;

    if (!isOk) {
        qsnprintf(msg, sizeof(msg), "Compared lists have different sizes.\n"
                  "   Actual   (%s) size: %zd\n"
                  "   Expected (%s) size: %zd", actual, actualSize,
                  expected, expectedSize);
    }

    for (auto expectedIt = expectedBegin; isOk && expectedIt < expectedEnd; ++actualIt, ++expectedIt) {
        if (!(*actualIt == *expectedIt)) {
            const qsizetype i = qsizetype(expectedIt - expectedBegin);
            char *val1 = toString(*actualIt);
            char *val2 = toString(*expectedIt);

            qsnprintf(msg, sizeof(msg), "Compared lists differ at index %zd.\n"
                      "   Actual   (%s): %s\n"
                      "   Expected (%s): %s", i, actual, val1 ? val1 : "<null>",
                      expected, val2 ? val2 : "<null>");
            isOk = false;

            delete [] val1;
            delete [] val2;
        }
    }
    return compare_helper(isOk, msg, actual, expected, file, line);
}

namespace Internal {

#if defined(TESTCASE_LOWDPI)
void disableHighDpi()
{
    qputenv("QT_ENABLE_HIGHDPI_SCALING", "0");
}
Q_CONSTRUCTOR_FUNCTION(disableHighDpi);
#endif

} // namespace Internal

template <typename T>
inline bool qCompare(QList<T> const &t1, QList<T> const &t2, const char *actual, const char *expected,
                     const char *file, int line)
{
    return _q_compareSequence(t1.cbegin(), t1.cend(), t2.cbegin(), t2.cend(),
                                     actual, expected, file, line);
}

template <typename T, int N>
bool qCompare(QList<T> const &t1, std::initializer_list<T> t2,
              const char *actual, const char *expected,
              const char *file, int line)
{
    return _q_compareSequence(t1.cbegin(), t1.cend(), t2.cbegin(), t2.cend(),
                                     actual, expected, file, line);
}

// Compare QList against array
template <typename T, int N>
bool qCompare(QList<T> const &t1, const T (& t2)[N],
              const char *actual, const char *expected,
              const char *file, int line)
{
    return _q_compareSequence(t1.cbegin(), t1.cend(), t2, t2 + N,
                                     actual, expected, file, line);
}

template <typename T>
inline bool qCompare(QFlags<T> const &t1, T const &t2, const char *actual, const char *expected,
                    const char *file, int line)
{
    using Int = typename QFlags<T>::Int;
    return qCompare(Int(t1), Int(t2), actual, expected, file, line);
}

template <typename T>
inline bool qCompare(QFlags<T> const &t1, int const &t2, const char *actual, const char *expected,
                    const char *file, int line)
{
    using Int = typename QFlags<T>::Int;
    return qCompare(Int(t1), Int(t2), actual, expected, file, line);
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

// Internal (but used by some testlib selftests to hack argc and argv).
// Tests should normally implement initMain() if they have set-up to do before
// instantiating the test class.
#define QTEST_MAIN_WRAPPER(TestObject, ...) \
int main(int argc, char *argv[]) \
{ \
    TESTLIB_SELFCOVERAGE_START(#TestObject) \
    QT_PREPEND_NAMESPACE(QTest::Internal::callInitMain)<TestObject>(); \
    __VA_ARGS__ \
    TestObject tc; \
    QTEST_SET_MAIN_SOURCE_PATH \
    return QTest::qExec(&tc, argc, argv); \
}

// For when you don't even want a QApplication:
#define QTEST_APPLESS_MAIN(TestObject) QTEST_MAIN_WRAPPER(TestObject)

#include <QtTest/qtestsystem.h>

#if defined(QT_NETWORK_LIB)
#  include <QtTest/qtest_network.h>
#endif

// Internal
#define QTEST_QAPP_SETUP(klaz) \
    klaz app(argc, argv); \
    app.setAttribute(Qt::AA_Use96Dpi, true);

#if defined(QT_WIDGETS_LIB)
#  include <QtTest/qtest_widgets.h>
#  ifdef QT_KEYPAD_NAVIGATION
#    define QTEST_DISABLE_KEYPAD_NAVIGATION QApplication::setNavigationMode(Qt::NavigationModeNone);
#  else
#    define QTEST_DISABLE_KEYPAD_NAVIGATION
#  endif
// Internal
#  define QTEST_MAIN_SETUP() QTEST_QAPP_SETUP(QApplication) QTEST_DISABLE_KEYPAD_NAVIGATION
#elif defined(QT_GUI_LIB)
#  include <QtTest/qtest_gui.h>
// Internal
#  define QTEST_MAIN_SETUP() QTEST_QAPP_SETUP(QGuiApplication)
#else
// Internal
#  define QTEST_MAIN_SETUP() QTEST_QAPP_SETUP(QCoreApplication)
#endif // QT_GUI_LIB

// For most tests:
#define QTEST_MAIN(TestObject) QTEST_MAIN_WRAPPER(TestObject, QTEST_MAIN_SETUP())

// For command-line tests
#define QTEST_GUILESS_MAIN(TestObject) \
    QTEST_MAIN_WRAPPER(TestObject, QTEST_QAPP_SETUP(QCoreApplication))

#endif
