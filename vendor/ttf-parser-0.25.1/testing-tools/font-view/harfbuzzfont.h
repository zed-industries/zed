#pragma once

#include <QCoreApplication>

#include "glyph.h"

struct hb_blob_t;
struct hb_face_t;
struct hb_font_t;
struct hb_draw_funcs_t;

class HarfBuzzFont
{
    Q_DECLARE_TR_FUNCTIONS(HarfBuzzFont)

public:
    HarfBuzzFont();
    ~HarfBuzzFont();

    void open(const QString &path, const quint32 index = 0);
    bool isOpen() const;

    Glyph outline(const quint16 gid) const;

    void setVariations(const QVector<Variation> &variations);

private:
    void reset();

private:
    hb_blob_t *m_blob = nullptr;
    hb_face_t *m_face = nullptr;
    hb_font_t *m_font = nullptr;
};
