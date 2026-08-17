#include <QDebug>
#include <QMouseEvent>
#include <QPainter>
#include <QScrollBar>

#include <cmath>

#include "glyphsview.h"

static const int COLUMNS_COUNT = 100;

GlyphsView::GlyphsView(QWidget *parent) : QAbstractScrollArea(parent)
{
    setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOn);
    setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOn);
}

void GlyphsView::setFontInfo(const FontInfo &fi)
{
    m_fontInfo = fi;
    m_glyphs.resize(fi.numberOfGlyphs);
#ifdef WITH_FREETYPE
    m_ftGlyphs.resize(fi.numberOfGlyphs);
#endif
#ifdef WITH_HARFBUZZ
    m_hbGlyphs.resize(fi.numberOfGlyphs);
#endif

    m_indexes.clear();
    for (int i = 0; i < fi.numberOfGlyphs; ++i) {
        QStaticText text(QString::number(i));
        text.prepare();
        m_indexes << text;
    }

    updateScrollBars();
    horizontalScrollBar()->setValue(0);
    verticalScrollBar()->setValue(0);
}

void GlyphsView::setGlyph(int idx, const Glyph &glyph)
{
    m_glyphs.replace(idx, glyph);
}

#ifdef WITH_FREETYPE
void GlyphsView::setFTGlyph(int idx, const Glyph &glyph)
{
    m_ftGlyphs.replace(idx, glyph);
}
#endif

#ifdef WITH_HARFBUZZ
void GlyphsView::setHBGlyph(int idx, const Glyph &glyph)
{
    m_hbGlyphs.replace(idx, glyph);
}
#endif

void GlyphsView::setDrawBboxes(const bool flag)
{
    m_drawBboxes = flag;
    viewport()->update();
}

void GlyphsView::setDrawGlyphs(const bool flag)
{
    m_drawGlyphs = flag;
    viewport()->update();
}

void GlyphsView::setDrawFTGlyphs(const bool flag)
{
    m_drawFTGlyphs = flag;
    viewport()->update();
}

void GlyphsView::setDrawHBGlyphs(const bool flag)
{
    m_drawHBGlyphs = flag;
    viewport()->update();
}

void GlyphsView::paintEvent(QPaintEvent *)
{
    QPainter p(viewport());
    p.translate(-horizontalScrollBar()->value(), -verticalScrollBar()->value());

    const double cellHeight = m_fontInfo.height * m_scale;
    drawGrid(p, cellHeight);

    p.setRenderHint(QPainter::Antialiasing);

    {
        auto font = p.font();
        font.setPointSize(10);
        p.setFont(font);
    }

    int x = 0;
    int y = m_fontInfo.ascender;
    int num_y = m_fontInfo.height;
    for (int i = 0; i < m_glyphs.size(); ++i) {
        // Text rendering is the slowest part, so we are using preprocessed text.
        p.setPen(palette().color(QPalette::Text));
        p.drawStaticText(
            qRound(x * m_scale + 1),
            qRound(num_y * m_scale - p.fontMetrics().ascent() - 2),
            m_indexes.at(i)
        );

        if (m_drawGlyphs) {
            p.save();

            const int dx = qRound((m_fontInfo.height - m_glyphs.at(i).bbox.width()) / 2.0)
                - m_glyphs.at(i).bbox.x();

            p.scale(m_scale, m_scale);
            p.translate(x + dx, y);

            if (m_drawBboxes) {
                p.setPen(QPen(Qt::darkGreen, 0.5 / m_scale));
                p.setBrush(Qt::NoBrush);
                p.drawRect(m_glyphs.at(i).bbox);
            }

            p.setPen(Qt::NoPen);
            p.setPen(Qt::NoPen);
            if (m_drawFTGlyphs || m_drawHBGlyphs) {
                p.setBrush(Qt::red);
            } else {
                p.setBrush(palette().color(QPalette::Text));
            }

            p.drawPath(m_glyphs.at(i).outline);

            p.restore();
        }

#ifdef WITH_HARFBUZZ
        if (m_drawHBGlyphs) {
            p.save();

            const int dx = qRound((m_fontInfo.height - m_hbGlyphs.at(i).bbox.width()) / 2.0)
                - m_hbGlyphs.at(i).bbox.x();

            p.scale(m_scale, m_scale);
            p.translate(x + dx, y);

            if (m_drawBboxes) {
                p.setPen(QPen(Qt::darkGreen, 0.5 / m_scale));
                p.setBrush(Qt::NoBrush);
                p.drawRect(m_hbGlyphs.at(i).bbox);
            }

            p.setPen(Qt::NoPen);
            if (m_drawFTGlyphs) {
                p.setBrush(Qt::blue);
            } else {
                p.setBrush(palette().color(QPalette::Text));
            }

            p.drawPath(m_hbGlyphs.at(i).outline);

            p.restore();
        }
#endif

#ifdef WITH_FREETYPE
        if (m_drawFTGlyphs) {
            p.save();

            const int dx = qRound((m_fontInfo.height - m_ftGlyphs.at(i).bbox.width()) / 2.0)
                - m_ftGlyphs.at(i).bbox.x();

            p.scale(m_scale, m_scale);
            p.translate(x + dx, y);

            if (m_drawBboxes) {
                p.setPen(QPen(Qt::darkGreen, 0.5 / m_scale));
                p.setBrush(Qt::NoBrush);
                p.drawRect(m_ftGlyphs.at(i).bbox);
            }

            p.setPen(Qt::NoPen);
            p.setBrush(palette().color(QPalette::Text));

            if (m_drawGlyphs || m_drawHBGlyphs) {
                p.setBrush(palette().color(QPalette::Base));
            }

            p.drawPath(m_ftGlyphs.at(i).outline);

            p.restore();
        }
#endif

        x += m_fontInfo.height;
        if (i > 0 && (i + 1) % COLUMNS_COUNT == 0) {
            x = 0;
            y += m_fontInfo.height;
            num_y += m_fontInfo.height;
        }
    }
}

