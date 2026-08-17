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

#ifndef QWIDGET_H
#define QWIDGET_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtGui/qwindowdefs.h>
#include <QtCore/qobject.h>
#include <QtCore/qmargins.h>
#include <QtGui/qpaintdevice.h>
#include <QtGui/qpalette.h>
#include <QtGui/qfont.h>
#include <QtGui/qfontmetrics.h>
#include <QtGui/qfontinfo.h>
#include <QtWidgets/qsizepolicy.h>
#include <QtGui/qregion.h>
#include <QtGui/qbrush.h>
#include <QtGui/qcursor.h>
#include <QtGui/qkeysequence.h>

#ifdef QT_INCLUDE_COMPAT
#include <QtGui/qevent.h>
#endif

QT_BEGIN_NAMESPACE


class QLayout;
class QWSRegionManager;
class QStyle;
class QAction;
class QVariant;
class QWindow;
class QActionEvent;
class QMouseEvent;
class QWheelEvent;
class QHoverEvent;
class QKeyEvent;
class QFocusEvent;
class QPaintEvent;
class QMoveEvent;
class QResizeEvent;
class QCloseEvent;
class QContextMenuEvent;
class QInputMethodEvent;
class QTabletEvent;
class QDragEnterEvent;
class QDragMoveEvent;
class QDragLeaveEvent;
class QDropEvent;
class QScreen;
class QShowEvent;
class QHideEvent;
class QIcon;
class QBackingStore;
class QPlatformWindow;
class QLocale;
class QGraphicsProxyWidget;
class QGraphicsEffect;
class QRasterWindowSurface;
class QUnifiedToolbarSurface;
class QPixmap;
#ifndef QT_NO_DEBUG_STREAM
class QDebug;
#endif

class QWidgetData
{
public:
    WId winid;
    uint widget_attributes;
    Qt::WindowFlags window_flags;
    uint window_state : 4;
    uint focus_policy : 4;
    uint sizehint_forced :1;
    uint is_closing :1;
    uint in_show : 1;
    uint in_set_window_state : 1;
    mutable uint fstrut_dirty : 1;
    uint context_menu_policy : 3;
    uint window_modality : 2;
    uint in_destructor : 1;
    uint unused : 13;
    QRect crect;
    mutable QPalette pal;
    QFont fnt;
    QRect wrect;
};

class QWidgetPrivate;

class Q_WIDGETS_EXPORT QWidget : public QObject, public QPaintDevice
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QWidget)

    Q_PROPERTY(bool modal READ isModal)
    Q_PROPERTY(Qt::WindowModality windowModality READ windowModality WRITE setWindowModality)
    Q_PROPERTY(bool enabled READ isEnabled WRITE setEnabled)
    Q_PROPERTY(QRect geometry READ geometry WRITE setGeometry)
    Q_PROPERTY(QRect frameGeometry READ frameGeometry)
    Q_PROPERTY(QRect normalGeometry READ normalGeometry)
    Q_PROPERTY(int x READ x)
    Q_PROPERTY(int y READ y)
    Q_PROPERTY(QPoint pos READ pos WRITE move DESIGNABLE false STORED false)
    Q_PROPERTY(QSize frameSize READ frameSize)
    Q_PROPERTY(QSize size READ size WRITE resize DESIGNABLE false STORED false)
    Q_PROPERTY(int width READ width)
    Q_PROPERTY(int height READ height)
    Q_PROPERTY(QRect rect READ rect)
    Q_PROPERTY(QRect childrenRect READ childrenRect)
    Q_PROPERTY(QRegion childrenRegion READ childrenRegion)
    Q_PROPERTY(QSizePolicy sizePolicy READ sizePolicy WRITE setSizePolicy)
    Q_PROPERTY(QSize minimumSize READ minimumSize WRITE setMinimumSize)
    Q_PROPERTY(QSize maximumSize READ maximumSize WRITE setMaximumSize)
    Q_PROPERTY(int minimumWidth READ minimumWidth WRITE setMinimumWidth STORED false DESIGNABLE false)
    Q_PROPERTY(int minimumHeight READ minimumHeight WRITE setMinimumHeight STORED false DESIGNABLE false)
    Q_PROPERTY(int maximumWidth READ maximumWidth WRITE setMaximumWidth STORED false DESIGNABLE false)
    Q_PROPERTY(int maximumHeight READ maximumHeight WRITE setMaximumHeight STORED false DESIGNABLE false)
    Q_PROPERTY(QSize sizeIncrement READ sizeIncrement WRITE setSizeIncrement)
    Q_PROPERTY(QSize baseSize READ baseSize WRITE setBaseSize)
    Q_PROPERTY(QPalette palette READ palette WRITE setPalette)
    Q_PROPERTY(QFont font READ font WRITE setFont)
