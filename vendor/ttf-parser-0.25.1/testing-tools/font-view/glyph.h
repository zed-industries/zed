#pragma once

#include <QPainterPath>

struct Tag
{
    Tag(quint32 v) : value(v) {}

    QString toString() const
    {
        QString s;
        s.append(QChar(value >> 24 & 0xff));
        s.append(QChar(value >> 16 & 0xff));
        s.append(QChar(value >> 8 & 0xff));
        s.append(QChar(value >> 0 & 0xff));
        return s;
    }

    quint32 value;
};

struct FontInfo
{
    qint16 ascender = 0;
    qint16 height = 1000;
    quint16 numberOfGlyphs = 0;
};

struct Glyph
{
    QPainterPath outline;
    QRect bbox;
};

struct VariationInfo
{
    QString name;
    Tag tag;
    qint16 min = 0;
    qint16 def = 0;
    qint16 max = 0;
};

struct Variation
{
    Tag tag;
    int value;
};
