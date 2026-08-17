// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSPLASHSCREEN_H
#define QSPLASHSCREEN_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtGui/qpixmap.h>
#include <QtWidgets/qwidget.h>

QT_REQUIRE_CONFIG(splashscreen);

QT_BEGIN_NAMESPACE

class QSplashScreenPrivate;

class Q_WIDGETS_EXPORT QSplashScreen : public QWidget
{
    Q_OBJECT
public:
    explicit QSplashScreen(const QPixmap &pixmap = QPixmap(), Qt::WindowFlags f = Qt::WindowFlags());
    QSplashScreen(QScreen *screen, const QPixmap &pixmap = QPixmap(), Qt::WindowFlags f = Qt::WindowFlags());
    virtual ~QSplashScreen();

    void setPixmap(const QPixmap &pixmap);
    const QPixmap pixmap() const;
    void finish(QWidget *w);
    void repaint();
    QString message() const;

public Q_SLOTS:
    void showMessage(const QString &message, int alignment = Qt::AlignLeft,
                  const QColor &color = Qt::black);
    void clearMessage();

Q_SIGNALS:
    void messageChanged(const QString &message);

protected:
    bool event(QEvent *e) override;
    virtual void drawContents(QPainter *painter);
    void mousePressEvent(QMouseEvent *) override;

private:
    Q_DISABLE_COPY(QSplashScreen)
    Q_DECLARE_PRIVATE(QSplashScreen)
};

QT_END_NAMESPACE

#endif // QSPLASHSCREEN_H
