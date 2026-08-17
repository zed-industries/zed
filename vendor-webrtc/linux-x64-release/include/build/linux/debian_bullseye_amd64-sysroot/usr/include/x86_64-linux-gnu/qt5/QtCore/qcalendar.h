/****************************************************************************
**
** Copyright (C) 2019 The Qt Company Ltd.
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

#ifndef QCALENDAR_H
#define QCALENDAR_H

#include <limits>

#include <QtCore/qglobal.h>
#include <QtCore/qlocale.h>
#include <QtCore/qstring.h>
#include <QtCore/qstringview.h>

/* Suggested enum names for other calendars known to CLDR (v33.1)

   Not yet implemented - see QCalendar::System - contributions welcome:

   * Buddhist -- Thai Buddhist, to be specific
   * Chinese
   * Coptic
   * Dangi -- Korean
   * Ethiopic (Amete Mihret - epoch approx. 8 C.E.)
   * EthiopicAmeteAlem (Amete Alem - epoch approx. 5493 B.C.E; data from
     type="ethiopic-amete-alem", an alias for type="ethioaa")
   * Hebrew
   * Indian -- National
   * Islamic -- Based on astronomical observations, not predictions, so hard to
     implement. CLDR's data for type="islamic" apply, unless overridden, to the
     other Islamic calendar variants, i.e. IslamicCivil, above, and the three
     following. See QHijriCalendar, a common base to provide that data.
   * IslamicTabular -- tabular, astronomical epoch (same as IslamicCivil, except
     for epoch), CLDR type="islamic-tbla"
   * Saudi -- Saudi Arabia, sighting; CLDR type="islamic-rgsa"
   * UmmAlQura -- Umm al-Qura, Saudi Arabia, calculated; CLDR type="islamic-umalqura"
   * Iso8601 -- as Gregorian, but treating ISO 8601 weeks as "months"
   * Japanese -- Imperial calendar
   * Minguo -- Republic of China, Taiwan; CLDR type="roc"

   See:
   http://www.unicode.org/repos/cldr/tags/latest/common/bcp47/calendar.xml

   These can potentially be supported, as features, using CLDR's data; any
   others shall need hand-crafted localization data; it would probably be best
   to do that by contributing data for them to CLDR.
*/

QT_BEGIN_NAMESPACE

class QCalendarBackend;
class QDate;

class Q_CORE_EXPORT QCalendar
{
    Q_GADGET
public:
    // (Extra parentheses to suppress bogus reading of min() as a macro.)
    enum : int { Unspecified = (std::numeric_limits<int>::min)() };
    struct YearMonthDay
    {
        YearMonthDay() = default;
        YearMonthDay(int y, int m = 1, int d = 1) : year(y), month(m), day(d) {}

        bool isValid() const
        { return month != Unspecified && day != Unspecified; }
        // (The first year supported by QDate has year == Unspecified.)

        int year = Unspecified;
        int month = Unspecified;
        int day = Unspecified;
    };
    // Feature (\w+)calendar uses CLDR type="\1" data, except as noted in type="..." comments below
    enum class System
    {
        Gregorian, // CLDR: type = "gregory", alias = "gregorian"
#ifndef QT_BOOTSTRAPPED
        Julian = 8,
        Milankovic = 9,
#endif // These are Roman-based, so share Gregorian's CLDR data

        // Feature-controlled calendars:
#if QT_CONFIG(jalalicalendar) // type="persian"
        Jalali = 10,
#endif
#if QT_CONFIG(islamiccivilcalendar) // type="islamic-civil", uses data from type="islamic"
        IslamicCivil = 11,
        // tabular, civil epoch
        // 30 year cycle, leap on 2, 5, 7, 10, 13, 16, 18, 21, 24, 26 and 29
        // (Other variants: 2, 5, 8, (10|11), 13, 16, 19, 21, 24, 27 and 29.)
#endif

        Last = 11, // Highest number of any above
        User = -1
    };
    // New entries must be added to the \enum doc in qcalendar.cpp and
    // handled in QCalendarBackend::fromEnum()
    Q_ENUM(System)

    explicit QCalendar(); // Gregorian, optimised
    explicit QCalendar(System system);
    explicit QCalendar(QLatin1String name);
    explicit QCalendar(QStringView name);

    // QCalendar is a trivially copyable value type.
    bool isValid() const { return d != nullptr; }

    // Date queries:
    int daysInMonth(int month, int year = Unspecified) const;
    int daysInYear(int year) const;
    int monthsInYear(int year) const;
    bool isDateValid(int year, int month, int day) const;

    // Leap years:
    bool isLeapYear(int year) const;

    // Properties of the calendar:
    bool isGregorian() const;
    bool isLunar() const;
    bool isLuniSolar() const;
    bool isSolar() const;
    bool isProleptic() const;
    bool hasYearZero() const;
    int maximumDaysInMonth() const;
    int minimumDaysInMonth() const;
    int maximumMonthsInYear() const;
    QString name() const;

    // QDate conversions:
    QDate dateFromParts(int year, int month, int day) const;
    QDate dateFromParts(const YearMonthDay &parts) const;
    YearMonthDay partsFromDate(QDate date) const;
    int dayOfWeek(QDate date) const;

    // Month and week-day names (as in QLocale):
    QString monthName(const QLocale &locale, int month, int year = Unspecified,
                      QLocale::FormatType format=QLocale::LongFormat) const;
    QString standaloneMonthName(const QLocale &locale, int month, int year = Unspecified,
                                QLocale::FormatType format = QLocale::LongFormat) const;
    QString weekDayName(const QLocale &locale, int day,
                        QLocale::FormatType format = QLocale::LongFormat) const;
    QString standaloneWeekDayName(const QLocale &locale, int day,
                                  QLocale::FormatType format=QLocale::LongFormat) const;

    // Formatting of date-times:
    QString dateTimeToString(QStringView format, const QDateTime &datetime,
                             const QDate &dateOnly, const QTime &timeOnly,
                             const QLocale &locale) const;

    // What's available ?
    static QStringList availableCalendars();
private:
    // Always supplied by QCalendarBackend and expected to be a singleton
    const QCalendarBackend *d;
};

QT_END_NAMESPACE

#endif // QCALENDAR_H
