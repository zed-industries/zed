#pragma once

#include <QMainWindow>

#ifdef WITH_FREETYPE
#include "freetypefont.h"
#endif

#ifdef WITH_FREETYPE
#include "harfbuzzfont.h"
#endif

#include "ttfparserfont.h"

namespace Ui { class MainWindow; }

class QSlider;

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private:
    void loadFont(const QString &path);
    void reloadGlyphs();
    void onVariationChanged();

private slots:
    void on_chBoxDrawBboxes_stateChanged(int flag);
    void on_chBoxDrawTtfParser_stateChanged(int flag);
    void on_chBoxDrawFreeType_stateChanged(int flag);
    void on_chBoxDrawHarfBuzz_stateChanged(int flag);

private:
    struct VariationSlider
    {
        QSlider *slider;
        Tag tag;
    };

    Ui::MainWindow * const ui;
    QVector<VariationSlider> m_variationSliders;
    TtfParserFont m_ttfpFont;
#ifdef WITH_FREETYPE
    FreeTypeFont m_ftFont;
#endif
#ifdef WITH_HARFBUZZ
    HarfBuzzFont m_hbFont;
#endif
};