#ifndef QT_NO_CURSOR
    Q_PROPERTY(QCursor cursor READ cursor WRITE setCursor RESET unsetCursor)
#endif
    Q_PROPERTY(bool mouseTracking READ hasMouseTracking WRITE setMouseTracking)
    Q_PROPERTY(bool tabletTracking READ hasTabletTracking WRITE setTabletTracking)
    Q_PROPERTY(bool isActiveWindow READ isActiveWindow)
    Q_PROPERTY(Qt::FocusPolicy focusPolicy READ focusPolicy WRITE setFocusPolicy)
    Q_PROPERTY(bool focus READ hasFocus)
    Q_PROPERTY(Qt::ContextMenuPolicy contextMenuPolicy READ contextMenuPolicy WRITE setContextMenuPolicy)
    Q_PROPERTY(bool updatesEnabled READ updatesEnabled WRITE setUpdatesEnabled DESIGNABLE false)
    Q_PROPERTY(bool visible READ isVisible WRITE setVisible DESIGNABLE false)
    Q_PROPERTY(bool minimized READ isMinimized)
    Q_PROPERTY(bool maximized READ isMaximized)
    Q_PROPERTY(bool fullScreen READ isFullScreen)
    Q_PROPERTY(QSize sizeHint READ sizeHint)
    Q_PROPERTY(QSize minimumSizeHint READ minimumSizeHint)
    Q_PROPERTY(bool acceptDrops READ acceptDrops WRITE setAcceptDrops)
    Q_PROPERTY(QString windowTitle READ windowTitle WRITE setWindowTitle NOTIFY windowTitleChanged)
    Q_PROPERTY(QIcon windowIcon READ windowIcon WRITE setWindowIcon NOTIFY windowIconChanged)
    Q_PROPERTY(QString windowIconText READ windowIconText WRITE setWindowIconText NOTIFY windowIconTextChanged) // deprecated
    Q_PROPERTY(double windowOpacity READ windowOpacity WRITE setWindowOpacity)
    Q_PROPERTY(bool windowModified READ isWindowModified WRITE setWindowModified)
#ifndef QT_NO_TOOLTIP
    Q_PROPERTY(QString toolTip READ toolTip WRITE setToolTip)
    Q_PROPERTY(int toolTipDuration READ toolTipDuration WRITE setToolTipDuration)
#endif
#if QT_CONFIG(statustip)
    Q_PROPERTY(QString statusTip READ statusTip WRITE setStatusTip)
#endif
#if QT_CONFIG(whatsthis)
    Q_PROPERTY(QString whatsThis READ whatsThis WRITE setWhatsThis)
#endif
#ifndef QT_NO_ACCESSIBILITY
    Q_PROPERTY(QString accessibleName READ accessibleName WRITE setAccessibleName)
    Q_PROPERTY(QString accessibleDescription READ accessibleDescription WRITE setAccessibleDescription)
#endif
    Q_PROPERTY(Qt::LayoutDirection layoutDirection READ layoutDirection WRITE setLayoutDirection RESET unsetLayoutDirection)
    QDOC_PROPERTY(Qt::WindowFlags windowFlags READ windowFlags WRITE setWindowFlags)
    Q_PROPERTY(bool autoFillBackground READ autoFillBackground WRITE setAutoFillBackground)
