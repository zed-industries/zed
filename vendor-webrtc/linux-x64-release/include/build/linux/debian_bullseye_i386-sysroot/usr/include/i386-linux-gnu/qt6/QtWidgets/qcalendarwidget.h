// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QCALENDARWIDGET_H
#define QCALENDARWIDGET_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qwidget.h>
#include <QtCore/qdatetime.h>

QT_REQUIRE_CONFIG(calendarwidget);

QT_BEGIN_NAMESPACE

class QDate;
class QTextCharFormat;
class QCalendarWidgetPrivate;

class Q_WIDGETS_EXPORT QCalendarWidget : public QWidget
{
    Q_OBJECT
    Q_ENUMS(Qt::DayOfWeek)
    Q_PROPERTY(QDate selectedDate READ selectedDate WRITE setSelectedDate)
    Q_PROPERTY(QDate minimumDate READ minimumDate WRITE setMinimumDate)
    Q_PROPERTY(QDate maximumDate READ maximumDate WRITE setMaximumDate)
    Q_PROPERTY(Qt::DayOfWeek firstDayOfWeek READ firstDayOfWeek WRITE setFirstDayOfWeek)
    Q_PROPERTY(bool gridVisible READ isGridVisible WRITE setGridVisible)
    Q_PROPERTY(SelectionMode selectionMode READ selectionMode WRITE setSelectionMode)
    Q_PROPERTY(HorizontalHeaderFormat horizontalHeaderFormat READ horizontalHeaderFormat
               WRITE setHorizontalHeaderFormat)
    Q_PROPERTY(VerticalHeaderFormat verticalHeaderFormat READ verticalHeaderFormat
               WRITE setVerticalHeaderFormat)
    Q_PROPERTY(bool navigationBarVisible READ isNavigationBarVisible WRITE setNavigationBarVisible)
    Q_PROPERTY(bool dateEditEnabled READ isDateEditEnabled WRITE setDateEditEnabled)
    Q_PROPERTY(int dateEditAcceptDelay READ dateEditAcceptDelay WRITE setDateEditAcceptDelay)

public:
    enum HorizontalHeaderFormat {
        NoHorizontalHeader,
        SingleLetterDayNames,
        ShortDayNames,
        LongDayNames
    };
    Q_ENUM(HorizontalHeaderFormat)

    enum VerticalHeaderFormat {
        NoVerticalHeader,
        ISOWeekNumbers
    };
    Q_ENUM(VerticalHeaderFormat)

    enum SelectionMode {
        NoSelection,
        SingleSelection
    };
    Q_ENUM(SelectionMode)

    explicit QCalendarWidget(QWidget *parent = nullptr);
    ~QCalendarWidget();

    virtual QSize sizeHint() const override;
    virtual QSize minimumSizeHint() const override;

    QDate selectedDate() const;

    int yearShown() const;
    int monthShown() const;

    QDate minimumDate() const;
    void setMinimumDate(QDate date);

    QDate maximumDate() const;
    void setMaximumDate(QDate date);

    Qt::DayOfWeek firstDayOfWeek() const;
    void setFirstDayOfWeek(Qt::DayOfWeek dayOfWeek);

    bool isNavigationBarVisible() const;
    bool isGridVisible() const;

    QCalendar calendar() const;
    void setCalendar(QCalendar calendar);

    SelectionMode selectionMode() const;
    void setSelectionMode(SelectionMode mode);

    HorizontalHeaderFormat horizontalHeaderFormat() const;
    void setHorizontalHeaderFormat(HorizontalHeaderFormat format);

    VerticalHeaderFormat verticalHeaderFormat() const;
    void setVerticalHeaderFormat(VerticalHeaderFormat format);

    QTextCharFormat headerTextFormat() const;
    void setHeaderTextFormat(const QTextCharFormat &format);

    QTextCharFormat weekdayTextFormat(Qt::DayOfWeek dayOfWeek) const;
    void setWeekdayTextFormat(Qt::DayOfWeek dayOfWeek, const QTextCharFormat &format);

    QMap<QDate, QTextCharFormat> dateTextFormat() const;
    QTextCharFormat dateTextFormat(QDate date) const;
    void setDateTextFormat(QDate date, const QTextCharFormat &format);

    bool isDateEditEnabled() const;
    void setDateEditEnabled(bool enable);

    int dateEditAcceptDelay() const;
    void setDateEditAcceptDelay(int delay);

protected:
    bool event(QEvent *event) override;
    bool eventFilter(QObject *watched, QEvent *event) override;
    void mousePressEvent(QMouseEvent *event) override;
    void resizeEvent(QResizeEvent * event) override;
    void keyPressEvent(QKeyEvent * event) override;

    virtual void paintCell(QPainter *painter, const QRect &rect, QDate date) const;
    void updateCell(QDate date);
    void updateCells();

public Q_SLOTS:
    void setSelectedDate(QDate date);
    void setDateRange(QDate min, QDate max);
    void setCurrentPage(int year, int month);
    void setGridVisible(bool show);
    void setNavigationBarVisible(bool visible);
    void showNextMonth();
    void showPreviousMonth();
    void showNextYear();
    void showPreviousYear();
    void showSelectedDate();
    void showToday();

Q_SIGNALS:
    void selectionChanged();
    void clicked(QDate date);
    void activated(QDate date);
    void currentPageChanged(int year, int month);

private:
    Q_DECLARE_PRIVATE(QCalendarWidget)
    Q_DISABLE_COPY(QCalendarWidget)

    Q_PRIVATE_SLOT(d_func(), void _q_slotShowDate(QDate date))
    Q_PRIVATE_SLOT(d_func(), void _q_slotChangeDate(QDate date))
    Q_PRIVATE_SLOT(d_func(), void _q_slotChangeDate(QDate date, bool changeMonth))
    Q_PRIVATE_SLOT(d_func(), void _q_editingFinished())
    Q_PRIVATE_SLOT(d_func(), void _q_prevMonthClicked())
    Q_PRIVATE_SLOT(d_func(), void _q_nextMonthClicked())
    Q_PRIVATE_SLOT(d_func(), void _q_yearEditingFinished())
    Q_PRIVATE_SLOT(d_func(), void _q_yearClicked())
    Q_PRIVATE_SLOT(d_func(), void _q_monthChanged(QAction *act))

};

QT_END_NAMESPACE

#endif // QCALENDARWIDGET_H
