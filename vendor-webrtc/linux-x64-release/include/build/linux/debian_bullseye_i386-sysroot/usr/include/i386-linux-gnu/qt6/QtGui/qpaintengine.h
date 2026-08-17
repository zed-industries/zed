// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPAINTENGINE_H
#define QPAINTENGINE_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qobjectdefs.h>
#include <QtCore/qscopedpointer.h>
#include <QtGui/qpainter.h>

QT_BEGIN_NAMESPACE


class QFontEngine;
class QLineF;
class QPaintDevice;
class QPaintEnginePrivate;
class QPainterPath;
class QPointF;
class QPolygonF;
class QRectF;
struct QGlyphLayout;
class QTextItemInt;
class QPaintEngineState;

class Q_GUI_EXPORT QTextItem {
public:
    enum RenderFlag {
        RightToLeft = 0x1,
        Overline = 0x10,
        Underline = 0x20,
        StrikeOut = 0x40,

        Dummy = 0xffffffff
    };
    Q_DECLARE_FLAGS(RenderFlags, RenderFlag)
    qreal descent() const;
    qreal ascent() const;
    qreal width() const;

    RenderFlags renderFlags() const;
    QString text() const;
    QFont font() const;
};
Q_DECLARE_TYPEINFO(QTextItem, Q_PRIMITIVE_TYPE);


class Q_GUI_EXPORT QPaintEngine
{
    Q_DECLARE_PRIVATE(QPaintEngine)
public:
    enum PaintEngineFeature {
        PrimitiveTransform          = 0x00000001, // Can transform primitives brushes
        PatternTransform            = 0x00000002, // Can transform pattern brushes
        PixmapTransform             = 0x00000004, // Can transform pixmaps
        PatternBrush                = 0x00000008, // Can fill with pixmaps and standard patterns
        LinearGradientFill          = 0x00000010, // Can fill gradient areas
        RadialGradientFill          = 0x00000020, // Can render radial gradients
        ConicalGradientFill         = 0x00000040, // Can render conical gradients
        AlphaBlend                  = 0x00000080, // Can do source over alpha blend
        PorterDuff                  = 0x00000100, // Can do general porter duff compositions
        PainterPaths                = 0x00000200, // Can fill, outline and clip paths
        Antialiasing                = 0x00000400, // Can antialias lines
        BrushStroke                 = 0x00000800, // Can render brush based pens
        ConstantOpacity             = 0x00001000, // Can render at constant opacity
        MaskedBrush                 = 0x00002000, // Can fill with textures that has an alpha channel or mask
        PerspectiveTransform        = 0x00004000, // Can do perspective transformations
        BlendModes                  = 0x00008000, // Can do extended Porter&Duff composition
        ObjectBoundingModeGradients = 0x00010000, // Can do object bounding mode gradients
        RasterOpModes               = 0x00020000, // Can do logical raster operations
        PaintOutsidePaintEvent      = 0x20000000, // Engine is capable of painting outside paint events
        /*                          0x10000000, // Used for emulating
                                    QGradient::StretchToDevice,
                                    defined in qpainter.cpp

                                    0x40000000, // Used internally for emulating opaque backgrounds
        */

        AllFeatures               = 0xffffffff  // For convenience
    };
    Q_DECLARE_FLAGS(PaintEngineFeatures, PaintEngineFeature)

    enum DirtyFlag {
        DirtyPen                = 0x0001,
        DirtyBrush              = 0x0002,
        DirtyBrushOrigin        = 0x0004,
        DirtyFont               = 0x0008,
        DirtyBackground         = 0x0010,
        DirtyBackgroundMode     = 0x0020,
        DirtyTransform          = 0x0040,
        DirtyClipRegion         = 0x0080,
        DirtyClipPath           = 0x0100,
        DirtyHints              = 0x0200,
        DirtyCompositionMode    = 0x0400,
        DirtyClipEnabled        = 0x0800,
        DirtyOpacity            = 0x1000,

        AllDirty                = 0xffff
    };
    Q_DECLARE_FLAGS(DirtyFlags, DirtyFlag)

    enum PolygonDrawMode {
        OddEvenMode,
        WindingMode,
        ConvexMode,
        PolylineMode
    };

    explicit QPaintEngine(PaintEngineFeatures features=PaintEngineFeatures());
    virtual ~QPaintEngine();

    bool isActive() const { return active; }
    void setActive(bool newState) { active = newState; }

    virtual bool begin(QPaintDevice *pdev) = 0;
    virtual bool end() = 0;

    virtual void updateState(const QPaintEngineState &state) = 0;

    virtual void drawRects(const QRect *rects, int rectCount);
    virtual void drawRects(const QRectF *rects, int rectCount);

    virtual void drawLines(const QLine *lines, int lineCount);
    virtual void drawLines(const QLineF *lines, int lineCount);

    virtual void drawEllipse(const QRectF &r);
    virtual void drawEllipse(const QRect &r);

    virtual void drawPath(const QPainterPath &path);

    virtual void drawPoints(const QPointF *points, int pointCount);
    virtual void drawPoints(const QPoint *points, int pointCount);

    virtual void drawPolygon(const QPointF *points, int pointCount, PolygonDrawMode mode);
    virtual void drawPolygon(const QPoint *points, int pointCount, PolygonDrawMode mode);

