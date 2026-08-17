// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QICON_H
#define QICON_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qsize.h>
#include <QtCore/qlist.h>
#include <QtGui/qpixmap.h>

QT_BEGIN_NAMESPACE


class QIconPrivate;
class QIconEngine;
class QPainter;

class Q_GUI_EXPORT QIcon
{
public:
    enum Mode { Normal, Disabled, Active, Selected };
    enum State { On, Off };

    QIcon() noexcept;
    QIcon(const QPixmap &pixmap);
    QIcon(const QIcon &other);
    QIcon(QIcon &&other) noexcept
        : d(qExchange(other.d, nullptr))
    {}
    explicit QIcon(const QString &fileName); // file or resource name
    explicit QIcon(QIconEngine *engine);
    ~QIcon();
    QIcon &operator=(const QIcon &other);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QIcon)
    inline void swap(QIcon &other) noexcept
    { qt_ptr_swap(d, other.d); }
    bool operator==(const QIcon &) const = delete;
    bool operator!=(const QIcon &) const = delete;

    operator QVariant() const;

    QPixmap pixmap(const QSize &size, Mode mode = Normal, State state = Off) const;
    inline QPixmap pixmap(int w, int h, Mode mode = Normal, State state = Off) const
        { return pixmap(QSize(w, h), mode, state); }
    inline QPixmap pixmap(int extent, Mode mode = Normal, State state = Off) const
        { return pixmap(QSize(extent, extent), mode, state); }
    QPixmap pixmap(const QSize &size, qreal devicePixelRatio, Mode mode = Normal, State state = Off) const;
#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_X_6_0("Use pixmap(size, devicePixelRatio) instead")
    QPixmap pixmap(QWindow *window, const QSize &size, Mode mode = Normal, State state = Off) const;
#endif

    QSize actualSize(const QSize &size, Mode mode = Normal, State state = Off) const;
#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_X_6_0("Use actualSize(size) instead")
    QSize actualSize(QWindow *window, const QSize &size, Mode mode = Normal, State state = Off) const;
#endif

    QString name() const;

    void paint(QPainter *painter, const QRect &rect, Qt::Alignment alignment = Qt::AlignCenter, Mode mode = Normal, State state = Off) const;
    inline void paint(QPainter *painter, int x, int y, int w, int h, Qt::Alignment alignment = Qt::AlignCenter, Mode mode = Normal, State state = Off) const
        { paint(painter, QRect(x, y, w, h), alignment, mode, state); }

    bool isNull() const;
    bool isDetached() const;
    void detach();

    qint64 cacheKey() const;

    void addPixmap(const QPixmap &pixmap, Mode mode = Normal, State state = Off);
    void addFile(const QString &fileName, const QSize &size = QSize(), Mode mode = Normal, State state = Off);

    QList<QSize> availableSizes(Mode mode = Normal, State state = Off) const;

    void setIsMask(bool isMask);
    bool isMask() const;

    static QIcon fromTheme(const QString &name);
    static QIcon fromTheme(const QString &name, const QIcon &fallback);
    static bool hasThemeIcon(const QString &name);

    static QStringList themeSearchPaths();
    static void setThemeSearchPaths(const QStringList &searchpath);

    static QStringList fallbackSearchPaths();
    static void setFallbackSearchPaths(const QStringList &paths);

    static QString themeName();
    static void setThemeName(const QString &path);

    static QString fallbackThemeName();
    static void setFallbackThemeName(const QString &name);

    Q_DUMMY_COMPARISON_OPERATOR(QIcon)

private:
    QIconPrivate *d;
#if !defined(QT_NO_DATASTREAM)
    friend Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QIcon &);
    friend Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QIcon &);
#endif

public:
    typedef QIconPrivate * DataPtr;
    inline DataPtr &data_ptr() { return d; }
};

Q_DECLARE_SHARED(QIcon)

#if !defined(QT_NO_DATASTREAM)
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QIcon &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QIcon &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug dbg, const QIcon &);
#endif

Q_GUI_EXPORT QString qt_findAtNxFile(const QString &baseFileName, qreal targetDevicePixelRatio,
                                     qreal *sourceDevicePixelRatio = nullptr);

QT_END_NAMESPACE

#endif // QICON_H
