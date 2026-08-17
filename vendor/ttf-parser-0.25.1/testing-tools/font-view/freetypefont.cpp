// Based on https://www.freetype.org/freetype2/docs/tutorial/example5.cpp

#include <QDebug>

#include "freetypefont.h"

const FT_Fixed MULTIPLIER_FT = 65536L;

const char* getErrorMessage(FT_Error err)
{
    #undef __FTERRORS_H__
    #define FT_ERRORDEF( e, v, s )  case e: return s;
    #define FT_ERROR_START_LIST     switch (err) {
    #define FT_ERROR_END_LIST       }
    #include FT_ERRORS_H
    return "(Unknown error)";
}

struct Outliner
{
    static int moveToFn(const FT_Vector *to, void *user)
    {
        auto self = static_cast<Outliner *>(user);
        self->path.moveTo(to->x, to->y);
        return 0;
    }

    static int lineToFn(const FT_Vector *to, void *user)
    {
        auto self = static_cast<Outliner *>(user);
        self->path.lineTo(to->x, to->y);
        return 0;
    }

    static int quadToFn(const FT_Vector *control, const FT_Vector *to, void *user)
    {
        auto self = static_cast<Outliner *>(user);
        self->path.quadTo(control->x, control->y, to->x, to->y);
        return 0;
    }

    static int cubicToFn(const FT_Vector *controlOne,
                         const FT_Vector *controlTwo,
                         const FT_Vector *to,
                         void *user)
    {
        auto self = static_cast<Outliner *>(user);
        self->path.cubicTo(controlOne->x, controlOne->y, controlTwo->x, controlTwo->y, to->x, to->y);
        return 0;
    }

    QPainterPath path;
};

FreeTypeFont::FreeTypeFont()
{
    const auto error = FT_Init_FreeType(&m_ftLibrary);
    if (error) {
        throw tr("Failed to init FreeType.\n%1").arg(getErrorMessage(error));
    }
}

FreeTypeFont::~FreeTypeFont()
{
    if (m_ftFace) {
        FT_Done_Face(m_ftFace);
    }

    FT_Done_FreeType(m_ftLibrary);
}

void FreeTypeFont::open(const QString &path, const quint32 index)
{
    if (isOpen()) {
        FT_Done_Face(m_ftFace);
        m_ftFace = nullptr;
    }

    const auto utf8Path = path.toUtf8();
    const auto error = FT_New_Face(m_ftLibrary, utf8Path.constData(), index, &m_ftFace);
    if (error) {
        throw tr("Failed to open a font.\n%1").arg(getErrorMessage(error));
    }
}

bool FreeTypeFont::isOpen() const
{
    return m_ftFace != nullptr;
}

FontInfo FreeTypeFont::fontInfo() const
{
    if (!isOpen()) {
        throw tr("Font is not loaded.");
    }

    return FontInfo {
        m_ftFace->ascender,
        m_ftFace->height,
        (quint16)m_ftFace->num_glyphs, // TrueType allows only u16.
    };
}

Glyph FreeTypeFont::outline(const quint16 gid) const
{
    if (!isOpen()) {
        throw tr("Font is not loaded.");
    }

    auto error = FT_Load_Glyph(m_ftFace, gid, FT_LOAD_NO_SCALE | FT_LOAD_NO_BITMAP);
    if (error) {
        throw tr("Failed to load a glyph.\n%1").arg(getErrorMessage(error));
    }

    Outliner outliner;

    FT_Outline_Funcs funcs;
    funcs.move_to = outliner.moveToFn;
    funcs.line_to = outliner.lineToFn;
    funcs.conic_to = outliner.quadToFn;
    funcs.cubic_to = outliner.cubicToFn;
    funcs.shift = 0;
    funcs.delta = 0;

    auto slot = m_ftFace->glyph;
    auto &outline = slot->outline;

    // Flip outline around x-axis.
    FT_Matrix matrix;
    matrix.xx = 1L * MULTIPLIER_FT;
    matrix.xy = 0L * MULTIPLIER_FT;
    matrix.yx = 0L * MULTIPLIER_FT;
    matrix.yy = -1L * MULTIPLIER_FT;
    FT_Outline_Transform(&outline, &matrix);

    FT_BBox bboxFt;
    FT_Outline_Get_BBox(&outline, &bboxFt);

    const QRect bbox(
        (int)bboxFt.xMin,
        (int)bboxFt.yMin,
        (int)bboxFt.xMax - (int)bboxFt.xMin,
        (int)bboxFt.yMax - (int)bboxFt.yMin
    );

    error = FT_Outline_Decompose(&outline, &funcs, &outliner);
    if (error) {
        throw tr("Failed to outline a glyph.\n%1").arg(getErrorMessage(error));
    }

    outliner.path.setFillRule(Qt::WindingFill);

    return Glyph {
        outliner.path,
        bbox,
    };
}

void FreeTypeFont::setVariations(const QVector<Variation> &variations)
{
    if (!isOpen()) {
        throw tr("Font is not loaded.");
    }

    QVector<FT_Fixed> ftCoords;

    for (const auto &var : variations) {
        ftCoords << var.value * MULTIPLIER_FT;
    }

    const auto error = FT_Set_Var_Design_Coordinates(m_ftFace, ftCoords.size(), ftCoords.data());
    if (error) {
        throw tr("Failed to set variation.\n%1").arg(getErrorMessage(error));
    }
}