#ifndef QT_NO_STYLE_STYLESHEET
    Q_PROPERTY(QString styleSheet READ styleSheet WRITE setStyleSheet)
#endif
    Q_PROPERTY(QLocale locale READ locale WRITE setLocale RESET unsetLocale)
    Q_PROPERTY(QString windowFilePath READ windowFilePath WRITE setWindowFilePath)
    Q_PROPERTY(Qt::InputMethodHints inputMethodHints READ inputMethodHints WRITE setInputMethodHints)

public:
    enum RenderFlag {
        DrawWindowBackground = 0x1,
        DrawChildren = 0x2,
        IgnoreMask = 0x4
    };
    Q_DECLARE_FLAGS(RenderFlags, RenderFlag)

    explicit QWidget(QWidget* parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());
    ~QWidget();

    int devType() const override;

    WId winId() const;
    void createWinId(); // internal, going away
    inline WId internalWinId() const { return data->winid; }
    WId effectiveWinId() const;

    // GUI style setting
    QStyle *style() const;
    void setStyle(QStyle *);
    // Widget types and states

    bool isTopLevel() const;
    bool isWindow() const;

    bool isModal() const;
    Qt::WindowModality windowModality() const;
    void setWindowModality(Qt::WindowModality windowModality);

    bool isEnabled() const;
    bool isEnabledTo(const QWidget *) const;
#if QT_DEPRECATED_SINCE(5, 13)
    QT_DEPRECATED_X ("Use isEnabled() instead")
    bool isEnabledToTLW() const;
#endif

public Q_SLOTS:
    void setEnabled(bool);
    void setDisabled(bool);
    void setWindowModified(bool);

    // Widget coordinates

public:
    QRect frameGeometry() const;
    const QRect &geometry() const;
    QRect normalGeometry() const;

    int x() const;
    int y() const;
    QPoint pos() const;
    QSize frameSize() const;
    QSize size() const;
    inline int width() const;
    inline int height() const;
    inline QRect rect() const;
    QRect childrenRect() const;
    QRegion childrenRegion() const;

    QSize minimumSize() const;
    QSize maximumSize() const;
    int minimumWidth() const;
    int minimumHeight() const;
    int maximumWidth() const;
    int maximumHeight() const;
    void setMinimumSize(const QSize &);
    void setMinimumSize(int minw, int minh);
    void setMaximumSize(const QSize &);
    void setMaximumSize(int maxw, int maxh);
    void setMinimumWidth(int minw);
    void setMinimumHeight(int minh);
    void setMaximumWidth(int maxw);
    void setMaximumHeight(int maxh);

#ifdef Q_QDOC
    void setupUi(QWidget *widget);
#endif

    QSize sizeIncrement() const;
    void setSizeIncrement(const QSize &);
    void setSizeIncrement(int w, int h);
    QSize baseSize() const;
    void setBaseSize(const QSize &);
    void setBaseSize(int basew, int baseh);

    void setFixedSize(const QSize &);
    void setFixedSize(int w, int h);
    void setFixedWidth(int w);
    void setFixedHeight(int h);

    // Widget coordinate mapping

    QPoint mapToGlobal(const QPoint &) const;
    QPoint mapFromGlobal(const QPoint &) const;
    QPoint mapToParent(const QPoint &) const;
    QPoint mapFromParent(const QPoint &) const;
    QPoint mapTo(const QWidget *, const QPoint &) const;
    QPoint mapFrom(const QWidget *, const QPoint &) const;

    QWidget *window() const;
    QWidget *nativeParentWidget() const;
    inline QWidget *topLevelWidget() const { return window(); }

    // Widget appearance functions
    const QPalette &palette() const;
    void setPalette(const QPalette &);

    void setBackgroundRole(QPalette::ColorRole);
    QPalette::ColorRole backgroundRole() const;

    void setForegroundRole(QPalette::ColorRole);
    QPalette::ColorRole foregroundRole() const;

    const QFont &font() const;
    void setFont(const QFont &);
    QFontMetrics fontMetrics() const;
    QFontInfo fontInfo() const;