    virtual void drawPixmap(const QRectF &r, const QPixmap &pm, const QRectF &sr) = 0;
    virtual void drawTextItem(const QPointF &p, const QTextItem &textItem);
    virtual void drawTiledPixmap(const QRectF &r, const QPixmap &pixmap, const QPointF &s);
    virtual void drawImage(const QRectF &r, const QImage &pm, const QRectF &sr,
                           Qt::ImageConversionFlags flags = Qt::AutoColor);

    void setPaintDevice(QPaintDevice *device);
    QPaintDevice *paintDevice() const;

    void setSystemClip(const QRegion &baseClip);
    QRegion systemClip() const;

    void setSystemRect(const QRect &rect);
    QRect systemRect() const;


    virtual QPoint coordinateOffset() const;

    enum Type {
        X11,
        Windows,
        QuickDraw, CoreGraphics, MacPrinter,
        QWindowSystem,
        OpenGL,
        Picture,
        SVG,
        Raster,
        Direct3D,
        Pdf,
        OpenVG,
        OpenGL2,
        PaintBuffer,
        Blitter,
        Direct2D,

        User = 50,    // first user type id
        MaxUser = 100 // last user type id
    };
    virtual Type type() const = 0;

    inline void fix_neg_rect(int *x, int *y, int *w, int *h);

    inline bool testDirty(DirtyFlags df);
    inline void setDirty(DirtyFlags df);
    inline void clearDirty(DirtyFlags df);

    bool hasFeature(PaintEngineFeatures feature) const { return bool(gccaps & feature); }

    QPainter *painter() const;

    void syncState();
    inline bool isExtended() const { return extended; }

    virtual QPixmap createPixmap(QSize size);
    virtual QPixmap createPixmapFromImage(QImage image, Qt::ImageConversionFlags flags = Qt::AutoColor);

protected:
    QPaintEngine(QPaintEnginePrivate &data, PaintEngineFeatures devcaps=PaintEngineFeatures());

    QPaintEngineState *state;
    PaintEngineFeatures gccaps;

    uint active : 1;
    uint selfDestruct : 1;
    uint extended : 1;

    QScopedPointer<QPaintEnginePrivate> d_ptr;

private:
    void setAutoDestruct(bool autoDestr) { selfDestruct = autoDestr; }
    bool autoDestruct() const { return selfDestruct; }
    Q_DISABLE_COPY(QPaintEngine)

    friend class QPainterReplayer;
    friend class QFontEngineBox;
    friend class QFontEngineMac;
    friend class QFontEngineWin;
    friend class QMacPrintEngine;
    friend class QMacPrintEnginePrivate;
    friend class QFontEngineQPF2;
    friend class QPainter;
    friend class QPainterPrivate;
    friend class QWidget;
    friend class QWidgetPrivate;
    friend class QWin32PaintEngine;
    friend class QWin32PaintEnginePrivate;
    friend class QMacCGContext;
    friend class QPreviewPaintEngine;
    friend class QX11GLPlatformPixmap;
};


class Q_GUI_EXPORT QPaintEngineState
{
public:
    QPaintEngine::DirtyFlags state() const { return dirtyFlags; }

    QPen pen() const;
    QBrush brush() const;
    QPointF brushOrigin() const;
    QBrush backgroundBrush() const;
    Qt::BGMode backgroundMode() const;
    QFont font() const;
    QTransform transform() const;

    Qt::ClipOperation clipOperation() const;
    QRegion clipRegion() const;
    QPainterPath clipPath() const;
    bool isClipEnabled() const;

    QPainter::RenderHints renderHints() const;
    QPainter::CompositionMode compositionMode() const;
    qreal opacity() const;

    QPainter *painter() const;

    bool brushNeedsResolving() const;
    bool penNeedsResolving() const;

protected:
    friend class QPaintEngine;
    friend class QRasterPaintEngine;
    friend class QWidget;
    friend class QPainter;
    friend class QPainterPrivate;
    friend class QMacPrintEnginePrivate;

    QPaintEngine::DirtyFlags dirtyFlags;
};

//
// inline functions
//

inline void QPaintEngine::fix_neg_rect(int *x, int *y, int *w, int *h)
{
    if (*w < 0) {
        *w = -*w;
        *x -= *w - 1;
    }
    if (*h < 0) {
        *h = -*h;
        *y -= *h - 1;
    }
}

inline bool QPaintEngine::testDirty(DirtyFlags df) {
    Q_ASSERT(state);
    return bool(state->dirtyFlags & df);
}

inline void QPaintEngine::setDirty(DirtyFlags df) {
    Q_ASSERT(state);
    state->dirtyFlags |= df;
}

inline void QPaintEngine::clearDirty(DirtyFlags df)
{
    Q_ASSERT(state);
    state->dirtyFlags &= ~df;
}

Q_DECLARE_OPERATORS_FOR_FLAGS(QTextItem::RenderFlags)
Q_DECLARE_OPERATORS_FOR_FLAGS(QPaintEngine::PaintEngineFeatures)
Q_DECLARE_OPERATORS_FOR_FLAGS(QPaintEngine::DirtyFlags)

QT_END_NAMESPACE

#endif // QPAINTENGINE_H
