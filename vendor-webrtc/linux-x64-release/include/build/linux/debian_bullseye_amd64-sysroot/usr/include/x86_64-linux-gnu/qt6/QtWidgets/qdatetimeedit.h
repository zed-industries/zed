// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDATETIMEEDIT_H
#define QDATETIMEEDIT_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qdatetime.h>
#include <QtCore/qcalendar.h>
#include <QtCore/qvariant.h>
#include <QtWidgets/qabstractspinbox.h>

QT_REQUIRE_CONFIG(datetimeedit);

QT_BEGIN_NAMESPACE

class QDateTimeEditPrivate;
class QStyleOptionSpinBox;
class QCalendarWidget;

class Q_WIDGETS_EXPORT QDateTimeEdit : public QAbstractSpinBox
{
    Q_OBJECT

    Q_PROPERTY(QDateTime dateTime READ dateTime WRITE setDateTime NOTIFY dateTimeChanged USER true)
    Q_PROPERTY(QDate date READ date WRITE setDate NOTIFY dateChanged)
    Q_PROPERTY(QTime time READ time WRITE setTime NOTIFY timeChanged)
    Q_PROPERTY(QDateTime maximumDateTime READ maximumDateTime WRITE setMaximumDateTime
               RESET clearMaximumDateTime)
    Q_PROPERTY(QDateTime minimumDateTime READ minimumDateTime WRITE setMinimumDateTime
               RESET clearMinimumDateTime)
    Q_PROPERTY(QDate maximumDate READ maximumDate WRITE setMaximumDate RESET clearMaximumDate)
    Q_PROPERTY(QDate minimumDate READ minimumDate WRITE setMinimumDate RESET clearMinimumDate)
    Q_PROPERTY(QTime maximumTime READ maximumTime WRITE setMaximumTime RESET clearMaximumTime)
    Q_PROPERTY(QTime minimumTime READ minimumTime WRITE setMinimumTime RESET clearMinimumTime)
    Q_PROPERTY(Section currentSection READ currentSection WRITE setCurrentSection)
    Q_PROPERTY(Sections displayedSections READ displayedSections)
    Q_PROPERTY(QString displayFormat READ displayFormat WRITE setDisplayFormat)
    Q_PROPERTY(bool calendarPopup READ calendarPopup WRITE setCalendarPopup)
    Q_PROPERTY(int currentSectionIndex READ currentSectionIndex WRITE setCurrentSectionIndex)
    Q_PROPERTY(int sectionCount READ sectionCount)
    Q_PROPERTY(Qt::TimeSpec timeSpec READ timeSpec WRITE setTimeSpec)
public:
    enum Section { // a sub-type of QDateTimeParser's like-named enum.
        NoSection = 0x0000,
        AmPmSection = 0x0001,
        MSecSection = 0x0002,
        SecondSection = 0x0004,
        MinuteSection = 0x0008,
        HourSection   = 0x0010,
        DaySection    = 0x0100,
        MonthSection  = 0x0200,
        YearSection   = 0x0400,
        TimeSections_Mask = AmPmSection|MSecSection|SecondSection|MinuteSection|HourSection,
        DateSections_Mask = DaySection|MonthSection|YearSection
    };
    Q_ENUM(Section)

    Q_DECLARE_FLAGS(Sections, Section)
    Q_FLAG(Sections)

    explicit QDateTimeEdit(QWidget *parent = nullptr);
    explicit QDateTimeEdit(const QDateTime &dt, QWidget *parent = nullptr);
    explicit QDateTimeEdit(QDate d, QWidget *parent = nullptr);
    explicit QDateTimeEdit(QTime t, QWidget *parent = nullptr);
    ~QDateTimeEdit();

    QDateTime dateTime() const;
    QDate date() const;
    QTime time() const;

    QCalendar calendar() const;
    void setCalendar(QCalendar calendar);

    QDateTime minimumDateTime() const;
    void clearMinimumDateTime();
    void setMinimumDateTime(const QDateTime &dt);