#ifndef QT_NO_CURSOR
    QCursor cursor() const;
    void setCursor(const QCursor &);
    void unsetCursor();
#endif

    void setMouseTracking(bool enable);
    bool hasMouseTracking() const;
    bool underMouse() const;

    void setTabletTracking(bool enable);
    bool hasTabletTracking() const;

    void setMask(const QBitmap &);
    void setMask(const QRegion &);
    QRegion mask() const;
    void clearMask();

    void render(QPaintDevice *target, const QPoint &targetOffset = QPoint(),
                const QRegion &sourceRegion = QRegion(),
                RenderFlags renderFlags = RenderFlags(DrawWindowBackground | DrawChildren));

    void render(QPainter *painter, const QPoint &targetOffset = QPoint(),
                const QRegion &sourceRegion = QRegion(),
                RenderFlags renderFlags = RenderFlags(DrawWindowBackground | DrawChildren));

    Q_INVOKABLE QPixmap grab(const QRect &rectangle = QRect(QPoint(0, 0), QSize(-1, -1)));

#if QT_CONFIG(graphicseffect)
    QGraphicsEffect *graphicsEffect() const;
    void setGraphicsEffect(QGraphicsEffect *effect);
#endif // QT_CONFIG(graphicseffect)

#ifndef QT_NO_GESTURES
    void grabGesture(Qt::GestureType type, Qt::GestureFlags flags = Qt::GestureFlags());
    void ungrabGesture(Qt::GestureType type);
#endif

public Q_SLOTS:
    void setWindowTitle(const QString &);
#ifndef QT_NO_STYLE_STYLESHEET
    void setStyleSheet(const QString& styleSheet);
#endif
public:
#ifndef QT_NO_STYLE_STYLESHEET
    QString styleSheet() const;
#endif
    QString windowTitle() const;
    void setWindowIcon(const QIcon &icon);
    QIcon windowIcon() const;
    void setWindowIconText(const QString &);
    QString windowIconText() const;
    void setWindowRole(const QString &);
    QString windowRole() const;
    void setWindowFilePath(const QString &filePath);
    QString windowFilePath() const;

    void setWindowOpacity(qreal level);
    qreal windowOpacity() const;

    bool isWindowModified() const;
#ifndef QT_NO_TOOLTIP
    void setToolTip(const QString &);
    QString toolTip() const;
    void setToolTipDuration(int msec);
    int toolTipDuration() const;
#endif
#if QT_CONFIG(statustip)
    void setStatusTip(const QString &);
    QString statusTip() const;
#endif
#if QT_CONFIG(whatsthis)
    void setWhatsThis(const QString &);
    QString whatsThis() const;
#endif
#ifndef QT_NO_ACCESSIBILITY
    QString accessibleName() const;
    void setAccessibleName(const QString &name);
    QString accessibleDescription() const;
    void setAccessibleDescription(const QString &description);
#endif

    void setLayoutDirection(Qt::LayoutDirection direction);
    Qt::LayoutDirection layoutDirection() const;
    void unsetLayoutDirection();

    void setLocale(const QLocale &locale);
    QLocale locale() const;
    void unsetLocale();

    inline bool isRightToLeft() const { return layoutDirection() == Qt::RightToLeft; }
    inline bool isLeftToRight() const { return layoutDirection() == Qt::LeftToRight; }

public Q_SLOTS:
    inline void setFocus() { setFocus(Qt::OtherFocusReason); }

public:
    bool isActiveWindow() const;
    void activateWindow();
    void clearFocus();

    void setFocus(Qt::FocusReason reason);
    Qt::FocusPolicy focusPolicy() const;
    void setFocusPolicy(Qt::FocusPolicy policy);
    bool hasFocus() const;
    static void setTabOrder(QWidget *, QWidget *);
    void setFocusProxy(QWidget *);
    QWidget *focusProxy() const;
    Qt::ContextMenuPolicy contextMenuPolicy() const;
    void setContextMenuPolicy(Qt::ContextMenuPolicy policy);

    // Grab functions
    void grabMouse();