void GlyphsView::drawGrid(QPainter &p, const double cellHeight)
{
    p.setRenderHint(QPainter::Antialiasing, false);
    p.setPen(QPen(palette().color(QPalette::Text), 0.25));
    p.setBrush(Qt::NoBrush);

    const int rows = qRound(floor(m_glyphs.size() / COLUMNS_COUNT)) + 1;
    const auto maxH = qMin(rows * cellHeight, (double)horizontalScrollBar()->maximum());

    double x = cellHeight;
    for (int c = 1; c < COLUMNS_COUNT; ++c) {
        p.drawLine(QLineF(x, 0, x, maxH));
        x += cellHeight;
    }

    double y = cellHeight;
    for (int r = 1; r <= rows; ++r) {
        p.drawLine(QLineF(0, y, horizontalScrollBar()->maximum() + viewport()->width(), y));
        y += cellHeight;
    }
}

void GlyphsView::mousePressEvent(QMouseEvent *e)
{
    if (e->button() & Qt::LeftButton) {
        m_mousePressPos = e->pos();
        m_origOffset = QPoint(horizontalScrollBar()->value(), verticalScrollBar()->value());
    }
}

void GlyphsView::mouseMoveEvent(QMouseEvent *e)
{
    if (m_mousePressPos.isNull()) {
        return;
    }

    const auto diff = m_mousePressPos - e->pos();
    horizontalScrollBar()->setValue(m_origOffset.x() + diff.x());
    verticalScrollBar()->setValue(m_origOffset.y() + diff.y());
}

void GlyphsView::mouseReleaseEvent(QMouseEvent *)
{
    m_mousePressPos = QPoint();
    m_origOffset = QPoint();
}

void GlyphsView::wheelEvent(QWheelEvent *e)
{
    e->accept();

    if (e->angleDelta().y() > 0) {
        m_scale += 0.01;
    } else {
        m_scale -= 0.01;
    }

    m_scale = qBound(0.03, m_scale, 1.0);

    updateScrollBars();
    viewport()->update();
}

void GlyphsView::resizeEvent(QResizeEvent *e)
{
    QAbstractScrollArea::resizeEvent(e);
    updateScrollBars();
}

void GlyphsView::updateScrollBars()
{
    const double cellHeight = m_fontInfo.height * m_scale;
    const int rows = qRound(floor(m_glyphs.size() / COLUMNS_COUNT)) + 1;
    const auto w = COLUMNS_COUNT * cellHeight - viewport()->width();
    const auto h = rows * cellHeight - viewport()->height();
    horizontalScrollBar()->setMinimum(0);
    verticalScrollBar()->setMinimum(0);
    horizontalScrollBar()->setMaximum(qMax(0, qRound(w)));
    verticalScrollBar()->setMaximum(qMax(0, qRound(h)));
}
