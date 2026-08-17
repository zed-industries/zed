#pragma once

#include <QAbstractScrollArea>
#include <QStaticText>

#include "glyph.h"

class GlyphsView : public QAbstractScrollArea
{
    Q_OBJECT

public:
    explicit GlyphsView(QWidget *parent = nullptr);

    void setFontInfo(const FontInfo &fi);
    void setGlyph(int idx, const Glyph &glyph);
#ifdef WITH_FREETYPE
    void setFTGlyph(int idx, const Glyph &glyph);
#endif
#ifdef WITH_HARFBUZZ
    void setHBGlyph(int idx, const Glyph &glyph);
#endif

    void setDrawBboxes(const bool flag);
    void setDrawGlyphs(const bool flag);
    void setDrawFTGlyphs(const bool flag);
    void setDrawHBGlyphs(const bool flag);

private:
    void paintEvent(QPaintEvent *);
    void drawGrid(QPainter &p, const double cellHeight);

    void mousePressEvent(QMouseEvent *e);
    void mouseMoveEvent(QMouseEvent *e);
    void mouseReleaseEvent(QMouseEvent *e);
    void wheelEvent(QWheelEvent *e);

    void resizeEvent(QResizeEvent *);

    void updateScrollBars();

private:
    QPoint m_mousePressPos;
    QPoint m_origOffset;

    double m_scale = 0.05;
    bool m_drawBboxes = true;
    bool m_drawGlyphs = true;
    bool m_drawFTGlyphs = false;
    bool m_drawHBGlyphs = false;

    FontInfo m_fontInfo;
    QVector<Glyph> m_glyphs;
#ifdef WITH_FREETYPE
    QVector<Glyph> m_ftGlyphs;
#endif
#ifdef WITH_HARFBUZZ
    QVector<Glyph> m_hbGlyphs;
#endif
    QVector<QStaticText> m_indexes;
};