#ifndef QT_NO_CURSOR
    void grabMouse(const QCursor &);
#endif
    void releaseMouse();
    void grabKeyboard();
    void releaseKeyboard();
#ifndef QT_NO_SHORTCUT
    int grabShortcut(const QKeySequence &key, Qt::ShortcutContext context = Qt::WindowShortcut);
    void releaseShortcut(int id);
    void setShortcutEnabled(int id, bool enable = true);
    void setShortcutAutoRepeat(int id, bool enable = true);
#endif
    static QWidget *mouseGrabber();
    static QWidget *keyboardGrabber();

    // Update/refresh functions
    inline bool updatesEnabled() const;
    void setUpdatesEnabled(bool enable);

#if QT_CONFIG(graphicsview)
    QGraphicsProxyWidget *graphicsProxyWidget() const;
#endif

public Q_SLOTS:
    void update();
    void repaint();

public:
    inline void update(int x, int y, int w, int h);
    void update(const QRect&);
    void update(const QRegion&);

    void repaint(int x, int y, int w, int h);
    void repaint(const QRect &);
    void repaint(const QRegion &);

public Q_SLOTS:
    // Widget management functions

    virtual void setVisible(bool visible);
    void setHidden(bool hidden);
    void show();
    void hide();

    void showMinimized();
    void showMaximized();
    void showFullScreen();
    void showNormal();

    bool close();
    void raise();
    void lower();

public:
    void stackUnder(QWidget*);
    void move(int x, int y);
    void move(const QPoint &);
    void resize(int w, int h);
    void resize(const QSize &);
    inline void setGeometry(int x, int y, int w, int h);
    void setGeometry(const QRect &);
    QByteArray saveGeometry() const;
    bool restoreGeometry(const QByteArray &geometry);
    void adjustSize();
    bool isVisible() const;
    bool isVisibleTo(const QWidget *) const;
    inline bool isHidden() const;

    bool isMinimized() const;
    bool isMaximized() const;
    bool isFullScreen() const;

    Qt::WindowStates windowState() const;
    void setWindowState(Qt::WindowStates state);
    void overrideWindowState(Qt::WindowStates state);

    virtual QSize sizeHint() const;
    virtual QSize minimumSizeHint() const;

    QSizePolicy sizePolicy() const;
    void setSizePolicy(QSizePolicy);
    inline void setSizePolicy(QSizePolicy::Policy horizontal, QSizePolicy::Policy vertical);
    virtual int heightForWidth(int) const;
    virtual bool hasHeightForWidth() const;

    QRegion visibleRegion() const;

    void setContentsMargins(int left, int top, int right, int bottom);
    void setContentsMargins(const QMargins &margins);
#if QT_DEPRECATED_SINCE(5, 14)
    QT_DEPRECATED_X("use contentsMargins()")
    void getContentsMargins(int *left, int *top, int *right, int *bottom) const;
#endif
    QMargins contentsMargins() const;

    QRect contentsRect() const;

public:
    QLayout *layout() const;
    void setLayout(QLayout *);
    void updateGeometry();

    void setParent(QWidget *parent);
    void setParent(QWidget *parent, Qt::WindowFlags f);

    void scroll(int dx, int dy);
    void scroll(int dx, int dy, const QRect&);

    // Misc. functions

    QWidget *focusWidget() const;
    QWidget *nextInFocusChain() const;
    QWidget *previousInFocusChain() const;

    // drag and drop
    bool acceptDrops() const;
    void setAcceptDrops(bool on);

#ifndef QT_NO_ACTION
    //actions
    void addAction(QAction *action);
#if QT_VERSION >= QT_VERSION_CHECK(6,0,0)
    void addActions(const QList<QAction*> &actions);
    void insertActions(QAction *before, const QList<QAction*> &actions);
