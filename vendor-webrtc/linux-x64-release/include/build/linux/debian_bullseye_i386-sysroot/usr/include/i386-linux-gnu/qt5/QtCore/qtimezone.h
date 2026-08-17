/****************************************************************************
**
** Copyright (C) 2013 John Layt <jlayt@kde.org>
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


#ifndef QTIMEZONE_H
#define QTIMEZONE_H

#include <QtCore/qshareddata.h>
#include <QtCore/qlocale.h>
#include <QtCore/qdatetime.h>

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
    typedef QVector<OffsetData> OffsetDataList;

    QTimeZone() noexcept;
    explicit QTimeZone(const QByteArray &ianaId);
    explicit QTimeZone(int offsetSeconds);
    /*implicit*/ QTimeZone(const QByteArray &zoneId, int offsetSeconds, const QString &name,
              const QString &abbreviation, QLocale::Country country = QLocale::AnyCountry,
              const QString &comment = QString());
    QTimeZone(const QTimeZone &other);
    ~QTimeZone();

    QTimeZone &operator=(const QTimeZone &other);
    QTimeZone &operator=(QTimeZone &&other) noexcept { swap(other); return *this; }

    void swap(QTimeZone &other) noexcept
    { d.swap(other.d); }

    bool operator==(const QTimeZone &other) const;
    bool operator!=(const QTimeZone &other) const;

    bool isValid() const;

    QByteArray id() const;
    QLocale::Country country() const;
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
    static QList<QByteArray> availableTimeZoneIds(QLocale::Country country);
    static QList<QByteArray> availableTimeZoneIds(int offsetSeconds);

    static QByteArray ianaIdToWindowsId(const QByteArray &ianaId);
    static QByteArray windowsIdToDefaultIanaId(const QByteArray &windowsId);
    static QByteArray windowsIdToDefaultIanaId(const QByteArray &windowsId,
                                                QLocale::Country country);
    static QList<QByteArray> windowsIdToIanaIds(const QByteArray &windowsId);
    static QList<QByteArray> windowsIdToIanaIds(const QByteArray &windowsId,
                                                 QLocale::Country country);

#if (defined(Q_OS_DARWIN) || defined(Q_QDOC)) && !defined(QT_NO_SYSTEMLOCALE)
    static QTimeZone fromCFTimeZone(CFTimeZoneRef timeZone);
    CFTimeZoneRef toCFTimeZone() const Q_DECL_CF_RETURNS_RETAINED;
    static QTimeZone fromNSTimeZone(const NSTimeZone *timeZone);
    NSTimeZone *toNSTimeZone() const Q_DECL_NS_RETURNS_AUTORELEASED;
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

Q_DECLARE_TYPEINFO(QTimeZone::OffsetData, Q_MOVABLE_TYPE);
Q_DECLARE_SHARED(QTimeZone)

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &ds, const QTimeZone &tz);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &ds, QTimeZone &tz);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug dbg, const QTimeZone &tz);
#endif

QT_END_NAMESPACE

#endif // QTIMEZONE_H
