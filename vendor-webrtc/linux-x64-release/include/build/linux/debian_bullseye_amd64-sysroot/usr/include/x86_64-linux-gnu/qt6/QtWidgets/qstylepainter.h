// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSTYLEPAINTER_H
#define QSTYLEPAINTER_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtGui/qpainter.h>
#include <QtWidgets/qstyle.h>
#include <QtWidgets/qwidget.h>

QT_BEGIN_NAMESPACE


class QStylePainter : public QPainter
{
public:
    inline QStylePainter() : QPainter(), widget(nullptr), wstyle(nullptr) {}
    inline explicit QStylePainter(QWidget *w) { begin(w, w); }
    inline QStylePainter(QPaintDevice *pd, QWidget *w) { begin(pd, w); }
    inline bool begin(QWidget *w) { return begin(w, w); }
    inline bool begin(QPaintDevice *pd, QWidget *w) {
        Q_ASSERT_X(w, "QStylePainter::QStylePainter", "Widget must be non-zero");
        widget = w;
        wstyle = w->style();
        const bool res = QPainter::begin(pd);
        setRenderHint(QPainter::SmoothPixmapTransform);
        return res;
    };
    inline void drawPrimitive(QStyle::PrimitiveElement pe, const QStyleOption &opt);
    inline void drawControl(QStyle::ControlElement ce, const QStyleOption &opt);
    inline void drawComplexControl(QStyle::ComplexControl cc, const QStyleOptionComplex &opt);
    inline void drawItemText(const QRect &r, int flags, const QPalette &pal, bool enabled,
                             const QString &text, QPalette::ColorRole textRole = QPalette::NoRole);
    inline void drawItemPixmap(const QRect &r, int flags, const QPixmap &pixmap);
    inline QStyle *style() const { return wstyle; }

private:
    QWidget *widget;
    QStyle *wstyle;
    Q_DISABLE_COPY(QStylePainter)
};

void QStylePainter::drawPrimitive(QStyle::PrimitiveElement pe, const QStyleOption &opt)
{
    wstyle->drawPrimitive(pe, &opt, this, widget);
}

void QStylePainter::drawControl(QStyle::ControlElement ce, const QStyleOption &opt)
{
    wstyle->drawControl(ce, &opt, this, widget);
}

void QStylePainter::drawComplexControl(QStyle::ComplexControl cc, const QStyleOptionComplex &opt)
{
    wstyle->drawComplexControl(cc, &opt, this, widget);
}

void QStylePainter::drawItemText(const QRect &r, int flags, const QPalette &pal, bool enabled,
                                 const QString &text, QPalette::ColorRole textRole)
{
    wstyle->drawItemText(this, r, flags, pal, enabled, text, textRole);
}

void QStylePainter::drawItemPixmap(const QRect &r, int flags, const QPixmap &pixmap)
{
    wstyle->drawItemPixmap(this, r, flags, pixmap);
}

QT_END_NAMESPACE

#endif // QSTYLEPAINTER_H
