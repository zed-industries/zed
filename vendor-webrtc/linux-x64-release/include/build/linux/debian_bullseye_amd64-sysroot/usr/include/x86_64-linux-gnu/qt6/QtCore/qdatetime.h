// Copyright (C) 2021 The Qt Company Ltd.
// Copyright (C) 2021 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDATETIME_H
#define QDATETIME_H

#include <QtCore/qstring.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qshareddata.h>
#include <QtCore/qcalendar.h>

#include <limits>
#include <chrono>

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
Q_FORWARD_DECLARE_CF_TYPE(CFDate);
Q_FORWARD_DECLARE_OBJC_CLASS(NSDate);
#endif

QT_BEGIN_NAMESPACE

#if QT_CONFIG(timezone)
class QTimeZone;
#endif
class QDateTime;

class Q_CORE_EXPORT QDate
{
    explicit constexpr QDate(qint64 julianDay) : jd(julianDay) {}
public:
    constexpr QDate() : jd(nullJd()) {}
    QDate(int y, int m, int d);
    QDate(int y, int m, int d, QCalendar cal);
#if __cpp_lib_chrono >= 201907L || defined(Q_QDOC)
    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    Q_IMPLICIT QDate(std::chrono::year_month_day ymd)
    {
        if (!ymd.ok())
            jd = nullJd();
        else
            *this = fromStdSysDays(ymd);
    }

    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    Q_IMPLICIT QDate(std::chrono::year_month_day_last ymdl)
    {
        if (!ymdl.ok())
            jd = nullJd();
        else
            *this = fromStdSysDays(ymdl);
    }

    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    Q_IMPLICIT QDate(std::chrono::year_month_weekday ymw)
    {
        if (!ymw.ok())
            jd = nullJd();
        else
            *this = fromStdSysDays(ymw);
    }

    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    Q_IMPLICIT QDate(std::chrono::year_month_weekday_last ymwl)
    {
        if (!ymwl.ok())
            jd = nullJd();
        else
            *this = fromStdSysDays(ymwl);
    }

    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    static QDate fromStdSysDays(const std::chrono::sys_days &days)
    {
        const QDate epoch(unixEpochJd());
        return epoch.addDays(days.time_since_epoch().count());
    }

    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    std::chrono::sys_days toStdSysDays() const
    {
        const QDate epoch(unixEpochJd());
        return std::chrono::sys_days(std::chrono::days(epoch.daysTo(*this)));
    }
#endif

    constexpr bool isNull() const { return !isValid(); }
    constexpr bool isValid() const { return jd >= minJd() && jd <= maxJd(); }

    // Gregorian-optimized:
    int year() const;
    int month() const;
    int day() const;
    int dayOfWeek() const;
    int dayOfYear() const;
    int daysInMonth() const;
    int daysInYear() const;
    int weekNumber(int *yearNum = nullptr) const; // ISO 8601, always Gregorian

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

#if QT_CONFIG(datestring)
    QString toString(Qt::DateFormat format = Qt::TextDate) const;
    QString toString(const QString &format, QCalendar cal = QCalendar()) const
    { return toString(qToStringViewIgnoringNull(format), cal); }
    QString toString(QStringView format, QCalendar cal = QCalendar()) const;
#endif
    bool setDate(int year, int month, int day); // Gregorian-optimized
    bool setDate(int year, int month, int day, QCalendar cal);

    void getDate(int *year, int *month, int *day) const;

    [[nodiscard]] QDate addDays(qint64 days) const;
#if __cpp_lib_chrono >= 201907L || defined(Q_QDOC)
    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    [[nodiscard]] QDate addDuration(std::chrono::days days) const
    {
        return addDays(days.count());
    }
#endif
    // Gregorian-optimized:
    [[nodiscard]] QDate addMonths(int months) const;
    [[nodiscard]] QDate addYears(int years) const;
    [[nodiscard]] QDate addMonths(int months, QCalendar cal) const;
    [[nodiscard]] QDate addYears(int years, QCalendar cal) const;
    qint64 daysTo(QDate d) const;