#else
    void addActions(QList<QAction*> actions);
    void insertActions(QAction *before, QList<QAction*> actions);
#endif
    void insertAction(QAction *before, QAction *action);
    void removeAction(QAction *action);
    QList<QAction*> actions() const;
#endif

    QWidget *parentWidget() const;

    void setWindowFlags(Qt::WindowFlags type);
    inline Qt::WindowFlags windowFlags() const;
    void setWindowFlag(Qt::WindowType, bool on = true);
    void overrideWindowFlags(Qt::WindowFlags type);

    inline Qt::WindowType windowType() const;

    static QWidget *find(WId);
    inline QWidget *childAt(int x, int y) const;
    QWidget *childAt(const QPoint &p) const;

    void setAttribute(Qt::WidgetAttribute, bool on = true);
    inline bool testAttribute(Qt::WidgetAttribute) const;

    QPaintEngine *paintEngine() const override;

    void ensurePolished() const;

    bool isAncestorOf(const QWidget *child) const;

#ifdef QT_KEYPAD_NAVIGATION
    bool hasEditFocus() const;
    void setEditFocus(bool on);
#endif

    bool autoFillBackground() const;
    void setAutoFillBackground(bool enabled);

    QBackingStore *backingStore() const;

    QWindow *windowHandle() const;
    QScreen *screen() const;

    static QWidget *createWindowContainer(QWindow *window, QWidget *parent=nullptr, Qt::WindowFlags flags=Qt::WindowFlags());

    friend class QDesktopScreenWidget;

Q_SIGNALS:
    void windowTitleChanged(const QString &title);
    void windowIconChanged(const QIcon &icon);
    void windowIconTextChanged(const QString &iconText);
    void customContextMenuRequested(const QPoint &pos);

protected:
    // Event handlers
    bool event(QEvent *event) override;
    virtual void mousePressEvent(QMouseEvent *event);
    virtual void mouseReleaseEvent(QMouseEvent *event);
    virtual void mouseDoubleClickEvent(QMouseEvent *event);
    virtual void mouseMoveEvent(QMouseEvent *event);
#if QT_CONFIG(wheelevent)
    virtual void wheelEvent(QWheelEvent *event);
#endif
    virtual void keyPressEvent(QKeyEvent *event);
    virtual void keyReleaseEvent(QKeyEvent *event);
    virtual void focusInEvent(QFocusEvent *event);
    virtual void focusOutEvent(QFocusEvent *event);
    virtual void enterEvent(QEvent *event);
    virtual void leaveEvent(QEvent *event);
    virtual void paintEvent(QPaintEvent *event);
    virtual void moveEvent(QMoveEvent *event);
    virtual void resizeEvent(QResizeEvent *event);
    virtual void closeEvent(QCloseEvent *event);
#ifndef QT_NO_CONTEXTMENU
    virtual void contextMenuEvent(QContextMenuEvent *event);
#endif
#if QT_CONFIG(tabletevent)
    virtual void tabletEvent(QTabletEvent *event);
#endif
#ifndef QT_NO_ACTION
    virtual void actionEvent(QActionEvent *event);
#endif

#if QT_CONFIG(draganddrop)
    virtual void dragEnterEvent(QDragEnterEvent *event);
    virtual void dragMoveEvent(QDragMoveEvent *event);
    virtual void dragLeaveEvent(QDragLeaveEvent *event);
    virtual void dropEvent(QDropEvent *event);
#endif

    virtual void showEvent(QShowEvent *event);
    virtual void hideEvent(QHideEvent *event);

#if QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)
    virtual bool nativeEvent(const QByteArray &eventType, void *message, qintptr *result);
#else
    virtual bool nativeEvent(const QByteArray &eventType, void *message, long *result);
#endif

    // Misc. protected functions
    virtual void changeEvent(QEvent *);

    int metric(PaintDeviceMetric) const override;
    void initPainter(QPainter *painter) const override;
    QPaintDevice *redirected(QPoint *offset) const override;
    QPainter *sharedPainter() const override;

    virtual void inputMethodEvent(QInputMethodEvent *);