    QDateTime maximumDateTime() const;
    void clearMaximumDateTime();
    void setMaximumDateTime(const QDateTime &dt);

    void setDateTimeRange(const QDateTime &min, const QDateTime &max);

    QDate minimumDate() const;
    void setMinimumDate(QDate min);
    void clearMinimumDate();

    QDate maximumDate() const;
    void setMaximumDate(QDate max);
    void clearMaximumDate();

    void setDateRange(QDate min, QDate max);

    QTime minimumTime() const;
    void setMinimumTime(QTime min);
    void clearMinimumTime();

    QTime maximumTime() const;
    void setMaximumTime(QTime max);
    void clearMaximumTime();

    void setTimeRange(QTime min, QTime max);

    Sections displayedSections() const;
    Section currentSection() const;
    Section sectionAt(int index) const;
    void setCurrentSection(Section section);

    int currentSectionIndex() const;
    void setCurrentSectionIndex(int index);

    QCalendarWidget *calendarWidget() const;
    void setCalendarWidget(QCalendarWidget *calendarWidget);

    int sectionCount() const;

    void setSelectedSection(Section section);

    QString sectionText(Section section) const;

    QString displayFormat() const;
    void setDisplayFormat(const QString &format);

    bool calendarPopup() const;
    void setCalendarPopup(bool enable);

    Qt::TimeSpec timeSpec() const;
    void setTimeSpec(Qt::TimeSpec spec);

    QSize sizeHint() const override;

    void clear() override;
    void stepBy(int steps) override;

    bool event(QEvent *event) override;
Q_SIGNALS:
    void dateTimeChanged(const QDateTime &dateTime);
    void timeChanged(QTime time);
    void dateChanged(QDate date);

public Q_SLOTS:
    void setDateTime(const QDateTime &dateTime);
    void setDate(QDate date);
    void setTime(QTime time);

protected:
    void keyPressEvent(QKeyEvent *event) override;
#if QT_CONFIG(wheelevent)
    void wheelEvent(QWheelEvent *event) override;
#endif
    void focusInEvent(QFocusEvent *event) override;
    bool focusNextPrevChild(bool next) override;
    QValidator::State validate(QString &input, int &pos) const override;
    void fixup(QString &input) const override;

    virtual QDateTime dateTimeFromText(const QString &text) const;
    virtual QString textFromDateTime(const QDateTime &dt) const;
    StepEnabled stepEnabled() const override;
    void mousePressEvent(QMouseEvent *event) override;
    void paintEvent(QPaintEvent *event) override;
    void initStyleOption(QStyleOptionSpinBox *option) const override;

    QDateTimeEdit(const QVariant &val, QMetaType::Type parserType, QWidget *parent = nullptr);
private:
    Q_DECLARE_PRIVATE(QDateTimeEdit)
    Q_DISABLE_COPY(QDateTimeEdit)

    Q_PRIVATE_SLOT(d_func(), void _q_resetButton())
};

class Q_WIDGETS_EXPORT QTimeEdit : public QDateTimeEdit
{
    Q_OBJECT
    Q_PROPERTY(QTime time READ time WRITE setTime NOTIFY userTimeChanged USER true)
public:
    explicit QTimeEdit(QWidget *parent = nullptr);
    explicit QTimeEdit(QTime time, QWidget *parent = nullptr);
    ~QTimeEdit();

Q_SIGNALS:
    void userTimeChanged(QTime time);
};

class Q_WIDGETS_EXPORT QDateEdit : public QDateTimeEdit
{
    Q_OBJECT
    Q_PROPERTY(QDate date READ date WRITE setDate NOTIFY userDateChanged USER true)
public:
    explicit QDateEdit(QWidget *parent = nullptr);
    explicit QDateEdit(QDate date, QWidget *parent = nullptr);
    ~QDateEdit();

Q_SIGNALS:
    void userDateChanged(QDate date);
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QDateTimeEdit::Sections)

QT_END_NAMESPACE

#endif // QDATETIMEEDIT_H