    static QDate currentDate();
#if QT_CONFIG(datestring)
    static QDate fromString(QStringView string, Qt::DateFormat format = Qt::TextDate);
    static QDate fromString(QStringView string, QStringView format, QCalendar cal = QCalendar())
    { return fromString(string.toString(), format, cal); }
    static QDate fromString(const QString &string, QStringView format, QCalendar cal = QCalendar());
    static QDate fromString(const QString &string, Qt::DateFormat format = Qt::TextDate)
    { return fromString(qToStringViewIgnoringNull(string), format); }
    static QDate fromString(const QString &string, const QString &format,
                            QCalendar cal = QCalendar())
    { return fromString(string, qToStringViewIgnoringNull(format), cal); }
#endif
    static bool isValid(int y, int m, int d);
    static bool isLeapYear(int year);

    static constexpr inline QDate fromJulianDay(qint64 jd_)
    { return jd_ >= minJd() && jd_ <= maxJd() ? QDate(jd_) : QDate() ; }
    constexpr inline qint64 toJulianDay() const { return jd; }

private:
    // using extra parentheses around min to avoid expanding it if it is a macro
    static constexpr inline qint64 nullJd() { return (std::numeric_limits<qint64>::min)(); }
    static constexpr inline qint64 minJd() { return Q_INT64_C(-784350574879); }
    static constexpr inline qint64 maxJd() { return Q_INT64_C( 784354017364); }
    static constexpr inline qint64 unixEpochJd() { return Q_INT64_C(2440588); }

    qint64 jd;

    friend class QDateTime;
    friend class QDateTimePrivate;

    friend constexpr bool operator==(QDate lhs, QDate rhs) { return lhs.jd == rhs.jd; }
    friend constexpr bool operator!=(QDate lhs, QDate rhs) { return lhs.jd != rhs.jd; }
    friend constexpr bool operator< (QDate lhs, QDate rhs) { return lhs.jd <  rhs.jd; }
    friend constexpr bool operator<=(QDate lhs, QDate rhs) { return lhs.jd <= rhs.jd; }
    friend constexpr bool operator> (QDate lhs, QDate rhs) { return lhs.jd >  rhs.jd; }
    friend constexpr bool operator>=(QDate lhs, QDate rhs) { return lhs.jd >= rhs.jd; }
#ifndef QT_NO_DATASTREAM
    friend Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, QDate);
    friend Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QDate &);
#endif
};
Q_DECLARE_TYPEINFO(QDate, Q_RELOCATABLE_TYPE);

class Q_CORE_EXPORT QTime
{
    explicit constexpr QTime(int ms) : mds(ms)
    {}
public:
    constexpr QTime(): mds(NullTime)
    {}
    QTime(int h, int m, int s = 0, int ms = 0);

    constexpr bool isNull() const { return mds == NullTime; }
    bool isValid() const;

    int hour() const;
    int minute() const;
    int second() const;
    int msec() const;
#if QT_CONFIG(datestring)
    QString toString(Qt::DateFormat f = Qt::TextDate) const;
    QString toString(const QString &format) const
    { return toString(qToStringViewIgnoringNull(format)); }
    QString toString(QStringView format) const;
#endif
    bool setHMS(int h, int m, int s, int ms = 0);

    [[nodiscard]] QTime addSecs(int secs) const;
    int secsTo(QTime t) const;
    [[nodiscard]] QTime addMSecs(int ms) const;
    int msecsTo(QTime t) const;

    static constexpr inline QTime fromMSecsSinceStartOfDay(int msecs) { return QTime(msecs); }
    constexpr inline int msecsSinceStartOfDay() const { return mds == NullTime ? 0 : mds; }

