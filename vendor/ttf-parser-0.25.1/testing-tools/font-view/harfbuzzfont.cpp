#include <QTransform>
#include <QDebug>

#include <hb.h>

#include "harfbuzzfont.h"

struct Outliner
{
    static void moveToFn(hb_position_t to_x, hb_position_t to_y, Outliner &outliner)
    {
        outliner.path.moveTo(to_x, to_y);
    }

    static void lineToFn(hb_position_t to_x, hb_position_t to_y, Outliner &outliner)
    {
        outliner.path.lineTo(to_x, to_y);
    }

    static void quadToFn(hb_position_t control_x, hb_position_t control_y,
                         hb_position_t to_x, hb_position_t to_y,
                         Outliner &outliner)
    {
        outliner.path.quadTo(control_x, control_y, to_x, to_y);
    }

    static void cubicToFn(hb_position_t control1_x, hb_position_t control1_y,
                          hb_position_t control2_x, hb_position_t control2_y,
                          hb_position_t to_x, hb_position_t to_y,
                          Outliner &outliner)
    {
        outliner.path.cubicTo(control1_x, control1_y, control2_x, control2_y, to_x, to_y);
    }

    static void closePathFn(Outliner &outliner)
    {
        outliner.path.closeSubpath();
    }

    QPainterPath path;
};

HarfBuzzFont::HarfBuzzFont()
{

}

HarfBuzzFont::~HarfBuzzFont()
{
    reset();
}

void HarfBuzzFont::open(const QString &path, const quint32 index)
{
    if (isOpen()) {
        reset();
    }

    const auto utf8Path = path.toUtf8();
    hb_blob_t *blob = hb_blob_create_from_file(utf8Path.constData());
    if (!blob) {
        throw tr("Failed to open a font.");
    }

    hb_face_t *face = hb_face_create(blob, index);
    if (!face) {
        throw tr("Failed to open a font.");
    }

    hb_font_t *font = hb_font_create(face);
    if (!font) {
        throw tr("Failed to open a font.");
    }

    m_blob = blob;
    m_face = face;
    m_font = font;
}

bool HarfBuzzFont::isOpen() const
{
    return m_font != nullptr;
}

Glyph HarfBuzzFont::outline(const quint16 gid) const
{
    if (!isOpen()) {
        throw tr("Font is not loaded.");
    }

    Outliner outliner;

    hb_draw_funcs_t *funcs = hb_draw_funcs_create();
    hb_draw_funcs_set_move_to_func(funcs, (hb_draw_move_to_func_t)outliner.moveToFn);
    hb_draw_funcs_set_line_to_func(funcs, (hb_draw_line_to_func_t)outliner.lineToFn);
    hb_draw_funcs_set_quadratic_to_func(funcs, (hb_draw_quadratic_to_func_t)outliner.quadToFn);
    hb_draw_funcs_set_cubic_to_func(funcs, (hb_draw_cubic_to_func_t)outliner.cubicToFn);
    hb_draw_funcs_set_close_path_func(funcs, (hb_draw_close_path_func_t)outliner.closePathFn);

    if (!hb_font_draw_glyph(m_font, gid, funcs, &outliner)) {
        throw tr("Failed to outline a glyph %1.").arg(gid);
    }

    hb_draw_funcs_destroy(funcs);

    hb_glyph_extents_t extents = {0, 0, 0, 0};
    if (!hb_font_get_glyph_extents(m_font, gid, &extents)) {
        throw tr("Failed to query glyph extents.");
    }

    const QRect bbox(
        extents.x_bearing,
        -extents.y_bearing,
        extents.width,
        -extents.height
    );

    // Flip outline around x-axis.
    QTransform ts(1, 0, 0, -1, 0, 0);
    outliner.path = ts.map(outliner.path);

    outliner.path.setFillRule(Qt::WindingFill);

    return Glyph {
        outliner.path,
        bbox,
    };
}

void HarfBuzzFont::setVariations(const QVector<Variation> &variations)
{
    if (!isOpen()) {
        throw tr("Font is not loaded.");
    }

    QVector<hb_variation_t> hbVariations;
    for (const auto &var : variations) {
        hbVariations.append({ var.tag.value, (float)var.value });
    }

    hb_font_set_variations(m_font, hbVariations.constData(), hbVariations.size());
}

void HarfBuzzFont::reset()
{
    if (m_blob) {
        hb_blob_destroy(m_blob);
        m_blob = nullptr;
    }

    if (m_font) {
        hb_font_destroy(m_font);
        m_font = nullptr;
    }

    if (m_face) {
        hb_face_destroy(m_face);
        m_face = nullptr;
    }
}
