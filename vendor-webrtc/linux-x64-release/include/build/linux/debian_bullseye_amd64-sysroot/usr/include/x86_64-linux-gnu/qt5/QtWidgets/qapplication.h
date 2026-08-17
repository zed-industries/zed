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

#ifndef QAPPLICATION_H
#define QAPPLICATION_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qcoreapplication.h>
#include <QtGui/qwindowdefs.h>
#include <QtCore/qpoint.h>
#include <QtCore/qsize.h>
#include <QtGui/qcursor.h>
#ifdef QT_INCLUDE_COMPAT
# include <QtWidgets/qdesktopwidget.h>
#endif
#include <QtGui/qguiapplication.h>

QT_BEGIN_NAMESPACE


class QDesktopWidget;
class QStyle;
class QEventLoop;
class QIcon;
template <typename T> class QList;
class QLocale;
class QPlatformNativeInterface;

class QApplication;
class QApplicationPrivate;
#if defined(qApp)
#undef qApp
#endif
#define qApp (static_cast<QApplication *>(QCoreApplication::instance()))

class Q_WIDGETS_EXPORT QApplication : public QGuiApplication
{
    Q_OBJECT
    Q_PROPERTY(QIcon windowIcon READ windowIcon WRITE setWindowIcon)
    Q_PROPERTY(int cursorFlashTime READ cursorFlashTime WRITE setCursorFlashTime)
    Q_PROPERTY(int doubleClickInterval  READ doubleClickInterval WRITE setDoubleClickInterval)
    Q_PROPERTY(int keyboardInputInterval READ keyboardInputInterval WRITE setKeyboardInputInterval)
#if QT_CONFIG(wheelevent)
    Q_PROPERTY(int wheelScrollLines  READ wheelScrollLines WRITE setWheelScrollLines)
#endif
#if QT_DEPRECATED_SINCE(5, 15)
    Q_PROPERTY(QSize globalStrut READ globalStrut WRITE setGlobalStrut)
#endif
    Q_PROPERTY(int startDragTime  READ startDragTime WRITE setStartDragTime)
    Q_PROPERTY(int startDragDistance  READ startDragDistance WRITE setStartDragDistance)
#ifndef QT_NO_STYLE_STYLESHEET
    Q_PROPERTY(QString styleSheet READ styleSheet WRITE setStyleSheet)
#endif
    Q_PROPERTY(bool autoSipEnabled READ autoSipEnabled WRITE setAutoSipEnabled)

public:
#ifdef Q_QDOC
    QApplication(int &argc, char **argv);
#else
    QApplication(int &argc, char **argv, int = ApplicationFlags);
#endif
    virtual ~QApplication();

    static QStyle *style();
    static void setStyle(QStyle*);
    static QStyle *setStyle(const QString&);
    enum ColorSpec { NormalColor=0, CustomColor=1, ManyColor=2 };
#if QT_DEPRECATED_SINCE(5, 8)
    QT_DEPRECATED static int colorSpec();
    QT_DEPRECATED static void setColorSpec(int);
#endif // QT_DEPRECATED_SINCE(5, 8)
#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED static inline void setGraphicsSystem(const QString &) {}
#endif

    using QGuiApplication::palette;
    static QPalette palette(const QWidget *);
    static QPalette palette(const char *className);
    static void setPalette(const QPalette &, const char* className = nullptr);
    static QFont font();
    static QFont font(const QWidget*);
    static QFont font(const char *className);
    static void setFont(const QFont &, const char* className = nullptr);
    static QFontMetrics fontMetrics();

#if QT_VERSION < 0x060000 // remove these forwarders in Qt 6
    static void setWindowIcon(const QIcon &icon);
    static QIcon windowIcon();
#endif

    static QWidgetList allWidgets();
    static QWidgetList topLevelWidgets();

    static QDesktopWidget *desktop();

    static QWidget *activePopupWidget();
    static QWidget *activeModalWidget();
    static QWidget *focusWidget();

    static QWidget *activeWindow();
    static void setActiveWindow(QWidget* act);

    static QWidget *widgetAt(const QPoint &p);
    static inline QWidget *widgetAt(int x, int y) { return widgetAt(QPoint(x, y)); }
    static QWidget *topLevelAt(const QPoint &p);
    static inline QWidget *topLevelAt(int x, int y)  { return topLevelAt(QPoint(x, y)); }

#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED static inline void syncX() {}
#endif
    static void beep();
    static void alert(QWidget *widget, int duration = 0);

    static void setCursorFlashTime(int);
    static int cursorFlashTime();

    static void setDoubleClickInterval(int);
    static int doubleClickInterval();

    static void setKeyboardInputInterval(int);
    static int keyboardInputInterval();

#if QT_CONFIG(wheelevent)
    static void setWheelScrollLines(int);
    static int wheelScrollLines();
#endif
#if QT_DEPRECATED_SINCE(5, 15)
    static void setGlobalStrut(const QSize &);
    static QSize globalStrut();
#endif

    static void setStartDragTime(int ms);
    static int startDragTime();
    static void setStartDragDistance(int l);
    static int startDragDistance();

    static bool isEffectEnabled(Qt::UIEffect);
    static void setEffectEnabled(Qt::UIEffect, bool enable = true);

#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED static QLocale keyboardInputLocale()
    { return qApp ? QGuiApplication::inputMethod()->locale() : QLocale::c(); }
    QT_DEPRECATED static Qt::LayoutDirection keyboardInputDirection()
    { return qApp ? QGuiApplication::inputMethod()->inputDirection() : Qt::LeftToRight; }
#endif

    static int exec();
    bool notify(QObject *, QEvent *) override;

#ifdef QT_KEYPAD_NAVIGATION
# if QT_DEPRECATED_SINCE(5, 13)
    static QT_DEPRECATED_X ("Use QApplication::setNavigationMode() instead")
    void setKeypadNavigationEnabled(bool);
    static QT_DEPRECATED_X ("Use QApplication::navigationMode() instead")
    bool keypadNavigationEnabled();
# endif
    static void setNavigationMode(Qt::NavigationMode mode);
    static Qt::NavigationMode navigationMode();
#endif

Q_SIGNALS:
    void focusChanged(QWidget *old, QWidget *now);

public:
    QString styleSheet() const;
public Q_SLOTS:
#ifndef QT_NO_STYLE_STYLESHEET
    void setStyleSheet(const QString& sheet);
#endif
    void setAutoSipEnabled(const bool enabled);
    bool autoSipEnabled() const;
    static void closeAllWindows();
    static void aboutQt();

protected:
    bool event(QEvent *) override;
    bool compressEvent(QEvent *, QObject *receiver, QPostEventList *) override;

private:
    Q_DISABLE_COPY(QApplication)
    Q_DECLARE_PRIVATE(QApplication)

    friend class QGraphicsWidget;
    friend class QGraphicsItem;
    friend class QGraphicsScene;
    friend class QGraphicsScenePrivate;
    friend class QWidget;
    friend class QWidgetPrivate;
    friend class QWidgetWindow;
    friend class QTranslator;
    friend class QWidgetAnimator;
#ifndef QT_NO_SHORTCUT
    friend class QShortcut;
    friend class QLineEdit;
    friend class QWidgetTextControl;
#endif
    friend class QAction;

#ifndef QT_NO_GESTURES
    friend class QGestureManager;
#endif
};

QT_END_NAMESPACE

#endif // QAPPLICATION_H