    static QTime currentTime();
#if QT_CONFIG(datestring)
    static QTime fromString(QStringView string, Qt::DateFormat format = Qt::TextDate);
    static QTime fromString(QStringView string, QStringView format)
    { return fromString(string.toString(), format); }
    static QTime fromString(const QString &string, QStringView format);
    static QTime fromString(const QString &string, Qt::DateFormat format = Qt::TextDate)
    { return fromString(qToStringViewIgnoringNull(string), format); }
    static QTime fromString(const QString &string, const QString &format)
    { return fromString(string, qToStringViewIgnoringNull(format)); }
#endif
    static bool isValid(int h, int m, int s, int ms = 0);

private:
    enum TimeFlag { NullTime = -1 };
    constexpr inline int ds() const { return mds == -1 ? 0 : mds; }
    int mds;

    friend constexpr bool operator==(QTime lhs, QTime rhs) { return lhs.mds == rhs.mds; }
    friend constexpr bool operator!=(QTime lhs, QTime rhs) { return lhs.mds != rhs.mds; }
    friend constexpr bool operator< (QTime lhs, QTime rhs) { return lhs.mds <  rhs.mds; }
    friend constexpr bool operator<=(QTime lhs, QTime rhs) { return lhs.mds <= rhs.mds; }
    friend constexpr bool operator> (QTime lhs, QTime rhs) { return lhs.mds >  rhs.mds; }
    friend constexpr bool operator>=(QTime lhs, QTime rhs) { return lhs.mds >= rhs.mds; }

    friend class QDateTime;
    friend class QDateTimePrivate;
#ifndef QT_NO_DATASTREAM
    friend Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, QTime);
    friend Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QTime &);
#endif
};
Q_DECLARE_TYPEINFO(QTime, Q_RELOCATABLE_TYPE);

class QDateTimePrivate;

class Q_CORE_EXPORT QDateTime
{
    struct ShortData {
#if QT_VERSION >= QT_VERSION_CHECK(7,0,0) || defined(QT_BOOTSTRAPPED)
#  if Q_BYTE_ORDER == Q_LITTLE_ENDIAN
        qint64 status : 8;
#  endif
        qint64 msecs : 56;

#  if Q_BYTE_ORDER == Q_BIG_ENDIAN
        qint64 status : 8;
#  endif
#else
#  if Q_BYTE_ORDER == Q_LITTLE_ENDIAN
        quintptr status : 8;
#  endif
        // note: this is only 24 bits on 32-bit systems...
        qintptr msecs : sizeof(void *) * 8 - 8;

#  if Q_BYTE_ORDER == Q_BIG_ENDIAN
        quintptr status : 8;
#  endif
#endif
    };

    union Data {
        enum {
            // To be of any use, we need at least 60 years around 1970, which
            // is 1,893,456,000,000 ms. That requires 41 bits to store, plus
            // the sign bit. With the status byte, the minimum size is 50 bits.
            CanBeSmall = sizeof(ShortData) * 8 > 50
        };

        Data() noexcept;
        Data(Qt::TimeSpec);
        Data(const Data &other);
        Data(Data &&other);
        Data &operator=(const Data &other);
        ~Data();

        void swap(Data &other) noexcept
        { std::swap(data, other.data); }

        bool isShort() const;
        void detach();

        const QDateTimePrivate *operator->() const;
        QDateTimePrivate *operator->();

        QDateTimePrivate *d;
        ShortData data;
    };

public:
    QDateTime() noexcept;
    QDateTime(QDate date, QTime time, Qt::TimeSpec spec = Qt::LocalTime, int offsetSeconds = 0);
#if QT_CONFIG(timezone)
    QDateTime(QDate date, QTime time, const QTimeZone &timeZone);
#endif // timezone
    QDateTime(const QDateTime &other) noexcept;
    QDateTime(QDateTime &&other) noexcept;
    ~QDateTime();

    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QDateTime)
    QDateTime &operator=(const QDateTime &other) noexcept;

    void swap(QDateTime &other) noexcept { d.swap(other.d); }

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

    void setDate(QDate date);
    void setTime(QTime time);
    void setTimeSpec(Qt::TimeSpec spec);
    void setOffsetFromUtc(int offsetSeconds);
