// Copyright (C) 2013 John Layt <jlayt@kde.org>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only


#ifndef QTIMEZONE_H
#define QTIMEZONE_H

#include <QtCore/qshareddata.h>
#include <QtCore/qlocale.h>
#include <QtCore/qdatetime.h>

#include <chrono>

QT_REQUIRE_CONFIG(timezone);

#if (defined(Q_OS_DARWIN) || defined(Q_QDOC)) && !defined(QT_NO_SYSTEMLOCALE)
Q_FORWARD_DECLARE_CF_TYPE(CFTimeZone);
Q_FORWARD_DECLARE_OBJC_CLASS(NSTimeZone);
#endif

QT_BEGIN_NAMESPACE

class QTimeZonePrivate;

class Q_CORE_EXPORT QTimeZone
{
public:
    // Sane UTC offsets range from -14 to +14 hours:
    enum {
        // No known zone > 12 hrs West of Greenwich (Baker Island, USA)
        MinUtcOffsetSecs = -14 * 3600,
        // No known zone > 14 hrs East of Greenwich (Kiritimati, Christmas Island, Kiribati)
        MaxUtcOffsetSecs = +14 * 3600
    };

    enum TimeType {
        StandardTime = 0,
        DaylightTime = 1,
        GenericTime = 2
    };

    enum NameType {
        DefaultName = 0,
        LongName = 1,
        ShortName = 2,
        OffsetName = 3
    };

    struct OffsetData {
        QString abbreviation;
        QDateTime atUtc;
        int offsetFromUtc;
        int standardTimeOffset;
        int daylightTimeOffset;
    };
    typedef QList<OffsetData> OffsetDataList;

    QTimeZone() noexcept;
    explicit QTimeZone(const QByteArray &ianaId);
    explicit QTimeZone(int offsetSeconds);
    QTimeZone(const QByteArray &zoneId, int offsetSeconds, const QString &name,
              const QString &abbreviation, QLocale::Territory territory = QLocale::AnyTerritory,
              const QString &comment = QString());
    QTimeZone(const QTimeZone &other);
    ~QTimeZone();

    QTimeZone &operator=(const QTimeZone &other);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QTimeZone)

    void swap(QTimeZone &other) noexcept
    { d.swap(other.d); }

    bool operator==(const QTimeZone &other) const;
    bool operator!=(const QTimeZone &other) const;

    bool isValid() const;

    QByteArray id() const;
    QLocale::Territory territory() const;
#if QT_DEPRECATED_SINCE(6, 6)
    QT_DEPRECATED_VERSION_X_6_6("Use territory() instead")
    QLocale::Country country() const;
#endif
    QString comment() const;

    QString displayName(const QDateTime &atDateTime,
                        QTimeZone::NameType nameType = QTimeZone::DefaultName,
                        const QLocale &locale = QLocale()) const;
    QString displayName(QTimeZone::TimeType timeType,
                        QTimeZone::NameType nameType = QTimeZone::DefaultName,
                        const QLocale &locale = QLocale()) const;
    QString abbreviation(const QDateTime &atDateTime) const;

    int offsetFromUtc(const QDateTime &atDateTime) const;
    int standardTimeOffset(const QDateTime &atDateTime) const;
    int daylightTimeOffset(const QDateTime &atDateTime) const;

    bool hasDaylightTime() const;
    bool isDaylightTime(const QDateTime &atDateTime) const;

    OffsetData offsetData(const QDateTime &forDateTime) const;

    bool hasTransitions() const;
    OffsetData nextTransition(const QDateTime &afterDateTime) const;
    OffsetData previousTransition(const QDateTime &beforeDateTime) const;
    OffsetDataList transitions(const QDateTime &fromDateTime, const QDateTime &toDateTime) const;

    static QByteArray systemTimeZoneId();
    static QTimeZone systemTimeZone();
    static QTimeZone utc();

    static bool isTimeZoneIdAvailable(const QByteArray &ianaId);

    static QList<QByteArray> availableTimeZoneIds();
    static QList<QByteArray> availableTimeZoneIds(QLocale::Territory territory);
    static QList<QByteArray> availableTimeZoneIds(int offsetSeconds);

    static QByteArray ianaIdToWindowsId(const QByteArray &ianaId);
    static QByteArray windowsIdToDefaultIanaId(const QByteArray &windowsId);
    static QByteArray windowsIdToDefaultIanaId(const QByteArray &windowsId,
                                               QLocale::Territory territory);
    static QList<QByteArray> windowsIdToIanaIds(const QByteArray &windowsId);
    static QList<QByteArray> windowsIdToIanaIds(const QByteArray &windowsId,
                                                QLocale::Territory territory);

#if (defined(Q_OS_DARWIN) || defined(Q_QDOC)) && !defined(QT_NO_SYSTEMLOCALE)
    static QTimeZone fromCFTimeZone(CFTimeZoneRef timeZone);
    CFTimeZoneRef toCFTimeZone() const Q_DECL_CF_RETURNS_RETAINED;
    static QTimeZone fromNSTimeZone(const NSTimeZone *timeZone);
    NSTimeZone *toNSTimeZone() const Q_DECL_NS_RETURNS_AUTORELEASED;
#endif

#if __cpp_lib_chrono >= 201907L || defined(Q_QDOC)
    QT_POST_CXX17_API_IN_EXPORTED_CLASS
    static QTimeZone fromStdTimeZonePtr(const std::chrono::time_zone *timeZone)
    {
        if (!timeZone)
            return QTimeZone();
        const std::string_view timeZoneName = timeZone->name();
        return QTimeZone(QByteArrayView(timeZoneName).toByteArray());
    }
#endif

private:
    QTimeZone(QTimeZonePrivate &dd);
#ifndef QT_NO_DATASTREAM
    friend Q_CORE_EXPORT QDataStream &operator<<(QDataStream &ds, const QTimeZone &tz);
#endif
    friend class QTimeZonePrivate;
    friend class QDateTime;
    friend class QDateTimePrivate;
    QSharedDataPointer<QTimeZonePrivate> d;
};

Q_DECLARE_TYPEINFO(QTimeZone::OffsetData, Q_RELOCATABLE_TYPE);
Q_DECLARE_SHARED(QTimeZone)

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &ds, const QTimeZone &tz);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &ds, QTimeZone &tz);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug dbg, const QTimeZone &tz);
#endif

#if __cpp_lib_chrono >= 201907L
// zoned_time
template <typename> // QT_POST_CXX17_API_IN_EXPORTED_CLASS
inline QDateTime QDateTime::fromStdZonedTime(const std::chrono::zoned_time<
                                                std::chrono::milliseconds,
                                                const std::chrono::time_zone *
                                             > &time)
{
    const auto sysTime = time.get_sys_time();
    const QTimeZone timeZone = QTimeZone::fromStdTimeZonePtr(time.get_time_zone());
    return fromMSecsSinceEpoch(sysTime.time_since_epoch().count(), timeZone);
}
#endif

QT_END_NAMESPACE

#endif // QTIMEZONE_H
