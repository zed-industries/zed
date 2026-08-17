#pragma once

#include <ft2build.h>
#include FT_FREETYPE_H
#include FT_OUTLINE_H
#include FT_BBOX_H
#include FT_MULTIPLE_MASTERS_H

#include <QCoreApplication>

#include "glyph.h"

class FreeTypeFont
{
    Q_DECLARE_TR_FUNCTIONS(FreeTypeFont)

public:
    FreeTypeFont();
    ~FreeTypeFont();

    void open(const QString &path, const quint32 index = 0);
    bool isOpen() const;

    FontInfo fontInfo() const;
    Glyph outline(const quint16 gid) const;

    void setVariations(const QVector<Variation> &variations);

private:
    FT_Library m_ftLibrary = nullptr;
    FT_Face m_ftFace = nullptr;
};