#if QT_CONFIG(timezone)
    void setTimeZone(const QTimeZone &toZone);
#endif // timezone
    void setMSecsSinceEpoch(qint64 msecs);
    void setSecsSinceEpoch(qint64 secs);

#if QT_CONFIG(datestring)
    QString toString(Qt::DateFormat format = Qt::TextDate) const;
    QString toString(const QString &format, QCalendar cal = QCalendar()) const
    { return toString(qToStringViewIgnoringNull(format), cal); }
    QString toString(QStringView format, QCalendar cal = QCalendar()) const;
#endif
    [[nodiscard]] QDateTime addDays(qint64 days) const;
    [[nodiscard]] QDateTime addMonths(int months) const;
    [[nodiscard]] QDateTime addYears(int years) const;
    [[nodiscard]] QDateTime addSecs(qint64 secs) const;
    [[nodiscard]] QDateTime addMSecs(qint64 msecs) const;
    [[nodiscard]] QDateTime addDuration(std::chrono::milliseconds msecs) const
    {
        return addMSecs(msecs.count());
    }

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

    static QDateTime currentDateTime();
    static QDateTime currentDateTimeUtc();
#if QT_CONFIG(datestring)
    static QDateTime fromString(QStringView string, Qt::DateFormat format = Qt::TextDate);
    static QDateTime fromString(QStringView string, QStringView format,
                                QCalendar cal = QCalendar())
    { return fromString(string.toString(), format, cal); }
    static QDateTime fromString(const QString &string, QStringView format,
                                QCalendar cal = QCalendar());
    static QDateTime fromString(const QString &string, Qt::DateFormat format = Qt::TextDate)
    { return fromString(qToStringViewIgnoringNull(string), format); }
    static QDateTime fromString(const QString &string, const QString &format,
                                QCalendar cal = QCalendar())
    { return fromString(string, qToStringViewIgnoringNull(format), cal); }
#endif

    static QDateTime fromMSecsSinceEpoch(qint64 msecs, Qt::TimeSpec spec = Qt::LocalTime,
                                         int offsetFromUtc = 0);
    static QDateTime fromSecsSinceEpoch(qint64 secs, Qt::TimeSpec spec = Qt::LocalTime,
                                        int offsetFromUtc = 0);

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

#if __cpp_lib_chrono >= 201907L || defined(Q_QDOC)
#if __cpp_concepts >= 201907L || defined(Q_QDOC)
    // Generic clock, as long as it's compatible with us (= system_clock)
    template <typename Clock, typename Duration>
    static QDateTime fromStdTimePoint(const std::chrono::time_point<Clock, Duration> &time)
        requires
            requires(const std::chrono::time_point<Clock, Duration> &t) {
                // the clock can be converted to system_clock
                std::chrono::clock_cast<std::chrono::system_clock>(t);
                // the duration can be converted to milliseconds
                requires std::is_convertible_v<Duration, std::chrono::milliseconds>;
            }
    {
        const auto sysTime = std::chrono::clock_cast<std::chrono::system_clock>(time);
        // clock_cast can change the duration, so convert it again to milliseconds
        const auto timeInMSec = std::chrono::time_point_cast<std::chrono::milliseconds>(sysTime);
        return fromMSecsSinceEpoch(timeInMSec.time_since_epoch().count(), Qt::UTC);
    }
#endif // __cpp_concepts

    // local_time
    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    static QDateTime fromStdTimePoint(const std::chrono::local_time<std::chrono::milliseconds> &time)
    {
        return fromStdLocalTime(time);
    }

    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    static QDateTime fromStdLocalTime(const std::chrono::local_time<std::chrono::milliseconds> &time)
    {
        QDateTime result(QDate(1970, 1, 1), QTime(0, 0, 0), Qt::LocalTime);
        return result.addMSecs(time.time_since_epoch().count());
    }

