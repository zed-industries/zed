/****************************************************************************
**
** Copyright (C) 2020 The Qt Company Ltd.
** Copyright (C) 2016 Intel Corporation.
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

#ifndef QDATETIME_H
#define QDATETIME_H

#include <QtCore/qstring.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qshareddata.h>

#include <limits>

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
Q_FORWARD_DECLARE_CF_TYPE(CFDate);
Q_FORWARD_DECLARE_OBJC_CLASS(NSDate);
#endif

QT_BEGIN_NAMESPACE

class QCalendar;
#if QT_CONFIG(timezone)
class QTimeZone;
#endif
class QDateTime;

class Q_CORE_EXPORT QDate // ### Qt 6: change to be used by value, not const &
{
public:
    enum MonthNameType { // ### Qt 6: remove, along with methods using it
        DateFormat = 0,
        StandaloneFormat
    };
private:
    explicit Q_DECL_CONSTEXPR QDate(qint64 julianDay) : jd(julianDay) {}
public:
    Q_DECL_CONSTEXPR QDate() : jd(nullJd()) {}
    QDate(int y, int m, int d);
    QDate(int y, int m, int d, QCalendar cal);

    Q_DECL_CONSTEXPR bool isNull() const { return !isValid(); }
    Q_DECL_CONSTEXPR bool isValid() const { return jd >= minJd() && jd <= maxJd(); }

    int year() const;
    int month() const;
    int day() const;
    int dayOfWeek() const;
    int dayOfYear() const;
    int daysInMonth() const;
    int daysInYear() const;
    int weekNumber(int *yearNum = nullptr) const;

    int year(QCalendar cal) const;
    int month(QCalendar cal) const;
    int day(QCalendar cal) const;
    int dayOfWeek(QCalendar cal) const;
    int dayOfYear(QCalendar cal) const;
    int daysInMonth(QCalendar cal) const;
    int daysInYear(QCalendar cal) const;

    QDateTime startOfDay(Qt::TimeSpec spec = Qt::LocalTime, int offsetSeconds = 0) const;
    QDateTime endOfDay(Qt::TimeSpec spec = Qt::LocalTime, int offsetSeconds = 0) const;
#if QT_CONFIG(timezone)
    QDateTime startOfDay(const QTimeZone &zone) const;
    QDateTime endOfDay(const QTimeZone &zone) const;
#endif

#if QT_DEPRECATED_SINCE(5, 10) && QT_CONFIG(textdate)
    QT_DEPRECATED_X("Use QLocale::monthName or QLocale::standaloneMonthName")
        static QString shortMonthName(int month, MonthNameType type = DateFormat);
    QT_DEPRECATED_X("Use QLocale::dayName or QLocale::standaloneDayName")
        static QString shortDayName(int weekday, MonthNameType type = DateFormat);
    QT_DEPRECATED_X("Use QLocale::monthName or QLocale::standaloneMonthName")
        static QString longMonthName(int month, MonthNameType type = DateFormat);
    QT_DEPRECATED_X("Use QLocale::dayName or QLocale::standaloneDayName")
        static QString longDayName(int weekday, MonthNameType type = DateFormat);
#endif // textdate && deprecated
#if QT_CONFIG(datestring)
    QString toString(Qt::DateFormat format = Qt::TextDate) const;
#if QT_DEPRECATED_SINCE(5, 15)
    // Only the deprecated locale-dependent formats use the calendar.
    QT_DEPRECATED_X("Use QLocale or omit the calendar")
    QString toString(Qt::DateFormat format, QCalendar cal) const;
#endif

#if QT_STRINGVIEW_LEVEL < 2
    QString toString(const QString &format) const;
    QString toString(const QString &format, QCalendar cal) const;
#endif

    QString toString(QStringView format) const;
    QString toString(QStringView format, QCalendar cal) const;
#endif
#if QT_DEPRECATED_SINCE(5,0)
    QT_DEPRECATED_X("Use setDate() instead") inline bool setYMD(int y, int m, int d)
    { if (uint(y) <= 99) y += 1900; return setDate(y, m, d); }
#endif

    bool setDate(int year, int month, int day);
    bool setDate(int year, int month, int day, QCalendar cal);

#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    void getDate(int *year, int *month, int *day); // ### Qt 6: remove
#endif // < Qt 6
    void getDate(int *year, int *month, int *day) const;

    Q_REQUIRED_RESULT QDate addDays(qint64 days) const;
    Q_REQUIRED_RESULT QDate addMonths(int months) const;
    Q_REQUIRED_RESULT QDate addYears(int years) const;
    Q_REQUIRED_RESULT QDate addMonths(int months, QCalendar cal) const;
    Q_REQUIRED_RESULT QDate addYears(int years, QCalendar cal) const;
    qint64 daysTo(const QDate &) const; // ### Qt 6: QDate

    Q_DECL_CONSTEXPR bool operator==(const QDate &other) const { return jd == other.jd; }
    Q_DECL_CONSTEXPR bool operator!=(const QDate &other) const { return jd != other.jd; }
    Q_DECL_CONSTEXPR bool operator< (const QDate &other) const { return jd <  other.jd; }
    Q_DECL_CONSTEXPR bool operator<=(const QDate &other) const { return jd <= other.jd; }
    Q_DECL_CONSTEXPR bool operator> (const QDate &other) const { return jd >  other.jd; }
    Q_DECL_CONSTEXPR bool operator>=(const QDate &other) const { return jd >= other.jd; }

    static QDate currentDate();
#if QT_CONFIG(datestring)
    static QDate fromString(const QString &s, Qt::DateFormat f = Qt::TextDate);
    static QDate fromString(const QString &s, const QString &format);
    static QDate fromString(const QString &s, const QString &format, QCalendar cal);
#endif
    static bool isValid(int y, int m, int d);
    static bool isLeapYear(int year);

    static Q_DECL_CONSTEXPR inline QDate fromJulianDay(qint64 jd_)
    { return jd_ >= minJd() && jd_ <= maxJd() ? QDate(jd_) : QDate() ; }
    Q_DECL_CONSTEXPR inline qint64 toJulianDay() const { return jd; }

private:
    // using extra parentheses around min to avoid expanding it if it is a macro
    static Q_DECL_CONSTEXPR inline qint64 nullJd() { return (std::numeric_limits<qint64>::min)(); }
    static Q_DECL_CONSTEXPR inline qint64 minJd() { return Q_INT64_C(-784350574879); }
    static Q_DECL_CONSTEXPR inline qint64 maxJd() { return Q_INT64_C( 784354017364); }

    qint64 jd;

    friend class QDateTime;
    friend class QDateTimePrivate;
#ifndef QT_NO_DATASTREAM
    friend Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QDate &);
    friend Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QDate &);
#endif
};
Q_DECLARE_TYPEINFO(QDate, Q_MOVABLE_TYPE);

class Q_CORE_EXPORT QTime // ### Qt 6: change to be used by value, not const &
{
    explicit Q_DECL_CONSTEXPR QTime(int ms) : mds(ms)
    {}
public:
    Q_DECL_CONSTEXPR QTime(): mds(NullTime)
    {}
    QTime(int h, int m, int s = 0, int ms = 0);

    Q_DECL_CONSTEXPR bool isNull() const { return mds == NullTime; }
    bool isValid() const;

    int hour() const;
    int minute() const;
    int second() const;
    int msec() const;
#if QT_CONFIG(datestring)
    QString toString(Qt::DateFormat f = Qt::TextDate) const;
#if QT_STRINGVIEW_LEVEL < 2
    QString toString(const QString &format) const;
#endif
    QString toString(QStringView format) const;
#endif
    bool setHMS(int h, int m, int s, int ms = 0);

    Q_REQUIRED_RESULT QTime addSecs(int secs) const;
    int secsTo(const QTime &) const; // ### Qt 6: plain QTime
    Q_REQUIRED_RESULT QTime addMSecs(int ms) const;
    int msecsTo(const QTime &) const; // ### Qt 6: plain QTime

    Q_DECL_CONSTEXPR bool operator==(const QTime &other) const { return mds == other.mds; }
    Q_DECL_CONSTEXPR bool operator!=(const QTime &other) const { return mds != other.mds; }
    Q_DECL_CONSTEXPR bool operator< (const QTime &other) const { return mds <  other.mds; }
    Q_DECL_CONSTEXPR bool operator<=(const QTime &other) const { return mds <= other.mds; }
    Q_DECL_CONSTEXPR bool operator> (const QTime &other) const { return mds >  other.mds; }
    Q_DECL_CONSTEXPR bool operator>=(const QTime &other) const { return mds >= other.mds; }

    static Q_DECL_CONSTEXPR inline QTime fromMSecsSinceStartOfDay(int msecs) { return QTime(msecs); }
    Q_DECL_CONSTEXPR inline int msecsSinceStartOfDay() const { return mds == NullTime ? 0 : mds; }

    static QTime currentTime();
#if QT_CONFIG(datestring)
    static QTime fromString(const QString &s, Qt::DateFormat f = Qt::TextDate);
    static QTime fromString(const QString &s, const QString &format);
#endif
    static bool isValid(int h, int m, int s, int ms = 0);

#if QT_DEPRECATED_SINCE(5, 14) // ### Qt 6: remove
    QT_DEPRECATED_X("Use QElapsedTimer instead") void start();
    QT_DEPRECATED_X("Use QElapsedTimer instead") int restart();
    QT_DEPRECATED_X("Use QElapsedTimer instead") int elapsed() const;
#endif
private:
    enum TimeFlag { NullTime = -1 };
    Q_DECL_CONSTEXPR inline int ds() const { return mds == -1 ? 0 : mds; }
    int mds;

    friend class QDateTime;
    friend class QDateTimePrivate;
#ifndef QT_NO_DATASTREAM // ### Qt 6: plain QTime
    friend Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QTime &);
    friend Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QTime &);
#endif
};
Q_DECLARE_TYPEINFO(QTime, Q_MOVABLE_TYPE);

class QDateTimePrivate;

class Q_CORE_EXPORT QDateTime
{
    // ### Qt 6: revisit the optimization
    struct ShortData {
#if Q_BYTE_ORDER == Q_LITTLE_ENDIAN
        quintptr status : 8;
#endif
        // note: this is only 24 bits on 32-bit systems...
        qintptr msecs : sizeof(void *) * 8 - 8;

#if Q_BYTE_ORDER == Q_BIG_ENDIAN
        quintptr status : 8;
#endif
    };

    union Data {
        enum {
            // To be of any use, we need at least 60 years around 1970, which
            // is 1,893,456,000,000 ms. That requires 41 bits to store, plus
            // the sign bit. With the status byte, the minimum size is 50 bits.
            CanBeSmall = sizeof(ShortData) * 8 > 50
        };

        Data();
        Data(Qt::TimeSpec);
        Data(const Data &other);
        Data(Data &&other);
        Data &operator=(const Data &other);
        ~Data();

        bool isShort() const;
        void detach();

        const QDateTimePrivate *operator->() const;
        QDateTimePrivate *operator->();

        QDateTimePrivate *d;
        ShortData data;
    };

public:
    QDateTime() noexcept(Data::CanBeSmall);
#if QT_DEPRECATED_SINCE(5, 15) // ### Qt 6: remove
    QT_DEPRECATED_X("Use QDate::startOfDay()") explicit QDateTime(const QDate &);
#endif
    QDateTime(const QDate &, const QTime &, Qt::TimeSpec spec = Qt::LocalTime);
    // ### Qt 6: Merge with above with default offsetSeconds = 0
    QDateTime(const QDate &date, const QTime &time, Qt::TimeSpec spec, int offsetSeconds);
#if QT_CONFIG(timezone)
    QDateTime(const QDate &date, const QTime &time, const QTimeZone &timeZone);
#endif // timezone
    QDateTime(const QDateTime &other) noexcept;
    QDateTime(QDateTime &&other) noexcept;
    ~QDateTime();

    QDateTime &operator=(QDateTime &&other) noexcept { swap(other); return *this; }
    QDateTime &operator=(const QDateTime &other) noexcept;

    void swap(QDateTime &other) noexcept { qSwap(d.d, other.d.d); }

    bool isNull() const;
    bool isValid() const;

    QDate date() const;
    QTime time() const;
    Qt::TimeSpec timeSpec() const;
    int offsetFromUtc() const;
#if QT_CONFIG(timezone)
    QTimeZone timeZone() const;
#endif // timezone
    QString timeZoneAbbreviation() const;
    bool isDaylightTime() const;

    qint64 toMSecsSinceEpoch() const;
    qint64 toSecsSinceEpoch() const;

    void setDate(const QDate &date); // ### Qt 6: plain QDate
    void setTime(const QTime &time);
    void setTimeSpec(Qt::TimeSpec spec);
    void setOffsetFromUtc(int offsetSeconds);
#if QT_CONFIG(timezone)
    void setTimeZone(const QTimeZone &toZone);
#endif // timezone
    void setMSecsSinceEpoch(qint64 msecs);
    void setSecsSinceEpoch(qint64 secs);

#if QT_CONFIG(datestring)
    QString toString(Qt::DateFormat format = Qt::TextDate) const;
#if QT_STRINGVIEW_LEVEL < 2
    QString toString(const QString &format) const;
    QString toString(const QString &format, QCalendar cal) const;
#endif
    QString toString(QStringView format) const;
    QString toString(QStringView format, QCalendar cal) const;
#endif
    Q_REQUIRED_RESULT QDateTime addDays(qint64 days) const;
    Q_REQUIRED_RESULT QDateTime addMonths(int months) const;
    Q_REQUIRED_RESULT QDateTime addYears(int years) const;
    Q_REQUIRED_RESULT QDateTime addSecs(qint64 secs) const;
    Q_REQUIRED_RESULT QDateTime addMSecs(qint64 msecs) const;

    QDateTime toTimeSpec(Qt::TimeSpec spec) const;
    inline QDateTime toLocalTime() const { return toTimeSpec(Qt::LocalTime); }
    inline QDateTime toUTC() const { return toTimeSpec(Qt::UTC); }
    QDateTime toOffsetFromUtc(int offsetSeconds) const;
#if QT_CONFIG(timezone)
    QDateTime toTimeZone(const QTimeZone &toZone) const;
#endif // timezone

    qint64 daysTo(const QDateTime &) const;
    qint64 secsTo(const QDateTime &) const;
    qint64 msecsTo(const QDateTime &) const;

    bool operator==(const QDateTime &other) const;
    inline bool operator!=(const QDateTime &other) const { return !(*this == other); }
    bool operator<(const QDateTime &other) const;
    inline bool operator<=(const QDateTime &other) const { return !(other < *this); }
    inline bool operator>(const QDateTime &other) const { return other < *this; }
    inline bool operator>=(const QDateTime &other) const { return !(*this < other); }

#if QT_DEPRECATED_SINCE(5, 2) // ### Qt 6: remove
    QT_DEPRECATED_X("Use setOffsetFromUtc() instead") void setUtcOffset(int seconds);
    QT_DEPRECATED_X("Use offsetFromUtc() instead") int utcOffset() const;
#endif // QT_DEPRECATED_SINCE

    static QDateTime currentDateTime();
    static QDateTime currentDateTimeUtc();
#if QT_CONFIG(datestring)
    static QDateTime fromString(const QString &s, Qt::DateFormat f = Qt::TextDate);
    static QDateTime fromString(const QString &s, const QString &format);
    static QDateTime fromString(const QString &s, const QString &format, QCalendar cal);
#endif

#if QT_DEPRECATED_SINCE(5, 8)
    uint toTime_t() const;
    void setTime_t(uint secsSince1Jan1970UTC);
    static QDateTime fromTime_t(uint secsSince1Jan1970UTC);
    static QDateTime fromTime_t(uint secsSince1Jan1970UTC, Qt::TimeSpec spec,
                                int offsetFromUtc = 0);
#  if QT_CONFIG(timezone)
    static QDateTime fromTime_t(uint secsSince1Jan1970UTC, const QTimeZone &timeZone);
#  endif
#endif

    static QDateTime fromMSecsSinceEpoch(qint64 msecs);
    // ### Qt 6: Merge with above with default spec = Qt::LocalTime
    static QDateTime fromMSecsSinceEpoch(qint64 msecs, Qt::TimeSpec spec, int offsetFromUtc = 0);
    static QDateTime fromSecsSinceEpoch(qint64 secs, Qt::TimeSpec spe = Qt::LocalTime, int offsetFromUtc = 0);

#if QT_CONFIG(timezone)
    static QDateTime fromMSecsSinceEpoch(qint64 msecs, const QTimeZone &timeZone);
    static QDateTime fromSecsSinceEpoch(qint64 secs, const QTimeZone &timeZone);
#endif

    static qint64 currentMSecsSinceEpoch() noexcept;
    static qint64 currentSecsSinceEpoch() noexcept;

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
    static QDateTime fromCFDate(CFDateRef date);
    CFDateRef toCFDate() const Q_DECL_CF_RETURNS_RETAINED;
    static QDateTime fromNSDate(const NSDate *date);
    NSDate *toNSDate() const Q_DECL_NS_RETURNS_AUTORELEASED;
#endif

    // (1<<63) ms is 292277024.6 (average Gregorian) years, counted from the start of 1970, so
    // Last is floor(1970 + 292277024.6); no year 0, so First is floor(1970 - 1 - 292277024.6)
    enum class YearRange : qint32 { First = -292275056,  Last = +292278994 };

private:
    friend class QDateTimePrivate;

    Data d;

#ifndef QT_NO_DATASTREAM
    friend Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QDateTime &);
    friend Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QDateTime &);
#endif

#if !defined(QT_NO_DEBUG_STREAM) && QT_CONFIG(datestring)
    friend Q_CORE_EXPORT QDebug operator<<(QDebug, const QDateTime &);
#endif
};
Q_DECLARE_SHARED(QDateTime)

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QDate &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QDate &);
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QTime &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QTime &);
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QDateTime &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QDateTime &);
#endif // QT_NO_DATASTREAM

#if !defined(QT_NO_DEBUG_STREAM) && QT_CONFIG(datestring)
Q_CORE_EXPORT QDebug operator<<(QDebug, const QDate &);
Q_CORE_EXPORT QDebug operator<<(QDebug, const QTime &);
Q_CORE_EXPORT QDebug operator<<(QDebug, const QDateTime &);
#endif

// QDateTime is not noexcept for now -- to be revised once
// timezone and calendaring support is added
Q_CORE_EXPORT uint qHash(const QDateTime &key, uint seed = 0);
Q_CORE_EXPORT uint qHash(const QDate &key, uint seed = 0) noexcept;
Q_CORE_EXPORT uint qHash(const QTime &key, uint seed = 0) noexcept;

QT_END_NAMESPACE

#endif // QDATETIME_H
