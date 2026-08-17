/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtWidgets module of the Qt Toolkit.
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

#ifndef QDESKTOPWIDGET_H
#define QDESKTOPWIDGET_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qwidget.h>

QT_BEGIN_NAMESPACE


class QApplication;
class QDesktopWidgetPrivate;

class Q_WIDGETS_EXPORT QDesktopWidget : public QWidget
{
    Q_OBJECT
#if QT_DEPRECATED_SINCE(5, 11)
    Q_PROPERTY(bool virtualDesktop READ isVirtualDesktop)
    Q_PROPERTY(int screenCount READ screenCount NOTIFY screenCountChanged)
    Q_PROPERTY(int primaryScreen READ primaryScreen NOTIFY primaryScreenChanged)
#endif
public:
    QDesktopWidget();
    ~QDesktopWidget();

    int screenNumber(const QWidget *widget = nullptr) const;
    const QRect screenGeometry(const QWidget *widget) const;
    const QRect availableGeometry(const QWidget *widget) const;

#if QT_DEPRECATED_SINCE(5, 11)
    QT_DEPRECATED_X("Use QScreen::virtualSiblings() of primary screen")  bool isVirtualDesktop() const;

    QT_DEPRECATED_X("Use QGuiApplication::screens()") int numScreens() const;
    QT_DEPRECATED_X("Use QGuiApplication::screens()") int screenCount() const;
    QT_DEPRECATED_X("Use QGuiApplication::primaryScreen()") int primaryScreen() const;

    QT_DEPRECATED_X("Use QGuiApplication::screenAt()") int screenNumber(const QPoint &) const;

    QT_DEPRECATED_X("Use QScreen") QWidget *screen(int screen = -1);

    QT_DEPRECATED_X("Use QGuiApplication::screens()") const QRect screenGeometry(int screen = -1) const;
    QT_DEPRECATED_X("Use QGuiApplication::screenAt()") const QRect screenGeometry(const QPoint &point) const
    {
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
        return screenGeometry(screenNumber(point));
QT_WARNING_POP
    }

    QT_DEPRECATED_X("Use QGuiApplication::screens()") const QRect availableGeometry(int screen = -1) const;
    QT_DEPRECATED_X("Use QGuiApplication::screenAt()") const QRect availableGeometry(const QPoint &point) const
    {
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
    return availableGeometry(screenNumber(point));
QT_WARNING_POP
    }

Q_SIGNALS:
    QT_DEPRECATED_X("Use QScreen::geometryChanged()") void resized(int);
    QT_DEPRECATED_X("Use QScreen::availableGeometryChanged()") void workAreaResized(int);
    QT_DEPRECATED_X("Use QGuiApplication::screenAdded/Removed()") void screenCountChanged(int);
    QT_DEPRECATED_X("Use QGuiApplication::primaryScreenChanged()") void primaryScreenChanged();
#endif

protected:
    void resizeEvent(QResizeEvent *e) override;

private:
    Q_DISABLE_COPY(QDesktopWidget)
    Q_DECLARE_PRIVATE(QDesktopWidget)
    Q_PRIVATE_SLOT(d_func(), void _q_updateScreens())
    Q_PRIVATE_SLOT(d_func(), void _q_availableGeometryChanged())

    friend class QApplication;
    friend class QApplicationPrivate;
};

#if QT_DEPRECATED_SINCE(5, 11)
inline int QDesktopWidget::screenCount() const
{
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
    return numScreens();
QT_WARNING_POP
}
#endif

QT_END_NAMESPACE

#endif // QDESKTOPWIDGET_H