#if QT_CONFIG(timezone)
    // zoned_time. defined in qtimezone.h
    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    static QDateTime fromStdZonedTime(const std::chrono::zoned_time<
                                          std::chrono::milliseconds,
                                          const std::chrono::time_zone *
                                      > &time);
#endif // QT_CONFIG(timezone)

    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    std::chrono::sys_time<std::chrono::milliseconds> toStdSysMilliseconds() const
    {
        const std::chrono::milliseconds duration(toMSecsSinceEpoch());
        return std::chrono::sys_time<std::chrono::milliseconds>(duration);
    }

    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    std::chrono::sys_seconds toStdSysSeconds() const
    {
        const std::chrono::seconds duration(toSecsSinceEpoch());
        return std::chrono::sys_seconds(duration);
    }
#endif // __cpp_lib_chrono >= 201907L

    friend std::chrono::milliseconds operator-(const QDateTime &lhs, const QDateTime &rhs)
    {
        return std::chrono::milliseconds(rhs.msecsTo(lhs));
    }

    friend QDateTime operator+(const QDateTime &dateTime, std::chrono::milliseconds duration)
    {
        return dateTime.addMSecs(duration.count());
    }

    friend QDateTime operator+(std::chrono::milliseconds duration, const QDateTime &dateTime)
    {
        return dateTime + duration;
    }

    QDateTime &operator+=(std::chrono::milliseconds duration)
    {
        *this = addMSecs(duration.count());
        return *this;
    }

    friend QDateTime operator-(const QDateTime &dateTime, std::chrono::milliseconds duration)
    {
        return dateTime.addMSecs(-duration.count());
    }

    QDateTime &operator-=(std::chrono::milliseconds duration)
    {
        *this = addMSecs(-duration.count());
        return *this;
    }

    // (1<<63) ms is 292277024.6 (average Gregorian) years, counted from the start of 1970, so
    // Last is floor(1970 + 292277024.6); no year 0, so First is floor(1970 - 1 - 292277024.6)
    enum class YearRange : qint32 { First = -292275056,  Last = +292278994 };

private:
    bool equals(const QDateTime &other) const;
    bool precedes(const QDateTime &other) const;
    friend class QDateTimePrivate;

    Data d;

    friend bool operator==(const QDateTime &lhs, const QDateTime &rhs) { return lhs.equals(rhs); }
    friend bool operator!=(const QDateTime &lhs, const QDateTime &rhs) { return !(lhs == rhs); }
    friend bool operator<(const QDateTime &lhs, const QDateTime &rhs) { return lhs.precedes(rhs); }
    friend bool operator<=(const QDateTime &lhs, const QDateTime &rhs) { return !(rhs < lhs); }
    friend bool operator>(const QDateTime &lhs, const QDateTime &rhs) { return rhs.precedes(lhs); }
    friend bool operator>=(const QDateTime &lhs, const QDateTime &rhs) { return !(lhs < rhs); }

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
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, QDate);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QDate &);
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, QTime);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QTime &);
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QDateTime &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QDateTime &);
#endif // QT_NO_DATASTREAM

#if !defined(QT_NO_DEBUG_STREAM) && QT_CONFIG(datestring)
Q_CORE_EXPORT QDebug operator<<(QDebug, QDate);
Q_CORE_EXPORT QDebug operator<<(QDebug, QTime);
Q_CORE_EXPORT QDebug operator<<(QDebug, const QDateTime &);
#endif

// QDateTime is not noexcept for now -- to be revised once
// timezone and calendaring support is added
Q_CORE_EXPORT size_t qHash(const QDateTime &key, size_t seed = 0);
Q_CORE_EXPORT size_t qHash(QDate key, size_t seed = 0) noexcept;
Q_CORE_EXPORT size_t qHash(QTime key, size_t seed = 0) noexcept;

QT_END_NAMESPACE

#endif // QDATETIME_H