public:
    virtual QVariant inputMethodQuery(Qt::InputMethodQuery) const;

    Qt::InputMethodHints inputMethodHints() const;
    void setInputMethodHints(Qt::InputMethodHints hints);

protected Q_SLOTS:
    void updateMicroFocus();
protected:

    void create(WId = 0, bool initializeWindow = true,
                         bool destroyOldWindow = true);
    void destroy(bool destroyWindow = true,
                 bool destroySubWindows = true);

    friend class QDataWidgetMapperPrivate; // for access to focusNextPrevChild
    virtual bool focusNextPrevChild(bool next);
    inline bool focusNextChild() { return focusNextPrevChild(true); }
    inline bool focusPreviousChild() { return focusNextPrevChild(false); }

protected:
    QWidget(QWidgetPrivate &d, QWidget* parent, Qt::WindowFlags f);
private:
    void setBackingStore(QBackingStore *store);

    bool testAttribute_helper(Qt::WidgetAttribute) const;

    QLayout *takeLayout();

    friend class QBackingStoreDevice;
    friend class QWidgetRepaintManager;
    friend class QApplication;
    friend class QApplicationPrivate;
    friend class QGuiApplication;
    friend class QGuiApplicationPrivate;
    friend class QBaseApplication;
    friend class QPainter;
    friend class QPainterPrivate;
    friend class QPixmap; // for QPixmap::fill()
    friend class QFontMetrics;
    friend class QFontInfo;
    friend class QLayout;
    friend class QWidgetItem;
    friend class QWidgetItemV2;
    friend class QGLContext;
    friend class QGLWidget;
    friend class QGLWindowSurface;
    friend class QX11PaintEngine;
    friend class QWin32PaintEngine;
    friend class QShortcutPrivate;
    friend class QWindowSurface;
    friend class QGraphicsProxyWidget;
    friend class QGraphicsProxyWidgetPrivate;
    friend class QStyleSheetStyle;
    friend struct QWidgetExceptionCleaner;
    friend class QWidgetWindow;
    friend class QAccessibleWidget;
    friend class QAccessibleTable;
    friend class QAccessibleTabButton;
#ifndef QT_NO_GESTURES
    friend class QGestureManager;
    friend class QWinNativePanGestureRecognizer;
#endif // QT_NO_GESTURES
    friend class QWidgetEffectSourcePrivate;

#ifdef Q_OS_MAC
    friend bool qt_mac_is_metal(const QWidget *w);
#endif
    friend Q_WIDGETS_EXPORT QWidgetData *qt_qwidget_data(QWidget *widget);
    friend Q_WIDGETS_EXPORT QWidgetPrivate *qt_widget_private(QWidget *widget);

private:
    Q_DISABLE_COPY(QWidget)
    Q_PRIVATE_SLOT(d_func(), void _q_showIfNotHidden())

    QWidgetData *data;
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QWidget::RenderFlags)

#ifndef Q_QDOC
template <> inline QWidget *qobject_cast<QWidget*>(QObject *o)
{
    if (!o || !o->isWidgetType()) return nullptr;
    return static_cast<QWidget*>(o);
}
template <> inline const QWidget *qobject_cast<const QWidget*>(const QObject *o)
{
    if (!o || !o->isWidgetType()) return nullptr;
    return static_cast<const QWidget*>(o);
}
#endif // !Q_QDOC

inline QWidget *QWidget::childAt(int ax, int ay) const
{ return childAt(QPoint(ax, ay)); }

inline Qt::WindowType QWidget::windowType() const
{ return static_cast<Qt::WindowType>(int(data->window_flags & Qt::WindowType_Mask)); }
inline Qt::WindowFlags QWidget::windowFlags() const
{ return data->window_flags; }

inline bool QWidget::isTopLevel() const
{ return (windowType() & Qt::Window); }

inline bool QWidget::isWindow() const
{ return (windowType() & Qt::Window); }

