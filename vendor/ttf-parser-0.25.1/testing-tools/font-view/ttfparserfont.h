#pragma once

#include <QCoreApplication>
#include <QPainterPath>

#include <memory>

#define TTFP_VARIABLE_FONTS
#include <ttfparser.h>

#include "glyph.h"

class TtfParserFont
{
    Q_DECLARE_TR_FUNCTIONS(TtfParserFont)

public:
    TtfParserFont();

    void open(const QString &path, const quint32 index = 0);
    bool isOpen() const;

    FontInfo fontInfo() const;
    Glyph outline(const quint16 gid) const;

    QVector<VariationInfo> loadVariations();
    void setVariations(const QVector<Variation> &variations);

private:
    struct FreeCPtr
    { void operator()(void* x) { free(x); } };

    QByteArray m_fontData;
    std::unique_ptr<ttfp_face, FreeCPtr> m_face;
};
