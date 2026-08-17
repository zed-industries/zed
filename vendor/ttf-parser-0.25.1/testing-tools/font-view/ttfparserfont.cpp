#include <QTransform>
#include <QFile>
#include <QDebug>

#include "ttfparserfont.h"

struct Outliner
{
    static void moveToFn(float x, float y, void *user)
    {
        auto self = static_cast<Outliner *>(user);
        self->path.moveTo(double(x), double(y));
    }

    static void lineToFn(float x, float y, void *user)
    {
        auto self = static_cast<Outliner *>(user);
       self->path.lineTo(double(x), double(y));
    }

    static void quadToFn(float x1, float y1, float x, float y, void *user)
    {
        auto self = static_cast<Outliner *>(user);
        self->path.quadTo(double(x1), double(y1), double(x), double(y));
    }

    static void curveToFn(float x1, float y1, float x2, float y2, float x, float y, void *user)
    {
        auto self = static_cast<Outliner *>(user);
        self->path.cubicTo(double(x1), double(y1), double(x2), double(y2), double(x), double(y));
    }

    static void closePathFn(void *user)
    {
        auto self = static_cast<Outliner *>(user);
        self->path.closeSubpath();
    }

    QPainterPath path;
};

TtfParserFont::TtfParserFont()
{
}

void TtfParserFont::open(const QString &path, const quint32 index)
{
    if (isOpen()) {
        m_face.reset();
    }

    QFile file(path);
    file.open(QFile::ReadOnly);
    m_fontData = file.readAll();

    m_face.reset((ttfp_face*)malloc(ttfp_face_size_of()));
    const auto res = ttfp_face_init(m_fontData.constData(), m_fontData.size(), index, m_face.get());

    if (!res) {
        throw tr("Failed to open a font.");
    }
}

bool TtfParserFont::isOpen() const
{
    return m_face != nullptr;
}

FontInfo TtfParserFont::fontInfo() const
{
    if (!isOpen()) {
        throw tr("Font is not loaded.");
    }

    return FontInfo {
        ttfp_get_ascender(m_face.get()),
        ttfp_get_height(m_face.get()),
        ttfp_get_number_of_glyphs(m_face.get()),
    };
}

Glyph TtfParserFont::outline(const quint16 gid) const
{
    if (!isOpen()) {
        throw tr("Font is not loaded.");
    }

    Outliner outliner;
    ttfp_outline_builder builder;
    builder.move_to = outliner.moveToFn;
    builder.line_to = outliner.lineToFn;
    builder.quad_to = outliner.quadToFn;
    builder.curve_to = outliner.curveToFn;
    builder.close_path = outliner.closePathFn;

    ttfp_rect rawBbox;

    const bool ok = ttfp_outline_glyph(
        m_face.get(),
        builder,
        &outliner,
        gid,
        &rawBbox
    );

    if (!ok) {
        return Glyph {
            QPainterPath(),
            QRect(),
        };
    }

    const QRect bbox(
        rawBbox.x_min,
        -rawBbox.y_max,
        rawBbox.x_max - rawBbox.x_min,
        rawBbox.y_max - rawBbox.y_min
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

QVector<VariationInfo> TtfParserFont::loadVariations()
{
    if (!isOpen()) {
        throw tr("Font is not loaded.");
    }

    QVector<VariationInfo> variations;

    for (uint16_t i = 0; i < ttfp_get_variation_axes_count(m_face.get()); ++i) {
        ttfp_variation_axis axis;
        ttfp_get_variation_axis(m_face.get(), i, &axis);

        variations.append(VariationInfo {
            Tag(axis.tag).toString(),
            { static_cast<quint32>(axis.tag) },
            static_cast<qint16>(axis.min_value),
            static_cast<qint16>(axis.def_value),
            static_cast<qint16>(axis.max_value),
        });
    }

    return variations;
}

void TtfParserFont::setVariations(const QVector<Variation> &variations)
{
    if (!isOpen()) {
        throw tr("Font is not loaded.");
    }

    for (const auto &variation : variations) {
        ttfp_set_variation(m_face.get(), variation.tag.value, variation.value);
    }
}