inline bool QWidget::isEnabled() const
{ return !testAttribute(Qt::WA_Disabled); }

inline bool QWidget::isModal() const
{ return data->window_modality != Qt::NonModal; }

#if QT_DEPRECATED_SINCE(5, 13)
inline bool QWidget::isEnabledToTLW() const
{ return isEnabled(); }
#endif

inline int QWidget::minimumWidth() const
{ return minimumSize().width(); }

inline int QWidget::minimumHeight() const
{ return minimumSize().height(); }

inline int QWidget::maximumWidth() const
{ return maximumSize().width(); }

inline int QWidget::maximumHeight() const
{ return maximumSize().height(); }

inline void QWidget::setMinimumSize(const QSize &s)
{ setMinimumSize(s.width(),s.height()); }

inline void QWidget::setMaximumSize(const QSize &s)
{ setMaximumSize(s.width(),s.height()); }

inline void QWidget::setSizeIncrement(const QSize &s)
{ setSizeIncrement(s.width(),s.height()); }

inline void QWidget::setBaseSize(const QSize &s)
{ setBaseSize(s.width(),s.height()); }

inline const QFont &QWidget::font() const
{ return data->fnt; }

inline QFontMetrics QWidget::fontMetrics() const
{ return QFontMetrics(data->fnt); }

inline QFontInfo QWidget::fontInfo() const
{ return QFontInfo(data->fnt); }

inline void QWidget::setMouseTracking(bool enable)
{ setAttribute(Qt::WA_MouseTracking, enable); }

inline bool QWidget::hasMouseTracking() const
{ return testAttribute(Qt::WA_MouseTracking); }

inline bool QWidget::underMouse() const
{ return testAttribute(Qt::WA_UnderMouse); }

inline void QWidget::setTabletTracking(bool enable)
{ setAttribute(Qt::WA_TabletTracking, enable); }

inline bool QWidget::hasTabletTracking() const
{ return testAttribute(Qt::WA_TabletTracking); }

inline bool QWidget::updatesEnabled() const
{ return !testAttribute(Qt::WA_UpdatesDisabled); }

inline void QWidget::update(int ax, int ay, int aw, int ah)
{ update(QRect(ax, ay, aw, ah)); }

inline bool QWidget::isVisible() const
{ return testAttribute(Qt::WA_WState_Visible); }

inline bool QWidget::isHidden() const
{ return testAttribute(Qt::WA_WState_Hidden); }

inline void QWidget::move(int ax, int ay)
{ move(QPoint(ax, ay)); }

inline void QWidget::resize(int w, int h)
{ resize(QSize(w, h)); }

inline void QWidget::setGeometry(int ax, int ay, int aw, int ah)
{ setGeometry(QRect(ax, ay, aw, ah)); }

inline QRect QWidget::rect() const
{ return QRect(0,0,data->crect.width(),data->crect.height()); }

inline const QRect &QWidget::geometry() const
{ return data->crect; }

inline QSize QWidget::size() const
{ return data->crect.size(); }

inline int QWidget::width() const
{ return data->crect.width(); }

inline int QWidget::height() const
{ return data->crect.height(); }

inline QWidget *QWidget::parentWidget() const
{ return static_cast<QWidget *>(QObject::parent()); }

inline void QWidget::setSizePolicy(QSizePolicy::Policy hor, QSizePolicy::Policy ver)
{ setSizePolicy(QSizePolicy(hor, ver)); }

inline bool QWidget::testAttribute(Qt::WidgetAttribute attribute) const
{
    if (attribute < int(8*sizeof(uint)))
        return data->widget_attributes & (1<<attribute);
    return testAttribute_helper(attribute);
}


#define QWIDGETSIZE_MAX ((1<<24)-1)

#ifndef QT_NO_DEBUG_STREAM
Q_WIDGETS_EXPORT QDebug operator<<(QDebug, const QWidget *);
#endif

QT_END_NAMESPACE

#endif // QWIDGET_H
